//! Verus proof content for Rust standard-library carriers.
//!
//! This crate contains *only* what `verus --crate-type=lib
//! crates/amenable_verus/src/lib.rs` needs to check: real `verus! { ... }`
//! spec functions, nothing else. Unlike `amenable_creusot` (which still
//! depends on `amenable_core`/`amenable_derive`, since `cargo creusot`
//! resolves ordinary Cargo dependencies), Verus is invoked as a bare
//! compiler over a single file tree — it never sees `Cargo.toml`, so it
//! cannot resolve `amenable_core`, `inventory`, or any proc-macro crate at
//! all. Confirmed empirically: pointing `verus` at the old, pre-split
//! version of this crate (which depended on all three) failed immediately
//! with unresolved-crate errors, not proof errors.
//!
//! So this crate now has exactly two dependencies — `verus_builtin_macros`
//! and `vstd` — mirroring `elicitation_verus`'s own real, working
//! structure exactly (confirmed by reading `~/repos/elicitation/crates/
//! elicitation_verus/src/*.rs`: every file there is equally
//! dependency-free). The witness/registry machinery that bridges these
//! proofs to `amenable_core::Witness`/`amenable_std::RustStdStandard<T>`
//! lives entirely in `amenable_std::verus_witness` instead, which captures
//! each proof's verbatim source via `include_str!` (a plain Rust feature,
//! no proc-macro needed) rather than `amenable_derive::harness!` (which,
//! being a proc-macro from a crate Verus can't resolve, wouldn't compile
//! under Verus's toolchain either).
//!
//! `char` and `String` are real Rust primitives/std types with genuine
//! `vstd` spec support (`char as u32` casts and `String`'s `View` impl
//! giving `s@`/`s@.len()` both verified empirically against a real Verus
//! install) — no shadow-struct workaround needed for either, unlike most
//! of `elicitation_verus`'s own std-adjacent proofs (their `strings.rs`
//! deliberately avoids storing a real `String` field at all, citing "Verus
//! doesn't have specs for it" — true for their `Contract` wrapper types'
//! needs at the time, but `elicitation_verus`'s own `gallery::level5`
//! demonstrates the real `s@.len()` pattern this crate uses directly).

pub mod rust_std;
