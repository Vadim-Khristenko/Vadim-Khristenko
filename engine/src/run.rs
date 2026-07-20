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
    /// Last.fm music data (live fetch or fixtures) — None = graceful skip.
    pub music: Option<crate::lastfm::Music>,
    /// ISO year*100 + ISO week (weekly rotations).
    pub seed: u64,
    /// Day-ordinal / 2 — rotates the vibe card every 2 days.
    pub vibe_seed: u64,
    /// Manual rotation lever (`--pin-game` / `PIN_GAME`): forces which game
    /// key sits in the hero/vibe "in rotation" slot. None = seed rotation.
    pub pin_game: Option<String>,
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

    /// The game in the "in rotation" slot — shared by the vibe card and the
    /// games hero shelf so they always agree. A valid pin (`--pin-game` /
    /// `PIN_GAME` matching a configured key) beats the vibe-seed rotation.
    pub fn featured_game(&self) -> &crate::config::profile::Game {
        if let Some(pin) = &self.pin_game {
            if let Some(g) = self.cfg.profile.games.iter().find(|g| &g.key == pin) {
                return g;
            }
        }
        cards::pick(&self.cfg.profile.games, self.vibe_seed, 1)
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
    let platforms: Vec<_> = provider_set
        .iter()
        .map(|p| providers::collect(p.as_ref(), &cfg.stats))
        .collect();
    let agg = providers::aggregate(platforms);
    let flagship: Vec<FlagshipLive> = cfg
        .flagship
        .project
        .iter()
        .map(|proj| providers::resolve_flagship(proj, &agg, &provider_set, &cfg.stats))
        .collect();

    // Music: fixtures in offline mode, live Last.fm otherwise (graceful skip
    // when the username or key is absent).
    let music = if fixtures {
        crate::lastfm::from_fixtures(&fixtures_dir)
    } else {
        cfg.profile.lastfm.as_ref().and_then(crate::lastfm::fetch)
    };

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
        music,
        seed,
        vibe_seed,
        pin_game: None,
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
    // Typos in `--only` would otherwise silently render nothing.
    if let Some(only) = only {
        for req in only {
            if !CARDS.iter().any(|(name, _)| name == req) {
                log::warn(&format!(
                    "--only: unknown card '{req}' — known: {}",
                    CARDS.iter().map(|(n, _)| *n).collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }
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

pub fn build_all(
    only: Option<BTreeSet<String>>,
    no_readme: bool,
    fixtures: bool,
    pin_game: Option<String>,
) -> Result<Ctx> {
    let mut ctx = build_context(fixtures)?;
    if let Some(pin) = pin_game {
        if ctx.cfg.profile.games.iter().any(|g| g.key == pin) {
            log::step("pin-game", &pin, "rotation slot forced (--pin-game / PIN_GAME)");
            ctx.pin_game = Some(pin);
        } else {
            log::warn(&format!(
                "pin-game '{pin}' is not a configured game key — using the rotation pick. Known keys: {}",
                ctx.cfg.profile.games.iter().map(|g| g.key.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }
    }
    let built = render(&ctx, only.as_ref());
    warn_missing_art(&ctx);
    if !no_readme && only.is_none() {
        update_readme(&ctx)?;
    }
    log::done(&format!("{} cards · seed {}", built.len(), ctx.seed));
    Ok(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_ctx() -> Ctx {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("engine/ has a parent")
            .to_path_buf();
        build_context_at(root, true).expect("fixture context must build")
    }

    #[test]
    fn featured_game_defaults_to_the_seed_pick() {
        let ctx = fixture_ctx();
        let expected = crate::cards::pick(&ctx.cfg.profile.games, ctx.vibe_seed, 1);
        assert_eq!(ctx.featured_game().key, expected.key);
    }

    #[test]
    fn pin_game_overrides_the_seed_pick_everywhere() {
        let mut ctx = fixture_ctx();
        // Pin a key that is NOT the current rotation pick, for every seed we
        // try — the pin must always win.
        for seed in [ctx.vibe_seed, ctx.vibe_seed + 1, ctx.vibe_seed + 7] {
            ctx.vibe_seed = seed;
            let rotation = crate::cards::pick(&ctx.cfg.profile.games, seed, 1).key.clone();
            let pin = ctx
                .cfg
                .profile
                .games
                .iter()
                .map(|g| g.key.clone())
                .find(|k| *k != rotation)
                .expect("more than one game configured");
            ctx.pin_game = Some(pin.clone());
            assert_eq!(ctx.featured_game().key, pin, "pin must beat seed {seed}");

            // Both consumers agree: the vibe card PLAYING slot and the games
            // hero shelf feature the pinned title.
            let pinned_title = ctx.featured_game().title.clone();
            let (_, vibe) = &crate::cards::vibe::build(&ctx).unwrap()[0];
            let (_, games) = &crate::cards::games::build(&ctx).unwrap()[0];
            assert!(vibe.contains(&pinned_title), "vibe card must show the pin");
            assert!(
                games.contains(&format!("featuring {pinned_title}")),
                "games shelf hero must feature the pin"
            );
            ctx.pin_game = None;
        }
    }

    #[test]
    fn unknown_pin_falls_back_to_rotation() {
        let mut ctx = fixture_ctx();
        ctx.pin_game = Some("definitely-not-a-game".into());
        let expected = crate::cards::pick(&ctx.cfg.profile.games, ctx.vibe_seed, 1);
        assert_eq!(ctx.featured_game().key, expected.key);
    }

    #[test]
    fn miside_is_pinnable() {
        let mut ctx = fixture_ctx();
        ctx.pin_game = Some("miside".into());
        assert_eq!(ctx.featured_game().key, "miside");
        let (_, games) = &crate::cards::games::build(&ctx).unwrap()[0];
        assert!(games.contains("featuring MiSide"));
    }
}
