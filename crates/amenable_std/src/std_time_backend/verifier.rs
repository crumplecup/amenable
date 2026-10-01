use amenable_core::{
    ClassifiedWitness, Metadata, OwnedEntry, Provenance, Standard, Verifier, Witness,
    WitnessSupportSummary,
};
use amenable_time::TemporalProvenance;

// ── CanaryVerifier ──────────────────────────────────────────────────

/// Reporting surface for [`CanaryVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifierMetadata;

impl Metadata for CanaryVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "canary"),
            OwnedEntry::new("kind", "runtime oracle over the std::time slice"),
            OwnedEntry::new("formal_tool", "none"),
        ]
    }
}

impl Provenance for CanaryVerifierMetadata {}

/// A [`Verifier`] marker for the `std::time` canary.
///
/// It runs no formal tool. What it *does* is let the canary's `Exchange`
/// bodies execute the same contract predicates the real backends prove
/// statically — `order_offset_endpoints` resolves both endpoints to epoch
/// seconds and refuses to establish `IntervalEndpointsOrdered` unless the
/// ordering genuinely holds; the duration bridge refuses to realize a
/// calendar-variable span as a fixed `std::time::Duration`. A violation is
/// an `Err` at runtime, not a proof obligation, so from this verifier's
/// point of view the machine-checkable contracts are trusted citations
/// (see `canary_trusts!`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifier;

impl Verifier for CanaryVerifier {
    type Metadata = CanaryVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "canary"
    }
}

// ── Trusted witnesses for the machine-checkable contracts ───────────
//
// The 23 range / ordering / arithmetic contracts carry real
// `Witness<KaniVerifier>` / `<CreusotVerifier>` / `<VerusVerifier>` proofs
// (`amenable_kani::time` etc.). `CanaryVerifier` is not one of those
// backends — it checks nothing — so from its point of view these are
// trusted citations exactly like the structural contracts. Without this
// block a `proof_composition` aggregate that reaches one of them (e.g.
// `IntervalEndpointsOrdered` → `IntervalStartPrecedesEnd`) would not be
// `Witness<CanaryVerifier>`, and the interval exchange surface would not
// type-check.

/// One `Witness<CanaryVerifier>` + `ClassifiedWitness<CanaryVerifier>` per
/// named contract, resting on the contract's own citation (the canary
/// verifies nothing).
macro_rules! canary_trusts {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Witness<CanaryVerifier> for $ty {
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

            impl ClassifiedWitness<CanaryVerifier> for $ty {}
        )+
    };
}

canary_trusts! {
    amenable_time::CalendarMonthInRangeOneToTwelve,
    amenable_time::HourInRangeZeroToTwentyFour,
    amenable_time::MinuteInRangeZeroToFiftyNine,
    amenable_time::SecondInRangeZeroToSixty,
    amenable_time::UtcOffsetHourInRangeZeroToTwentyThree,
    amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine,
    amenable_time::WeekdayInRangeOneToSeven,
    amenable_time::WeekNumberInRangeOneToFiftyThree,
    amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix,
    amenable_time::CenturyOrdinalInRangeZeroToNinetyNine,
    amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    amenable_time::IntervalStartPrecedesEnd,
    amenable_time::IntervalDurationIsNonNegative,
    amenable_time::UtcTimelineOrderingAppliesToFixedInstants,
    amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    amenable_time::CentennialYearDivisibleByOneHundred,
    amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays,
    amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays,
    amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    amenable_time::CalendarDayWithinMonthBounds,
    amenable_time::LeapDayOccursOnlyInLeapYear,
}

/// `TemporalInputReceived` — "a caller handed us this text" — is a root
/// claim asserted by construction (see `exchange::markers`); its
/// `Witness<V>` is trivial and backend-provided, so the canary provides
/// the `CanaryVerifier` one here.
impl Witness<CanaryVerifier> for amenable_time::TemporalInputReceived {
    type SupportingEvidence = Self;
    type ProofArtifact = &'static str;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        "raw text received at the exchange boundary — a root claim, asserted by construction"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trivial_leaf()
    }
}

impl ClassifiedWitness<CanaryVerifier> for amenable_time::TemporalInputReceived {}
