//! Precision and subsecond-retention propositions.
//!
//! Sources: ISO 8601-1:2019/Amd 1:2022, 5.3.1.4; ISO 8601-2:2019, 7.11,
//! 7.12, 7.13, 14.2-14.4; CalConnect CC 18011:2018,
//! representations-precision / representations-reduced-precision, §8.2,
//! §8.3-§8.5; RFC 3339 §5.6.
//!
//! The first contract module ported from `elicit_temporal` — the
//! pattern-setter for `temporal_standard!`.
//! Four ISO-sourced contracts are tier C (paraphrase only); the one RFC
//! 3339 contract is tier A and embeds its normative ABNF verbatim.

use crate::NormativeQuotation;

temporal_standard! {
    /// Fractional-second precision is explicitly declared when subseconds
    /// are present, rather than left implicit.
    FractionalSecondPrecisionDeclared => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.1.4",
        body: Iso,
        status: Normative,
        summary: "a representation carrying a fractional second explicitly declares its fractional-second precision; precision is never inferred from digit count alone",
        quotation: NormativeQuotation::ParaphraseOnly,
        cross_check: ("RFC 3339", "§5.6", Informative),
    );

    /// Fractional-second digits form a contiguous decimal suffix on the
    /// lowest-order time component.
    FractionalSecondDigitsAreContiguous => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.1.4",
        body: Iso,
        status: Normative,
        summary: "the digits of a fractional second are a single contiguous run immediately after the decimal sign, with no internal separators",
        quotation: NormativeQuotation::ParaphraseOnly,
        cross_check: ("RFC 3339", "§5.6", Informative),
    );

    /// A precision reduction is explicitly declared rather than implied
    /// by omitting lower-order components.
    PrecisionReductionDeclared => (
        document: "ISO 8601-2:2019",
        section: "7.11, 7.13",
        body: Iso,
        status: Normative,
        summary: "reducing the precision of a temporal value is an explicit, declared operation; a shorter representation is not silently treated as a reduced-precision one",
        quotation: NormativeQuotation::ParaphraseOnly,
        cross_check: ("CalConnect CC 18011:2018", "representations-precision, representations-reduced-precision", Informative),
    );

    /// A rounding mode is explicitly declared when precision is reduced.
    RoundingModeDeclared => (
        document: "ISO 8601-2:2019",
        section: "7.13, 14.2, 14.3, 14.4",
        body: Iso,
        status: Normative,
        summary: "when a precision reduction discards significant digits, the rounding mode that governed the reduction is declared, not assumed",
        quotation: NormativeQuotation::ParaphraseOnly,
        cross_check: ("CalConnect CC 18011:2018", "§8.2, §8.3-§8.5", Informative),
    );

    /// Subsecond digits are preserved across an exchange.
    ///
    /// The accord requirement (digits survive a round trip) rests on RFC
    /// 3339's normative `time-secfrac` production, cited verbatim below.
    SubsecondDigitsPreserved => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "an exchange preserves every fractional-second digit it received; the RFC 3339 time-secfrac production admits an arbitrary number of digits and none may be dropped in transit",
        quotation: NormativeQuotation::verbatim("time-secfrac    = \".\" 1*DIGIT"),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );
}
