//! Verus `Witness` impls for `amenable_time`'s genuinely-checkable
//! temporal contracts (`AMENABLE_TIME_PLAN.md` Phase 6).
//!
//! Unlike Kani and Creusot, the Verus toolchain is invoked as a bare
//! compiler over `amenable_verus/src/lib.rs` — it never resolves Cargo
//! dependencies, so the *proof* content (the real `verus! { ... }` spec
//! functions) lives there. This module is the ordinary-Rust half: it
//! ties each `amenable_time` contract to the Verus spec file that
//! machine-checks it (`include_str!`, so the two can never drift), and
//! registers the `Witness<VerusVerifier>` the `proof_composition`
//! composites need to resolve under `VerusVerifier`.

use amenable_std::VerusVerifier;
use derive_getters::Getters;
use derive_new::new;

use crate::CalendarMonthInRangeOneToTwelve;

/// Proof artifact naming an `amenable_verus` Verus spec function that
/// machine-checks a temporal contract's invariant, and carrying the
/// whole spec file it lives in verbatim (`include_str!`) — the same
/// one-claim-per-file granularity `amenable_derive::harness!` gives the
/// Kani/Creusot artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Getters, new)]
pub struct TemporalVerusProof {
    /// The Verus spec function that checks this contract's invariant.
    #[new(into)]
    harness: String,
    /// The spec's own source, verbatim.
    #[new(into)]
    claim: String,
}

impl core::fmt::Display for TemporalVerusProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        write!(f, "claim: {}", self.claim)
    }
}

// ── CalendarMonthInRangeOneToTwelve ──────────────────────────────────

const CALENDAR_MONTH_IN_RANGE_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/calendar_month_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for CalendarMonthInRangeOneToTwelve {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_calendar_month_in_range",
            CALENDAR_MONTH_IN_RANGE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for CalendarMonthInRangeOneToTwelve {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CalendarMonthInRangeOneToTwelve",
        "verus",
        || {
            <CalendarMonthInRangeOneToTwelve as amenable_core::Witness<VerusVerifier>>::proof()
                .to_string()
        },
    )
}
