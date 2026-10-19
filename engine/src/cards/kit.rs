//! Shared pieces of the cards: the Miku Dark palette, the SVG frame, number
//! formatting, config lookups and the live-stats chip the project cards wear.

use crate::model::FlagshipLive;
use crate::run::Ctx;
use crate::svg::esc;
use crate::typo::{Doc, Font, T};

/// Hatsune Miku Dark, straight from the theme's `mikuV1-dark.json`.
pub mod pal {
    pub const BG: &str = "#0d0e1a";
    pub const BG_DEEP: &str = "#090a16";
    pub const STATUS: &str = "#070810";
    pub const FG: &str = "#ccd6f6";
    pub const FG_HI: &str = "#eef2ff";
    pub const MUTED: &str = "#4a5280";
    /// Readable secondary text on BG (the theme's comment colour is too dim
    /// for information).
    pub const DIM: &str = "#8a93c4";
    pub const LINE: &str = "#3d4470";
    pub const TEAL: &str = "#39c5bb";
    pub const KW: &str = "#ff6e8a";
    pub const VIOLET: &str = "#d4a0ff";
    pub const SAKURA: &str = "#e87ea1";
    pub const AMBER: &str = "#e8a840";
}

pub const W: f64 = 1000.0;

/// Every animation stops for readers who asked for less motion.
pub const REDUCED: &str = "@media (prefers-reduced-motion:reduce){*{animation:none!important}}";

/// Assemble a card. Glyph outlines from `doc` go into `<defs>` together with
/// the card's own `defs`.
pub fn svg(w: f64, h: f64, label: &str, css: &str, defs: &str, doc: &Doc, body: &str) -> String {
    let label = esc(label);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" role="img" aria-label="{label}"><title>{label}</title><style>{css}{REDUCED}</style><defs>{defs}{glyphs}</defs>{body}</svg>"#,
        glyphs = doc.defs()
    )
}

/// 1483 → "1 483", the way the game and the concept cards print numbers.
pub fn group(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

/// A `[showcase.<card>]` string with a fallback.
pub fn show(ctx: &Ctx, card: &str, key: &str, default: &str) -> String {
    ctx.cfg
        .profile
        .showcase
        .get(card)
        .and_then(|t| t.get(key))
        .map(|v| match v {
            toml::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
        .unwrap_or_else(|| default.to_string())
}

/// A `[showcase.<card>]` list of strings.
pub fn show_list(ctx: &Ctx, card: &str, key: &str) -> Vec<String> {
    ctx.cfg
        .profile
        .showcase
        .get(card)
        .and_then(|t| t.get(key))
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

pub fn show_num(ctx: &Ctx, card: &str, key: &str, default: i64) -> i64 {
    ctx.cfg
        .profile
        .showcase
        .get(card)
        .and_then(|t| t.get(key))
        .and_then(|v| v.as_integer())
        .unwrap_or(default)
}

/// Live stats of a flagship repo, when the providers found it.
pub fn live<'a>(ctx: &'a Ctx, repo: &str) -> Option<&'a FlagshipLive> {
    ctx.flagship
        .iter()
        .find(|f| f.repo_key.eq_ignore_ascii_case(repo) && f.repo.is_some())
}

/// "★ 24 · 4 forks · 41 commits" right-aligned at (x, y). Empty when the repo
/// is not live, so a card never shows made-up numbers.
pub fn live_chip(
    doc: &mut Doc,
    ctx: &Ctx,
    repo: &str,
    x: f64,
    y: f64,
    ink: &str,
    accent: &str,
) -> String {
    let Some(l) = live(ctx, repo) else {
        return String::new();
    };
    let t = T::new(Font::Mono, 11.0).fill(ink);
    let star = T::new(Font::MonoBold, 11.0).fill(accent);
    let mut runs: Vec<(String, &T)> = vec![(format!("★ {}", group(l.stars)), &star)];
    runs.push((format!("  ·  {} forks", group(l.forks)), &t));
    if let Some(c) = l.pulse.total_commits {
        runs.push((format!("  ·  {} commits", group(c)), &t));
    }
    let refs: Vec<(&str, &T)> = runs.iter().map(|(s, t)| (s.as_str(), *t)).collect();
    let total: f64 = refs.iter().map(|(s, t)| crate::typo::width(s, t)).sum();
    doc.spans(x - total, y, &refs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_inserts_spaces_by_thousands() {
        assert_eq!(group(7), "7");
        assert_eq!(group(1483), "1 483");
        assert_eq!(group(175375), "175 375");
        assert_eq!(group(1234567), "1 234 567");
    }
}
