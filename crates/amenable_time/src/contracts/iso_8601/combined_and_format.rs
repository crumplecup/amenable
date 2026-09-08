//! Combined date-time representations and the format-representation /
//! separator / placeholder rules. Almost all cite only the working draft
//! ISO/WD 8601-1:2016(E) → `status: OpenTextCrossCheck`. See
//! [`super::definitions`] for conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A combined date-time uses the time designator between the parts.
    CombinedDateTimeUsesTimeDesignator => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.2, 4.3.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a combined date-time representation separates its date and time parts with the time designator T",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A combined date-time may use a calendar-date component.
    CombinedDateTimePermitsCalendarDateComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.2 a)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the date part of a combined date-time may be a complete calendar date",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A combined date-time may use an ordinal-date component.
    CombinedDateTimePermitsOrdinalDateComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.2 b)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the date part of a combined date-time may be a complete ordinal date",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A combined date-time may use a week-date component.
    CombinedDateTimePermitsWeekDateComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.2 c)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the date part of a combined date-time may be a complete week date",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A combined date-time's date part is not reduced-accuracy.
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.3 c)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the date part of a combined date-time is complete, never a reduced-accuracy date",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A combined date-time uses one format family throughout.
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.3.3 d)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a combined date-time uses the same format family (basic or extended) for both its date and time parts",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A date-time representation denotes a point, interval, or recurrence.
    DateTimeRepresentationIdentifiesPointIntervalOrRecurrence => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a date-time representation identifies a time point, a time interval, or a recurring time interval",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A complete representation carries every required component.
    CompleteRepresentationCarriesAllRequiredComponents => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.5",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a complete representation includes all date and time components the expression requires",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A format representation describes a family of representations.
    DateTimeFormatRepresentationDescribesRepresentationFamily => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a date-time format representation is a template describing the shape of a group of concrete representations",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A basic format keeps only the components required for accuracy.
    BasicFormatUsesMinimumComponentsForRequiredAccuracy => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a basic format carries the minimum number of components needed for the required accuracy",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-accuracy representation omits lower-order components.
    ReducedAccuracyRepresentationOmitsLowerOrderComponents => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.7",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-accuracy representation is formed by dropping lower-order components",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// An expanded representation needs an additional agreement.
    ExpandedRepresentationRequiresAdditionalAgreement => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.8, 3.5",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "widening a component beyond its standard width (an expanded representation) requires prior agreement between the parties",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Fixed-width components are zero-padded.
    FixedWidthComponentsRequireLeadingZeros => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.6",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a fixed-width component is left-padded with zeros to its full width",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Space is forbidden unless explicitly permitted.
    SpaceForbiddenUnlessExplicitlyPermitted => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a representation contains no space character except where the standard explicitly allows one",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Format representations are not for the ITU-T S.1 telex repertoire.
    DateTimeFormatRepresentationsForbiddenInTelexRepertoire => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "date-time format representations are not used in ITU-T Recommendation S.1 telex-repertoire environments",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// The underline fallback precedes the character it qualifies.
    UnderlineFallbackPrecedesQualifiedFormatCharacter => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "when underlining is unavailable, the substitute underline marker is written immediately before the format character it would have underlined",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Format representations use placeholder characters.
    FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a format representation stands in for digits and signs with designated placeholder characters",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// An underlined placeholder means zero or more digits.
    UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "an underlined digit placeholder in a format representation denotes zero or more digits in the concrete representation",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Non-placeholder characters copy through literally.
    LiteralFormatCharactersCopyIntoRepresentations => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a character in a format representation that is not a placeholder is copied verbatim into the concrete representation",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A week date uses the W designator.
    WeekDateUsesWeekDesignator => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a week-date representation introduces the week component with the designator W",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Hyphens separate date components in extended form.
    HyphenSeparatesDateComponents => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.4",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "in the extended format, adjacent date components are separated by a hyphen",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Colons separate time components in extended form.
    ColonSeparatesTimeComponents => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.4",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "in the extended format, adjacent clock-time components are separated by a colon",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// The UTC designator is uppercase Z.
    UtcDesignatorIsUppercaseZ => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.4.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the UTC designator is the uppercase letter Z",
        quotation: NormativeQuotation::ParaphraseOnly,
    );
}
