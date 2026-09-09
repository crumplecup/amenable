//! Cross-authority conversion law: what a conversion between temporal
//! representations must declare, what it must preserve, and when it
//! needs explicit authority to lose information.
//!
//! Mixed tier: RFC 3339 §5.1 citations are tier A (verbatim); ISO
//! 8601-1/-2 citations are tier C (`ParaphraseOnly`).

use crate::NormativeQuotation;

temporal_standard! {
    /// The source semantic kind is declared.
    ConversionSourceSemanticKindDeclared => (
        document: "ISO 8601-1:2019",
        section: "3.1.3, 5.2-5.6",
        body: Iso,
        status: Normative,
        summary: "a conversion states the temporal semantic kind of its input (instant, local date-time, date, duration, ...) explicitly",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// The target semantic kind is declared.
    ConversionTargetSemanticKindDeclared => (
        document: "ISO 8601-1:2019",
        section: "3.1.3, 5.2-5.6",
        body: Iso,
        status: Normative,
        summary: "a conversion states the temporal semantic kind of its output explicitly",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A conversion preserves the represented instant.
    ConversionPreservesRepresentedInstant => (
        document: "RFC 3339",
        section: "5.1",
        body: Ietf,
        status: Normative,
        summary: "an instant-preserving conversion yields an output denoting the same point on the UTC timeline as its input",
        quotation: NormativeQuotation::verbatim(
            "a time-ordered sequence will result.  The presence of optional punctuation would violate this characteristic."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.1",
    );

    /// A conversion preserves temporal ordering.
    ConversionPreservesTemporalOrdering => (
        document: "RFC 3339",
        section: "5.1",
        body: Ietf,
        status: Normative,
        summary: "an order-preserving conversion keeps the relative order of any two inputs on the UTC timeline",
        quotation: NormativeQuotation::verbatim(
            "the date and time strings may be sorted as strings (e.g., using the strcmp() function in C) and a time-ordered sequence will result."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.1",
    );

    /// A conversion drops named-zone identity.
    ConversionDropsNamedZoneIdentity => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "converting to a bare offset loses the named time zone — a rule set — keeping only the offset at that instant",
        quotation: NormativeQuotation::verbatim(
            "Unlike the UTC offset of a timestamp, which makes no claims about the UTC offset of other related timestamps (and which is therefore unsuitable for performing local-time operations, such as \"one day later\"), a time zone also defines how to derive new timestamps based on differences in local time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A conversion drops subsecond precision.
    ConversionDropsSubsecondPrecision => (
        document: "ISO 8601-2:2019",
        section: "7.11, 7.12, 7.13",
        body: Iso,
        status: Normative,
        summary: "a precision-reducing conversion may discard fractional-second digits present in the input",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("CalConnect CC 18011:2018", "representations-precision, representations-reduced-precision", Informative),
    );

    /// A lossy conversion needs explicit authority.
    ConversionRequiresExplicitAuthorityWhenLossy => (
        document: "ISO 8601-2:2019",
        section: "7.13, 14.2, 14.3, 14.4",
        body: Iso,
        status: Normative,
        summary: "a conversion that loses information (precision, zone identity, ...) is performed only under a declared authority, never as a silent coercion",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("CalConnect CC 18011:2018", "8.2, 8.3-8.5", Informative),
    );
}
