//! The two state markers for the Phase 0 `Exchange` edge: a
//! [`FractionalSecond`](crate::FractionalSecond) carried across a backend
//! boundary, `Received` then `Preserved`.
//!
//! Root state claims, asserted rather than derived — the same shape as
//! `amenable_gaap::{Pending, Validated}` (`docs/AMENABLE_PLAN.md`, "States
//! Are Roots, Transitions Are Relations"). Each is a `Standard` whose
//! `Provenance` is itself. The real "the digits came through unchanged"
//! invariant is structural — the `carry` transition moves the digit
//! string straight from input to output, and the type system admits no
//! other `Exchange` from `Received` — so the proof backing the
//! `Received -> Preserved` edge only needs to establish "never panics,
//! always `Ok`", exactly like `Stoplight`'s own trivial edges.
//!
//! The carrier, the tokens, the edge, and the proofs live in
//! `amenable_kani::time` (Kani), with generated Creusot / Verus
//! companions — `amenable_time` stays free of any verifier dependency
//! (`docs/AMENABLE_TIME_PLAN.md`, "Crate hierarchy").

use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;

/// A `FractionalSecond` has entered a backend exchange but has not yet
/// been shown to survive it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Received;

impl Metadata for Received {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![OwnedEntry::new(
            "asserted",
            "a fractional second entered a backend exchange, by construction",
        )]
    }
}

impl Provenance for Received {}

/// A `FractionalSecond` crossed a backend exchange with every digit
/// intact — reachable only via a proven `Received -> Preserved` exchange
/// (`RFC 3339 §5.6`; the `SubsecondDigitsPreserved` accord requirement).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Preserved;

impl Metadata for Preserved {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![OwnedEntry::new(
            "asserted",
            "fractional-second digits survived a backend exchange, reachable only via a proven Received -> Preserved exchange",
        )]
    }
}

impl Provenance for Preserved {}
