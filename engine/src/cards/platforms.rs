//! Platforms — one stat card per provider plus a combined "All Platforms"
//! rollup. The per-provider cards deliberately use three different layouts
//! (source-of-truth console · social observatory · mirror node) so the stack
//! reads as a story, not three clones.

use crate::model::{format_count, PlatformData};
use crate::run::Ctx;
use crate::svg::{esc, fit_text, text_width_px};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;
use std::fmt::Write;

fn platform_color(id: &str) -> &'static str {
    match id {
        "vai-git" => t::RUST,
        "github" => t::BLUE,
        "codeberg" => t::TEAL,
        _ => t::PURPLE,
    }
}

// ── All-platforms rollup ────────────────────────────────────────────────────

fn all_card(ctx: &Ctx) -> (String, String) {
    let agg = &ctx.agg;
    let c = &agg.combined;
    let w = t::CARD_W;
    let h = 276;
    let m = t::MARGIN as f64;
    let usable = w as f64 - 2.0 * m;

    let cells = [
        ("repositories", format_count(Some(c.repo_count)), t::RUST),
        ("stars", format_count(Some(c.stars)), t::YELLOW),
        ("forks", format_count(Some(c.forks)), t::BLUE),
        ("est. LOC", format!("≈{}", format_count(Some(c.loc))), t::GREEN),
        (
            &format!("{}·1y", c.commit_label),
            format_count(c.commits.y1),
            t::ORANGE,
        ),
        ("streak", format!("{}🔥", c.streak), t::RED),
    ];
    let mut inner = String::new();
    for (i, (label, value, col)) in cells.iter().enumerate() {
        let cx = m + (i as f64 + 0.5) * (usable / 6.0);
        write!(
            inner,
            r#"<text x="{cx:.0}" y="64" text-anchor="middle" font-family="{mono}" font-size="30" font-weight="800" fill="{col}" filter="url(#glow)">{v}</text><text x="{cx:.0}" y="86" text-anchor="middle" font-family="{mono}" font-size="11.5" fill="{muted}" letter-spacing="1">{l}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(value),
            l = esc(&label.to_uppercase()),
        )
        .unwrap();
    }

    // Contribution share bar: each platform's slice of the yearly activity.
    write!(
        inner,
        r#"<line x1="{m:.0}" y1="104" x2="{x2:.0}" y2="104" stroke="{bghl}" stroke-width="1"/><text x="{m:.0}" y="128" font-family="{mono}" font-size="12" fill="{muted}" letter-spacing="1">WHERE THE WORK LANDS · share of yearly activity</text>"#,
        x2 = w as f64 - m,
        bghl = t::BG_HL,
        mono = t::MONO,
        muted = t::MUTED,
    )
    .unwrap();
    let shares: Vec<(String, u64, &str)> = agg
        .platforms
        .iter()
        .map(|p| {
            (
                p.display.clone(),
                p.rollup.commits.y1.unwrap_or(0),
                platform_color(&p.id),
            )
        })
        .collect();
    let total: u64 = shares.iter().map(|s| s.1).sum::<u64>().max(1);
    let mut lx = m;
    for (_, val, col) in &shares {
        let seg = *val as f64 / total as f64 * usable;
        write!(
            inner,
            r#"<rect x="{lx:.1}" y="140" width="{sw:.1}" height="14" rx="2" fill="{col}"/>"#,
            sw = (seg - 1.5).max(0.0),
        )
        .unwrap();
        lx += seg;
    }
    let mut legx = m;
    let leg_end = w as f64 - m;
    for (name, val, col) in &shares {
        let pct = *val as f64 / total as f64 * 100.0;
        let text = fit_text(
            &format!("{name} {pct:.0}% · {}", format_count(Some(*val))),
            (leg_end - legx - 16.0).max(20.0),
            11.5,
            true,
        );
        if legx + 16.0 >= leg_end {
            break;
        }
        write!(
            inner,
            r#"<circle cx="{cx:.0}" cy="174" r="4.5" fill="{col}"/><text x="{tx:.0}" y="178" font-family="{mono}" font-size="11.5" fill="{fgd}">{text}</text>"#,
            cx = legx + 5.0,
            tx = legx + 16.0,
            mono = t::MONO,
            fgd = t::FG_DIM,
            text = esc(&text),
        )
        .unwrap();
        legx += 16.0 + text_width_px(&text, 11.5, true) + 26.0;
    }
    // Static prose, wrapped by hand so it stays inside the margins.
    write!(
        inner,
        r#"<text x="{m:.0}" y="202" font-family="{sans}" font-size="12" fill="{muted}">Aggregation rule: mirrored repos count ONCE for code metrics (bytes → LOC, repo count) —</text><text x="{m:.0}" y="220" font-family="{sans}" font-size="12" fill="{muted}">but every platform's stars, forks and open items are summed. A mirror earns its own applause.</text>"#,
        sans = t::SANS,
        muted = t::MUTED,
    )
    .unwrap();

    let reachable = agg.platforms.iter().filter(|p| p.reachable).count();
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/platforms/all.sum",
            badge: &format!("{reachable}/{} online", agg.platforms.len()),
            accent: t::PURPLE,
            texture: Texture::Grid,
            alt: "All platforms combined: repositories, stars, forks, lines of code and activity",
        },
        &inner,
    );
    ("platforms_all.svg".to_string(), svg)
}

// ── Variant A: the source-of-truth console (primary Forgejo) ────────────────

fn primary_card(p: &PlatformData) -> String {
    let w = t::CARD_W;
    let h = 196;
    let m = t::MARGIN as f64;
    let accent = platform_color(&p.id);
    let r = &p.rollup;
    let mut inner = format!(
        r#"
    <text x="{m:.0}" y="52" font-family="{mono}" font-size="26" font-weight="800" fill="{accent}" filter="url(#glow)">{name}</text>
    <rect x="{m:.0}" y="66" width="126" height="22" rx="11" fill="{bghl}" stroke="{green}" stroke-width="0.9"/>
    <text x="{cx:.0}" y="81" text-anchor="middle" font-family="{mono}" font-size="11" fill="{green}">source of truth</text>
    <text x="{m:.0}" y="116" font-family="{mono}" font-size="12" fill="{muted}">@{user} · self-hosted Forgejo</text>
    <text x="{m:.0}" y="140" font-family="{sans}" font-size="12" fill="{fgd}">Every repo is born here, then mirrors ride out to the public forges.</text>
    "#,
        mono = t::MONO,
        sans = t::SANS,
        muted = t::MUTED,
        fgd = t::FG_DIM,
        bghl = t::BG_HL,
        green = t::GREEN,
        name = esc(&fit_text(&p.display, 420.0, 26.0, true)),
        cx = m + 63.0,
        user = esc(&fit_text(&p.user, 300.0, 12.0, true)),
    );
    // Middle stats.
    let stats = [
        ("repos", format_count(Some(r.repo_count)), accent),
        (
            &format!("{}·30d", r.commit_label),
            format_count(r.commits.d30),
            t::CYAN,
        ),
        ("streak", format!("{}🔥", r.streak), t::RED),
    ];
    for (i, (label, value, col)) in stats.iter().enumerate() {
        let x = 470.0 + i as f64 * 120.0;
        write!(
            inner,
            r#"<text x="{x:.0}" y="72" font-family="{mono}" font-size="26" font-weight="800" fill="{col}">{v}</text><text x="{x:.0}" y="92" font-family="{mono}" font-size="11" fill="{muted}">{l}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(&fit_text(value, 108.0, 26.0, true)),
            l = esc(&fit_text(label, 108.0, 11.0, true)),
        )
        .unwrap();
    }
    // Right: 12-week activity lattice (7 rows × 12 cols, most recent last).
    let days = &p.heatmap.days;
    let take = days.len().min(84);
    let recent = &days[days.len() - take..];
    let x0 = 470.0;
    let y0 = 108.0;
    write!(
        inner,
        r#"<text x="{x0:.0}" y="{ly:.0}" font-family="{mono}" font-size="11" fill="{muted}">last 12 weeks</text>"#,
        mono = t::MONO,
        muted = t::MUTED,
        ly = y0 - 4.0,
    )
    .unwrap();
    let max = recent.iter().map(|d| d.1).max().unwrap_or(1).max(1) as f64;
    for (i, (_, count)) in recent.iter().enumerate() {
        let week = i / 7;
        let day = i % 7;
        let cellx = x0 + week as f64 * 11.0;
        let celly = y0 + day as f64 * 6.4;
        let op = if *count == 0 {
            0.16
        } else {
            0.35 + 0.65 * (*count as f64 / max).sqrt()
        };
        write!(
            inner,
            r#"<rect x="{cellx:.1}" y="{celly:.1}" width="8.5" height="4.8" rx="1" fill="{green}" opacity="{op:.2}"/>"#,
            green = t::GREEN,
        )
        .unwrap();
    }
    t::card(
        &CardSpec {
            w,
            h,
            title: &format!("~/platforms/{}.host", p.id),
            badge: "primary",
            accent,
            texture: Texture::Ternary,
            alt: &format!("Platform stats for {} — the source-of-truth host", p.display),
        },
        &inner,
    )
}

// ── Variant B: the social observatory (GitHub) ──────────────────────────────

fn github_card(p: &PlatformData) -> String {
    let w = t::CARD_W;
    let h = 176;
    let m = t::MARGIN as f64;
    let usable = w as f64 - 2.0 * m;
    let accent = platform_color(&p.id);
    let r = &p.rollup;
    let cells = [
        ("repositories", format_count(Some(r.repo_count)), accent),
        ("stars", format_count(Some(r.stars)), t::YELLOW),
        ("followers", format_count(Some(p.profile.followers)), t::PURPLE),
        (
            &format!("{}·1y", r.commit_label),
            format_count(r.commits.y1),
            t::ORANGE,
        ),
    ];
    let mut inner = String::new();
    for (i, (label, value, col)) in cells.iter().enumerate() {
        let cx = m + (i as f64 + 0.5) * (usable / 4.0);
        write!(
            inner,
            r#"<text x="{cx:.0}" y="66" text-anchor="middle" font-family="{mono}" font-size="34" font-weight="800" fill="{col}" filter="url(#glow)">{v}</text><text x="{cx:.0}" y="88" text-anchor="middle" font-family="{mono}" font-size="11.5" fill="{muted}" letter-spacing="1">{l}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(value),
            l = esc(&label.to_uppercase()),
        )
        .unwrap();
    }
    // Top languages inline (this platform only).
    let mut langs: Vec<(&String, &u64)> = r.lang_bytes.iter().collect();
    langs.sort_by(|a, b| b.1.cmp(a.1));
    let total: u64 = r.lang_bytes.values().sum::<u64>().max(1);
    let mut lx = m;
    write!(
        inner,
        r#"<line x1="{m:.0}" y1="102" x2="{x2:.0}" y2="102" stroke="{bghl}" stroke-width="1"/>"#,
        x2 = w as f64 - m,
        bghl = t::BG_HL,
    )
    .unwrap();
    // Language chips stop before the right-anchored "@user" caption (~250px).
    let lang_end = w as f64 - m - 250.0;
    for (lang, bytes) in langs.iter().take(5) {
        let pct = **bytes as f64 / total as f64 * 100.0;
        let text = format!("{lang} {pct:.0}%");
        if lx + 14.0 + text_width_px(&text, 11.5, true) > lang_end {
            break;
        }
        write!(
            inner,
            r#"<circle cx="{cx:.0}" cy="122" r="4" fill="{col}"/><text x="{tx:.0}" y="126" font-family="{mono}" font-size="11.5" fill="{fgd}">{text}</text>"#,
            cx = lx + 4.0,
            tx = lx + 14.0,
            col = t::lang_color(lang),
            mono = t::MONO,
            fgd = t::FG_DIM,
            text = esc(&text),
        )
        .unwrap();
        lx += 14.0 + text_width_px(&text, 11.5, true) + 22.0;
    }
    write!(
        inner,
        r#"<text x="{x:.0}" y="126" text-anchor="end" font-family="{mono}" font-size="11" fill="{muted}">@{user} · the public stage</text>"#,
        x = w as f64 - m,
        mono = t::MONO,
        muted = t::MUTED,
        user = esc(&fit_text(&p.user, 170.0, 11.0, true)),
    )
    .unwrap();
    t::card(
        &CardSpec {
            w,
            h,
            title: &format!("~/platforms/{}.host", p.id),
            badge: &p.display,
            accent,
            texture: Texture::Grid,
            alt: &format!("Platform stats for {}", p.display),
        },
        &inner,
    )
}

// ── Variant C: the mirror node (other Forgejo instances) ────────────────────

fn mirror_card(p: &PlatformData) -> String {
    let w = t::CARD_W;
    let h = 140;
    let m = t::MARGIN as f64;
    let accent = platform_color(&p.id);
    let r = &p.rollup;
    let mut inner = format!(
        r#"
    <text x="{m:.0}" y="48" font-family="{mono}" font-size="22" font-weight="800" fill="{accent}">{name}</text>
    <rect x="{bx:.0}" y="30" width="102" height="22" rx="11" fill="{bghl}" stroke="{accent}" stroke-width="0.9"/>
    <text x="{btx:.0}" y="45" text-anchor="middle" font-family="{mono}" font-size="11" fill="{accent}">mirror node</text>
    <text x="{m:.0}" y="76" font-family="{sans}" font-size="12" fill="{fgd}">Mirrors of the source tree — code counted once, stars counted here too.</text>
    "#,
        mono = t::MONO,
        sans = t::SANS,
        bghl = t::BG_HL,
        fgd = t::FG_DIM,
        name = esc(&fit_text(&p.display, 380.0, 22.0, true)),
        bx = m + text_width_px(&p.display, 22.0, true).min(380.0) + 16.0,
        btx = m + text_width_px(&p.display, 22.0, true).min(380.0) + 67.0,
    );
    // Four FIXED slots so a big number can't push its neighbours off the card.
    let stats = [
        ("repos", format_count(Some(r.repo_count)), accent),
        ("stars", format_count(Some(r.stars)), t::YELLOW),
        ("forks", format_count(Some(r.forks)), t::BLUE),
        ("open items", format_count(Some(r.open_issues + r.open_prs)), t::RED),
    ];
    let sx0 = 560.0;
    let slot = (w as f64 - m - sx0) / stats.len() as f64;
    for (i, (label, value, col)) in stats.iter().enumerate() {
        let sx = sx0 + i as f64 * slot;
        write!(
            inner,
            r#"<text x="{sx:.0}" y="52" font-family="{mono}" font-size="24" font-weight="800" fill="{col}">{v}</text><text x="{sx:.0}" y="72" font-family="{mono}" font-size="10.5" fill="{muted}">{l}</text>"#,
            mono = t::MONO,
            muted = t::MUTED,
            v = esc(&fit_text(value, slot - 12.0, 24.0, true)),
            l = esc(&fit_text(label, slot - 12.0, 10.5, true)),
        )
        .unwrap();
    }
    let badge = format!("@{}", p.user);
    t::card(
        &CardSpec {
            w,
            h,
            title: &format!("~/platforms/{}.host", p.id),
            badge: &badge,
            accent,
            texture: Texture::Plain,
            alt: &format!("Platform stats for {} — mirror node", p.display),
        },
        &inner,
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let mut out = vec![all_card(ctx)];
    for p in &ctx.agg.platforms {
        if !p.reachable {
            crate::log::warn(&format!("platform {} unreachable — card skipped", p.id));
            continue;
        }
        let svg = if p.primary {
            primary_card(p)
        } else if p.id == "github" {
            github_card(p)
        } else {
            mirror_card(p)
        };
        out.push((format!("platform_{}.svg", p.id), svg));
    }
    Ok(out)
}
