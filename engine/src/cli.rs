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
//! Art commands accept a SteamGridDB key via --key or the environment
//! (SGDB_KEY / STEAMGRIDDB_KEY / STEAMGRIDDB_API_KEY — first non-empty wins).
//! The rotation slot can be forced with --pin-game <key> (or PIN_GAME).

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
        /// Force which game key sits in the rotation slot (or PIN_GAME env).
        #[arg(long)]
        pin_game: Option<String>,
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
        /// SteamGridDB key (or SGDB_KEY / STEAMGRIDDB_KEY / STEAMGRIDDB_API_KEY).
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        fixtures: bool,
        /// Force which game key sits in the rotation slot (or PIN_GAME env).
        #[arg(long)]
        pin_game: Option<String>,
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
        /// Force which game key sits in the rotation slot (or PIN_GAME env).
        #[arg(long)]
        pin_game: Option<String>,
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

/// `--pin-game` beats the PIN_GAME environment variable; empty values are
/// treated as unset so `PIN_GAME=""` in CI means "no pin".
fn pin_game(flag: &Option<String>) -> Option<String> {
    flag.clone()
        .or_else(|| std::env::var("PIN_GAME").ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Every env var accepted for the SteamGridDB key, in precedence order.
/// STEAMGRIDDB_API_KEY is the historical CI secret name — all three work.
const SGDB_KEY_VARS: [&str; 3] = ["SGDB_KEY", "STEAMGRIDDB_KEY", "STEAMGRIDDB_API_KEY"];

/// Key resolution as a pure function (env injected) so tests never race on
/// process-global environment state. Returns (key, source-label).
fn resolve_sgdb_key(
    cli: Option<String>,
    env: impl Fn(&str) -> Option<String>,
) -> Option<(String, &'static str)> {
    if let Some(k) = cli.filter(|k| !k.is_empty()) {
        return Some((k, "--key"));
    }
    for var in SGDB_KEY_VARS {
        if let Some(k) = env(var).filter(|k| !k.is_empty()) {
            return Some((k, var));
        }
    }
    None
}

/// Resolve the key and log WHERE it came from — set/not-set only, never the
/// value.
fn sgdb_key(key: &Option<String>) -> Option<String> {
    match resolve_sgdb_key(key.clone(), |var| std::env::var(var).ok()) {
        Some((k, source)) => {
            log::step("sgdb key", "set", &format!("via {source}"));
            Some(k)
        }
        None => {
            log::warn(&format!(
                "sgdb key: not set (checked --key, {})",
                SGDB_KEY_VARS.join(", ")
            ));
            None
        }
    }
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd.unwrap_or(Cmd::Build {
        only: None,
        no_readme: false,
        fixtures: false,
        pin_game: None,
    }) {
        Cmd::Build {
            only,
            no_readme,
            fixtures,
            pin_game: pin,
        } => {
            run::build_all(parse_only(&only), no_readme, env_fixtures(fixtures), pin_game(&pin))?;
        }
        Cmd::Rebuild {
            only,
            games,
            no_fetch,
            key,
            fixtures,
            pin_game: pin,
        } => {
            if !no_fetch {
                let root = crate::paths::repo_root()?;
                let cfg = crate::config::Config::load(&root.join("config"))?;
                let key = sgdb_key(&key);
                art::fetch_bestgame(&cfg, &root, key.as_deref());
                if games {
                    let only_games = std::env::var("GAME_ONLY").ok().map(|s| {
                        s.split(',').map(|x| x.trim().to_string()).collect::<BTreeSet<_>>()
                    });
                    art::fetch_games(&cfg, &root, key.as_deref(), only_games.as_ref());
                }
            }
            run::build_all(parse_only(&only), false, env_fixtures(fixtures), pin_game(&pin))?;
        }
        Cmd::Preview {
            only,
            no_open,
            fixtures,
            pin_game: pin,
        } => {
            let parsed = parse_only(&only);
            let ctx = run::build_all(parsed.clone(), true, env_fixtures(fixtures), pin_game(&pin))?;
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
                // sgdb_key() already logged set/not-set with the source.
                art::fetch_games(&cfg, &root, key.as_deref(), only_games.as_ref());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn env_of(pairs: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: BTreeMap<String, String> = pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |var: &str| map.get(var).cloned()
    }

    #[test]
    fn sgdb_key_accepts_the_historical_secret_name() {
        let got = resolve_sgdb_key(None, env_of(&[("STEAMGRIDDB_API_KEY", "k3")]));
        assert_eq!(got, Some(("k3".into(), "STEAMGRIDDB_API_KEY")));
    }

    #[test]
    fn sgdb_key_precedence_cli_then_env_order() {
        let all = [
            ("SGDB_KEY", "k1"),
            ("STEAMGRIDDB_KEY", "k2"),
            ("STEAMGRIDDB_API_KEY", "k3"),
        ];
        // CLI flag beats every env var.
        let got = resolve_sgdb_key(Some("cli".into()), env_of(&all));
        assert_eq!(got, Some(("cli".into(), "--key")));
        // SGDB_KEY beats the longer names.
        let got = resolve_sgdb_key(None, env_of(&all));
        assert_eq!(got, Some(("k1".into(), "SGDB_KEY")));
        // STEAMGRIDDB_KEY beats STEAMGRIDDB_API_KEY.
        let got = resolve_sgdb_key(None, env_of(&all[1..]));
        assert_eq!(got, Some(("k2".into(), "STEAMGRIDDB_KEY")));
    }

    #[test]
    fn sgdb_key_ignores_empty_values() {
        let got = resolve_sgdb_key(
            Some(String::new()),
            env_of(&[("SGDB_KEY", ""), ("STEAMGRIDDB_API_KEY", "real")]),
        );
        assert_eq!(got, Some(("real".into(), "STEAMGRIDDB_API_KEY")));
        assert_eq!(resolve_sgdb_key(None, env_of(&[])), None);
    }

    #[test]
    fn pin_game_flag_is_trimmed_and_empty_means_unset() {
        assert_eq!(pin_game(&Some(" nikke ".into())), Some("nikke".into()));
        // An explicitly empty flag never resolves to a pin.
        assert_eq!(pin_game(&Some("  ".into())), None);
    }
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
