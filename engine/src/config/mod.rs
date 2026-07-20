//! TOML configuration: load + validate the three files under `config/`.
//!
//! * `profile.toml`   — editorial content (aliases, games, research, AI lab…)
//! * `providers.toml` — the platform list (github / forgejo instances)
//! * `flagship.toml`  — the coolest-projects card
//!
//! Validation is strict and fails early with a readable message: a broken
//! config should stop the run before any card is touched.

pub mod flagship;
pub mod profile;
pub mod providers;
pub mod stats;

pub use flagship::FlagshipConfig;
pub use profile::ProfileConfig;
pub use providers::{ProviderEntry, ProviderKind, ProvidersConfig};
pub use stats::StatsConfig;

use anyhow::{bail, Context, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Config {
    pub profile: ProfileConfig,
    pub providers: ProvidersConfig,
    pub flagship: FlagshipConfig,
    /// Optional `stats.toml` — rollup exclusions (defaults to none).
    pub stats: StatsConfig,
}

fn load_toml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("cannot parse {}", path.display()))
}

impl Config {
    pub fn load(config_dir: &Path) -> Result<Config> {
        let profile: ProfileConfig = load_toml(&config_dir.join("profile.toml"))?;
        let providers: ProvidersConfig = load_toml(&config_dir.join("providers.toml"))?;
        let flagship: FlagshipConfig = load_toml(&config_dir.join("flagship.toml"))?;
        // stats.toml is optional: a missing file means "no exclusions", but a
        // present-and-broken file must still fail loudly.
        let stats_path = config_dir.join("stats.toml");
        let stats: StatsConfig = if stats_path.is_file() {
            load_toml(&stats_path)?
        } else {
            StatsConfig::default()
        };
        let cfg = Config {
            profile,
            providers,
            flagship,
            stats,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        if self.profile.aliases.is_empty() {
            bail!("profile.toml: `aliases` must not be empty");
        }
        if self.profile.games.is_empty() {
            bail!("profile.toml: at least one [[games]] entry is required");
        }
        if self.profile.quotes.is_empty() {
            bail!("profile.toml: `quotes` must not be empty");
        }
        if self.providers.provider.is_empty() {
            bail!("providers.toml: at least one [[provider]] entry is required");
        }
        let mut seen = std::collections::BTreeSet::new();
        for p in &self.providers.provider {
            if p.id.is_empty() || p.user.is_empty() {
                bail!("providers.toml: provider `id` and `user` must be non-empty");
            }
            if !seen.insert(p.id.clone()) {
                bail!("providers.toml: duplicate provider id `{}`", p.id);
            }
            if p.kind == ProviderKind::Forgejo && p.base_url.is_none() {
                bail!(
                    "providers.toml: provider `{}` is kind=forgejo and needs `base_url`",
                    p.id
                );
            }
        }
        if self
            .providers
            .provider
            .iter()
            .filter(|p| p.primary)
            .count()
            > 1
        {
            bail!("providers.toml: at most one provider may set primary = true");
        }
        for pr in &self.flagship.project {
            if pr.name.is_empty() || pr.repo.is_empty() {
                bail!("flagship.toml: every [[project]] needs `name` and `repo`");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_config_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../config")
    }

    #[test]
    fn real_config_parses_and_validates() {
        let cfg = Config::load(&repo_config_dir()).expect("config/ must load");
        assert!(cfg.profile.aliases.contains(&"VAI_PROG".to_string()));
        assert_eq!(cfg.providers.provider.len(), 3);
        assert!(cfg.providers.provider.iter().any(|p| p.id == "vai-git" && p.primary));
        assert!(cfg.flagship.project.len() >= 4);
        // The editorial lists are user-tuned — assert shape, not exact counts.
        assert!(!cfg.profile.ai.models.is_empty());
        assert!(!cfg.profile.ai.favourites.is_empty());
        assert!(cfg.profile.ai.favourites.len() <= cfg.profile.ai.models.len());
    }

    #[test]
    fn forgejo_without_base_url_is_rejected() {
        let providers: ProvidersConfig = toml::from_str(
            r#"
            [[provider]]
            id = "x"
            kind = "forgejo"
            display = "X"
            user = "u"
            "#,
        )
        .unwrap();
        let mut cfg = Config::load(&repo_config_dir()).unwrap();
        cfg.providers = providers;
        let err = cfg.validate().unwrap_err().to_string();
        assert!(err.contains("base_url"), "got: {err}");
    }

    #[test]
    fn duplicate_provider_id_is_rejected() {
        let providers: ProvidersConfig = toml::from_str(
            r#"
            [[provider]]
            id = "x"
            kind = "github"
            display = "X"
            user = "u"
            [[provider]]
            id = "x"
            kind = "github"
            display = "X2"
            user = "u"
            "#,
        )
        .unwrap();
        let mut cfg = Config::load(&repo_config_dir()).unwrap();
        cfg.providers = providers;
        let err = cfg.validate().unwrap_err().to_string();
        assert!(err.contains("duplicate"), "got: {err}");
    }

    #[test]
    fn unknown_provider_kind_fails_to_parse() {
        let res: Result<ProvidersConfig, _> = toml::from_str(
            r#"
            [[provider]]
            id = "x"
            kind = "gitlab"
            display = "X"
            user = "u"
            "#,
        );
        assert!(res.is_err());
    }
}
