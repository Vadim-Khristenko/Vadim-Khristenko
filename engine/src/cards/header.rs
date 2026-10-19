//! Header: "Vocal Track 01". A vocal-synth piano roll whose notes sing the
//! lyric from `[header]`, a playhead that lights each note as it passes, the
//! twin-tail ribbons, the name, and the projects as tracks in a mixer list.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 340.0;
const LOOP: f64 = 12.0;
const X0: f64 = 48.0;
const LANE_Y0: f64 = 48.0;
const LANE_H: f64 = 12.0;
const LANES: usize = 21;

fn hit_delay(x: f64) -> f64 {
    (x - X0) / (kit::W - X0) * LOOP
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let p = &ctx.cfg.profile;
    let hdr = &p.header;
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let lanes_bottom = LANE_Y0 + LANES as f64 * LANE_H;

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="url(#bg)"/>"#);
    // piano roll lanes and the key strip
    for i in 0..LANES {
        let y = LANE_Y0 + i as f64 * LANE_H;
        let black = [1, 3, 6, 8, 10].contains(&((LANES - i) % 12));
        if black {
            let _ = write!(
                b,
                r#"<rect x="40" y="{y}" width="{}" height="{LANE_H}" fill="{}" opacity=".55"/>"#,
                w - 40.0,
                pal::BG_DEEP
            );
        }
        let _ = write!(
            b,
            r#"<rect x="16" y="{}" width="22" height="{}" rx="1.5" fill="{}"/>"#,
            y + 0.5,
            LANE_H - 1.0,
            if black { "#1a1c33" } else { "#2a2e52" }
        );
    }
    let mut x = X0;
    while x < w {
        let bar = ((x - X0) as i64) % 160 == 0;
        let _ = write!(
            b,
            r#"<line x1="{x}" y1="{LANE_Y0}" x2="{x}" y2="{lanes_bottom}" stroke="{}" stroke-opacity="{}"/>"#,
            pal::LINE,
            if bar { ".55" } else { ".18" }
        );
        x += 40.0;
    }
    b.push_str(&doc.text(
        18.0,
        lanes_bottom - 2.0,
        "C3",
        &T::new(Font::Mono, 7.0).fill(pal::MUTED),
    ));

    // ruler
    let _ = write!(
        b,
        r#"<rect width="{w}" height="36" fill="{}"/><line x1="0" y1="36" x2="{w}" y2="36" stroke="{}" stroke-opacity=".6"/>"#,
        pal::BG_DEEP,
        pal::LINE
    );
    let mut bar_x = X0;
    let mut n = 1;
    while bar_x < w - 160.0 {
        b.push_str(&doc.text(
            bar_x + 4.0,
            24.0,
            &n.to_string(),
            &T::new(Font::Mono, 9.0).fill(pal::DIM),
        ));
        for k in 0..4 {
            let lx = bar_x + k as f64 * 40.0;
            let _ = write!(
                b,
                r#"<line x1="{lx}" y1="{}" x2="{lx}" y2="36" stroke="{}" stroke-opacity=".7"/>"#,
                if k == 0 { 18 } else { 30 },
                pal::MUTED
            );
        }
        bar_x += 160.0;
        n += 1;
    }
    b.push_str(&doc.text(
        w - 20.0,
        23.0,
        "BPM 139 · 4/4 · VOCAL TRACK 01",
        &T::new(Font::Mono, 9.0).fill(pal::DIM).ls(1.0).end(),
    ));

    // twin-tail ribbons
    b.push_str(r#"<g class="rib"><path d="M-20 90 C 220 40, 420 210, 640 150 S 900 40, 1040 120" fill="none" stroke="url(#rib)" stroke-width="1.6"/></g><g class="rib2"><path d="M-20 110 C 260 70, 400 260, 660 190 S 920 90, 1040 170" fill="none" stroke="url(#rib)" stroke-width=".8"/></g>"#);

    // lyric notes, spread over the left half below the code line
    let lanes = [19, 18, 16, 17, 15, 16, 19, 17, 15];
    let note_t = T::new(Font::MonoBold, 8.5).fill(pal::BG);
    let words: Vec<&String> = hdr.lyric.iter().take(9).collect();
    let natural: Vec<f64> = words
        .iter()
        .map(|w| (crate::typo::width(w, &note_t) + 14.0).max(36.0))
        .collect();
    let span = 590.0 - 64.0 - 4.0 * words.len() as f64;
    let stretch = (span / natural.iter().sum::<f64>().max(1.0)).clamp(1.0, 1.9);
    let mut nx = 64.0;
    for (i, (word, nat)) in words.iter().zip(&natural).enumerate() {
        let nw = nat * stretch;
        let y = LANE_Y0 + lanes[i % lanes.len()] as f64 * LANE_H;
        let _ = write!(
            b,
            r#"<rect class="note" style="animation-delay:{:.2}s" x="{nx}" y="{}" width="{}" height="{}" rx="2"/>"#,
            hit_delay(nx),
            y + 1.0,
            nw - 4.0,
            LANE_H - 2.0
        );
        b.push_str(&doc.text(nx + 5.0, y + 9.0, word, &note_t));
        nx += nw + 4.0;
    }
    for (gx, lane, gw) in [
        (680.0, 3, 36.0),
        (720.0, 5, 36.0),
        (760.0, 2, 76.0),
        (848.0, 6, 36.0),
        (888.0, 4, 72.0),
    ] {
        let y = LANE_Y0 + lane as f64 * LANE_H;
        let _ = write!(
            b,
            r#"<rect class="note" style="animation-delay:{:.2}s" x="{gx}" y="{}" width="{}" height="{}" rx="2"/>"#,
            hit_delay(gx),
            y + 1.0,
            gw - 4.0,
            LANE_H - 2.0
        );
    }

    // name block
    let aliases: Vec<&str> = p
        .aliases
        .iter()
        .map(String::as_str)
        .filter(|a| !a.contains("Programmer"))
        .collect();
    b.push_str(&doc.text(
        64.0,
        86.0,
        &aliases.join(" · "),
        &T::new(Font::Mono, 11.0).fill(pal::TEAL).ls(2.4),
    ));
    b.push_str(&doc.text(
        62.0,
        146.0,
        &p.name,
        &T::new(Font::ZenBlack, 58.0).fill(pal::FG_HI).ls(-1.0),
    ));
    b.push_str(&doc.text(
        64.0,
        180.0,
        &crate::typo::fit(&hdr.tagline, &T::new(Font::ZenLight, 17.0), 520.0),
        &T::new(Font::ZenLight, 17.0).fill(pal::FG),
    ));
    let code = [
        ("let", pal::KW),
        (" home ", pal::FG),
        ("=", pal::DIM),
        (" Language", pal::TEAL),
        ("::", pal::DIM),
        ("Rust", pal::VIOLET),
        (";", pal::DIM),
        ("  // vai-rice.space", pal::DIM),
    ];
    let styles: Vec<T> = code
        .iter()
        .map(|(_, c)| T::new(Font::Mono, 12.0).fill(c))
        .collect();
    let runs: Vec<(&str, &T)> = code
        .iter()
        .zip(&styles)
        .map(|((s, _), t)| (*s, t))
        .collect();
    let code_w: f64 = runs.iter().map(|(s, t)| crate::typo::width(s, t)).sum();
    b.push_str(&doc.spans(64.0, 208.0, &runs));
    let _ = write!(
        b,
        r#"<rect class="blink" x="{}" y="198" width="7" height="13" fill="{}"/>"#,
        64.0 + code_w + 4.0,
        pal::TEAL
    );
    if !hdr.katakana.is_empty() {
        b.push_str(&doc.vertical(
            604.0,
            74.0,
            &hdr.katakana,
            &T::new(Font::ZenLight, 11.0).fill(pal::DIM),
            14.0,
        ));
    }

    // track list
    let tx = 636.0;
    let _ = write!(
        b,
        r#"<rect x="{}" y="52" width="{}" height="228" rx="8" fill="{}" fill-opacity=".82" stroke="{}" stroke-opacity=".7"/>"#,
        tx - 12.0,
        w - tx - 12.0,
        pal::BG_DEEP,
        pal::LINE
    );
    b.push_str(&doc.text(
        tx + 4.0,
        72.0,
        "TRACKS",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(2.0),
    ));
    b.push_str(&doc.text(
        w - 36.0,
        72.0,
        "M  S",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.0).end(),
    ));
    let tracks: Vec<_> = hdr.tracks.iter().take(5).collect();
    for (i, t) in tracks.iter().enumerate() {
        let y = 84.0 + i as f64 * 38.0;
        let _ = write!(
            b,
            r#"<rect x="{tx}" y="{y}" width="3" height="30" rx="1.5" fill="{}"/>"#,
            t.accent
        );
        let name_t = T::new(Font::ZenBlack, 13.5).fill(pal::FG_HI);
        b.push_str(&doc.text(
            tx + 14.0,
            y + 13.0,
            &crate::typo::fit(&t.name, &name_t, 250.0),
            &name_t,
        ));
        let note_t = T::new(Font::Mono, 9.5).fill(pal::DIM);
        b.push_str(&doc.text(
            tx + 14.0,
            y + 27.0,
            &crate::typo::fit(&t.note, &note_t, 260.0),
            &note_t,
        ));
        let _ = write!(
            b,
            r#"<rect x="{}" y="{}" width="13" height="13" rx="2" fill="none" stroke="{}"/><rect x="{}" y="{}" width="13" height="13" rx="2" fill="{}" stroke="{}"/>"#,
            w - 62.0,
            y + 6.0,
            pal::LINE,
            w - 44.0,
            y + 6.0,
            if t.solo { t.accent.as_str() } else { "none" },
            if t.solo { t.accent.as_str() } else { pal::LINE }
        );
        if i + 1 < tracks.len() {
            let _ = write!(
                b,
                r#"<line x1="{tx}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-opacity=".45"/>"#,
                y + 35.0,
                w - 28.0,
                y + 35.0,
                pal::LINE
            );
        }
    }

    // playhead
    let _ = write!(
        b,
        r#"<g class="head"><line x1="{X0}" y1="18" x2="{X0}" y2="{lanes_bottom}" stroke="{}" stroke-width="1.2"/><path d="M{} 28 h10 l-5 8z" fill="{}"/></g>"#,
        pal::TEAL,
        X0 - 5.0,
        pal::TEAL
    );

    // status bar
    let sb = H - 28.0;
    let _ = write!(
        b,
        r#"<rect x="0" y="{sb}" width="{w}" height="28" fill="{}"/><rect x="0" y="{sb}" width="74" height="28" fill="{}"/>"#,
        pal::STATUS,
        pal::TEAL
    );
    b.push_str(&doc.text(
        37.0,
        sb + 18.0,
        "VAI",
        &T::new(Font::MonoBold, 11.0).fill(pal::BG).mid(),
    ));
    let st = T::new(Font::Mono, 10.5).fill(pal::FG);
    b.push_str(&doc.text(90.0, sb + 18.0, "main    Rust    UTF-8    LF", &st));
    b.push_str(&doc.text(
        w - 18.0,
        sb + 18.0,
        "vai-rice.space    Ln 39, Col 1    Hatsune Miku Dark",
        &st.clone().end(),
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = format!(
        ".note{{fill:{t};opacity:.62;animation:hit {LOOP}s linear infinite}}@keyframes hit{{0%{{opacity:1;fill:#7ff0e6}}8%{{opacity:.62;fill:{t}}}100%{{opacity:.62}}}}.head{{animation:sweep {LOOP}s linear infinite}}@keyframes sweep{{from{{transform:translateX(0)}}to{{transform:translateX({d}px)}}}}.rib{{transform-origin:0 90px;animation:sway 9s ease-in-out infinite alternate}}.rib2{{transform-origin:0 110px;animation:sway 11s ease-in-out infinite alternate-reverse}}@keyframes sway{{from{{transform:rotate(-.8deg)}}to{{transform:rotate(.9deg)}}}}.blink{{animation:blink 1.1s steps(1) infinite}}@keyframes blink{{50%{{opacity:0}}}}",
        t = pal::TEAL,
        d = w - X0
    );
    let defs = format!(
        r##"<linearGradient id="bg" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="{bg}"/><stop offset="1" stop-color="{deep}"/></linearGradient><linearGradient id="rib" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="{t}" stop-opacity="0"/><stop offset=".25" stop-color="{t}" stop-opacity=".55"/><stop offset=".8" stop-color="{s}" stop-opacity=".25"/><stop offset="1" stop-color="{s}" stop-opacity="0"/></linearGradient><clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"##,
        bg = pal::BG,
        deep = pal::BG_DEEP,
        t = pal::TEAL,
        s = pal::SAKURA
    );
    let label = format!("{}, {}", p.name, hdr.tagline);
    Ok(vec![(
        "header.svg".into(),
        kit::svg(w, H, &label, &css, &defs, &doc, &frame),
    )])
}
