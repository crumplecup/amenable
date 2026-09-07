//! Explicit root-role wrappers and their governing traits.

use std::{fmt::Display, marker::PhantomData};

use crate::{Evidence, OwnedMetadataReport, Provenance, Registry};

/// A provenance-backed root obligation the program claims it will uphold.
///
/// Every `Standard` is also `Evidence`: it terminates an evidence chain by
/// naming itself as its own [`Basis`](Evidence::Basis), since nothing
/// stands behind a root obligation but the provenance that grounds it.
/// Deliberately verifier-agnostic, like `Evidence` itself — a `Standard`
/// used purely for provenance/audit reporting (see `Evidence`'s own doc
/// comment) has no business requiring a proof under any particular backend
/// just to compile. Where a `Standard`-backed value genuinely needs to
/// carry a real proof, that requirement is stated explicitly at the
/// `Sidecar`/`Exchange` level that actually needs it, not baked in here.
///
/// The metadata query surface is **not** re-exposed here — reach the facts
/// through `standard.provenance()`, which is itself a [`Metadata`](crate::
/// Metadata) record.
pub trait Standard: Evidence {
    /// Structured provenance for this root obligation.
    type Provenance: Provenance;

    /// Produce the provenance record describing this standard.
    fn provenance(&self) -> Self::Provenance;

    /// Render the backing provenance as an owned audit surface.
    fn report(&self) -> OwnedMetadataReport<Self::Provenance> {
        OwnedMetadataReport::new(self.provenance())
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
