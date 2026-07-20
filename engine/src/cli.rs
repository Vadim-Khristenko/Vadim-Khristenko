//! Friendly command-line front-end for the engine.
//!
//!   vai-profile build                 # render cards + refresh README
//!   vai-profile build --only header   # one/few cards, skips README rewrite
//!   vai-profile build --fixtures     # offline dry-run from recorded fixtures
//!   vai-profile preview               # build, then open a stacked HTML preview
//!   vai-profile rebuild               # fetch best-game art (+ --games) then build
//!   vai-profile bestgame              # (re)download best-game cover + characters
//!   vai-profile art                   # (re)download the game-shelf covers
//!
//! Art commands accept a SteamGridDB key via --key or the SGDB_KEY env var.

use crate::{art, log, run};
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::BTreeSet;

#[derive(Parser)]
#[command(
    name = "vai-profile",
    version,
    about = "Self-regenerating multi-provider profile engine (Tokyo-Night SVG cards + live stats)"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Render cards + refresh README markers (default).
    Build {
        /// Comma list of card names (e.g. header,vibe). Skips the README rewrite.
        #[arg(long)]
        only: Option<String>,
        #[arg(long)]
        no_readme: bool,
        /// Offline mode: read recorded fixtures instead of the network.
        #[arg(long)]
        fixtures: bool,
    },
    /// Fetch best-game art (and optionally shelf covers), then build everything.
    Rebuild {
        #[arg(long)]
        only: Option<String>,
        /// Also re-fetch the game-shelf covers.
        #[arg(long)]
        games: bool,
        /// Skip art fetching, just render.
        #[arg(long)]
        no_fetch: bool,
        /// SteamGridDB key (or SGDB_KEY env).
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        fixtures: bool,
    },
    /// Build, then open a stacked HTML preview in the browser.
    Preview {
        #[arg(long)]
        only: Option<String>,
        /// Write the file but don't open a browser.
        #[arg(long)]
        no_open: bool,
        #[arg(long)]
        fixtures: bool,
    },
    /// (Re)download the game-shelf covers from SteamGridDB.
    Art {
        #[arg(long)]
        key: Option<String>,
        /// Comma list of game keys (or GAME_ONLY env).
        #[arg(long)]
        only: Option<String>,
        /// Also fetch the best-game art.
        #[arg(long)]
        bestgame: bool,
        /// Skip the shelf covers.
        #[arg(long)]
        skip_games: bool,
    },
    /// (Re)download best-game cover + character art.
    Bestgame {
        #[arg(long)]
        key: Option<String>,
    },
}

fn parse_only(only: &Option<String>) -> Option<BTreeSet<String>> {
    only.as_ref().map(|s| {
        s.split(',')
            .map(|x| x.trim().trim_end_matches(".svg").to_string())
            .filter(|x| !x.is_empty())
            .collect()
    })
}

fn env_fixtures(flag: bool) -> bool {
    flag || std::env::var("VAI_FIXTURES").map_or(false, |v| v == "1" || v == "true")
}

fn sgdb_key(key: &Option<String>) -> Option<String> {
    key.clone()
        .or_else(|| std::env::var("SGDB_KEY").ok())
        .or_else(|| std::env::var("STEAMGRIDDB_KEY").ok())
        .filter(|k| !k.is_empty())
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd.unwrap_or(Cmd::Build {
        only: None,
        no_readme: false,
        fixtures: false,
    }) {
        Cmd::Build {
            only,
            no_readme,
            fixtures,
        } => {
            run::build_all(parse_only(&only), no_readme, env_fixtures(fixtures))?;
        }
        Cmd::Rebuild {
            only,
            games,
            no_fetch,
            key,
            fixtures,
        } => {
            if !no_fetch {
                let root = crate::paths::repo_root()?;
                let cfg = crate::config::Config::load(&root.join("config"))?;
                art::fetch_bestgame(&cfg, &root, sgdb_key(&key).as_deref());
                if games {
                    let only_games = std::env::var("GAME_ONLY").ok().map(|s| {
                        s.split(',').map(|x| x.trim().to_string()).collect::<BTreeSet<_>>()
                    });
                    art::fetch_games(&cfg, &root, sgdb_key(&key).as_deref(), only_games.as_ref());
                }
            }
            run::build_all(parse_only(&only), false, env_fixtures(fixtures))?;
        }
        Cmd::Preview {
            only,
            no_open,
            fixtures,
        } => {
            let parsed = parse_only(&only);
            let ctx = run::build_all(parsed.clone(), true, env_fixtures(fixtures))?;
            preview(&ctx, parsed.as_ref(), no_open)?;
        }
        Cmd::Art {
            key,
            only,
            bestgame,
            skip_games,
        } => {
            let root = crate::paths::repo_root()?;
            let cfg = crate::config::Config::load(&root.join("config"))?;
            let key = sgdb_key(&key);
            log::banner("Game Art Fetcher", "SteamGridDB → uniform covers");
            let only_games = only
                .or_else(|| std::env::var("GAME_ONLY").ok())
                .map(|s| s.split(',').map(|x| x.trim().to_string()).collect::<BTreeSet<_>>());
            if bestgame {
                art::fetch_bestgame(&cfg, &root, key.as_deref());
            }
            if !skip_games {
                art::fetch_games(&cfg, &root, key.as_deref(), only_games.as_ref());
                if key.is_none() {
                    log::warn("no SteamGridDB key — shelf covers need SGDB_KEY");
                }
            }
            log::done("art → assets/");
        }
        Cmd::Bestgame { key } => {
            let root = crate::paths::repo_root()?;
            let cfg = crate::config::Config::load(&root.join("config"))?;
            log::banner("Game Art Fetcher", "best-game cover + characters");
            art::fetch_bestgame(&cfg, &root, sgdb_key(&key).as_deref());
            log::done("art → assets/bestgame/");
        }
    }
    Ok(())
}

/// Assemble tmp_prev/preview.html from the rendered cards and open it.
fn preview(ctx: &run::Ctx, only: Option<&BTreeSet<String>>, no_open: bool) -> Result<()> {
    let mut names: Vec<String> = Vec::new();
    for (card, _) in run::CARDS {
        if let Some(only) = only {
            if !only.contains(*card) {
                continue;
            }
        }
        match *card {
            "divider" => names.extend(
                [
                    "divider",
                    "divider_wave",
                    "divider_circuit",
                    "divider_pulse",
                    "divider_editor",
                ]
                .iter()
                .map(|s| s.to_string()),
            ),
            "platforms" => names.extend(run::platform_assets(ctx).iter().map(|n| n.trim_end_matches(".svg").to_string())),
            other => names.push(other.to_string()),
        }
    }
    let mut parts = String::new();
    for name in names {
        let p = ctx.assets.join(format!("{name}.svg"));
        if let Ok(svg) = std::fs::read_to_string(&p) {
            parts.push_str(&format!("<div style=\"margin:12px 0\">{svg}</div>"));
        }
    }
    let out_dir = ctx.root.join("tmp_prev");
    std::fs::create_dir_all(&out_dir)?;
    let out = out_dir.join("preview.html");
    std::fs::write(
        &out,
        format!(
            "<!doctype html><meta charset=\"utf-8\"><title>VAI profile preview</title><body style=\"margin:0;padding:20px;background:#0d0d12\">{parts}</body>"
        ),
    )?;
    log::ok("preview", &out.display().to_string());
    if !no_open {
        open_browser(&out);
    }
    Ok(())
}

fn open_browser(path: &std::path::Path) {
    let url = format!("file:///{}", path.display().to_string().replace('\\', "/"));
    #[cfg(target_os = "windows")]
    let result = std::process::Command::new("cmd").args(["/C", "start", "", &url]).spawn();
    #[cfg(target_os = "macos")]
    let result = std::process::Command::new("open").arg(&url).spawn();
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    let result = std::process::Command::new("xdg-open").arg(&url).spawn();
    if let Err(e) = result {
        log::warn(&format!("could not open browser: {e}"));
    }
}
