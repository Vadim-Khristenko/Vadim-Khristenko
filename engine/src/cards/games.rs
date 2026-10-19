//! Now-playing shelf: the game in rotation as a 2x2 hero tile plus the rest in
//! a grid. Cover art from assets/games is re-encoded at the size it is drawn
//! (so the card stays light), gradient tiles stand in for missing art, and
//! every tile gets a staggered light sweep.

use super::kit::{self, pal};
use crate::config::profile::Game;
use crate::img;
use crate::run::Ctx;
use crate::svg::find_media;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const COLS: usize = 4;
const GAP: f64 = 16.0;
const PAD: f64 = 24.0;
const TOP: f64 = 56.0;

fn tile(ctx: &Ctx, g: &Game, idx: usize, tw: f64, th: f64, real: &mut usize) -> String {
    let mut out = format!(
        r##"<clipPath id="clip{idx}"><rect width="{tw:.1}" height="{th}" rx="12"/></clipPath><linearGradient id="sh{idx}" x1="0" y1="0" x2="0" y2="1"><stop offset=".42" stop-color="#000" stop-opacity="0"/><stop offset="1" stop-color="#05040a" stop-opacity=".93"/></linearGradient>"##
    );
    let art = find_media(&ctx.assets.join("games"), &g.key)
        .and_then(|p| img::load(&p, (tw * 1.25) as u32, None).ok());
    match art {
        Some(e) => {
            *real += 1;
            let _ = write!(
                out,
                r#"<g clip-path="url(#clip{idx})">{}</g>"#,
                e.tag(&format!("mg{idx}"), 0.0, 0.0, tw, th, "")
            );
        }
        None => {
            let _ = write!(
                out,
                r#"<linearGradient id="gg{idx}" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="{}"/><stop offset="1" stop-color="{}"/></linearGradient><rect width="{tw:.1}" height="{th}" rx="12" fill="url(#gg{idx})"/>"#,
                g.cb,
                pal::BG_DEEP
            );
        }
    }
    let _ = write!(
        out,
        r#"<rect width="{tw:.1}" height="{th}" rx="12" fill="url(#sh{idx})"/>"#
    );
    out
}

fn sweep(idx: usize, tw: f64, th: f64, begin: f64) -> String {
    format!(
        r#"<g clip-path="url(#clip{idx})"><rect x="-90" y="-24" width="46" height="{h}" fill="url(#tile_sweep)" transform="skewX(-18)"><animate attributeName="x" values="-90;{end:.0}" dur="7s" begin="{begin:.1}s" repeatCount="indefinite" calcMode="spline" keySplines="0.4 0 0.2 1"/></rect></g>"#,
        h = th + 48.0,
        end = tw + 80.0
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let games = &ctx.cfg.profile.games;
    let w = kit::W;
    let tile_w = (w - 2.0 * PAD - (COLS as f64 - 1.0) * GAP) / COLS as f64;
    let tile_h = (tile_w / 2.14).round();
    let featured = ctx.featured_game();
    let rest: Vec<&Game> = games.iter().filter(|g| g.key != featured.key).collect();

    let mut cells: Vec<(usize, usize)> = Vec::new();
    let mut row = 0;
    while cells.len() < rest.len() {
        for col in 0..COLS {
            if !(row < 2 && col < 2) {
                cells.push((col, row));
            }
        }
        row += 1;
    }
    let rows = row.max(2);
    let h = TOP + rows as f64 * tile_h + (rows as f64 - 1.0) * GAP + 24.0;

    let mut doc = Doc::new();
    let mut b = String::new();
    let mut real = 0;
    let _ = write!(b, r#"<rect width="{w}" height="{h}" fill="{}"/>"#, pal::BG);

    // hero tile
    let (fw, fh) = (2.0 * tile_w + GAP, 2.0 * tile_h + GAP);
    let g = featured;
    let _ = write!(
        b,
        r#"<g transform="translate({PAD} {TOP})">{}"#,
        tile(ctx, g, 0, fw, fh, &mut real)
    );
    let _ = write!(
        b,
        r##"<rect width="{fw:.1}" height="{fh}" rx="12" fill="none" stroke="{ca}" stroke-width="1.8" opacity=".95"/><rect x="14" y="14" width="124" height="24" rx="12" fill="#05040a" fill-opacity=".72" stroke="{ca}"/><circle class="live" cx="28" cy="26" r="3.4" fill="{ca}"/>"##,
        ca = g.ca
    );
    b.push_str(&doc.text(
        38.0,
        30.5,
        "IN ROTATION",
        &T::new(Font::MonoBold, 11.0).fill(pal::FG_HI).ls(1.0),
    ));
    let note_t = T::new(Font::Zen, 12.0).fill(pal::FG);
    b.push_str(&doc.text(
        16.0,
        fh - 56.0,
        &typo::fit(&g.note, &note_t, fw - 32.0),
        &note_t,
    ));
    let short_t = T::new(Font::ZenBlack, 26.0).fill("#ffffff");
    b.push_str(&doc.text(
        16.0,
        fh - 28.0,
        &typo::fit(&g.short, &short_t, fw - 32.0),
        &short_t,
    ));
    let title_t = T::new(Font::Zen, 12.0).fill(&g.ca);
    b.push_str(&doc.text(
        16.0,
        fh - 11.0,
        &typo::fit(&g.title, &title_t, fw - 32.0),
        &title_t,
    ));
    b.push_str(&sweep(0, fw, fh, 0.0));
    b.push_str("</g>");

    for (i, g) in rest.iter().enumerate() {
        let (col, r) = cells[i];
        let x = PAD + col as f64 * (tile_w + GAP);
        let y = TOP + r as f64 * (tile_h + GAP);
        let idx = i + 1;
        let _ = write!(
            b,
            r#"<g transform="translate({x:.1} {y})">{}"#,
            tile(ctx, g, idx, tile_w, tile_h, &mut real)
        );
        let _ = write!(
            b,
            r#"<rect width="{tile_w:.1}" height="{tile_h}" rx="12" fill="none" stroke="{}" stroke-width="1.3" opacity=".8"/>"#,
            g.ca
        );
        let st = T::new(Font::ZenBlack, 14.0).fill("#ffffff");
        b.push_str(&doc.text(
            14.0,
            tile_h - 22.0,
            &typo::fit(&g.short, &st, tile_w - 26.0),
            &st,
        ));
        let tt = T::new(Font::Zen, 9.5).fill(&g.ca);
        b.push_str(&doc.text(
            14.0,
            tile_h - 8.0,
            &typo::fit(&g.title, &tt, tile_w - 26.0),
            &tt,
        ));
        b.push_str(&sweep(idx, tile_w, tile_h, 0.8 + i as f64 * 0.55));
        b.push_str("</g>");
    }

    b.push_str(&doc.text(
        PAD,
        36.0,
        "NOW PLAYING",
        &T::new(Font::Mono, 11.0).fill(pal::TEAL).ls(2.4),
    ));
    let badge = if real > 0 {
        format!("{real} / {} with cover art", games.len())
    } else {
        format!("{} titles", games.len())
    };
    b.push_str(&doc.text(
        w - PAD,
        36.0,
        &badge,
        &T::new(Font::Mono, 11.0).fill(pal::DIM).end(),
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        h - 1.0,
        pal::LINE
    );
    let css = ".live{animation:live 1.8s ease-in-out infinite}@keyframes live{50%{opacity:.3}}";
    let defs = format!(
        r##"<clipPath id="card"><rect width="{w}" height="{h}" rx="14"/></clipPath><linearGradient id="tile_sweep" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#fff" stop-opacity="0"/><stop offset=".5" stop-color="#fff" stop-opacity=".14"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></linearGradient>"##
    );
    let alt = format!("Now playing shelf, featuring {}", featured.title);
    Ok(vec![(
        "games.svg".into(),
        kit::svg(w, h, &alt, css, &defs, &doc, &frame),
    )])
}
