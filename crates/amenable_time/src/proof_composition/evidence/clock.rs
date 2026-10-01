//! Reduced local-time precision evidence branch.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    MinuteInRangeZeroToFiftyNine, ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
};

/// Evidence branch for the declared reduced local-time precision.
///
/// Normative source: ISO 8601-1:2019, 5.3.1.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.2.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ReducedLocalTimePrecisionEvidence {
    /// The reduced local time uses the hour-only form.
    HourOnly {
        /// The representation uses the hour-only lexical form.
        form: ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
    },
    /// The reduced local time uses the hour-minute form.
    HourMinute {
        /// The representation uses the hour-minute lexical form.
        form: ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
        /// The minute is within the legal ISO 8601 range.
        minute: MinuteInRangeZeroToFiftyNine,
    },
}

impl core::default::Default for ReducedLocalTimePrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::HourOnly {
            form: core::default::Default::default(),
        }
    }
}
