//! Best game — a configurable spotlight (config/profile.toml [best_game]).
//!
//! Two cover modes: "portrait" (cover as a 2:3 panel on the left) and
//! "landscape" (cover full-bleed behind the text). Up to 5 favourite
//! characters are laid out automatically. Cover + character art may be remote
//! URLs or local files under assets/bestgame/, including animated WEBP/GIF
//! (embedded as-is); missing character art falls back to an initials avatar.

use crate::config::profile::{BestGame, Character};
use crate::run::Ctx;
use crate::svg::{esc, find_media, media_data_uri};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;
use std::path::Path;

fn media(dir: &Path, key: &str) -> Option<String> {
    find_media(dir, key).and_then(|p| media_data_uri(&p).ok())
}

fn chip(x: f64, y: f64, label: &str, value: &str, accent: &str) -> (String, f64) {
    let lw = label.chars().count() as f64 * 7.0;
    let vw = value.chars().count() as f64 * 9.0;
    let w = 18.0 + lw + 8.0 + vw;
    (
        format!(
            r##"<g transform="translate({x:.0},{y:.0})"><rect x="0" y="-17" width="{w:.0}" height="25" rx="12.5" fill="#0c0a12" fill-opacity="0.55" stroke="{accent}" stroke-width="0.9"/><text x="11" y="0" font-family="{mono}" font-size="11" fill="{muted}">{l}</text><text x="{vx:.0}" y="0" font-family="{mono}" font-size="12.5" font-weight="700" fill="{fg}">{v}</text></g>"##,
            mono = t::MONO,
            muted = t::MUTED,
            fg = t::FG,
            l = esc(label),
            vx = 11.0 + lw + 6.0,
            v = esc(value),
        ),
        w,
    )
}

fn avatar(art_dir: &Path, ch: &Character, cx: i64, cy: i64, r: i64) -> String {
    let key = ch.key();
    let accent = if ch.accent.is_empty() { t::RUST } else { &ch.accent };
    let cid = format!("clip_{key}");
    let ring = format!(
        r#"<circle cx="{cx}" cy="{cy}" r="{ro}" fill="none" stroke="{accent}" stroke-width="2" opacity="0.9"/><circle cx="{cx}" cy="{cy}" r="{ro}" fill="none" stroke="{accent}" stroke-width="2" opacity="0.3" filter="url(#softglow)"/>"#,
        ro = r + 3,
    );
    let inner = if let Some(uri) = media(art_dir, &key) {
        format!(
            r#"<clipPath id="{cid}"><circle cx="{cx}" cy="{cy}" r="{r}"/></clipPath><circle cx="{cx}" cy="{cy}" r="{r}" fill="{bghl}"/><image href="{uri}" x="{ix}" y="{iy}" width="{d}" height="{d}" preserveAspectRatio="xMidYMid slice" clip-path="url(#{cid})"/>"#,
            bghl = t::BG_HL,
            ix = cx - r,
            iy = cy - r,
            d = 2 * r,
        )
    } else {
        let initials: String = ch
            .short
            .split_whitespace()
            .take(2)
            .filter_map(|w| w.chars().next())
            .collect::<String>()
            .to_uppercase();
        format!(
            r#"<circle cx="{cx}" cy="{cy}" r="{r}" fill="{bghl}"/><text x="{cx}" y="{ty}" text-anchor="middle" font-family="{mono}" font-size="{fs}" font-weight="800" fill="{accent}">{init}</text>"#,
            bghl = t::BG_HL,
            ty = cy + 7,
            mono = t::MONO,
            fs = (r as f64 * 0.62) as i64,
            init = esc(&initials),
        )
    };
    let label = format!(
        r#"<text x="{cx}" y="{ly}" text-anchor="middle" font-family="{mono}" font-size="11" fill="{fgd}">{name}</text>"#,
        mono = t::MONO,
        ly = cy + r + 18,
        fgd = t::FG_DIM,
        name = esc(&ch.short),
    );
    let star = format!(
        r#"<text x="{sx}" y="{sy}" font-family="{sans}" font-size="12">⭐</text>"#,
        sx = cx + r - 3,
        sy = cy - r + 5,
        sans = t::SANS,
    );
    format!("{inner}{ring}{star}{label}")
}

fn squad(
    art_dir: &Path,
    x0: f64,
    total_w: f64,
    cy: i64,
    chars: &[Character],
    accent: &str,
    label_x: u32,
    label_y: i64,
) -> String {
    let n = chars.len().max(1);
    let slot = total_w / n as f64;
    let r = (slot / 2.0 - 16.0).clamp(20.0, 30.0) as i64;
    let mut out = format!(
        r#"<text x="{label_x}" y="{label_y}" font-family="{mono}" font-size="12" fill="{accent}" letter-spacing="2">★ FAVOURITE SQUAD</text>"#,
        mono = t::MONO,
    );
    for (i, ch) in chars.iter().enumerate() {
        let cx = x0 + slot * (i as f64 + 0.5);
        out.push_str(&avatar(art_dir, ch, cx.round() as i64, cy, r));
    }
    out
}

fn chips(x: f64, y: f64, g: &BestGame, accent: &str) -> String {
    let mut data: Vec<(&str, &str)> = Vec::new();
    if !g.nick.is_empty() {
        data.push(("NICK", &g.nick));
    }
    data.push(("LV", &g.level));
    data.push(("SERVER", &g.server));
    data.push(("ID", &g.game_id));
    let mut out = String::new();
    let mut cx = x;
    for (label, value) in data {
        if value.is_empty() {
            continue;
        }
        let (node, cw) = chip(cx, y, label, value, accent);
        out.push_str(&node);
        cx += cw + 10.0;
    }
    out
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let g = &ctx.cfg.profile.best_game;
    let art_dir = ctx.assets.join("bestgame");
    let w = t::CARD_W;
    let h = 364u32;
    let accent = if g.accent.is_empty() { t::RUST } else { &g.accent };
    let inner_h = h - t::BAR_H;
    let chars: Vec<Character> = g.characters.iter().take(5).cloned().collect();
    let cover = media(&art_dir, "cover");
    let m = t::MARGIN as f64;

    let mut defs = String::from(
        r##"<linearGradient id="cover_sheen" x1="0" y1="0" x2="0" y2="1"><stop offset="0.55" stop-color="#0a0810" stop-opacity="0"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.6"/></linearGradient><linearGradient id="land_scrim" x1="0" y1="0" x2="1" y2="0.2"><stop offset="0" stop-color="#0a0810" stop-opacity="0.95"/><stop offset="0.6" stop-color="#0a0810" stop-opacity="0.72"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.42"/></linearGradient><linearGradient id="land_bottom" x1="0" y1="0" x2="0" y2="1"><stop offset="0.45" stop-color="#0a0810" stop-opacity="0"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.92"/></linearGradient>"##,
    );

    let body = if g.cover_mode == "landscape" {
        write!(defs, r#"<clipPath id="bgclip"><rect width="{w}" height="{inner_h}"/></clipPath>"#)?;
        let bg = if let Some(uri) = &cover {
            format!(
                r#"<image href="{uri}" x="0" y="0" width="{w}" height="{inner_h}" preserveAspectRatio="xMidYMid slice" clip-path="url(#bgclip)"/><rect width="{w}" height="{inner_h}" fill="url(#land_scrim)"/><rect width="{w}" height="{inner_h}" fill="url(#land_bottom)"/>"#
            )
        } else {
            format!(r#"<rect width="{w}" height="{inner_h}" fill="url(#land_bottom)"/>"#)
        };
        let squad_s = squad(
            &art_dir,
            m,
            w as f64 - 2.0 * m,
            (inner_h - 58) as i64,
            &chars,
            accent,
            t::MARGIN,
            (inner_h - 106) as i64,
        );
        format!(
            r#"
        {bg}
        <text x="{m:.0}" y="44" font-family="{mono}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{m:.0}" y="92" font-family="{mono}" font-size="46" font-weight="800" fill="{fg}" filter="url(#softglow)">{title}</text>
        <text x="{m:.0}" y="116" font-family="{mono}" font-size="13" fill="{fgd}" letter-spacing="2">{subtitle}</text>
        {chips_s}
        <text x="{m:.0}" y="182" font-family="{sans}" font-size="13" fill="{fgd}">{blurb}</text>
        {squad_s}"#,
            mono = t::MONO,
            sans = t::SANS,
            fg = t::FG,
            fgd = t::FG_DIM,
            title = esc(&g.title),
            subtitle = esc(&g.subtitle),
            chips_s = chips(m, 150.0, g, accent),
            blurb = esc(&g.blurb),
        )
    } else {
        // portrait
        let pad = 14u32;
        let cw = 180u32;
        let chh = inner_h - 2 * pad;
        let (cx0, cy0) = (t::MARGIN, pad);
        write!(
            defs,
            r#"<clipPath id="coverclip"><rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12"/></clipPath>"#
        )?;
        let mut panel = if let Some(uri) = &cover {
            format!(
                r#"<image href="{uri}" x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" preserveAspectRatio="xMidYMid slice" clip-path="url(#coverclip)"/><rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="url(#cover_sheen)"/>"#
            )
        } else {
            format!(
                r#"<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="{bghl}"/><text x="{tx}" y="{ty}" text-anchor="middle" font-family="{mono}" font-size="13" fill="{muted}">no cover</text>"#,
                bghl = t::BG_HL,
                tx = cx0 + cw / 2,
                ty = cy0 + chh / 2,
                mono = t::MONO,
                muted = t::MUTED,
            )
        };
        write!(
            panel,
            r#"<rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="none" stroke="{accent}" stroke-width="1.5" opacity="0.85"/>"#
        )?;
        let rx = (t::MARGIN + cw + 28) as f64;
        let avail = w as f64 - rx - m;
        let squad_s = squad(&art_dir, rx, avail, 268, &chars, accent, rx as u32, 212);
        format!(
            r#"
        {panel}
        <text x="{rx:.0}" y="44" font-family="{mono}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{rx:.0}" y="92" font-family="{mono}" font-size="44" font-weight="800" fill="{fg}" filter="url(#softglow)">{title}</text>
        <text x="{rx:.0}" y="116" font-family="{mono}" font-size="13" fill="{fgd}" letter-spacing="2">{subtitle}</text>
        {chips_s}
        <text x="{rx:.0}" y="184" font-family="{sans}" font-size="13" fill="{fgd}">{blurb}</text>
        {squad_s}"#,
            mono = t::MONO,
            sans = t::SANS,
            fg = t::FG,
            fgd = t::FG_DIM,
            title = esc(&g.title),
            subtitle = esc(&g.subtitle),
            chips_s = chips(rx, 152.0, g, accent),
            blurb = esc(&g.blurb),
        )
    };

    let inner = format!("<defs>{defs}</defs>{body}");
    let alt = format!("Best game: {}", g.title);
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/best-game.cfg",
            badge: &g.title,
            accent,
            texture: Texture::Plain,
            alt: &alt,
        },
        &inner,
    );
    Ok(vec![("bestgame.svg".into(), svg)])
}
