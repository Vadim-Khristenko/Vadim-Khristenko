//! Provider list model — one entry per platform. Adding a platform is a
//! config entry (plus, for a new API family, one `Provider` impl).

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProvidersConfig {
    #[serde(default)]
    pub provider: Vec<ProviderEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderKind {
    Github,
    /// Gitea-compatible API — covers Codeberg and any self-hosted Forgejo.
    Forgejo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderEntry {
    pub id: String,
    pub kind: ProviderKind,
    pub display: String,
    /// Required for forgejo; ignored for github.
    #[serde(default)]
    pub base_url: Option<String>,
    pub user: String,
    /// Environment variable holding a READ-ONLY API token. Missing token →
    /// tokenless public reads (graceful degradation), never fatal.
    #[serde(default)]
    pub token_env: Option<String>,
    /// The source-of-truth platform (at most one).
    #[serde(default)]
    pub primary: bool,
}

impl ProviderEntry {
    pub fn token(&self) -> Option<String> {
        let var = self.token_env.as_deref()?;
        match std::env::var(var) {
            Ok(v) if !v.is_empty() => Some(v),
            _ => None,
        }
    }
}
