//! GitHub provider: REST + GraphQL contributions + tokenless calendar scrape.
//!
//! Sources, with graceful degradation (behavioral port of the previous engine):
//!   * REST    /users, /users/{u}/repos, /repos/{o}/{r}/languages
//!   * GraphQL contributionsCollection (precise commit counts; needs a token)
//!   * HTML scrape of github.com/users/{u}/contributions (works tokenless)

use super::Provider;
use crate::config::ProviderEntry;
use crate::log;
use crate::model::*;
use anyhow::{anyhow, Context, Result};
use chrono::{NaiveDate, Utc};
use reqwest::blocking::Client;
use std::collections::BTreeMap;
use std::time::Duration;

const API: &str = "https://api.github.com";

pub struct GitHubProvider {
    entry: ProviderEntry,
    token: Option<String>,
    client: Client,
}

impl GitHubProvider {
    pub fn new(entry: ProviderEntry) -> Self {
        // GH_TOKEN (configured) with GITHUB_TOKEN as an ambient CI fallback.
        let token = entry
            .token()
            .or_else(|| std::env::var("GITHUB_TOKEN").ok().filter(|v| !v.is_empty()));
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("vai-profile-engine")
            .build()
            .expect("reqwest client");
        GitHubProvider {
            entry,
            token,
            client,
        }
    }

    fn rest(&self, path: &str) -> Result<serde_json::Value> {
        let mut req = self
            .client
            .get(format!("{API}{path}"))
            .header("Accept", "application/vnd.github+json");
        if let Some(tok) = &self.token {
            req = req.header("Authorization", format!("Bearer {tok}"));
        }
        let resp = super::retry::send_retrying(req).with_context(|| format!("GET {path}"))?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!("GET {path}: HTTP {status}"));
        }
        resp.json().with_context(|| format!("GET {path}: bad JSON"))
    }

    fn graphql(&self, query: &str, variables: serde_json::Value) -> Option<serde_json::Value> {
        let tok = self.token.as_ref()?;
        let body = serde_json::json!({ "query": query, "variables": variables });
        let req = self
            .client
            .post(format!("{API}/graphql"))
            .header("Authorization", format!("Bearer {tok}"))
            .json(&body);
        match super::retry::send_retrying(req)
            .and_then(|r| r.error_for_status().map_err(Into::into))
            .and_then(|r| r.json().map_err(Into::into))
        {
            Ok(v) => Some(v),
            Err(e) => {
                log::warn(&format!("graphql: {e}"));
                None
            }
        }
    }

    fn map_repo(v: &serde_json::Value) -> Repo {
        let s = |k: &str| v.get(k).and_then(|x| x.as_str()).map(|x| x.to_string());
        let n = |k: &str| v.get(k).and_then(|x| x.as_u64()).unwrap_or(0);
        Repo {
            name: s("name").unwrap_or_default(),
            fork: v.get("fork").and_then(|x| x.as_bool()).unwrap_or(false),
            stars: n("stargazers_count"),
            forks: n("forks_count"),
            // The repo-list API has no true subscriber count (watchers_count
            // mirrors stars there); a single-repo GET refines it below.
            watchers: v.get("subscribers_count").and_then(|x| x.as_u64()).unwrap_or(0),
            // GitHub folds open PRs into open_issues_count.
            open_issues: n("open_issues_count"),
            open_prs: None,
            language: s("language"),
            pushed_at: s("pushed_at"),
            html_url: s("html_url").unwrap_or_default(),
            description: s("description"),
        }
    }
}

impl Provider for GitHubProvider {
    fn id(&self) -> &str {
        &self.entry.id
    }
    fn display(&self) -> &str {
        &self.entry.display
    }
    fn user(&self) -> &str {
        &self.entry.user
    }
    fn primary(&self) -> bool {
        self.entry.primary
    }

    fn profile(&self) -> Result<Profile> {
        let v = self.rest(&format!("/users/{}", self.entry.user))?;
        Ok(Profile {
            login: v
                .get("login")
                .and_then(|x| x.as_str())
                .unwrap_or(&self.entry.user)
                .to_string(),
            name: v.get("name").and_then(|x| x.as_str()).map(|x| x.into()),
            followers: v.get("followers").and_then(|x| x.as_u64()).unwrap_or(0),
            following: v.get("following").and_then(|x| x.as_u64()).unwrap_or(0),
        })
    }

    fn repos(&self) -> Result<Vec<Repo>> {
        let mut out = Vec::new();
        for page in 1..=10 {
            let chunk = match self.rest(&format!(
                "/users/{}/repos?per_page=100&page={page}&type=owner&sort=pushed",
                self.entry.user
            )) {
                Ok(v) => v,
                Err(e) => {
                    log::warn(&format!("repos p{page}: {e}"));
                    break;
                }
            };
            let arr = chunk.as_array().cloned().unwrap_or_default();
            if arr.is_empty() {
                break;
            }
            let len = arr.len();
            out.extend(arr.iter().map(Self::map_repo));
            if len < 100 {
                break;
            }
        }
        Ok(out)
    }

    fn languages(&self, repo: &Repo) -> Result<LangBytes> {
        let v = self.rest(&format!("/repos/{}/{}/languages", self.entry.user, repo.name))?;
        let mut out = LangBytes::new();
        if let Some(map) = v.as_object() {
            for (k, val) in map {
                out.insert(k.clone(), val.as_u64().unwrap_or(0));
            }
        }
        Ok(out)
    }

    fn commit_windows(&self) -> Option<CommitWindows> {
        self.token.as_ref()?;
        let q = "query($login:String!,$from:DateTime!,$to:DateTime!){\n\
                 user(login:$login){ contributionsCollection(from:$from,to:$to){ totalCommitContributions } } }";
        let now = Utc::now();
        let fetch = |days: i64| -> Option<u64> {
            let from = now - chrono::Duration::days(days);
            let vars = serde_json::json!({
                "login": self.entry.user,
                "from": from.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                "to": now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            });
            self.graphql(q, vars)?
                .pointer("/data/user/contributionsCollection/totalCommitContributions")?
                .as_u64()
        };
        let cw = CommitWindows {
            d7: fetch(7),
            d30: fetch(30),
            y1: fetch(365),
        };
        if cw.d7.is_none() && cw.d30.is_none() && cw.y1.is_none() {
            None
        } else {
            Some(cw)
        }
    }

    fn activity(&self) -> Result<Heatmap> {
        let html = self
            .client
            .get(format!(
                "https://github.com/users/{}/contributions",
                self.entry.user
            ))
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .context("contributions scrape")?
            .text()
            .context("contributions scrape: body")?;
        Ok(parse_contributions(&html))
    }

    fn repo(&self, name: &str) -> Result<Option<Repo>> {
        match self.rest(&format!("/repos/{}/{name}", self.entry.user)) {
            Ok(v) => Ok(Some(Self::map_repo(&v))),
            Err(e) if e.to_string().contains("404") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn pulse(&self, repo: &Repo, stats: &crate::config::StatsConfig) -> RepoPulse {
        let mut pulse = RepoPulse::default();
        // Total commits: per_page=1 + the rel="last" page number of the Link
        // header — the cheapest exact count GitHub offers. (Repo-wide; the
        // author filter applies to the 30-day series below.)
        let url = format!(
            "{API}/repos/{}/{}/commits?per_page=1",
            self.entry.user, repo.name
        );
        let mut req = self.client.get(&url);
        if let Some(tok) = &self.token {
            req = req.header("Authorization", format!("Bearer {tok}"));
        }
        if let Ok(resp) = super::retry::send_retrying(req) {
            if resp.status().is_success() {
                let link = resp
                    .headers()
                    .get("link")
                    .and_then(|h| h.to_str().ok())
                    .map(|s| s.to_string());
                pulse.total_commits = link
                    .as_deref()
                    .and_then(parse_last_page)
                    .or(Some(1));
            }
        }
        // 30-day activity: list commits since 30 days ago, drop excluded
        // authors (CI bots), bucket the rest per day.
        let since = (Utc::now() - chrono::Duration::days(30))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        if let Ok(v) = self.rest(&format!(
            "/repos/{}/{}/commits?since={since}&per_page=100",
            self.entry.user, repo.name
        )) {
            let commits: Vec<CommitMeta> = v
                .as_array()
                .map(|arr| arr.iter().map(commit_meta_from_github).collect())
                .unwrap_or_default();
            pulse.daily_30 = bucket_commit_days(stats.filter_commit_dates(&commits));
        }
        pulse
    }
}

/// Authorship metadata from one GitHub commit-list item.
pub fn commit_meta_from_github(c: &serde_json::Value) -> CommitMeta {
    let s = |ptr: &str| {
        c.pointer(ptr)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    CommitMeta {
        date: s("/commit/author/date"),
        author_email: s("/commit/author/email"),
        author_name: s("/commit/author/name"),
        author_login: s("/author/login"),
        days_ago: None,
    }
}

/// Extract the rel="last" page number from a GitHub Link header.
pub fn parse_last_page(link: &str) -> Option<u64> {
    let seg = link.split(',').find(|s| s.contains("rel=\"last\""))?;
    let url = seg.split('<').nth(1)?.split('>').next()?;
    let page = url
        .split(['?', '&'])
        .find_map(|kv| kv.strip_prefix("page="))?;
    page.parse().ok()
}

/// Bucket ISO commit timestamps into a 30-slot per-day series (oldest→newest).
pub fn bucket_commit_days(dates: Vec<String>) -> Vec<u32> {
    let today = Utc::now().date_naive();
    let mut out = vec![0u32; 30];
    for iso in dates {
        if let Ok(d) = NaiveDate::parse_from_str(&iso[..10.min(iso.len())], "%Y-%m-%d") {
            let age = (today - d).num_days();
            if (0..30).contains(&age) {
                out[29 - age as usize] += 1;
            }
        }
    }
    out
}

// ── Tokenless calendar scrape ───────────────────────────────────────────────

fn tag_attr(tag: &str, name: &str) -> Option<String> {
    let needle = format!(" {name}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')? + start;
    Some(tag[start..end].to_string())
}

/// Parse GitHub's contribution-calendar HTML into daily counts.
/// Tooltips carry the counts; `<td data-date>` cells carry the dates.
pub fn parse_contributions(html: &str) -> Heatmap {
    // Pass 1: tool-tip counts keyed by cell id.
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut rest = html;
    while let Some(open) = rest.find("<tool-tip") {
        let tail = &rest[open..];
        let Some(tag_end) = tail.find('>') else { break };
        let tag = &tail[..tag_end];
        let inner_start = tag_end + 1;
        let Some(close) = tail[inner_start..].find("</tool-tip>") else {
            break;
        };
        let text = tail[inner_start..inner_start + close].trim().to_string();
        if let Some(cell_id) = tag_attr(tag, "for") {
            let count = if text.starts_with("No ") {
                0
            } else {
                let num: String = text
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == ',')
                    .filter(|c| c.is_ascii_digit())
                    .collect();
                num.parse().unwrap_or(0)
            };
            counts.insert(cell_id, count);
        }
        rest = &tail[inner_start + close..];
    }

    // Pass 2: calendar cells with data-date + id.
    let mut days: BTreeMap<NaiveDate, u32> = BTreeMap::new();
    let mut rest = html;
    while let Some(open) = rest.find("<td") {
        let tail = &rest[open..];
        let Some(tag_end) = tail.find('>') else { break };
        let tag = &tail[..tag_end];
        if let (Some(date_s), Some(id)) = (tag_attr(tag, "data-date"), tag_attr(tag, "id")) {
            if let Ok(date) = NaiveDate::parse_from_str(&date_s, "%Y-%m-%d") {
                days.insert(date, counts.get(&id).copied().unwrap_or(0));
            }
        }
        rest = &tail[tag_end..];
    }
    Heatmap::from_map(days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_contribution_calendar_fixture() {
        let html = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("fixtures/github_contributions.html"),
        )
        .unwrap();
        let hm = parse_contributions(&html);
        assert_eq!(hm.days.len(), 5);
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        assert_eq!(hm.days[0], (d("2026-07-13"), 0)); // "No contributions"
        assert_eq!(hm.days[1], (d("2026-07-14"), 3));
        assert_eq!(hm.days[2], (d("2026-07-15"), 1204)); // comma in "1,204"
        assert_eq!(hm.days[3], (d("2026-07-16"), 0)); // cell without tooltip
        assert_eq!(hm.days[4], (d("2026-07-17"), 7));
    }

    #[test]
    fn parses_link_header_last_page() {
        let link = "<https://api.github.com/repos/u/r/commits?per_page=1&page=2>; rel=\"next\", \
                    <https://api.github.com/repos/u/r/commits?per_page=1&page=347>; rel=\"last\"";
        assert_eq!(parse_last_page(link), Some(347));
        assert_eq!(parse_last_page("<x>; rel=\"next\""), None);
    }

    #[test]
    fn buckets_commit_days_into_30_slots() {
        let today = Utc::now().date_naive();
        let iso = |days_ago: i64| {
            (today - chrono::Duration::days(days_ago))
                .format("%Y-%m-%dT12:00:00Z")
                .to_string()
        };
        let out = bucket_commit_days(vec![iso(0), iso(0), iso(29), iso(31)]);
        assert_eq!(out.len(), 30);
        assert_eq!(out[29], 2);
        assert_eq!(out[0], 1);
        assert_eq!(out.iter().sum::<u32>(), 3); // the 31-day-old one is dropped
    }
}
