//! Installs the process-wide `tracing` subscriber.
//!
//! Lives here, not in the top-level `amenable` facade: every crate in the
//! family that has its own `tests/` directory (`amenable_core` itself,
//! `amenable_derive`, `amenable_gaap`, `amenable_std`) depends on
//! `amenable_core`, but `amenable_core` cannot depend on the facade
//! without a cycle. This is the one crate every test-bearing crate can
//! reach without a workspace re-export (see `CLAUDE.md`'s Workspace
//! Organization section) -- `amenable` re-exports it for its own
//! convenience, same as every other core role.

use tracing_subscriber::EnvFilter;

/// Reads `RUST_LOG` (falling back to `"info"` when unset) and installs a
/// `fmt` subscriber.
///
/// Safe to call more than once: `try_init` silently no-ops if a subscriber
/// is already installed, so `fn main` and every `#[test]` under `tests/`
/// can call this unconditionally rather than coordinating who goes first.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
