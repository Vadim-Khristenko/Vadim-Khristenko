//! Active research: Aethelgard TQ-1.58 HVRL. A ternary weight matrix that
//! slowly re-quantizes, the log2(3) arithmetic behind the name, a manager and
//! worker tree for the hierarchical RL, and the real phase and progress from
//! `[research]`.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 400.0;

/// Small deterministic generator so the matrix is stable between runs.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.0 >> 33) as f64 / (1u64 << 31) as f64
    }
}

fn val_color(v: i32) -> Option<&'static str> {
    match v {
        1 => Some(pal::TEAL),
        -1 => Some(pal::SAKURA),
        _ => None,
    }
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let r = &ctx.cfg.profile.research;
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let mut rng = Lcg(158);

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{}"/>"#, pal::BG);
    let (mx, my, c, cols, rows) = (40.0, 120.0, 15.0, 24, 12);
    b.push_str(&doc.text(
        mx,
        my - 14.0,
        "W ∈ {-1, 0, +1}  ·  a sample block",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.6),
    ));
    let mut cells = String::new();
    let mut solid: std::collections::BTreeMap<&str, String> = Default::default();
    for row in 0..rows {
        for col in 0..cols {
            let pick = |x: f64| [-1, 0, 0, 1][((x * 4.0) as usize).min(3)];
            let v = pick(rng.next());
            let (x, y) = (mx + col as f64 * c, my + row as f64 * c);
            let _ = write!(
                solid.entry("frame").or_default(),
                "M{} {}h{}v{}h-{}z",
                x + 1.0,
                y + 1.0,
                c - 2.0,
                c - 2.0,
                c - 2.0
            );
            if rng.next() < 0.18 {
                let v2 = [-1, 0, 1]
                    .into_iter()
                    .filter(|u| *u != v)
                    .nth((rng.next() * 2.0) as usize % 2)
                    .unwrap_or(0);
                let d = format!("animation-delay:{:.2}s", rng.next() * 7.0);
                if let Some(col1) = val_color(v) {
                    let _ = write!(
                        cells,
                        r#"<rect class="flip" style="{d}" x="{}" y="{}" width="{}" height="{}" rx="2" fill="{col1}"/>"#,
                        x + 2.0,
                        y + 2.0,
                        c - 4.0,
                        c - 4.0
                    );
                }
                if let Some(col2) = val_color(v2) {
                    let _ = write!(
                        cells,
                        r#"<rect class="flop" style="{d}" x="{}" y="{}" width="{}" height="{}" rx="2" fill="{col2}"/>"#,
                        x + 2.0,
                        y + 2.0,
                        c - 4.0,
                        c - 4.0
                    );
                }
            } else if let Some(col1) = val_color(v) {
                let _ = write!(
                    solid.entry(col1).or_default(),
                    "M{} {}h{}v{}h-{}z",
                    x + 2.0,
                    y + 2.0,
                    c - 4.0,
                    c - 4.0,
                    c - 4.0
                );
            }
        }
    }
    for (k, d) in &solid {
        if *k == "frame" {
            let _ = write!(
                b,
                r#"<path d="{d}" fill="none" stroke="{}" stroke-opacity=".5"/>"#,
                pal::LINE
            );
        } else {
            let _ = write!(b, r#"<path d="{d}" fill="{k}" fill-opacity=".85"/>"#);
        }
    }
    b.push_str(&cells);
    let ly = my + rows as f64 * c + 22.0;
    for (i, (v, lab)) in [(1, "+1  add"), (0, "0  skip"), (-1, "-1  subtract")]
        .iter()
        .enumerate()
    {
        let x = mx + i as f64 * 120.0;
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{}" width="11" height="11" rx="2" fill="{}" stroke="{}"/>"#,
            ly - 10.0,
            val_color(*v).unwrap_or("none"),
            pal::LINE
        );
        b.push_str(&doc.text(x + 18.0, ly, lab, &T::new(Font::Mono, 11.0).fill(pal::FG)));
    }

    let hx = 450.0;
    b.push_str(&doc.text(
        hx,
        58.0,
        "ACTIVE RESEARCH",
        &T::new(Font::Mono, 11.0).fill(pal::VIOLET).ls(2.4),
    ));
    let mut name = r.name.split_whitespace();
    let first = name.next().unwrap_or("Aethelgard").to_string();
    let second: String = name.next().unwrap_or("").to_string();
    let t1 = T::new(Font::ZenBlack, 40.0).fill(pal::FG_HI);
    b.push_str(&doc.text(hx - 2.0, 104.0, &first, &t1));
    b.push_str(&doc.text(
        hx + typo::width(&first, &t1) + 24.0,
        104.0,
        &second,
        &T::new(Font::ZenLight, 40.0).fill(pal::TEAL),
    ));
    for (i, line) in [
        "a low-bit agentic reasoning model",
        "with verifiable training, hierarchical RL",
        "and a software engineering specialization",
    ]
    .iter()
    .enumerate()
    {
        b.push_str(&doc.text(
            hx,
            132.0 + i as f64 * 20.0,
            line,
            &T::new(Font::ZenLight, 15.0).fill(pal::FG),
        ));
    }
    let m = T::new(Font::Mono, 13.0).fill(pal::DIM);
    let tl = T::new(Font::MonoBold, 13.0).fill(pal::TEAL);
    b.push_str(&doc.spans(
        hx,
        210.0,
        &[("log₂ 3 = ", &m), ("1.585", &tl), (" bits per weight", &m)],
    ));
    let plus = T::new(Font::Mono, 13.0).fill(pal::TEAL);
    let zero = T::new(Font::Mono, 13.0).fill(pal::FG);
    let minus = T::new(Font::Mono, 13.0).fill(pal::SAKURA);
    b.push_str(&doc.spans(
        hx,
        232.0,
        &[
            ("w · x  →  ", &m),
            ("+x", &plus),
            (",  ", &m),
            ("0", &zero),
            ("  or  ", &m),
            ("−x", &minus),
            (": no multiply", &m),
        ],
    ));

    let (tx, ty) = (870.0, 200.0);
    let nodes = [
        (tx, ty - 28.0, "manager"),
        (tx - 62.0, ty + 30.0, "plan"),
        (tx, ty + 30.0, "code"),
        (tx + 62.0, ty + 30.0, "verify"),
    ];
    for (x, y, _) in &nodes[1..] {
        let _ = write!(
            b,
            r#"<line x1="{tx}" y1="{}" x2="{x}" y2="{}" stroke="{}"/>"#,
            ty - 16.0,
            y - 12.0,
            pal::LINE
        );
    }
    for (i, (x, y, lab)) in nodes.iter().enumerate() {
        let col = if i == 0 {
            pal::VIOLET
        } else if *lab == "verify" {
            pal::TEAL
        } else {
            pal::FG
        };
        let _ = write!(
            b,
            r#"<rect x="{}" y="{}" width="60" height="22" rx="11" fill="{}" stroke="{col}" stroke-opacity=".8"/>"#,
            x - 30.0,
            y - 12.0,
            pal::BG_DEEP
        );
        b.push_str(&doc.text(*x, y + 3.0, lab, &T::new(Font::Mono, 10.0).fill(col).mid()));
    }
    let _ = write!(
        b,
        r#"<circle class="pulse" cx="{}" cy="{}" r="4" fill="{}"/>"#,
        tx + 92.0,
        ty + 30.0,
        pal::TEAL
    );
    b.push_str(&doc.text(
        tx,
        ty + 64.0,
        "rewards you can check",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).mid(),
    ));

    let py = 300.0;
    let progress = r.progress.clamp(0.0, 1.0);
    b.push_str(&doc.text(
        hx,
        py,
        "PHASE",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.6),
    ));
    let phase_t = T::new(Font::ZenBlack, 16.0).fill(pal::FG_HI);
    b.push_str(&doc.text(
        hx,
        py + 22.0,
        &typo::fit(&r.phase, &phase_t, 420.0),
        &phase_t,
    ));
    let _ = write!(
        b,
        r#"<rect x="{hx}" y="{a}" width="500" height="6" rx="3" fill="{}" fill-opacity=".6"/><rect class="bar" x="{hx}" y="{a}" width="{pw:.1}" height="6" rx="3" fill="{}"/>"#,
        pal::LINE,
        pal::TEAL,
        a = py + 38.0,
        pw = 500.0 * progress
    );
    b.push_str(&doc.text(
        hx + 500.0,
        py + 22.0,
        &format!("{}%", (progress * 100.0).round()),
        &T::new(Font::ZenBlack, 22.0).fill(pal::TEAL).end(),
    ));
    let tag_t = T::new(Font::Mono, 10.0).fill(pal::FG);
    let mut x = hx;
    for t in &r.tags {
        let tw = typo::width(t, &tag_t) + 16.0;
        if x + tw > w - 20.0 {
            break;
        }
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{}" width="{tw}" height="20" rx="10" fill="none" stroke="{}"/>"#,
            py + 58.0,
            pal::LINE
        );
        b.push_str(&doc.text(x + 8.0, py + 72.0, t, &tag_t));
        x += tw + 8.0;
    }

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = ".flip{animation:flip 7s steps(1) infinite}@keyframes flip{0%,62%{opacity:1}63%,100%{opacity:0}}.flop{opacity:0;animation:flop 7s steps(1) infinite}@keyframes flop{0%,62%{opacity:0}63%,100%{opacity:1}}.pulse{animation:pulse 2.4s ease-in-out infinite}@keyframes pulse{50%{opacity:.35}}.bar{transform-box:fill-box;transform-origin:left;animation:bar 2.6s cubic-bezier(.16,1,.3,1) both}@keyframes bar{from{transform:scaleX(0)}}";
    let defs =
        format!(r#"<clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"#);
    let label = format!("Active research: {}, {}", r.name, r.subtitle);
    Ok(vec![(
        "research.svg".into(),
        kit::svg(w, H, &label, css, &defs, &doc, &frame),
    )])
}
