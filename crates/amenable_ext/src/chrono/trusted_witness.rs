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
