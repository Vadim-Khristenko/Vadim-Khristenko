//! The repos whose live stats the project cards print (`config/flagship.toml`).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FlagshipConfig {
    #[serde(default)]
    pub project: Vec<FlagshipProject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FlagshipProject {
    /// Human name, for reading the config and for test messages.
    pub name: String,
    /// Repository name, looked up live across every configured provider.
    pub repo: String,
    /// Which platform's copy supplies commits and languages (falls back to
    /// any hit).
    #[serde(default)]
    pub prefer: Option<String>,
}
