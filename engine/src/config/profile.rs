//! Editorial data model — everything that used to be hard-coded content.
//! Keeps the card modules about *layout*; the words live in `config/profile.toml`.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub aliases: Vec<String>,
    pub quotes: Vec<String>,
    pub research: Research,
    #[serde(default)]
    pub games: Vec<Game>,
    #[serde(default)]
    pub composers: Vec<NoteItem>,
    #[serde(default)]
    pub focus: Vec<NoteItem>,
    #[serde(default)]
    pub socials: Vec<Social>,
    pub ai: AiLab,
    #[serde(default)]
    pub learning: Learning,
    pub best_game: BestGame,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Research {
    pub name: String,
    pub subtitle: String,
    pub blurb: String,
    /// Short human phase label, e.g. "architecture & data pipeline".
    #[serde(default)]
    pub phase: String,
    /// 0.0..=1.0 — rendered as a progress bar on the learning card.
    #[serde(default)]
    pub progress: f64,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    pub key: String,
    pub title: String,
    pub short: String,
    pub query: String,
    /// Accent colour for the fallback tile.
    pub ca: String,
    /// Base colour for the fallback tile.
    pub cb: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NoteItem {
    pub name: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Social {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiLab {
    /// "Companions", not autopilot.
    pub models: Vec<String>,
    pub favourites: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Learning {
    #[serde(default)]
    pub topics: Vec<NoteItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BestGame {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    /// SteamGridDB search term used when `art_url` is empty.
    #[serde(default)]
    pub query: String,
    /// Remote URL or local repo path for the cover ("" → SteamGridDB lookup).
    #[serde(default)]
    pub art_url: String,
    /// "portrait" (left 2:3 panel) or "landscape" (full-width behind text).
    #[serde(default = "default_cover_mode")]
    pub cover_mode: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default)]
    pub nick: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub server: String,
    #[serde(default)]
    pub game_id: String,
    #[serde(default)]
    pub blurb: String,
    #[serde(default)]
    pub characters: Vec<Character>,
}

fn default_cover_mode() -> String {
    "portrait".into()
}

#[derive(Debug, Clone, Deserialize)]
pub struct Character {
    pub name: String,
    pub short: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default)]
    pub art_url: String,
}

impl Character {
    /// Asset key: `char_` + lowercase alphanumerics of the short name.
    pub fn key(&self) -> String {
        let tail: String = self
            .short
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        format!("char_{tail}")
    }
}
