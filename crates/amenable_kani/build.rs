//! Declares this crate's own real cfg names to rustc's `--check-cfg`, so
//! `cargo check`/`clippy` don't warn `unexpected_cfgs` on them. Per-crate,
//! not a single workspace-wide union list: a union can't catch a
//! verifier's cfg name copy-pasted into the wrong crate (it's declared
//! "expected" everywhere), a per-crate list can.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(kani)");
}
