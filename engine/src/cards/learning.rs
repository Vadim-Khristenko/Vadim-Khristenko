//! Learning — live "now learning / building": a rotating study topic, the
//! repo where changes are actually landing, and the research programme with a
//! real progress bar.

use super::pick;
use crate::run::Ctx;
use crate::svg::esc;
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{}…", cut.trim_end())
    }
}

fn field(x: u32, icon: &str, col: &str, label: &str, value: &str, note: &str) -> String {
    format!(
        r#"
    <text x="{x}" y="40" font-family="{mono}" font-size="12" fill="{col}" letter-spacing="1">{icon} {label}</text>
    <text x="{x}" y="68" font-family="{mono}" font-size="17" font-weight="700" fill="{fg}">{value}</text>
    <text x="{x}" y="88" font-family="{sans}" font-size="11.5" fill="{muted}">{note}</text>"#,
        mono = t::MONO,
        sans = t::SANS,
        fg = t::FG,
        muted = t::MUTED,
        label = esc(label),
        value = esc(value),
        note = esc(note),
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let w = t::CARD_W;
    let h = 216;
    let m = t::MARGIN;
    let p = &ctx.cfg.profile;
    let r = &p.research;

    // Rotating topic (2-day cadence, offset from the vibe card's picks).
    let fallback = crate::config::profile::NoteItem {
        name: r.name.clone(),
        note: r.subtitle.clone(),
    };
    let topic = if p.learning.topics.is_empty() {
        &fallback
    } else {
        pick(&p.learning.topics, ctx.vibe_seed, 3)
    };

    // Building: the freshest repo across every platform + its description.
    let building = ctx.agg.combined.most_active_repo.clone().unwrap_or_else(|| "—".into());
    let building_note = ctx
        .agg
        .platforms
        .iter()
        .flat_map(|pl| pl.repos.iter())
        .find(|repo| {
            crate::model::normalize_repo_name(&repo.name)
                == crate::model::normalize_repo_name(&building)
        })
        .and_then(|repo| repo.description.clone())
        .map(|d| truncate(&d, 44))
        .unwrap_or_else(|| "freshest push across all platforms".into());

    let pct = (r.progress.clamp(0.0, 1.0) * 100.0).round();
    let usable = (w - 2 * m) as f64;
    let fill_w = usable * r.progress.clamp(0.0, 1.0);
    let inner = format!(
        r#"
    {f1}
    {f2}
    {f3}
    <line x1="{m}" y1="106" x2="{x2}" y2="106" stroke="{bghl}" stroke-width="1"/>
    <text x="{m}" y="132" font-family="{mono}" font-size="12" fill="{purple}" letter-spacing="1">◆ RESEARCH PROGRESS · {rname}</text>
    <text x="{x2}" y="132" text-anchor="end" font-family="{mono}" font-size="12" fill="{fgd}">phase: {phase}</text>
    <rect x="{m}" y="142" width="{usable:.0}" height="8" rx="4" fill="{bghl}"/>
    <rect x="{m}" y="142" width="{fill_w:.0}" height="8" rx="4" fill="{purple}"/>
    <circle cx="{tip:.0}" cy="146" r="5" fill="{purple}" filter="url(#glow)">
      <animate attributeName="opacity" values="1;0.55;1" dur="2.6s" repeatCount="indefinite"/>
    </circle>
    <text x="{m}" y="168" font-family="{mono}" font-size="11" fill="{muted}">{pct:.0}% — measured in shipped experiments, not vibes</text>
    "#,
        f1 = field(m, "📚", t::GREEN, "NOW LEARNING", &topic.name, &topic.note),
        f2 = field(360, "🔨", t::ORANGE, "BUILDING", &building, &building_note),
        f3 = field(680, "🧪", t::PURPLE, "RESEARCH", &r.name, &truncate(&r.subtitle, 40)),
        x2 = w - m,
        bghl = t::BG_HL,
        mono = t::MONO,
        purple = t::PURPLE,
        fgd = t::FG_DIM,
        muted = t::MUTED,
        rname = esc(&r.name),
        phase = esc(&r.phase),
        tip = m as f64 + fill_w,
    );

    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/learning.now",
            badge: "rotates every 2 days",
            accent: t::GREEN,
            texture: Texture::Grid,
            alt: "Now learning and building: current topic, active repo and research progress",
        },
        &inner,
    );
    Ok(vec![("learning.svg".into(), svg)])
}
