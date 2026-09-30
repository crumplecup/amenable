//! IXDTF time-zone-suffix inconsistency-handling propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    CriticalTimeZoneSuffixInconsistencyRequiresAction,
    ElectiveTimeZoneSuffixInconsistencyMayBeHandled,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    IxdtfTimestampValid, OffsetTimeZoneAnnotationPresent,
    OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset, OffsetTimeZoneRepeatsTimestampOffset,
    OffsetTimeZoneUseIsStronglyDiscouraged, Rfc3339UnknownLocalOffsetUsesZuluDesignator,
    ZoneOffsetResolvedForRepresentedInstant, ZuluTimeZoneSuffixAvoidsOffsetInconsistency,
};

/// Aggregate proof that an offset time-zone annotation is consistent with the timestamp.
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
pub struct OffsetTimeZoneAnnotationConsistentWithTimestamp {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The annotation uses an offset time-zone form.
    offset_zone: OffsetTimeZoneAnnotationPresent,
    /// The suffix offset repeats the RFC 3339 timestamp offset.
    repeated_offset: OffsetTimeZoneRepeatsTimestampOffset,
    /// Offset time zones are only a discouraged compatibility form.
    discouraged_use: OffsetTimeZoneUseIsStronglyDiscouraged,
    /// The offset-zone annotation was not synthesized by copying the timestamp offset.
    not_synthesized: OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset,
}

/// Aggregate proof that a `Z`-based time-zone timestamp avoids offset inconsistency.
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
pub struct ZuluTimeZoneInconsistencyAvoidanceValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The RFC 3339 portion uses `Z` to express that local offset information is unknown.
    unknown_local_offset: Rfc3339UnknownLocalOffsetUsesZuluDesignator,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// The timestamp therefore avoids asserting a conflicting local offset.
    no_inconsistency: ZuluTimeZoneSuffixAvoidsOffsetInconsistency,
    /// The named-zone rules resolve the represented offset for the instant.
    resolved_offset: ZoneOffsetResolvedForRepresentedInstant,
}
/// Aggregate proof that a critical time-zone suffix inconsistency is handled lawfully.
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
pub struct CriticalTimeZoneInconsistencyHandlingValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Criticality is expressed with a leading `!` when present.
    critical_flag: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Critical suffixes require processing or explicit error handling.
    critical_consumption: IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    /// A critical time-zone inconsistency requires the application to act.
    must_act: CriticalTimeZoneSuffixInconsistencyRequiresAction,
}

/// Aggregate proof that an elective time-zone suffix inconsistency is handled lawfully.
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
pub struct ElectiveTimeZoneInconsistencyHandlingValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Elective time-zone suffixes may be ignored by recipients.
    elective_consumption: IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    /// An elective time-zone inconsistency permits, but does not require, action.
    may_act: ElectiveTimeZoneSuffixInconsistencyMayBeHandled,
}
