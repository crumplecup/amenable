//! [`TemporalProvenance`] — the provenance record every temporal contract
//! carries, built from the [`provenance_vocab`](crate::provenance_vocab).

use std::fmt::{self, Display, Formatter};

use amenable_core::{Entry as _, Metadata, OwnedEntry, OwnedMetadataReport, Provenance};
use amenable_std::{SemanticSummary, SourceUrl};

use crate::{
    CrossCheck, NormativeDocument, NormativeQuotation, NormativeSection, NormativeStatus,
    StandardsBody,
};

/// Structured provenance for one standards-anchored temporal contract.
///
/// A single flexible record, not one struct per authority
/// (`docs/AMENABLE_TIME_PLAN.md`, "Metadata approach"): the authorities
/// differ only in which values fill the same slots. `SemanticSummary` (our
/// paraphrase) is always present; [`NormativeQuotation`] carries their
/// verbatim text only where the source licence permits it.
///
/// Hand-written `new` + `with_*` setters rather than `derive_builder`:
/// `url` and `cross_checks` are genuinely optional, so a builder here
/// would only ever produce a `.build().expect(...)` that cannot fail —
/// a panic surface for a call that can't panic. The setters chain by
/// value, so [`temporal_standard!`](crate::temporal_standard) builds a
/// record with plain `let` shadowing and no `mut`.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters)]
pub struct TemporalProvenance {
    /// The normative document cited.
    document: NormativeDocument,
    /// The section or clause within it.
    section: NormativeSection,
    /// Whether the primary citation is normative or informative.
    status: NormativeStatus,
    /// The publishing standards body — implies the redistributability tier.
    body: StandardsBody,
    /// Our own concise paraphrase of what the clause requires.
    summary: SemanticSummary,
    /// The verbatim clause text, where the source licence permits it.
    quotation: NormativeQuotation,
    /// A stable deep link to the clause, where one exists.
    url: Option<SourceUrl>,
    /// Secondary citations — informative cross-checks or open-text
    /// stand-ins for a paywalled clause.
    cross_checks: Vec<CrossCheck>,
}

impl TemporalProvenance {
    /// A provenance record with its primary citation set and no URL or
    /// cross-checks yet.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(document, section, summary, quotation))
    )]
    pub fn new(
        document: impl Into<NormativeDocument>,
        section: impl Into<NormativeSection>,
        status: NormativeStatus,
        body: StandardsBody,
        summary: impl Into<SemanticSummary>,
        quotation: NormativeQuotation,
    ) -> Self {
        Self {
            document: document.into(),
            section: section.into(),
            status,
            body,
            summary: summary.into(),
            quotation,
            url: None,
            cross_checks: Vec::new(),
        }
    }

    /// Attach a stable deep link to the cited clause.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, url)))]
    #[must_use]
    pub fn with_url(mut self, url: impl Into<SourceUrl>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Add one secondary citation.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(self, cross_check))
    )]
    #[must_use]
    pub fn with_cross_check(mut self, cross_check: CrossCheck) -> Self {
        self.cross_checks.push(cross_check);
        self
    }
}

impl Metadata for TemporalProvenance {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        let mut entries = vec![
            self.document.clone().into_entry(),
            self.section.clone().into_entry(),
            self.status.into_entry(),
            self.body.into_entry(),
            self.summary.clone().into_entry(),
            self.quotation.clone().into_entry(),
        ];
        if let Some(url) = &self.url {
            entries.push(url.clone().into_entry());
        }
        for (index, cross_check) in self.cross_checks.iter().enumerate() {
            entries.push(OwnedEntry::new(
                format!("cross_check_{index}"),
                cross_check.clone(),
            ));
        }
        entries
    }
}

impl Provenance for TemporalProvenance {}

impl Display for TemporalProvenance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", OwnedMetadataReport::new(self.clone()))
    }
}
