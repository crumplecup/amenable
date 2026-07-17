//! Explicit root-role wrappers and their governing traits.

use std::{fmt::Display, marker::PhantomData};

use crate::{Evidence, MetadataEntry, OwnedProvenanceReport, Provenance, Registry};

/// A provenance-backed root obligation the program claims it will uphold.
///
/// Every `Standard` is also `Evidence`: it terminates an evidence chain by
/// naming itself as its own [`Basis`](Evidence::Basis), since nothing
/// stands behind a root obligation but the provenance that grounds it.
pub trait Standard: Evidence {
    /// Structured provenance for this root obligation.
    type Provenance: Provenance;

    /// Produce the provenance record describing this standard.
    fn provenance(&self) -> Self::Provenance;

    /// Iterate over the projected metadata for the backing provenance.
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        self.provenance().metadata().collect::<Vec<_>>().into_iter()
    }

    /// Look up the fact for a given metadata key.
    fn get(&self, key: &str) -> Option<MetadataEntry> {
        self.provenance().get(key)
    }

    /// Return whether the backing provenance contains a given key.
    fn contains_key(&self, key: &str) -> bool {
        self.provenance().contains_key(key)
    }

    /// Count the projected provenance facts.
    fn len(&self) -> usize {
        self.provenance().len()
    }

    /// Return whether the backing provenance yields no facts.
    fn is_empty(&self) -> bool {
        self.provenance().is_empty()
    }

    /// Iterate over every projected provenance key.
    fn keys(&self) -> impl Iterator<Item = String> {
        self.provenance().keys().collect::<Vec<_>>().into_iter()
    }

    /// Iterate over every projected provenance value.
    fn values(&self) -> impl Iterator<Item = String> {
        self.provenance().values().collect::<Vec<_>>().into_iter()
    }

    /// Render the backing provenance as an owned audit surface.
    fn report(&self) -> OwnedProvenanceReport<Self::Provenance> {
        OwnedProvenanceReport::new(self.provenance())
    }

    /// Issue a tracked provenance certificate for this standard.
    fn certification<R>(&self, registry: &mut R, subject: impl Display) -> R::Certificate
    where
        R: Registry,
    {
        registry.issue_standard_certificate(subject, self)
    }
}

/// Explicit refinement of a type into a standard-root role.
///
/// This wrapper is purely type-level. It does not carry a value. The point is
/// to make the root-role promotion explicit in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsStandard<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> AsStandard<P> {
    /// Promote type `P` into the standard-root role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> From<P> for AsStandard<P> {
    fn from(_value: P) -> Self {
        Self::new()
    }
}
