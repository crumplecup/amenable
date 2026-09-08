//! The temporal provenance vocabulary: canonically-keyed
//! [`Entry`](amenable_core::Entry) types every contract's
//! [`TemporalProvenance`](crate::TemporalProvenance) record is built from.
//!
//! This vocabulary is about *standards documents* — which normative text a
//! contract encodes, and how much of that text may be reproduced.
//! `amenable_std::provenance_vocab` is about *carrier libraries* (which
//! Rust crate/module defines a type); the two are deliberately separate
//! (`docs/AMENABLE_TIME_PLAN.md`, decision 1). Only `SourceUrl` and
//! `SemanticSummary` are shared, imported from `amenable_std`.
//!
//! # The three-tier quotation rule
//!
//! How much normative text a contract embeds is governed by the source's
//! redistributability, expressed through [`NormativeQuotation`]:
//!
//! - **Tier A** (RFC 3339 / RFC 9557 under IETF Trust, LoC EDTF, IANA
//!   TZDB): [`NormativeQuotation::Verbatim`] — the specific clause, never
//!   a whole section.
//! - **Tier B** (CalConnect CC 18011 / 18012): `Verbatim`, limited to the
//!   single relevant clause.
//! - **Tier C** (ISO 8601-1 / -2 / Amd 1, the ISO working drafts —
//!   paywalled): [`NormativeQuotation::ParaphraseOnly`]. No ISO prose is
//!   reproduced anywhere in this repo; the `SemanticSummary` paraphrase
//!   and the exact `NormativeSection` pointer carry the citation.

use amenable_derive::Entry;

/// Define a `String`-valued vocabulary [`Entry`](amenable_core::Entry): a
/// `#[derive(Entry)]` newtype with `new` / `From<&str>` wrappers. Mirrors
/// `amenable_std::provenance_vocab`'s own `string_vocab_entry!`.
macro_rules! vocab_str {
    ($(#[$doc:meta])* $name:ident, $key:literal) => {
        $(#[$doc])*
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Entry,
            derive_more::Display,
            derive_more::From,
        )]
        #[entry(key = $key, crate = "amenable_core")]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Create a `", stringify!($name), "` fact.")]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }
    };
}

vocab_str! {
    /// The normative document a contract cites, e.g. `"ISO 8601-1:2019"`,
    /// `"RFC 3339"`, `"CalConnect CC 18011:2018"`.
    NormativeDocument, "normative_document"
}
vocab_str! {
    /// The section or clause number within the normative document, e.g.
    /// `"5.2.2"`, `"§5.6"`, `"representations-precision"`.
    NormativeSection, "normative_section"
}

/// The verbatim normative text a contract encodes — governed by the
/// three-tier redistributability rule (see the module docs).
///
/// This is *their* words. The contract's own `SemanticSummary` is *our*
/// paraphrase and is present regardless of tier.
#[derive(Debug, Clone, PartialEq, Eq, Entry, derive_more::Display)]
#[entry(key = "normative_quotation", crate = "amenable_core")]
pub enum NormativeQuotation {
    /// The exact clause text, reproduced verbatim. Tiers A and B only
    /// (freely redistributable, or single-clause from a public spec).
    #[display("{_0}")]
    Verbatim(String),
    /// The source is paywalled (ISO), so no prose is reproduced. The
    /// `SemanticSummary` paraphrase and `NormativeSection` pointer stand
    /// in. Tier C.
    #[display("(paraphrase only — licensed source; see semantic_summary)")]
    ParaphraseOnly,
    /// No accessible normative source text exists for this contract at
    /// all (rare — a claim resting only on committee convention).
    #[display("(normative source text unavailable)")]
    Unavailable,
}

impl NormativeQuotation {
    /// A tier A/B verbatim clause excerpt.
    pub fn verbatim(text: impl Into<String>) -> Self {
        Self::Verbatim(text.into())
    }
}

/// Whether a citation is normative, informative, or a public-draft
/// cross-check standing in for a paywalled clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Entry, derive_more::Display)]
#[entry(key = "normative_status", crate = "amenable_core")]
pub enum NormativeStatus {
    /// A binding requirement in the cited document.
    #[display("normative")]
    Normative,
    /// A non-binding note or example in the cited document.
    #[display("informative")]
    Informative,
    /// A publicly-available draft or mirror cited because the published,
    /// authoritative text is paywalled.
    #[display("open-text cross-check")]
    OpenTextCrossCheck,
}

/// The standards body that publishes a cited document — determines the
/// redistributability tier (see the module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Entry, derive_more::Display)]
#[entry(key = "standards_body", crate = "amenable_core")]
pub enum StandardsBody {
    /// International Organization for Standardization — paywalled (tier C).
    #[display("ISO")]
    Iso,
    /// Internet Engineering Task Force — freely redistributable (tier A).
    #[display("IETF")]
    Ietf,
    /// The Calendaring and Scheduling Consortium — public spec (tier B).
    #[display("CalConnect")]
    CalConnect,
    /// Internet Assigned Numbers Authority (the TZDB) — public domain (tier A).
    #[display("IANA")]
    Iana,
    /// Bureau International des Poids et Mesures — the SI second, leap seconds.
    #[display("BIPM")]
    Bipm,
    /// The Library of Congress — the EDTF specification (tier A).
    #[display("Library of Congress")]
    LibraryOfCongress,
}

/// A secondary citation supporting a contract's primary normative source
/// — an informative cross-check, or a public draft standing in for a
/// paywalled clause.
#[derive(Debug, Clone, PartialEq, Eq, derive_new::new, derive_getters::Getters)]
pub struct CrossCheck {
    /// The cross-checked document.
    #[new(into)]
    document: String,
    /// The section within it.
    #[new(into)]
    section: String,
    /// Whether the cross-check is informative or an open-text stand-in.
    status: NormativeStatus,
}

impl std::fmt::Display for CrossCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.document, self.section, self.status)
    }
}
