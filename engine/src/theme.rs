//! Design system for the VAI Profile Engine.
//! =========================================
//!
//! Think of this module as the stylesheet + component library of a tiny
//! website. Every "card" is a window pane in a Tokyo-Night-themed IDE, framed
//! with a single clean animated neon border, a faint grid + ternary {-1,0,+1}
//! texture (a nod to 1.58-bit research) and a tidy title bar. Rust orange is
//! the signature accent.
//!
//! COMPOSITION RULES (so everything stacks cleanly, especially on mobile):
//!   * Every card is exactly CARD_W wide. GitHub scales all images to the
//!     column width uniformly, so identical intrinsic widths ⇒ identical
//!     rendered widths.
//!   * The chrome is: gradient border + title bar (dots · title · badge)
//!     + content. 24px content margin, 42px title bar, 8pt spacing grid.
//!   * CONTRAST TIERS (all ≥ 4.5:1 on the panel): FG for headings, FG_DIM for
//!     body, MUTED for labels. COMMENT is *decorative only* (textures,
//!     non-essential flourishes) — never for information-bearing text.
//!   * STATIC-FIRST: GitHub sometimes freezes SVG animation at frame 0, so
//!     every card must be fully legible with no animation at all. Loops may
//!     enhance; entrance fades from opacity 0 are banned.

#![allow(dead_code)]

use crate::svg::esc;

// ── Palette — Tokyo Night (night variant) + Rust signature ──────────────────

pub const BG: &str = "#1a1b26";
pub const BG_DARK: &str = "#15161e";
pub const BG_PANEL: &str = "#1f2335";
pub const BG_HL: &str = "#292e42";
pub const FG: &str = "#c0caf5";
pub const FG_DIM: &str = "#a9b1d6";
/// Label tier — information-bearing secondary text (≈6:1 on the panel).
pub const MUTED: &str = "#9aa5ce";
/// Decorative tier only (grid lines, textures). Not for readable text.
pub const COMMENT: &str = "#565f89";
pub const BLUE: &str = "#7aa2f7";
pub const CYAN: &str = "#7dcfff";
pub const PURPLE: &str = "#bb9af7";
pub const GREEN: &str = "#9ece6a";
pub const RED: &str = "#f7768e";
pub const ORANGE: &str = "#ff9e64";
pub const YELLOW: &str = "#e0af68";
pub const TEAL: &str = "#2ac3de";
pub const MAGENTA: &str = "#c678dd";

// Rust — the primary accent.
pub const RUST: &str = "#dea584";
pub const RUST_DEEP: &str = "#e06c47";
pub const RUST_BRAND: &str = "#ce422b";

// One width to rule them all (uniform mobile scaling) + the 8pt grid anchors.
pub const CARD_W: u32 = 1000;
pub const MARGIN: u32 = 24;
pub const BAR_H: u32 = 42;

/// Font stacks. Web fonts don't load inside GitHub-proxied SVG, so we lean on
/// system monospace (terminal/Rust vibe) with broad fallbacks.
pub const MONO: &str = "'JetBrains Mono','Cascadia Code','Fira Code','SFMono-Regular','Consolas','Liberation Mono',monospace";
pub const SANS: &str = "'Segoe UI','Inter',system-ui,-apple-system,'Helvetica Neue',sans-serif";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Texture {
    Grid,
    Ternary,
    Plain,
}

// ── Shared <defs>: gradients, glow, grid ────────────────────────────────────

fn defs(accent: &str) -> String {
    // Border gradient: slow, eased multi-stop colour drift (spline ease-in-out
    // between stops instead of a linear snap).
    let spline = r#"calcMode="spline" keySplines="0.4 0 0.2 1;0.4 0 0.2 1;0.4 0 0.2 1;0.4 0 0.2 1""#;
    format!(
        r##"
  <defs>
    <linearGradient id="panel" x1="0" y1="0" x2="0.5" y2="1">
      <stop offset="0" stop-color="{BG_PANEL}"/>
      <stop offset="1" stop-color="{BG_DARK}"/>
    </linearGradient>
    <linearGradient id="border" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="{accent}">
        <animate attributeName="stop-color" values="{accent};{BLUE};{PURPLE};{CYAN};{accent}" keyTimes="0;0.28;0.55;0.8;1" {spline} dur="18s" repeatCount="indefinite"/>
      </stop>
      <stop offset="0.5" stop-color="{PURPLE}">
        <animate attributeName="stop-color" values="{PURPLE};{CYAN};{accent};{BLUE};{PURPLE}" keyTimes="0;0.28;0.55;0.8;1" {spline} dur="18s" repeatCount="indefinite"/>
      </stop>
      <stop offset="1" stop-color="{BLUE}">
        <animate attributeName="stop-color" values="{BLUE};{accent};{CYAN};{PURPLE};{BLUE}" keyTimes="0;0.28;0.55;0.8;1" {spline} dur="18s" repeatCount="indefinite"/>
      </stop>
    </linearGradient>
    <linearGradient id="accentbar" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{accent}"/>
      <stop offset="1" stop-color="{accent}" stop-opacity="0"/>
    </linearGradient>
    <linearGradient id="tophl" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#ffffff" stop-opacity="0.05"/>
      <stop offset="1" stop-color="#ffffff" stop-opacity="0"/>
    </linearGradient>
    <filter id="glow" x="-40%" y="-40%" width="180%" height="180%">
      <feGaussianBlur stdDeviation="1.7" result="b"/>
      <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    <filter id="softglow" x="-60%" y="-60%" width="220%" height="220%">
      <feGaussianBlur stdDeviation="4.5" result="b"/>
      <feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    <pattern id="grid" width="28" height="28" patternUnits="userSpaceOnUse">
      <path d="M28 0 L0 0 0 28" fill="none" stroke="{COMMENT}" stroke-width="0.5" opacity="0.08"/>
    </pattern>
  </defs>"##
    )
}

/// Scattered ternary glyphs {-1, 0, +1} — the 1.58-bit signature motif.
/// Restrained flicker: a few glyphs breathe, most stay put; always faint.
fn ternary_texture(w: u32, h: u32, dense: bool) -> String {
    let glyphs = ["−1", "0", "+1", "−1", "0", "+1", "1", "0"];
    let (step_x, step_y) = if dense { (58, 30) } else { (130, 76) };
    let base_op = if dense { 0.06 } else { 0.045 };
    let mut out = String::new();
    let mut i: usize = 0;
    let mut y = BAR_H + 22;
    while y < h.saturating_sub(8) {
        let mut x = 24;
        while x < w.saturating_sub(18) {
            let g = glyphs[i % glyphs.len()];
            let flick = if dense && i % 5 == 0 {
                format!(
                    r#"<animate attributeName="opacity" values="{base_op};0.22;{base_op}" dur="{}s" begin="{:.1}s" repeatCount="indefinite"/>"#,
                    3 + i % 4,
                    (i % 7) as f64 * 0.4
                )
            } else {
                String::new()
            };
            out.push_str(&format!(
                r#"<text x="{x}" y="{y}" font-family="{MONO}" font-size="11" fill="{PURPLE}" opacity="{base_op}">{g}{flick}</text>"#
            ));
            x += step_x;
            i += 1;
        }
        y += step_y;
    }
    out
}

// ── The window-pane chrome shared by every card ─────────────────────────────

pub struct CardSpec<'a> {
    pub w: u32,
    pub h: u32,
    /// Title-bar text, e.g. "~/telemetry.dash".
    pub title: &'a str,
    /// Right-side title-bar badge ("" = none).
    pub badge: &'a str,
    pub accent: &'a str,
    pub texture: Texture,
    /// Accessible description (role="img" + <title>).
    pub alt: &'a str,
}

/// Compose a complete standalone SVG card with the shared chrome: panel +
/// grid/ternary texture + animated neon border + inner hairline + top
/// highlight + title bar (traffic-light dots · mono title · optional badge).
/// `inner` is placed in a group translated below the title bar.
pub fn card(spec: &CardSpec, inner: &str) -> String {
    let CardSpec {
        w,
        h,
        title,
        badge,
        accent,
        texture,
        alt,
    } = *spec;
    let tex = match texture {
        Texture::Plain => String::new(),
        Texture::Grid | Texture::Ternary => format!(
            r#"<rect x="6" y="6" width="{}" height="{}" rx="13" fill="url(#grid)"/>{}"#,
            w - 12,
            h - 12,
            ternary_texture(w, h, texture == Texture::Ternary)
        ),
    };

    let dots: String = [RED, YELLOW, GREEN]
        .iter()
        .enumerate()
        .map(|(i, c)| {
            format!(
                r#"<circle cx="{}" cy="{}" r="5.5" fill="{c}" opacity="0.92"/>"#,
                MARGIN + i as u32 * 20,
                BAR_H / 2
            )
        })
        .collect();

    let title_x = MARGIN + 3 * 20 + 12;
    let badge_node = if badge.is_empty() {
        String::new()
    } else {
        format!(
            r#"<circle cx="{cx}" cy="{cy}" r="3.5" fill="{accent}"><animate attributeName="opacity" values="1;0.4;1" dur="2.6s" repeatCount="indefinite"/></circle><text x="{tx}" y="{ty}" text-anchor="end" font-family="{MONO}" font-size="12" fill="{MUTED}">{}</text>"#,
            esc(badge),
            cx = w - MARGIN - 1,
            cy = BAR_H / 2,
            tx = w - MARGIN - 12,
            ty = BAR_H / 2 + 4,
        )
    };

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}" role="img" aria-label="{alt_esc}" font-family="{SANS}">
  <title>{alt_esc}</title>
  {defs}
  <rect x="2" y="2" width="{wp}" height="{hp}" rx="15" fill="url(#panel)"/>
  {tex}
  <rect x="2" y="2" width="{wp}" height="24" rx="15" fill="url(#tophl)"/>
  <rect x="2.5" y="2.5" width="{wb}" height="{hb}" rx="14" fill="none" stroke="url(#border)" stroke-width="2"/>
  <rect x="3.5" y="3.5" width="{wi}" height="{hi}" rx="13" fill="none" stroke="{BG_HL}" stroke-width="1" opacity="0.6"/>
  <path d="M2 18 Q2 2 18 2" fill="none" stroke="{accent}" stroke-width="2.4" opacity="0.9"/>
  {dots}
  <text x="{title_x}" y="{title_y}" font-family="{MONO}" font-size="13" fill="{FG_DIM}">{title_esc}</text>
  {badge_node}
  <rect x="{m}" y="{bar}" width="{aw}" height="2" fill="url(#accentbar)"/>
  <g transform="translate(0,{bar})">
    {inner}
  </g>
</svg>"#,
        alt_esc = esc(alt),
        defs = defs(accent),
        wp = w - 4,
        hp = h - 4,
        wb = w - 5,
        hb = h - 5,
        wi = w - 7,
        hi = h - 7,
        title_y = BAR_H / 2 + 4,
        title_esc = esc(title),
        m = MARGIN,
        bar = BAR_H,
        aw = w - 2 * MARGIN,
    )
}

/// Colour for a language in bars/legends/radars.
pub fn lang_color(lang: &str) -> &'static str {
    match lang {
        "Rust" => RUST,
        "Python" => "#7aa2f7",
        "C++" => "#f7768e",
        "TypeScript" => "#7dcfff",
        "JavaScript" => "#e0af68",
        "Java" => "#ff9e64",
        "Kotlin" => "#bb9af7",
        "Cython" => "#fedf5b",
        "HTML" => "#f7768e",
        "CSS" => "#7dcfff",
        "Vue" => "#9ece6a",
        "Shell" => "#9ece6a",
        "C" => "#9aa5ce",
        "Go" => "#2ac3de",
        "Dockerfile" => "#7dcfff",
        "Makefile" => "#565f89",
        "Svelte" => "#ff5d2b",
        "SCSS" => "#c6538c",
        "Jupyter Notebook" => "#da5b0b",
        _ => PURPLE,
    }
}
