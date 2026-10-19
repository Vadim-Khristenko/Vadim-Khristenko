//! Last.fm integration: live "now playing", recent scrobbles and top artists
//! for the vibe and music cards.
//!
//! Config: `[lastfm] username = "..."` in config/profile.toml. The API key is
//! ONLY read from the environment (`LASTFM_API_KEY` by default, name
//! configurable via `api_key_env`) and is never logged or embedded anywhere.
//!
//! Last.fm is flaky in two ways, and both are handled here:
//! - it answers transient failures as HTTP 200 with `{"error": 8|11|16|29}`
//!   in the body, so the body is checked and those codes are retried;
//! - a whole run can land in an outage, so the last good answer is kept in
//!   `assets/state/lastfm.json` (committed with the cards) and served, marked
//!   stale, instead of dropping back to the static composers list.

use crate::config::profile::Lastfm;
use crate::log;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const API: &str = "https://ws.audioscrobbler.com/2.0/";
const ATTEMPTS: u32 = 3;
const RECENT_LIMIT: usize = 6;
/// A cached answer older than this is not shown at all.
const CACHE_MAX_AGE_SECS: i64 = 7 * 24 * 3600;
/// Last.fm's grey star placeholder, returned when a track has no artwork.
const PLACEHOLDER_ART: &str = "2a96cbd8b46e442fc41c2b86b821562f";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub artist: String,
    pub name: String,
    #[serde(default)]
    pub album: String,
    /// Largest artwork URL Last.fm offers, None for the placeholder.
    #[serde(default)]
    pub image: Option<String>,
    pub now_playing: bool,
    /// Unix seconds of the scrobble; None while the track is still playing.
    #[serde(default)]
    pub played_at: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Music {
    /// Currently playing track, or the most recent scrobble.
    pub track: Option<Track>,
    /// Up to RECENT_LIMIT scrobbles after `track`, newest first.
    #[serde(default)]
    pub recent: Vec<Track>,
    /// (artist, playcount) for the last month, most-played first.
    pub top_artists: Vec<(String, u64)>,
    /// Lifetime scrobble count from `recenttracks/@attr/total`.
    #[serde(default)]
    pub total_scrobbles: Option<u64>,
    /// Unix seconds of the API answer this came from.
    #[serde(default)]
    pub fetched_at: i64,
    /// True when served from the cache because the live call failed.
    #[serde(default, skip_serializing)]
    pub stale: bool,
}

/// Why a call failed. Only the retryable ones are worth another attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum CallError {
    Network(String),
    Http(u16),
    Api { code: i64, message: String },
    Decode,
}

impl CallError {
    pub fn retryable(&self) -> bool {
        match self {
            CallError::Network(_) => true,
            CallError::Http(code) => *code == 429 || *code >= 500,
            // 8 operation failed, 11 service offline, 16 temporarily
            // unavailable, 29 rate limit exceeded.
            CallError::Api { code, .. } => matches!(code, 8 | 11 | 16 | 29),
            CallError::Decode => false,
        }
    }
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::Network(e) => write!(f, "network: {e}"),
            CallError::Http(code) => write!(f, "HTTP {code}"),
            CallError::Api { code, message } => write!(f, "API error {code}: {message}"),
            CallError::Decode => write!(f, "response is not JSON"),
        }
    }
}

/// Last.fm reports API failures inside a 200 body.
pub fn api_error(v: &Value) -> Option<CallError> {
    let code = v.get("error")?.as_i64()?;
    let message = v
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or_default()
        .to_string();
    Some(CallError::Api { code, message })
}

fn text<'a>(v: &'a Value, ptr: &str) -> Option<&'a str> {
    v.pointer(ptr)
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty())
}

fn parse_track(t: &Value) -> Track {
    let image = t
        .get("image")
        .and_then(|i| i.as_array())
        .and_then(|arr| arr.iter().rev().find_map(|i| text(i, "/#text")))
        .filter(|url| !url.contains(PLACEHOLDER_ART))
        .map(str::to_string);
    Track {
        artist: text(t, "/artist/#text")
            .or_else(|| text(t, "/artist/name"))
            .unwrap_or_default()
            .to_string(),
        name: text(t, "/name").unwrap_or_default().to_string(),
        album: text(t, "/album/#text").unwrap_or_default().to_string(),
        image,
        now_playing: text(t, "/@attr/nowplaying") == Some("true"),
        played_at: text(t, "/date/uts").and_then(|s| s.parse().ok()),
    }
}

/// Every track in a `user.getrecenttracks` answer, newest first. Last.fm
/// sends `recenttracks.track` as an array (sometimes with a leading
/// now-playing entry) or as a bare object; both are handled.
pub fn parse_recent_list(v: &Value) -> Vec<Track> {
    match v.pointer("/recenttracks/track") {
        Some(Value::Array(arr)) => arr.iter().map(parse_track).collect(),
        Some(obj @ Value::Object(_)) => vec![parse_track(obj)],
        _ => Vec::new(),
    }
}

/// The most recent track only.
#[cfg(test)]
pub fn parse_recent(v: &Value) -> Option<Track> {
    parse_recent_list(v).into_iter().next()
}

/// Lifetime scrobbles (Last.fm serializes counts as strings).
pub fn parse_total(v: &Value) -> Option<u64> {
    text(v, "/recenttracks/@attr/total").and_then(|s| s.parse().ok())
}

/// Top artists with playcounts.
pub fn parse_top_artists(v: &Value) -> Vec<(String, u64)> {
    v.pointer("/topartists/artist")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    let name = a.get("name")?.as_str()?.to_string();
                    let plays = text(a, "/playcount")
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(0);
                    Some((name, plays))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Build a Music value from the two raw answers. None when neither held
/// anything usable.
pub fn assemble(recent: Option<&Value>, top: Option<&Value>, now: i64) -> Option<Music> {
    let mut tracks = recent.map(parse_recent_list).unwrap_or_default();
    let top_artists = top.map(parse_top_artists).unwrap_or_default();
    if tracks.is_empty() && top_artists.is_empty() {
        return None;
    }
    let track = if tracks.is_empty() {
        None
    } else {
        Some(tracks.remove(0))
    };
    tracks.truncate(RECENT_LIMIT);
    Some(Music {
        track,
        recent: tracks,
        top_artists,
        total_scrobbles: recent.and_then(parse_total),
        fetched_at: now,
        stale: false,
    })
}

/// Fill whatever the live answer lacks from the cache. A cached answer never
/// claims a live spin: if it is used for the track, `now_playing` goes false.
pub fn merge_with_cache(fresh: Option<Music>, cached: Option<Music>, now: i64) -> Option<Music> {
    let cached = cached
        .filter(|c| now - c.fetched_at <= CACHE_MAX_AGE_SECS)
        .map(|mut c| {
            if let Some(t) = c.track.as_mut() {
                t.now_playing = false;
            }
            c.stale = true;
            c
        });
    match (fresh, cached) {
        (Some(mut f), Some(c)) => {
            if f.track.is_none() {
                f.track = c.track;
                f.recent = c.recent;
                f.total_scrobbles = f.total_scrobbles.or(c.total_scrobbles);
            }
            if f.top_artists.is_empty() {
                f.top_artists = c.top_artists;
            }
            Some(f)
        }
        (Some(f), None) => Some(f),
        (None, c) => c,
    }
}

pub fn read_cache(path: &Path) -> Option<Music> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

fn write_cache(path: &Path, m: &Music) {
    let res = path
        .parent()
        .map(std::fs::create_dir_all)
        .unwrap_or(Ok(()))
        .and_then(|_| {
            let json = serde_json::to_string_pretty(m).expect("Music serializes");
            std::fs::write(path, json + "\n")
        });
    if let Err(e) = res {
        log::warn(&format!(
            "lastfm: could not write cache {}: {e}",
            path.display()
        ));
    }
}

/// One API method, retried on the failures Last.fm recovers from by itself.
/// The URL carries the key, so errors are stripped of it before logging.
fn call(
    client: &reqwest::blocking::Client,
    key: &str,
    user: &str,
    method: &str,
    extra: &[(&str, &str)],
) -> Result<Value, CallError> {
    let mut query: Vec<(&str, &str)> = vec![
        ("method", method),
        ("user", user),
        ("api_key", key),
        ("format", "json"),
    ];
    query.extend_from_slice(extra);
    let mut attempt = 0;
    loop {
        attempt += 1;
        let result = client
            .get(API)
            .query(&query)
            .send()
            .map_err(|e| CallError::Network(e.without_url().to_string()))
            .and_then(|resp| {
                let status = resp.status();
                let body: Result<Value, CallError> = resp.json().map_err(|_| CallError::Decode);
                match (status.is_success(), body) {
                    (_, Ok(v)) if api_error(&v).is_some() => Err(api_error(&v).expect("checked")),
                    (true, Ok(v)) => Ok(v),
                    (false, _) => Err(CallError::Http(status.as_u16())),
                    (true, Err(e)) => Err(e),
                }
            });
        match result {
            Ok(v) => return Ok(v),
            Err(e) if e.retryable() && attempt < ATTEMPTS => {
                let wait = Duration::from_secs(u64::from(attempt) * 2);
                log::warn(&format!(
                    "lastfm: {method} {e}, retry {attempt}/{} in {}s",
                    ATTEMPTS - 1,
                    wait.as_secs()
                ));
                sleep(wait);
            }
            Err(e) => {
                log::warn(&format!("lastfm: {method} gave up: {e}"));
                return Err(e);
            }
        }
    }
}

/// Live fetch with cache fallback. Returns None only when the key is absent
/// and nothing is cached, or when both the API and the cache come up empty.
/// Never fatal, never leaks the key.
pub fn fetch(cfg: &Lastfm, cache_path: &Path) -> Option<Music> {
    let now = chrono::Utc::now().timestamp();
    let cached = read_cache(cache_path);
    let key = match std::env::var(&cfg.api_key_env) {
        Ok(k) if !k.is_empty() => k,
        _ => {
            log::warn(&format!(
                "lastfm: no {} in the environment, using the cache if there is one",
                cfg.api_key_env
            ));
            return merge_with_cache(None, cached, now);
        }
    };
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(12))
        .user_agent("vai-profile-engine")
        .build()
        .ok()?;
    let limit = (RECENT_LIMIT + 1).to_string();
    let recent = call(
        &client,
        &key,
        &cfg.username,
        "user.getrecenttracks",
        &[("limit", &limit)],
    )
    .ok();
    let top = call(
        &client,
        &key,
        &cfg.username,
        "user.gettopartists",
        &[("period", "1month"), ("limit", "6")],
    )
    .ok();
    let fresh = assemble(recent.as_ref(), top.as_ref(), now);
    if let Some(f) = &fresh {
        let complete = f.track.is_some() && !f.top_artists.is_empty();
        if complete {
            write_cache(cache_path, f);
        }
    }
    let music = merge_with_cache(fresh, cached, now);
    match &music {
        Some(m) => {
            if let Some(t) = &m.track {
                let state = match (m.stale, t.now_playing) {
                    (true, _) => "cached",
                    (false, true) => "now playing",
                    (false, false) => "last played",
                };
                log::step("lastfm", state, &format!("{} · {}", t.artist, t.name));
            }
        }
        None => log::warn("lastfm: nothing live and nothing cached, music falls back to config"),
    }
    music
}

/// Offline fixtures for `--fixtures` runs and tests.
pub fn from_fixtures(dir: &Path) -> Option<Music> {
    let read = |name: &str| -> Option<Value> {
        serde_json::from_str(&std::fs::read_to_string(dir.join(name)).ok()?).ok()
    };
    let recent = read("lastfm_recent.json")?;
    let top = read("lastfm_top_artists.json");
    assemble(Some(&recent), top.as_ref(), 0)
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
        assert_eq!(m.total_scrobbles, Some(48211));
    }

    #[test]
    fn recent_track_object_and_missing_nowplaying_degrade() {
        // Bare object (not array), no @attr: not now-playing.
        let v: Value = serde_json::from_str(
            r##"{"recenttracks":{"track":{"artist":{"#text":"Kanro"},"name":"night drive"}}}"##,
        )
        .unwrap();
        let t = parse_recent(&v).unwrap();
        assert_eq!(t.artist, "Kanro");
        assert!(!t.now_playing);
        // Garbage gives None, no panic.
        assert!(parse_recent(&serde_json::json!({"error": 6})).is_none());
        assert!(parse_top_artists(&serde_json::json!({"error": 6})).is_empty());
    }

    #[test]
    fn recent_list_reads_art_dates_and_skips_placeholder() {
        let v: Value = serde_json::from_str(
            r##"{"recenttracks":{"@attr":{"total":"1200"},"track":[
              {"artist":{"#text":"Deco*27"},"name":"Vampire","album":{"#text":"Vampire"},
               "@attr":{"nowplaying":"true"},
               "image":[{"#text":"https://img/s.png"},{"#text":"https://img/xl.png"}]},
              {"artist":{"#text":"Kanro"},"name":"night drive","date":{"uts":"1760860000"},
               "image":[{"#text":"https://lastfm.freetls.fastly.net/i/u/300x300/2a96cbd8b46e442fc41c2b86b821562f.png"}]}
            ]}}"##,
        )
        .unwrap();
        let list = parse_recent_list(&v);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].image.as_deref(), Some("https://img/xl.png"));
        assert_eq!(list[0].album, "Vampire");
        assert_eq!(list[0].played_at, None);
        assert_eq!(list[1].image, None);
        assert_eq!(list[1].played_at, Some(1760860000));
        assert_eq!(parse_total(&v), Some(1200));
        let m = assemble(Some(&v), None, 5).unwrap();
        assert_eq!(m.track.unwrap().name, "Vampire");
        assert_eq!(m.recent.len(), 1);
    }

    #[test]
    fn api_errors_in_200_bodies_are_classified() {
        let busy = api_error(&serde_json::json!({"error": 16, "message": "try again"})).unwrap();
        assert!(busy.retryable());
        let limited = api_error(&serde_json::json!({"error": 29, "message": "slow down"})).unwrap();
        assert!(limited.retryable());
        let bad_key =
            api_error(&serde_json::json!({"error": 10, "message": "Invalid API key"})).unwrap();
        assert!(!bad_key.retryable());
        assert!(api_error(&serde_json::json!({"recenttracks": {}})).is_none());
        assert!(CallError::Http(503).retryable());
        assert!(!CallError::Http(404).retryable());
    }

    fn sample(now: i64) -> Music {
        Music {
            track: Some(Track {
                artist: "Deco*27".into(),
                name: "Rabbit Hole".into(),
                album: String::new(),
                image: None,
                now_playing: true,
                played_at: None,
            }),
            recent: vec![],
            top_artists: vec![("Hatsune Miku".into(), 812)],
            total_scrobbles: Some(10),
            fetched_at: now,
            stale: false,
        }
    }

    #[test]
    fn cache_fills_an_outage_but_never_claims_now_playing() {
        let now = 1_000_000;
        let got = merge_with_cache(None, Some(sample(now - 3600)), now).unwrap();
        assert!(got.stale);
        assert!(!got.track.unwrap().now_playing);
        // Too old to show.
        assert!(merge_with_cache(None, Some(sample(now - CACHE_MAX_AGE_SECS - 1)), now).is_none());
    }

    #[test]
    fn partial_live_answer_borrows_the_missing_half_from_cache() {
        let now = 2_000_000;
        let mut fresh = sample(now);
        fresh.top_artists.clear();
        let got = merge_with_cache(Some(fresh), Some(sample(now - 60)), now).unwrap();
        assert!(got.track.unwrap().now_playing, "the live track stays live");
        assert_eq!(got.top_artists.len(), 1);
    }

    #[test]
    fn cache_round_trips_through_disk() {
        let dir = std::env::temp_dir().join(format!("vai-lastfm-{}", std::process::id()));
        let path = dir.join("state").join("lastfm.json");
        let m = sample(42);
        write_cache(&path, &m);
        assert_eq!(read_cache(&path), Some(m));
        let _ = std::fs::remove_dir_all(dir);
    }
}
