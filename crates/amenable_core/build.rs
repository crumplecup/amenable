//! Declares this crate's own real cfg names to rustc's `--check-cfg`, so
//! `cargo check`/`clippy` don't warn `unexpected_cfgs` on them. Per-crate,
//! not a single workspace-wide union list: a union can't catch a
//! verifier's cfg name copy-pasted into the wrong crate (it's declared
//! "expected" everywhere), a per-crate list can.
//!
//! `kani`: `stoplight.rs`'s `Green`/`Yellow`/`Red` conditionally derive
//! `kani::Arbitrary`. `verus_keep_ghost`: `evidence.rs`, one of the
//! files `amenable_verus` splices in directly via `#[path]` -- both this
//! crate's own normal build and `amenable_verus`'s spliced-in copy need
//! the name declared independently, since Cargo lints each compilation
//! unit separately.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(kani)");
    println!("cargo::rustc-check-cfg=cfg(verus_keep_ghost)");
}
