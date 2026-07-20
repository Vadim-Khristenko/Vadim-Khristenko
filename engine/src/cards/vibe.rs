//! Weekly vibe — game & track rotate every 2 days; FOCUS is real data: the
//! repo where the most recent changes land, across every platform.

use super::pick;
use crate::run::Ctx;
use crate::svg::{esc, fit_text};
use crate::theme as t;
use crate::theme::{CardSpec, Texture};
use anyhow::Result;

fn field(x: u32, max_w: f64, icon: &str, col: &str, label: &str, value: &str, note: &str) -> String {
    format!(
        r#"
    <text x="{x}" y="40" font-family="{mono}" font-size="12" fill="{col}" letter-spacing="1">{icon} {label}</text>
    <text x="{x}" y="68" font-family="{mono}" font-size="18" font-weight="700" fill="{fg}">{value}</text>
    <text x="{x}" y="88" font-family="{sans}" font-size="12" fill="{muted}">{note}</text>"#,
        mono = t::MONO,
        sans = t::SANS,
        fg = t::FG,
        muted = t::MUTED,
        label = esc(&fit_text(label, max_w, 12.0, true)),
        value = esc(&fit_text(value, max_w, 18.0, true)),
        note = esc(&fit_text(note, max_w, 12.0, false)),
    )
}

pub fn build(ctx: &Ctx) -> Result<Vec<(String, String)>> {
    let seed = ctx.vibe_seed;
    let w = t::CARD_W;
    // Inner content reaches y≈131 below the 42px title bar → 192 keeps the
    // quote line fully inside the viewBox (it used to clip at 150).
    let h = 192;
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
        f1 = field(t::MARGIN, 312.0, "🎮", t::RED, "PLAYING", &game.title, &game.note),
        f2 = field(360, 296.0, "🎧", t::PURPLE, "ON LOOP", &composer.name, &composer.note),
        f3 = field(680, w as f64 - 680.0 - t::MARGIN as f64, "🛠", t::CYAN, "FOCUS", &focus_repo, "where most changes land"),
        m = t::MARGIN,
        x2 = w - t::MARGIN,
        bghl = t::BG_HL,
        sans = t::SANS,
        yellow = t::YELLOW,
        quote = esc(&fit_text(quote, w as f64 - 2.0 * t::MARGIN as f64 - 20.0, 13.0, false)),
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
