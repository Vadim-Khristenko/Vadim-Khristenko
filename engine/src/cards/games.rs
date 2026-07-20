//! Now-playing shelf, v2: a featured "in rotation" hero tile (rotates on the
//! same 2-day cadence as the vibe card, so the shelf and the PLAYING field
//! always agree) plus a supporting grid. Real cover art
//! (assets/games/<key>.jpg, base64) when present, neon gradient tile
//! otherwise; every tile gets a staggered light sweep and an accent frame —
//! same family, clear hierarchy.

use super::pick;
use crate::config::profile::Game;
use crate::run::Ctx;
use crate::svg::{esc, find_media, fit_text, media_data_uri};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;
use std::path::Path;

const COLS: u32 = 4;
const GAP: u32 = 16;

/// Cover art (clipped) or gradient fallback + bottom scrim, for any tile size.
fn tile_media(
    defs: &mut String,
    art_dir: &Path,
    g: &Game,
    idx: usize,
    tw: f64,
    th: u32,
    real: &mut usize,
) -> String {
    write!(
        defs,
        r##"<clipPath id="clip{idx}"><rect width="{tw:.1}" height="{th}" rx="12"/></clipPath><linearGradient id="sh{idx}" x1="0" y1="0" x2="0" y2="1"><stop offset="0.42" stop-color="#000" stop-opacity="0"/><stop offset="1" stop-color="#05040a" stop-opacity="0.93"/></linearGradient>"##
    )
    .unwrap();
    if let Some(uri) = find_media(art_dir, &g.key).and_then(|p| media_data_uri(&p).ok()) {
        *real += 1;
        format!(
            r#"<image href="{uri}" x="0" y="0" width="{tw:.1}" height="{th}" preserveAspectRatio="xMidYMid slice" clip-path="url(#clip{idx})"/><rect width="{tw:.1}" height="{th}" rx="12" fill="url(#sh{idx})"/>"#
        )
    } else {
        write!(
            defs,
            r#"<linearGradient id="gg{idx}" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="{cb}"/><stop offset="1" stop-color="{bgd}"/></linearGradient>"#,
            cb = g.cb,
            bgd = t::BG_DARK,
        )
        .unwrap();
        format!(
            r#"<rect width="{tw:.1}" height="{th}" rx="12" fill="url(#gg{idx})"/><rect width="{tw:.1}" height="{th}" rx="12" fill="url(#sh{idx})"/>"#
        )
    }
}

/// Staggered diagonal light sweep — decorative only, off-tile at rest.
fn sweep(idx: usize, tw: f64, th: u32, begin: f64) -> String {
    format!(
        r#"<g clip-path="url(#clip{idx})"><rect x="-90" y="-24" width="46" height="{h}" fill="url(#tile_sweep)" transform="skewX(-18)"><animate attributeName="x" values="-90;{end:.0}" dur="7s" begin="{begin:.1}s" repeatCount="indefinite" calcMode="spline" keySplines="0.4 0 0.2 1"/></rect></g>"#,
        h = th + 48,
        end = tw + 80.0,
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let games = &ctx.cfg.profile.games;
    let art_dir = ctx.assets.join("games");
    let w = t::CARD_W;
    let pad = t::MARGIN;
    let top = 16u32;
    let tile_w = (w - 2 * pad - (COLS - 1) * GAP) as f64 / COLS as f64;
    let tile_h = (tile_w / 2.14).round() as u32; // Steam-capsule aspect

    // Featured tile = the game the vibe card currently calls PLAYING.
    let featured = pick(games, ctx.vibe_seed, 1);
    let rest: Vec<&Game> = games.iter().filter(|g| g.key != featured.key).collect();

    // Grid cells, skipping the 2×2 block the hero occupies (top-left).
    let mut cells: Vec<(u32, u32)> = Vec::new();
    let mut row = 0u32;
    while cells.len() < rest.len() {
        for col in 0..COLS {
            if row < 2 && col < 2 {
                continue;
            }
            cells.push((col, row));
        }
        row += 1;
    }
    let rows = row.max(2);
    let h = t::BAR_H + top + rows * tile_h + (rows - 1) * GAP + 18;

    let mut defs = String::from(
        r##"<linearGradient id="tile_sweep" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#fff" stop-opacity="0"/><stop offset="0.5" stop-color="#fff" stop-opacity="0.14"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></linearGradient>"##,
    );
    let mut tiles = String::new();
    let mut real = 0usize;

    // ── Hero tile (2×2 cells) ───────────────────────────────────────────────
    let fw = 2.0 * tile_w + GAP as f64;
    let fh = 2 * tile_h + GAP;
    {
        let g = featured;
        let media = tile_media(&mut defs, &art_dir, g, 0, fw, fh, &mut real);
        let ca = &g.ca;
        write!(
            tiles,
            r##"
      <g transform="translate({x},{y})">
        {media}
        <rect width="{fw:.1}" height="{fh}" rx="12" fill="none" stroke="{ca}" stroke-width="1.8" opacity="0.95"/>
        <rect x="14" y="14" width="130" height="24" rx="12" fill="#05040a" fill-opacity="0.72" stroke="{ca}" stroke-width="1"/>
        <circle cx="28" cy="26" r="3.4" fill="{ca}"><animate attributeName="opacity" values="1;0.3;1" dur="1.8s" repeatCount="indefinite"/></circle>
        <text x="38" y="30" font-family="{mono}" font-size="11" font-weight="700" fill="{fg}" letter-spacing="1">IN ROTATION</text>
        <text x="16" y="{ny}" font-family="{sans}" font-size="12" fill="{fgd}">{note}</text>
        <text x="16" y="{sy}" font-family="{mono}" font-size="25" font-weight="800" fill="#fff" letter-spacing="0.5" filter="url(#glow)">{short}</text>
        <text x="16" y="{ty}" font-family="{sans}" font-size="12" fill="{ca}">{title}</text>
        {sweep}
      </g>"##,
            x = pad,
            y = top,
            mono = t::MONO,
            sans = t::SANS,
            fg = t::FG,
            fgd = t::FG_DIM,
            ny = fh - 56,
            sy = fh - 30,
            ty = fh - 12,
            note = esc(&fit_text(&g.note, fw - 32.0, 12.0, false)),
            short = esc(&fit_text(&g.short, fw - 32.0, 25.0, true)),
            title = esc(&fit_text(&g.title, fw - 32.0, 12.0, false)),
            sweep = sweep(0, fw, fh, 0.0),
        )?;
    }

    // ── Supporting grid ─────────────────────────────────────────────────────
    for (i, g) in rest.iter().enumerate() {
        let (col, r) = cells[i];
        let x = pad as f64 + col as f64 * (tile_w + GAP as f64);
        let y = top + r * (tile_h + GAP);
        let idx = i + 1;
        let media = tile_media(&mut defs, &art_dir, g, idx, tile_w, tile_h, &mut real);
        let ca = &g.ca;
        write!(
            tiles,
            r##"
      <g transform="translate({x:.1},{y})">
        {media}
        <rect width="{tile_w:.1}" height="{tile_h}" rx="12" fill="none" stroke="{ca}" stroke-width="1.3" opacity="0.8"/>
        <text x="14" y="{ty1}" font-family="{mono}" font-size="13" font-weight="800" fill="#fff" letter-spacing="0.5">{short}</text>
        <text x="14" y="{ty2}" font-family="{sans}" font-size="9.5" fill="{ca}">{title}</text>
        {sweep}
      </g>"##,
            ty1 = tile_h - 22,
            ty2 = tile_h - 8,
            mono = t::MONO,
            sans = t::SANS,
            short = esc(&fit_text(&g.short, tile_w - 26.0, 13.0, true)),
            title = esc(&fit_text(&g.title, tile_w - 26.0, 9.5, false)),
            sweep = sweep(idx, tile_w, tile_h, 0.8 + i as f64 * 0.55),
        )?;
    }

    let badge = if real > 0 {
        format!("{real}/{} cover art", games.len())
    } else {
        format!("{} titles", games.len())
    };
    let accent = ctx.accent("games", t::TEAL);
    let alt = format!("Now playing shelf — featuring {}", featured.title);
    let inner = format!("<defs>{defs}</defs>{tiles}");
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/now-playing",
            badge: &badge,
            accent: &accent,
            texture: Texture::Grid,
            alt: &alt,
        },
        &inner,
    );
    Ok(vec![("games.svg".into(), svg)])
}
