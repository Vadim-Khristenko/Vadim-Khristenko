//! Small SVG utilities: escaping, embedded media, XML validation.

#![allow(dead_code)]

use anyhow::{Context, Result};
use base64::Engine;
use std::path::{Path, PathBuf};

/// Escape text for use in SVG/XML content and attribute values.
pub fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(c),
        }
    }
    out
}

const MEDIA_EXTS: [&str; 7] = [".webp", ".gif", ".avif", ".apng", ".png", ".jpg", ".jpeg"];

fn mime_for(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "jpeg",
        Some("webp") => "webp",
        Some("gif") => "gif",
        Some("avif") => "avif",
        Some("apng") => "apng",
        _ => "png",
    }
}

/// Embed a raster/animated file as a data URI. Keeps SVGs self-contained (no
/// cross-origin fetches through GitHub's camo proxy); animated WEBP/GIF are
/// embedded byte-for-byte so they keep animating inside the card.
pub fn media_data_uri(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(format!(
        "data:image/{};base64,{}",
        mime_for(path),
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

/// Return the first existing media file for `key` (any supported extension,
/// animated formats preferred).
pub fn find_media(dir: &Path, key: &str) -> Option<PathBuf> {
    for ext in MEDIA_EXTS {
        let p = dir.join(format!("{key}{ext}"));
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// Validate that a rendered card parses as well-formed XML. Every SVG goes
/// through this before it is written — a malformed card must never land.
pub fn validate_xml(svg: &str) -> Result<()> {
    roxmltree::Document::parse_with_options(
        svg,
        roxmltree::ParsingOptions {
            allow_dtd: false,
            nodes_limit: u32::MAX,
        },
    )
    .map(|_| ())
    .context("generated SVG is not well-formed XML")
}

/// Approximate advance width of monospace text at `size` px (used for chips).
pub fn mono_w(text: &str, size: f64) -> f64 {
    text.chars().count() as f64 * size * 0.62
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml_metacharacters() {
        assert_eq!(esc(r#"<a & "b">'c'"#), "&lt;a &amp; &quot;b&quot;&gt;&#x27;c&#x27;");
    }

    #[test]
    fn validate_accepts_good_and_rejects_bad_xml() {
        assert!(validate_xml("<svg xmlns=\"http://www.w3.org/2000/svg\"><text>hi</text></svg>").is_ok());
        assert!(validate_xml("<svg><text>unclosed</svg>").is_err());
        assert!(validate_xml("<svg><text>bad & amp</text></svg>").is_err());
    }
}
