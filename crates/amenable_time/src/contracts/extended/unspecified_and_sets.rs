//! ISO 8601-2:2019 unspecified-digit (`X`) placeholders and temporal-set
//! representations. See [`super::qualification`] for the tier-C
//! conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// An unspecified digit is written as `X`.
    UnspecifiedDigitUsesUppercaseXPlaceholder => (
        document: "ISO 8601-2:2019",
        section: "4.6.1, 4.6.2, 4.6.3, 9.2.1, 9.2.2, 9.3",
        body: Iso,
        status: Normative,
        summary: "an unspecified digit position is written with the uppercase X placeholder",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Unspecified digit(s); Level 2 - Unspecified Digit", Informative),
    );

    /// An `X` means the value is unspecified.
    UnspecifiedDigitsDeclareUnknownValue => (
        document: "ISO 8601-2:2019",
        section: "4.6.1, 4.6.2, 4.6.3, 9.2.1, 9.2.2, 9.3",
        body: Iso,
        status: Normative,
        summary: "an X placeholder declares that the corresponding digit or component value is not specified",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Unspecified digit(s); Level 2 - Unspecified Digit", Informative),
    );

    /// Level 1 `X` digits are rightmost-only.
    LevelOneUnspecifiedDigitsOccupyRightmostPositions => (
        document: "ISO 8601-2:2019",
        section: "4.6.2, 9.2.1, 9.3",
        body: Iso,
        status: Normative,
        summary: "at Level 1, unspecified digits occupy one or more rightmost positions of a component",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Unspecified digit(s) from the right", Informative),
    );

    /// Level 2 `X` digits may appear anywhere in a component.
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent => (
        document: "ISO 8601-2:2019",
        section: "4.6.3, 9.2.2, 9.3",
        body: Iso,
        status: Normative,
        summary: "at Level 2, unspecified digits may appear at any position within a component, not only the rightmost",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Unspecified Digit", Informative),
    );

    /// A temporal set separates its members explicitly.
    TemporalSetMemberSeparatorDeclared => (
        document: "ISO 8601-2:2019",
        section: "6.1, 6.4",
        body: Iso,
        status: Normative,
        summary: "a temporal set representation separates its member expressions with an explicit separator",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A temporal set carries multiple members.
    TemporalSetCarriesMultipleMembers => (
        document: "ISO 8601-2:2019",
        section: "6.1",
        body: Iso,
        status: Normative,
        summary: "a temporal set holds more than one member temporal expression",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A temporal set declares one-of semantics.
    TemporalSetDeclaresAlternativeSemantics => (
        document: "ISO 8601-2:2019",
        section: "6.2",
        body: Iso,
        status: Normative,
        summary: "a temporal set may declare one-of (alternative) semantics: exactly one member is the intended value",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A one-of temporal set uses square brackets.
    TemporalChoiceSetUsesSquareBrackets => (
        document: "ISO 8601-2:2019",
        section: "6.2",
        body: Iso,
        status: Normative,
        summary: "a one-of (alternative) temporal set is delimited by square brackets",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// An all-of temporal set uses curly braces.
    TemporalInclusiveSetUsesCurlyBraces => (
        document: "ISO 8601-2:2019",
        section: "6.1, 6.4",
        body: Iso,
        status: Normative,
        summary: "an inclusive (all-of) temporal set is delimited by curly braces",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// An inclusive temporal set declares all-members semantics.
    TemporalSetDeclaresInclusiveMemberSemantics => (
        document: "ISO 8601-2:2019",
        section: "6.1, 6.4",
        body: Iso,
        status: Normative,
        summary: "an inclusive temporal set declares that every member is part of the intended value",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A temporal set forbids internal whitespace.
    TemporalSetForbidsInternalWhitespace => (
        document: "ISO 8601-2:2019",
        section: "6.4",
        body: Iso,
        status: Normative,
        summary: "a temporal set expression contains no whitespace between its delimiters, separators, and members",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A `..` set range means the inclusive values between bounds.
    TemporalSetRangeUsesInclusiveDoubleDotSemantics => (
        document: "ISO 8601-2:2019",
        section: "6.3, 6.4",
        body: Iso,
        status: Normative,
        summary: "within a temporal set, `..` between two values denotes every value inclusively between them",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A leading/trailing `..` is an open-ended set boundary.
    TemporalSetOpenRangeUsesBoundaryDoubleDot => (
        document: "ISO 8601-2:2019",
        section: "6.3",
        body: Iso,
        status: Normative,
        summary: "a `..` at the start or end of a temporal set denotes an open-ended boundary",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );

    /// A `..` range shares precision with its neighbours.
    TemporalSetRangeNeighborhoodSharesPrecision => (
        document: "ISO 8601-2:2019",
        section: "6.4, 6.5",
        body: Iso,
        status: Normative,
        summary: "the elements immediately adjacent to a `..` range have the same precision as the values that range denotes",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Set representation", Informative),
    );
}
