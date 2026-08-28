//! Core provenance and role types shared by every `RustStdType` registration.

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use amenable_core::{OwnedProvenanceReport, Provenance as _, Registry};
use amenable_derive::{Provenance, Standard};

/// Shared provenance for Rust-authored documented semantics.
#[derive(Debug, Clone, PartialEq, Eq, Provenance, derive_builder::Builder)]
#[provenance(crate = "amenable_core")]
#[builder(setter(into))]
pub struct RustLanguageProvenance {
    /// The class of authority this provenance record represents.
    authority_kind: String,
    /// The authorizing body for the documented semantics.
    authority: String,
    /// The Rust crate that normatively defines the type.
    source_crate: String,
    /// The Rust module path that normatively defines the type.
    source_module: String,
}

impl RustLanguageProvenance {
    /// Provenance for Rust's primitive carriers as documented through `core`.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn core_primitive() -> Self {
        RustLanguageProvenanceBuilder::default()
            .authority_kind("external_standard")
            .authority("Rust Project Developers")
            .source_crate("core")
            .source_module("core::primitive")
            .build()
            .expect("all fields set")
    }

    /// Provenance for `String` as documented through `alloc`.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn alloc_string() -> Self {
        RustLanguageProvenanceBuilder::default()
            .authority_kind("external_standard")
            .authority("Rust Project Developers")
            .source_crate("alloc")
            .source_module("alloc::string")
            .build()
            .expect("all fields set")
    }

    /// Provenance for a type documented through `source_crate`/`source_module`.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(source_crate, source_module))
    )]
    pub fn for_source(source_crate: impl Into<String>, source_module: impl Into<String>) -> Self {
        RustLanguageProvenanceBuilder::default()
            .authority_kind("external_standard")
            .authority("Rust Project Developers")
            .source_crate(source_crate)
            .source_module(source_module)
            .build()
            .expect("all fields set")
    }
}

/// Structured provenance for a Rust standard-library-backed carrier.
#[derive(Debug, Clone, PartialEq, Eq, Provenance, derive_builder::Builder)]
#[provenance(crate = "amenable_core")]
#[builder(setter(into))]
pub struct RustStdProvenance {
    /// The shared Rust-language provenance this type-specific record relies on.
    rust: RustLanguageProvenance,
    /// The canonical documentation URL for the type.
    source_url: String,
    /// The fully-qualified Rust type name being certified.
    type_name: String,
    /// Concise summary of the semantic promise made by the standard library.
    semantic_summary: String,
}

impl Display for RustStdProvenance {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", OwnedProvenanceReport::new(self.clone()))
    }
}

/// Explicit standard-role wrapper for a Rust standard-library-backed type.
///
/// `T: ?Sized` so unsized carriers (`CStr`, and any other DST) can be
/// wrapped directly: the marker is `PhantomData<*const T>` rather than
/// `PhantomData<fn() -> T>` specifically because a `fn() -> T` function
/// pointer type requires `T: Sized` on its own (you cannot return an
/// unsized value by value) — a restriction independent of, and stricter
/// than, anything this struct itself needs. `*const T` is a fat pointer
/// for unsized `T` and carries the same covariant variance over `T` that
/// `fn() -> T` did, so this is a pure widening, not a behavior change for
/// the sized carriers already using it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new()",
    provenance = "<T as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance",
    bound = "T: RustStdType"
)]
pub struct RustStdStandard<T: ?Sized> {
    _marker: PhantomData<*const T>,
}

impl<T: ?Sized> RustStdStandard<T> {
    /// Promote a Rust standard-library-backed type into the standard role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

/// Provenance helper for Rust standard-library-backed carriers.
///
/// This trait names the authoritative Rust documentation surface and semantic
/// summary for concrete std or core types used as trusted roots.
pub trait RustStdType {
    /// Structured provenance for the documented semantics of this type.
    fn provenance() -> RustStdProvenance {
        RustStdProvenanceBuilder::default()
            .rust(Self::rust_language_provenance())
            .source_url(Self::rust_doc_url())
            .type_name(std::any::type_name::<Self>())
            .semantic_summary(Self::rust_semantics_summary())
            .build()
            .expect("all fields set")
    }

    /// Shared Rust-language provenance for this carrier family.
    fn rust_language_provenance() -> RustLanguageProvenance {
        RustLanguageProvenance::core_primitive()
    }

    /// Render the provenance report for this standard type.
    fn report() -> OwnedProvenanceReport<RustStdProvenance> {
        OwnedProvenanceReport::new(Self::provenance())
    }

    /// Issue a certificate for this trusted standard type through a registry.
    fn certification<R>(registry: &mut R) -> R::Certificate
    where
        Self: Sized,
        R: Registry,
    {
        let provenance = Self::provenance();

        provenance.certification(registry, std::any::type_name::<Self>())
    }

    /// The canonical documentation URL for the type.
    fn rust_doc_url() -> &'static str;

    /// Concise summary of the semantic promise made by the standard library.
    fn rust_semantics_summary() -> &'static str;
}
