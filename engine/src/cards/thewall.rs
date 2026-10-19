//! The Wall Dev in the Win95 skin the extension itself wears: a zoomed patch
//! of the pixel canvas with a template overlay, "dots" marking pixels that are
//! still wrong, and those pixels filling in one by one.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 380.0;
const DESK: &str = "#1d2233";
const GREY: &str = "#c0c0c0";
const GREY_D: &str = "#808080";
const WHITE: &str = "#ffffff";
const BLACK: &str = "#0a0a0a";
const NAVY: &str = "#000080";
const PX: f64 = 13.0;
const LOOP: f64 = 8.0;

const TEMPLATE: [&str; 12] = [
    "..........................",
    "...TTT......PPP...........",
    "..TTTTT....PPPPP..........",
    "..TTTTTT..PPPPPP..........",
    "..TTTTTTTTPPPPPP.....KKK..",
    "...TTTTTTTPPPPP......K.KK.",
    "....TTTTTTTPPP.......K..K.",
    ".....TTTTTTPP........K....",
    "......TTTTPP.......KKK....",
    ".......TTTP.......KKKK....",
    "........TP........KKKK....",
    "..........................",
];
const WRONG: [(usize, usize); 12] = [
    (4, 3),
    (7, 4),
    (12, 2),
    (14, 3),
    (6, 7),
    (9, 8),
    (11, 5),
    (8, 9),
    (21, 6),
    (19, 9),
    (13, 4),
    (5, 5),
];

fn col(c: char) -> Option<&'static str> {
    match c {
        'T' => Some("#39c5bb"),
        'P' => Some("#ff3d8b"),
        'K' => Some("#f4f4f4"),
        _ => None,
    }
}

/// Pad by one cell and shift the art so its bounding box is centred.
fn centred() -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let rows: Vec<Vec<char>> = std::iter::once(vec!['.'; 28])
        .chain(TEMPLATE.iter().map(|r| {
            std::iter::once('.')
                .chain(r.chars())
                .chain(std::iter::once('.'))
                .collect()
        }))
        .chain(std::iter::once(vec!['.'; 28]))
        .collect();
    let wrong: Vec<(usize, usize)> = WRONG.iter().map(|(c, r)| (c + 1, r + 1)).collect();
    let cols: Vec<usize> = rows
        .iter()
        .flat_map(|r| {
            r.iter()
                .enumerate()
                .filter(|(_, c)| **c != '.')
                .map(|(i, _)| i)
        })
        .collect();
    let lines: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| r.iter().any(|c| *c != '.'))
        .map(|(i, _)| i)
        .collect();
    let (w, h) = (rows[0].len() as i64, rows.len() as i64);
    let dx =
        (w - (*cols.iter().max().unwrap() as i64 + 1 + *cols.iter().min().unwrap() as i64)) / 2;
    let dy =
        (h - (*lines.iter().max().unwrap() as i64 + 1 + *lines.iter().min().unwrap() as i64)) / 2;
    let mut grid = vec![vec!['.'; w as usize]; h as usize];
    for (r, row) in rows.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch != '.' {
                grid[(r as i64 + dy) as usize][(c as i64 + dx) as usize] = *ch;
            }
        }
    }
    let wrong = wrong
        .into_iter()
        .map(|(c, r)| ((c as i64 + dx) as usize, (r as i64 + dy) as usize))
        .collect();
    (grid, wrong)
}

fn bevel(x: f64, y: f64, w: f64, h: f64, inset: bool) -> String {
    let (hi, lo) = if inset {
        (GREY_D, WHITE)
    } else {
        (WHITE, GREY_D)
    };
    format!(
        r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{GREY}"/><path d="M{x} {b} V{y} H{r}" fill="none" stroke="{hi}" stroke-width="2"/><path d="M{x} {b} H{r} V{y}" fill="none" stroke="{lo}" stroke-width="2"/>"#,
        b = y + h,
        r = x + w
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "thewall", "repo", "TheWall");
    let site = kit::show(ctx, "thewall", "site", "the-wall.vai-rice.space");
    let (grid, wrong) = centred();
    let (x, y) = (28.0, 26.0);
    let (cx, cy) = (x + 12.0, y + 36.0);
    let (cw, chh) = (grid[0].len() as f64 * PX, grid.len() as f64 * PX);
    let (ww, wh) = (cw + 24.0, chh + 74.0);

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{DESK}"/>"#);
    b.push_str(&bevel(x, y, ww, wh, false));
    let _ = write!(
        b,
        r#"<rect x="{}" y="{}" width="{}" height="24" fill="url(#title)"/>"#,
        x + 4.0,
        y + 4.0,
        ww - 8.0
    );
    b.push_str(&doc.text(
        x + 12.0,
        y + 21.0,
        "THE WALL DEV · OVERLAY",
        &T::new(Font::Silk, 12.0).fill(WHITE),
    ));
    for (k, g) in ["_", "×"].iter().enumerate() {
        let bx = x + ww - 44.0 + k as f64 * 20.0;
        b.push_str(&bevel(bx, y + 8.0, 17.0, 16.0, false));
        b.push_str(&doc.text(
            bx + 8.5,
            y + 20.0,
            g,
            &T::new(Font::PtBold, 11.0).fill(BLACK).mid(),
        ));
    }
    b.push_str(&bevel(cx - 2.0, cy - 2.0, cw + 4.0, chh + 4.0, true));
    let _ = write!(
        b,
        r##"<rect x="{cx}" y="{cy}" width="{cw}" height="{chh}" fill="#e9e4d8"/>"##
    );
    let mut paths: std::collections::BTreeMap<&str, String> = Default::default();
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if let Some(color) = col(*ch) {
                if !wrong.contains(&(c, r)) {
                    let _ = write!(
                        paths.entry(color).or_default(),
                        "M{} {}h{PX}v{PX}h-{PX}z",
                        cx + c as f64 * PX,
                        cy + r as f64 * PX
                    );
                }
            }
        }
    }
    for (color, d) in paths {
        let _ = write!(b, r#"<path d="{d}" fill="{color}"/>"#);
    }
    for (i, (c, r)) in wrong.iter().enumerate() {
        let color = col(grid[*r][*c]).unwrap_or("#e9e4d8");
        let (px, py) = (cx + *c as f64 * PX, cy + *r as f64 * PX);
        let _ = write!(
            b,
            r#"<rect class="w{i}" x="{px}" y="{py}" width="{PX}" height="{PX}" fill="{color}"/><rect class="d{i}" x="{}" y="{}" width="5" height="5" fill="{color}" stroke="{BLACK}" stroke-width=".8"/>"#,
            px + PX / 2.0 - 2.5,
            py + PX / 2.0 - 2.5
        );
    }
    let _ = write!(
        b,
        r#"<rect x="{cx}" y="{cy}" width="{cw}" height="{chh}" fill="url(#pg)"/><g class="cross"><rect x="{}" y="{}" width="{PX}" height="{PX}" fill="none" stroke="{BLACK}" stroke-width="2"/></g>"#,
        cx + 2.0 * PX,
        cy + 11.0 * PX
    );
    b.push_str(&doc.text(
        cx,
        y + wh - 12.0,
        "X 3120  Y 2077  ·  zoom 1000%  ·  dots mode",
        &T::new(Font::Mono, 11.0).fill(BLACK),
    ));

    let rx = x + ww + 40.0;
    b.push_str(&doc.text(
        rx,
        70.0,
        "THE WALL DEV",
        &T::new(Font::Silk, 30.0).fill(WHITE),
    ));
    let sub = T::new(Font::Pt, 14.0).fill("#aab3cf");
    b.push_str(&doc.text(
        rx,
        98.0,
        "Template overlay and tools for The Wall, the 6000 × 4200",
        &sub,
    ));
    b.push_str(&doc.text(
        rx,
        117.0,
        "collaborative pixel canvas of the Snackers at filianislost.com",
        &sub,
    ));
    let feats = [
        (
            "OVERLAY",
            "quantized to the Wall palette, dots for wrong pixels",
            "#39c5bb",
        ),
        (
            "TEMPLATES",
            "public, clan and personal, animated sprite sheets",
            "#ff3d8b",
        ),
        (
            "CLANS",
            "live chat over SSE, shared library, roster",
            "#f2d64b",
        ),
        (
            "ACCOUNTS",
            "Telegram and Discord linking, bot DMs",
            "#86a8ff",
        ),
    ];
    for (i, (k, v, c)) in feats.iter().enumerate() {
        let fy = 156.0 + i as f64 * 30.0;
        let _ = write!(
            b,
            r#"<rect x="{rx}" y="{}" width="8" height="8" fill="{c}"/>"#,
            fy - 10.0
        );
        b.push_str(&doc.text(
            rx + 18.0,
            fy - 1.0,
            k,
            &T::new(Font::Silk, 10.0).fill(WHITE),
        ));
        b.push_str(&doc.text(
            rx + 108.0,
            fy - 1.0,
            v,
            &T::new(Font::Pt, 13.0).fill("#aab3cf"),
        ));
    }
    b.push_str(&doc.text(
        rx,
        296.0,
        "TypeScript extension · Rust backend · Bun · PostgreSQL",
        &T::new(Font::Mono, 11.0).fill("#8f98b8"),
    ));
    b.push_str(&doc.text(
        rx,
        318.0,
        &site,
        &T::new(Font::MonoBold, 11.0).fill("#39c5bb"),
    ));
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        w - 24.0,
        318.0,
        "#8f98b8",
        "#39c5bb",
    ));

    let (tx, ty) = (w - 270.0, H - 50.0);
    b.push_str("<g>");
    b.push_str(&bevel(tx, ty, 246.0, 40.0, false));
    let _ = write!(
        b,
        r#"<rect x="{}" y="{}" width="238" height="14" fill="url(#title)"/>"#,
        tx + 4.0,
        ty + 4.0
    );
    b.push_str(&doc.text(
        tx + 10.0,
        ty + 15.0,
        "THE WALL DEV BOT",
        &T::new(Font::Silk, 8.0).fill(WHITE),
    ));
    b.push_str(&doc.text(
        tx + 10.0,
        ty + 33.0,
        "Template done. Every pixel is right.",
        &T::new(Font::Pt, 12.0).fill(BLACK),
    ));
    b.push_str("</g>");

    let frame = format!(
        r##"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="12" fill="none" stroke="#3a4160"/>"##,
        w - 1.0,
        H - 1.0
    );
    let n = wrong.len() as f64;
    let mut css = format!(".cross{{animation:cross 6s steps(6) infinite}}@keyframes cross{{to{{transform:translateX({}px)}}}}", 6.0 * PX);
    for i in 0..wrong.len() {
        let at = (i as f64 + 1.0) / n * 75.0;
        let _ = write!(
            css,
            ".w{i}{{opacity:0;animation:w{i} {LOOP}s steps(1) infinite}}@keyframes w{i}{{0%,{at:.1}%{{opacity:0}}{b:.1}%,96%{{opacity:1}}100%{{opacity:0}}}}.d{i}{{animation:d{i} {LOOP}s steps(1) infinite}}@keyframes d{i}{{0%,{at:.1}%{{opacity:1}}{b:.1}%,96%{{opacity:0}}100%{{opacity:1}}}}",
            b = at + 0.1
        );
    }
    let defs = format!(
        r##"<linearGradient id="title" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="{NAVY}"/><stop offset="1" stop-color="{}"/></linearGradient><pattern id="pg" x="{cx}" y="{cy}" width="{PX}" height="{PX}" patternUnits="userSpaceOnUse"><path d="M{PX} 0H0V{PX}" fill="none" stroke="#000" stroke-opacity=".12"/></pattern><clipPath id="card"><rect width="{w}" height="{H}" rx="12"/></clipPath>""##,
        "#1084d0"
    );
    Ok(vec![(
        "repo_thewall.svg".into(),
        kit::svg(
            w,
            H,
            "The Wall Dev: template overlay and tools for the Snackers pixel canvas",
            &css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
