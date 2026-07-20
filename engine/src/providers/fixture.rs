//! Offline provider backed by recorded JSON — powers `--fixtures` dry runs and
//! keeps every provider/aggregation test off the network.

use super::Provider;
use crate::config::ProviderEntry;
use crate::model::*;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FixtureData {
    #[serde(default)]
    pub profile: Profile,
    #[serde(default)]
    pub repos: Vec<Repo>,
    /// Language bytes keyed by repo name.
    #[serde(default)]
    pub languages: BTreeMap<String, LangBytes>,
    /// `[["YYYY-MM-DD", count], …]`
    #[serde(default)]
    pub heatmap: Vec<(String, u32)>,
    #[serde(default)]
    pub commit_windows: Option<CommitWindows>,
    /// Pulses keyed by repo name.
    #[serde(default)]
    pub pulses: BTreeMap<String, RepoPulse>,
    /// Raw 30-day commit lists keyed by repo name — used instead of `pulses`
    /// when present, so author-exclusion filtering is exercised end-to-end.
    /// Fixture commits use `days_ago` so they never age out of the window.
    #[serde(default)]
    pub commits_30d: BTreeMap<String, Vec<CommitMeta>>,
}

pub struct FixtureProvider {
    entry: ProviderEntry,
    data: FixtureData,
}

impl FixtureProvider {
    pub fn load(entry: &ProviderEntry, dir: &Path) -> Result<Self> {
        let path = dir.join(format!("{}.json", entry.id));
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("cannot read fixture {}", path.display()))?;
        let data: FixtureData = serde_json::from_str(&text)
            .with_context(|| format!("cannot parse fixture {}", path.display()))?;
        Ok(FixtureProvider {
            entry: entry.clone(),
            data,
        })
    }

    pub fn from_data(entry: ProviderEntry, data: FixtureData) -> Self {
        FixtureProvider { entry, data }
    }
}

impl Provider for FixtureProvider {
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
        Ok(self.data.profile.clone())
    }

    fn repos(&self) -> Result<Vec<Repo>> {
        let mut repos = self.data.repos.clone();
        repos.sort_by(|a, b| b.pushed_at.cmp(&a.pushed_at));
        Ok(repos)
    }

    fn languages(&self, repo: &Repo) -> Result<LangBytes> {
        Ok(self.data.languages.get(&repo.name).cloned().unwrap_or_default())
    }

    fn activity(&self) -> Result<Heatmap> {
        let mut map = BTreeMap::new();
        for (date_s, count) in &self.data.heatmap {
            if let Ok(d) = NaiveDate::parse_from_str(date_s, "%Y-%m-%d") {
                map.insert(d, *count);
            }
        }
        Ok(Heatmap::from_map(map))
    }

    fn repo(&self, name: &str) -> Result<Option<Repo>> {
        let key = normalize_repo_name(name);
        Ok(self
            .data
            .repos
            .iter()
            .find(|r| normalize_repo_name(&r.name) == key)
            .cloned())
    }

    fn commit_windows(&self) -> Option<CommitWindows> {
        self.data.commit_windows.clone()
    }

    fn pulse(&self, repo: &Repo, stats: &crate::config::StatsConfig) -> RepoPulse {
        // Raw commit list (author-filterable) wins over a canned pulse.
        if let Some(commits) = self.data.commits_30d.get(&repo.name) {
            let now = chrono::Utc::now();
            let resolved: Vec<CommitMeta> = commits
                .iter()
                .map(|c| {
                    let mut c = c.clone();
                    if let Some(days) = c.days_ago {
                        c.date = (now - chrono::Duration::days(days))
                            .format("%Y-%m-%dT12:00:00Z")
                            .to_string();
                    }
                    c
                })
                .collect();
            let total = self
                .data
                .pulses
                .get(&repo.name)
                .and_then(|p| p.total_commits)
                .or(Some(resolved.len() as u64));
            return RepoPulse {
                total_commits: total,
                daily_30: super::github::bucket_commit_days(
                    stats.filter_commit_dates(&resolved),
                ),
            };
        }
        self.data.pulses.get(&repo.name).cloned().unwrap_or_default()
    }
}
