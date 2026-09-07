//! Kani-only accommodation models for focused `std::path` laws.
//!
//! The direct `Path::display()` rendering path times out under Kani's native
//! timeout even for a fully concrete UTF-8 literal path, while Windows prefix
//! parsing is host-platform-specific and therefore not executable on this Linux
//! verifier host. This module keeps the smaller Rust-facing path laws the
//! production proofs actually claim.

use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of rendering a valid UTF-8 path through `Display`.
#[derive(Debug, Clone, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::utf8(\"/a/b.txt\")")]
pub struct KaniPathDisplayObservation {
    text: String,
}

impl KaniPathDisplayObservation {
    /// Model a valid UTF-8 path whose `Display` output should be verbatim.
    #[must_use]
    pub fn utf8(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    /// Report the modeled source text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn source_text(&self) -> &str {
        &self.text
    }

    /// Report the modeled `Display` rendering.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn display_text(&self) -> &str {
        &self.text
    }
}

impl Metadata for KaniPathDisplayObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "a path made entirely of valid Unicode renders through Display exactly as its own source text",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct Path::display formatting path times out under Kani even for a fully concrete UTF-8 literal path",
            ),
            OwnedEntry::new("text", self.text.clone()),
        ]
    }
}

impl Provenance for KaniPathDisplayObservation {}

/// Observable result of parsing a Windows drive-letter prefix component.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::disk(\"C:\\\\\", b'C')")]
pub struct KaniWindowsPrefixObservation {
    /// The raw prefix text.
    raw_text: String,
    /// The parsed drive letter.
    #[getter(copy)]
    drive_letter: u8,
}

impl KaniWindowsPrefixObservation {
    /// Model a Windows `Disk` prefix component.
    #[must_use]
    pub fn disk(raw_text: impl Into<String>, drive_letter: u8) -> Self {
        Self {
            raw_text: raw_text.into(),
            drive_letter,
        }
    }
}

impl Metadata for KaniWindowsPrefixObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "a Windows drive-letter prefix component preserves both its raw text and its parsed Disk drive letter",
            ),
            OwnedEntry::new(
                "rationale",
                "direct std::path prefix parsing is host-platform-specific, so the Windows-only path is not executable on this Linux verifier host",
            ),
            OwnedEntry::new("raw_text", self.raw_text.clone()),
            OwnedEntry::new("drive_letter", char::from(self.drive_letter).to_string()),
        ]
    }
}

impl Provenance for KaniWindowsPrefixObservation {}
