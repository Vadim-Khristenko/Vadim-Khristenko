//! Stats hygiene rules (`config/stats.toml`) — keeps rollup metrics honest.
//!
//! * `exclude_repos` — repositories dropped from EVERY rollup metric
//!   (languages, LOC, stars, forks, open items, most-active, flagship hits).
//!   Matched with the same normalization as mirror dedup (lowercase,
//!   trailing `.git` stripped), so one entry covers all platforms.
//! * `exclude_commit_authors` — commit authors (email, name or login,
//!   case-insensitive) filtered out of per-repo commit series so CI
//!   auto-commits never inflate activity.
//!
//! The file is optional: absent → no exclusions, nothing breaks.

use crate::model::{normalize_repo_name, CommitMeta};
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StatsConfig {
    #[serde(default)]
    pub exclude_repos: Vec<String>,
    #[serde(default)]
    pub exclude_commit_authors: Vec<String>,
}

impl StatsConfig {
    /// Is this repository excluded from rollups? (mirror-normalized match)
    pub fn repo_excluded(&self, name: &str) -> bool {
        let key = normalize_repo_name(name);
        self.exclude_repos
            .iter()
            .any(|r| normalize_repo_name(r) == key)
    }

    /// Is this commit authored by an excluded identity? Every configured
    /// entry is checked against the author email, display name AND login.
    pub fn commit_excluded(&self, c: &CommitMeta) -> bool {
        let hit = |field: &str| {
            !field.is_empty()
                && self
                    .exclude_commit_authors
                    .iter()
                    .any(|a| a.eq_ignore_ascii_case(field))
        };
        hit(&c.author_email) || hit(&c.author_name) || hit(&c.author_login)
    }

    /// Commit dates that survive the author filter (feed to bucketing).
    pub fn filter_commit_dates(&self, commits: &[CommitMeta]) -> Vec<String> {
        commits
            .iter()
            .filter(|c| !self.commit_excluded(c))
            .map(|c| c.date.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> StatsConfig {
        StatsConfig {
            exclude_repos: vec!["Vadim-Khristenko".into()],
            exclude_commit_authors: vec![
                "actions@git.vai-rice.space".into(),
                "VIA GIT".into(),
                "github-actions[bot]".into(),
            ],
        }
    }

    fn commit(email: &str, name: &str, login: &str) -> CommitMeta {
        CommitMeta {
            date: "2026-07-19T12:00:00Z".into(),
            author_email: email.into(),
            author_name: name.into(),
            author_login: login.into(),
            days_ago: None,
        }
    }

    #[test]
    fn repo_exclusion_is_mirror_normalized() {
        let s = cfg();
        assert!(s.repo_excluded("Vadim-Khristenko"));
        assert!(s.repo_excluded("vadim-khristenko"));
        assert!(s.repo_excluded("Vadim-Khristenko.git"));
        assert!(!s.repo_excluded("TheWall"));
    }

    #[test]
    fn commit_exclusion_matches_email_name_or_login() {
        let s = cfg();
        assert!(s.commit_excluded(&commit("actions@git.vai-rice.space", "x", "")));
        assert!(s.commit_excluded(&commit("x@y.z", "via git", ""))); // case-insensitive
        assert!(s.commit_excluded(&commit("", "", "github-actions[bot]")));
        assert!(!s.commit_excluded(&commit("vadim@vai-rice.space", "Vadim Khristenko", "vai")));
        // Empty fields never match by accident.
        assert!(!StatsConfig::default().commit_excluded(&commit("", "", "")));
    }

    #[test]
    fn filter_keeps_only_human_commit_dates() {
        let s = cfg();
        let commits = vec![
            commit("vadim@vai-rice.space", "Vadim Khristenko", "vai"),
            commit("actions@git.vai-rice.space", "VIA GIT", ""),
            commit("", "github-actions[bot]", "github-actions[bot]"),
        ];
        assert_eq!(s.filter_commit_dates(&commits).len(), 1);
    }
}
