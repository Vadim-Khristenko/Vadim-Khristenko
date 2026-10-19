//! Kumir 3 as a Russian school notebook: squared paper with the red margin,
//! the title written by hand, a real Kumir program for the Robot, the Robot
//! field where it paints a corridor, and a teacher's red "5" in the corner.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 400.0;
const PAPER: &str = "#f7f5ee";
const GRIDL: &str = "#b9cde3";
const MARGIN: &str = "#e06a6a";
const INK: &str = "#1d3a8a";
const PENCIL: &str = "#55607a";
const RED: &str = "#d23b3b";
const STEPS: usize = 6;
const LOOP: f64 = 9.0;

const CODE: [&[(bool, &str)]; 7] = [
    &[(true, "использовать"), (false, " Робот")],
    &[(true, "алг"), (false, " Закрасить коридор")],
    &[(true, "нач")],
    &[
        (false, "  "),
        (true, "нц пока"),
        (false, " справа свободно"),
    ],
    &[(false, "    закрасить; вправо")],
    &[(false, "  "), (true, "кц")],
    &[(true, "кон")],
];

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "kumir", "repo", "kumir3-core");

    let _ = write!(
        b,
        r##"<rect width="{w}" height="{H}" fill="{PAPER}"/><rect width="{w}" height="{H}" fill="url(#sq)"/><line x1="80" y1="0" x2="80" y2="{H}" stroke="{MARGIN}" stroke-width="1.6"/><line x1="{m}" y1="0" x2="{m}" y2="{H}" stroke="#d9d2bf" stroke-width="6"/><line x1="{m}" y1="0" x2="{m}" y2="{H}" stroke="#b8ae95"/>"##,
        m = w / 2.0
    );
    b.push_str(&doc.text(
        100.0,
        66.0,
        "Кумир 3",
        &T::new(Font::CaveatBold, 50.0).fill(INK),
    ));
    b.push_str(&doc.text(
        282.0,
        64.0,
        "House Edition",
        &T::new(Font::Caveat, 22.0).fill(PENCIL),
    ));
    b.push_str(&doc.text(
        100.0,
        98.0,
        "школьный язык, переписанный на Rust",
        &T::new(Font::Caveat, 23.0).fill(INK),
    ));
    let kw = T::new(Font::MonoBold, 14.0).fill(INK);
    let plain = T::new(Font::Mono, 14.0).fill("#2a2f45");
    for (i, toks) in CODE.iter().enumerate() {
        let y = 140.0 + i as f64 * 22.0;
        b.push_str(&doc.text(
            86.0,
            y,
            &(i + 1).to_string(),
            &T::new(Font::Mono, 10.0).fill(MARGIN).op(0.8).end(),
        ));
        let runs: Vec<(&str, &T)> = toks
            .iter()
            .map(|(k, s)| (*s, if *k { &kw } else { &plain }))
            .collect();
        b.push_str(&doc.spans(100.0, y, &runs));
    }
    let _ = write!(
        b,
        r##"<rect class="hl" x="94" y="{}" width="300" height="22" fill="#ffe58a" opacity=".45"/>"##,
        140.0 + 4.0 * 22.0 - 16.0
    );
    b.push_str(&doc.text(
        100.0,
        330.0,
        ".kum → IR → Rust · WASM",
        &T::new(Font::Caveat, 21.0).fill(PENCIL),
    ));
    b.push_str(&doc.text(
        100.0,
        362.0,
        "interpreter-cli · interpreter-tui · kumir3-compiler",
        &T::new(Font::Mono, 11.0).fill(PENCIL),
    ));

    let (fx, fy, cols, rows, fc) = (540.0, 120.0, 9usize, 4usize, 40.0);
    b.push_str(&doc.text(fx, 66.0, "Робот", &T::new(Font::CaveatBold, 30.0).fill(INK)));
    b.push_str(&doc.text(
        fx + 86.0,
        64.0,
        "обстановка: коридор",
        &T::new(Font::Caveat, 21.0).fill(PENCIL),
    ));
    let _ = write!(
        b,
        r##"<rect x="{fx}" y="{fy}" width="{}" height="{}" fill="#3e8a63"/>"##,
        cols as f64 * fc,
        rows as f64 * fc
    );
    for c in 0..=cols {
        let x = fx + c as f64 * fc;
        let _ = write!(
            b,
            r##"<line x1="{x}" y1="{fy}" x2="{x}" y2="{}" stroke="#d8e9c8" stroke-opacity=".6"/>"##,
            fy + rows as f64 * fc
        );
    }
    for r in 0..=rows {
        let y = fy + r as f64 * fc;
        let _ = write!(
            b,
            r##"<line x1="{fx}" y1="{y}" x2="{}" y2="{y}" stroke="#d8e9c8" stroke-opacity=".6"/>"##,
            fx + cols as f64 * fc
        );
    }
    let row = 1.0;
    for i in 0..STEPS {
        let _ = write!(
            b,
            r##"<rect class="p{i}" x="{}" y="{}" width="{}" height="{}" fill="#9aa3ad"/>"##,
            fx + (i + 1) as f64 * fc + 3.0,
            fy + row * fc + 3.0,
            fc - 6.0,
            fc - 6.0
        );
    }
    let _ = write!(
        b,
        r##"<rect x="{}" y="{}" width="{}" height="{}" fill="#9aa3ad"/>"##,
        fx + 3.0,
        fy + row * fc + 3.0,
        fc - 6.0,
        fc - 6.0
    );
    let end = fx + (STEPS + 1) as f64 * fc;
    let _ = write!(
        b,
        r##"<g stroke="#f2d64b" stroke-width="5"><line x1="{fx}" y1="{a}" x2="{end}" y2="{a}"/><line x1="{fx}" y1="{c}" x2="{end}" y2="{c}"/><line x1="{end}" y1="{a}" x2="{end}" y2="{c}"/></g>"##,
        a = fy + row * fc,
        c = fy + (row + 1.0) * fc
    );
    let (rx, ry) = (fx + fc / 2.0, fy + row * fc + fc / 2.0);
    let _ = write!(
        b,
        r##"<g class="robot"><path d="M{rx} {} L{} {ry} L{rx} {} L{} {ry}z" fill="#1d1f2e" stroke="#ffffff" stroke-width="2"/></g>"##,
        ry - 14.0,
        rx + 14.0,
        ry + 14.0,
        rx - 14.0
    );
    b.push_str(&doc.text(
        fx,
        fy + rows as f64 * fc + 30.0,
        "жёлтые линии: стены · серые клетки: закрашено",
        &T::new(Font::Mono, 11.0).fill(PENCIL),
    ));
    let _ = write!(
        b,
        r#"<g transform="rotate(-10 {g} 330)"><circle cx="{g}" cy="330" r="34" fill="none" stroke="{RED}" stroke-width="2.5"/>"#,
        g = w - 80.0
    );
    b.push_str(&doc.text(
        w - 80.0,
        350.0,
        "5",
        &T::new(Font::CaveatBold, 58.0).fill(RED).mid(),
    ));
    b.push_str("</g>");
    b.push_str(&doc.text(
        fx,
        H - 46.0,
        &format!("github.com/Vadim-Khristenko/{repo}"),
        &T::new(Font::MonoBold, 11.0).fill(INK),
    ));
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        w - 140.0,
        H - 22.0,
        PENCIL,
        INK,
    ));

    let frame = format!(
        r##"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="12" fill="none" stroke="#c9c0a8"/>"##,
        w - 1.0,
        H - 1.0
    );
    let n = STEPS as f64;
    let mut keys = String::new();
    for i in 0..=STEPS {
        let _ = write!(
            keys,
            "{:.1}%{{transform:translateX({}px)}}",
            i as f64 / n * 70.0,
            i as f64 * fc
        );
    }
    let mut css = format!(
        ".robot{{animation:robot {LOOP}s steps(1) infinite}}@keyframes robot{{{keys}96%{{transform:translateX({}px)}}100%{{transform:translateX(0)}}}}.hl{{animation:hl {LOOP}s steps(1) infinite}}@keyframes hl{{0%,70%{{transform:translateY(0)}}70.1%,100%{{transform:translateY(22px)}}}}",
        n * fc
    );
    for i in 0..STEPS {
        let at = (i as f64 + 1.0) / n * 70.0;
        let _ = write!(
            css,
            ".p{i}{{animation:p{i} {LOOP}s steps(1) infinite}}@keyframes p{i}{{0%,{at:.1}%{{opacity:0}}{b:.1}%,96%{{opacity:1}}100%{{opacity:0}}}}",
            b = at + 0.1
        );
    }
    let defs = format!(
        r#"<pattern id="sq" width="20" height="20" patternUnits="userSpaceOnUse"><path d="M20 0H0V20" fill="none" stroke="{GRIDL}" stroke-width=".8"/></pattern><clipPath id="card"><rect width="{w}" height="{H}" rx="12"/></clipPath>"#
    );
    Ok(vec![(
        "repo_kumir.svg".into(),
        kit::svg(
            w,
            H,
            "Kumir 3: the classroom language rebuilt in Rust",
            &css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
