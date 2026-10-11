//! `BoundedStepWalk`: a reusable, chrono-independent arithmetic Standard
//! for any grid walk that steps forward by a fixed `delta`, clamping at
//! an upper bound `hi` (the real shape the chrono-tz offset-discovery
//! scan uses: `next = (prev + delta).min(hi)`).
//!
//! Lives here, not in a Kani/Creusot/Verus backend crate: it's the
//! `Proposition` this module's own discovery `Exchange` names on its
//! `Output`, and a domain crate can't depend on a verifier-backend crate
//! (the dependency graph runs the other way — `amenable_kani` depends on
//! `amenable_ext`, never the reverse), exactly the reason
//! `amenable_time::IntervalStartPrecedesEnd` lives in `amenable_time`
//! rather than `amenable_kani::time`, with each backend separately
//! supplying its own `Witness<V>` for it.
//!
//! The useful bound this proves: if every single step of a walk
//! satisfies `0 <= next - prev <= delta`, then no real continuous
//! interval of length `>= delta` can go unobserved by the walk —
//! anything the walk's own table doesn't record must have had a real
//! duration strictly less than `delta`. That's the Tier-1 guarantee the
//! TzOffset discovery `Exchange` rests on.

use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

/// Claim: one step of the real `next = (prev + delta).min(hi)` stepping
/// formula never moves backward and never overshoots `delta`.
///
/// Stated under the precondition `delta > 0 && prev <= hi &&
/// prev.checked_add(delta).is_some()` (the overflow guard is load-
/// bearing, not defensive boilerplate: a real Kani run caught `prev +
/// delta` overflowing for `prev`/`delta` near `i64::MAX` before this
/// guard was added). Outside that domain this is a different
/// (degenerate) claim, not this one.
#[derive(Debug, Clone, Copy, Default, Standard)]
#[standard(basis = "RustStdStandard<i64>")]
pub struct BoundedStepWalk;

impl Metadata for BoundedStepWalk {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        Vec::new()
    }
}

impl Provenance for BoundedStepWalk {}

::inventory::submit! {
    ::amenable_core::EvidenceLink::new(
        concat!(module_path!(), "::", stringify!(BoundedStepWalk)),
        "amenable_std::rust_std::RustStdStandard<i64>",
        0,
    )
}
