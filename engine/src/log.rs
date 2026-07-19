//! Tiny, pretty, dependency-free logger for the engine.
//!
//! Goals: laconic and scannable. Section headers, aligned step lines, status
//! glyphs and elapsed timing. ANSI colour when stdout is a TTY and NO_COLOR is
//! unset; plain text otherwise (so CI logs stay clean).

use std::io::IsTerminal;
use std::sync::OnceLock;
use std::time::Instant;

fn use_color() -> bool {
    static COLOR: OnceLock<bool> = OnceLock::new();
    *COLOR.get_or_init(|| std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none())
}

fn t0() -> Instant {
    static T0: OnceLock<Instant> = OnceLock::new();
    *T0.get_or_init(Instant::now)
}

/// Call once at startup so elapsed time is measured from process start.
pub fn init() {
    let _ = t0();
}

fn c(code: &str, text: &str) -> String {
    if use_color() {
        format!("\x1b[{code}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}

fn dim(t: &str) -> String {
    c("2;37", t)
}
fn rust(t: &str) -> String {
    c("38;5;215", t)
}
fn green(t: &str) -> String {
    c("38;5;150", t)
}
fn red(t: &str) -> String {
    c("38;5;210", t)
}
fn yellow(t: &str) -> String {
    c("38;5;222", t)
}
fn cyan(t: &str) -> String {
    c("38;5;117", t)
}
fn bold(t: &str) -> String {
    c("1", t)
}

fn elapsed() -> String {
    format!("{:6.1}s", t0().elapsed().as_secs_f64())
}

pub fn banner(title: &str, subtitle: &str) {
    let bar = "═".repeat(58);
    println!("{}", rust(&format!("╔{bar}╗")));
    println!("{}{}{}", rust("║"), bold(&format!("  {title:<56}")), rust("║"));
    if !subtitle.is_empty() {
        println!("{}{}{}", rust("║"), dim(&format!("  {subtitle:<56}")), rust("║"));
    }
    println!("{}", rust(&format!("╚{bar}╝")));
}

pub fn section(name: &str) {
    println!("\n{} {}", cyan("▸"), bold(name));
}

pub fn step(label: &str, value: &str, note: &str) {
    let mut line = format!("  {} {:<16} {}", dim("·"), label, cyan(value));
    if !note.is_empty() {
        line.push_str(&format!("  {}", dim(note)));
    }
    println!("{line}");
}

pub fn ok(label: &str, note: &str) {
    println!("  {} {:<16} {}", green("✓"), label, dim(note));
}

pub fn warn(msg: &str) {
    println!("  {} {}", yellow("⚠"), msg);
}

pub fn fail(label: &str, msg: &str) {
    eprintln!("  {} {:<16} {}", red("✗"), label, red(msg));
}

pub fn done(extra: &str) {
    let tail = if extra.is_empty() {
        String::new()
    } else {
        format!("  {}", dim(extra))
    };
    println!(
        "\n{} {} {}{}\n",
        green("◆"),
        bold("done"),
        dim(&format!("in {}", elapsed())),
        tail
    );
}
