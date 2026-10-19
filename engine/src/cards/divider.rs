//! Dividers from the same score as the header. One labelled ruler per README
//! section (`[[sections]]` → divider_<key>.svg) plus three ornaments:
//! divider_staff.svg (notes light up in turn), divider_wave.svg (a playhead
//! rides a vocal waveform) and divider_tails.svg (twin tails crossing at 39).

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const FADE: &str = r##"<linearGradient id="fade" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#39c5bb" stop-opacity="0"/><stop offset=".2" stop-color="#39c5bb" stop-opacity=".8"/><stop offset=".8" stop-color="#e87ea1" stop-opacity=".8"/><stop offset="1" stop-color="#e87ea1" stop-opacity="0"/></linearGradient><linearGradient id="spark" x1="0" y1="0" x2="1" y2="0"><stop offset="0" stop-color="#fff" stop-opacity="0"/><stop offset=".5" stop-color="#fff" stop-opacity=".9"/><stop offset="1" stop-color="#fff" stop-opacity="0"/></linearGradient>"##;

fn section(n: usize, label: &str) -> String {
    let w = kit::W;
    let h = 56.0;
    let mut doc = Doc::new();
    let mut b = format!(
        r#"<line x1="0" y1="34" x2="{w}" y2="34" stroke="url(#fade)" stroke-width="1.2"/>"#
    );
    let mut x = 20.0;
    while x < w {
        let big = (x as i64) % 100 == 0;
        let _ = write!(
            b,
            r#"<line x1="{x}" y1="{}" x2="{x}" y2="34" stroke="{}" stroke-opacity="{}"/>"#,
            if big { 27 } else { 31 },
            pal::LINE,
            if big { ".9" } else { ".5" }
        );
        x += 20.0;
    }
    let text = format!("{n:02} · {label}");
    let t = T::new(Font::MonoBold, 12.0).fill(pal::TEAL).ls(2.0).mid();
    let tw = typo::width(&text, &t) + 26.0;
    let _ = write!(
        b,
        r#"<path d="M{} 34 L{} 16 H{} L{} 34z" fill="{}" stroke="{}" stroke-opacity=".7"/>"#,
        w / 2.0 - tw / 2.0 - 14.0,
        w / 2.0 - tw / 2.0,
        w / 2.0 + tw / 2.0,
        w / 2.0 + tw / 2.0 + 14.0,
        pal::BG,
        pal::TEAL
    );
    b.push_str(&doc.text(w / 2.0, 30.0, &text, &t));
    b.push_str(
        r#"<g class="glint"><rect x="0" y="33" width="90" height="2.4" fill="url(#spark)"/></g>"#,
    );
    let css = format!(".glint{{animation:glint 6s cubic-bezier(.65,0,.35,1) infinite}}@keyframes glint{{from{{transform:translateX(-120px)}}to{{transform:translateX({}px)}}}}", w + 120.0);
    kit::svg(w, h, &format!("Section: {label}"), &css, FADE, &doc, &b)
}

fn staff() -> String {
    let w = kit::W;
    let mut doc = Doc::new();
    let mut b = String::new();
    for i in 0..5 {
        let y = 14.0 + i as f64 * 9.0;
        let _ = write!(
            b,
            r#"<line x1="40" y1="{y}" x2="{}" y2="{y}" stroke="{}" stroke-opacity=".8"/>"#,
            w - 40.0,
            pal::LINE
        );
    }
    b.push_str(&doc.text(
        44.0,
        51.0,
        "𝄞",
        &T::new(Font::ZenBlack, 46.0).fill(pal::TEAL).op(0.85),
    ));
    let _ = write!(
        b,
        r#"<line x1="{x}" y1="14" x2="{x}" y2="50" stroke="{}" stroke-width="2"/>"#,
        pal::LINE,
        x = w - 40.0
    );
    let notes = [3, 2, 4, 1, 3, 5, 2, 4, 3, 1, 2, 4, 3, 5, 2, 3];
    for (i, step) in notes.iter().enumerate() {
        let x = 120.0 + i as f64 * 50.0;
        let y = 50.0 - *step as f64 * 4.5;
        let col = if i % 3 == 0 { pal::SAKURA } else { pal::TEAL };
        let _ = write!(
            b,
            r#"<g class="note" style="animation-delay:{:.2}s"><ellipse cx="{x}" cy="{y}" rx="5.4" ry="4" transform="rotate(-20 {x} {y})" fill="{col}"/><line x1="{a}" y1="{c}" x2="{a}" y2="{d}" stroke="{col}" stroke-width="1.4"/></g>"#,
            i as f64 * 0.375,
            a = x + 5.0,
            c = y - 1.0,
            d = y - 24.0
        );
    }
    let css = ".note{animation:hit 6s linear infinite;opacity:.45}@keyframes hit{0%{opacity:1}10%{opacity:.45}100%{opacity:.45}}";
    kit::svg(w, 64.0, "Divider: a staff of notes", css, FADE, &doc, &b)
}

fn wave() -> String {
    let w = kit::W;
    let mut pts = Vec::new();
    let mut x = 0.0;
    while x <= w {
        let env = (std::f64::consts::PI * x / w).sin().powf(0.6);
        let y = 24.0 + 14.0 * env * (x * 0.09).sin() * (x * 0.021).cos();
        pts.push(format!("{x},{y:.1}"));
        x += 4.0;
    }
    let path = format!("M{}", pts.join(" L"));
    let b = format!(
        r##"<path d="{path}" fill="none" stroke="url(#fade)" stroke-width="1.6"/><circle r="9" fill="{}" opacity=".25"><animateMotion dur="8s" repeatCount="indefinite" path="{path}"/></circle><circle r="4" fill="#fff"><animateMotion dur="8s" repeatCount="indefinite" path="{path}"/></circle>"##,
        pal::TEAL
    );
    kit::svg(
        w,
        48.0,
        "Divider: vocal waveform",
        "",
        FADE,
        &Doc::new(),
        &b,
    )
}

fn tails() -> String {
    let w = kit::W;
    let mut doc = Doc::new();
    let mut b = format!(
        r#"<g class="sway"><path d="M0 18 C 250 18, 380 46, 500 30 S 760 12, {w} 40" fill="none" stroke="{}" stroke-width="2" stroke-opacity=".8"/></g><g class="sway2"><path d="M0 42 C 250 44, 380 12, 500 30 S 760 50, {w} 20" fill="none" stroke="{}" stroke-width="2" stroke-opacity=".7"/></g><circle cx="500" cy="30" r="15" fill="{}" stroke="{}"/>"#,
        pal::TEAL,
        pal::SAKURA,
        pal::BG,
        pal::TEAL
    );
    b.push_str(&doc.text(
        500.0,
        35.0,
        "39",
        &T::new(Font::MonoBold, 13.0).fill(pal::FG_HI).mid(),
    ));
    let css = ".sway{transform-origin:500px 30px;animation:sway 7s ease-in-out infinite alternate}.sway2{transform-origin:500px 30px;animation:sway 9s ease-in-out infinite alternate-reverse}@keyframes sway{from{transform:rotate(-1deg)}to{transform:rotate(1deg)}}";
    kit::svg(w, 60.0, "Divider: twin tails", css, FADE, &doc, &b)
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut out: Vec<(String, String)> = ctx
        .cfg
        .profile
        .sections
        .iter()
        .enumerate()
        .map(|(i, s)| (format!("divider_{}.svg", s.key), section(i + 1, &s.label)))
        .collect();
    out.push(("divider_staff.svg".into(), staff()));
    out.push(("divider_wave.svg".into(), wave()));
    out.push(("divider_tails.svg".into(), tails()));
    Ok(out)
}
