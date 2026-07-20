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

// ── Text metrics + overflow-proof fitting ───────────────────────────────────
//
// GitHub renders our SVGs with system fonts we cannot measure, so all text
// layout runs on a conservative per-character advance model:
//   * monospace ≈ 0.60 em per character
//   * sans      ≈ 0.52 em per character (average across a Latin mix)
//   * emoji / CJK / dingbats ≈ 1.05–1.20 em (they render on square advances)
//   * zero-width joiners / variation selectors → 0
// The model deliberately over-estimates a little: text that "fits" here fits
// on every real renderer, which is the property the cards need.

/// Advance of one char in em, given the base advance for the font family.
fn char_em(c: char, base: f64) -> f64 {
    let cp = c as u32;
    match cp {
        // Zero-width: ZWJ, ZWNJ, variation selectors, combining marks.
        0x200C..=0x200D | 0xFE00..=0xFE0F | 0x0300..=0x036F => 0.0,
        // Emoji planes render ~square regardless of family.
        0x1F000..=0x1FAFF => 1.20,
        // Misc symbols, dingbats, arrows, geometric shapes (★ ⑂ ⧗ ◌ ⭐ …).
        0x2190..=0x2BFF => 1.05,
        // CJK + Hangul + fullwidth forms.
        0x2E80..=0x9FFF | 0xAC00..=0xD7A3 | 0xF900..=0xFAFF | 0xFF00..=0xFF60 => 1.05,
        _ => base,
    }
}

/// Estimated rendered width of `text` at `font_px`, `mono` or sans.
pub fn text_width_px(text: &str, font_px: f64, mono: bool) -> f64 {
    let base = if mono { 0.60 } else { 0.52 };
    text.chars().map(|c| char_em(c, base)).sum::<f64>() * font_px
}

/// Fit `text` into `max_px`: returned unchanged when it fits, otherwise
/// truncated (on the estimate above) with a trailing ellipsis. Every card
/// routes user/data-driven strings through this so no text can cross a card
/// boundary or run under a neighbouring column.
pub fn fit_text(text: &str, max_px: f64, font_px: f64, mono: bool) -> String {
    if text_width_px(text, font_px, mono) <= max_px {
        return text.to_string();
    }
    let base = if mono { 0.60 } else { 0.52 };
    let ell_w = char_em('…', base) * font_px;
    let mut out = String::new();
    let mut used = 0.0;
    for c in text.chars() {
        let cw = char_em(c, base) * font_px;
        if used + cw + ell_w > max_px {
            break;
        }
        out.push(c);
        used += cw;
    }
    format!("{}…", out.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml_metacharacters() {
        assert_eq!(esc(r#"<a & "b">'c'"#), "&lt;a &amp; &quot;b&quot;&gt;&#x27;c&#x27;");
    }

    #[test]
    fn text_width_estimates_scale_with_font_and_family() {
        // 10 chars mono at 10px → 60px; sans → 52px.
        assert!((text_width_px("abcdefghij", 10.0, true) - 60.0).abs() < 1e-9);
        assert!((text_width_px("abcdefghij", 10.0, false) - 52.0).abs() < 1e-9);
        // Emoji are wider than Latin; ZWJ/variation selectors are free.
        assert!(text_width_px("🦀", 10.0, true) > text_width_px("a", 10.0, true));
        assert_eq!(text_width_px("\u{200d}\u{fe0f}", 12.0, true), 0.0);
    }

    #[test]
    fn fit_text_passes_short_and_truncates_long() {
        // Fits → unchanged.
        assert_eq!(fit_text("short", 200.0, 12.0, true), "short");
        // Too long → truncated with ellipsis, and the result actually fits.
        let out = fit_text("a very long string that cannot possibly fit", 80.0, 12.0, true);
        assert!(out.ends_with('…'), "got: {out}");
        assert!(text_width_px(&out, 12.0, true) <= 80.0);
        assert!(out.chars().count() < 44);
        // Degenerate budget still terminates with just the ellipsis.
        assert_eq!(fit_text("abc", 1.0, 12.0, true), "…");
        // No trailing space before the ellipsis.
        let out = fit_text("one two three four five six", 100.0, 12.0, false);
        assert!(!out.contains(" …"), "got: {out}");
    }

    #[test]
    fn validate_accepts_good_and_rejects_bad_xml() {
        assert!(validate_xml("<svg xmlns=\"http://www.w3.org/2000/svg\"><text>hi</text></svg>").is_ok());
        assert!(validate_xml("<svg><text>unclosed</svg>").is_err());
        assert!(validate_xml("<svg><text>bad & amp</text></svg>").is_err());
    }
}
