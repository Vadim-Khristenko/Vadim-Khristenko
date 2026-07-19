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

    fn pulse(&self, repo: &Repo) -> RepoPulse {
        self.data.pulses.get(&repo.name).cloned().unwrap_or_default()
    }
}
