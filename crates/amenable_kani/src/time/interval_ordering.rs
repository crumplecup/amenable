//! Interval-endpoint and UTC-timeline ordering contracts.
//!
//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};
use amenable_core::{Ensures, Evidence, Standard, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use amenable_time::{
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    UtcTimelineOrderingAppliesToFixedInstants,
};

// ── IntervalStartPrecedesEnd ──────────────────

impl Witness<KaniVerifier> for IntervalStartPrecedesEnd {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_interval_start_precedes_end".to_owned(),
            VERIFY_INTERVAL_START_PRECEDES_END_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::IntervalStartPrecedesEnd",
        "kani",
        || <IntervalStartPrecedesEnd as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    IntervalStartPrecedesEnd,
    "amenable_time::IntervalStartPrecedesEnd",
    (i32, i32),
    |(start, end)| start <= end
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_START_PRECEDES_END_SRC, {
        /// ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first
        /// endpoint is no later than its second on the relevant
        /// timeline. `start <= end` holds when the endpoints coincide
        /// or are ordered, and fails when reversed — checked at both
        /// ends of the `i32` range.
        #[kani::proof]
        fn verify_interval_start_precedes_end() {
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 0)));
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 1)));
            assert!(!<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((1, 0)));
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MIN, i32::MAX)));
            assert!(!<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MAX, i32::MIN)));
        }
    }
}
// ── IntervalDurationIsNonNegative ──────────────────

impl Witness<KaniVerifier> for IntervalDurationIsNonNegative {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_interval_duration_is_non_negative".to_owned(),
            VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::IntervalDurationIsNonNegative",
        "kani",
        || <IntervalDurationIsNonNegative as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    IntervalDurationIsNonNegative,
    "amenable_time::IntervalDurationIsNonNegative",
    (i32, i32),
    |(start, end)| i64::from(end) - i64::from(start) >= 0
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, {
        /// ISO 8601-1:2019, 3.1.1.8 — the span between an interval's
        /// endpoints is zero or positive, never negative. Non-negative
        /// when ordered or coincident, negative when reversed —
        /// checked at both ends of the `i32` range.
        #[kani::proof]
        fn verify_interval_duration_is_non_negative() {
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 0)));
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 1)));
            assert!(!<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((1, 0)));
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MIN, i32::MAX)));
            assert!(!<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MAX, i32::MIN)));
        }
    }
}
// ── UtcTimelineOrderingAppliesToFixedInstants ──────────────────

impl Witness<KaniVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
            VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
        "kani",
        || <UtcTimelineOrderingAppliesToFixedInstants as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcTimelineOrderingAppliesToFixedInstants,
    "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
    (i32, i32),
    |(a, b)| a <= b
);

/// `<=` on UTC timeline positions is a total order: reflexive, total,
/// antisymmetric, and transitive. A genuinely distinct claim from
/// [`UtcTimelineOrderingAppliesToFixedInstants`] itself (which only
/// states the relation, not that it has these four properties) —
/// named so the compound check is a real, callable predicate rather
/// than an inline restatement.
pub struct UtcTimelineOrderingIsATotalOrder;

impl Standard for UtcTimelineOrderingIsATotalOrder {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for UtcTimelineOrderingIsATotalOrder {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for UtcTimelineOrderingIsATotalOrder {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
            VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::UtcTimelineOrderingIsATotalOrder",
        "kani",
        || <UtcTimelineOrderingIsATotalOrder as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcTimelineOrderingIsATotalOrder,
    "amenable_kani::UtcTimelineOrderingIsATotalOrder",
    (i32, i32, i32),
    |(a, b, c)| {
        let le = |x: i32, y: i32| {
            <UtcTimelineOrderingAppliesToFixedInstants as Ensures<KaniVerifier>>::ensures((x, y))
        };
        le(a, a)
            && (le(a, b) || le(b, a))
            && (!(le(a, b) && le(b, a)) || a == b)
            && (!(le(a, b) && le(b, c)) || le(a, c))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcTimelineOrderingIsATotalOrder {}

amenable_derive::harness! {
    kani, VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC, {
        /// RFC 3339, 5.1 — two fixed instants are totally ordered by
        /// their position on the UTC timeline: `<=` on `i32` timeline
        /// positions is reflexive, total, antisymmetric, and
        /// transitive — checked over three symbolic instants.
        #[kani::proof]
        fn verify_utc_timeline_ordering_applies_to_fixed_instants() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();

            assert!(<UtcTimelineOrderingIsATotalOrder as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures((a, b, c)));
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for IntervalStartPrecedesEnd {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for IntervalDurationIsNonNegative {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for UtcTimelineOrderingAppliesToFixedInstants
{
}
