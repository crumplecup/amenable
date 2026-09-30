//! RFC 3339 timestamp propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    OffsetDateTimeIdentifiesSingleInstant, OffsetDateTimeValid,
    Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator, Rfc3339FractionUsesDotSeparator,
    Rfc3339FractionalSecondsAreOnlyRarelyUsedOption, Rfc3339GeneratorsShouldUseUppercaseTAndZ,
    Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
    Rfc3339LocalityDisplayMayTranslateUtcToLocalTime, Rfc3339OffsetIsUtcOrNumeric,
    Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory, Rfc3339RequiresUtcRelationship,
    Rfc3339TimestampExcludesRedundantWeekdayInformation,
    Rfc3339UnknownLocalOffsetUsesZuluDesignator, Rfc3339UnqualifiedLocalTimeForbidden,
    Rfc3339UsesExtendedCalendarDate, Rfc3339UsesFourDigitYear, Rfc3339UsesFullTime,
    TimestampHasExplicitUtcOffset,
};

/// Aggregate proof that a timestamp denotes a single fixed instant.
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
pub struct TimestampRepresentsFixedInstant {
    /// The offset date-time representation is structurally valid.
    offset_date_time: OffsetDateTimeValid,
    /// The timestamp carries an explicit UTC relationship.
    explicit_offset: TimestampHasExplicitUtcOffset,
    /// The representation identifies a single instant on the UTC timeline.
    single_instant: OffsetDateTimeIdentifiesSingleInstant,
}

/// Aggregate proof that an RFC 3339 timestamp is structurally valid.
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
pub struct Rfc3339TimestampValid {
    /// The timestamp rests on a valid offset date-time representation.
    offset_date_time: OffsetDateTimeValid,
    /// The year uses the RFC 3339 four-digit profile.
    year: Rfc3339UsesFourDigitYear,
    /// The date uses the RFC 3339 `full-date` profile.
    full_date: Rfc3339UsesExtendedCalendarDate,
    /// The time uses the RFC 3339 `full-time` profile.
    full_time: Rfc3339UsesFullTime,
    /// The timestamp does not carry redundant weekday information.
    no_redundant_weekday: Rfc3339TimestampExcludesRedundantWeekdayInformation,
    /// The timestamp carries an explicit relationship to UTC.
    utc_relationship: Rfc3339RequiresUtcRelationship,
    /// Local time without an offset or `Z` is forbidden.
    no_unqualified_local: Rfc3339UnqualifiedLocalTimeForbidden,
    /// Fractional seconds use `.` rather than the broader ISO 8601 decimal options.
    secfrac_separator: Rfc3339FractionUsesDotSeparator,
    /// The UTC relationship is encoded as `Z` or a numeric offset.
    offset_encoding: Rfc3339OffsetIsUtcOrNumeric,
    /// Unknown local-offset semantics use the `Z` designator rather than legacy `-00:00`.
    unknown_offset: Rfc3339UnknownLocalOffsetUsesZuluDesignator,
}

/// Aggregate proof that RFC 3339 display-localization guidance is structurally valid.
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
pub struct Rfc3339DisplayGuidanceValid {
    /// Locality-oriented display transformation may translate UTC timestamps into local time.
    utc_to_local_translation: Rfc3339LocalityDisplayMayTranslateUtcToLocalTime,
}

/// Aggregate proof that RFC 3339 generation guidance is structurally valid.
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
pub struct Rfc3339GenerationGuidanceValid {
    /// Fractional seconds are the only rarely used profile option.
    secfrac_rarity: Rfc3339FractionalSecondsAreOnlyRarelyUsedOption,
    /// Generators should prefer uppercase `T` and `Z`.
    uppercase_tz: Rfc3339GeneratorsShouldUseUppercaseTAndZ,
    /// Application profiles may choose to allow a readability-motivated space separator.
    space_separator_option: Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator,
    /// Inserted leap-second timestamps are not generated before announcement.
    leap_second_announcement: Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
}

/// Aggregate proof that RFC 3339 lexical ordering preconditions are structurally valid.
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
pub struct Rfc3339LexicalOrderingSemanticsValid {
    /// The underlying timestamp shape is a valid RFC 3339 timestamp.
    timestamp: Rfc3339TimestampValid,
    /// Most fields and punctuation are mandatory in the profile.
    mandatory_shape: Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory,
}
