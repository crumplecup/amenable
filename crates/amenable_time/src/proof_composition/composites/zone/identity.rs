//! UTC-offset and named-zone identity/revision-tracking propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    NamedTimeZoneIdentifierExcludesDotSegments, NamedTimeZoneIdentifierIsCaseSensitive,
    NamedTimeZoneIsNotNumericOffsetAlias, NamedTimeZoneMeaningUsesCurrentTzdbRules,
    NamedTimeZoneRetainsCivilRuleIdentity, NamedTimeZoneUsesIanaIdentifier,
    NumericOffsetDoesNotIdentifyNamedZone, Rfc3339LocalOffsetNotUnknown,
    TimestampRepresentsFixedInstant, UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
    UtcIsReferenceTimeScale, UtcOffsetCarriesSignHourAndOptionalMinute,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetPrecisionEvidence,
    ZoneOffsetResolvedForRepresentedInstant, ZonedDateTimeHasNamedZone,
};

/// Aggregate proof that a numeric UTC offset is structurally valid.
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
pub struct UtcOffsetValid {
    /// The offset includes a sign, an hour component, and optional minute precision.
    shape: UtcOffsetCarriesSignHourAndOptionalMinute,
    /// The offset hour is in range.
    hour: UtcOffsetHourInRangeZeroToTwentyThree,
    /// The declared precision branch is structurally valid.
    precision: UtcOffsetPrecisionEvidence,
}

/// Aggregate proof that the UTC relationship does not use unknown-local-offset semantics.
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
pub struct UtcOffsetKnown {
    /// The timestamp already carries a valid UTC-offset representation.
    offset: UtcOffsetValid,
    /// The timestamp uses a known local-offset relationship rather than updated unknown-offset semantics.
    known_convention: Rfc3339LocalOffsetNotUnknown,
}

/// Aggregate proof that the UTC reference time scale semantics are established.
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
pub struct UtcTimeScaleValid {
    /// UTC acts as the reference time scale.
    reference: UtcIsReferenceTimeScale,
}

/// Aggregate proof that a named zone carries generic named-zone identity.
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
pub struct NamedTimeZoneIdentityValid {
    /// The zone uses an IANA time-zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The identifier excludes the forbidden `"."` and `".."` path segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// The identifier is interpreted case-sensitively.
    case_sensitivity: NamedTimeZoneIdentifierIsCaseSensitive,
    /// The zone is not merely a numeric-offset alias.
    named_zone: NamedTimeZoneIsNotNumericOffsetAlias,
    /// The zone preserves civil-time rule identity beyond the current offset.
    civil_rules: NamedTimeZoneRetainsCivilRuleIdentity,
}

/// Aggregate proof that named-zone interpretation tracks current TZDB revision semantics.
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
pub struct NamedTimeZoneInterpretationTracksTzdbRevision {
    /// The timestamp already carries named-zone identity.
    zoned: ZonedDateTimeHasNamedZone,
    /// Interpretation follows the TZDB rules current at interpretation time.
    current_rules: NamedTimeZoneMeaningUsesCurrentTzdbRules,
    /// Unknown names under revision skew are treated as inconsistencies.
    unknown_name: UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
}

/// Aggregate proof that the timestamp offset agrees with the named-zone rules.
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
pub struct OffsetConsistentWithNamedZone {
    /// The timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The timestamp carries named-zone identity.
    zoned: ZonedDateTimeHasNamedZone,
    /// The named-zone rules resolve the represented offset.
    resolved_offset: ZoneOffsetResolvedForRepresentedInstant,
}

/// Aggregate proof that offset-only semantics remain weaker than named-zone semantics.
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
pub struct OffsetOnlyZoneSemanticsLimited {
    /// The timestamp carries a valid numeric UTC offset.
    offset: UtcOffsetValid,
    /// A numeric offset alone does not identify a named zone.
    not_a_zone: NumericOffsetDoesNotIdentifyNamedZone,
}
