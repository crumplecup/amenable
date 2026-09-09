//! RFC 9557 elective-vs-critical consumption (§3.3) and the fields a
//! registered suffix key carries (§3.2). See [`super::syntax`] for the
//! tier-A conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A generator may omit all suffix tags.
    IxdtfGeneratorsMayOmitSuffixTags => (
        document: "RFC 9557",
        section: "3.3",
        body: Ietf,
        status: Normative,
        summary: "suffix tags are always optional to emit; a bare RFC 3339 timestamp is a valid IXDTF string",
        quotation: NormativeQuotation::verbatim(
            "For the IXDTF format, suffix tags are always optional. They can be added or left out as desired by the generator of the string."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.3",
    );

    /// A recipient may ignore an elective suffix tag.
    IxdtfRecipientsMayIgnoreElectiveSuffixTags => (
        document: "RFC 9557",
        section: "3.3",
        body: Ietf,
        status: Normative,
        summary: "unless a tag is marked critical, the recipient is free to ignore it",
        quotation: NormativeQuotation::verbatim(
            "Without further indication, suffix tags are also elective. The recipient is free to ignore any suffix tag included in an IXDTF string."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.3",
    );

    /// A critical suffix tag must be processed or the string rejected.
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling => (
        document: "RFC 9557",
        section: "3.3",
        body: Ietf,
        status: Normative,
        summary: "a recipient that cannot process a critical suffix tag as specified must not act on the string",
        quotation: NormativeQuotation::verbatim(
            "A suffix tag may also indicate that it is critical: The recipient is advised that it MUST NOT act on the IXDTF string unless it can process the suffix tag as specified."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.3",
    );

    /// Duplicate elective keys resolve to the first occurrence.
    IxdtfDuplicateElectiveSuffixUsesFirstOccurrence => (
        document: "RFC 9557",
        section: "3.3",
        body: Ietf,
        status: Normative,
        summary: "when an elective key appears more than once and no inconsistency handling is done, the first occurrence wins",
        quotation: NormativeQuotation::verbatim(
            "An application that encounters duplicate use of a suffix key in elective suffixes and does not want to perform additional processing on this inconsistency MUST choose the first suffix that has that key"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.3",
    );

    /// A registered key carries a key identifier.
    IxdtfRegisteredSuffixKeyCarriesKeyIdentifier => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a registry entry names the key identifier itself, conforming to the suffix-key ABNF",
        quotation: NormativeQuotation::verbatim(
            "Key Identifier:  The key (conforming to suffix-key in Section 4.1)"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A registered key carries a registration status.
    IxdtfRegisteredSuffixKeyCarriesRegistrationStatus => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a registry entry records the key's registration status",
        quotation: NormativeQuotation::verbatim("Registration Status:  \"Provisional\" or \"Permanent\""),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// The registration status is Provisional or Permanent.
    IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a key's registration status is exactly one of Provisional or Permanent",
        quotation: NormativeQuotation::verbatim("Registration Status:  \"Provisional\" or \"Permanent\""),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A registered key carries a description.
    IxdtfRegisteredSuffixKeyCarriesDescription => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a registry entry includes a very brief description of the key",
        quotation: NormativeQuotation::verbatim("Description:  A very brief description of the key"),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A registered key carries a change controller.
    IxdtfRegisteredSuffixKeyCarriesChangeController => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a registry entry names who controls the evolution of the key's value specification",
        quotation: NormativeQuotation::verbatim(
            "Change Controller:  Who is in control of evolving the specification governing values for this key."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A registered key carries a reference.
    IxdtfRegisteredSuffixKeyCarriesReference => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a registry entry includes a reference for the key",
        quotation: NormativeQuotation::verbatim("Reference:  A reference."),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A permanent key's reference is a full specification.
    IxdtfPermanentRegisteredSuffixKeyRequiresFullSpecificationReference => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "for a permanent registration, the reference material is a full specification",
        quotation: NormativeQuotation::verbatim(
            "For permanent tag keys, this includes a full specification."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A provisional key needs at least some reference information.
    IxdtfProvisionalRegisteredSuffixKeyRequiresReferenceInformation => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a provisional registration needs some available reference information, though not a full specification",
        quotation: NormativeQuotation::verbatim(
            "For provisional tag keys, there is an expectation that some information is available even if that does not amount to a full specification"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// A provisional key's reference improves over time.
    IxdtfProvisionalRegisteredSuffixKeyReferenceExpectedToImproveOverTime => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "the registrant of a provisional key is expected to improve its reference information as the key matures",
        quotation: NormativeQuotation::verbatim(
            "the registrant is expected to improve this information over time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );
}
