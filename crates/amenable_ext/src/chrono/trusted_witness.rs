//! `ChronoVerifier` trusted citations for the machine-checkable temporal
//! contracts the chrono backend's bridges reach.
//!
//! Kani, Creusot, and Verus check these contracts in their own crates, but
//! `ChronoVerifier` runs no formal tool. For it, each is a trusted citation,
//! the same way the jiff backend's `JiffVerifier` treats them. A violation is
//! still an `Err` at runtime, because the bridges check their inputs.
//!
//! The list grows as bridges need it. Each entry here is a contract a compiled
//! bridge bundle actually reaches, so the set is not a copy of the jiff list.

use amenable_core::{ClassifiedWitness, Standard, Witness, WitnessSupportSummary};
use amenable_time::TemporalProvenance;

use crate::chrono::identity::ChronoVerifier;

/// Emit a `ChronoVerifier` trusted `Witness` and `ClassifiedWitness` for each
/// named contract.
macro_rules! chrono_backend_trusts {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Witness<ChronoVerifier> for $ty {
                type SupportingEvidence = Self;
                type ProofArtifact = TemporalProvenance;

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn proof() -> Self::ProofArtifact {
                    <Self as Standard>::provenance(
                        &<Self as ::std::default::Default>::default(),
                    )
                }

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn support() -> WitnessSupportSummary {
                    WitnessSupportSummary::trusted_leaf()
                }
            }

            impl ClassifiedWitness<ChronoVerifier> for $ty {}
        )+
    };
}

chrono_backend_trusts! {
    amenable_time::CalendarDayWithinMonthBounds,
    amenable_time::CalendarMonthInRangeOneToTwelve,
    amenable_time::HourInRangeZeroToTwentyFour,
    amenable_time::LeapDayOccursOnlyInLeapYear,
    amenable_time::MinuteInRangeZeroToFiftyNine,
    amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix,
    amenable_time::SecondInRangeZeroToSixty,
    amenable_time::UtcOffsetHourInRangeZeroToTwentyThree,
    amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine,
    amenable_time::UtcTimelineOrderingAppliesToFixedInstants,
    amenable_time::WeekdayInRangeOneToSeven,
    amenable_time::WeekNumberInRangeOneToFiftyThree,
}

// `BoundedStepWalk` is chrono-tz-gated (its only real consumer so far is
// the chrono-tz offset-discovery `Exchange`), unlike the rest of this
// file's always-on `amenable_time` contracts -- and its own `Standard`
// basis (`RustStdStandard<i64>`) has a different `Provenance` than
// `chrono_backend_trusts!` assumes (hardcoded to `TemporalProvenance`),
// so this is a separate, hand-written impl rather than a call into that
// macro.
#[cfg(feature = "chrono-tz")]
impl Witness<ChronoVerifier> for crate::BoundedStepWalk {
    type SupportingEvidence = Self;
    type ProofArtifact = &'static str;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        "one step of a grid walk never overshoots delta -- proven on Kani, Creusot, and Verus; \
         trusted here by citation, the same way every other machine-checkable contract is \
         trusted under ChronoVerifier"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trusted_leaf()
    }
}

#[cfg(feature = "chrono-tz")]
impl ClassifiedWitness<ChronoVerifier> for crate::BoundedStepWalk {}

/// `TemporalInputReceived` -- "a caller handed us this request" -- is a
/// root claim asserted by construction (see `amenable_time::exchange::
/// markers`); its `Witness<V>` is trivial and backend-provided, so this
/// backend provides the `ChronoVerifier` one here, mirroring the jiff
/// backend's own `Witness<JiffVerifier> for TemporalInputReceived`.
impl Witness<ChronoVerifier> for amenable_time::TemporalInputReceived {
    type SupportingEvidence = Self;
    type ProofArtifact = &'static str;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        "raw request received at the exchange boundary -- a root claim, asserted by construction"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trivial_leaf()
    }
}

impl ClassifiedWitness<ChronoVerifier> for amenable_time::TemporalInputReceived {}
