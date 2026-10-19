//! The top repo: Hatsune Miku editor themes. Four real variants as mini
//! editors in the theme's own token colours, a focus ring that walks between
//! them, the editor ports as tabs and dashed slots for the programs to come.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 452.0;
const CYCLE: f64 = 12.0;

struct Variant {
    name: &'static str,
    bg: &'static str,
    fg: &'static str,
    ln: &'static str,
    com: &'static str,
    kw: &'static str,
    func: &'static str,
    ty: &'static str,
    num: &'static str,
    bar: &'static str,
    acc: &'static str,
}

/// Token colours from `vsc/themes/*.json` of HatsuneMikuEditorTheme.
const VARIANTS: [Variant; 4] = [
    Variant {
        name: "Miku Dark",
        bg: "#0d0e1a",
        fg: "#ccd6f6",
        ln: "#3d4470",
        com: "#4a5280",
        kw: "#ff6e8a",
        func: "#39c5bb",
        ty: "#39c5bb",
        num: "#d4a0ff",
        bar: "#070810",
        acc: "#39c5bb",
    },
    Variant {
        name: "Sakura Dark",
        bg: "#130d14",
        fg: "#f0d6e8",
        ln: "#4a3050",
        com: "#6a4860",
        kw: "#ff7799",
        func: "#39c5bb",
        ty: "#99aaff",
        num: "#ffaacc",
        bar: "#0b0810",
        acc: "#e87ea1",
    },
    Variant {
        name: "Miku Light",
        bg: "#f0fafa",
        fg: "#1a3a3a",
        ln: "#a0c8c6",
        com: "#88b0ae",
        kw: "#cc2244",
        func: "#007a72",
        ty: "#2255aa",
        num: "#8833bb",
        bar: "#c0e8e5",
        acc: "#007a72",
    },
    Variant {
        name: "Sakura Light",
        bg: "#fff5f8",
        fg: "#3a1428",
        ln: "#ddb0c0",
        com: "#c088a0",
        kw: "#cc2244",
        func: "#227766",
        ty: "#2244aa",
        num: "#aa2255",
        bar: "#f5c8d8",
        acc: "#cc4477",
    },
];

#[derive(Clone, Copy)]
enum Tok {
    Com,
    Kw,
    Func,
    Fg,
    Ty,
    Num,
}

const CODE: [&[(Tok, &str)]; 5] = [
    &[(Tok::Com, "// miku.rs")],
    &[
        (Tok::Kw, "fn "),
        (Tok::Func, "sing"),
        (Tok::Fg, "(bpm: "),
        (Tok::Ty, "u32"),
        (Tok::Fg, ") -> "),
        (Tok::Ty, "Song"),
        (Tok::Fg, " {"),
    ],
    &[
        (Tok::Fg, "    "),
        (Tok::Kw, "let "),
        (Tok::Fg, "tails = "),
        (Tok::Num, "2"),
        (Tok::Fg, ";"),
    ],
    &[
        (Tok::Fg, "    "),
        (Tok::Ty, "Song"),
        (Tok::Fg, "::"),
        (Tok::Func, "new"),
        (Tok::Fg, "(bpm, "),
        (Tok::Num, "39"),
        (Tok::Fg, ")"),
    ],
    &[(Tok::Fg, "}")],
];

fn tok_color(v: &Variant, t: Tok) -> &'static str {
    match t {
        Tok::Com => v.com,
        Tok::Kw => v.kw,
        Tok::Func => v.func,
        Tok::Fg => v.fg,
        Tok::Ty => v.ty,
        Tok::Num => v.num,
    }
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "miku_themes", "repo", "HatsuneMikuEditorTheme");
    let mut editors = kit::show_list(ctx, "miku_themes", "editors");
    if editors.is_empty() {
        editors = [
            "VS Code",
            "Zed",
            "JetBrains",
            "Neovim",
            "Sublime Text",
            "Notepad++",
        ]
        .map(String::from)
        .to_vec();
    }
    let next_ports = kit::show_num(ctx, "miku_themes", "next_ports", 2).clamp(0, 4) as usize;

    let _ = write!(
        b,
        r#"<rect width="{w}" height="{H}" fill="{}"/><rect width="{w}" height="{H}" fill="url(#glow)"/>"#,
        pal::BG
    );
    b.push_str(&doc.text(
        40.0,
        66.0,
        "TOP REPO",
        &T::new(Font::Mono, 11.0).fill(pal::SAKURA).ls(2.4),
    ));
    let hx = 40.0 + typo::width("TOP REPO", &T::new(Font::Mono, 11.0).ls(2.4)) + 14.0;
    let _ = write!(
        b,
        r#"<path class="heart" style="transform-origin:{hx}px 62px" d="M{hx} 67 l-6.5 -6.2 a3.8 3.8 0 0 1 6.5 -4.6 a3.8 3.8 0 0 1 6.5 4.6z" fill="{}"/>"#,
        pal::SAKURA
    );
    b.push_str(&doc.text(
        38.0,
        118.0,
        "Hatsune Miku",
        &T::new(Font::ZenBlack, 44.0).fill(pal::FG_HI).ls(-0.5),
    ));
    b.push_str(&doc.text(
        38.0,
        164.0,
        "editor themes",
        &T::new(Font::ZenLight, 44.0).fill(pal::TEAL).ls(-0.5),
    ));
    let body_t = T::new(Font::ZenLight, 14.5).fill(pal::FG);
    for (i, line) in [
        format!(
            "One palette contract, four looks, {} editors.",
            editors.len()
        ),
        "Readable for hours, checked by a contrast tool,".to_string(),
        "not by eye. Themes for more programs are next.".to_string(),
    ]
    .iter()
    .enumerate()
    {
        b.push_str(&doc.text(40.0, 204.0 + i as f64 * 22.0, line, &body_t));
    }
    let mut sx = 40.0;
    for (n, label) in [
        ("4".to_string(), "LOOKS"),
        (editors.len().to_string(), "EDITORS"),
        ("1".to_string(), "MIT LICENSE"),
    ] {
        b.push_str(&doc.text(
            sx,
            300.0,
            &n,
            &T::new(Font::ZenBlack, 30.0).fill(pal::FG_HI),
        ));
        b.push_str(&doc.text(
            sx,
            318.0,
            label,
            &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.2),
        ));
        sx += 108.0;
    }
    b.push_str(&doc.text(
        40.0,
        350.0,
        &format!("github.com/Vadim-Khristenko/{repo}"),
        &T::new(Font::Mono, 10.0).fill(pal::DIM),
    ));

    // 2x2 grid of mini editors
    let (gx, gy, ew, eh, gap) = (452.0, 36.0, 252.0, 150.0, 12.0);
    for (i, v) in VARIANTS.iter().enumerate() {
        let ex = gx + (i % 2) as f64 * (ew + gap);
        let ey = gy + (i / 2) as f64 * (eh + gap);
        let _ = write!(
            b,
            r#"<rect x="{ex}" y="{ey}" width="{ew}" height="{eh}" rx="8" fill="{bg}" stroke="{line}" stroke-opacity=".6"/><rect x="{ex}" y="{fy}" width="{ew}" height="20" fill="{bar}"/><rect x="{ex}" y="{ey}" width="{ew}" height="22" rx="8" fill="{bar}" fill-opacity=".6"/><rect x="{tx}" y="{ey}" width="62" height="22" fill="{bg}"/><rect x="{tx}" y="{ey}" width="62" height="2" fill="{acc}"/>"#,
            bg = v.bg,
            line = pal::LINE,
            fy = ey + eh - 20.0,
            bar = v.bar,
            tx = ex + 10.0,
            acc = v.acc
        );
        b.push_str(&doc.text(
            ex + 18.0,
            ey + 15.0,
            "miku.rs",
            &T::new(Font::Mono, 9.0).fill(v.fg),
        ));
        b.push_str(&doc.text(
            ex + 12.0,
            ey + eh - 6.0,
            v.name,
            &T::new(Font::MonoBold, 9.5).fill(v.acc),
        ));
        b.push_str(&doc.text(
            ex + ew - 12.0,
            ey + eh - 6.0,
            &format!("{} / {}", v.bg, v.acc),
            &T::new(Font::Mono, 9.0).fill(v.com).end(),
        ));
        for (li, toks) in CODE.iter().enumerate() {
            let ly = ey + 40.0 + li as f64 * 16.0;
            b.push_str(&doc.text(
                ex + 12.0,
                ly,
                &(li + 1).to_string(),
                &T::new(Font::Mono, 9.5).fill(v.ln),
            ));
            let styles: Vec<T> = toks
                .iter()
                .map(|(t, _)| T::new(Font::Mono, 10.0).fill(tok_color(v, *t)))
                .collect();
            let runs: Vec<(&str, &T)> = toks
                .iter()
                .zip(&styles)
                .map(|((_, s), t)| (*s, t))
                .collect();
            b.push_str(&doc.spans(ex + 30.0, ly, &runs));
        }
        if i == 0 {
            let _ = write!(
                b,
                r#"<rect class="blink" x="{}" y="{}" width="6" height="11" fill="{}"/>"#,
                ex + 38.0,
                ey + 40.0 + 4.0 * 16.0 - 9.0,
                v.acc
            );
        }
        let _ = write!(
            b,
            r#"<rect class="focus" style="animation-delay:{}s" x="{}" y="{}" width="{}" height="{}" rx="10" fill="none" stroke="{}" stroke-width="2"/>"#,
            i as f64 * CYCLE / 4.0,
            ex - 3.0,
            ey - 3.0,
            ew + 6.0,
            eh + 6.0,
            v.acc
        );
    }

    // port tabs
    let ty = 380.0;
    let _ = write!(
        b,
        r#"<line x1="0" y1="{ty}" x2="{w}" y2="{ty}" stroke="{}" stroke-opacity=".7"/><rect x="0" y="{ty}" width="{w}" height="{}" fill="{}"/>"#,
        pal::LINE,
        H - ty,
        pal::BG_DEEP
    );
    b.push_str(&doc.text(
        40.0,
        ty + 28.0,
        "PORTS",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(2.0),
    ));
    let tab_t = T::new(Font::Mono, 11.0).fill(pal::FG);
    let mut x = 106.0;
    for e in &editors {
        let tw = typo::width(e, &tab_t) + 18.0;
        let _ = write!(
            b,
            r##"<rect x="{x}" y="{}" width="{tw}" height="26" rx="4" fill="#11121f" stroke="{}" stroke-opacity=".7"/><rect x="{x}" y="{}" width="{tw}" height="2" rx="1" fill="{}"/>"##,
            ty + 12.0,
            pal::LINE,
            ty + 12.0,
            pal::TEAL
        );
        b.push_str(&doc.text(x + 9.0, ty + 29.0, e, &tab_t));
        x += tw + 8.0;
    }
    let ghost_t = T::new(Font::Mono, 11.0).fill(pal::SAKURA);
    for _ in 0..next_ports {
        let label = "+ next port";
        let tw = typo::width(label, &ghost_t) + 18.0;
        if x + tw > w - 20.0 {
            break;
        }
        let _ = write!(
            b,
            r#"<g class="ghost"><rect x="{x}" y="{}" width="{tw}" height="26" rx="4" fill="none" stroke="{}" stroke-dasharray="4 3"/>"#,
            ty + 12.0,
            pal::SAKURA
        );
        b.push_str(&doc.text(x + 9.0, ty + 29.0, label, &ghost_t));
        b.push_str("</g>");
        x += tw + 12.0;
    }
    b.push_str(&doc.text(
        40.0,
        ty + 60.0,
        "VS Code themes are the source of truth, every other port follows them.",
        &T::new(Font::ZenLight, 11.0).fill(pal::DIM),
    ));
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        w - 24.0,
        ty + 60.0,
        pal::DIM,
        pal::TEAL,
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = format!(".focus{{opacity:0;animation:focus {CYCLE}s infinite}}@keyframes focus{{0%,22%{{opacity:1}}25%,100%{{opacity:0}}}}.ghost{{animation:breathe 3.2s ease-in-out infinite}}@keyframes breathe{{50%{{opacity:.35}}}}.blink{{animation:blink 1.1s steps(1) infinite}}@keyframes blink{{50%{{opacity:0}}}}.heart{{animation:beat 1.6s ease-in-out infinite}}@keyframes beat{{0%,40%,100%{{transform:scale(1)}}12%{{transform:scale(1.25)}}24%{{transform:scale(1.05)}}}}");
    let defs = format!(
        r#"<radialGradient id="glow" cx=".75" cy=".35" r=".7"><stop offset="0" stop-color="{t}" stop-opacity=".16"/><stop offset="1" stop-color="{t}" stop-opacity="0"/></radialGradient><clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"#,
        t = pal::TEAL
    );
    let label = format!(
        "Hatsune Miku editor themes: four looks, {} editors, more coming",
        editors.len()
    );
    Ok(vec![(
        "repo_miku_themes.svg".into(),
        kit::svg(w, H, &label, &css, &defs, &doc, &frame),
    )])
}
