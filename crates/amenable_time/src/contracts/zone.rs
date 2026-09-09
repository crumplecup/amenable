//! Time-zone annotation propositions (RFC 9557 §1.2, §3.4, §4.1; the
//! §2.2 update to RFC 3339). Tier A — RFC text is freely
//! redistributable, so every contract embeds its clause via
//! `NormativeQuotation::verbatim(..)`. IANA TZDB naming semantics appear
//! as informative cross-checks.

use crate::NormativeQuotation;

temporal_standard! {
    /// An IXDTF string carries a named time-zone annotation.
    NamedTimeZoneAnnotationPresent => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "the suffix's time-zone slot holds a bracketed IANA time-zone name",
        quotation: NormativeQuotation::verbatim(
            "time-zone         = \"[\" critical-flag time-zone-name / time-numoffset \"]\""
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
    );

    /// A named zone uses an IANA identifier.
    NamedTimeZoneUsesIanaIdentifier => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "the time-zone-name production is intended to hold the name of an IANA time zone",
        quotation: NormativeQuotation::verbatim(
            "time-zone-name is intended to be the name of an IANA Time Zone."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
        cross_check: ("RFC 9557", "1.2", Normative),
    );

    /// A named-zone identifier excludes `.` and `..` segments.
    NamedTimeZoneIdentifierExcludesDotSegments => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "a time-zone-part matching \".\" or \"..\" is explicitly excluded",
        quotation: NormativeQuotation::verbatim(
            "The ABNF definition of time-zone-part matches \".\" and \"..\", which are both explicitly excluded"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
    );

    /// An IXDTF string may carry an offset time-zone annotation.
    OffsetTimeZoneAnnotationPresent => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "the time-zone slot may instead hold a numeric UTC offset used as the zone's name, e.g. [+08:45]",
        quotation: NormativeQuotation::verbatim(
            "Offset Time Zone:  A time zone defined by a specific UTC offset, e.g., +08:45, and serialized using as its name the same numeric UTC offset format used in an [RFC3339] timestamp"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// Named-zone identifiers are case-sensitive.
    NamedTimeZoneIdentifierIsCaseSensitive => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "an IANA time-zone name is matched case-sensitively",
        quotation: NormativeQuotation::verbatim("Keys are lowercase only.  Values are case-sensitive unless otherwise specified."),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
        cross_check: ("IANA TZDB", "tz naming (BCP 175)", Informative),
    );

    /// A named zone is not a numeric-offset alias.
    NamedTimeZoneIsNotNumericOffsetAlias => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "a named time zone is a rule set, not a synonym for a fixed offset — it maps timestamps to offsets over time",
        quotation: NormativeQuotation::verbatim(
            "Mathematically, a time zone can be thought of as a function that maps timestamps to UTC offsets."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A bare numeric offset does not identify a named zone.
    NumericOffsetDoesNotIdentifyNamedZone => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "a timestamp's UTC offset says nothing about the offset of any related timestamp, so it cannot stand for a named zone",
        quotation: NormativeQuotation::verbatim(
            "Unlike the UTC offset of a timestamp, which makes no claims about the UTC offset of other related timestamps (and which is therefore unsuitable for performing local-time operations, such as \"one day later\")"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// An offset time-zone annotation repeats the timestamp offset.
    OffsetTimeZoneRepeatsTimestampOffset => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "an offset in the suffix that does not repeat the timestamp's offset is an inconsistency",
        quotation: NormativeQuotation::verbatim(
            "An offset in the suffix that does not repeat the offset of the timestamp is inconsistent (see Section 3.4)."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
        cross_check: ("RFC 9557", "3.4", Normative),
    );

    /// Offset time zones are strongly discouraged.
    OffsetTimeZoneUseIsStronglyDiscouraged => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "offset time zones exist for backwards compatibility only and their use is strongly discouraged",
        quotation: NormativeQuotation::verbatim(
            "Although serialization with offset time zones is supported in this document for backwards compatibility with java.time.ZonedDateTime [JAVAZDT], use of offset time zones is strongly discouraged."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// An offset zone must not be synthesized from the timestamp offset.
    OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "a program must not copy a timestamp's offset into an offset time-zone suffix to satisfy a consumer that needs one",
        quotation: NormativeQuotation::verbatim(
            "programs MUST NOT copy the UTC offset from a timestamp into an offset time zone in order to satisfy another program that requires a time zone suffix in its input."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A named zone keeps its civil-rule identity beyond the offset.
    NamedTimeZoneRetainsCivilRuleIdentity => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "a named zone also defines how to derive new timestamps from local-time differences, which a bare offset cannot",
        quotation: NormativeQuotation::verbatim(
            "a time zone also defines how to derive new timestamps based on differences in local time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
        cross_check: ("IANA TZDB", "tz semantics", Informative),
    );

    /// The named-zone rules resolve an offset for the instant.
    ZoneOffsetResolvedForRepresentedInstant => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "given an instant, the named zone's rules deterministically yield the UTC offset in effect at that instant",
        quotation: NormativeQuotation::verbatim(
            "Time zones can deterministically convert a timestamp to local time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A named zone is interpreted with current TZDB rules.
    NamedTimeZoneMeaningUsesCurrentTzdbRules => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "a generator and a recipient may hold different TZDB revisions, and the recipient interprets the name against its own",
        quotation: NormativeQuotation::verbatim(
            "As a generator and a recipient may be using different revisions of the Time Zone Database, recipients may not be aware of such an IANA Time Zone name and should treat such a situation as any other inconsistency."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
    );

    /// An unknown zone name (from revision skew) is an inconsistency.
    UnknownNamedTimeZoneIdentifierTreatedAsInconsistency => (
        document: "RFC 9557",
        section: "4.1",
        body: Ietf,
        status: Normative,
        summary: "a name the recipient does not recognise because of TZDB revision skew is handled as any other inconsistency",
        quotation: NormativeQuotation::verbatim(
            "recipients may not be aware of such an IANA Time Zone name and should treat such a situation as any other inconsistency."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-4.1",
        cross_check: ("RFC 9557", "3.4", Normative),
    );

    /// A critical zone-suffix inconsistency requires action.
    CriticalTimeZoneSuffixInconsistencyRequiresAction => (
        document: "RFC 9557",
        section: "3.4",
        body: Ietf,
        status: Normative,
        summary: "if the offset and a critical time-zone suffix disagree, the application must act on the inconsistency",
        quotation: NormativeQuotation::verbatim(
            "In case of an inconsistency between time-offset and time zone suffix, if the critical flag is used on the time zone suffix, an application MUST act on the inconsistency."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.4",
    );

    /// An elective zone-suffix inconsistency may be handled.
    ElectiveTimeZoneSuffixInconsistencyMayBeHandled => (
        document: "RFC 9557",
        section: "3.4",
        body: Ietf,
        status: Normative,
        summary: "if the time-zone suffix is not critical, acting on an offset/suffix inconsistency is permitted but not required",
        quotation: NormativeQuotation::verbatim(
            "If the critical flag is not used, it MAY act on the inconsistency."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.4",
    );

    /// A `Z`-based timestamp with a zone suffix has no inconsistency.
    ZuluTimeZoneSuffixAvoidsOffsetInconsistency => (
        document: "RFC 9557",
        section: "3.4",
        body: Ietf,
        status: Normative,
        summary: "an IXDTF string using Z asserts no local offset, so a named-zone suffix on it cannot be inconsistent",
        quotation: NormativeQuotation::verbatim(
            "The IXDTF timestamps in Figure 2 ... are not inconsistent because they do not assert any particular local time nor local offset in their [RFC3339] part."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.4",
        cross_check: ("RFC 9557", "2.2", Normative),
    );

    /// A local timestamp declares transition ambiguity.
    ZoneTransitionAmbiguityDeclared => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "when a local time maps to multiple instants at a transition, that ambiguity is recorded, not silently resolved",
        quotation: NormativeQuotation::verbatim(
            "some local times may have zero or multiple possible timestamps due to nearby daylight saving time changes or other changes to the UTC offset of that time zone."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A local timestamp declares a transition gap.
    ZoneTransitionGapDeclared => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "when a local time maps to no instant at a transition, that gap is recorded, not silently resolved",
        quotation: NormativeQuotation::verbatim(
            "some local times may have zero or multiple possible timestamps due to nearby daylight saving time changes or other changes to the UTC offset of that time zone."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// Ambiguity resolution uses an explicit disambiguation authority.
    ZoneTransitionDisambiguationAuthorityDeclared => (
        document: "RFC 9557",
        section: "3.4",
        body: Ietf,
        status: Normative,
        summary: "choosing one instant for an ambiguous local time is done under a declared authority (user input or programmed behaviour), not implicitly",
        quotation: NormativeQuotation::verbatim(
            "Acting on the inconsistency may involve rejecting the timestamp or resolving the inconsistency via additional information, such as user input and/or programmed behavior."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.4",
    );

    /// Gap handling uses an explicit gap-handling authority.
    ZoneTransitionGapHandlingAuthorityDeclared => (
        document: "RFC 9557",
        section: "3.4",
        body: Ietf,
        status: Normative,
        summary: "choosing an instant for a local time that falls in a gap is done under a declared authority, not implicitly",
        quotation: NormativeQuotation::verbatim(
            "Acting on the inconsistency may involve rejecting the timestamp or resolving the inconsistency via additional information, such as user input and/or programmed behavior."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-3.4",
    );
}
