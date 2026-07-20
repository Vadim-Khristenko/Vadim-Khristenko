//! Unified telemetry dashboard — one full-width card combining combined
//! multi-platform stats, commit activity, a stacked language bar, the weekday
//! commit rhythm and a language radar. One width ⇒ clean mobile scaling.

use crate::model::format_count;
use crate::run::Ctx;
use crate::svg::{esc, fit_text};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

fn sparkline(daily: &[u32], x: f64, y: f64, w: f64, h: f64) -> String {
    if daily.is_empty() {
        return format!(
            r#"<text x="{x:.0}" y="{ty:.0}" font-family="{mono}" font-size="11" fill="{muted}">awaiting first authenticated run…</text>"#,
            ty = y + h,
            mono = t::MONO,
            muted = t::MUTED,
        );
    }
    // sqrt scaling so a single huge day doesn't flatten all the others
    let peak = (*daily.iter().max().unwrap_or(&1) as f64).max(1.0).sqrt();
    let bw = w / daily.len() as f64;
    let mut out = String::new();
    for (i, v) in daily.iter().enumerate() {
        let bh = if *v > 0 {
            ((*v as f64).sqrt() / peak * h).max(2.0)
        } else {
            2.0
        };
        let bx = x + i as f64 * bw;
        let col = if *v > 0 { t::GREEN } else { t::BG_HL };
        out.push_str(&format!(
            r#"<rect x="{bx:.1}" y="{by:.1}" width="{bwv:.1}" height="{bh:.1}" rx="1.5" fill="{col}"/>"#,
            by = y + h - bh,
            bwv = (bw - 2.5).max(2.0),
        ));
    }
    out
}

fn weekday_rhythm(totals: &[u64; 7], x0: f64, x1: f64, label_y: u32, base_y: f64) -> String {
    const NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let mut out = format!(
        r#"<text x="{x0:.0}" y="{label_y}" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">COMMIT RHYTHM · by weekday</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
    );
    let max = *totals.iter().max().unwrap_or(&0);
    if max == 0 {
        out.push_str(&format!(
            r#"<text x="{x0:.0}" y="{y:.0}" font-family="{mono}" font-size="11" fill="{muted}">no calendar data yet</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            y = base_y - 30.0,
        ));
        return out;
    }
    let best = totals
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| **v)
        .map(|(i, _)| i)
        .unwrap_or(0);
    out.push_str(&format!(
        r#"<text x="{x1:.0}" y="{label_y}" text-anchor="end" font-family="{mono}" font-size="11" fill="{yellow}">most productive: {name}</text>"#,
        mono = t::MONO,
        yellow = t::YELLOW,
        name = NAMES[best],
    ));
    let gap = 14.0;
    let bw = (x1 - x0 - 6.0 * gap) / 7.0;
    let hmax = 84.0;
    for (i, v) in totals.iter().enumerate() {
        let bx = x0 + i as f64 * (bw + gap);
        let bh = ((*v as f64 / max as f64) * hmax).max(3.0);
        let col = if i == best { t::ORANGE } else { t::BLUE };
        out.push_str(&format!(
            r#"<rect x="{bx:.1}" y="{by:.1}" width="{bw:.1}" height="{bh:.1}" rx="3" fill="{col}" opacity="{op}"/><text x="{cx:.1}" y="{vy:.1}" text-anchor="middle" font-family="{mono}" font-size="10.5" fill="{fgd}">{val}</text><text x="{cx:.1}" y="{ny:.1}" text-anchor="middle" font-family="{mono}" font-size="11" fill="{lab_col}">{name}</text>"#,
            by = base_y - bh,
            op = if i == best { "1" } else { "0.75" },
            cx = bx + bw / 2.0,
            vy = base_y - bh - 6.0,
            fgd = t::FG_DIM,
            val = format_count(Some(*v)),
            ny = base_y + 16.0,
            lab_col = if i == best { t::YELLOW } else { t::MUTED },
            name = NAMES[i],
            mono = t::MONO,
        ));
    }
    out
}

/// How many top languages the bar/legend/radar show before the rest collapse
/// into a single aggregated "Others" bucket.
const TOP_LANGS: usize = 8;

/// Top `TOP_LANGS` languages (Rust pinned first) + an aggregated "Others"
/// bucket summing everything else. Shared by the stacked bar AND the radar so
/// the two views always tell the same story.
pub fn top_langs_plus_others(lang_bytes: &std::collections::BTreeMap<String, u64>) -> Vec<(String, u64)> {
    let mut all: Vec<(String, u64)> = lang_bytes.iter().map(|(k, v)| (k.clone(), *v)).collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    if let Some(pos) = all.iter().position(|kv| kv.0 == "Rust") {
        let rust = all.remove(pos);
        all.insert(0, rust);
    }
    let others: u64 = all.iter().skip(TOP_LANGS).map(|kv| kv.1).sum();
    let mut items: Vec<(String, u64)> = all.into_iter().take(TOP_LANGS).collect();
    if others > 0 {
        items.push(("Others".into(), others));
    }
    items
}

fn radar_color(lang: &str) -> &'static str {
    if lang == "Others" {
        t::COMMENT
    } else {
        t::lang_color(lang)
    }
}

/// Language radar with overlap-proof labels: each label sits on a radial
/// offset OUTSIDE the polygon, is anchored by quadrant (left half anchors end,
/// right half anchors start, poles center) so it always grows AWAY from the
/// chart, gets a vertical nudge above/below at the poles, and is width-capped
/// against the zone bounds so neighbours can never collide.
fn lang_radar(
    items: &[(String, u64)],
    label_x: f64,
    label_y: u32,
    cx: f64,
    cy: f64,
    r: f64,
    zone_left: f64,
    zone_right: f64,
) -> String {
    let mut out = format!(
        r#"<text x="{label_x:.0}" y="{label_y}" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">LANGUAGE RADAR · share of bytes</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
    );
    if items.is_empty() {
        return out;
    }
    let n = items.len().min(9);
    let total: u64 = items.iter().map(|kv| kv.1).sum::<u64>().max(1);
    let max = items.iter().take(n).map(|kv| kv.1).max().unwrap_or(1).max(1);
    let angle = |i: usize| -std::f64::consts::FRAC_PI_2 + i as f64 * std::f64::consts::TAU / n as f64;
    // Rings
    for ring in [0.33, 0.66, 1.0] {
        let pts: Vec<String> = (0..n)
            .map(|i| {
                let a = angle(i);
                format!("{:.1},{:.1}", cx + r * ring * a.cos(), cy + r * ring * a.sin())
            })
            .collect();
        out.push_str(&format!(
            r#"<polygon points="{}" fill="none" stroke="{}" stroke-width="0.8" opacity="0.5"/>"#,
            pts.join(" "),
            t::BG_HL,
        ));
    }
    // Axes + quadrant-anchored labels.
    const FS: f64 = 11.0;
    for (i, (lang, bytes)) in items.iter().take(n).enumerate() {
        let a = angle(i);
        let (dx, dy) = (a.cos(), a.sin());
        let (ex, ey) = (cx + r * dx, cy + r * dy);
        out.push_str(&format!(
            r#"<line x1="{cx:.1}" y1="{cy:.1}" x2="{ex:.1}" y2="{ey:.1}" stroke="{bghl}" stroke-width="0.8" opacity="0.6"/>"#,
            bghl = t::BG_HL,
        ));
        let lx = cx + (r + 16.0) * dx;
        let mut ly = cy + (r + 16.0) * dy;
        // Baseline nudge: above the chart near the top pole, below near the
        // bottom pole, vertically centred on the sides.
        ly += if dy < -0.35 {
            -4.0
        } else if dy > 0.35 {
            10.0
        } else {
            4.0
        };
        let anchor = if dx > 0.30 {
            "start"
        } else if dx < -0.30 {
            "end"
        } else {
            "middle"
        };
        // Cap the label so it stays inside the zone on its growth side —
        // labels on opposite sides grow away from each other and can't meet.
        let max_w = match anchor {
            "start" => zone_right - lx,
            "end" => lx - zone_left,
            _ => 2.0 * (zone_right - lx).min(lx - zone_left),
        }
        .max(20.0);
        let pct = *bytes as f64 / total as f64 * 100.0;
        let label = fit_text(&format!("{lang} {pct:.0}%"), max_w, FS, true);
        out.push_str(&format!(
            r#"<text x="{lx:.1}" y="{ly:.1}" text-anchor="{anchor}" font-family="{mono}" font-size="{FS}" fill="{fgd}">{label}</text>"#,
            mono = t::MONO,
            fgd = t::FG_DIM,
            label = esc(&label),
        ));
    }
    // Value polygon (sqrt for perceptual area)
    let mut pts = Vec::new();
    let mut dots = String::new();
    for (i, (lang, bytes)) in items.iter().take(n).enumerate() {
        let a = angle(i);
        let frac = (*bytes as f64 / max as f64).sqrt();
        let (px, py) = (cx + r * frac * a.cos(), cy + r * frac * a.sin());
        pts.push(format!("{px:.1},{py:.1}"));
        dots.push_str(&format!(
            r#"<circle cx="{px:.1}" cy="{py:.1}" r="3" fill="{col}"/>"#,
            col = radar_color(lang),
        ));
    }
    out.push_str(&format!(
        r#"<polygon points="{}" fill="{rust}" fill-opacity="0.18" stroke="{rust}" stroke-width="1.5"/>{dots}"#,
        pts.join(" "),
        rust = t::RUST,
    ));
    out
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let agg = &ctx.agg;
    let c = &agg.combined;
    let w = t::CARD_W;
    let h = 640;
    let m = t::MARGIN as f64;
    let usable = w as f64 - 2.0 * m;

    // ── Zone A: headline numbers ────────────────────────────────────────────
    let cells = [
        ("repositories", Some(c.repo_count), t::RUST),
        ("total stars", Some(c.stars), t::YELLOW),
        ("followers", Some(agg.followers_total), t::BLUE),
        ("following", Some(agg.following_total), t::PURPLE),
    ];
    let mut zone_a = String::new();
    for (i, (label, val, col)) in cells.iter().enumerate() {
        let cx = m + (i as f64 + 0.5) * (usable / 4.0);
        zone_a.push_str(&format!(
            r#"<text x="{cx:.0}" y="70" text-anchor="middle" font-family="{mono}" font-size="40" font-weight="800" fill="{col}" filter="url(#glow)">{v}</text><text x="{cx:.0}" y="92" text-anchor="middle" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">{lab}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(&format_count(*val)),
            lab = esc(&label.to_uppercase()),
        ));
    }

    // ── Zone B: commit activity ─────────────────────────────────────────────
    let lab = &c.commit_label;
    let metrics = [
        (format!("{lab}·7d"), c.commits.d7, t::GREEN),
        (format!("{lab}·30d"), c.commits.d30, t::CYAN),
        (format!("{lab}·1y"), c.commits.y1, t::ORANGE),
    ];
    let mut zone_b = format!(
        r#"<text x="{m:.0}" y="130" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">COMMIT ACTIVITY · all platforms</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
    );
    for (i, (l, v, col)) in metrics.iter().enumerate() {
        let cx = m + 70.0 + i as f64 * 130.0;
        zone_b.push_str(&format!(
            r#"<text x="{cx:.0}" y="172" text-anchor="middle" font-family="{mono}" font-size="30" font-weight="800" fill="{col}" filter="url(#glow)">{v}</text><text x="{cx:.0}" y="190" text-anchor="middle" font-family="{mono}" font-size="11" fill="{muted}">{l}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(&format_count(*v)),
            l = esc(l),
        ));
    }
    let total_days = agg.heatmap.days.len().max(1) as u64;
    let ratio = format!("{:.0}%", c.active_days as f64 / total_days as f64 * 100.0);
    let loc = if c.loc > 0 {
        format!("≈{}", format_count(Some(c.loc)))
    } else {
        "—".into()
    };
    let chips = [
        ("lines of code", loc, t::RUST),
        ("streak", format!("{}🔥", c.streak), t::RED),
        ("active days", format!("{}d · {ratio}", c.active_days), t::YELLOW),
    ];
    for (i, (l, v, col)) in chips.iter().enumerate() {
        let cx = m + i as f64 * 150.0;
        zone_b.push_str(&format!(
            r#"<text x="{cx:.0}" y="220" font-family="{mono}" font-size="11" fill="{muted}">{l}</text><text x="{cx:.0}" y="240" font-family="{mono}" font-size="17" font-weight="700" fill="{col}">{v}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            l = esc(l),
            v = esc(v),
        ));
    }
    zone_b.push_str(&format!(
        r#"<text x="{x:.0}" y="130" text-anchor="end" font-family="{mono}" font-size="11" fill="{muted}">last 30 days</text>"#,
        x = w as f64 - m,
        mono = t::MONO,
        muted = t::MUTED,
    ));
    let daily_30: Vec<u32> = {
        let days = &agg.heatmap.days;
        days[days.len().saturating_sub(30)..]
            .iter()
            .map(|d| d.1)
            .collect()
    };
    zone_b.push_str(&sparkline(&daily_30, m + 470.0, 142.0, usable - 470.0, 96.0));

    // ── Zone C: stacked language bar (top 8 + "Others", Rust first) ─────────
    let items = top_langs_plus_others(&c.lang_bytes);
    let grand: u64 = items.iter().map(|kv| kv.1).sum::<u64>().max(1);
    let mut zone_c = format!(
        r#"<text x="{m:.0}" y="280" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">LANGUAGES · top 8 + others · by bytes · mirrors counted once</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
    );
    let mut lx = m;
    for (lang, val) in &items {
        let seg_w = *val as f64 / grand as f64 * usable;
        zone_c.push_str(&format!(
            r#"<rect x="{lx:.1}" y="292" width="{sw:.1}" height="16" fill="{col}" rx="2"/>"#,
            sw = (seg_w - 1.5).max(0.0),
            col = radar_color(lang),
        ));
        lx += seg_w;
    }
    let per_row = 5;
    let col_w = usable / per_row as f64;
    for (i, (lang, val)) in items.iter().enumerate() {
        let pct = *val as f64 / grand as f64 * 100.0;
        let (row, col_i) = (i / per_row, i % per_row);
        let lx0 = m + col_i as f64 * col_w;
        let ly = 332 + row as u32 * 22;
        let star = if lang == "Rust" { " 🦀" } else { "" };
        let label = fit_text(&format!("{lang} {pct:.0}%{star}"), col_w - 24.0, 11.5, true);
        zone_c.push_str(&format!(
            r#"<circle cx="{cx:.0}" cy="{cy}" r="4.5" fill="{cc}"/><text x="{tx:.0}" y="{ly}" font-family="{mono}" font-size="11.5" fill="{fgd}">{label}</text>"#,
            cx = lx0 + 5.0,
            cy = ly - 4,
            tx = lx0 + 16.0,
            cc = radar_color(lang),
            mono = t::MONO,
            fgd = t::FG_DIM,
            label = esc(&label),
        ));
    }

    // ── Zone D: weekday rhythm + language radar (Others is a real slice) ────
    let mut zone_d = weekday_rhythm(&agg.heatmap.weekday_totals(), m, 440.0, 404, 560.0);
    zone_d.push_str(&lang_radar(&items, 530.0, 404, 748.0, 504.0, 56.0, 500.0, w as f64 - m));
    zone_d.push_str(&format!(
        r#"<line x1="486" y1="396" x2="486" y2="580" stroke="{bghl}" stroke-width="1"/>"#,
        bghl = t::BG_HL,
    ));

    let sep = format!(
        r#"<line x1="{m:.0}" y1="108" x2="{x2:.0}" y2="108" stroke="{bghl}" stroke-width="1"/><line x1="{m:.0}" y1="258" x2="{x2:.0}" y2="258" stroke="{bghl}" stroke-width="1"/><line x1="{m:.0}" y1="380" x2="{x2:.0}" y2="380" stroke="{bghl}" stroke-width="1"/>"#,
        x2 = w as f64 - m,
        bghl = t::BG_HL,
    );

    let reachable = agg.platforms.iter().filter(|p| p.reachable).count();
    let badge = format!("live · {reachable} platforms");
    let inner = format!("{zone_a}{sep}{zone_b}{zone_c}{zone_d}");
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/telemetry.dash",
            badge: &badge,
            accent: t::RUST,
            texture: Texture::Grid,
            alt: "Live telemetry: repositories, stars, commit activity, languages, weekday rhythm",
        },
        &inner,
    );
    Ok(vec![("dashboard.svg".into(), svg)])
}
