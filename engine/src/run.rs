//! Orchestrator: collect platform data once, render every card in isolation
//! (one bad card never breaks the rest), then inject cache-busted image rows
//! into README.md.

use crate::cards;
use crate::config::Config;
use crate::log;
use crate::model::{Aggregate, FlagshipLive};
use crate::providers;
use crate::readme;
use crate::svg;
use anyhow::{Context, Result};
use chrono::{Datelike, Utc};
use std::collections::BTreeSet;
use std::path::PathBuf;

pub struct Ctx {
    pub cfg: Config,
    pub agg: Aggregate,
    pub flagship: Vec<FlagshipLive>,
    /// ISO year*100 + ISO week (weekly rotations).
    pub seed: u64,
    /// Day-ordinal / 2 — rotates the vibe card every 2 days.
    pub vibe_seed: u64,
    pub week: u32,
    pub year: i32,
    pub stamp: String,
    pub root: PathBuf,
    pub assets: PathBuf,
    pub raw_base: String,
}

impl Ctx {
    /// Is this card enabled? (`[cards.<name>] enabled = false` disables it.)
    pub fn card_enabled(&self, name: &str) -> bool {
        self.cfg
            .profile
            .cards
            .get(name)
            .map_or(true, |tw| tw.enabled)
    }

    /// The card's accent colour: config override, else the given default.
    pub fn accent(&self, name: &str, default: &str) -> String {
        self.cfg
            .profile
            .cards
            .get(name)
            .and_then(|tw| tw.accent.clone())
            .unwrap_or_else(|| default.to_string())
    }
}

/// A card module: name (for `--only`) + builder returning ≥1 (file, svg) pair.
type CardFn = fn(&Ctx) -> Result<Vec<(String, String)>>;

pub const CARDS: &[(&str, CardFn)] = &[
    ("header", cards::header::build),
    ("divider", cards::divider::build),
    ("research", cards::research::build),
    ("flagship", cards::flagship::build),
    ("dashboard", cards::dashboard::build),
    ("platforms", cards::platforms::build),
    ("learning", cards::learning::build),
    ("vibe", cards::vibe::build),
    ("bestgame", cards::bestgame::build),
    ("games", cards::games::build),
    ("footer", cards::footer::build),
];

pub fn build_context(fixtures: bool) -> Result<Ctx> {
    build_context_at(crate::paths::repo_root()?, fixtures)
}

/// Build the full render context rooted at an explicit repo directory —
/// `build_context` for normal runs, direct calls from tests.
pub fn build_context_at(root: PathBuf, fixtures: bool) -> Result<Ctx> {
    let cfg = Config::load(&root.join("config"))?;

    let now = Utc::now();
    let iso = now.iso_week();
    let seed = iso.year() as u64 * 100 + iso.week() as u64;
    let vibe_seed = (now.date_naive().num_days_from_ce() / 2) as u64;
    log::banner(
        "VAI Profile Engine v3",
        &format!(
            "{} providers  ·  week {}/{}  ·  vibe-bucket={}{}",
            cfg.providers.provider.len(),
            iso.week(),
            iso.year(),
            vibe_seed,
            if fixtures { "  ·  FIXTURE MODE" } else { "" }
        ),
    );

    let fixtures_dir = root.join("engine").join("fixtures");
    let provider_set =
        providers::make_providers(&cfg.providers, fixtures.then_some(fixtures_dir.as_path()));
    let platforms: Vec<_> = provider_set.iter().map(|p| providers::collect(p.as_ref())).collect();
    let agg = providers::aggregate(platforms);
    let flagship: Vec<FlagshipLive> = cfg
        .flagship
        .project
        .iter()
        .map(|proj| providers::resolve_flagship(proj, &agg, &provider_set))
        .collect();

    let gh_user = cfg
        .providers
        .provider
        .iter()
        .find(|p| p.kind == crate::config::ProviderKind::Github)
        .map(|p| p.user.clone())
        .unwrap_or_else(|| "Vadim-Khristenko".into());

    Ok(Ctx {
        raw_base: format!("https://raw.githubusercontent.com/{gh_user}/{gh_user}/main/assets"),
        assets: root.join("assets"),
        agg,
        flagship,
        seed,
        vibe_seed,
        week: iso.week(),
        year: iso.year(),
        stamp: now.format("%Y-%m-%d %H:%M UTC").to_string(),
        cfg,
        root,
    })
}

pub fn render(ctx: &Ctx, only: Option<&BTreeSet<String>>) -> Vec<String> {
    std::fs::create_dir_all(&ctx.assets).ok();
    log::section("Rendering cards");
    let mut built = Vec::new();
    for (name, builder) in CARDS {
        if let Some(only) = only {
            if !only.contains(*name) {
                continue;
            }
        }
        if !ctx.card_enabled(name) {
            log::warn(&format!("{name}: disabled via [cards.{name}] — skipped"));
            continue;
        }
        match builder(ctx) {
            Ok(outputs) => {
                for (file, svg_text) in outputs {
                    match svg::validate_xml(&svg_text) {
                        Ok(()) => match std::fs::write(ctx.assets.join(&file), &svg_text) {
                            Ok(()) => {
                                log::ok(&file, &format!("{:.1} KB", svg_text.len() as f64 / 1024.0));
                                built.push(file);
                            }
                            Err(e) => log::fail(&file, &e.to_string()),
                        },
                        Err(e) => log::fail(&file, &e.to_string()),
                    }
                }
            }
            // Autonomous: one bad card never breaks the rest.
            Err(e) => log::fail(name, &e.to_string()),
        }
    }
    built
}

/// Ordered platform-card asset names that exist on disk (combined card first).
pub fn platform_assets(ctx: &Ctx) -> Vec<String> {
    let mut names = vec!["platforms_all.svg".to_string()];
    for p in &ctx.agg.platforms {
        names.push(format!("platform_{}.svg", p.id));
    }
    names.retain(|n| ctx.assets.join(n).is_file());
    names
}

pub fn update_readme(ctx: &Ctx) -> Result<()> {
    let readme_path = ctx.root.join("README.md");
    if !readme_path.is_file() {
        log::warn("README.md not found");
        return Ok(());
    }
    log::section("Updating README");
    let mut text = std::fs::read_to_string(&readme_path).context("read README.md")?;
    // Cache also refreshes on the 2-day cadence.
    let seed_str = ctx.vibe_seed.to_string();
    let salt = &seed_str[seed_str.len().saturating_sub(5)..];
    for (key, content) in readme::blocks(&ctx.raw_base, &ctx.assets, salt, &platform_assets(ctx)) {
        text = readme::inject(&text, &key, &content);
    }
    std::fs::write(&readme_path, text).context("write README.md")?;
    log::ok("README", "markers refreshed");
    Ok(())
}

/// Nudge when config references character art that hasn't been fetched yet —
/// the exact cause of "I added a character but it shows an avatar".
pub fn warn_missing_art(ctx: &Ctx) {
    let dir = ctx.assets.join("bestgame");
    let missing: Vec<String> = ctx
        .cfg
        .profile
        .best_game
        .characters
        .iter()
        .filter(|ch| !ch.art_url.is_empty() && svg::find_media(&dir, &ch.key()).is_none())
        .map(|ch| ch.short.clone())
        .collect();
    if !missing.is_empty() {
        log::warn(&format!("character art not fetched yet: {}", missing.join(", ")));
        log::warn("  run:  vai-profile bestgame   (or: vai-profile rebuild)");
    }
}

pub fn build_all(only: Option<BTreeSet<String>>, no_readme: bool, fixtures: bool) -> Result<Ctx> {
    let ctx = build_context(fixtures)?;
    let built = render(&ctx, only.as_ref());
    warn_missing_art(&ctx);
    if !no_readme && only.is_none() {
        update_readme(&ctx)?;
    }
    log::done(&format!("{} cards · seed {}", built.len(), ctx.seed));
    Ok(ctx)
}
