//! Core data model shared by providers, the aggregator and the cards.
//!
//! Lines of code are estimated from total language bytes (always available and
//! immediate) — per-commit statistics endpoints are too flaky to depend on for
//! a live profile.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Rough constant for the LOC estimate (bytes of source per line).
pub const AVG_BYTES_PER_LINE: u64 = 34;

/// Language name → byte count.
pub type LangBytes = BTreeMap<String, u64>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Profile {
    pub login: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub followers: u64,
    #[serde(default)]
    pub following: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Repo {
    pub name: String,
    #[serde(default)]
    pub fork: bool,
    #[serde(default)]
    pub stars: u64,
    #[serde(default)]
    pub forks: u64,
    #[serde(default)]
    pub watchers: u64,
    /// GitHub reports issues+PRs combined here; Forgejo reports issues only.
    #[serde(default)]
    pub open_issues: u64,
    /// Forgejo exposes a separate open-PR counter; GitHub folds PRs into
    /// `open_issues`, so this stays `None` there.
    #[serde(default)]
    pub open_prs: Option<u64>,
    #[serde(default)]
    pub language: Option<String>,
    /// ISO-8601 push timestamp (sortable lexicographically).
    #[serde(default)]
    pub pushed_at: Option<String>,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl Repo {
    /// Open issues + open PRs combined, regardless of platform accounting.
    pub fn open_items(&self) -> u64 {
        self.open_issues + self.open_prs.unwrap_or(0)
    }
}

/// Mirror-dedup key: lowercase, trailing `.git` stripped.
pub fn normalize_repo_name(name: &str) -> String {
    let lower = name.to_lowercase();
    lower.strip_suffix(".git").unwrap_or(&lower).to_string()
}

/// Daily contribution calendar, sorted ascending by date.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Heatmap {
    pub days: Vec<(NaiveDate, u32)>,
}

impl Heatmap {
    pub fn from_map(map: BTreeMap<NaiveDate, u32>) -> Self {
        Heatmap {
            days: map.into_iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// Current streak in days; a trailing zero (today so far) doesn't break it.
    pub fn streak(&self) -> u64 {
        let seq: Vec<u32> = self.days.iter().map(|d| d.1).collect();
        if seq.is_empty() {
            return 0;
        }
        let mut i = seq.len() as i64 - 1;
        if seq[i as usize] == 0 {
            i -= 1;
        }
        let mut s = 0;
        while i >= 0 && seq[i as usize] > 0 {
            s += 1;
            i -= 1;
        }
        s
    }

    pub fn active_days(&self) -> u64 {
        self.days.iter().filter(|d| d.1 > 0).count() as u64
    }

    pub fn best_day(&self) -> u64 {
        self.days.iter().map(|d| d.1 as u64).max().unwrap_or(0)
    }

    /// Date of the single best day (most contributions).
    pub fn best_day_date(&self) -> Option<NaiveDate> {
        self.days
            .iter()
            .max_by_key(|d| d.1)
            .filter(|d| d.1 > 0)
            .map(|d| d.0)
    }

    pub fn sum_last(&self, n: usize) -> u64 {
        let len = self.days.len();
        self.days[len.saturating_sub(n)..]
            .iter()
            .map(|d| d.1 as u64)
            .sum()
    }

    pub fn total(&self) -> u64 {
        self.days.iter().map(|d| d.1 as u64).sum()
    }

    /// Contribution totals bucketed by weekday, Monday = index 0.
    pub fn weekday_totals(&self) -> [u64; 7] {
        let mut out = [0u64; 7];
        for (date, count) in &self.days {
            out[date.weekday().num_days_from_monday() as usize] += *count as u64;
        }
        out
    }

    /// Merge several calendars by summing per-date counts.
    pub fn merge<'a>(maps: impl IntoIterator<Item = &'a Heatmap>) -> Heatmap {
        let mut acc: BTreeMap<NaiveDate, u32> = BTreeMap::new();
        for hm in maps {
            for (date, count) in &hm.days {
                *acc.entry(*date).or_insert(0) += count;
            }
        }
        Heatmap::from_map(acc)
    }
}

/// Per-repo live pulse used by the flagship card.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepoPulse {
    #[serde(default)]
    pub total_commits: Option<u64>,
    /// Commits per day over the last 30 days (oldest → newest).
    #[serde(default)]
    pub daily_30: Vec<u32>,
}

/// Commit-count windows (7d / 30d / 1y). `None` = unknown for that window.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitWindows {
    pub d7: Option<u64>,
    pub d30: Option<u64>,
    pub y1: Option<u64>,
}

/// One platform's fully-derived statistics.
#[derive(Debug, Clone, Default)]
pub struct Rollup {
    /// Owned, non-fork repositories.
    pub repo_count: u64,
    pub stars: u64,
    pub forks: u64,
    pub watchers: u64,
    pub open_issues: u64,
    pub open_prs: u64,
    pub lang_bytes: LangBytes,
    pub total_bytes: u64,
    pub loc: u64,
    pub commits: CommitWindows,
    pub streak: u64,
    pub active_days: u64,
    pub best_day: u64,
    /// "commits" when precise, "contributions" for the calendar fallback.
    pub commit_label: String,
    pub most_active_repo: Option<String>,
}

/// Everything collected from a single provider, plus its rollup.
#[derive(Debug, Clone, Default)]
pub struct PlatformData {
    pub id: String,
    pub display: String,
    pub user: String,
    pub primary: bool,
    pub reachable: bool,
    pub profile: Profile,
    pub repos: Vec<Repo>,
    /// Language bytes per owned repo (normalized name → langs).
    pub langs_by_repo: BTreeMap<String, LangBytes>,
    pub heatmap: Heatmap,
    pub rollup: Rollup,
}

/// The combined view over every platform.
#[derive(Debug, Clone, Default)]
pub struct Aggregate {
    /// Ordered as configured in providers.toml.
    pub platforms: Vec<PlatformData>,
    pub combined: Rollup,
    /// All platform calendars merged (per-date sums).
    pub heatmap: Heatmap,
    /// Followers summed across platforms (people, not mirrors).
    pub followers_total: u64,
    pub following_total: u64,
}

impl Aggregate {
    pub fn platform(&self, id: &str) -> Option<&PlatformData> {
        self.platforms.iter().find(|p| p.id == id)
    }

    /// The primary platform if configured (else the first reachable one).
    pub fn primary(&self) -> Option<&PlatformData> {
        self.platforms
            .iter()
            .find(|p| p.primary && p.reachable)
            .or_else(|| self.platforms.iter().find(|p| p.reachable))
    }
}

/// A flagship project resolved against live platform data.
#[derive(Debug, Clone, Default)]
pub struct FlagshipLive {
    /// Display name from config.
    pub name: String,
    pub repo_key: String,
    pub site: Option<String>,
    pub tags: Vec<String>,
    pub blurb: String,
    pub accent: Option<String>,
    /// Which platform the live stats came from (display name).
    pub source: Option<String>,
    pub repo: Option<Repo>,
    /// Combined across platforms (mirror rule: stars/forks/open summed).
    pub stars: u64,
    pub forks: u64,
    pub open_items: u64,
    pub langs: LangBytes,
    pub pulse: RepoPulse,
}

pub fn format_count(n: Option<u64>) -> String {
    match n {
        None => "—".into(),
        Some(n) => {
            if n >= 1_000_000 {
                let s = format!("{:.1}M", n as f64 / 1_000_000.0);
                s.replace(".0M", "M")
            } else if n >= 1000 {
                let s = format!("{:.1}k", n as f64 / 1000.0);
                s.replace(".0k", "k")
            } else {
                n.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strips_git_and_lowers() {
        assert_eq!(normalize_repo_name("The-Wall.GIT"), "the-wall"); // lowercased before strip
        assert_eq!(normalize_repo_name("The-Wall.git"), "the-wall");
        assert_eq!(normalize_repo_name("AmneziaWG-Architect"), "amneziawg-architect");
    }

    #[test]
    fn heatmap_streak_allows_trailing_zero() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        let hm = Heatmap {
            days: vec![
                (d("2026-07-14"), 0),
                (d("2026-07-15"), 3),
                (d("2026-07-16"), 1),
                (d("2026-07-17"), 2),
                (d("2026-07-18"), 5),
                (d("2026-07-19"), 0),
            ],
        };
        assert_eq!(hm.streak(), 4);
        assert_eq!(hm.active_days(), 4);
        assert_eq!(hm.best_day(), 5);
        assert_eq!(hm.sum_last(3), 7);
    }

    #[test]
    fn heatmap_streak_broken_by_gap() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        let hm = Heatmap {
            days: vec![(d("2026-07-17"), 2), (d("2026-07-18"), 0), (d("2026-07-19"), 4)],
        };
        assert_eq!(hm.streak(), 1);
    }

    #[test]
    fn heatmap_merge_sums_per_date() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        let a = Heatmap {
            days: vec![(d("2026-07-18"), 2), (d("2026-07-19"), 1)],
        };
        let b = Heatmap {
            days: vec![(d("2026-07-19"), 3), (d("2026-07-20"), 7)],
        };
        let m = Heatmap::merge([&a, &b]);
        assert_eq!(
            m.days,
            vec![(d("2026-07-18"), 2), (d("2026-07-19"), 4), (d("2026-07-20"), 7)]
        );
    }

    #[test]
    fn format_count_scales() {
        assert_eq!(format_count(None), "—");
        assert_eq!(format_count(Some(999)), "999");
        assert_eq!(format_count(Some(1500)), "1.5k");
        assert_eq!(format_count(Some(2000)), "2k");
        assert_eq!(format_count(Some(2_400_000)), "2.4M");
    }
}
