//! Flagship — the coolest projects, config-driven, rendered with LIVE stats:
//! stars, forks, total commits, open issues+PRs, a mini language bar and a
//! 30-day health sparkline per project. Mirror rule applies: stars/forks/open
//! items are summed across every platform hosting the repo.

use crate::model::{format_count, FlagshipLive};
use crate::run::Ctx;
use crate::svg::{esc, fit_text, text_width_px};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;

const ROW_H: u32 = 118;

fn lang_bar(langs: &[(String, u64)], x: f64, y: f64, w: f64) -> String {
    if langs.is_empty() {
        return String::new();
    }
    let total: u64 = langs.iter().map(|kv| kv.1).sum::<u64>().max(1);
    let mut out = String::new();
    let mut lx = x;
    for (lang, bytes) in langs.iter().take(4) {
        let seg = *bytes as f64 / total as f64 * w;
        write!(
            out,
            r#"<rect x="{lx:.1}" y="{y:.1}" width="{sw:.1}" height="7" rx="2" fill="{col}"/>"#,
            sw = (seg - 1.5).max(1.0),
            col = t::lang_color(lang),
        )
        .unwrap();
        lx += seg;
    }
    // Name the dominant language so the bar carries meaning, not just colour.
    if let Some((lang, bytes)) = langs.first() {
        write!(
            out,
            r#"<text x="{tx:.1}" y="{ty:.1}" text-anchor="end" font-family="{mono}" font-size="10.5" fill="{muted}">{lang} {pct:.0}%</text>"#,
            tx = x + w,
            ty = y - 5.0,
            mono = t::MONO,
            muted = t::MUTED,
            lang = esc(lang),
            pct = *bytes as f64 / total as f64 * 100.0,
        )
        .unwrap();
    }
    out
}

fn spark(daily: &[u32], x: f64, y: f64, w: f64, h: f64, accent: &str) -> String {
    if daily.is_empty() || daily.iter().all(|v| *v == 0) {
        return format!(
            r#"<text x="{x:.0}" y="{ty:.0}" font-family="{mono}" font-size="10.5" fill="{muted}">quiet month — stable release</text>"#,
            ty = y + h - 2.0,
            mono = t::MONO,
            muted = t::MUTED,
        );
    }
    let peak = (*daily.iter().max().unwrap() as f64).sqrt();
    let bw = w / daily.len() as f64;
    let mut out = String::new();
    for (i, v) in daily.iter().enumerate() {
        let bh = if *v > 0 {
            ((*v as f64).sqrt() / peak * h).max(2.0)
        } else {
            1.5
        };
        let col = if *v > 0 { accent } else { t::BG_HL };
        write!(
            out,
            r#"<rect x="{bx:.1}" y="{by:.1}" width="{bwv:.1}" height="{bh:.1}" rx="1" fill="{col}"/>"#,
            bx = x + i as f64 * bw,
            by = y + h - bh,
            bwv = (bw - 2.0).max(1.5),
        )
        .unwrap();
    }
    out
}

fn row(p: &FlagshipLive, y: u32, w: u32) -> String {
    let m = t::MARGIN as f64;
    let accent = p.accent.as_deref().unwrap_or(t::RUST);
    let mut out = String::new();

    // Identity column: x ∈ [m+26, ID_END]. Everything is width-capped.
    const ID_END: f64 = 584.0;
    let nx = m + 26.0;
    let name = fit_text(&p.name, ID_END - nx - 90.0, 18.0, true);
    write!(
        out,
        r#"<path d="M{dx},{dy} l6,6 l-6,6 l-6,-6 z" fill="{accent}"/><text x="{nx:.0}" y="{ny}" font-family="{mono}" font-size="18" font-weight="800" fill="{fg}">{name}</text>"#,
        dx = t::MARGIN + 6,
        dy = y + 22,
        ny = y + 34,
        mono = t::MONO,
        fg = t::FG,
        name = esc(&name),
    )
    .unwrap();
    if let Some(source) = &p.source {
        let x = nx + text_width_px(&name, 18.0, true) + 10.0;
        let via = fit_text(&format!("· via {source}"), ID_END - x, 11.0, true);
        write!(
            out,
            r#"<text x="{x:.0}" y="{ny}" font-family="{mono}" font-size="11" fill="{muted}">{via}</text>"#,
            mono = t::MONO,
            ny = y + 33,
            muted = t::MUTED,
            via = esc(&via),
        )
        .unwrap();
    }
    if let Some(site) = &p.site {
        let display = site.trim_start_matches("https://").trim_start_matches("http://");
        let display = fit_text(&format!("↗ {display}"), ID_END - nx, 12.0, true);
        write!(
            out,
            r#"<text x="{nx:.0}" y="{sy}" font-family="{mono}" font-size="12" fill="{cyan}">{display}</text>"#,
            mono = t::MONO,
            sy = y + 56,
            cyan = t::CYAN,
            display = esc(&display),
        )
        .unwrap();
    }
    write!(
        out,
        r#"<text x="{nx:.0}" y="{by}" font-family="{sans}" font-size="12.5" fill="{fgd}">{blurb}</text>"#,
        by = y + 78,
        sans = t::SANS,
        fgd = t::FG_DIM,
        blurb = esc(&fit_text(&p.blurb, ID_END - nx, 12.5, false)),
    )
    .unwrap();
    let tags: String = p.tags.iter().map(|tag| format!("#{tag}")).collect::<Vec<_>>().join("  ");
    write!(
        out,
        r#"<text x="{nx:.0}" y="{ty}" font-family="{mono}" font-size="11" fill="{muted}">{tags}</text>"#,
        mono = t::MONO,
        ty = y + 100,
        muted = t::MUTED,
        tags = esc(&fit_text(&tags, ID_END - nx, 11.0, true)),
    )
    .unwrap();

    // Live-stats column: four FIXED slots so huge numbers can't push their
    // neighbours (or the card edge) — each value is width-capped to its slot.
    let sx = 596.0;
    let sw = w as f64 - sx - m;
    if p.repo.is_some() {
        let stats = [
            ("★", format_count(Some(p.stars)), "stars", t::YELLOW),
            ("⑂", format_count(Some(p.forks)), "forks", t::BLUE),
            ("⧗", format_count(p.pulse.total_commits), "commits", t::GREEN),
            ("◌", format_count(Some(p.open_items)), "open", t::RED),
        ];
        let slot = sw / stats.len() as f64;
        for (i, (icon, value, label, col)) in stats.iter().enumerate() {
            let cx = sx + i as f64 * slot;
            let value = fit_text(value, slot - 26.0, 14.0, true);
            write!(
                out,
                r#"<text x="{cx:.0}" y="{sy}" font-family="{mono}" font-size="13" fill="{col}">{icon}</text><text x="{vx:.0}" y="{sy}" font-family="{mono}" font-size="14" font-weight="700" fill="{fg}">{value}</text><text x="{vx:.0}" y="{ly}" font-family="{mono}" font-size="10" fill="{muted}">{label}</text>"#,
                sy = y + 30,
                ly = y + 44,
                vx = cx + 18.0,
                fg = t::FG,
                muted = t::MUTED,
                mono = t::MONO,
                value = esc(&value),
            )
            .unwrap();
        }
        let langs: Vec<(String, u64)> = {
            let mut v: Vec<(String, u64)> = p.langs.iter().map(|(k, b)| (k.clone(), *b)).collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            v
        };
        out.push_str(&lang_bar(&langs, sx, y as f64 + 60.0, sw));
        out.push_str(&spark(&p.pulse.daily_30, sx, y as f64 + 74.0, sw, 30.0, accent));
    } else {
        write!(
            out,
            r#"<text x="{sx:.0}" y="{sy}" font-family="{mono}" font-size="12" fill="{muted}">live stats unavailable</text><text x="{sx:.0}" y="{sy2}" font-family="{mono}" font-size="11" fill="{muted}">repo private or platform offline</text>"#,
            sy = y + 30,
            sy2 = y + 48,
            mono = t::MONO,
            muted = t::MUTED,
        )
        .unwrap();
    }
    out
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let projects = &ctx.flagship;
    let w = t::CARD_W;
    let n = projects.len().max(1) as u32;
    let top = 52u32;
    let h = t::BAR_H + top + n * ROW_H + 8;
    let m = t::MARGIN;

    let mut inner = format!(
        r#"<text x="{m}" y="32" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">FLAGSHIP BUILDS · live stats · mirrors summed across platforms</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
    );
    for (i, p) in projects.iter().enumerate() {
        let y = top + i as u32 * ROW_H;
        if i > 0 {
            write!(
                inner,
                r#"<line x1="{m}" y1="{y}" x2="{x2}" y2="{y}" stroke="{bghl}" stroke-width="1" opacity="0.8"/>"#,
                x2 = w - m,
                bghl = t::BG_HL,
            )?;
        }
        inner.push_str(&row(p, y + 4, w));
    }

    let live = projects.iter().filter(|p| p.repo.is_some()).count();
    let badge = format!("{live}/{} live", projects.len());
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/flagship/projects.toml",
            badge: &badge,
            accent: t::RUST,
            texture: Texture::Grid,
            alt: "Flagship projects with live stars, forks, commits and activity",
        },
        &inner,
    );
    Ok(vec![("flagship.svg".into(), svg)])
}
