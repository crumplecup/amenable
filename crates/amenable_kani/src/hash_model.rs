//! Kani-only accommodation models for focused `std::hash` laws.
//!
//! The direct `RandomState::new()` path reaches an unsupported OS entropy
//! boundary under Kani before the small Rust-facing determinism law can be
//! checked. This module keeps the narrower contract the production proof
//! actually claims.

use amenable_core::{Metadata, OwnedEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of hashing the same input twice through one `RandomState`.
#[derive(Debug, Clone, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(basis = "Self", basis_ctor = "Self::same_input(\"some value\", 7)")]
pub struct KaniRandomStateObservation {
    /// The shared input both hashers saw.
    input: String,
    /// The shared digest both hashers produced.
    #[getter(skip)]
    digest: u64,
}

impl KaniRandomStateObservation {
    /// Model two hashers built from the same `RandomState` on one shared input.
    #[must_use]
    pub fn same_input(input: impl Into<String>, digest: u64) -> Self {
        Self {
            input: input.into(),
            digest,
        }
    }

    /// Report the first observed hash digest.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn first_digest(&self) -> u64 {
        self.digest
    }

    /// Report the second observed hash digest.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn second_digest(&self) -> u64 {
        self.digest
    }

    /// Report whether the two same-input hashes agree.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn same_input_hashes_agree(&self) -> bool {
        self.first_digest() == self.second_digest()
    }
}

impl Metadata for KaniRandomStateObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new(
                "assumed",
                "two hashers built from the same RandomState instance hash the same input to the same digest",
            ),
            OwnedEntry::new(
                "rationale",
                "the direct RandomState::new path reaches an unsupported OS entropy-source boundary under Kani",
            ),
            OwnedEntry::new("input", self.input.clone()),
            OwnedEntry::new("digest", self.digest.to_string()),
        ]
    }
}

impl Provenance for KaniRandomStateObservation {}
