//! Shared artifact-directory resolution.

use std::path::{Path, PathBuf};

/// The workspace's `artifacts/` directory, resolved relative to this
/// crate's manifest so it works regardless of the caller's current
/// working directory.
#[must_use]
pub fn artifacts_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
}

/// The `amenable_verus` crate's `src/` directory, resolved relative to
/// this crate's manifest so CLI entrypoints can emit proof modules
/// without depending on the caller's current working directory.
#[must_use]
pub fn verus_source_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_verus/src")
}

/// The `amenable_creusot` crate's generated-companion directory, resolved
/// the same way `verus_source_directory` is.
#[must_use]
pub fn creusot_generated_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_creusot/src/generated")
}

/// The `amenable_verus` gallery's generated Exchange-edge companion
/// directory -- the parent under which `write_verus_exchange_companions`
/// writes one subdirectory per `self_ty` group (`stoplight_exchange/`,
/// `ledger_exchange/`, ...), matching the `include!("generated/
/// {group}/{name}.rs")` paths already written into each gallery module
/// (relative to `amenable_verus/src/gallery/`).
#[must_use]
pub fn verus_exchange_generated_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_verus/src/gallery/generated")
}
