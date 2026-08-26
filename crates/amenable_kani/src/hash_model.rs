//! Kani-only accommodation models for focused `std::hash` laws.
//!
//! The direct `RandomState::new()` path reaches an unsupported OS entropy
//! boundary under Kani before the small Rust-facing determinism law can be
//! checked. This module keeps the narrower contract the production proof
//! actually claims.

use amenable_core::{MetadataEntry, Provenance};
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

impl Provenance for KaniRandomStateObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "two hashers built from the same RandomState instance hash the same input to the same digest",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct RandomState::new path reaches an unsupported OS entropy-source boundary under Kani",
            ),
            MetadataEntry::new("input", self.input.clone()),
            MetadataEntry::new("digest", self.digest.to_string()),
        ]
        .into_iter()
        })
    }
}
