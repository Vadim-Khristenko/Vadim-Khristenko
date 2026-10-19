//! A CAT (awg-containers-and-tools): a night container yard. One stencilled
//! container per AmneziaWG generation plus the resolver and status images, a
//! cat loafing on the newest one, and awg-tool typing real commands.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 380.0;
const NIGHT2: &str = "#070a0f";
const STEEL: &str = "#c9d3dd";
const PAPERW: &str = "#e9eef3";
const TEAL: &str = "#39c5bb";

const CONTAINERS: [(&str, &str); 5] = [
    ("AWG 1.0", "#8a4b3a"),
    ("AWG 1.5", "#3d5a73"),
    ("AWG 2.0", "#5a6b3a"),
    ("AWG 3.0", "#7a5a2a"),
    ("AWG 3.1", "#2f7d76"),
];

/// Straight from the project README.
const TERM: [&str; 5] = [
    "awg-tool gen --version 3.1",
    "awg-tool install --host 203.0.113.9 --user root",
    "awg-tool peer add phone --vpn",
    "awg-tool status --server home",
    "awg-tool doctor",
];

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "acat", "repo", "awg-containers-and-tools");
    let version = kit::show(ctx, "acat", "version", "");
    let images = kit::show(ctx, "acat", "images", "vaiprog/amnezia-wg-*");

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="url(#sky)"/>"#);
    for (sx, sy, d) in [
        (250, 34, 0.0),
        (318, 72, 1.3),
        (402, 28, 0.6),
        (455, 96, 2.1),
        (520, 52, 1.7),
        (286, 128, 0.9),
    ] {
        let _ = write!(
            b,
            r#"<circle class="star" style="animation-delay:{d}s" cx="{sx}" cy="{sy}" r="1.1" fill="{PAPERW}"/>"#
        );
    }
    let g = H - 40.0;
    let _ = write!(
        b,
        r##"<g fill="none" stroke="#1a2433" stroke-width="5"><path d="M470 {g} V96 M520 {g} V96 M440 96 H560 M470 140 L520 96 M470 190 L520 140 M470 240 L520 190"/></g><path d="M400 96 H560 V104 H400z" fill="#1a2433"/><rect x="432" y="104" width="2" height="60" fill="#1a2433"/><rect x="424" y="164" width="18" height="8" fill="#1a2433"/>"##
    );
    let (lx, ly) = (330.0, 70.0);
    let _ = write!(
        b,
        r##"<path class="lamp" d="M{a} {c} L{bb} {c} L{d} {g} L{e} {g}z" fill="url(#cone)"/><rect x="{f}" y="{ly}" width="3" height="{hh}" fill="#1c2533"/><path d="M{h} {ly} h24 l-4 7 h-16z" fill="#2a3444"/><rect x="{i}" y="{j}" width="16" height="3" fill="#ffe2a0"/>"##,
        a = lx - 4.0,
        c = ly + 6.0,
        bb = lx + 4.0,
        d = lx + 190.0,
        e = lx - 300.0,
        f = lx - 1.5,
        hh = g - ly,
        h = lx - 12.0,
        i = lx - 8.0,
        j = ly + 6.0
    );
    let _ = write!(
        b,
        r##"<rect x="0" y="{g}" width="{w}" height="40" fill="#080b10"/><line x1="0" y1="{g}" x2="{w}" y2="{g}" stroke="{STEEL}" stroke-opacity=".22"/><line x1="0" y1="{d}" x2="{w}" y2="{d}" stroke="#ffc94a" stroke-opacity=".28" stroke-width="2" stroke-dasharray="22 16"/>"##,
        d = H - 18.0
    );

    b.push_str(&doc.text(
        40.0,
        92.0,
        "A CAT",
        &T::new(Font::Stencil, 84.0).fill(PAPERW).ls(2.0),
    ));
    b.push_str(&doc.text(
        44.0,
        120.0,
        "awg-containers-and-tools",
        &T::new(Font::Mono, 11.5).fill(TEAL),
    ));
    b.push_str(&doc.text(
        44.0,
        140.0,
        "self-hosted AmneziaWG, every generation, one host",
        &T::new(Font::Mono, 11.0).fill(STEEL).op(0.8),
    ));

    let (cw, ch) = (132.0, 54.0);
    let base_y = g - ch;
    let pos = [
        (40.0, base_y),
        (40.0 + cw + 6.0, base_y),
        (40.0 + 2.0 * (cw + 6.0), base_y),
        (40.0 + (cw + 6.0) / 2.0, base_y - ch - 4.0),
        (40.0 + (cw + 6.0) * 1.5, base_y - ch - 4.0),
    ];
    for ((label, col), (x, y)) in CONTAINERS.iter().zip(pos) {
        let _ = write!(
            b,
            r##"<rect x="{x}" y="{y}" width="{cw}" height="{ch}" fill="{col}"/><rect x="{x}" y="{y}" width="{cw}" height="{ch}" fill="url(#rib)"/><rect x="{x}" y="{y}" width="{cw}" height="{ch}" fill="none" stroke="#000" stroke-opacity=".45"/><rect x="{a}" y="{c}" width="6" height="6" fill="#000" fill-opacity=".35"/><rect x="{d}" y="{c}" width="6" height="6" fill="#000" fill-opacity=".35"/>"##,
            a = x + 3.0,
            c = y + 3.0,
            d = x + cw - 9.0
        );
        b.push_str(&doc.text(
            x + 12.0,
            y + 30.0,
            label,
            &T::new(Font::Stencil, 22.0).fill(PAPERW).op(0.9).ls(1.0),
        ));
        b.push_str(&doc.text(
            x + 12.0,
            y + 45.0,
            "~33 MB · amd64 · arm64",
            &T::new(Font::Mono, 8.5).fill(PAPERW).op(0.75),
        ));
    }
    for (i, (label, sub, col)) in [
        ("DNS", "~20 MB · .vpn", "#4b4f63"),
        ("STATUS", "amiunder.vpn", "#6b3a5a"),
    ]
    .iter()
    .enumerate()
    {
        let (x, y) = (40.0 + 3.0 * (cw + 6.0), base_y - i as f64 * (ch - 12.0));
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{a}" width="86" height="{hh}" fill="{col}"/><rect x="{x}" y="{a}" width="86" height="{hh}" fill="url(#rib)"/>"#,
            a = y + 12.0,
            hh = ch - 12.0
        );
        b.push_str(&doc.text(
            x + 8.0,
            y + 34.0,
            label,
            &T::new(Font::Stencil, 17.0).fill(PAPERW).op(0.9),
        ));
        b.push_str(&doc.text(
            x + 8.0,
            y + 47.0,
            sub,
            &T::new(Font::Mono, 8.0).fill(PAPERW).op(0.75),
        ));
    }

    // the cat
    let (cx, cy) = (pos[4].0 + 88.0, pos[4].1);
    let (hx, hy) = (cx + 16.0, cy - 24.0);
    let cat = "#05070a";
    let _ = write!(
        b,
        r##"<g fill="{cat}"><path d="M{a} {cy} C{a} {c1} {c2} {c3} {c4} {cy}z"/><ellipse cx="{hx}" cy="{hy}" rx="13" ry="11.5"/><path d="M{e1} {e2} L{e3} {e4} L{e5} {e6}z"/><path d="M{f1} {e6} L{f3} {e4} L{f5} {e2}z"/><path class="tail" d="M{t1} {t2} C{t3} {t4} {t5} {t6} {t7} {t8}" fill="none" stroke="{cat}" stroke-width="5" stroke-linecap="round"/></g><path d="M{i1} {i2} L{i3} {i4} L{i5} {i6}z M{j1} {i6} L{j3} {i4} L{j5} {i2}z" fill="#2a1a24"/><g class="eyes"><ellipse cx="{y1}" cy="{y2}" rx="2.1" ry="2.7" fill="{TEAL}"/><ellipse cx="{y3}" cy="{y2}" rx="2.1" ry="2.7" fill="{TEAL}"/></g><path d="M{n1} {n2} h2.4 l-1.2 1.4z" fill="#3a2a30"/>"##,
        a = cx - 30.0,
        c1 = cy - 26.0,
        c2 = cx + 20.0,
        c3 = cy - 30.0,
        c4 = cx + 30.0,
        e1 = hx - 12.0,
        e2 = hy - 4.0,
        e3 = hx - 10.0,
        e4 = hy - 21.0,
        e5 = hx - 2.0,
        e6 = hy - 10.0,
        f1 = hx + 2.0,
        f3 = hx + 10.0,
        f5 = hx + 12.0,
        t1 = cx - 28.0,
        t2 = cy - 3.0,
        t3 = cx - 46.0,
        t4 = cy - 4.0,
        t5 = cx - 48.0,
        t6 = cy - 22.0,
        t7 = cx - 38.0,
        t8 = cy - 30.0,
        i1 = hx - 9.5,
        i2 = hy - 7.0,
        i3 = hx - 8.5,
        i4 = hy - 16.0,
        i5 = hx - 4.5,
        i6 = hy - 10.0,
        j1 = hx + 4.5,
        j3 = hx + 8.5,
        j5 = hx + 9.5,
        y1 = hx - 5.0,
        y2 = hy - 1.0,
        y3 = hx + 5.0,
        n1 = hx - 1.2,
        n2 = hy + 4.0
    );

    // terminal
    let (tx, ty, tw, th) = (560.0, 40.0, 400.0, 230.0);
    let _ = write!(
        b,
        r##"<rect x="{tx}" y="{ty}" width="{tw}" height="{th}" rx="10" fill="#05080c" stroke="{STEEL}" stroke-opacity=".22"/><line x1="{tx}" y1="{a}" x2="{c}" y2="{a}" stroke="{STEEL}" stroke-opacity=".14"/>"##,
        a = ty + 36.0,
        c = tx + tw
    );
    b.push_str(&doc.text(
        tx + 16.0,
        ty + 24.0,
        "root@yard: ~",
        &T::new(Font::Mono, 10.0).fill(STEEL).op(0.6),
    ));
    let ver = if version.is_empty() {
        "awg-tool · Rust".to_string()
    } else {
        format!("awg-tool {version} · Rust")
    };
    b.push_str(&doc.text(
        tx + tw - 16.0,
        ty + 24.0,
        &ver,
        &T::new(Font::Mono, 10.0).fill(TEAL).end(),
    ));
    for (i, cmd) in TERM.iter().enumerate() {
        let y = ty + 64.0 + i as f64 * 30.0;
        b.push_str(&doc.text(tx + 18.0, y, "$", &T::new(Font::Mono, 12.0).fill(TEAL)));
        b.push_str(
            &doc.text(
                tx + 34.0,
                y,
                cmd,
                &T::new(Font::Mono, 12.0)
                    .fill(PAPERW)
                    .class(&format!("l{i}")),
            ),
        );
    }
    b.push_str(&doc.text(
        tx,
        ty + th + 28.0,
        "five generations side by side · talks to the daemon over UAPI",
        &T::new(Font::Mono, 10.5).fill(STEEL).op(0.8),
    ));
    let label_t = T::new(Font::Mono, 10.5).fill(STEEL).op(0.8);
    let img_t = T::new(Font::MonoBold, 10.5).fill(TEAL);
    b.push_str(&doc.spans(
        tx,
        ty + th + 46.0,
        &[
            ("images on Docker Hub: ", &label_t),
            (images.as_str(), &img_t),
        ],
    ));
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        tx + tw,
        ty + th + 64.0,
        STEEL,
        TEAL,
    ));

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{STEEL}" stroke-opacity=".18"/>"#,
        w - 1.0,
        H - 1.0
    );
    let step = 1.6;
    let lp = step * TERM.len() as f64 + 3.0;
    let mut css = String::from(".eyes{transform-box:fill-box;transform-origin:center;animation:blinkcat 5s infinite}@keyframes blinkcat{0%,90%,100%{transform:scaleY(1)}94%{transform:scaleY(.1)}}.tail{transform-box:fill-box;transform-origin:100% 100%;animation:tail 4s ease-in-out infinite alternate}@keyframes tail{from{transform:rotate(-6deg)}to{transform:rotate(10deg)}}.lamp{animation:lamp 3s ease-in-out infinite}@keyframes lamp{50%{opacity:.55}}.star{animation:star 3.2s ease-in-out infinite}@keyframes star{50%{opacity:.15}}");
    for (i, cmd) in TERM.iter().enumerate() {
        let a = i as f64 * step / lp * 100.0;
        let c = (i as f64 * step + 1.1) / lp * 100.0;
        let _ = write!(
            css,
            ".l{i}{{animation:t{i} {lp:.1}s steps({n}) infinite}}@keyframes t{i}{{0%,{a:.1}%{{clip-path:inset(0 100% 0 0)}}{c:.1}%,96%{{clip-path:inset(0 0 0 0)}}100%{{clip-path:inset(0 100% 0 0)}}}}",
            n = cmd.len() + 2
        );
    }
    let defs = format!(
        r##"<linearGradient id="sky" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#0d1624"/><stop offset="1" stop-color="{NIGHT2}"/></linearGradient><linearGradient id="cone" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffd27a" stop-opacity=".26"/><stop offset="1" stop-color="#ffd27a" stop-opacity=".03"/></linearGradient><pattern id="rib" width="8" height="10" patternUnits="userSpaceOnUse"><line x1="1" y1="0" x2="1" y2="10" stroke="#000" stroke-opacity=".28" stroke-width="2"/></pattern><clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"##
    );
    Ok(vec![(
        "repo_acat.svg".into(),
        kit::svg(
            w,
            H,
            "A CAT: awg-containers-and-tools, self-hosted AmneziaWG 1.0 to 3.1",
            &css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
