//! RFC 9557 the `u-ca` calendar-awareness suffix key (§5) and the IANA
//! Timestamp Suffix Tag Keys registry (§6). See [`super::syntax`] for
//! the tier-A conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A calendar-awareness annotation is present in the suffix.
    IxdtfCalendarAnnotationPresent => (
        document: "RFC 9557",
        section: "5",
        body: Ietf,
        status: Normative,
        summary: "the calendar-awareness annotation appears as a suffix tag on the IXDTF string",
        quotation: NormativeQuotation::verbatim(
            "the suffix key u-ca is allocated to indicate the calendar in which the date/time is preferably presented."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-5",
    );

    /// The calendar-awareness key is `u-ca`.
    IxdtfCalendarKeyUsesUCa => (
        document: "RFC 9557",
        section: "5",
        body: Ietf,
        status: Normative,
        summary: "the suffix key reserved for calendar awareness is u-ca",
        quotation: NormativeQuotation::verbatim(
            "Out of the possible suffix keys, the suffix key u-ca is allocated to indicate the calendar in which the date/time is preferably presented."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-5",
    );

    /// The `u-ca` value is a Unicode Calendar Identifier.
    IxdtfCalendarValueUsesUnicodeCalendarIdentifier => (
        document: "RFC 9557",
        section: "5",
        body: Ietf,
        status: Normative,
        summary: "the value of a u-ca tag is one of the Unicode Calendar Identifier values defined by UTS 35",
        quotation: NormativeQuotation::verbatim(
            "The set of suffix values allowed for this suffix key is the set of values defined for the Unicode Calendar Identifier [TR35]."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-5",
    );

    /// The `u-ca` suffix declares the preferred presentation calendar.
    IxdtfCalendarAnnotationDeclaresPreferredPresentationCalendar => (
        document: "RFC 9557",
        section: "5",
        body: Ietf,
        status: Normative,
        summary: "a u-ca tag states the calendar a calendar-aware consumer should project the instant into for presentation",
        quotation: NormativeQuotation::verbatim(
            "the suffix key u-ca is allocated to indicate the calendar in which the date/time is preferably presented."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-5",
    );

    /// The initial registry contains a `u-ca` entry.
    IxdtfRegistryInitiallyContainsUCaEntry => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the Timestamp Suffix Tag Keys registry is created with one initial entry, u-ca",
        quotation: NormativeQuotation::verbatim(
            "u-ca | Permanent | Preferred Calendar for Presentation | IETF | Section 5 of RFC 9557"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// The initial `u-ca` entry is Permanent.
    IxdtfUCaRegistryEntryIsPermanent => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the initial u-ca registry entry has registration status Permanent",
        quotation: NormativeQuotation::verbatim(
            "u-ca | Permanent | Preferred Calendar for Presentation | IETF | Section 5 of RFC 9557"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// The initial `u-ca` entry's description.
    IxdtfUCaRegistryEntryUsesPreferredCalendarForPresentationDescription => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the initial u-ca registry entry's description is \"Preferred Calendar for Presentation\"",
        quotation: NormativeQuotation::verbatim(
            "u-ca | Permanent | Preferred Calendar for Presentation | IETF | Section 5 of RFC 9557"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// The initial `u-ca` entry's change controller is IETF.
    IxdtfUCaRegistryEntryUsesIetfChangeController => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the initial u-ca registry entry names IETF as its change controller",
        quotation: NormativeQuotation::verbatim(
            "u-ca | Permanent | Preferred Calendar for Presentation | IETF | Section 5 of RFC 9557"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// The initial `u-ca` entry references §5 of RFC 9557.
    IxdtfUCaRegistryEntryReferencesSectionFive => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the initial u-ca registry entry's reference is Section 5 of RFC 9557",
        quotation: NormativeQuotation::verbatim(
            "u-ca | Permanent | Preferred Calendar for Presentation | IETF | Section 5 of RFC 9557"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// Permanent entries use the Specification Required policy.
    IxdtfPermanentEntriesUseSpecificationRequiredPolicy => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "adding a permanent Timestamp Suffix Tag Key requires the Specification Required IANA policy",
        quotation: NormativeQuotation::verbatim(
            "The registration policy [BCP26] is \"Specification Required\" for permanent entries and \"Expert Review\" for provisional ones."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// Provisional entries use the Expert Review policy.
    IxdtfProvisionalEntriesUseExpertReviewPolicy => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "adding a provisional Timestamp Suffix Tag Key requires the Expert Review IANA policy",
        quotation: NormativeQuotation::verbatim(
            "The registration policy [BCP26] is \"Specification Required\" for permanent entries and \"Expert Review\" for provisional ones."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// Expert review checks that a basic specification exists.
    IxdtfExpertReviewAscertainsBasicSpecificationExists => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "the reviewing expert for a provisional key confirms a basic specification exists, even if incomplete or unpublished",
        quotation: NormativeQuotation::verbatim(
            "the experts are instructed to ascertain that a basic specification does exist, even if not complete or published yet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// Concise, generally-applicable key names are held in reserve.
    IxdtfExpertReviewReservesConciseGenerallyApplicableKeys => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "reviewing experts are frugal with short, broadly-meaningful key names, reserving them for keys likely to see wide use",
        quotation: NormativeQuotation::verbatim(
            "the experts are also instructed to be frugal in the allocation of key identifiers that are suggestive of generally applicable semantics, keeping them in reserve"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );

    /// Experts may register a key to avert future collisions.
    IxdtfExpertsMayInitiateRegistrationToAvoidFutureCollisions => (
        document: "RFC 9557",
        section: "6",
        body: Ietf,
        status: Normative,
        summary: "reviewing experts may register a deployed-but-unregistered key on their own initiative to prevent future identifier collisions",
        quotation: NormativeQuotation::verbatim(
            "they may also initiate a registration on their own if they deem such a registration can avert potential future collisions."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-6",
    );
}
