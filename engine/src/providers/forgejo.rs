//! Forgejo / Gitea-compatible provider (`/api/v1`). One struct, any host:
//! Codeberg and a self-hosted instance are the same code with a different
//! `base_url`.

use super::Provider;
use crate::config::ProviderEntry;
use crate::log;
use crate::model::*;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use std::collections::BTreeMap;
use std::time::Duration;

pub struct ForgejoProvider {
    entry: ProviderEntry,
    base_url: String,
    token: Option<String>,
    client: Client,
}

impl ForgejoProvider {
    pub fn new(entry: ProviderEntry) -> Self {
        let base_url = entry
            .base_url
            .clone()
            .unwrap_or_default()
            .trim_end_matches('/')
            .to_string();
        let token = entry.token();
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("vai-profile-engine")
            .build()
            .expect("reqwest client");
        ForgejoProvider {
            entry,
            base_url,
            token,
            client,
        }
    }

    fn get(&self, path: &str) -> Result<reqwest::blocking::Response> {
        let mut req = self.client.get(format!("{}/api/v1{path}", self.base_url));
        if let Some(tok) = &self.token {
            req = req.header("Authorization", format!("token {tok}"));
        }
        let resp = super::retry::send_retrying(req).with_context(|| format!("GET {path}"))?;
        if !resp.status().is_success() {
            return Err(anyhow!("GET {path}: HTTP {}", resp.status()));
        }
        Ok(resp)
    }

    fn json(&self, path: &str) -> Result<serde_json::Value> {
        self.get(path)?
            .json()
            .with_context(|| format!("GET {path}: bad JSON"))
    }

    fn map_repo(v: &serde_json::Value) -> Repo {
        let s = |k: &str| v.get(k).and_then(|x| x.as_str()).map(|x| x.to_string());
        let n = |k: &str| v.get(k).and_then(|x| x.as_u64()).unwrap_or(0);
        Repo {
            name: s("name").unwrap_or_default(),
            fork: v.get("fork").and_then(|x| x.as_bool()).unwrap_or(false),
            stars: n("stars_count"),
            forks: n("forks_count"),
            watchers: n("watchers_count"),
            // Forgejo keeps issues and PRs separate — exactly what we want.
            open_issues: n("open_issues_count"),
            open_prs: Some(n("open_pr_counter")),
            language: s("language").filter(|l| !l.is_empty()),
            pushed_at: s("updated_at"),
            html_url: s("html_url").unwrap_or_default(),
            description: s("description"),
        }
    }
}

impl Provider for ForgejoProvider {
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
        let v = self.json(&format!("/users/{}", self.entry.user))?;
        Ok(Profile {
            login: v
                .get("login")
                .and_then(|x| x.as_str())
                .unwrap_or(&self.entry.user)
                .to_string(),
            name: v
                .get("full_name")
                .and_then(|x| x.as_str())
                .filter(|s| !s.is_empty())
                .map(|x| x.into()),
            followers: v.get("followers_count").and_then(|x| x.as_u64()).unwrap_or(0),
            following: v.get("following_count").and_then(|x| x.as_u64()).unwrap_or(0),
        })
    }

    fn repos(&self) -> Result<Vec<Repo>> {
        let mut out = Vec::new();
        for page in 1..=10 {
            let chunk = match self.json(&format!(
                "/users/{}/repos?page={page}&limit=50",
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
            if len < 50 {
                break;
            }
        }
        // Freshest-pushed first, matching the GitHub ordering contract.
        out.sort_by(|a, b| b.pushed_at.cmp(&a.pushed_at));
        Ok(out)
    }

    fn languages(&self, repo: &Repo) -> Result<LangBytes> {
        let v = self.json(&format!(
            "/repos/{}/{}/languages",
            self.entry.user, repo.name
        ))?;
        let mut out = LangBytes::new();
        if let Some(map) = v.as_object() {
            for (k, val) in map {
                out.insert(k.clone(), val.as_u64().unwrap_or(0));
            }
        }
        Ok(out)
    }

    fn activity(&self) -> Result<Heatmap> {
        // [{ "timestamp": <unix-seconds>, "contributions": n }, …]
        let v = self.json(&format!("/users/{}/heatmap", self.entry.user))?;
        let mut map: BTreeMap<chrono::NaiveDate, u32> = BTreeMap::new();
        for item in v.as_array().cloned().unwrap_or_default() {
            let ts = item.get("timestamp").and_then(|x| x.as_i64()).unwrap_or(0);
            let count = item
                .get("contributions")
                .and_then(|x| x.as_u64())
                .unwrap_or(0) as u32;
            if let Some(dt) = DateTime::<Utc>::from_timestamp(ts, 0) {
                *map.entry(dt.date_naive()).or_insert(0) += count;
            }
        }
        Ok(Heatmap::from_map(map))
    }

    fn repo(&self, name: &str) -> Result<Option<Repo>> {
        match self.json(&format!("/repos/{}/{name}", self.entry.user)) {
            Ok(v) => Ok(Some(Self::map_repo(&v))),
            Err(e) if e.to_string().contains("404") => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn pulse(&self, repo: &Repo, stats: &crate::config::StatsConfig) -> RepoPulse {
        let mut pulse = RepoPulse::default();
        // Exact total via the X-Total-Count header on a limit=1 commit list.
        // (Repo-wide; the author filter applies to the 30-day series below.)
        if let Ok(resp) = self.get(&format!(
            "/repos/{}/{}/commits?limit=1&stat=false",
            self.entry.user, repo.name
        )) {
            pulse.total_commits = resp
                .headers()
                .get("x-total-count")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse().ok());
        }
        let since = (Utc::now() - chrono::Duration::days(30))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        if let Ok(v) = self.json(&format!(
            "/repos/{}/{}/commits?since={since}&limit=100&stat=false",
            self.entry.user, repo.name
        )) {
            let commits: Vec<CommitMeta> = v
                .as_array()
                .map(|arr| arr.iter().map(commit_meta_from_forgejo).collect())
                .unwrap_or_default();
            pulse.daily_30 = super::github::bucket_commit_days(stats.filter_commit_dates(&commits));
        }
        pulse
    }
}

/// Authorship metadata from one Forgejo/Gitea commit-list item.
fn commit_meta_from_forgejo(c: &serde_json::Value) -> CommitMeta {
    let s = |ptr: &str| {
        c.pointer(ptr)
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string()
    };
    let date = {
        let d = s("/commit/author/date");
        if d.is_empty() { s("/created") } else { d }
    };
    CommitMeta {
        date,
        author_email: s("/commit/author/email"),
        author_name: s("/commit/author/name"),
        author_login: s("/author/login"),
        days_ago: None,
    }
}
