//! Editorial data model, everything that used to be hard-coded content.
//! Keeps the card modules about *layout*; the words live in `config/profile.toml`.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub aliases: Vec<String>,
    pub research: Research,
    #[serde(default)]
    pub games: Vec<Game>,
    #[serde(default)]
    pub composers: Vec<NoteItem>,
    pub best_game: BestGame,
    /// Optional per-card tweaks keyed by card name (`[cards.<name>]`):
    /// toggle a card off entirely or override its accent colour.
    #[serde(default)]
    pub cards: std::collections::BTreeMap<String, CardTweak>,
    /// Optional Last.fm hookup (`[lastfm]`), live now-playing / top artists.
    #[serde(default)]
    pub lastfm: Option<Lastfm>,
    /// Keycap badges above the header (`[[badges]]`).
    #[serde(default)]
    pub badges: Vec<Badge>,
    /// Words for the header card (`[header]`).
    #[serde(default)]
    pub header: Header,
    /// README sections that get a labelled divider (`[[sections]]`).
    #[serde(default)]
    pub sections: Vec<Section>,
    /// Free-form per-card facts (`[showcase.<card>] key = "value"`), so a
    /// version bump is a config edit, not a code change.
    #[serde(default)]
    pub showcase:
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, toml::Value>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Badge {
    /// Small legend in the keycap corner ("portfolio", "lang"...).
    pub legend: String,
    pub label: String,
    /// Keycap colour.
    pub color: String,
    /// Text colour; defaults to the dark ground.
    #[serde(default)]
    pub ink: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Header {
    #[serde(default)]
    pub tagline: String,
    /// Name in katakana, written top to bottom beside the name.
    #[serde(default)]
    pub katakana: String,
    /// Words the piano-roll notes sing, one per note.
    #[serde(default)]
    pub lyric: Vec<String>,
    #[serde(default)]
    pub tracks: Vec<HeaderTrack>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HeaderTrack {
    pub name: String,
    #[serde(default)]
    pub note: String,
    pub accent: String,
    /// The solo button lights up (the one project to look at first).
    #[serde(default)]
    pub solo: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Section {
    pub key: String,
    pub label: String,
}

/// Per-card overrides. Every field is optional so existing configs keep
/// working untouched.
#[derive(Debug, Clone, Deserialize)]
pub struct CardTweak {
    /// `false` removes the card from the render set (its README marker block
    /// simply stops being refreshed).
    #[serde(default = "default_true")]
    pub enabled: bool,
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
    /// Short human phase label, e.g. "architecture & data pipeline".
    #[serde(default)]
    pub phase: String,
    /// 0.0..=1.0, the progress bar on the research card.
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
pub struct BestGame {
    pub title: String,
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
    pub nick: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub server: String,
    #[serde(default)]
    pub game_id: String,
    #[serde(default)]
    pub characters: Vec<Character>,
    /// Every Nikke loved for the lore (`[[best_game.loved]]`).
    #[serde(default)]
    pub loved: Vec<Character>,
    /// The lobby scene of the best-game card (`[best_game.lobby]`).
    #[serde(default)]
    pub lobby: Lobby,
}

/// Numbers and names for the lobby scene. Strings are shown as written.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Lobby {
    /// Short name of the Nikke standing in the lobby.
    #[serde(default)]
    pub character: String,
    /// Full-body art for that Nikke (fetched by `vai-profile bestgame`).
    #[serde(default)]
    pub art_url: String,
    /// Short name of the Nikke used as the commander avatar.
    #[serde(default)]
    pub avatar: String,
    #[serde(default)]
    pub gems: u64,
    #[serde(default)]
    pub credits: u64,
    #[serde(default)]
    pub next_stage: String,
    #[serde(default)]
    pub normal: String,
    #[serde(default)]
    pub hard: String,
    #[serde(default)]
    pub cleared_at: String,
    #[serde(default)]
    pub clear_power: u64,
    #[serde(default)]
    pub standard_power: u64,
    #[serde(default)]
    pub union: String,
    #[serde(default)]
    pub union_level: u32,
    #[serde(default)]
    pub union_members: String,
    #[serde(default)]
    pub union_rank: String,
    #[serde(default)]
    pub outpost_defense_level: u32,
    #[serde(default)]
    pub research: String,
    #[serde(default)]
    pub synchro: String,
    #[serde(default)]
    pub towers: String,
    #[serde(default)]
    pub nikkes: String,
    #[serde(default)]
    pub overclock: String,
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
    /// Character level shown on the squad card.
    #[serde(default)]
    pub level: Option<u32>,
    /// Combat power as shown in game ("26 340").
    #[serde(default)]
    pub power: String,
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
