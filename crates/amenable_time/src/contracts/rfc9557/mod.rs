//! RFC 9557 (*Date and Time on the Internet: Timestamps with additional
//! information* — IXDTF) propositions (38 in `elicit_temporal`).
//!
//! Tier A throughout — RFC text is freely redistributable, so every
//! contract embeds its clause via `NormativeQuotation::verbatim(..)`
//! with a section-deep rfc-editor link.

mod calendar_and_registry;
mod consumption;
mod syntax;

pub use calendar_and_registry::{
    IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar, IxdtfCalendarAnnotationPresent,
    IxdtfCalendarKeyUsesUCa, IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfExpertReviewAscertainsBasicSpecificationExists,
    IxdtfExpertReviewReservesConciseGenerallyApplicableKeys,
    IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions,
    IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    IxdtfProvisionalEntriesUseExpertReviewPolicy, IxdtfRegistryInitiallyContainsUCaEntry,
    IxdtfUCaRegistryEntryIsPermanent, IxdtfUCaRegistryEntryReferencesSectionFive,
    IxdtfUCaRegistryEntryUsesIetfChangeController,
    IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription,
};
pub use consumption::{
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfDuplicateElectiveSuffixUsesFirstOccurrence, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference,
    IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime,
    IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags, IxdtfRegisteredSuffixKeyCarriesChangeController,
    IxdtfRegisteredSuffixKeyCarriesDescription, IxdtfRegisteredSuffixKeyCarriesKeyIdentifier,
    IxdtfRegisteredSuffixKeyCarriesReference, IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
};
pub use syntax::{
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfExperimentalSuffixKeysAreNotForInterchange, IxdtfExperimentalSuffixKeysCannotBeRegistered,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
};
