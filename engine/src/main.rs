mod art;
mod cards;
mod cli;
mod config;
mod lastfm;
mod log;
mod model;
mod paths;
mod providers;
mod readme;
mod run;
mod svg;
mod theme;

fn main() {
    log::init();
    if let Err(e) = cli::main() {
        log::fail("engine", &format!("{e:#}"));
        std::process::exit(1);
    }
}
