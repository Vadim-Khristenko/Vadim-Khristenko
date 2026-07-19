mod config;
mod log;
mod model;
mod paths;
mod providers;
mod readme;
mod svg;
mod theme;

fn main() -> anyhow::Result<()> {
    log::init();
    let root = paths::repo_root()?;
    let cfg = config::Config::load(&root.join("config"))?;
    log::banner(
        "VAI Profile Engine v3",
        &format!("{} providers configured", cfg.providers.provider.len()),
    );
    Ok(())
}
