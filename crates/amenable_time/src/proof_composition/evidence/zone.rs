//! Named-zone/UTC-offset evidence branches.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    IxdtfTimestampValid, NamedTimeZoneAnnotationPresent,
    NamedTimeZoneIdentifierExcludesDotSegments, NamedTimeZoneIdentityValid,
    NamedTimeZoneIsNotNumericOffsetAlias, NamedTimeZoneRetainsCivilRuleIdentity,
    NamedTimeZoneUsesIanaIdentifier, TimestampRepresentsFixedInstant,
    UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets, UtcOffsetMinuteInRangeZeroToFiftyNine,
};

/// Evidence bundle for generic named-zone attachment to a fixed instant.
///
/// Normative sources: RFC 9557 §1.2 and §4.1.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct NamedZoneAttachmentEvidence {
    /// The attached timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The attached zone carries generic named-zone identity semantics.
    zone_identity: NamedTimeZoneIdentityValid,
}
/// Evidence branch for the declared numeric UTC-offset precision.
///
/// Normative source: ISO 8601-1:2019, 5.3.4.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.5.1.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum UtcOffsetPrecisionEvidence {
    /// The offset is expressed with hours only.
    HourOnly {
        /// Minute omission is legal only for integral-hour offsets.
        omitted_minutes: UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    },
    /// The offset is expressed with hour-and-minute precision.
    HourMinute {
        /// The offset minute is in range.
        minute: UtcOffsetMinuteInRangeZeroToFiftyNine,
    },
}

impl core::default::Default for UtcOffsetPrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::HourOnly {
            omitted_minutes: core::default::Default::default(),
        }
    }
}
/// Evidence bundle for named-zone identity.
///
/// Normative sources: RFC 9557 §1.2, §3.3, and §4.1
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ZonedTimestampEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The timestamp carries a named-zone annotation.
    zone_annotation: NamedTimeZoneAnnotationPresent,
    /// The zone annotation uses an IANA zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The identifier excludes the forbidden `"."` and `".."` segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// The annotation is not merely a numeric offset alias.
    named_zone: NamedTimeZoneIsNotNumericOffsetAlias,
    /// The named zone preserves civil-time rule identity.
    civil_rules: NamedTimeZoneRetainsCivilRuleIdentity,
}
