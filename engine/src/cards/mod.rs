//! Card modules. Each exposes `build(&Ctx) -> Result<Vec<(String, String)>>`
//! returning one or more `(file_name, svg)` pairs, every one 1000 px wide so
//! the README stacks pixel-uniform on mobile. Text is drawn through
//! `crate::typo`, shared pieces live in `kit`.

#[cfg(test)]
mod tests;

pub mod acat;
pub mod architect;
pub mod badges;
pub mod bestgame;
pub mod dashboard;
pub mod divider;
pub mod footer;
pub mod games;
pub mod header;
pub mod homelab;
pub mod kit;
pub mod kumir;
pub mod music;
pub mod platforms;
pub mod research;
pub mod snackers;
pub mod themes;
pub mod thewall;
pub mod wiki;

/// Rotation helper shared by the editorial cards.
pub fn pick<T>(seq: &[T], seed: u64, salt: u64) -> &T {
    &seq[((seed + salt) % seq.len() as u64) as usize]
}
