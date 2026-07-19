//! Weekly vibe — game & track rotate every 2 days; FOCUS is real data: the
//! repo where the most recent changes land, across every platform.

use super::pick;
use crate::run::Ctx;
use crate::svg::esc;
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

fn field(x: u32, icon: &str, col: &str, label: &str, value: &str, note: &str) -> String {
    format!(
        r#"
    <text x="{x}" y="40" font-family="{mono}" font-size="12" fill="{col}" letter-spacing="1">{icon} {label}</text>
    <text x="{x}" y="68" font-family="{mono}" font-size="18" font-weight="700" fill="{fg}">{value}</text>
    <text x="{x}" y="88" font-family="{sans}" font-size="12" fill="{muted}">{note}</text>"#,
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
    let seed = ctx.vibe_seed;
    let w = t::CARD_W;
    let h = 150;
    let p = &ctx.cfg.profile;
    let game = pick(&p.games, seed, 1);
    let composer = pick(&p.composers, seed, 2);
    let quote = pick(&p.quotes, seed, 4);
    let focus_repo = ctx
        .agg
        .combined
        .most_active_repo
        .clone()
        .unwrap_or_else(|| "—".into());

    let inner = format!(
        r#"
    {f1}
    {f2}
    {f3}
    <line x1="{m}" y1="104" x2="{x2}" y2="104" stroke="{bghl}" stroke-width="1"/>
    <text x="{m}" y="128" font-family="{sans}" font-size="13" font-style="italic" fill="{yellow}">“{quote}”</text>
    "#,
        f1 = field(t::MARGIN, "🎮", t::RED, "PLAYING", &game.title, &game.note),
        f2 = field(360, "🎧", t::PURPLE, "ON LOOP", &composer.name, &composer.note),
        f3 = field(680, "🛠", t::CYAN, "FOCUS", &focus_repo, "where most changes land"),
        m = t::MARGIN,
        x2 = w - t::MARGIN,
        bghl = t::BG_HL,
        sans = t::SANS,
        yellow = t::YELLOW,
        quote = esc(quote),
    );

    let badge = format!("refreshed {}", &ctx.stamp[..10]);
    let svg = t::card(
        &CardSpec {
            w,
            h,
            title: "~/now.vibe",
            badge: &badge,
            accent: t::MAGENTA,
            texture: Texture::Grid,
            alt: "Current vibe: what I'm playing, listening to and focused on",
        },
        &inner,
    );
    Ok(vec![("vibe.svg".into(), svg)])
}
