//! Section dividers, v2 — each one is a tiny "web page rendered inside an
//! SVG": a self-contained mini panel that could be a real UI strip.
//!
//!   divider.svg          a slim browser window streaming ternary bits
//!   divider_wave.svg     an audio-player bar with a live waveform
//!   divider_circuit.svg  a PCB inspection strip with a packet on the trace
//!   divider_pulse.svg    a terminal readout with prompt + heartbeat trace
//!   divider_editor.svg   one line of a Rust file in a code editor
//!
//! All are 1000×64, Tokyo-Night, legible as static art; animation only adds
//! drift. README sections rotate between them so no two neighbours match.

use crate::run::Ctx;
use crate::theme as t;
use anyhow::Result;
use std::fmt::Write;

const W: u32 = t::CARD_W;
const H: u32 = 64;

/// Shared shell: a floating mini-panel (the "web page") + per-divider defs.
fn shell(id: &str, label: &str, defs: &str, body: &str) -> String {
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" role="img" aria-label="{label}">
  <title>{label}</title>
  <defs>
    <linearGradient id="panel_{id}" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="{panel}"/>
      <stop offset="1" stop-color="{bgd}"/>
    </linearGradient>
    <linearGradient id="trace_{id}" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0" stop-color="{rust}"/>
      <stop offset="0.5" stop-color="{purple}"/>
      <stop offset="1" stop-color="{blue}"/>
    </linearGradient>
    <filter id="dg_{id}" x="-30%" y="-300%" width="160%" height="700%">
      <feGaussianBlur stdDeviation="1.4" result="b"/><feMerge><feMergeNode in="b"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
    {defs}
  </defs>
  <rect x="8" y="4" width="{pw}" height="{ph}" rx="10" fill="url(#panel_{id})"/>
  <rect x="8.5" y="4.5" width="{pw2}" height="{ph2}" rx="9.5" fill="none" stroke="{bghl}" stroke-width="1"/>
  <path d="M8 16 Q8 4 20 4" fill="none" stroke="{rust}" stroke-width="2" opacity="0.85"/>
  {body}
</svg>"#,
        panel = t::BG_PANEL,
        bgd = t::BG_DARK,
        bghl = t::BG_HL,
        rust = t::RUST,
        purple = t::PURPLE,
        blue = t::BLUE,
        pw = W - 16,
        ph = H - 8,
        pw2 = W - 17,
        ph2 = H - 9,
    )
}

/// divider.svg — a slim BROWSER: traffic dots, a URL pill, and a content lane
/// where ternary bits stream toward the right edge.
fn browser() -> String {
    let mut body = String::new();
    // Chrome row: dots + URL pill + window controls.
    for (i, c) in [t::RED, t::YELLOW, t::GREEN].iter().enumerate() {
        write!(body, r#"<circle cx="{}" cy="20" r="4" fill="{c}" opacity="0.9"/>"#, 30 + i * 16).unwrap();
    }
    write!(
        body,
        r#"<rect x="86" y="10" width="330" height="20" rx="10" fill="{bgd}" stroke="{bghl}" stroke-width="1"/>
  <circle cx="102" cy="20" r="3" fill="{green}"><animate attributeName="opacity" values="1;0.4;1" dur="2.4s" repeatCount="indefinite"/></circle>
  <text x="114" y="24" font-family="{mono}" font-size="11" fill="{muted}">vai-rice.space/stream/ternary</text>
  <text x="{re}" y="24" text-anchor="end" font-family="{mono}" font-size="10" fill="{comment}">— ▢ ✕</text>"#,
        bgd = t::BG_DARK,
        bghl = t::BG_HL,
        green = t::GREEN,
        muted = t::MUTED,
        comment = t::COMMENT,
        mono = t::MONO,
        re = W - 30,
    )
    .unwrap();
    // Content lane: a conduit with streaming ternary bits.
    write!(
        body,
        r#"<line x1="30" y1="46" x2="{xe}" y2="46" stroke="url(#trace_br)" stroke-width="1.2" opacity="0.8" filter="url(#dg_br)"/>
  <line x1="30" y1="46" x2="{xe}" y2="46" stroke="{purple}" stroke-width="1.2" stroke-dasharray="2 10" opacity="0.5">
    <animate attributeName="stroke-dashoffset" values="0;-48" dur="2.4s" repeatCount="indefinite"/>
  </line>"#,
        purple = t::PURPLE,
        xe = W - 30,
    )
    .unwrap();
    let glyphs = ["+1", "0", "−1", "1", "0", "+1", "−1", "0"];
    for i in 0..11u32 {
        let gx = 70 + i * 86;
        let g = glyphs[i as usize % glyphs.len()];
        let col = if g.starts_with('+') {
            t::GREEN
        } else if g.starts_with('−') {
            t::RED
        } else {
            t::MUTED
        };
        write!(
            body,
            r#"<text x="{gx}" y="50" font-family="{mono}" font-size="10.5" fill="{col}" opacity="0.55" text-anchor="middle">{g}<animate attributeName="opacity" values="0.3;0.95;0.3" dur="3.2s" begin="{b:.2}s" repeatCount="indefinite"/><animate attributeName="x" values="{gx};{gx2}" dur="3.2s" begin="{b:.2}s" repeatCount="indefinite"/></text>"#,
            mono = t::MONO,
            b = i as f64 * 0.29,
            gx2 = gx + 24,
        )
        .unwrap();
    }
    shell("br", "Section divider: browser strip", "", &body)
}

/// divider_wave.svg — an AUDIO PLAYER bar: play button, live waveform,
/// track name and a 1:58 timestamp (of course it's 1:58).
fn player() -> String {
    let mut body = format!(
        r#"<circle cx="38" cy="32" r="13" fill="none" stroke="{rust}" stroke-width="1.6"/>
  <circle cx="38" cy="32" r="13" fill="none" stroke="{rust}" stroke-width="1.6" opacity="0.35" filter="url(#dg_wv)">
    <animate attributeName="r" values="13;16;13" dur="3s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0.35;0;0.35" dur="3s" repeatCount="indefinite"/>
  </circle>
  <path d="M34 26 L46 32 L34 38 Z" fill="{rust}"/>
  <text x="66" y="27" font-family="{mono}" font-size="11.5" fill="{fg}">tokyo-night.flac</text>
  <text x="66" y="43" font-family="{mono}" font-size="10" fill="{muted}">VAI · profile OST</text>"#,
        rust = t::RUST,
        fg = t::FG,
        muted = t::MUTED,
        mono = t::MONO,
    );
    // Waveform: centred bars, sine-seeded heights, gentle staggered breathing.
    let (x0, x1) = (250.0, W as f64 - 190.0);
    let n = 46;
    let step = (x1 - x0) / n as f64;
    for i in 0..n {
        let x = x0 + i as f64 * step;
        let ph = (i as f64 * 0.55).sin() * 0.5 + 0.5; // 0..1
        let hh = 4.0 + ph * 12.0;
        let col = if i % 7 == 3 { t::CYAN } else { t::BLUE };
        write!(
            body,
            r#"<rect x="{x:.1}" y="{y:.1}" width="3.4" height="{h:.1}" rx="1.7" fill="{col}" opacity="0.85"><animateTransform attributeName="transform" type="scale" values="1 1;1 {s:.2};1 1" dur="{d:.1}s" begin="{b:.2}s" repeatCount="indefinite" additive="sum"/></rect>"#,
            y = 32.0 - hh / 2.0,
            h = hh,
            s = 1.0 + 0.5 * ((i % 5) as f64 / 5.0),
            d = 1.6 + (i % 4) as f64 * 0.3,
            b = i as f64 * 0.07,
        )
        .unwrap();
    }
    // Progress + time.
    write!(
        body,
        r#"<rect x="250" y="52" width="{pw:.0}" height="2.4" rx="1.2" fill="{bghl}"/>
  <rect x="250" y="52" width="{fw:.0}" height="2.4" rx="1.2" fill="url(#trace_wv)"/>
  <circle cx="{cx:.0}" cy="53.2" r="3" fill="{cyan}" filter="url(#dg_wv)"/>
  <text x="{te}" y="28" text-anchor="end" font-family="{mono}" font-size="11.5" fill="{fg}">1:58</text>
  <text x="{te}" y="43" text-anchor="end" font-family="{mono}" font-size="10" fill="{muted}">/ 3:41</text>"#,
        bghl = t::BG_HL,
        cyan = t::CYAN,
        fg = t::FG,
        muted = t::MUTED,
        mono = t::MONO,
        pw = x1 - 250.0,
        fw = (x1 - 250.0) * 0.53,
        cx = 250.0 + (x1 - 250.0) * 0.53,
        te = W - 30,
    )
    .unwrap();
    shell("wv", "Section divider: audio player", "", &body)
}

/// divider_circuit.svg — a PCB strip: silkscreen label, an IC package, traces
/// with vias and one packet riding the bus.
fn pcb() -> String {
    let (y0, y1) = (42.0, 24.0);
    let path = format!("M30,{y0} H240 V{y1} H420 V{y0} H588 V{y1} H780 V{y0} H{}", W - 30);
    let mut nodes = String::new();
    for (x, y, col) in [
        (240.0, y0, t::RUST),
        (240.0, y1, t::RUST),
        (420.0, y1, t::PURPLE),
        (420.0, y0, t::PURPLE),
        (588.0, y0, t::CYAN),
        (588.0, y1, t::CYAN),
        (780.0, y1, t::BLUE),
        (780.0, y0, t::BLUE),
    ] {
        write!(
            nodes,
            r#"<circle cx="{x:.0}" cy="{y:.0}" r="3" fill="{bgd}" stroke="{col}" stroke-width="1.2"/>"#,
            bgd = t::BG_DARK,
        )
        .unwrap();
    }
    // IC package mid-board with pins.
    let icx = 486.0;
    let mut ic = format!(
        r#"<rect x="{x:.0}" y="20" width="56" height="24" rx="3" fill="{bgd}" stroke="{rust}" stroke-width="1.3"/><text x="{tx:.0}" y="36" text-anchor="middle" font-family="{mono}" font-size="9.5" fill="{rust}" letter-spacing="1">TQ-1.58</text><circle cx="{px:.0}" cy="25" r="1.6" fill="{muted}"/>"#,
        x = icx - 28.0,
        tx = icx,
        px = icx - 21.0,
        bgd = t::BG_DARK,
        rust = t::RUST,
        muted = t::MUTED,
        mono = t::MONO,
    );
    for k in 0..5 {
        let px = icx - 22.0 + k as f64 * 11.0;
        write!(
            ic,
            r#"<line x1="{px:.0}" y1="14" x2="{px:.0}" y2="20" stroke="{muted}" stroke-width="1.4"/><line x1="{px:.0}" y1="44" x2="{px:.0}" y2="50" stroke="{muted}" stroke-width="1.4"/>"#,
            muted = t::COMMENT,
        )
        .unwrap();
    }
    let body = format!(
        r#"
  <text x="30" y="16" font-family="{mono}" font-size="9.5" fill="{comment}" letter-spacing="2">VAI·PCB rev3 — 1.58-bit bus</text>
  <path d="{path}" fill="none" stroke="url(#trace_cc)" stroke-width="1.5" filter="url(#dg_cc)"/>
  <path d="{path}" fill="none" stroke="{bghl}" stroke-width="0.6" opacity="0.8"/>
  {nodes}
  {ic}
  <circle cx="30" cy="{y0}" r="3.2" fill="{rust}" filter="url(#dg_cc)"/>
  <circle cx="{xe}" cy="{y0}" r="3.2" fill="{blue}" filter="url(#dg_cc)"/>
  <circle r="2.6" fill="{green}" filter="url(#dg_cc)">
    <animateMotion dur="7s" repeatCount="indefinite" path="{path}"/>
    <animate attributeName="opacity" values="0;1;1;1;0" keyTimes="0;0.06;0.5;0.94;1" dur="7s" repeatCount="indefinite"/>
  </circle>"#,
        mono = t::MONO,
        comment = t::COMMENT,
        bghl = t::BG_HL,
        rust = t::RUST,
        blue = t::BLUE,
        green = t::GREEN,
        xe = W - 30,
    );
    shell("cc", "Section divider: circuit board", "", &body)
}

/// divider_pulse.svg — a TERMINAL readout: prompt + command + blinking block
/// cursor on the left, a heartbeat trace with live status on the right.
fn terminal() -> String {
    let mid = 40.0;
    let cx = 740.0;
    let path = format!(
        "M560,{mid:.0} H{a:.0} L{b:.0},{up:.0} L{c:.0},{dn:.0} L{d:.0},{m2:.0} L{e:.0},{mid:.0} H{xe}",
        a = cx - 60.0,
        b = cx - 36.0,
        up = mid - 13.0,
        c = cx - 10.0,
        dn = mid + 14.0,
        d = cx + 14.0,
        m2 = mid - 5.0,
        e = cx + 38.0,
        xe = W - 30,
    );
    let body = format!(
        r#"
  <text x="30" y="27" font-family="{mono}" font-size="12" fill="{green}">vai@fleet:~$</text>
  <text x="138" y="27" font-family="{mono}" font-size="12" fill="{fg}">cargo watch -x &#x27;run --release&#x27;</text>
  <rect x="368" y="17" width="7" height="13" fill="{fg}"><animate attributeName="opacity" values="1;1;0;0" keyTimes="0;0.5;0.5;1" dur="1.2s" repeatCount="indefinite"/></rect>
  <text x="30" y="49" font-family="{mono}" font-size="11" fill="{muted}">[engine] 11 cards re-rendered · 0 warnings · fleet green</text>
  <text x="560" y="22" font-family="{mono}" font-size="10" fill="{comment}" letter-spacing="2">UPTIME MONITOR</text>
  <text x="{xe}" y="22" text-anchor="end" font-family="{mono}" font-size="10.5" fill="{green}">● 200 OK · 12 ms</text>
  <path d="{path}" fill="none" stroke="url(#trace_pl)" stroke-width="1.5" filter="url(#dg_pl)"/>
  <path d="{path}" fill="none" stroke="{red}" stroke-width="1.6" stroke-dasharray="26 900" opacity="0.9">
    <animate attributeName="stroke-dashoffset" values="926;0" dur="3.6s" repeatCount="indefinite"/>
  </path>
  <circle cx="{cx:.0}" cy="{mid:.0}" r="9" fill="none" stroke="{red}" stroke-width="1" opacity="0.35">
    <animate attributeName="r" values="5;13" dur="2.4s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0.5;0" dur="2.4s" repeatCount="indefinite"/>
  </circle>"#,
        mono = t::MONO,
        green = t::GREEN,
        fg = t::FG,
        muted = t::MUTED,
        comment = t::COMMENT,
        red = t::RED,
        xe = W - 30,
    );
    shell("pl", "Section divider: terminal readout", "", &body)
}

/// divider_editor.svg — one line of a CODE EDITOR: gutter, diff marker,
/// syntax-coloured Rust, a blinking caret and a minimap sliver.
fn editor() -> String {
    // Token stream with per-token colours (x advances by mono estimate).
    let tokens: &[(&str, &str)] = &[
        ("let", t::PURPLE),
        (" vibe ", t::FG),
        ("=", t::MUTED),
        (" TokyoNight", t::CYAN),
        ("::", t::MUTED),
        ("new", t::BLUE),
        ("()", t::FG),
        (".", t::MUTED),
        ("accent", t::BLUE),
        ("(", t::FG),
        ("Palette", t::CYAN),
        ("::", t::MUTED),
        ("Rust", t::ORANGE),
        (")", t::FG),
        (";", t::MUTED),
        ("  // 1.58 bits of style", t::COMMENT),
    ];
    let mut code = String::new();
    let mut x = 92.0;
    for (tok, col) in tokens {
        write!(
            code,
            r#"<text x="{x:.1}" y="38" font-family="{mono}" font-size="12.5" fill="{col}" xml:space="preserve">{tok}</text>"#,
            mono = t::MONO,
            tok = crate::svg::esc(tok),
        )
        .unwrap();
        x += crate::svg::text_width_px(tok, 12.5, true);
    }
    // Minimap sliver on the right.
    let mut minimap = String::new();
    for (i, wl) in [38.0, 26.0, 44.0, 20.0, 34.0, 42.0, 16.0, 30.0].iter().enumerate() {
        let col = match i % 4 {
            0 => t::PURPLE,
            1 => t::BLUE,
            2 => t::COMMENT,
            _ => t::CYAN,
        };
        write!(
            minimap,
            r#"<rect x="{x:.0}" y="{y}" width="{wl}" height="2.6" rx="1.3" fill="{col}" opacity="0.55"/>"#,
            x = W as f64 - 86.0,
            y = 12 + i * 6,
        )
        .unwrap();
    }
    let body = format!(
        r#"
  <rect x="8" y="4" width="58" height="{gh}" fill="{bgd}" opacity="0.75"/>
  <line x1="66" y1="4" x2="66" y2="{gb}" stroke="{bghl}" stroke-width="1"/>
  <text x="52" y="22" text-anchor="end" font-family="{mono}" font-size="11" fill="{comment}">157</text>
  <text x="52" y="38" text-anchor="end" font-family="{mono}" font-size="11" fill="{muted}">158</text>
  <text x="52" y="54" text-anchor="end" font-family="{mono}" font-size="11" fill="{comment}">159</text>
  <text x="74" y="38" font-family="{mono}" font-size="12.5" fill="{green}" font-weight="700">+</text>
  <rect x="68" y="26" width="{hlw}" height="17" fill="{green}" opacity="0.07"/>
  {code}
  <rect x="{caret:.0}" y="27" width="2" height="14" fill="{fg}"><animate attributeName="opacity" values="1;1;0;0" keyTimes="0;0.5;0.5;1" dur="1.1s" repeatCount="indefinite"/></rect>
  {minimap}
  <line x1="{mx}" y1="4" x2="{mx}" y2="{gb}" stroke="{bghl}" stroke-width="1"/>
  <rect x="{mx2}" y="24" width="86" height="16" fill="{fg}" opacity="0.05"/>"#,
        bgd = t::BG_DARK,
        bghl = t::BG_HL,
        comment = t::COMMENT,
        muted = t::MUTED,
        green = t::GREEN,
        fg = t::FG,
        mono = t::MONO,
        gh = H - 8,
        gb = H - 4,
        hlw = W as f64 - 68.0 - 92.0,
        caret = x + 4.0,
        mx = W - 92,
        mx2 = W - 90,
    );
    shell("ed", "Section divider: code editor", "", &body)
}

pub fn build(_ctx: &Ctx) -> Result<Vec<(String, String)>> {
    Ok(vec![
        ("divider.svg".into(), browser()),
        ("divider_wave.svg".into(), player()),
        ("divider_circuit.svg".into(), pcb()),
        ("divider_pulse.svg".into(), terminal()),
        ("divider_editor.svg".into(), editor()),
    ])
}
