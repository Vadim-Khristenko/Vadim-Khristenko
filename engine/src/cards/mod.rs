//! Card modules. Each exposes `build(&Ctx) -> Result<Vec<(String, String)>>`
//! returning one or more `(file_name, svg)` pairs, every one `CARD_W` wide so
//! the README stacks pixel-uniform on mobile.

pub mod bestgame;
pub mod dashboard;
pub mod divider;
pub mod flagship;
pub mod footer;
pub mod games;
pub mod header;
pub mod learning;
pub mod platforms;
pub mod research;
pub mod vibe;

/// Rotation helper shared by the editorial cards.
pub fn pick<T>(seq: &[T], seed: u64, salt: u64) -> &T {
    &seq[((seed + salt) % seq.len() as u64) as usize]
}
