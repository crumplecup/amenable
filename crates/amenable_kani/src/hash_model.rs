//! Kani-only accommodation models for focused `std::hash` laws.
//!
//! The direct `RandomState::new()` path reaches an unsupported OS entropy
//! boundary under Kani before the small Rust-facing determinism law can be
//! checked. This module keeps the narrower contract the production proof
//! actually claims.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of hashing the same input twice through one `RandomState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::same_input(\"some value\", 7)")]
pub struct KaniRandomStateObservation {
    input: &'static str,
    digest: u64,
}

impl KaniRandomStateObservation {
    /// Model two hashers built from the same `RandomState` on one shared input.
    #[must_use]
    pub fn same_input(input: &'static str, digest: u64) -> Self {
        Self { input, digest }
    }

    /// Report the shared input both hashers saw.
    #[must_use]
    pub fn input(&self) -> &'static str {
        self.input
    }

    /// Report the first observed hash digest.
    #[must_use]
    pub fn first_digest(&self) -> u64 {
        self.digest
    }

    /// Report the second observed hash digest.
    #[must_use]
    pub fn second_digest(&self) -> u64 {
        self.digest
    }

    /// Report whether the two same-input hashes agree.
    #[must_use]
    pub fn same_input_hashes_agree(&self) -> bool {
        self.first_digest() == self.second_digest()
    }
}

impl Provenance for KaniRandomStateObservation {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![
            MetadataEntry::new(
                "assumed",
                "two hashers built from the same RandomState instance hash the same input to the same digest",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct RandomState::new path reaches an unsupported OS entropy-source boundary under Kani",
            ),
            MetadataEntry::new("input", self.input),
            MetadataEntry::new("digest", self.digest.to_string()),
        ]
        .into_iter()
    }
}
