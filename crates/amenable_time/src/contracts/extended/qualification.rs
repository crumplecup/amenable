//! ISO 8601-2:2019 qualification (uncertain / approximate / before-after)
//! and enhanced-interval boundary rules.
//!
//! Primary source is ISO 8601-2:2019 — tier C, so
//! `NormativeQuotation::ParaphraseOnly`, `status: Normative`, catalog
//! URL. The publicly-available Library of Congress EDTF profile and the
//! ISO/WD 8601-2:2016(E) working draft appear as `cross_check`s.

use crate::NormativeQuotation;

temporal_standard! {
    /// A temporal expression declares uncertainty explicitly.
    UncertaintyQualificationDeclared => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.1, 8.5",
        body: Iso,
        status: Normative,
        summary: "a temporal expression that is uncertain says so with an explicit uncertainty qualifier, rather than leaving uncertainty implicit",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Qualification of a date (complete)", Informative),
    );

    /// A temporal expression declares approximation explicitly.
    ApproximationQualificationDeclared => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.1, 8.5",
        body: Iso,
        status: Normative,
        summary: "a temporal expression that is approximate says so with an explicit approximation qualifier",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Qualification of a date (complete)", Informative),
    );

    /// Uncertainty and approximation may be combined.
    UncertaintyAndApproximationMayBeCombined => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.1, 8.5",
        body: Iso,
        status: Normative,
        summary: "one temporal expression may carry both an uncertainty and an approximation qualifier at once",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Qualification of a date (complete)", Informative),
    );

    /// A qualification applies to a whole expression or one component.
    QualificationScopeDeclared => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.2, 8.2.3",
        body: Iso,
        status: Normative,
        summary: "a qualification's scope — the entire temporal expression, or a single component — is explicit in how the marker is placed",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Qualification", Informative),
    );

    /// A group qualifier sits immediately right of its component.
    GroupQualificationUsesImmediateRightPlacement => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.2, 8.4.4",
        body: Iso,
        status: Normative,
        summary: "a group qualification marker is written immediately to the right of the component it qualifies from",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Qualification", Informative),
    );

    /// A right-placed group qualifier covers marked-and-left components.
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.2, 8.4.4",
        body: Iso,
        status: Normative,
        summary: "a right-placed group qualifier applies to the marked component and every more-significant component to its left",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Qualification", Informative),
    );

    /// A component qualifier sits immediately left of its component.
    ComponentQualificationUsesImmediateLeftPlacement => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.3, 8.4.5",
        body: Iso,
        status: Normative,
        summary: "an individual-component qualification marker is written immediately to the left of the component it qualifies",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Qualification", Informative),
    );

    /// A left-placed component qualifier covers only that component.
    ComponentQualificationAppliesOnlyToMarkedComponent => (
        document: "ISO 8601-2:2019",
        section: "4.5, 8.2.3, 8.4.5",
        body: Iso,
        status: Normative,
        summary: "a left-placed component qualifier applies only to the single component that immediately follows it",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Qualification", Informative),
    );

    /// Before-or-after qualification is a Level 2 feature.
    BeforeOrAfterQualificationIsLevelTwoOnly => (
        document: "ISO 8601-2:2019",
        section: "4.4.1, 4.4.2",
        body: Iso,
        status: Normative,
        summary: "before-or-after qualification of a date is available only at extension Level 2",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
    );

    /// A leading `..` means before-or-on.
    BeforeOrOnDateUsesLeadingDoubleDotQualifier => (
        document: "ISO 8601-2:2019",
        section: "4.4.2",
        body: Iso,
        status: Normative,
        summary: "a date prefixed with `..` denotes before-or-on that date",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
    );

    /// A trailing `..` means on-or-after.
    OnOrAfterDateUsesTrailingDoubleDotQualifier => (
        document: "ISO 8601-2:2019",
        section: "4.4.2",
        body: Iso,
        status: Normative,
        summary: "a date suffixed with `..` denotes on-or-after that date",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
    );

    /// A Level 1 enhanced interval may qualify a boundary's end.
    EnhancedIntervalLevelOnePermitsTerminalBoundaryQualification => (
        document: "ISO 8601-2:2019",
        section: "4.5.1",
        body: Iso,
        status: Normative,
        summary: "a Level 1 enhanced interval may place an uncertainty or approximation marker at the end of a boundary date",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("ISO/WD 8601-2:2016(E)", "4.5.1", OpenTextCrossCheck),
    );

    /// A Level 2 enhanced interval may qualify parts of a boundary.
    EnhancedIntervalLevelTwoPermitsInternalBoundaryQualification => (
        document: "ISO 8601-2:2019",
        section: "4.5.2",
        body: Iso,
        status: Normative,
        summary: "a Level 2 enhanced interval may mark portions of a boundary date as approximate or uncertain",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("ISO/WD 8601-2:2016(E)", "4.5.2", OpenTextCrossCheck),
    );

    /// A Level 2 enhanced interval may mark boundary digits unspecified.
    EnhancedIntervalLevelTwoPermitsInternalBoundaryUnspecifiedDigits => (
        document: "ISO 8601-2:2019",
        section: "4.5.2",
        body: Iso,
        status: Normative,
        summary: "a Level 2 enhanced interval may mark portions of a boundary date as unspecified digits",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("ISO/WD 8601-2:2016(E)", "4.5.2", OpenTextCrossCheck),
    );

    /// A Level 2 enhanced interval may tag start before / end after.
    EnhancedIntervalLevelTwoPermitsBeforeOrAfterBoundaryQualification => (
        document: "ISO 8601-2:2019",
        section: "4.5.2",
        body: Iso,
        status: Normative,
        summary: "a Level 2 enhanced interval may tag its start boundary as before-or and its end boundary as or-after",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("ISO/WD 8601-2:2016(E)", "4.5.2", OpenTextCrossCheck),
    );

    /// An interval may declare an open boundary.
    OpenIntervalBoundaryDeclared => (
        document: "ISO 8601-2:2019",
        section: "10.2",
        body: Iso,
        status: Normative,
        summary: "an interval may explicitly declare one boundary as open (extending indefinitely) rather than a concrete endpoint",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Extended Interval", Informative),
    );

    /// An interval may declare an unknown boundary.
    UnknownIntervalBoundaryDeclared => (
        document: "ISO 8601-2:2019",
        section: "10.2",
        body: Iso,
        status: Normative,
        summary: "an interval may explicitly declare one boundary as unknown, distinct from open",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Extended Interval", Informative),
    );
}
