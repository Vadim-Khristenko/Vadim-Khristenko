//! Provider abstraction + the per-platform collector.
//!
//! A `Provider` is one code-hosting platform. Everything degrades gracefully:
//! a missing token means tokenless public reads, an unreachable host is logged
//! and skipped — never fatal. Adding a platform = implement this trait (or
//! reuse `ForgejoProvider` with a different `base_url`) + a providers.toml entry.

pub mod fixture;
pub mod forgejo;
pub mod github;
pub mod retry;

use crate::config::{ProviderKind, ProvidersConfig, StatsConfig};
use crate::log;
use crate::model::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::Path;

pub trait Provider {
    fn id(&self) -> &str;
    fn display(&self) -> &str;
    fn user(&self) -> &str;
    fn primary(&self) -> bool;

    fn profile(&self) -> Result<Profile>;
    /// Owner repos, sorted freshest-pushed first.
    fn repos(&self) -> Result<Vec<Repo>>;
    fn languages(&self, repo: &Repo) -> Result<LangBytes>;
    /// Daily contribution calendar (best available source).
    fn activity(&self) -> Result<Heatmap>;
    /// Live lookup of a single repo (for the flagship card). Ok(None) = absent.
    fn repo(&self, name: &str) -> Result<Option<Repo>>;

    /// Precise commit-count windows when the platform can provide them
    /// (GitHub GraphQL with a token). `None` → derived from the calendar.
    fn commit_windows(&self) -> Option<CommitWindows> {
        None
    }

    /// Per-repo pulse for the flagship card (total commits + 30-day
    /// activity). `stats` filters excluded commit authors (CI bots) out of
    /// the 30-day series.
    fn pulse(&self, _repo: &Repo, _stats: &StatsConfig) -> RepoPulse {
        RepoPulse::default()
    }
}

/// Build the provider set from config. `fixtures` switches every provider to
/// recorded JSON under that directory (offline / dry-run mode).
pub fn make_providers(
    cfg: &ProvidersConfig,
    fixtures: Option<&Path>,
) -> Vec<Box<dyn Provider>> {
    let mut out: Vec<Box<dyn Provider>> = Vec::new();
    for entry in &cfg.provider {
        if let Some(dir) = fixtures {
            match fixture::FixtureProvider::load(entry, dir) {
                Ok(p) => out.push(Box::new(p)),
                Err(e) => log::warn(&format!("fixture {}: {e}", entry.id)),
            }
            continue;
        }
        match entry.kind {
            ProviderKind::Github => out.push(Box::new(github::GitHubProvider::new(entry.clone()))),
            ProviderKind::Forgejo => {
                out.push(Box::new(forgejo::ForgejoProvider::new(entry.clone())))
            }
        }
    }
    out
}

/// Collect everything one platform offers and derive its rollup.
/// Ported from the previous engine's data-collection flow. Repositories in
/// `stats.exclude_repos` are dropped up front, so they never reach ANY
/// rollup metric (languages, LOC, stars, forks, activity, most-active).
pub fn collect(provider: &dyn Provider, stats: &StatsConfig) -> PlatformData {
    log::section(&format!("Fetching {} data", provider.display()));
    let mut pd = PlatformData {
        id: provider.id().to_string(),
        display: provider.display().to_string(),
        user: provider.user().to_string(),
        primary: provider.primary(),
        ..Default::default()
    };

    match provider.profile() {
        Ok(p) => {
            log::step("profile", &p.login, &format!("{} followers", p.followers));
            pd.profile = p;
            pd.reachable = true;
        }
        Err(e) => log::warn(&format!("profile: {e}")),
    }

    match provider.repos() {
        Ok(repos) => {
            if !repos.is_empty() {
                pd.reachable = true;
            }
            let before = repos.len();
            pd.repos = repos
                .into_iter()
                .filter(|r| !stats.repo_excluded(&r.name))
                .collect();
            let dropped = before - pd.repos.len();
            if dropped > 0 {
                log::step(
                    "excluded",
                    &dropped.to_string(),
                    "repos removed from rollups (stats.toml)",
                );
            }
        }
        Err(e) => log::warn(&format!("repos: {e}")),
    }

    let owned: Vec<Repo> = pd.repos.iter().filter(|r| !r.fork).cloned().collect();
    let r = &mut pd.rollup;
    r.repo_count = owned.len() as u64;
    r.stars = pd.repos.iter().map(|x| x.stars).sum();
    r.forks = pd.repos.iter().map(|x| x.forks).sum();
    r.watchers = pd.repos.iter().map(|x| x.watchers).sum();
    r.open_issues = pd.repos.iter().map(|x| x.open_issues).sum();
    r.open_prs = pd.repos.iter().filter_map(|x| x.open_prs).sum();

    // owned is already sorted by pushed desc → the freshest is where changes
    // flow. Skip the profile repo itself: its constant engine auto-commits
    // would otherwise always win and pin it to the top.
    let user_lower = pd.user.to_lowercase();
    r.most_active_repo = owned
        .iter()
        .find(|x| x.name.to_lowercase() != user_lower)
        .or(owned.first())
        .map(|x| x.name.clone());
    log::step(
        "repositories",
        &r.repo_count.to_string(),
        &format!("{}★  {} forks", r.stars, r.forks),
    );

    let mut langs_by_repo: BTreeMap<String, LangBytes> = BTreeMap::new();
    for repo in &owned {
        match provider.languages(repo) {
            Ok(langs) => {
                for (k, v) in &langs {
                    *r.lang_bytes.entry(k.clone()).or_insert(0) += v;
                }
                langs_by_repo.insert(normalize_repo_name(&repo.name), langs);
            }
            Err(_) => {
                if let Some(lang) = &repo.language {
                    *r.lang_bytes.entry(lang.clone()).or_insert(0) += 1;
                    let mut lb = LangBytes::new();
                    lb.insert(lang.clone(), 1);
                    langs_by_repo.insert(normalize_repo_name(&repo.name), lb);
                }
            }
        }
    }
    r.total_bytes = r.lang_bytes.values().sum();
    r.loc = r.total_bytes / AVG_BYTES_PER_LINE;
    let top = r
        .lang_bytes
        .iter()
        .max_by_key(|kv| *kv.1)
        .map(|kv| kv.0.clone())
        .unwrap_or_else(|| "—".into());
    log::step(
        "languages",
        &r.lang_bytes.len().to_string(),
        &format!("top={top}  ≈{} LOC", r.loc),
    );
    pd.langs_by_repo = langs_by_repo;

    r.commit_label = "contributions".into();
    if let Some(cw) = provider.commit_windows() {
        if cw.d7.is_some() || cw.d30.is_some() || cw.y1.is_some() {
            r.commits = cw;
            r.commit_label = "commits".into();
            log::step(
                "commits",
                &format!(
                    "{}/{}/{}",
                    format_count(r.commits.d7),
                    format_count(r.commits.d30),
                    format_count(r.commits.y1)
                ),
                "7d/30d/1y",
            );
        }
    }

    match provider.activity() {
        Ok(hm) if !hm.is_empty() => {
            r.streak = hm.streak();
            r.active_days = hm.active_days();
            r.best_day = hm.best_day();
            if r.commits.d7.is_none() {
                r.commits.d7 = Some(hm.sum_last(7));
            }
            if r.commits.d30.is_none() {
                r.commits.d30 = Some(hm.sum_last(30));
            }
            if r.commits.y1.is_none() {
                r.commits.y1 = Some(hm.sum_last(365));
            }
            log::step(
                "calendar",
                &format!("{}d", hm.days.len()),
                &format!(
                    "streak={}  active={}d  best={}",
                    r.streak, r.active_days, r.best_day
                ),
            );
            pd.heatmap = hm;
            pd.reachable = true;
        }
        Ok(_) => {}
        Err(e) => log::warn(&format!("activity: {e}")),
    }

    log::step(
        "most active",
        r.most_active_repo.as_deref().unwrap_or("—"),
        "where changes land",
    );
    pd
}

/// Combine per-platform data into one `Aggregate` (see design §2.5):
///
/// * **Mirror dedup by normalized repo name** — the same name on several
///   platforms is a mirror. Code metrics (language bytes → LOC, repo count)
///   are counted ONCE, from the copy with the largest language byte total.
/// * **Social metrics are SUMMED across every platform** — stars, forks,
///   watchers, open issues, open PRs (a mirror earns its own stars).
pub fn aggregate(platforms: Vec<PlatformData>) -> Aggregate {
    let mut combined = Rollup::default();

    // Social metrics: straight sums over every platform copy.
    for p in &platforms {
        combined.stars += p.rollup.stars;
        combined.forks += p.rollup.forks;
        combined.watchers += p.rollup.watchers;
        combined.open_issues += p.rollup.open_issues;
        combined.open_prs += p.rollup.open_prs;
    }

    // Code metrics: dedup owned repos by normalized name across platforms.
    // Canonical copy = the platform whose language byte total is largest.
    let mut groups: BTreeMap<String, Vec<(&PlatformData, &Repo)>> = BTreeMap::new();
    for p in &platforms {
        for repo in p.repos.iter().filter(|x| !x.fork) {
            groups
                .entry(normalize_repo_name(&repo.name))
                .or_default()
                .push((p, repo));
        }
    }
    combined.repo_count = groups.len() as u64;
    for (key, members) in &groups {
        let canonical = members
            .iter()
            .max_by_key(|(p, _)| {
                p.langs_by_repo
                    .get(key)
                    .map(|lb| lb.values().sum::<u64>())
                    .unwrap_or(0)
            })
            .expect("group is never empty");
        if let Some(lb) = canonical.0.langs_by_repo.get(key) {
            for (lang, bytes) in lb {
                *combined.lang_bytes.entry(lang.clone()).or_insert(0) += bytes;
            }
        }
    }
    combined.total_bytes = combined.lang_bytes.values().sum();
    combined.loc = combined.total_bytes / AVG_BYTES_PER_LINE;

    // Activity: merge calendars, sum commit windows.
    let heatmap = Heatmap::merge(platforms.iter().map(|p| &p.heatmap));
    combined.streak = heatmap.streak();
    combined.active_days = heatmap.active_days();
    combined.best_day = heatmap.best_day();
    let sum_opt = |f: fn(&CommitWindows) -> Option<u64>| -> Option<u64> {
        let vals: Vec<u64> = platforms.iter().filter_map(|p| f(&p.rollup.commits)).collect();
        if vals.is_empty() {
            None
        } else {
            Some(vals.iter().sum())
        }
    };
    combined.commits = CommitWindows {
        d7: sum_opt(|c| c.d7),
        d30: sum_opt(|c| c.d30),
        y1: sum_opt(|c| c.y1),
    };
    combined.commit_label = if platforms
        .iter()
        .filter(|p| p.reachable)
        .any(|p| p.rollup.commit_label == "contributions")
    {
        "contributions".into()
    } else {
        "commits".into()
    };

    // Most-active repo: freshest push across every platform, skipping each
    // platform's profile repo.
    let mut freshest: Option<(&str, &str)> = None; // (pushed_at, name)
    for p in &platforms {
        let user_lower = p.user.to_lowercase();
        for repo in p.repos.iter().filter(|x| !x.fork) {
            if repo.name.to_lowercase() == user_lower {
                continue;
            }
            if let Some(pushed) = repo.pushed_at.as_deref() {
                if freshest.map_or(true, |(best, _)| pushed > best) {
                    freshest = Some((pushed, &repo.name));
                }
            }
        }
    }
    combined.most_active_repo = freshest
        .map(|(_, name)| name.to_string())
        .or_else(|| {
            platforms
                .iter()
                .find_map(|p| p.rollup.most_active_repo.clone())
        });

    let followers_total = platforms.iter().map(|p| p.profile.followers).sum();
    let following_total = platforms.iter().map(|p| p.profile.following).sum();

    Aggregate {
        platforms,
        combined,
        heatmap,
        followers_total,
        following_total,
    }
}

/// Resolve one flagship project against the already-collected platform data,
/// pulling the live pulse from the preferred (or first matching) provider.
pub fn resolve_flagship(
    project: &crate::config::flagship::FlagshipProject,
    agg: &Aggregate,
    providers: &[Box<dyn Provider>],
    stats: &StatsConfig,
) -> FlagshipLive {
    let key = normalize_repo_name(&project.repo);
    let mut live = FlagshipLive {
        name: project.name.clone(),
        repo_key: key.clone(),
        site: project.site.clone(),
        tags: project.tags.clone(),
        blurb: project.blurb.clone(),
        accent: project.accent.clone(),
        ..Default::default()
    };

    // Mirror rule: stars / forks / open items summed across every platform copy.
    let mut hits: Vec<(&PlatformData, &Repo)> = Vec::new();
    for p in &agg.platforms {
        if let Some(repo) = p.repos.iter().find(|r| normalize_repo_name(&r.name) == key) {
            hits.push((p, repo));
            live.stars += repo.stars;
            live.forks += repo.forks;
            live.open_items += repo.open_items();
        }
    }
    if hits.is_empty() {
        return live;
    }

    // Headline copy: the preferred platform when it has the repo, else the
    // copy with the most language bytes.
    let headline = project
        .prefer
        .as_deref()
        .and_then(|id| hits.iter().find(|(p, _)| p.id == id))
        .copied()
        .unwrap_or_else(|| {
            *hits
                .iter()
                .max_by_key(|(p, _)| {
                    p.langs_by_repo
                        .get(&key)
                        .map(|lb| lb.values().sum::<u64>())
                        .unwrap_or(0)
                })
                .expect("hits is non-empty")
        });
    live.source = Some(headline.0.display.clone());
    live.repo = Some(headline.1.clone());
    live.langs = headline
        .0
        .langs_by_repo
        .get(&key)
        .cloned()
        .unwrap_or_default();
    if let Some(provider) = providers.iter().find(|pr| pr.id() == headline.0.id) {
        live.pulse = provider.pulse(headline.1, stats);
    }
    live
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::flagship::FlagshipProject;
    use crate::config::ProviderEntry;

    fn fixtures_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures")
    }

    fn entry(id: &str, kind: ProviderKind, display: &str, user: &str, primary: bool) -> ProviderEntry {
        ProviderEntry {
            id: id.into(),
            kind,
            display: display.into(),
            base_url: None,
            user: user.into(),
            token_env: None,
            primary,
        }
    }

    fn fixture_providers() -> Vec<Box<dyn Provider>> {
        let entries = [
            entry("vai-git", ProviderKind::Forgejo, "VAI Git", "VAI_PROG", true),
            entry("github", ProviderKind::Github, "GitHub", "Vadim-Khristenko", false),
            entry("codeberg", ProviderKind::Forgejo, "Codeberg", "VAI_PROG", false),
        ];
        entries
            .iter()
            .map(|e| {
                Box::new(fixture::FixtureProvider::load(e, &fixtures_dir()).unwrap())
                    as Box<dyn Provider>
            })
            .collect()
    }

    fn no_stats() -> StatsConfig {
        StatsConfig::default()
    }

    /// The real seeded exclusions: profile repo + CI bot identities.
    fn seeded_stats() -> StatsConfig {
        StatsConfig {
            exclude_repos: vec!["Vadim-Khristenko".into()],
            exclude_commit_authors: vec![
                "actions@git.vai-rice.space".into(),
                "VIA GIT".into(),
                "github-actions[bot]".into(),
            ],
        }
    }

    fn fixture_aggregate() -> (Aggregate, Vec<Box<dyn Provider>>) {
        let providers = fixture_providers();
        let platforms: Vec<PlatformData> = providers
            .iter()
            .map(|p| collect(p.as_ref(), &no_stats()))
            .collect();
        (aggregate(platforms), providers)
    }

    #[test]
    fn collect_derives_platform_rollup() {
        let providers = fixture_providers();
        let gh = providers.iter().find(|p| p.id() == "github").unwrap();
        let pd = collect(gh.as_ref(), &no_stats());
        assert!(pd.reachable);
        assert_eq!(pd.rollup.repo_count, 7); // fork excluded
        assert_eq!(pd.rollup.stars, 353); // summed incl. the fork
        assert_eq!(pd.rollup.commit_label, "commits"); // GraphQL windows present
        assert_eq!(pd.rollup.commits.d7, Some(26));
        // Profile repo is skipped as "most active" despite freshest push.
        assert_eq!(pd.rollup.most_active_repo.as_deref(), Some("AmneziaWG-Architect"));
        assert!(pd.rollup.streak >= 5); // fixture keeps last 5 days active
    }

    #[test]
    fn aggregate_sums_social_metrics_across_mirrors() {
        let (agg, _) = fixture_aggregate();
        // Stars: every platform copy counts (github 353 + codeberg 16 + vai-git 16).
        assert_eq!(agg.combined.stars, 385);
        // Followers are people: summed across platforms.
        assert_eq!(agg.followers_total, 61 + 7 + 4);
    }

    #[test]
    fn aggregate_counts_code_metrics_once_per_mirror_group() {
        let (agg, _) = fixture_aggregate();
        // 10 distinct owned repo names across the three platforms.
        assert_eq!(agg.combined.repo_count, 10);
        // aethelgard's canonical copy is vai-git (largest byte total), so its
        // Cython bytes appear exactly once.
        assert_eq!(agg.combined.lang_bytes.get("Cython"), Some(&36000));
        // Python = profile repo (github, 41k) + aethelgard canonical (512k) —
        // NOT the smaller github aethelgard mirror (60k).
        assert_eq!(agg.combined.lang_bytes.get("Python"), Some(&553_000));
        // TypeScript must not be triple-counted across the three mirrors.
        let ts_all: u64 = agg
            .platforms
            .iter()
            .map(|p| p.rollup.lang_bytes.get("TypeScript").copied().unwrap_or(0))
            .sum();
        let ts_combined = agg.combined.lang_bytes.get("TypeScript").copied().unwrap();
        assert!(ts_combined < ts_all, "{ts_combined} !< {ts_all}");
        assert_eq!(agg.combined.loc, agg.combined.total_bytes / AVG_BYTES_PER_LINE);
    }

    #[test]
    fn aggregate_merges_activity_and_commit_windows() {
        let (agg, _) = fixture_aggregate();
        // Commit windows: github's GraphQL numbers plus calendar-derived ones.
        let d7 = agg.combined.commits.d7.unwrap();
        assert!(d7 >= 26, "combined 7d ≥ github's precise 26, got {d7}");
        // Two platforms fall back to calendar counts → combined label degrades.
        assert_eq!(agg.combined.commit_label, "contributions");
        assert!(agg.heatmap.days.len() >= 140);
        assert!(agg.combined.streak >= 5);
    }

    #[test]
    fn flagship_resolution_follows_mirror_rules() {
        let (agg, providers) = fixture_aggregate();
        let project = FlagshipProject {
            name: "AmneziaWG Architect".into(),
            repo: "AmneziaWG-Architect".into(),
            prefer: Some("github".into()),
            site: Some("https://architect.vai-rice.space".into()),
            tags: vec!["vpn".into()],
            blurb: String::new(),
            accent: None,
        };
        let live = resolve_flagship(&project, &agg, &providers, &no_stats());
        assert_eq!(live.stars, 218 + 11 + 4);
        assert_eq!(live.forks, 31 + 2 + 1);
        // github open_issues includes PRs (14); forgejo copies add issues+PRs.
        assert_eq!(live.open_items, 14 + (1 + 1) + (2 + 1));
        assert_eq!(live.source.as_deref(), Some("GitHub"));
        assert_eq!(live.pulse.total_commits, Some(347));
        assert!(live.langs.contains_key("TypeScript"));
    }

    #[test]
    fn every_configured_flagship_resolves_from_fixtures() {
        // Guards the config ↔ fixture contract: `--fixtures` runs (and the
        // offline tests) must resolve live stats for EVERY flagship project.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config");
        let cfg = crate::config::Config::load(&dir).expect("config loads");
        let (agg, providers) = fixture_aggregate();
        for project in &cfg.flagship.project {
            let live = resolve_flagship(project, &agg, &providers, &no_stats());
            assert!(
                live.repo.is_some(),
                "flagship '{}' (repo {}) has no fixture entry — offline runs would render a dead row",
                project.name,
                project.repo
            );
        }
        // The Kumir 3 entry specifically headlines from the GitHub fixture.
        let kumir = cfg
            .flagship
            .project
            .iter()
            .find(|p| p.name == "Kumir 3")
            .expect("Kumir 3 configured");
        let live = resolve_flagship(kumir, &agg, &providers, &no_stats());
        assert_eq!(live.source.as_deref(), Some("GitHub"));
        assert_eq!(live.stars, 46);
        assert_eq!(live.pulse.total_commits, Some(128));
        assert!(live.langs.contains_key("Rust"));
    }

    #[test]
    fn flagship_missing_repo_degrades_gracefully() {
        let (agg, providers) = fixture_aggregate();
        let project = FlagshipProject {
            name: "Ghost".into(),
            repo: "does-not-exist".into(),
            prefer: None,
            site: None,
            tags: vec![],
            blurb: String::new(),
            accent: None,
        };
        let live = resolve_flagship(&project, &agg, &providers, &no_stats());
        assert!(live.repo.is_none());
        assert_eq!(live.stars, 0);
    }

    #[test]
    fn excluded_repo_never_reaches_any_rollup_metric() {
        let providers = fixture_providers();
        let gh = providers.iter().find(|p| p.id() == "github").unwrap();
        let base = collect(gh.as_ref(), &no_stats());
        let pd = collect(gh.as_ref(), &seeded_stats());
        // The profile repo (6 repos → 5) disappears from every rollup:
        assert_eq!(pd.rollup.repo_count, base.rollup.repo_count - 1);
        assert_eq!(pd.rollup.stars, base.rollup.stars - 9); // its 9 stars gone
        assert!(pd.rollup.total_bytes < base.rollup.total_bytes); // langs gone
        assert!(pd.repos.iter().all(|r| r.name != "Vadim-Khristenko"));
        // And the aggregate view can't resurrect it.
        let platforms: Vec<PlatformData> = providers
            .iter()
            .map(|p| collect(p.as_ref(), &seeded_stats()))
            .collect();
        let agg = aggregate(platforms);
        assert_eq!(agg.combined.repo_count, 9); // 10 distinct − the profile repo
    }

    #[test]
    fn excluded_bot_authors_are_dropped_from_commit_series() {
        let (agg, providers) = fixture_aggregate();
        let project = FlagshipProject {
            name: "The Wall Dev".into(),
            repo: "TheWall".into(),
            prefer: Some("github".into()),
            site: None,
            tags: vec![],
            blurb: String::new(),
            accent: None,
        };
        // Unfiltered: all 4 fixture commits (2 human + VIA GIT + gh-actions).
        let all = resolve_flagship(&project, &agg, &providers, &no_stats());
        assert_eq!(all.pulse.daily_30.iter().sum::<u32>(), 4);
        // Filtered: only the two human commits remain.
        let human = resolve_flagship(&project, &agg, &providers, &seeded_stats());
        assert_eq!(human.pulse.daily_30.iter().sum::<u32>(), 2);
        assert_eq!(human.pulse.total_commits, Some(152)); // repo-wide total intact
    }
}
