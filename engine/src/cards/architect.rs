//! Any Tech Architect as its own working drawing (the project's DESIGN.md
//! concept). The handshake initiation is drawn at the real byte widths of its
//! fields, the wire timeline cycles through the mimicry profiles, and the
//! facts sit in a title block. Amber accent, Unbounded and JetBrains Mono.

use super::kit;
use crate::run::Ctx;
use crate::typo::{Doc, Font, T};
use anyhow::Result;
use std::fmt::Write;

const H: f64 = 420.0;
const PAPER: &str = "#13110d";
const INK: &str = "#f2e4c4";
const INK_DIM: &str = "#a8987a";
const AMB: &str = "#e8a840";

fn dim(doc: &mut Doc, x1: f64, x2: f64, y: f64, label: &str, up: bool) -> String {
    let ty = if up { y - 6.0 } else { y + 14.0 };
    format!(
        r#"<line x1="{x1}" y1="{y}" x2="{x2}" y2="{y}" stroke="{AMB}" marker-start="url(#tick)" marker-end="url(#tick)"/><line x1="{x1}" y1="{a}" x2="{x1}" y2="{c}" stroke="{AMB}" stroke-opacity=".6"/><line x1="{x2}" y1="{a}" x2="{x2}" y2="{c}" stroke="{AMB}" stroke-opacity=".6"/>{t}"#,
        a = y - 6.0,
        c = y + 6.0,
        t = doc.text(
            (x1 + x2) / 2.0,
            ty,
            label,
            &T::new(Font::Mono, 9.5).fill(AMB).mid()
        )
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let repo = kit::show(ctx, "architect", "repo", "Any-Tech-ARCHITECT");
    let site = kit::show(ctx, "architect", "site", "architect.vai-rice.space");
    let version = kit::show(ctx, "architect", "version", "");
    let engines = kit::show(ctx, "architect", "engines", "AmneziaWG · XRay");
    let params = kit::show(ctx, "architect", "params", "");

    let _ = write!(
        b,
        r#"<rect width="{w}" height="{H}" fill="{PAPER}"/><rect width="{w}" height="{H}" fill="url(#grid)"/><rect width="{w}" height="{H}" fill="url(#grid5)"/><rect x="14" y="14" width="{}" height="{}" fill="none" stroke="{AMB}" stroke-opacity=".5"/>"#,
        w - 28.0,
        H - 28.0
    );
    b.push_str(&doc.text(
        40.0,
        62.0,
        "Any Tech Architect",
        &T::new(Font::UnboundedBold, 27.0).fill(INK).ls(-0.5),
    ));
    b.push_str(&doc.text(
        40.0,
        86.0,
        "A working drawing of a packet. Every number says where its bound came from.",
        &T::new(Font::Mono, 11.5).fill(AMB),
    ));
    b.push_str(&kit::live_chip(
        &mut doc,
        ctx,
        &repo,
        w - 40.0,
        62.0,
        INK_DIM,
        AMB,
    ));

    // wire timeline
    let wy = 142.0;
    b.push_str(&doc.text(
        40.0,
        wy - 36.0,
        "ON THE WIRE, LEFT TO RIGHT IN TIME",
        &T::new(Font::Mono, 9.0).fill(INK_DIM).ls(1.6),
    ));
    let _ = write!(
        b,
        r#"<line x1="40" y1="{}" x2="660" y2="{}" stroke="{INK_DIM}" stroke-opacity=".5" stroke-dasharray="2 4"/>"#,
        wy + 12.0,
        wy + 12.0
    );
    let seq = [
        ("I1", 40.0, 48.0, false),
        ("I2", 92.0, 30.0, false),
        ("J1", 140.0, 26.0, true),
        ("J2", 170.0, 40.0, true),
        ("J3", 214.0, 20.0, true),
        ("J4", 238.0, 32.0, true),
        ("init", 290.0, 76.0, false),
        ("resp", 372.0, 52.0, false),
        ("data", 440.0, 96.0, false),
        ("data", 542.0, 72.0, false),
    ];
    for (lab, x, sw, hatch) in seq {
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{wy}" width="{sw}" height="24" fill="{}" stroke="{AMB}" stroke-opacity=".85"/>"#,
            if hatch { "url(#hatch)" } else { PAPER }
        );
        b.push_str(&doc.text(
            x + sw / 2.0,
            wy + 16.0,
            lab,
            &T::new(Font::MonoBold, 9.5).fill(INK).mid(),
        ));
    }
    b.push_str(&dim(
        &mut doc,
        140.0,
        270.0,
        wy - 8.0,
        "Jc = 4 · Jmin…Jmax",
        true,
    ));
    let _ = write!(
        b,
        r#"<rect class="pkt" x="40" y="{}" width="10" height="4" fill="{AMB}"/>"#,
        wy + 30.0
    );
    b.push_str(&doc.text(
        688.0,
        wy + 8.0,
        "LOOKS LIKE",
        &T::new(Font::Mono, 9.0).fill(INK_DIM).ls(1.6),
    ));
    for (i, p) in ["QUIC", "TLS", "SIP", "STUN / TURN", "DNS"]
        .iter()
        .enumerate()
    {
        let t = T::new(Font::UnboundedBold, 20.0)
            .fill(AMB)
            .class(if i == 0 { "proto p0" } else { "proto" })
            .style(&format!("animation-delay:{}s", i * 2));
        b.push_str(&doc.text(688.0, wy + 32.0, p, &t));
    }

    // field map of the handshake initiation, 1 octet = 4.4 px
    let (fx, fy, px) = (40.0, 236.0, 4.4);
    let fields = [
        ("S1", 40.0, true),
        ("H1", 4.0, false),
        ("sender", 4.0, false),
        ("ephemeral", 32.0, false),
        ("static (enc)", 48.0, false),
        ("timestamp (enc)", 28.0, false),
        ("mac1", 16.0, false),
        ("mac2", 16.0, false),
    ];
    b.push_str(&doc.text(
        fx,
        fy - 34.0,
        &format!("FIELD MAP · HANDSHAKE INITIATION · 1 OCTET = {px} PX"),
        &T::new(Font::Mono, 9.0).fill(INK_DIM).ls(1.6),
    ));
    let mut x = fx;
    let mut starts = Vec::new();
    for (name, n, hatch) in fields {
        let fw = n * px;
        starts.push(x);
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{fy}" width="{fw}" height="34" fill="{}" stroke="{AMB}"/>"#,
            if hatch { "url(#hatch)" } else { PAPER }
        );
        if fw > 40.0 {
            b.push_str(&doc.text(
                x + fw / 2.0,
                fy + 15.0,
                name,
                &T::new(Font::MonoBold, 9.5).fill(INK).mid(),
            ));
            b.push_str(&doc.text(
                x + fw / 2.0,
                fy + 27.0,
                &format!("{n} B"),
                &T::new(Font::Mono, 8.5).fill(INK_DIM).mid(),
            ));
        }
        x += fw;
    }
    let end = x;
    for ((name, n, _), sx) in fields.iter().zip(&starts) {
        if n * px <= 40.0 {
            let cx = sx + n * px / 2.0;
            let lift = if *name == "H1" { 14.0 } else { 28.0 };
            let _ = write!(
                b,
                r#"<line x1="{cx}" y1="{fy}" x2="{cx}" y2="{ly}" stroke="{AMB}" stroke-opacity=".8"/><line x1="{cx}" y1="{ly}" x2="{lx}" y2="{ly}" stroke="{AMB}" stroke-opacity=".8"/>"#,
                ly = fy - lift,
                lx = cx + 8.0
            );
            b.push_str(&doc.text(
                cx + 12.0,
                fy - lift + 3.0,
                &format!("{name} · {n} B"),
                &T::new(Font::Mono, 9.0).fill(INK),
            ));
        }
    }
    b.push_str(&dim(
        &mut doc,
        fx,
        fx + 40.0 * px,
        fy + 52.0,
        "S1 padding, size you pick",
        false,
    ));
    b.push_str(&dim(
        &mut doc,
        fx + 40.0 * px,
        end,
        fy + 52.0,
        "148 B, the WireGuard message itself",
        false,
    ));
    let ry = fy + 34.0;
    for o in (0..=188).step_by(4) {
        let xx = fx + o as f64 * px;
        let big = o % 32 == 0;
        let _ = write!(
            b,
            r#"<line x1="{xx}" y1="{ry}" x2="{xx}" y2="{}" stroke="{AMB}" stroke-opacity="{}"/>"#,
            ry + if big { 7.0 } else { 3.0 },
            if big { ".9" } else { ".45" }
        );
    }
    let nx = starts[1] + 2.0 * px;
    let _ = write!(
        b,
        r#"<circle cx="{nx}" cy="{ry}" r="2.5" fill="{AMB}"/><circle class="dot" cx="{nx}" cy="{ry}" r="2.5" fill="none" stroke="{AMB}"/>"#
    );

    // title block
    let (tx, ty, tw, rh) = (580.0, 330.0, 380.0, 18.0);
    let rows = [
        (
            "PROJECT",
            "Any Tech Architect".to_string(),
            "REV",
            version.clone(),
        ),
        ("ENGINES", engines, "PARAMS", params),
        (
            "COMPUTED",
            "in your browser, nothing sent".to_string(),
            "SHEET",
            "1 / 1".to_string(),
        ),
    ];
    let _ = write!(
        b,
        r#"<rect x="{tx}" y="{ty}" width="{tw}" height="{}" fill="{PAPER}" stroke="{AMB}"/>"#,
        rh * 3.0 + 12.0
    );
    for (i, (k1, v1, k2, v2)) in rows.iter().enumerate() {
        let y = ty + 6.0 + i as f64 * rh;
        if i > 0 {
            let _ = write!(
                b,
                r#"<line x1="{tx}" y1="{y}" x2="{}" y2="{y}" stroke="{AMB}" stroke-opacity=".45"/>"#,
                tx + tw
            );
        }
        b.push_str(&doc.text(
            tx + 8.0,
            y + 13.0,
            k1,
            &T::new(Font::Mono, 8.0).fill(INK_DIM).ls(1.0),
        ));
        b.push_str(&doc.text(
            tx + 74.0,
            y + 13.0,
            v1,
            &T::new(Font::MonoBold, 10.0).fill(INK),
        ));
        b.push_str(&doc.text(
            tx + tw - 88.0,
            y + 13.0,
            k2,
            &T::new(Font::Mono, 7.5).fill(INK_DIM).ls(1.0),
        ));
        b.push_str(&doc.text(
            tx + tw - 10.0,
            y + 13.0,
            v2,
            &T::new(Font::MonoBold, 10.0).fill(AMB).end(),
        ));
    }
    let _ = write!(
        b,
        r#"<line x1="{x}" y1="{ty}" x2="{x}" y2="{}" stroke="{AMB}" stroke-opacity=".45"/>"#,
        ty + rh * 3.0 + 12.0,
        x = tx + tw - 96.0
    );
    b.push_str(&doc.text(
        40.0,
        ty + 20.0,
        "hatched = junk you choose · leaders = fields too small to label",
        &T::new(Font::Mono, 10.0).fill(INK_DIM),
    ));
    b.push_str(&doc.text(
        40.0,
        ty + 40.0,
        &site,
        &T::new(Font::MonoBold, 11.0).fill(AMB),
    ));

    let frame = format!(r#"<g clip-path="url(#card)">{b}</g>"#);
    let css = ".proto{opacity:0;animation:proto 10s infinite}@keyframes proto{0%,18%{opacity:1}20%,100%{opacity:0}}.pkt{animation:wire 5s cubic-bezier(.16,1,.3,1) infinite}@keyframes wire{0%{transform:translateX(0);opacity:0}8%{opacity:1}80%{opacity:1}100%{transform:translateX(560px);opacity:0}}.dot{animation:dot 2.4s ease-out infinite}@keyframes dot{0%{r:2.5;opacity:1}100%{r:9;opacity:0}}";
    let defs = format!(
        r#"<pattern id="grid" width="20" height="20" patternUnits="userSpaceOnUse"><path d="M20 0H0V20" fill="none" stroke="{AMB}" stroke-opacity=".06"/></pattern><pattern id="grid5" width="100" height="100" patternUnits="userSpaceOnUse"><path d="M100 0H0V100" fill="none" stroke="{AMB}" stroke-opacity=".11"/></pattern><pattern id="hatch" width="6" height="6" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><line x1="0" y1="0" x2="0" y2="6" stroke="{AMB}" stroke-opacity=".55" stroke-width="1.2"/></pattern><marker id="tick" viewBox="0 0 10 10" refX="5" refY="5" markerWidth="9" markerHeight="9" orient="auto"><path d="M2 8 L8 2" stroke="{AMB}" stroke-width="1.4"/></marker><clipPath id="card"><rect width="{w}" height="{H}" rx="6"/></clipPath>"#
    );
    // The first protocol stays visible when animation is off.
    let css = format!("{css}.p0{{opacity:1}}");
    Ok(vec![(
        "repo_architect.svg".into(),
        kit::svg(
            w,
            H,
            "Any Tech Architect: obfuscation configs drawn and explained",
            &css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
