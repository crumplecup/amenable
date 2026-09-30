//! IXDTF (RFC 9557) suffix-registry timestamp propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    IxdtfCalendarAnnotationPresent, IxdtfCalendarKeyUsesUCa,
    IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfExperimentalSuffixKeysAreNotForInterchange,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    IxdtfExpertReviewReservesConciseGenerallyApplicableKeys, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    IxdtfProvisionalEntriesUseExpertReviewPolicy, IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    IxdtfRegisteredSuffixKeyCarriesChangeController, IxdtfRegisteredSuffixKeyCarriesDescription,
    IxdtfRegisteredSuffixKeyCarriesKeyIdentifier, IxdtfRegisteredSuffixKeyCarriesReference,
    IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent, IxdtfRegistryInitiallyContainsUCaEntry,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    IxdtfUCaRegistryEntryIsPermanent, IxdtfUCaRegistryEntryReferencesSectionFive,
    IxdtfUCaRegistryEntryUsesIetfChangeController, Rfc3339TimestampValid,
};

/// Aggregate proof that an RFC 9557 IXDTF timestamp is structurally valid.
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
pub struct IxdtfTimestampValid {
    /// The base timestamp is a valid RFC 3339 timestamp.
    base: Rfc3339TimestampValid,
    /// Additional information is appended to an RFC 3339 timestamp.
    suffix: IxdtfSuffixFollowsRfc3339Timestamp,
    /// The time-zone annotation uses the RFC 9557 bracketed form.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Criticality is expressed with a leading `!` when present.
    critical: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Additional-information keys obey RFC 9557 casing rules.
    key_case: IxdtfSuffixKeysAreLowercase,
}

/// Aggregate proof that an IXDTF timestamp declares a preferred presentation calendar.
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
pub struct IxdtfTimestampHasPreferredPresentationCalendar {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// A calendar-awareness annotation is present.
    calendar_annotation: IxdtfCalendarAnnotationPresent,
    /// The calendar-awareness key uses the RFC 9557 `u-ca` token.
    calendar_key: IxdtfCalendarKeyUsesUCa,
    /// The calendar value uses a Unicode calendar identifier.
    calendar_identifier: IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
}

/// Aggregate proof that IXDTF additional-information semantics are structurally valid.
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
pub struct IxdtfAdditionalInformationSemanticsValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// Suffix tags use the bracketed key-value form.
    tag_form: IxdtfSuffixTagsUseBracketedKeyValueForm,
    /// Suffix values use one or more hyphen-delimited items.
    value_items: IxdtfSuffixValuesUseHyphenDelimitedItems,
    /// Suffix keys remain lowercase.
    key_case: IxdtfSuffixKeysAreLowercase,
    /// Suffix values are case-sensitive unless a key says otherwise.
    value_case: IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    /// Generators may omit suffix tags entirely.
    optional_generation: IxdtfGeneratorsMayOmitSuffixTags,
    /// Elective suffix tags may be ignored by recipients.
    elective_consumption: IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    /// Criticality is expressed with a leading `!` when present.
    critical_flag: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Critical suffix tags require processing or explicit error handling.
    critical_consumption: IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    /// Experimental suffix keys use a leading underscore.
    experimental_key: IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    /// Experimental suffix keys are not valid for general interchange.
    experimental_interchange: IxdtfExperimentalSuffixKeysAreNotForInterchange,
}

/// Aggregate proof that the initial `u-ca` registry entry semantics are structurally valid.
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
pub struct IxdtfCalendarKeyRegistrySemanticsValid {
    /// The entry satisfies the generic Section 3.2 registry field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// The initial registry contents include a `u-ca` entry.
    registry_entry: IxdtfRegistryInitiallyContainsUCaEntry,
    /// The entry is permanent.
    permanent: IxdtfUCaRegistryEntryIsPermanent,
    /// The entry uses `IETF` as change controller.
    change_controller: IxdtfUCaRegistryEntryUsesIetfChangeController,
    /// The entry references Section 5 of RFC 9557.
    reference: IxdtfUCaRegistryEntryReferencesSectionFive,
}

/// Aggregate proof that a permanent IXDTF suffix-key registration satisfies its reference semantics.
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
pub struct IxdtfPermanentSuffixKeyRegistrationSemanticsValid {
    /// The underlying entry carries the full Section 3.2 field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// Permanent registrations use the permanent/provisional status domain law.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
}

/// Aggregate proof that a provisional IXDTF suffix-key registration satisfies its reference semantics.
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
pub struct IxdtfProvisionalSuffixKeyRegistrationSemanticsValid {
    /// The underlying entry carries the full Section 3.2 field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// Provisional registrations use the permanent/provisional status domain law.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
}

/// Aggregate proof that an IXDTF suffix-key registry entry carries the required field set.
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
pub struct IxdtfSuffixKeyRegistryEntryValid {
    /// The entry carries a key identifier field.
    key_identifier: IxdtfRegisteredSuffixKeyCarriesKeyIdentifier,
    /// The entry carries a registration-status field.
    registration_status: IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    /// The registration status is provisional or permanent.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
    /// The entry carries a description field.
    description: IxdtfRegisteredSuffixKeyCarriesDescription,
    /// The entry carries a change-controller field.
    change_controller: IxdtfRegisteredSuffixKeyCarriesChangeController,
    /// The entry carries a reference field.
    reference: IxdtfRegisteredSuffixKeyCarriesReference,
}

/// Aggregate proof that IXDTF registry-policy semantics are structurally valid.
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
pub struct IxdtfSuffixKeyRegistryPolicySemanticsValid {
    /// Permanent entries use the Specification Required registration policy.
    permanent_policy: IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    /// Provisional entries use the Expert Review registration policy.
    provisional_policy: IxdtfProvisionalEntriesUseExpertReviewPolicy,
    /// Experts reserve concise generally applicable identifiers for broad use.
    frugal_key_allocation: IxdtfExpertReviewReservesConciseGenerallyApplicableKeys,
}
