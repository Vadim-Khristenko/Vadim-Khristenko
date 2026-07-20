//! Game / best-game art fetcher (SteamGridDB + arbitrary URLs / local files).
//!
//! Static images are cover-cropped to a uniform size (image crate); animated
//! WEBP/GIF are kept byte-for-byte (animation preserved) under a size cap with
//! a static first-frame fallback; transparent sprites are kept as PNG.
//! Downloads retry with backoff.

use crate::config::Config;
use crate::log;
use anyhow::{anyhow, bail, Context, Result};
use image::{DynamicImage, GenericImageView};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

const API: &str = "https://www.steamgriddb.com/api/v2";
const TARGET_W: u32 = 460;
const TARGET_H: u32 = 215;
const SIZE_CAP: usize = 2_200_000;
const DOWNLOAD_ATTEMPTS: u32 = 4;
const MEDIA_EXTS: [&str; 7] = ["webp", "gif", "avif", "apng", "png", "jpg", "jpeg"];

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("vai-profile-engine")
        .build()
        .expect("reqwest client")
}

fn sgdb_get(path: &str, token: &str) -> Result<serde_json::Value> {
    let resp = client()
        .get(format!("{API}{path}"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .with_context(|| format!("GET {path}"))?
        .error_for_status()
        .with_context(|| format!("GET {path}"))?;
    resp.json().context("bad JSON from SteamGridDB")
}

/// Fetch bytes with retries + backoff.
fn download(url: &str) -> Result<Vec<u8>> {
    let mut last: Option<anyhow::Error> = None;
    for i in 0..DOWNLOAD_ATTEMPTS {
        let attempt = client()
            .get(url)
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .and_then(|r| r.error_for_status())
            .map_err(anyhow::Error::from)
            .and_then(|r| r.bytes().map(|b| b.to_vec()).map_err(anyhow::Error::from))
            .and_then(|b| {
                if b.is_empty() {
                    Err(anyhow!("empty response"))
                } else {
                    Ok(b)
                }
            });
        match attempt {
            Ok(bytes) => return Ok(bytes),
            Err(e) => {
                if i < DOWNLOAD_ATTEMPTS - 1 {
                    log::warn(&format!("download retry {}/{}: {e}", i + 1, DOWNLOAD_ATTEMPTS - 1));
                    std::thread::sleep(Duration::from_millis(1200 * (i as u64 + 1)));
                }
                last = Some(e);
            }
        }
    }
    Err(last.unwrap_or_else(|| anyhow!("download failed")))
}

/// Fetch bytes from a URL or a local path (relative to the repo root).
fn get_bytes(src: &str, root: &Path) -> Result<Vec<u8>> {
    if src.starts_with("http://") || src.starts_with("https://") {
        return download(src);
    }
    let path = if Path::new(src).is_absolute() {
        PathBuf::from(src)
    } else {
        root.join(src)
    };
    std::fs::read(&path).with_context(|| format!("read {}", path.display()))
}

fn find_game_id(query: &str, token: &str) -> Result<Option<u64>> {
    let encoded: String = query
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || "-_.~".contains(c) {
                c.to_string()
            } else {
                c.to_string()
                    .bytes()
                    .map(|b| format!("%{b:02X}"))
                    .collect::<String>()
            }
        })
        .collect();
    let data = sgdb_get(&format!("/search/autocomplete/{encoded}"), token)?;
    Ok(data
        .pointer("/data/0/id")
        .and_then(|v| v.as_u64()))
}

/// Prefer a 460x215 capsule, then 920x430, then any grid, then a hero.
fn pick_art_url(gid: u64, token: &str) -> Option<String> {
    for path in [
        format!("/grids/game/{gid}?dimensions=460x215&types=static"),
        format!("/grids/game/{gid}?dimensions=920x430&types=static"),
        format!("/grids/game/{gid}?types=static"),
        format!("/heroes/game/{gid}?types=static"),
    ] {
        if let Ok(data) = sgdb_get(&path, token) {
            if let Some(url) = data.pointer("/data/0/url").and_then(|v| v.as_str()) {
                return Some(url.to_string());
            }
        }
    }
    None
}

/// Remove any prior <dest_base>.<ext> so a format switch never leaves a stale
/// file that find_media() would pick by extension priority.
fn purge(dest_base: &Path) {
    for ext in MEDIA_EXTS {
        let _ = std::fs::remove_file(dest_base.with_extension(ext));
    }
}

fn cover_crop(im: &DynamicImage, tw: u32, th: u32) -> DynamicImage {
    let (sw, sh) = im.dimensions();
    let scale = (tw as f64 / sw as f64).max(th as f64 / sh as f64);
    let (nw, nh) = (
        (sw as f64 * scale + 0.5) as u32,
        (sh as f64 * scale + 0.5) as u32,
    );
    let resized = im.resize_exact(nw, nh, image::imageops::FilterType::Lanczos3);
    let left = (nw - tw) / 2;
    let top = (nh - th) / 2;
    resized.crop_imm(left, top, tw, th)
}

fn save_jpeg(im: &DynamicImage, dest: &Path) -> Result<()> {
    let rgb = im.to_rgb8();
    let file = std::fs::File::create(dest)?;
    let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(std::io::BufWriter::new(file), 74);
    enc.encode_image(&rgb)?;
    Ok(())
}

fn is_animated(bytes: &[u8]) -> bool {
    // GIF: more than one image descriptor / graphic control; cheap heuristic:
    // GIF89a header + at least two frame separators.
    if bytes.starts_with(b"GIF8") {
        return bytes.windows(2).filter(|w| w == b"\x00\x2C").count() > 1;
    }
    // WebP: RIFF container with an ANIM chunk.
    if bytes.len() > 16 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return bytes.windows(4).take(512).any(|w| w == b"ANIM");
    }
    false
}

fn ext_for(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(b"GIF8") {
        "gif"
    } else if bytes.len() > 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        "webp"
    } else if bytes.starts_with(b"\x89PNG") {
        "png"
    } else {
        "jpg"
    }
}

/// dest_base.<ext>: animated → kept as-is (cap → static first-frame JPEG);
/// transparent static → cropped PNG; else cropped JPEG. Returns written path.
fn save_media(raw: &[u8], dest_base: &Path, tw: u32, th: u32) -> Result<PathBuf> {
    purge(dest_base);
    if is_animated(raw) {
        if raw.len() <= SIZE_CAP {
            let dest = dest_base.with_extension(ext_for(raw));
            std::fs::write(&dest, raw)?;
            return Ok(dest);
        }
        // Too heavy — fall back to a static cropped first frame if decodable.
        if let Ok(im) = image::load_from_memory(raw) {
            let dest = dest_base.with_extension("jpg");
            save_jpeg(&cover_crop(&im, tw, th), &dest)?;
            return Ok(dest);
        }
        bail!("animated file over size cap and not decodable");
    }
    let im = image::load_from_memory(raw).context("decode image")?;
    let has_alpha = im.color().has_alpha();
    let cropped = cover_crop(&im, tw, th);
    if has_alpha {
        let dest = dest_base.with_extension("png");
        cropped.to_rgba8().save(&dest)?;
        return Ok(dest);
    }
    let dest = dest_base.with_extension("jpg");
    save_jpeg(&cropped, &dest)?;
    Ok(dest)
}

fn file_kb(path: &Path) -> f64 {
    std::fs::metadata(path).map(|m| m.len() as f64 / 1024.0).unwrap_or(0.0)
}

pub fn fetch_bestgame(cfg: &Config, root: &Path, token: Option<&str>) {
    let g = &cfg.profile.best_game;
    let out = root.join("assets").join("bestgame");
    std::fs::create_dir_all(&out).ok();
    log::section(&format!("Best game: {}", g.title));

    let portrait = g.cover_mode != "landscape";
    let (ctw, cth) = if portrait { (300, 450) } else { (1100, 340) };
    let mut url = g.art_url.clone();
    if url.is_empty() {
        if let Some(tok) = token {
            let query = if g.query.is_empty() { &g.title } else { &g.query };
            match find_game_id(query, tok) {
                Ok(Some(gid)) => {
                    url = sgdb_get(&format!("/heroes/game/{gid}?types=static"), tok)
                        .ok()
                        .and_then(|d| d.pointer("/data/0/url").and_then(|v| v.as_str()).map(String::from))
                        .or_else(|| pick_art_url(gid, tok))
                        .unwrap_or_default();
                }
                _ => {}
            }
        }
    }
    if url.is_empty() {
        log::warn("cover: no art_url and no SteamGridDB key");
    } else {
        match get_bytes(&url, root).and_then(|bytes| save_media(&bytes, &out.join("cover"), ctw, cth)) {
            Ok(dest) => log::ok(
                "cover",
                &format!("{:.1} KB  {}", file_kb(&dest), dest.file_name().unwrap_or_default().to_string_lossy()),
            ),
            Err(e) => log::fail("cover", &format!("all retries failed ({e})")),
        }
    }

    for ch in &g.characters {
        if ch.art_url.is_empty() {
            log::step(&ch.short, "—", "no art_url → avatar fallback");
            continue;
        }
        match get_bytes(&ch.art_url, root)
            .and_then(|bytes| save_media(&bytes, &out.join(ch.key()), 300, 300))
        {
            Ok(dest) => log::ok(
                &ch.short,
                &format!("{:.1} KB  {}", file_kb(&dest), dest.file_name().unwrap_or_default().to_string_lossy()),
            ),
            Err(e) => log::fail(&ch.short, &format!("all retries failed → avatar fallback ({e})")),
        }
    }
}

pub fn fetch_games(cfg: &Config, root: &Path, token: Option<&str>, only: Option<&BTreeSet<String>>) {
    let out = root.join("assets").join("games");
    std::fs::create_dir_all(&out).ok();
    log::section("Fetching game-shelf covers");
    let Some(token) = token else {
        log::warn("shelf covers need a SteamGridDB key (--key / SGDB_KEY / STEAMGRIDDB_KEY / STEAMGRIDDB_API_KEY)");
        return;
    };
    let mut manifest = serde_json::Map::new();
    let games = &cfg.profile.games;
    for g in games {
        if let Some(only) = only {
            if !only.contains(&g.key) {
                continue;
            }
        }
        let result = (|| -> Result<(u64, PathBuf)> {
            let gid = find_game_id(&g.query, token)?
                .ok_or_else(|| anyhow!("not found on SteamGridDB"))?;
            let url = pick_art_url(gid, token).ok_or_else(|| anyhow!("no art for id={gid}"))?;
            purge(&out.join(&g.key));
            let bytes = download(&url)?;
            let im = image::load_from_memory(&bytes).context("decode")?;
            let dest = out.join(&g.key).with_extension("jpg");
            save_jpeg(&cover_crop(&im, TARGET_W, TARGET_H), &dest)?;
            manifest.insert(
                g.key.clone(),
                serde_json::json!({ "sgdb_id": gid, "source": url }),
            );
            Ok((gid, dest))
        })();
        match result {
            Ok((gid, dest)) => log::ok(&g.key, &format!("{:.1} KB  (sgdb id={gid})", file_kb(&dest))),
            Err(e) => log::fail(&g.key, &e.to_string()),
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    let count = manifest.len();
    if let Ok(json) = serde_json::to_string_pretty(&serde_json::Value::Object(manifest)) {
        let _ = std::fs::write(out.join("manifest.json"), json + "\n");
    }
    log::step("shelf", &format!("{count}/{}", games.len()), "covers → assets/games/");
}
