//! Section dividers — a small family of DISTINCT separators, used variably
//! between README sections:
//!   divider.svg          ternary stream → hex conduit (the original)
//!   divider_wave.svg     layered waveform
//!   divider_circuit.svg  circuit trace with junction nodes
//!   divider_pulse.svg    ECG pulse-line
//! All are 1000×58 and legible as static art; animation only adds drift.

use crate::run::Ctx;
use crate::theme as t;
use anyhow::Result;
use std::fmt::Write;

const W: u32 = t::CARD_W;
const H: u32 = 58;

fn shell(id: &str, defs: &str, body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" role="img" aria-label="Section divider">
  <title>Section divider</title>
  <defs>
    <linearGradient id="trace_{id}" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{bg}" stop-opacity="0"/>
      <stop offset="0.2" stop-color="{rust}"/>
      <stop offset="0.5" stop-color="{purple}"/>
      <stop offset="0.8" stop-color="{blue}"/>
      <stop offset="1" stop-color="{bg}" stop-opacity="0"/>
    </linearGradient>
    <filter id="dg_{id}" x="-30%" y="-300%" width="160%" height="700%">
      <feGaussianBlur stdDeviation="1.5" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    {defs}
  </defs>
  {body}
</svg>"#,
        bg = t::BG,
        rust = t::RUST,
        purple = t::PURPLE,
        blue = t::BLUE,
    )
}

fn hexagon(cx: f64, cy: f64, r: f64) -> String {
    (0..6)
        .map(|k| {
            let ang = (60.0 * k as f64 - 30.0).to_radians();
            format!("{:.1},{:.1}", cx + r * ang.cos(), cy + r * ang.sin())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The original: ternary bits streaming along a neon conduit into a hex node.
fn ternary_stream() -> String {
    let mid = H / 2;
    let cx = W / 2;
    let glyphs = ["+1", "0", "−1", "1", "0", "+1", "−1", "0"];
    let mut bits = String::new();
    for i in 0..10u32 {
        let gx = 90 + i * 70;
        let side: i32 = if gx < cx { -1 } else { 1 };
        let g = glyphs[i as usize % glyphs.len()];
        let col = if g.starts_with('+') {
            t::GREEN
        } else if g.starts_with('−') {
            t::RED
        } else {
            t::MUTED
        };
        write!(
            bits,
            r#"<text x="{gx}" y="{ty}" font-family="{mono}" font-size="11" fill="{col}" opacity="0.35" text-anchor="middle">{g}<animate attributeName="opacity" values="0.2;0.85;0.2" dur="3.2s" begin="{b:.2}s" repeatCount="indefinite"/><animate attributeName="x" values="{gx};{gx2}" dur="3.2s" begin="{b:.2}s" repeatCount="indefinite"/></text>"#,
            ty = mid + 4,
            mono = t::MONO,
            b = i as f64 * 0.32,
            gx2 = gx as i32 + side * 22,
        )
        .unwrap();
    }
    let body = format!(
        r#"
  <line x1="70" y1="{mid}" x2="{xe}" y2="{mid}" stroke="url(#trace_ts)" stroke-width="1.4" filter="url(#dg_ts)"/>
  <line x1="70" y1="{mid}" x2="{xe}" y2="{mid}" stroke="{purple}" stroke-width="1.4" stroke-dasharray="2 10" opacity="0.5">
    <animate attributeName="stroke-dashoffset" values="0;-48" dur="2.4s" repeatCount="indefinite"/>
  </line>
  {bits}
  <circle cx="70" cy="{mid}" r="3" fill="{cyan}" filter="url(#dg_ts)">
    <animate attributeName="cx" values="70;{lc}" dur="2.6s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0;1;0" dur="2.6s" repeatCount="indefinite"/>
  </circle>
  <circle cx="{xe}" cy="{mid}" r="3" fill="{rust}" filter="url(#dg_ts)">
    <animate attributeName="cx" values="{xe};{rc}" dur="2.6s" begin="1.3s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0;1;0" dur="2.6s" begin="1.3s" repeatCount="indefinite"/>
  </circle>
  <g transform="translate({cx},{mid})">
    <polygon points="{hex_out}" fill="none" stroke="{purple}" stroke-width="1" opacity="0.5">
      <animateTransform attributeName="transform" type="rotate" from="0" to="360" dur="14s" repeatCount="indefinite"/>
    </polygon>
    <polygon points="{hex_in}" fill="{bgd}" stroke="{rust}" stroke-width="1.6" filter="url(#dg_ts)"/>
    <circle r="2.4" fill="{cyan}"><animate attributeName="r" values="1.6;3;1.6" dur="2s" repeatCount="indefinite"/>
      <animate attributeName="opacity" values="0.5;1;0.5" dur="2s" repeatCount="indefinite"/></circle>
  </g>"#,
        xe = W - 70,
        purple = t::PURPLE,
        cyan = t::CYAN,
        rust = t::RUST,
        bgd = t::BG_DARK,
        lc = cx - 40,
        rc = cx + 40,
        hex_out = hexagon(0.0, 0.0, 15.0),
        hex_in = hexagon(0.0, 0.0, 9.0),
    );
    shell("ts", "", &body)
}

/// Layered waveform: two phase-shifted sines + a slow drifting dash shimmer.
fn waveform() -> String {
    let mid = H as f64 / 2.0;
    let sine_path = |amp: f64, period: f64, phase: f64| -> String {
        let mut d = String::new();
        let mut x = 70.0;
        write!(d, "M{x:.1},{:.1}", mid + amp * (x / period * std::f64::consts::TAU + phase).sin())
            .unwrap();
        while x < (W - 70) as f64 {
            x += 8.0;
            write!(d, " L{x:.1},{:.1}", mid + amp * (x / period * std::f64::consts::TAU + phase).sin())
                .unwrap();
        }
        d
    };
    let body = format!(
        r#"
  <path d="{p1}" fill="none" stroke="url(#trace_wv)" stroke-width="1.6" filter="url(#dg_wv)"/>
  <path d="{p2}" fill="none" stroke="{cyan}" stroke-width="1" opacity="0.45"/>
  <path d="{p1}" fill="none" stroke="{fg}" stroke-width="1.6" stroke-dasharray="12 130" opacity="0.65">
    <animate attributeName="stroke-dashoffset" values="0;-142" dur="4.5s" repeatCount="indefinite"/>
  </path>
  <circle cx="70" cy="{mid:.0}" r="2.6" fill="{rust}"/>
  <circle cx="{xe}" cy="{mid:.0}" r="2.6" fill="{blue}"/>"#,
        p1 = sine_path(11.0, 160.0, 0.0),
        p2 = sine_path(6.0, 90.0, 1.6),
        cyan = t::CYAN,
        fg = t::FG,
        rust = t::RUST,
        blue = t::BLUE,
        xe = W - 70,
    );
    shell("wv", "", &body)
}

/// Circuit trace: a horizontal run with right-angle jogs, via nodes and one
/// packet dot travelling the path.
fn circuit_trace() -> String {
    let y0 = 36.0;
    let y1 = 22.0;
    let path = format!(
        "M70,{y0} H260 V{y1} H430 V{y0} H620 V{y1} H790 V{y0} H{}",
        W - 70
    );
    let mut nodes = String::new();
    for (x, y, col) in [
        (260.0, y0, t::RUST),
        (260.0, y1, t::RUST),
        (430.0, y1, t::PURPLE),
        (430.0, y0, t::PURPLE),
        (620.0, y0, t::CYAN),
        (620.0, y1, t::CYAN),
        (790.0, y1, t::BLUE),
        (790.0, y0, t::BLUE),
    ] {
        write!(
            nodes,
            r#"<rect x="{:.1}" y="{:.1}" width="5" height="5" rx="1" fill="{bgd}" stroke="{col}" stroke-width="1.1"/>"#,
            x - 2.5,
            y - 2.5,
            bgd = t::BG_DARK,
        )
        .unwrap();
    }
    let body = format!(
        r#"
  <path d="{path}" fill="none" stroke="url(#trace_cc)" stroke-width="1.5" filter="url(#dg_cc)"/>
  <path d="{path}" fill="none" stroke="{bghl}" stroke-width="0.6" opacity="0.8"/>
  {nodes}
  <circle cx="70" cy="{y0}" r="3.2" fill="{rust}" filter="url(#dg_cc)"/>
  <circle cx="{xe}" cy="{y0}" r="3.2" fill="{blue}" filter="url(#dg_cc)"/>
  <circle r="2.6" fill="{green}" filter="url(#dg_cc)">
    <animateMotion dur="7s" repeatCount="indefinite" path="{path}"/>
    <animate attributeName="opacity" values="0;1;1;1;0" keyTimes="0;0.06;0.5;0.94;1" dur="7s" repeatCount="indefinite"/>
  </circle>"#,
        bghl = t::BG_HL,
        rust = t::RUST,
        blue = t::BLUE,
        green = t::GREEN,
        xe = W - 70,
    );
    shell("cc", "", &body)
}

/// ECG pulse-line: flat baseline with a spike cluster mid-line.
fn pulse_line() -> String {
    let mid = H as f64 / 2.0;
    let cx = W as f64 / 2.0;
    let path = format!(
        "M70,{mid:.0} H{a:.0} L{b:.0},{up:.0} L{c:.0},{dn:.0} L{d:.0},{mid2:.0} L{e:.0},{mid:.0} H{xe}",
        a = cx - 60.0,
        b = cx - 34.0,
        up = mid - 16.0,
        c = cx - 8.0,
        dn = mid + 18.0,
        d = cx + 16.0,
        mid2 = mid - 6.0,
        e = cx + 40.0,
        xe = W - 70,
    );
    let body = format!(
        r#"
  <path d="{path}" fill="none" stroke="url(#trace_pl)" stroke-width="1.6" filter="url(#dg_pl)"/>
  <path d="{path}" fill="none" stroke="{red}" stroke-width="1.6" stroke-dasharray="26 900" opacity="0.9">
    <animate attributeName="stroke-dashoffset" values="926;0" dur="3.6s" repeatCount="indefinite"/>
  </path>
  <circle cx="{cx:.0}" cy="{mid:.0}" r="10" fill="none" stroke="{red}" stroke-width="1" opacity="0.35">
    <animate attributeName="r" values="6;14" dur="2.4s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0.5;0" dur="2.4s" repeatCount="indefinite"/>
  </circle>
  <circle cx="70" cy="{mid:.0}" r="2.6" fill="{rust}"/>
  <circle cx="{xe}" cy="{mid:.0}" r="2.6" fill="{blue}"/>"#,
        red = t::RED,
        rust = t::RUST,
        blue = t::BLUE,
        xe = W - 70,
    );
    shell("pl", "", &body)
}

pub fn build(_ctx: &Ctx) -> Result<Vec<(String, String)>> {
    Ok(vec![
        ("divider.svg".into(), ternary_stream()),
        ("divider_wave.svg".into(), waveform()),
        ("divider_circuit.svg".into(), circuit_trace()),
        ("divider_pulse.svg".into(), pulse_line()),
    ])
}
