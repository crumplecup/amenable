#![cfg(creusot)]
//! The opaque `Into<i64>`-stand-in accessor and its three concrete-width
//! trusted lemmas.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was three separately `#[cfg(creusot)]`-gated `use`s plus three
//! `fn`s in the parent file down to a single `use` there, cordial's
//! CFG-SCATTER finding. `span_i64_of`/`span_i64_of_*_lemma` stay
//! `pub(crate)`, declared directly at this module's own top level (not
//! nested any deeper): reused by `accessors.rs` and `span_fieldwise.rs`,
//! and a two-hop re-export through private nesting is real toolchain
//! territory Creusot's own visibility check rejects even though plain
//! rustc accepts it — the same finding `accessors.rs`'s own doc comment
//! records.
//!
//! The three lemmas' `#[ensures(..)]` clauses reference the parent
//! file's own `span_i64_of_*_matches_cast` logic predicates (each
//! `harness!`-generated in `lemmas.rs` itself), imported by name
//! rather than called via a `super::`-qualified path — a child module
//! calling back into its parent is ordinary Rust name resolution, not
//! a cycle: both sides only exist together, under the same `creusot`
//! compilation. (A qualified call here reads identically to Rust but
//! defeats cordial's contract-fragment name matching, which expects
//! the bare name the `harness!` macro registered — confirmed by a
//! real ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001 finding when this was
//! first written with `super::` inline.)

use super::{
    span_i64_of_i16_matches_cast, span_i64_of_i32_matches_cast, span_i64_of_i64_matches_cast,
};
use creusot_std::macros::{ensures, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn span_i64_of<I>(_x: I) -> i64 {
    dead
}

#[trusted]
#[ensures(span_i64_of_i16_matches_cast(x))]
pub(crate) fn span_i64_of_i16_lemma(x: i16) {}

#[trusted]
#[ensures(span_i64_of_i32_matches_cast(x))]
pub(crate) fn span_i64_of_i32_lemma(x: i32) {}

#[trusted]
#[ensures(span_i64_of_i64_matches_cast(x))]
pub(crate) fn span_i64_of_i64_lemma(x: i64) {}
