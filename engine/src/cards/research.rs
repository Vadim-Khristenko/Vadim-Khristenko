//! Active research — Aethelgard TQ-1.58 HVRL, with flipping ternary weight
//! cells. Every state is legible at rest; the flicker only recolours.

use crate::run::Ctx;
use crate::svg::{esc, fit_text};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

/// A lattice of {-1, 0, +1} cells that recolour — a live 1.58-bit weight tile.
fn weight_grid(x: u32, y: u32, cols: u32, rows: u32) -> String {
    let cell = 22;
    let states = ["−1", "0", "+1"];
    let color = |s: &str| match s {
        "−1" => t::RED,
        "+1" => t::GREEN,
        _ => t::MUTED,
    };
    let mut out = String::new();
    let mut k = 0u32;
    for r in 0..rows {
        for c in 0..cols {
            let cx = x + c * cell;
            let cy = y + r * cell;
            let s0 = states[((r + c) % 3) as usize];
            let s1 = states[((r + c + 1) % 3) as usize];
            let s2 = states[((r + c + 2) % 3) as usize];
            let dur = 3.0 + (k % 5) as f64 * 0.7;
            let begin = (k % 8) as f64 * 0.3;
            out.push_str(&format!(
                r#"<g transform="translate({cx},{cy})"><rect x="1" y="-13" width="{rw}" height="{rh}" rx="3" fill="{bgd}" stroke="{bghl}" stroke-width="0.6"/><text x="{tx}" y="2" text-anchor="middle" font-family="{mono}" font-size="10" fill="{c0}">{s0}<animate attributeName="fill" values="{c0};{c1};{c2};{c0}" dur="{dur}s" begin="{begin}s" repeatCount="indefinite"/></text></g>"#,
                rw = cell - 3,
                rh = cell - 3,
                bgd = t::BG_DARK,
                bghl = t::BG_HL,
                tx = (cell - 2) / 2,
                mono = t::MONO,
                c0 = color(s0),
                c1 = color(s1),
                c2 = color(s2),
            ));
            k += 1;
        }
    }
    out
}

/// Greedy two-line word wrap for the blurb (the prose must never run under
/// the weight lattice on the right).
fn two_lines(text: &str, max_chars: usize) -> (String, String) {
    let mut line1 = String::new();
    let mut line2 = String::new();
    for word in text.split_whitespace() {
        if line2.is_empty() && line1.len() + word.len() + 1 <= max_chars {
            if !line1.is_empty() {
                line1.push(' ');
            }
            line1.push_str(word);
        } else {
            if !line2.is_empty() {
                line2.push(' ');
            }
            line2.push_str(word);
        }
    }
    (line1, line2)
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let w = t::CARD_W;
    let h = 256;
    let r = &ctx.cfg.profile.research;

    // Tag chips flow left→right and STOP before the weight lattice; a "+N"
    // marker owns whatever doesn't fit instead of running under the grid.
    let chips_end = (w - 245 - 16) as f64;
    let mut chips = String::new();
    let mut cx = 40f64;
    let cy = 196;
    for (i, tag) in r.tags.iter().enumerate() {
        let tw = 20.0 + crate::svg::text_width_px(tag, 12.0, true);
        if cx + tw > chips_end {
            let left = r.tags.len() - i;
            chips.push_str(&format!(
                r#"<text x="{cx:.0}" y="{cy}" font-family="{mono}" font-size="12" fill="{muted}">+{left} more</text>"#,
                mono = t::MONO,
                muted = t::MUTED,
            ));
            break;
        }
        chips.push_str(&format!(
            r#"<rect x="{cx:.0}" y="{ry}" width="{tw:.0}" height="24" rx="12" fill="{bghl}" stroke="{purple}" stroke-width="0.8" opacity="0.9"/><text x="{tx:.0}" y="{cy}" text-anchor="middle" font-family="{mono}" font-size="12" fill="{fgd}">{tag}</text>"#,
            ry = cy - 16,
            bghl = t::BG_HL,
            purple = t::PURPLE,
            tx = cx + tw / 2.0,
            mono = t::MONO,
            fgd = t::FG_DIM,
            tag = esc(tag),
        ));
        cx += tw + 12.0;
    }

    let (line1, line2) = two_lines(&r.blurb, 64);
    let inner = format!(
        r#"
    <text x="40" y="44" font-family="{mono}" font-size="12" fill="{red}" letter-spacing="2">● ACTIVE RESEARCH · low-bit ML</text>
    <text x="40" y="84" font-family="{mono}" font-size="29" font-weight="800" fill="{purple}" filter="url(#softglow)">{name}</text>
    <text x="40" y="108" font-family="{mono}" font-size="14" fill="{cyan}">{subtitle}</text>
    <text x="40" y="132" font-family="{sans}" font-size="13" fill="{fgd}">{line1}</text>
    <text x="40" y="150" font-family="{sans}" font-size="13" fill="{fgd}">{line2}</text>
    <line x1="40" y1="172" x2="700" y2="172" stroke="{bghl}" stroke-width="1" opacity="0.7"/>
    {chips}
    <text x="{wx}" y="36" font-family="{mono}" font-size="11" fill="{muted}">weights ∈ {{-1, 0, +1}}</text>
    {grid}
    "#,
        mono = t::MONO,
        sans = t::SANS,
        red = t::RED,
        purple = t::PURPLE,
        cyan = t::CYAN,
        fgd = t::FG_DIM,
        muted = t::MUTED,
        bghl = t::BG_HL,
        name = esc(&fit_text(&r.name, (w - 245 - 56) as f64, 29.0, true)),
        subtitle = esc(&fit_text(&r.subtitle, (w - 245 - 56) as f64, 14.0, true)),
        line1 = esc(&fit_text(&line1, (w - 245 - 56) as f64, 13.0, false)),
        line2 = esc(&fit_text(&line2, (w - 245 - 56) as f64, 13.0, false)),
        wx = w - 245,
        grid = weight_grid(w - 245, 56, 10, 5),
    );

    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/research/aethelgard.rs",
            badge: "TQ-1.58 HVRL",
            accent: t::PURPLE,
            texture: Texture::Ternary,
            alt: "Active research: Aethelgard TQ-1.58 HVRL, a low-bit agentic reasoning architecture",
        },
        &inner,
    );
    Ok(vec![("research.svg".into(), svg)])
}
