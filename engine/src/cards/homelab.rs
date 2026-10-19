//! Homelab: the laptops at home drawn as an isometric desk, each screen
//! showing its distro, a router at the corner and uplinks to the VPS slabs
//! floating on the right. Nodes come from `[showcase.homelab]`, so adding a
//! laptop or a server is a config change.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 520.0;
const C: f64 = 0.866;
// laptop: deck width (x), depth (y), thickness, screen height, screen tilt
const LW: f64 = 96.0;
const LD: f64 = 62.0;
const LT: f64 = 5.0;
const SH: f64 = 60.0;
const TILT: f64 = 10.0;
const SX: f64 = 124.0;
const SY: f64 = 104.0;

fn distro(key: &str) -> (&'static str, &'static str) {
    match key.to_ascii_lowercase().as_str() {
        "arch" => ("Arch Linux", "#1793d1"),
        "nix" | "nixos" => ("NixOS", "#7ebae4"),
        "ubuntu" => ("Ubuntu", "#e95420"),
        "debian" => ("Debian", "#d70a53"),
        "windows" | "win11" => ("Windows 11", "#4cc2ff"),
        _ => ("Linux", pal::TEAL),
    }
}

fn iso(x: f64, y: f64, z: f64) -> (f64, f64) {
    ((x - y) * C, (x + y) * 0.5 - z)
}

fn poly(pts: &[(f64, f64, f64)], at: (f64, f64, f64), attrs: &str) -> String {
    let p: Vec<String> = pts
        .iter()
        .map(|&(x, y, z)| {
            let (sx, sy) = iso(at.0 + x, at.1 + y, at.2 + z);
            format!("{sx:.1},{sy:.1}")
        })
        .collect();
    format!(r#"<polygon points="{}" {attrs}/>"#, p.join(" "))
}

/// A laptop on the desk. One system makes it a server with a scrolling log;
/// several make it the main machine with a boot menu flipping between them.
fn laptop(doc: &mut Doc, i: usize, systems: &[String], at: (f64, f64, f64)) -> String {
    let os = systems.first().map(String::as_str).unwrap_or("linux");
    let (short, color) = (os.to_ascii_lowercase(), distro(os).1);
    let main = systems.len() > 1;
    let mut s = String::new();
    // screen panel, its back edge on the hinge
    s.push_str(&poly(
        &[
            (0.0, 0.0, LT),
            (LW, 0.0, LT),
            (LW, -TILT, LT + SH),
            (0.0, -TILT, LT + SH),
        ],
        at,
        r##"fill="#1a1d36" stroke="#3d4470" stroke-width="1""##,
    ));
    // the display lives in the panel's own plane
    let (px, py) = iso(at.0, at.1 - TILT, at.2 + LT + SH);
    let (c, d) = (TILT * C / SH, (SH + 0.5 * TILT) / SH);
    let _ = write!(
        s,
        r##"<g transform="matrix({C} 0.5 {:.4} {d:.4} {px:.1} {py:.1})"><rect x="4" y="4" width="{}" height="{}" rx="2" fill="#070812"/><rect x="4" y="4" width="{}" height="{}" rx="2" fill="{color}" opacity=".07"/>"##,
        -c,
        LW - 8.0,
        SH - 8.0,
        LW - 8.0,
        SH - 8.0
    );
    if main {
        s.push_str(&doc.text(
            9.0,
            14.0,
            "main · dual boot",
            &T::new(Font::Mono, 7.5).fill(pal::DIM),
        ));
        for (k, sys) in systems.iter().take(3).enumerate() {
            let (name, c) = distro(sys);
            let y = 19.0 + k as f64 * 12.0;
            let _ = write!(
                s,
                r#"<rect class="sel" style="animation-delay:{:.1}s" x="7" y="{y}" width="{}" height="11" rx="2" fill="{c}" opacity=".28"/>"#,
                -(k as f64) * 6.0 / systems.len() as f64,
                LW - 14.0
            );
            s.push_str(&doc.text(11.0, y + 8.3, name, &T::new(Font::MonoBold, 8.5).fill(c)));
        }
    } else {
        s.push_str(&doc.text(9.0, 16.0, &short, &T::new(Font::MonoBold, 10.0).fill(color)));
        let widths = [62.0, 44.0, 70.0, 36.0, 54.0, 66.0, 40.0];
        for k in 0..4 {
            let w = widths[(i * 3 + k) % widths.len()];
            let _ = write!(
                s,
                r#"<rect class="ln" style="animation-delay:{:.1}s" x="9" y="{}" width="{w}" height="3" rx="1.5" fill="{}" opacity="{}"/>"#,
                (i * 4 + k) as f64 * 0.37,
                22.0 + k as f64 * 6.5,
                if k % 2 == 0 { color } else { pal::DIM },
                if k % 2 == 0 { ".55" } else { ".35" }
            );
        }
    }
    let _ = write!(
        s,
        r#"<rect class="caret" x="9" y="{}" width="5" height="6" fill="{color}"/></g>"#,
        SH - 15.0
    );
    // deck: top, front and right sides
    s.push_str(&poly(
        &[(0.0, 0.0, LT), (LW, 0.0, LT), (LW, LD, LT), (0.0, LD, LT)],
        at,
        &format!(
            r##"fill="#20243f" stroke="{}" stroke-width="1""##,
            if main { pal::TEAL } else { pal::LINE }
        ),
    ));
    s.push_str(&poly(
        &[(0.0, LD, 0.0), (LW, LD, 0.0), (LW, LD, LT), (0.0, LD, LT)],
        at,
        r##"fill="#141629""##,
    ));
    s.push_str(&poly(
        &[(LW, 0.0, 0.0), (LW, LD, 0.0), (LW, LD, LT), (LW, 0.0, LT)],
        at,
        r##"fill="#0f1122""##,
    ));
    // keyboard and trackpad
    s.push_str(&poly(
        &[
            (8.0, 8.0, LT),
            (LW - 8.0, 8.0, LT),
            (LW - 8.0, 34.0, LT),
            (8.0, 34.0, LT),
        ],
        at,
        r##"fill="#0d0e1a""##,
    ));
    for r in 1..4 {
        let y = 8.0 + r as f64 * 6.5;
        let (a, b) = (
            iso(at.0 + 10.0, at.1 + y, at.2 + LT),
            iso(at.0 + LW - 10.0, at.1 + y, at.2 + LT),
        );
        let _ = write!(
            s,
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{}" stroke-width=".8" stroke-dasharray="4 2"/>"#,
            a.0,
            a.1,
            b.0,
            b.1,
            pal::LINE
        );
    }
    s.push_str(&poly(
        &[
            (LW / 2.0 - 16.0, 40.0, LT),
            (LW / 2.0 + 16.0, 40.0, LT),
            (LW / 2.0 + 16.0, 56.0, LT),
            (LW / 2.0 - 16.0, 56.0, LT),
        ],
        at,
        r##"fill="#191c33""##,
    ));
    // power led on the front edge
    let (lx, ly) = iso(at.0 + LW - 10.0, at.1 + LD, at.2 + LT / 2.0);
    let _ = write!(
        s,
        r#"<circle class="led" style="animation-delay:{:.1}s" cx="{lx:.1}" cy="{ly:.1}" r="1.8" fill="{color}"/>"#,
        i as f64 * 0.45
    );
    s
}

fn slab(doc: &mut Doc, i: usize, os: &str, cx: f64, cy: f64) -> String {
    let (name, color) = distro(os);
    let (w, d, t) = (88.0, 52.0, 14.0);
    let at = (0.0, 0.0, 0.0);
    let mut s = format!(
        r#"<g class="float" style="animation-delay:{:.1}s"><g transform="translate({cx:.1} {cy:.1})">"#,
        i as f64 * 0.6
    );
    s.push_str(&poly(
        &[(0.0, d, 0.0), (w, d, 0.0), (w, d, t), (0.0, d, t)],
        at,
        r##"fill="#141629" stroke="#3d4470" stroke-width=".8""##,
    ));
    s.push_str(&poly(
        &[(w, 0.0, 0.0), (w, d, 0.0), (w, d, t), (w, 0.0, t)],
        at,
        r##"fill="#0f1122" stroke="#3d4470" stroke-width=".8""##,
    ));
    s.push_str(&poly(
        &[(0.0, 0.0, t), (w, 0.0, t), (w, d, t), (0.0, d, t)],
        at,
        &format!(r##"fill="#1d2140" stroke="{color}" stroke-width="1.2""##),
    ));
    for k in 0..5 {
        let (vx, vy) = iso(12.0 + k as f64 * 12.0, d, t / 2.0);
        let _ = write!(
            s,
            r#"<circle class="led" style="animation-delay:{:.1}s" cx="{vx:.1}" cy="{vy:.1}" r="1.6" fill="{}"/>"#,
            (i * 5 + k) as f64 * 0.23,
            if k == 0 { color } else { pal::MUTED }
        );
    }
    for k in 0..3 {
        let y = 14.0 + k as f64 * 10.0;
        let (a, b) = (iso(14.0, y, t), iso(w - 14.0, y, t));
        let _ = write!(
            s,
            r#"<line x1="{:.1}" y1="{:.1}" x2="{:.1}" y2="{:.1}" stroke="{color}" stroke-opacity=".35" stroke-width="2" stroke-linecap="round"/>"#,
            a.0, a.1, b.0, b.1
        );
    }
    s.push_str("</g></g>");
    let lx = cx + w * C + 22.0;
    s.push_str(&doc.text(
        lx,
        cy + 22.0,
        &format!("VPS {:02}", i + 1),
        &T::new(Font::Mono, 10.0).fill(pal::DIM).ls(1.2),
    ));
    s.push_str(&doc.text(
        lx,
        cy + 40.0,
        name,
        &T::new(Font::ZenBlack, 15.0).fill(color),
    ));
    s
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut home = kit::show_list(ctx, "homelab", "home");
    if home.is_empty() {
        home = vec!["arch".into(), "nixos".into()];
    }
    let main = kit::show_list(ctx, "homelab", "main");
    let vps = kit::show_list(ctx, "homelab", "vps");
    // one row on the desk: the main machine first, a gap, then the servers
    let mut machines: Vec<(Vec<String>, f64, f64)> = Vec::new();
    let mut x = 0.0;
    if !main.is_empty() {
        machines.push((main.clone(), 0.0, 0.0));
        x = SX + 24.0;
    }
    for os in &home {
        machines.push((vec![os.clone()], x, 0.0));
        x += SX;
    }
    let w = kit::W;
    let mut doc = Doc::new();
    let mut b = String::new();
    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{}"/>"#, pal::BG);
    // faint floor grid
    for k in 0..14 {
        let x = k as f64 * 80.0 - 40.0;
        let _ = write!(
            b,
            r#"<line x1="{x}" y1="{H}" x2="{:.0}" y2="{:.0}" stroke="{}" stroke-opacity=".25"/>"#,
            x + 260.0,
            H - 150.0,
            pal::LINE
        );
    }

    // desk with the laptops, centred in the left part of the card
    let (pw, pd) = (x + 10.0, SY + 10.0);
    let corners = [
        iso(-30.0, -40.0, LT + SH),
        iso(pw, -40.0, 0.0),
        iso(pw, pd, -12.0),
        iso(-30.0, pd, -12.0),
    ];
    let (minx, maxx) = corners
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, z), p| (a.min(p.0), z.max(p.0)));
    let (miny, maxy) = corners
        .iter()
        .fold((f64::MAX, f64::MIN), |(a, z), p| (a.min(p.1), z.max(p.1)));
    let (dx, dy) = (310.0 - (minx + maxx) / 2.0, 292.0 - (miny + maxy) / 2.0);
    let mut scene = String::new();
    let o = (0.0, 0.0, 0.0);
    let desk = [
        (-30.0, -40.0, 0.0),
        (pw, -40.0, 0.0),
        (pw, pd, 0.0),
        (-30.0, pd, 0.0),
    ];
    scene.push_str(&poly(
        &[
            (-30.0, pd, -12.0),
            (pw, pd, -12.0),
            (pw, pd, 0.0),
            (-30.0, pd, 0.0),
        ],
        o,
        r##"fill="#101224""##,
    ));
    scene.push_str(&poly(
        &[
            (pw, -40.0, -12.0),
            (pw, pd, -12.0),
            (pw, pd, 0.0),
            (pw, -40.0, 0.0),
        ],
        o,
        r##"fill="#0b0c1a""##,
    ));
    scene.push_str(&poly(
        &desk,
        o,
        &format!(
            r##"fill="#161830" stroke="{}" stroke-width="1""##,
            pal::TEAL
        ),
    ));
    // router at the back right corner and a cable to every laptop
    let router = (pw - 30.0, -34.0, 0.0);
    let mut slots: Vec<(usize, f64, f64)> = machines
        .iter()
        .enumerate()
        .map(|(i, m)| (i, m.1, m.2))
        .collect();
    for &(_, x, y) in &slots {
        let a = iso(x + LW, y + LD / 2.0, 0.0);
        let m = iso(x + LW + 14.0, y + LD / 2.0, 0.0);
        let r = iso(router.0, router.1 + 6.0, 0.0);
        let _ = write!(
            scene,
            r#"<path d="M{:.1} {:.1} L{:.1} {:.1} L{:.1} {:.1}" fill="none" stroke="{}" stroke-opacity=".55" stroke-width="1.4"/>"#,
            a.0,
            a.1,
            m.0,
            m.1,
            r.0,
            r.1,
            pal::TEAL
        );
    }
    slots.sort_by(|a, b| (a.1 + a.2).total_cmp(&(b.1 + b.2)));
    for (i, x, y) in slots {
        scene.push_str(&laptop(&mut doc, i, &machines[i].0, (x, y, 0.0)));
    }
    scene.push_str(&poly(
        &[
            (0.0, 0.0, 8.0),
            (26.0, 0.0, 8.0),
            (26.0, 16.0, 8.0),
            (0.0, 16.0, 8.0),
        ],
        router,
        r##"fill="#20243f" stroke="#39c5bb" stroke-width="1""##,
    ));
    scene.push_str(&poly(
        &[
            (0.0, 16.0, 0.0),
            (26.0, 16.0, 0.0),
            (26.0, 16.0, 8.0),
            (0.0, 16.0, 8.0),
        ],
        router,
        r##"fill="#141629""##,
    ));
    scene.push_str(&poly(
        &[
            (26.0, 0.0, 0.0),
            (26.0, 16.0, 0.0),
            (26.0, 16.0, 8.0),
            (26.0, 0.0, 8.0),
        ],
        router,
        r##"fill="#0f1122""##,
    ));
    for k in 0..3 {
        let (lx, ly) = iso(router.0 + 6.0 + k as f64 * 7.0, router.1 + 16.0, 4.0);
        let _ = write!(
            scene,
            r#"<circle class="led" style="animation-delay:{:.1}s" cx="{lx:.1}" cy="{ly:.1}" r="1.5" fill="{}"/>"#,
            k as f64 * 0.3,
            pal::TEAL
        );
    }
    let up = iso(router.0 + 26.0, router.1 + 8.0, 8.0);
    let uplink = (up.0 + dx, up.1 + dy);
    let _ = write!(
        b,
        r#"<g transform="translate({dx:.1} {dy:.1})">{scene}</g>"#
    );

    // VPS slabs and the uplinks to them
    let n = vps.len().max(1) as f64;
    let step = (330.0 / n).min(86.0);
    let top = 140.0 + (330.0 - step * n) / 2.0;
    let mut links = String::new();
    let mut slabs = String::new();
    for (i, os) in vps.iter().enumerate() {
        let (sx, sy) = (690.0, top + i as f64 * step);
        let color = distro(os).1;
        let (tx, ty) = (sx + 2.0, sy + iso(0.0, 26.0, 7.0).1);
        let tx = tx + iso(0.0, 26.0, 7.0).0;
        let id = format!("up{i}");
        let _ = write!(
            links,
            r##"<path id="{id}" d="M{:.1} {:.1} C{:.1} {:.1} {:.1} {:.1} {tx:.1} {ty:.1}" fill="none" stroke="{color}" stroke-opacity=".5" stroke-width="1.4" stroke-dasharray="5 6" class="flow"/><circle r="3" fill="{color}"><animateMotion dur="2.6s" begin="{:.1}s" repeatCount="indefinite"><mpath href="#{id}"/></animateMotion></circle>"##,
            uplink.0,
            uplink.1,
            uplink.0 + 90.0,
            uplink.1,
            tx - 90.0,
            ty,
            i as f64 * 0.5
        );
        slabs.push_str(&slab(&mut doc, i, os, sx, sy));
    }
    b.push_str(&links);
    b.push_str(&slabs);

    // titles
    b.push_str(&doc.text(
        40.0,
        44.0,
        "HOMELAB · SELF-HOSTED",
        &T::new(Font::Mono, 11.0).fill(pal::TEAL).ls(2.4),
    ));
    b.push_str(&doc.text(
        40.0,
        82.0,
        "Home is the datacenter",
        &T::new(Font::UnboundedBold, 26.0).fill(pal::FG_HI).ls(-0.3),
    ));
    let tag_t = T::new(Font::Zen, 13.0).fill(pal::DIM);
    b.push_str(&doc.text(
        40.0,
        106.0,
        &typo::fit(
            &kit::show(
                ctx,
                "homelab",
                "tagline",
                "Most of my infrastructure runs at home, on laptops turned servers.",
            ),
            &tag_t,
            620.0,
        ),
        &tag_t,
    ));
    let label = T::new(Font::MonoBold, 10.0).fill(pal::DIM).ls(1.6);
    b.push_str(&doc.text(
        690.0,
        44.0,
        &format!(
            "LAB {}  ·  MAIN {}  ·  VPS {}",
            home.len(),
            usize::from(!main.is_empty()),
            vps.len()
        ),
        &label,
    ));
    b.push_str(&doc.text(
        690.0,
        62.0,
        "the edge, out in the world",
        &T::new(Font::Zen, 12.0).fill(pal::MUTED),
    ));

    // distro legend
    let mut order: Vec<&str> = Vec::new();
    for os in home.iter().chain(main.iter()).chain(vps.iter()) {
        if !order.iter().any(|k| distro(k).0 == distro(os).0) {
            order.push(os);
        }
    }
    let mut x = 40.0;
    let y = H - 26.0;
    for key in order {
        let (name, color) = distro(key);
        let count = |list: &[String]| list.iter().filter(|o| distro(o).0 == name).count();
        let mut where_ = Vec::new();
        if count(&home) > 0 {
            where_.push(format!("lab {}", count(&home)));
        }
        if count(&main) > 0 {
            where_.push("main".to_string());
        }
        if count(&vps) > 0 {
            where_.push(format!("vps {}", count(&vps)));
        }
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{}" width="10" height="10" rx="2" fill="{color}"/>"#,
            y - 9.0
        );
        let nt = T::new(Font::ZenBlack, 12.0).fill(pal::FG_HI);
        let ct = T::new(Font::Mono, 10.5).fill(pal::DIM);
        let meta = format!("  {}", where_.join(" · "));
        b.push_str(&doc.spans(x + 16.0, y, &[(name, &nt), (&meta, &ct)]));
        x += 16.0 + typo::width(name, &nt) + typo::width(&meta, &ct) + 24.0;
    }

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = ".led{animation:led 2.2s ease-in-out infinite}@keyframes led{50%{opacity:.25}}.caret{animation:caret 1.1s steps(1) infinite}@keyframes caret{50%{opacity:0}}.sel{animation:sel 6s steps(1) infinite}@keyframes sel{50%{opacity:0}}.ln{animation:ln 3.4s ease-in-out infinite}@keyframes ln{50%{opacity:.12}}.flow{animation:flow 1.2s linear infinite}@keyframes flow{to{stroke-dashoffset:-22}}.float{animation:float 5s ease-in-out infinite}@keyframes float{50%{transform:translateY(-5px)}}";
    let defs =
        format!(r#"<clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath>"#);
    Ok(vec![(
        "homelab.svg".into(),
        kit::svg(
            w,
            H,
            "Homelab: two laptop servers on Arch and NixOS, a main laptop dual booting Windows 11 and Arch, VPS on Arch, NixOS, Ubuntu and Debian",
            css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
