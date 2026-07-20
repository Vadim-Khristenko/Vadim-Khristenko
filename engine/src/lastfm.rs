//! Last.fm integration — live "now playing" / top artists for the vibe card.
//!
//! Config: `[lastfm] username = "…"` in config/profile.toml. The API key is
//! ONLY read from the environment (`LASTFM_API_KEY` by default, name
//! configurable via `api_key_env`) and is never logged or embedded anywhere.
//! Missing username or key → graceful skip; the static composers list keeps
//! the vibe card fully populated, exactly like a tokenless provider run.

use crate::config::profile::Lastfm;
use crate::log;
use serde_json::Value;
use std::path::Path;
use std::time::Duration;

const API: &str = "https://ws.audioscrobbler.com/2.0/";

#[derive(Debug, Clone)]
pub struct Track {
    pub artist: String,
    pub name: String,
    pub now_playing: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Music {
    /// Currently playing track, or the most recent scrobble.
    pub track: Option<Track>,
    /// (artist, playcount) for the last month, most-played first.
    pub top_artists: Vec<(String, u64)>,
}

/// The most recent track. Last.fm returns `recenttracks.track` as an array
/// (sometimes with a leading now-playing entry) or a bare object — both are
/// handled. `@attr.nowplaying == "true"` marks a live spin.
pub fn parse_recent(v: &Value) -> Option<Track> {
    let node = v.pointer("/recenttracks/track")?;
    let t = if node.is_array() {
        node.get(0)?
    } else {
        node
    };
    Some(Track {
        artist: t
            .pointer("/artist/#text")
            .or_else(|| t.pointer("/artist/name"))
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
        name: t.get("name").and_then(|x| x.as_str()).unwrap_or_default().to_string(),
        now_playing: t
            .pointer("/@attr/nowplaying")
            .and_then(|x| x.as_str())
            .map(|s| s == "true")
            .unwrap_or(false),
    })
}

/// Top artists with playcounts (Last.fm serializes counts as strings).
pub fn parse_top_artists(v: &Value) -> Vec<(String, u64)> {
    v.pointer("/topartists/artist")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    let name = a.get("name")?.as_str()?.to_string();
                    let plays = a
                        .get("playcount")
                        .and_then(|p| p.as_str())
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(0);
                    Some((name, plays))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Live fetch. Returns None (and logs a friendly note) when the key is
/// absent or the API is unreachable — never fatal, never leaks the key.
pub fn fetch(cfg: &Lastfm) -> Option<Music> {
    let key = match std::env::var(&cfg.api_key_env) {
        Ok(k) if !k.is_empty() => k,
        _ => {
            log::warn(&format!(
                "lastfm: no {} in the environment — music falls back to config",
                cfg.api_key_env
            ));
            return None;
        }
    };
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent("vai-profile-engine")
        .build()
        .ok()?;
    // The URL contains the key — it must never appear in logs or errors.
    let call = |method: &str, extra: &str| -> Option<Value> {
        let url = format!(
            "{API}?method={method}&user={}&api_key={key}&format=json{extra}",
            cfg.username
        );
        let resp = client.get(url).send().ok()?;
        if !resp.status().is_success() {
            log::warn(&format!("lastfm: {method} → HTTP {}", resp.status()));
            return None;
        }
        resp.json().ok()
    };
    let track = call("user.getrecenttracks", "&limit=1")
        .as_ref()
        .and_then(parse_recent);
    let top_artists = call("user.gettopartists", "&period=1month&limit=6")
        .as_ref()
        .map(parse_top_artists)
        .unwrap_or_default();
    if track.is_none() && top_artists.is_empty() {
        log::warn("lastfm: nothing usable returned — music falls back to config");
        return None;
    }
    if let Some(t) = &track {
        log::step(
            "lastfm",
            if t.now_playing { "now playing" } else { "last played" },
            &format!("{} — {}", t.artist, t.name),
        );
    }
    Some(Music { track, top_artists })
}

/// Offline fixtures for `--fixtures` runs and tests.
pub fn from_fixtures(dir: &Path) -> Option<Music> {
    let read = |name: &str| -> Option<Value> {
        serde_json::from_str(&std::fs::read_to_string(dir.join(name)).ok()?).ok()
    };
    let recent = read("lastfm_recent.json")?;
    let top = read("lastfm_top_artists.json");
    Some(Music {
        track: parse_recent(&recent),
        top_artists: top.as_ref().map(parse_top_artists).unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_dir() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures")
    }

    #[test]
    fn parses_recorded_lastfm_fixtures() {
        let m = from_fixtures(&fixtures_dir()).expect("lastfm fixtures load");
        let t = m.track.expect("track present");
        assert_eq!(t.artist, "Deco*27");
        assert_eq!(t.name, "Rabbit Hole");
        assert!(t.now_playing);
        assert!(m.top_artists.len() >= 3);
        assert_eq!(m.top_artists[0].0, "Hatsune Miku");
        assert_eq!(m.top_artists[0].1, 812);
    }

    #[test]
    fn recent_track_object_and_missing_nowplaying_degrade() {
        // Bare object (not array), no @attr → not now-playing.
        let v: Value = serde_json::from_str(
            r##"{"recenttracks":{"track":{"artist":{"#text":"Kanro"},"name":"night drive"}}}"##,
        )
        .unwrap();
        let t = parse_recent(&v).unwrap();
        assert_eq!(t.artist, "Kanro");
        assert!(!t.now_playing);
        // Garbage → None, no panic.
        assert!(parse_recent(&serde_json::json!({"error": 6})).is_none());
        assert!(parse_top_artists(&serde_json::json!({"error": 6})).is_empty());
    }
}
