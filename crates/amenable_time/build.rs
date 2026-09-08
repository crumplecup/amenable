//! Declares this crate's own real cfg names to rustc's `--check-cfg`, so
//! `cargo check`/`clippy` don't warn `unexpected_cfgs` on them. Per-crate,
//! not a single workspace-wide union list -- see `amenable_gaap/build.rs`
//! and the workspace `Cargo.toml`'s own comment for why.
//!
//! `kani` is declared because `#[derive(Standard)]` / `#[derive(Entry)]`
//! and this crate's own tracing convention emit `#[cfg_attr(not(kani),
//! ...)]`. `creusot` / `verus` are added here when the first contract
//! carries a backend-specific proof cfg (plan Phase 6).

fn main() {
    println!("cargo::rustc-check-cfg=cfg(kani)");
}
