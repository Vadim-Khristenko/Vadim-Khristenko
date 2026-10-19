//! README marker injection. The engine owns whatever lives between
//! `<!-- ENGINE:KEY:START -->` and `<!-- ENGINE:KEY:END -->`; prose stays
//! untouched.
//!
//! Each image gets a `?v=<content-hash>` cache-bust token, so the README only
//! changes (and GitHub only re-fetches) when that specific asset's bytes
//! change; a short salt suffix additionally busts every 2-day cycle so
//! refreshed stats never get served stale from a sticky image cache.

use sha1::{Digest, Sha1};
use std::path::Path;

/// Cache-bust token: 10-hex content hash + optional salt suffix.
pub fn ver(assets_dir: &Path, name: &str, salt: &str) -> String {
    let h = match std::fs::read(assets_dir.join(name)) {
        Ok(bytes) => {
            let mut hasher = Sha1::new();
            hasher.update(&bytes);
            let digest = hasher.finalize();
            digest
                .iter()
                .take(5)
                .map(|b| format!("{b:02x}"))
                .collect::<String>()
        }
        Err(_) => "0".to_string(),
    };
    if salt.is_empty() {
        h
    } else {
        format!("{h}-{salt}")
    }
}

fn img(raw_base: &str, assets_dir: &Path, name: &str, alt: &str, salt: &str) -> String {
    format!(
        r#"<img src="{raw_base}/{name}?v={}" alt="{alt}" width="100%"/>"#,
        ver(assets_dir, name, salt)
    )
}

fn block(raw_base: &str, assets_dir: &Path, salt: &str, images: &[(&str, &str)]) -> String {
    let rows: Vec<String> = images
        .iter()
        .map(|(name, alt)| format!("  {}", img(raw_base, assets_dir, name, alt, salt)))
        .collect();
    format!("<p align=\"center\">\n{}\n</p>", rows.join("\n"))
}

/// Every engine-owned README block, in page order: marker key and the
/// images it holds (file, alt text). One `[[sections]]` entry adds a
/// `DIV_<KEY>` block with its labelled divider.
pub fn layout(
    sections: &[crate::config::profile::Section],
) -> Vec<(String, Vec<(String, String)>)> {
    let one = |key: &str, file: &str, alt: &str| {
        (key.to_string(), vec![(file.to_string(), alt.to_string())])
    };
    let mut out = vec![
        one("BADGES", "badges.svg", "Badges"),
        one("HEADER", "header.svg", "Vadim Khristenko"),
        one(
            "THEMES",
            "repo_miku_themes.svg",
            "Hatsune Miku editor themes",
        ),
        one("ARCHITECT", "repo_architect.svg", "Any Tech Architect"),
        one("ACAT", "repo_acat.svg", "A CAT: awg-containers-and-tools"),
        one(
            "WIKI",
            "repo_wiki.svg",
            "NotAnOtherVTUBERwiki and FilianWIKI",
        ),
        one("THEWALL", "repo_thewall.svg", "The Wall Dev"),
        one("KUMIR", "repo_kumir.svg", "Kumir 3"),
        one(
            "RESEARCH",
            "research.svg",
            "Active research: Aethelgard TQ-1.58",
        ),
        one("SNACKERS", "fandom_snackers.svg", "Snackers"),
        one("BESTGAME", "bestgame.svg", "Best game: NIKKE"),
        one("GAMES", "games.svg", "Now playing"),
        one("MUSIC", "music.svg", "Music via Last.fm"),
        one("STATS", "dashboard.svg", "Live telemetry"),
        one("PLATFORMS", "platforms.svg", "Platforms"),
        one(
            "HOMELAB",
            "homelab.svg",
            "Homelab: laptops at home, VPS out in the world",
        ),
        one(
            "FOOTER",
            "footer.svg",
            "Fine. Rendered by the VAI Profile Engine",
        ),
        one("DIV_STAFF", "divider_staff.svg", ""),
        one("DIV_WAVE", "divider_wave.svg", ""),
        one("DIV_TAILS", "divider_tails.svg", ""),
    ];
    for s in sections {
        out.push(one(
            &format!("DIV_{}", s.key.to_uppercase()),
            &format!("divider_{}.svg", s.key),
            &s.label,
        ));
    }
    out
}

/// Rendered README blocks for `inject`.
pub fn blocks(
    raw_base: &str,
    assets_dir: &Path,
    salt: &str,
    sections: &[crate::config::profile::Section],
) -> Vec<(String, String)> {
    layout(sections)
        .into_iter()
        .map(|(key, images)| {
            let refs: Vec<(&str, &str)> = images
                .iter()
                .map(|(f, a)| (f.as_str(), a.as_str()))
                .collect();
            (key, block(raw_base, assets_dir, salt, &refs))
        })
        .collect()
}

pub fn inject(text: &str, key: &str, content: &str) -> String {
    let start = format!("<!-- ENGINE:{key}:START -->");
    let end = format!("<!-- ENGINE:{key}:END -->");
    let (Some(s), Some(e)) = (text.find(&start), text.find(&end)) else {
        crate::log::warn(&format!("marker {key} not found"));
        return text.to_string();
    };
    format!("{}\n{}\n{}", &text[..s + start.len()], content, &text[e..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_replaces_only_between_markers() {
        let text = "prose\n<!-- ENGINE:X:START -->\nold\n<!-- ENGINE:X:END -->\nmore prose";
        let out = inject(text, "X", "new");
        assert_eq!(
            out,
            "prose\n<!-- ENGINE:X:START -->\nnew\n<!-- ENGINE:X:END -->\nmore prose"
        );
    }

    #[test]
    fn inject_is_idempotent() {
        let text = "a\n<!-- ENGINE:K:START -->\nz\n<!-- ENGINE:K:END -->\nb";
        let once = inject(text, "K", "content");
        let twice = inject(&once, "K", "content");
        assert_eq!(once, twice);
    }

    #[test]
    fn inject_missing_marker_leaves_text_untouched() {
        let text = "no markers here";
        assert_eq!(inject(text, "NOPE", "x"), text);
    }

    #[test]
    fn ver_hashes_content_and_salt() {
        let dir = std::env::temp_dir().join("vai_ver_test");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.svg"), b"hello").unwrap();
        let v1 = ver(&dir, "a.svg", "");
        let v2 = ver(&dir, "a.svg", "");
        assert_eq!(v1, v2);
        assert_eq!(v1.len(), 10);
        // sha1("hello") = aaf4c61ddc… → first 10 hex chars
        assert_eq!(v1, "aaf4c61ddc");
        assert_eq!(ver(&dir, "a.svg", "123"), "aaf4c61ddc-123");
        std::fs::write(dir.join("a.svg"), b"changed").unwrap();
        assert_ne!(ver(&dir, "a.svg", ""), v1);
        // Missing file → sentinel, still deterministic.
        assert_eq!(ver(&dir, "missing.svg", "s"), "0-s");
    }
}
