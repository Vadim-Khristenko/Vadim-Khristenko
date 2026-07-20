//! Best game — a configurable spotlight (config/profile.toml [best_game]).
//!
//! Two cover modes: "portrait" (cover as a 2:3 panel on the left) and
//! "landscape" (cover full-bleed behind the text). Up to 5 favourite
//! characters are laid out automatically, and any number of user-defined
//! `[[best_game.extra]]` stats render as labelled plates — with a micro
//! progress bar when `current`/`max` are given. Cover + character art may be
//! remote URLs or local files under assets/bestgame/ (animated WEBP/GIF are
//! embedded as-is); missing character art falls back to an initials avatar.

use crate::config::profile::{BestGame, Character, ExtraStat};
use crate::run::Ctx;
use crate::svg::{esc, find_media, fit_text, media_data_uri, text_width_px};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;
use std::path::Path;

fn media(dir: &Path, key: &str) -> Option<String> {
    find_media(dir, key).and_then(|p| media_data_uri(&p).ok())
}

fn chip(x: f64, y: f64, label: &str, value: &str, accent: &str) -> (String, f64) {
    let label = fit_text(label, 90.0, 11.0, true);
    let value = fit_text(value, 150.0, 12.5, true);
    let lw = text_width_px(&label, 11.0, true);
    let vw = text_width_px(&value, 12.5, true);
    let w = 22.0 + lw + 8.0 + vw;
    (
        format!(
            r##"<g transform="translate({x:.0},{y:.0})"><rect x="0" y="-17" width="{w:.0}" height="25" rx="12.5" fill="#0c0a12" fill-opacity="0.55" stroke="{accent}" stroke-width="0.9"/><text x="11" y="0" font-family="{mono}" font-size="11" fill="{muted}">{l}</text><text x="{vx:.0}" y="0" font-family="{mono}" font-size="12.5" font-weight="700" fill="{fg}">{v}</text></g>"##,
            mono = t::MONO,
            muted = t::MUTED,
            fg = t::FG,
            l = esc(&label),
            vx = 11.0 + lw + 8.0,
            v = esc(&value),
        ),
        w,
    )
}

fn avatar(art_dir: &Path, ch: &Character, cx: i64, cy: i64, r: i64, slot: f64) -> String {
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
    let name = fit_text(&ch.short, slot - 6.0, 11.0, true);
    let label = format!(
        r#"<text x="{cx}" y="{ly}" text-anchor="middle" font-family="{mono}" font-size="11" fill="{fgd}">{name}</text>"#,
        mono = t::MONO,
        ly = cy + r + 18,
        fgd = t::FG_DIM,
        name = esc(&name),
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
        out.push_str(&avatar(art_dir, ch, cx.round() as i64, cy, r, slot));
    }
    out
}

fn chips(x: f64, y: f64, max_x: f64, g: &BestGame, accent: &str) -> String {
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
        if cx + cw > max_x {
            break;
        }
        out.push_str(&node);
        cx += cw + 10.0;
    }
    out
}

/// User-defined stat plates: LABEL over value, plus a micro progress bar when
/// `current`/`max` are configured. Plates flow left→right and stop cleanly at
/// `max_x` — configuration can never push them off the card.
fn extras_row(x: f64, y: f64, max_x: f64, extras: &[ExtraStat], accent: &str) -> String {
    let mut out = String::new();
    let mut cx = x;
    for ex in extras {
        let label = fit_text(&ex.label.to_uppercase(), 110.0, 10.0, true);
        let value = fit_text(&ex.display(), 140.0, 14.0, true);
        let lw = text_width_px(&label, 10.0, true);
        let vw = text_width_px(&value, 14.0, true);
        let bar = ex.fraction().map(|f| (f, 64.0f64));
        let content_w = lw.max(vw).max(bar.map_or(0.0, |b| b.1));
        let pw = content_w + 24.0;
        if cx + pw > max_x {
            break;
        }
        write!(
            out,
            r##"<g transform="translate({cx:.0},{y:.0})"><rect x="0" y="0" width="{pw:.0}" height="52" rx="9" fill="#0c0a12" fill-opacity="0.5" stroke="{bghl}" stroke-width="1"/><rect x="0" y="0" width="3" height="52" rx="1.5" fill="{accent}" opacity="0.9"/><text x="12" y="17" font-family="{mono}" font-size="10" fill="{muted}" letter-spacing="1">{label}</text><text x="12" y="37" font-family="{mono}" font-size="14" font-weight="700" fill="{fg}">{value}</text>"##,
            bghl = t::BG_HL,
            mono = t::MONO,
            muted = t::MUTED,
            fg = t::FG,
            label = esc(&label),
            value = esc(&value),
        )
        .unwrap();
        if let Some((frac, bw)) = bar {
            write!(
                out,
                r#"<rect x="12" y="43" width="{bw:.0}" height="4" rx="2" fill="{bghl}"/><rect x="12" y="43" width="{fw:.1}" height="4" rx="2" fill="{accent}"/>"#,
                bghl = t::BG_HL,
                fw = bw * frac,
            )
            .unwrap();
        }
        out.push_str("</g>");
        cx += pw + 10.0;
    }
    out
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let g = &ctx.cfg.profile.best_game;
    let art_dir = ctx.assets.join("bestgame");
    let w = t::CARD_W;
    let has_extras = !g.extra.is_empty();
    // The extras row costs one 62px band; without it the card stays compact.
    let h: u32 = if has_extras { 428 } else { 366 };
    let accent = if g.accent.is_empty() { t::RUST } else { &g.accent };
    let inner_h = h - t::BAR_H;
    let chars: Vec<Character> = g.characters.iter().take(5).cloned().collect();
    let cover = media(&art_dir, "cover");
    let m = t::MARGIN as f64;
    // Vertical anchors shared by both cover modes.
    // Avatars: max radius 30 (+3 ring) and a name baseline at cy+r+18, so
    // cy = inner_h − 52 keeps everything inside the pane.
    let squad_cy = (inner_h - 52) as i64;
    let (extras_y, squad_label_y) = if has_extras {
        (206.0, 288i64)
    } else {
        (0.0, 226i64)
    };

    let mut defs = String::from(
        r##"<linearGradient id="cover_sheen" x1="0" y1="0" x2="0" y2="1"><stop offset="0.55" stop-color="#0a0810" stop-opacity="0"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.6"/></linearGradient><linearGradient id="land_scrim" x1="0" y1="0" x2="1" y2="0.2"><stop offset="0" stop-color="#0a0810" stop-opacity="0.95"/><stop offset="0.6" stop-color="#0a0810" stop-opacity="0.72"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.42"/></linearGradient><linearGradient id="land_bottom" x1="0" y1="0" x2="0" y2="1"><stop offset="0.45" stop-color="#0a0810" stop-opacity="0"/><stop offset="1" stop-color="#0a0810" stop-opacity="0.92"/></linearGradient><linearGradient id="cover_glint" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#ffffff" stop-opacity="0"/><stop offset="0.5" stop-color="#ffffff" stop-opacity="0.10"/><stop offset="1" stop-color="#ffffff" stop-opacity="0"/></linearGradient>"##,
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
            squad_cy,
            &chars,
            accent,
            t::MARGIN,
            squad_label_y,
        );
        let extras_s = if has_extras {
            extras_row(m, extras_y, w as f64 - m, &g.extra, accent)
        } else {
            String::new()
        };
        format!(
            r#"
        {bg}
        <text x="{m:.0}" y="44" font-family="{mono}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{m:.0}" y="92" font-family="{mono}" font-size="46" font-weight="800" fill="{fg}" filter="url(#softglow)">{title}</text>
        <text x="{m:.0}" y="116" font-family="{mono}" font-size="13" fill="{fgd}" letter-spacing="2">{subtitle}</text>
        {chips_s}
        <text x="{m:.0}" y="184" font-family="{sans}" font-size="13" fill="{fgd}">{blurb}</text>
        {extras_s}
        {squad_s}"#,
            mono = t::MONO,
            sans = t::SANS,
            fg = t::FG,
            fgd = t::FG_DIM,
            title = esc(&fit_text(&g.title, w as f64 - 2.0 * m, 46.0, true)),
            // letter-spacing="2" widens each char by 2px → fit at an
            // equivalent font of fs + ls/0.6.
            subtitle = esc(&fit_text(&g.subtitle, w as f64 - 2.0 * m - 40.0, 13.0 + 2.0 / 0.6, true)),
            chips_s = chips(m, 150.0, w as f64 - m, g, accent),
            blurb = esc(&fit_text(&g.blurb, w as f64 - 2.0 * m, 13.0, false)),
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
                r#"<image href="{uri}" x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" preserveAspectRatio="xMidYMid slice" clip-path="url(#coverclip)"/><rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="url(#cover_sheen)"/><rect x="{cx0}" y="{cy0}" width="{cw}" height="{chh}" rx="12" fill="url(#cover_glint)"><animate attributeName="opacity" values="0;1;0" keyTimes="0;0.5;1" dur="7s" repeatCount="indefinite"/></rect>"#
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
        let squad_s = squad(&art_dir, rx, avail, squad_cy, &chars, accent, rx as u32, squad_label_y);
        let extras_s = if has_extras {
            extras_row(rx, extras_y, w as f64 - m, &g.extra, accent)
        } else {
            String::new()
        };
        format!(
            r#"
        {panel}
        <text x="{rx:.0}" y="44" font-family="{mono}" font-size="12" fill="{accent}" letter-spacing="3">★ BEST GAME · my pick</text>
        <text x="{rx:.0}" y="92" font-family="{mono}" font-size="44" font-weight="800" fill="{fg}" filter="url(#softglow)">{title}</text>
        <text x="{rx:.0}" y="116" font-family="{mono}" font-size="13" fill="{fgd}" letter-spacing="2">{subtitle}</text>
        {chips_s}
        <text x="{rx:.0}" y="184" font-family="{sans}" font-size="13" fill="{fgd}">{blurb}</text>
        {extras_s}
        {squad_s}"#,
            mono = t::MONO,
            sans = t::SANS,
            fg = t::FG,
            fgd = t::FG_DIM,
            title = esc(&fit_text(&g.title, avail, 44.0, true)),
            // letter-spacing="2" widens each char by 2px → fit at an
            // equivalent font of fs + ls/0.6.
            subtitle = esc(&fit_text(&g.subtitle, avail - 40.0, 13.0 + 2.0 / 0.6, true)),
            chips_s = chips(rx, 152.0, w as f64 - m, g, accent),
            blurb = esc(&fit_text(&g.blurb, avail, 13.0, false)),
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
