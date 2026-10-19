//! Small SVG utilities: escaping, media lookup, XML validation.

use anyhow::{Context, Result};
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
/// through this before it is written: a malformed card must never land.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_xml_metacharacters() {
        assert_eq!(
            esc(r#"<a & "b">'c'"#),
            "&lt;a &amp; &quot;b&quot;&gt;&#x27;c&#x27;"
        );
    }

    #[test]
    fn validate_accepts_good_and_rejects_bad_xml() {
        assert!(
            validate_xml("<svg xmlns=\"http://www.w3.org/2000/svg\"><text>hi</text></svg>").is_ok()
        );
        assert!(validate_xml("<svg><text>unclosed</svg>").is_err());
        assert!(validate_xml("<svg><text>bad & amp</text></svg>").is_err());
    }
}
