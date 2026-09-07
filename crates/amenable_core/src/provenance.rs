//! The provenance role: a metadata record that is also a trust basis.

use std::fmt::Display;

use crate::{Metadata, Registry};

/// A [`Metadata`] record whose facts constitute an auditable trust basis.
///
/// `Provenance` is the first link in the constitutional chain `Provenance ->
/// Standard -> Evidence -> Witness`. Where [`Metadata`] is a bag of typed facts
/// making no claim about meaning, a `Provenance` record asserts *why a trust
/// decision is legitimate* -- authority, source, scope, rationale. A plain spec
/// bag such as a widget's `WidgetSpecs` is [`Metadata`] and must **not** be
/// `Provenance`.
///
/// It is a **supertrait** of [`Metadata`], not an associated type: a provenance
/// type *is* its own metadata record, so `provenance.get_as::<T>("...")` and
/// `provenance.report()` work directly. `#[derive(Provenance)]` reuses the
/// `#[derive(Metadata)]` field walk and adds `impl Provenance for T {}` -- the
/// role is today mostly a marker, letting `<P: Provenance>` bounds demand a
/// certified trust basis where `<M: Metadata>` bounds accept any spec bag.
/// Required entries and further methods land as real bounds need them.
pub trait Provenance: Metadata {
    /// Issue a tracked certificate for this trust basis.
    ///
    /// A certificate certifies a *trust basis*, not arbitrary metadata, so this
    /// stays on `Provenance` rather than [`Metadata`].
    fn certification<R>(&self, registry: &mut R, subject: impl Display) -> R::Certificate
    where
        R: Registry,
        Self: Sized,
    {
        registry.issue_provenance_certificate(subject, self)
    }
}
