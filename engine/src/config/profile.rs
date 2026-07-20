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
    /// Optional per-card tweaks keyed by card name (`[cards.<name>]`):
    /// toggle a card off entirely or override its accent colour.
    #[serde(default)]
    pub cards: std::collections::BTreeMap<String, CardTweak>,
    /// Optional Last.fm hookup (`[lastfm]`) — live now-playing / top artists.
    #[serde(default)]
    pub lastfm: Option<Lastfm>,
}

/// Per-card overrides. Every field is optional so existing configs keep
/// working untouched.
#[derive(Debug, Clone, Deserialize)]
pub struct CardTweak {
    /// `false` removes the card from the render set (its README marker block
    /// simply stops being refreshed).
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Accent colour override for the card chrome (e.g. "#bb9af7").
    #[serde(default)]
    pub accent: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Last.fm integration: the username is public config; the API key is only
/// ever read from the environment (never stored in the repo).
#[derive(Debug, Clone, Deserialize)]
pub struct Lastfm {
    pub username: String,
    /// Environment variable holding the API key (default: LASTFM_API_KEY).
    #[serde(default = "default_lastfm_env")]
    pub api_key_env: String,
}

fn default_lastfm_env() -> String {
    "LASTFM_API_KEY".into()
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
    /// Free-form labelled stats (`[[best_game.extra]]`): anything you want on
    /// the card — friends, union, playtime… Add `current` + `max` to get a
    /// small progress bar under the value.
    #[serde(default)]
    pub extra: Vec<ExtraStat>,
    #[serde(default)]
    pub characters: Vec<Character>,
}

/// One user-defined stat on the best-game card.
#[derive(Debug, Clone, Deserialize)]
pub struct ExtraStat {
    pub label: String,
    /// Display string ("" + current/max set → rendered as "current/max").
    #[serde(default)]
    pub value: String,
    /// Optional progress pair — renders a micro progress bar.
    #[serde(default)]
    pub current: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}

impl ExtraStat {
    /// The string shown as the value (explicit value wins; else current/max).
    pub fn display(&self) -> String {
        if !self.value.is_empty() {
            return self.value.clone();
        }
        match (self.current, self.max) {
            (Some(c), Some(m)) => format!("{}/{}", trim_f(c), trim_f(m)),
            (Some(c), None) => trim_f(c),
            _ => "—".into(),
        }
    }

    /// Fill fraction for the micro bar, when both numbers are present.
    pub fn fraction(&self) -> Option<f64> {
        match (self.current, self.max) {
            (Some(c), Some(m)) if m > 0.0 => Some((c / m).clamp(0.0, 1.0)),
            _ => None,
        }
    }
}

fn trim_f(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        format!("{v}")
    }
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
