//! Now-playing shelf. Embeds real cover art (assets/games/<key>.jpg, base64)
//! when present; otherwise falls back to a neon tile — all tiles share one
//! capsule shape so the shelf stays "one type" either way.

use crate::run::Ctx;
use crate::svg::{esc, find_media, media_data_uri};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let games = &ctx.cfg.profile.games;
    let art_dir = ctx.assets.join("games");
    let w = t::CARD_W;
    let (cols, gap, pad, top) = (4u32, 16u32, t::MARGIN, 16u32);
    let tile_w = (w - 2 * pad - (cols - 1) * gap) as f64 / cols as f64;
    let tile_h = (tile_w / 2.14).round() as u32; // Steam-capsule aspect
    let rows = (games.len() as u32 + cols - 1) / cols;
    let h = 42 + top + rows * tile_h + (rows - 1) * gap + 18;

    let mut defs = String::new();
    let mut tiles = String::new();
    let mut real = 0usize;
    for (i, g) in games.iter().enumerate() {
        let (r, col) = (i as u32 / cols, i as u32 % cols);
        let x = pad as f64 + col as f64 * (tile_w + gap as f64);
        let y = top + r * (tile_h + gap);
        let ca = &g.ca;
        write!(
            defs,
            r##"<clipPath id="clip{i}"><rect width="{tile_w:.1}" height="{tile_h}" rx="11"/></clipPath><linearGradient id="sh{i}" x1="0" y1="0" x2="0" y2="1"><stop offset="0.45" stop-color="#000" stop-opacity="0"/><stop offset="1" stop-color="#05040a" stop-opacity="0.92"/></linearGradient>"##
        )?;
        let art = find_media(&art_dir, &g.key).and_then(|p| media_data_uri(&p).ok());
        let body = if let Some(uri) = art {
            real += 1;
            format!(
                r#"<image href="{uri}" x="0" y="0" width="{tile_w:.1}" height="{tile_h}" preserveAspectRatio="xMidYMid slice" clip-path="url(#clip{i})"/><rect width="{tile_w:.1}" height="{tile_h}" rx="11" fill="url(#sh{i})"/>"#
            )
        } else {
            write!(
                defs,
                r#"<linearGradient id="gg{i}" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="{cb}"/><stop offset="1" stop-color="{bgd}"/></linearGradient>"#,
                cb = g.cb,
                bgd = t::BG_DARK,
            )?;
            format!(r#"<rect width="{tile_w:.1}" height="{tile_h}" rx="11" fill="url(#gg{i})"/>"#)
        };
        write!(
            tiles,
            r##"
      <g transform="translate({x:.1},{y})">
        {body}
        <rect width="{tile_w:.1}" height="{tile_h}" rx="11" fill="none" stroke="{ca}" stroke-width="1.3" opacity="0.85"/>
        <rect x="0" y="0" width="4" height="{tile_h}" rx="2" fill="{ca}" filter="url(#glow)"/>
        <text x="14" y="{ty1}" font-family="{mono}" font-size="13" font-weight="800" fill="#fff" letter-spacing="0.5">{short}</text>
        <text x="14" y="{ty2}" font-family="{sans}" font-size="9.5" fill="{ca}">{title}</text>
      </g>"##,
            ty1 = tile_h - 14,
            ty2 = tile_h - 2,
            mono = t::MONO,
            sans = t::SANS,
            short = esc(&g.short),
            title = esc(&g.title),
        )?;
    }

    let badge = if real > 0 {
        format!("{real}/{} cover art", games.len())
    } else {
        format!("{} titles", games.len())
    };
    let inner = format!("<defs>{defs}</defs>{tiles}");
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/now-playing",
            badge: &badge,
            accent: t::TEAL,
            texture: Texture::Grid,
            alt: "Now playing: the current game shelf",
        },
        &inner,
    );
    Ok(vec![("games.svg".into(), svg)])
}
