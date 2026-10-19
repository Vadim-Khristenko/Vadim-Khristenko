//! Live telemetry as the same vocal editor as the header: totals in LCD
//! readouts, the year of contributions as a piano roll (seven lanes, one per
//! weekday, a note per active day, brighter for more commits) with a
//! playhead, and languages as one stacked spectrum bar.

use super::kit::{self, pal};
use crate::run::Ctx;
use crate::typo::{self, Doc, Font, T};
use anyhow::Result;
use chrono::{Datelike, Duration};
use std::fmt::Write;

const H: f64 = 430.0;
/// GitHub's linguist colours for the usual suspects, so a language keeps
/// its colour whatever its rank; anything else takes the next spare tone.
fn lang_color(name: &str, rank: usize) -> &'static str {
    match name {
        "Rust" => "#dea584",
        "TypeScript" => "#3178c6",
        "JavaScript" => "#f1e05a",
        "Python" => "#3572a5",
        "Vue" => "#41b883",
        "CSS" => "#663399",
        "HTML" => "#e34c26",
        "Shell" => "#89e051",
        "Cython" => "#fedf5b",
        "C++" => "#f34b7d",
        "Kotlin" => "#a97bff",
        "Dockerfile" => "#384d54",
        _ => [pal::TEAL, pal::SAKURA, pal::VIOLET, "#7aa2f7", "#e0af68"][rank % 5],
    }
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let agg = &ctx.agg;
    let mut doc = Doc::new();
    let mut b = String::new();
    let w = kit::W;
    let hm = &agg.heatmap;
    let year = agg.combined.commits.y1.unwrap_or_else(|| hm.sum_last(365));
    let totals = [
        ("STARS", kit::group(agg.combined.stars)),
        ("COMMITS / YEAR", kit::group(year)),
        ("REPOS", kit::group(agg.combined.repo_count)),
        ("FOLLOWERS", kit::group(agg.followers_total)),
        ("STREAK", format!("{} d", hm.streak())),
    ];

    let _ = write!(b, r#"<rect width="{w}" height="{H}" fill="{}"/>"#, pal::BG);
    let forges = agg.platforms.iter().filter(|p| p.reachable).count();
    b.push_str(&doc.text(
        40.0,
        44.0,
        &format!("LIVE TELEMETRY · {forges} FORGES · MIRRORS COUNTED ONCE"),
        &T::new(Font::Mono, 11.0).fill(pal::TEAL).ls(2.4),
    ));
    let _ = write!(
        b,
        r#"<circle class="led" cx="{}" cy="40" r="4" fill="{}"/>"#,
        w - 46.0,
        pal::TEAL
    );
    b.push_str(&doc.text(
        w - 56.0,
        44.0,
        &format!("synced {}", &ctx.stamp[..10.min(ctx.stamp.len())]),
        &T::new(Font::Mono, 11.0).fill(pal::DIM).end(),
    ));

    let mut x = 40.0;
    for (k, v) in &totals {
        let _ = write!(
            b,
            r#"<rect x="{x}" y="62" width="160" height="74" rx="8" fill="url(#lcd)" stroke="{}" stroke-opacity=".35"/>"#,
            pal::TEAL
        );
        b.push_str(&doc.text(
            x + 14.0,
            82.0,
            k,
            &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.4),
        ));
        let ghost: String = v
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { '8' } else { c })
            .collect();
        let lcd = T::new(Font::MonoBold, 32.0);
        let vw = typo::width(v, &lcd);
        let size = if vw > 136.0 { 32.0 * 136.0 / vw } else { 32.0 };
        b.push_str(&doc.text(
            x + 12.0,
            122.0,
            &ghost,
            &T::new(Font::MonoBold, size).fill(pal::TEAL).op(0.12),
        ));
        b.push_str(&doc.text(
            x + 12.0,
            122.0,
            v,
            &T::new(Font::MonoBold, size).fill(pal::TEAL),
        ));
        x += 172.0;
    }

    // piano-roll heatmap: 53 weeks by 7 weekdays
    let (rx, ry, c) = (92.0, 172.0, 16.0);
    b.push_str(&doc.text(
        40.0,
        ry - 10.0,
        "A YEAR OF COMMITS, ONE LANE PER WEEKDAY",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.6),
    ));
    for d in 0..7 {
        let y = ry + d as f64 * c;
        let black = d % 2 == 1;
        let _ = write!(
            b,
            r#"<rect x="40" y="{}" width="44" height="{}" rx="2" fill="{}"/><rect x="{rx}" y="{y}" width="{}" height="{c}" fill="{}"/>"#,
            y + 0.5,
            c - 1.0,
            if black { "#1a1c33" } else { "#2a2e52" },
            53.0 * c,
            if black { pal::BG_DEEP } else { pal::BG }
        );
        if let Some(lab) = ["Mon", "", "Wed", "", "Fri", "", "Sun"]
            .get(d)
            .filter(|s| !s.is_empty())
        {
            b.push_str(&doc.text(46.0, y + 11.0, lab, &T::new(Font::Mono, 8.5).fill(pal::FG)));
        }
    }
    for wk in 0..=53 {
        let xx = rx + wk as f64 * c;
        let _ = write!(
            b,
            r#"<line x1="{xx}" y1="{ry}" x2="{xx}" y2="{}" stroke="{}" stroke-opacity="{}"/>"#,
            ry + 7.0 * c,
            pal::LINE,
            if wk % 4 == 0 { ".6" } else { ".18" }
        );
    }
    if let Some((last, _)) = hm.days.last() {
        let start = *last - Duration::days(52 * 7 + last.weekday().num_days_from_monday() as i64);
        let best = hm.best_day().max(1) as f64;
        let mut teal = String::new();
        let mut month_seen = None;
        for (date, n) in &hm.days {
            if *date < start {
                continue;
            }
            let wk = (*date - start).num_days() / 7;
            let d = date.weekday().num_days_from_monday();
            if date.day() <= 7 && d == 0 && month_seen != Some(date.month()) {
                month_seen = Some(date.month());
                b.push_str(&doc.text(
                    rx + wk as f64 * c + 2.0,
                    ry + 7.0 * c + 14.0,
                    &date.format("%b").to_string(),
                    &T::new(Font::Mono, 8.5).fill(pal::DIM),
                ));
            }
            if *n == 0 {
                continue;
            }
            let ratio = *n as f64 / best;
            let (nx, ny) = (rx + wk as f64 * c + 1.5, ry + d as f64 * c + 2.5);
            if ratio > 0.8 {
                let _ = write!(
                    b,
                    r#"<rect x="{nx}" y="{ny}" width="{}" height="{}" rx="2" fill="{}"/>"#,
                    c - 3.0,
                    c - 5.0,
                    pal::SAKURA
                );
            } else {
                let _ = write!(
                    teal,
                    r#"<rect x="{nx}" y="{ny}" width="{}" height="{}" rx="2" fill-opacity="{:.2}"/>"#,
                    c - 3.0,
                    c - 5.0,
                    0.3 + 0.7 * ratio
                );
            }
        }
        let _ = write!(b, r#"<g fill="{}">{teal}</g>"#, pal::TEAL);
    }
    let _ = write!(
        b,
        r#"<g class="head"><line x1="{rx}" y1="{}" x2="{rx}" y2="{}" stroke="{t}" stroke-width="1.4"/><path d="M{} {} h10 l-5 7z" fill="{t}"/></g>"#,
        ry - 6.0,
        ry + 7.0 * c,
        rx - 5.0,
        ry - 12.0,
        t = pal::TEAL
    );

    // language spectrum
    let ly = 340.0;
    b.push_str(&doc.text(
        40.0,
        ly - 10.0,
        "LANGUAGES BY BYTES, CANONICAL COPIES ONLY",
        &T::new(Font::Mono, 9.5).fill(pal::DIM).ls(1.6),
    ));
    let mut langs: Vec<(&String, &u64)> = agg.combined.lang_bytes.iter().collect();
    langs.sort_by(|a, b| b.1.cmp(a.1));
    let total: u64 = langs.iter().map(|l| *l.1).sum::<u64>().max(1);
    let mut parts: Vec<(String, f64, &str)> = langs
        .iter()
        .take(5)
        .enumerate()
        .map(|(i, (n, v))| ((*n).clone(), **v as f64 / total as f64, lang_color(n, i)))
        .collect();
    let rest = 1.0 - parts.iter().map(|p| p.1).sum::<f64>();
    if rest > 0.005 {
        parts.push(("other".into(), rest, pal::MUTED));
    }
    let mut x = 40.0;
    let span = w - 80.0;
    for (i, (_, f, col)) in parts.iter().enumerate() {
        let lw = span * f;
        let _ = write!(
            b,
            r#"<rect class="grow" style="animation-delay:{:.2}s" x="{x:.1}" y="{ly}" width="{:.1}" height="14" rx="3" fill="{col}"/>"#,
            i as f64 * 0.08,
            (lw - 2.0).max(1.0)
        );
        x += lw;
    }
    let mut x = 40.0;
    for (name, f, col) in &parts {
        let nt = T::new(Font::ZenBlack, 13.0).fill(pal::FG_HI);
        let pct = format!("{}%", (f * 100.0).round());
        let _ = write!(
            b,
            r#"<rect x="{x}" y="{}" width="9" height="9" rx="2" fill="{col}"/>"#,
            ly + 32.0
        );
        b.push_str(&doc.text(x + 15.0, ly + 41.0, name, &nt));
        let nw = typo::width(name, &nt);
        b.push_str(&doc.text(
            x + 21.0 + nw,
            ly + 41.0,
            &pct,
            &T::new(Font::Mono, 11.0).fill(pal::DIM),
        ));
        x += 21.0 + nw + typo::width(&pct, &T::new(Font::Mono, 11.0)) + 30.0;
    }

    let frame = format!(
        r#"<g clip-path="url(#card)">{b}</g><rect x=".5" y=".5" width="{}" height="{}" rx="14" fill="none" stroke="{}" stroke-opacity=".9"/>"#,
        w - 1.0,
        H - 1.0,
        pal::LINE
    );
    let css = ".head{animation:sweep 14s linear infinite}@keyframes sweep{from{transform:translateX(0)}to{transform:translateX(848px)}}.grow{transform-box:fill-box;transform-origin:left;animation:grow 2.4s cubic-bezier(.16,1,.3,1) both}@keyframes grow{from{transform:scaleX(0)}}.led{animation:led 2s ease-in-out infinite}@keyframes led{50%{opacity:.3}}";
    let defs = format!(
        r##"<clipPath id="card"><rect width="{w}" height="{H}" rx="14"/></clipPath><linearGradient id="lcd" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#0a1418"/><stop offset="1" stop-color="#06100f"/></linearGradient>"##
    );
    Ok(vec![(
        "dashboard.svg".into(),
        kit::svg(
            w,
            H,
            "Live telemetry: totals, a year of commits and languages",
            css,
            &defs,
            &doc,
            &frame,
        ),
    )])
}
