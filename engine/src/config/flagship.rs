//! The coolest-projects card model (`config/flagship.toml`).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FlagshipConfig {
    #[serde(default)]
    pub project: Vec<FlagshipProject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlagshipProject {
    /// Display name for the card.
    pub name: String,
    /// Repository name, looked up live across every configured provider.
    pub repo: String,
    /// Which platform's live stats to headline (falls back to any hit).
    #[serde(default)]
    pub prefer: Option<String>,
    #[serde(default)]
    pub site: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub blurb: String,
    /// Optional accent colour override for the row.
    #[serde(default)]
    pub accent: Option<String>,
}
