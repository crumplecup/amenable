//! Serialization-profile propositions: basic vs extended ISO 8601 form,
//! and what a serialization must carry explicitly.
//!
//! Primary source ISO 8601-1:2019 (tier C → `ParaphraseOnly`); the
//! working draft, RFC 3339, and RFC 9557 appear as cross-checks.

use crate::NormativeQuotation;

temporal_standard! {
    /// The basic form omits separators.
    Iso8601BasicFormUsesCompactRepresentation => (
        document: "ISO 8601-1:2019",
        section: "3.1.3, 5.1",
        body: Iso,
        status: Normative,
        summary: "the basic format writes components adjacently, without hyphen or colon separators",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.3.3", OpenTextCrossCheck),
    );

    /// The extended form uses separators.
    Iso8601ExtendedFormUsesSeparators => (
        document: "ISO 8601-1:2019",
        section: "3.1.3, 5.1",
        body: Iso,
        status: Normative,
        summary: "the extended format places hyphens between date components and colons between time components",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.3.4", OpenTextCrossCheck),
    );

    /// The chosen serialization profile is declared.
    SerializationProfileDeclared => (
        document: "ISO 8601-1:2019",
        section: "3.1.3, 5.1",
        body: Iso,
        status: Normative,
        summary: "which profile a value is serialized under (ISO 8601 basic or extended, RFC 3339, RFC 9557 IXDTF) is explicit, not inferred",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("RFC 3339", "5.6", Informative),
    );

    /// The serialization carries an explicit UTC relationship.
    SerializationCarriesExplicitUtcRelationship => (
        document: "RFC 3339",
        section: "4.4",
        body: Ietf,
        status: Normative,
        summary: "a serialized timestamp always includes its offset or Z designator on the wire",
        quotation: NormativeQuotation::verbatim(
            "the interoperability problems of unqualified local time are deemed unacceptable for the Internet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.4",
        cross_check: ("RFC 3339", "5.6", Normative),
    );

    /// An IXDTF serialization keeps the named-zone annotation.
    IxdtfSerializationCarriesNamedZoneAnnotation => (
        document: "RFC 9557",
        section: "3.1",
        body: Ietf,
        status: Normative,
        summary: "when zone identity is being preserved, an IXDTF serialization includes the bracketed named-zone annotation",
        quotation: NormativeQuotation::verbatim(
            "Applications can build an informative timestamp suffix using any number of these tags."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.1",
        cross_check: ("RFC 9557", "3.3", Normative),
    );
}
