//! Header — the hero terminal pane: name, rotating aliases, Rust-flavoured
//! tagline, "known for" chips fed by the flagship config.

use crate::run::Ctx;
use crate::svg::esc;
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

/// Cycle the aliases with crossfades. alias[0] is fully visible at rest (t=0)
/// so the line never renders blank in a static first frame.
fn alias_rotator(aliases: &[String], x: u32, y: u32) -> String {
    let n = aliases.len();
    let loop_s = n as f64 * 2.4;
    let cf = 0.03;
    let mut nodes = String::new();
    for (i, alias) in aliases.iter().enumerate() {
        let s = i as f64 / n as f64;
        let e = (i + 1) as f64 / n as f64;
        let (kt, kv): (Vec<f64>, Vec<u8>) = if i == 0 {
            // visible at the very start AND wrap-around end of the loop
            (vec![0.0, e - cf, e, 1.0 - cf, 1.0], vec![1, 1, 0, 0, 1])
        } else {
            (
                vec![0.0, s - cf, s, e - cf, e, 1.0],
                vec![0, 0, 1, 1, 0, 0],
            )
        };
        let kt: Vec<String> = kt
            .iter()
            .map(|v| format!("{:.4}", v.clamp(0.0, 1.0)))
            .collect();
        let kv_s: Vec<String> = kv.iter().map(|v| v.to_string()).collect();
        nodes.push_str(&format!(
            r#"<text x="{x}" y="{y}" font-family="{mono}" font-size="27" font-weight="700" fill="{col}" opacity="{op}" filter="url(#glow)">{alias}<animate attributeName="opacity" values="{values}" keyTimes="{keytimes}" dur="{loop_s}s" repeatCount="indefinite"/></text>"#,
            mono = t::MONO,
            col = t::ORANGE,
            op = kv[0],
            alias = esc(alias),
            values = kv_s.join(";"),
            keytimes = kt.join(";"),
        ));
    }
    nodes
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let w = t::CARD_W;
    let h = 300;
    let name = &ctx.cfg.profile.name;
    // Underline spans the full rendered name (≈ monospace advance × chars).
    // Static-first: full width at rest, with a soft shimmer instead of a grow.
    let underline_w = (name.chars().count() as f64 * 25.6).round() as u32;

    let chip = |y: u32, dot: &str, label: &str| {
        format!(
            r#"<circle cx="724" cy="{cy}" r="4" fill="{dot}"><animate attributeName="opacity" values="1;0.4;1" dur="2.4s" repeatCount="indefinite"/></circle><text x="740" y="{y}" font-family="{mono}" font-size="14" fill="{fgd}">{label}</text>"#,
            cy = y - 5,
            mono = t::MONO,
            fgd = t::FG_DIM,
            label = esc(label),
        )
    };

    // "Known for" chips: the first three flagship names + the evergreen one.
    let mut known: Vec<String> = ctx
        .cfg
        .flagship
        .project
        .iter()
        .take(3)
        .map(|p| p.name.clone())
        .collect();
    known.push("production bots & fleets".into());
    let chip_colors = [t::RED, t::ORANGE, t::PURPLE, t::GREEN];
    let chips: String = known
        .iter()
        .zip(chip_colors.iter())
        .enumerate()
        .map(|(i, (label, col))| chip(86 + i as u32 * 30, col, label))
        .collect();

    let m = t::MARGIN;
    let inner = format!(
        r#"
    <text x="{m}" y="78" font-family="{mono}" font-size="44" font-weight="800" fill="{fg}" filter="url(#softglow)">{name}</text>
    <rect x="{m2}" y="96" width="{underline_w}" height="3" rx="1.5" fill="{rust}">
      <animate attributeName="opacity" values="1;0.75;1" dur="4s" repeatCount="indefinite"/>
    </rect>
    <text x="{m}" y="143" font-family="{mono}" font-size="18" fill="{muted}">aka&gt;</text>
    <g transform="translate(64,0)">{rotator}</g>
    <text x="{m}" y="185" font-family="{mono}" font-size="15" fill="{muted}">// backend architect · systems engineer · chaos engineer by hobby</text>
    <text x="{m}" y="217" font-family="{mono}" font-size="15" fill="{purple}">let</text>
    <text x="{mx34}" y="217" font-family="{mono}" font-size="15" fill="{fg}">primary</text>
    <text x="{mx112}" y="217" font-family="{mono}" font-size="15" fill="{muted}">=</text>
    <text x="{mx130}" y="217" font-family="{mono}" font-size="15" fill="{cyan}">Language</text>
    <text x="{mx208}" y="217" font-family="{mono}" font-size="15" fill="{muted}">::</text>
    <text x="{mx226}" y="217" font-family="{mono}" font-size="15" fill="{orange}">Rust</text>
    <text x="{mx265}" y="217" font-family="{mono}" font-size="15" fill="{fg}">;</text>
    <text x="{mx282}" y="217" font-family="{mono}" font-size="15">🦀</text>
    <text x="{mx350}" y="217" font-family="{mono}" font-size="13" fill="{muted}">// portfolio → vai-rice.space</text>
    <text x="724" y="56" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="2">// KNOWN FOR</text>
    {chips}
    <line x1="704" y1="40" x2="704" y2="196" stroke="{bghl}" stroke-width="1"/>
    "#,
        mono = t::MONO,
        fg = t::FG,
        muted = t::MUTED,
        purple = t::PURPLE,
        cyan = t::CYAN,
        orange = t::ORANGE,
        rust = t::RUST,
        bghl = t::BG_HL,
        name = esc(name),
        m2 = m + 2,
        mx34 = m + 34,
        mx112 = m + 112,
        mx130 = m + 130,
        mx208 = m + 208,
        mx226 = m + 226,
        mx265 = m + 265,
        mx282 = m + 282,
        mx350 = m + 350,
        rotator = alias_rotator(&ctx.cfg.profile.aliases, m, 143),
    );

    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "vadim@vai-rice:~$ whoami",
            badge: "online",
            accent: t::RUST,
            texture: Texture::Grid,
            alt: "Vadim Khristenko — backend architect and systems engineer",
        },
        &inner,
    );
    Ok(vec![("header.svg".into(), svg)])
}
