//! Shared artifact-directory resolution.

use std::path::{Path, PathBuf};

/// The workspace's `artifacts/` directory, resolved relative to this
/// crate's manifest so it works regardless of the caller's current
/// working directory.
#[must_use]
pub fn artifacts_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
}
