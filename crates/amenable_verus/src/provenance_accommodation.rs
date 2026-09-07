//! Hand-trimmed accommodation mirror of `amenable_core`'s `Provenance` /
//! `Standard` layer -- the same pattern as [`witness_accommodation`](crate::
//! witness_accommodation), and for the same reason.
//!
//! The real `Provenance` is now `trait Provenance: Metadata`, and `Metadata`
//! (plus `OwnedMetadataReport`, which `Standard::report` returns) lives in
//! `amenable_core/src/metadata.rs`, which uses `Arc<dyn ...>` and
//! `core::any::Any`. This crate cannot `#[path]`-include that file: the raw
//! `verus` compiler has no need for the live typed-metadata machinery, and
//! `metadata.rs` was deliberately kept out of the `#[path]` set for exactly
//! this migration. So `provenance.rs` and `roles.rs` are no longer
//! mod-included verbatim -- these trimmed definitions stand in.
//!
//! Nothing in this crate's proofs reasons about provenance or standards.
//! `GalleryVerifierMetadata`'s `Provenance` impl exists only to satisfy
//! `Verifier::Metadata`'s bound; `Standard` is named only as a bound in
//! `cert.rs`'s `Registry::issue_standard_certificate`.

use std::marker::PhantomData;

use crate::Evidence;

/// Marker mirror of `amenable_core::Provenance` (real trait: `Provenance:
/// Metadata`, with a `certification()` default method).
pub trait Provenance {}

/// Trimmed mirror of `amenable_core::Standard`. The real trait also carries
/// `report()` and `certification()` default methods; only `provenance()` is
/// referenced from a `#[path]`-included file (`cert.rs`).
pub trait Standard: Evidence {
    /// Structured provenance for this root obligation.
    type Provenance: Provenance;

    /// Produce the provenance record describing this standard.
    fn provenance(&self) -> Self::Provenance;
}

/// Verbatim copy of `amenable_core::AsStandard`.
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
