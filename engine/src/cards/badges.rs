//! Badge row above the header: 3D keycaps with a legend in the corner, pressed
//! one after another as if someone is typing. Words come from `[[badges]]`.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::{bail, Result};
use std::fmt::Write;

const H: f64 = 92.0;
const TOP: f64 = 18.0;
const CAPH: f64 = 44.0;
const SIDE: f64 = 8.0;
const GAP: f64 = 10.0;
const LOOP: f64 = 7.0;

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let keys = &ctx.cfg.profile.badges;
    if keys.is_empty() {
        bail!("no [[badges]] in profile.toml");
    }
    let mut doc = Doc::new();
    let label_t = T::new(Font::MonoBold, 13.0);
    let widths: Vec<f64> = keys
        .iter()
        .map(|k| (typo::width(&k.label, &label_t) + 30.0).max(96.0))
        .collect();
    let total: f64 = widths.iter().sum::<f64>() + GAP * (keys.len() as f64 - 1.0);
    let scale = if total > kit::W - 20.0 {
        (kit::W - 20.0) / total
    } else {
        1.0
    };
    let step = 90.0 / keys.len() as f64;
    let mut css = String::new();
    for i in 0..keys.len() {
        let a = i as f64 * step;
        let _ = write!(
            css,
            ".k{i}{{animation:k{i} {LOOP}s ease-in-out infinite}}@keyframes k{i}{{0%,{a:.0}%,{e:.0}%,100%{{transform:translateY(0)}}{b:.0}%,{c:.0}%{{transform:translateY({p}px)}}}}",
            e = a + step * 0.9,
            b = a + step * 0.35,
            c = a + step * 0.55,
            p = SIDE - 2.0
        );
    }
    let mut body = format!(
        r#"<g transform="translate({} 0) scale({scale:.4})">"#,
        (kit::W - total * scale) / 2.0
    );
    let mut x = 0.0;
    for (i, (k, w)) in keys.iter().zip(&widths).enumerate() {
        let ink = k.ink.clone().unwrap_or_else(|| pal::BG.to_string());
        let col = &k.color;
        let _ = write!(
            body,
            r##"<rect x="{x}" y="{y}" width="{w}" height="{CAPH}" rx="9" fill="#05060d" opacity=".55"/><rect x="{x}" y="{s}" width="{w}" height="{CAPH}" rx="9" fill="{col}"/><rect x="{x}" y="{s}" width="{w}" height="{CAPH}" rx="9" fill="url(#shade)"/><g class="k{i}"><rect x="{x}" y="{TOP}" width="{w}" height="{CAPH}" rx="9" fill="{col}"/><rect x="{gx}" y="{gy}" width="{gw}" height="{gh}" rx="7" fill="url(#gloss)"/><rect x="{ox}" y="{oy}" width="{ow}" height="{oh}" rx="8.5" fill="none" stroke="#ffffff" stroke-opacity=".25"/>"##,
            y = TOP + SIDE + 4.0,
            s = TOP + SIDE,
            gx = x + 4.0,
            gy = TOP + 3.0,
            gw = w - 8.0,
            gh = CAPH - 10.0,
            ox = x + 0.5,
            oy = TOP + 0.5,
            ow = w - 1.0,
            oh = CAPH - 1.0,
        );
        body.push_str(&doc.text(
            x + 9.0,
            TOP + 13.0,
            &k.legend,
            &T::new(Font::Mono, 8.0).fill(&ink).op(0.7),
        ));
        body.push_str(&doc.text(
            x + w / 2.0,
            TOP + 33.0,
            &k.label,
            &label_t.clone().fill(&ink).mid(),
        ));
        body.push_str("</g>");
        x += w + GAP;
    }
    body.push_str("</g>");
    let defs = r##"<linearGradient id="gloss" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#ffffff" stop-opacity=".35"/><stop offset=".5" stop-color="#ffffff" stop-opacity="0"/></linearGradient><linearGradient id="shade" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#000" stop-opacity=".1"/><stop offset="1" stop-color="#000" stop-opacity=".45"/></linearGradient>"##;
    let alt: Vec<&str> = keys.iter().map(|k| k.label.as_str()).collect();
    let svg = kit::svg(
        kit::W,
        H,
        &format!("Badges: {}", alt.join(", ")),
        &css,
        defs,
        &doc,
        &body,
    );
    Ok(vec![("badges.svg".into(), svg)])
}
