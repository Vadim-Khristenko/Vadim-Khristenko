//! Repo-root discovery: works whether the binary runs from the repo root,
//! from `engine/`, or from a CI checkout.

use anyhow::{bail, Result};
use std::path::PathBuf;

/// Walk up from the current directory until we find `config/profile.toml`.
pub fn repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("config").join("profile.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    // Dev fallback: the crate lives at <root>/engine.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(parent) = manifest.parent() {
        if parent.join("config").join("profile.toml").is_file() {
            return Ok(parent.to_path_buf());
        }
    }
    bail!("cannot locate repo root (no config/profile.toml found upward from cwd)")
}
