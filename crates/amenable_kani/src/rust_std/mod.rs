//! `KaniWitness` impls for Rust standard-library carriers, split into one
//! module per source-library area — mirrors `amenable_std::rust_std`'s
//! module boundaries, so the Kani proof for e.g. `core::num::NonZero<T>`
//! lives in `num.rs` here just as its `RustStdType` impl lives in
//! `amenable_std::rust_std::num`.
//!
//! Each type gets exactly the harness that's actually true of it — see
//! `macros.rs` for the two dispositions every type falls into (`trusted`:
//! nothing beyond what the type system already guarantees, so `proof()` is
//! just the chain-derived provenance; `checked`: a real `#[kani::proof]`
//! harness, captured verbatim via `amenable_derive::harness!` so it can
//! never drift from what `cargo kani` would actually run).

mod cmp;
mod convert;
mod macros;
mod net;
mod num;
mod ops;
mod primitives;
mod time;

pub use macros::CheckedProof;
