//! RFC 9557 IXDTF suffix syntax (§3.1, §4.1) and experimental-key rules
//! (§3.2). Tier A — RFC text is freely redistributable, so every
//! contract embeds its clause via `NormativeQuotation::verbatim(..)`.

use crate::NormativeQuotation;

temporal_standard! {
    /// An IXDTF string is an RFC 3339 timestamp plus a suffix.
    IxdtfSuffixFollowsRfc3339Timestamp => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "an IXDTF string is an RFC 3339 date-time immediately followed by an optional suffix",
        quotation: NormativeQuotation::verbatim(
            "date-time-ext     = date-time suffix\nsuffix            = [time-zone] *suffix-tag"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
    );

    /// A time-zone annotation is bracketed name or offset.
    IxdtfTimeZoneSuffixUsesBracketedNameOrOffset => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "the time-zone annotation is a bracketed IANA time-zone name or a numeric offset, with no equals sign",
        quotation: NormativeQuotation::verbatim(
            "time-zone         = \"[\" critical-flag time-zone-name / time-numoffset \"]\""
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
    );

    /// A critical annotation opens with `!`.
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent => (
        document: "RFC 9557",
        section: "3.3",
        body: Ietf,
        status: Normative,
        summary: "a critical annotation is marked by an exclamation mark immediately after the opening bracket",
        quotation: NormativeQuotation::verbatim(
            "A critical suffix tag is indicated by following its opening bracket with an exclamation mark (see critical-flag in Section 4.1)."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.3",
        cross_check: ("RFC 9557", "4.1", Normative),
    );

    /// Suffix keys are lowercase.
    IxdtfSuffixKeysAreLowercase => (
        document: "RFC 9557",
        section: "3.1",
        body: Ietf,
        status: Normative,
        summary: "an extended-information key contains only lowercase letters",
        quotation: NormativeQuotation::verbatim("Keys are lowercase only."),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.1",
    );

    /// A suffix tag is `[key=value]`.
    IxdtfSuffixTagsUseBracketedKeyValueForm => (
        document: "RFC 9557",
        section: "3.1",
        body: Ietf,
        status: Normative,
        summary: "a suffix tag is a bracketed key and value joined by an equals sign",
        quotation: NormativeQuotation::verbatim(
            "This is done by defining tags, each with a key and a value separated by an equals sign."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.1",
        cross_check: ("RFC 9557", "4.1", Normative),
    );

    /// A suffix value is hyphen-delimited items.
    IxdtfSuffixValuesUseHyphenDelimitedItems => (
        document: "RFC 9557",
        section: "3.1",
        body: Ietf,
        status: Normative,
        summary: "a suffix value is one or more items separated by hyphen/minus signs",
        quotation: NormativeQuotation::verbatim(
            "The value of a tag can be one or more items delimited by hyphen/minus signs."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.1",
        cross_check: ("RFC 9557", "4.1", Normative),
    );

    /// Suffix values are case-sensitive by default.
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified => (
        document: "RFC 9557",
        section: "3.1",
        body: Ietf,
        status: Normative,
        summary: "a suffix value is case-sensitive unless the key's own specification says otherwise",
        quotation: NormativeQuotation::verbatim(
            "Values are case-sensitive unless otherwise specified."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.1",
    );

    /// Experimental keys start with an underscore.
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "a key beginning with an underscore is an experimental key for controlled environments",
        quotation: NormativeQuotation::verbatim(
            "Key names that start with an underscore are intended for experiments in controlled environments and cannot be registered"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
        cross_check: ("RFC 9557", "4.1", Normative),
    );

    /// Experimental keys are not for interchange.
    IxdtfExperimentalSuffixKeysAreNotForInterchange => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "an experimental (underscore-prefixed) key must not be used in general interchange",
        quotation: NormativeQuotation::verbatim(
            "such keys MUST NOT be used for interchange and MUST be rejected by implementations not specifically configured to take part in such an experiment."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// Recipients must reject unconfigured experimental keys.
    IxdtfRecipientsMustRejectUnconfiguredExperimentalSuffixKeys => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "an implementation not part of the relevant experiment rejects an experimental key it receives",
        quotation: NormativeQuotation::verbatim(
            "such keys MUST NOT be used for interchange and MUST be rejected by implementations not specifically configured to take part in such an experiment."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );

    /// Experimental keys cannot be registered.
    IxdtfExperimentalSuffixKeysCannotBeRegistered => (
        document: "RFC 9557",
        section: "3.2",
        body: Ietf,
        status: Normative,
        summary: "an underscore-prefixed experimental key is never entered in the Timestamp Suffix Tag Keys registry",
        quotation: NormativeQuotation::verbatim(
            "Key names that start with an underscore are intended for experiments in controlled environments and cannot be registered"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.2",
    );
}
