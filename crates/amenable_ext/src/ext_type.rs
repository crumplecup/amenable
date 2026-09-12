//! [`ExtType`] and [`ExtStandard<T>`] — the third-party-crate counterpart
//! of `amenable_std::{RustStdType, RustStdStandard<T>}`.

use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use amenable_core::{OwnedMetadataReport, Provenance as _, Registry, SemanticSummary, SourceUrl};
use amenable_derive::{Provenance, Standard};

use crate::{Authority, AuthorityKind, SourceCrate, SourceModule, TypeName};

/// Provenance helper for third-party-crate-backed carriers.
///
/// Unlike `RustStdType::rust_language_provenance`, `ext_language_provenance`
/// has no default implementation: `amenable_std` covers exactly one
/// authority ("Rust Project Developers") for every type it registers, but
/// `amenable_ext` covers a different maintaining project per target crate
/// (jiff, later chrono, …) — there is no universal default to fall back
/// to, so every registration names its own authority explicitly (see
/// [`crate::impl_ext_type`]).
pub trait ExtType {
    /// Structured provenance for the documented semantics of this type.
    fn provenance() -> ExtProvenance {
        ExtProvenance::new(
            Self::ext_language_provenance(),
            Self::ext_doc_url(),
            std::any::type_name::<Self>(),
            Self::ext_semantics_summary(),
        )
    }

    /// The target crate's own language provenance for this carrier family.
    fn ext_language_provenance() -> ExtLanguageProvenance;

    /// Render the provenance report for this third-party type.
    fn report() -> OwnedMetadataReport<ExtProvenance> {
        OwnedMetadataReport::new(Self::provenance())
    }

    /// Issue a certificate for this trusted third-party type through a registry.
    fn certification<R>(registry: &mut R) -> R::Certificate
    where
        Self: Sized,
        R: Registry,
    {
        let provenance = Self::provenance();

        provenance.certification(registry, std::any::type_name::<Self>())
    }

    /// The canonical documentation URL for the type.
    fn ext_doc_url() -> &'static str;

    /// Concise summary of the semantic promise made by the target crate.
    fn ext_semantics_summary() -> &'static str;
}

/// Shared provenance for a third-party crate's own documented semantics.
///
/// `derive_new::new`, not `derive_builder::Builder` — every constructor
/// below sets all four fields unconditionally (no optional fields, no
/// external input to validate), matching `amenable_std::
/// RustLanguageProvenance`'s own reasoning for the same choice.
#[derive(Debug, Clone, PartialEq, Eq, Provenance, derive_new::new)]
#[provenance(crate = "amenable_core")]
pub struct ExtLanguageProvenance {
    /// The class of authority this provenance record represents.
    #[entry(flatten)]
    authority_kind: AuthorityKind,
    /// The authorizing body for the documented semantics.
    #[entry(flatten)]
    authority: Authority,
    /// The third-party crate that normatively defines the type.
    #[entry(flatten)]
    source_crate: SourceCrate,
    /// The module path within that crate that normatively defines the type.
    #[entry(flatten)]
    source_module: SourceModule,
}

impl ExtLanguageProvenance {
    /// Provenance for a type documented through `authority`/`source_crate`/
    /// `source_module`. Every field is explicit — see [`ExtType`]'s own
    /// doc comment for why there is no crate-wide default the way
    /// `RustLanguageProvenance::core_primitive` has for `amenable_std`.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(authority, source_crate, source_module))
    )]
    pub fn for_source(
        authority: impl Into<String>,
        source_crate: impl Into<String>,
        source_module: impl Into<String>,
    ) -> Self {
        Self::new(
            AuthorityKind::ExternalStandard,
            Authority::new(authority),
            SourceCrate::new(source_crate),
            SourceModule::new(source_module),
        )
    }
}

/// Structured provenance for a third-party-crate-backed carrier.
///
/// `derive_new::new`, not `derive_builder::Builder` — see
/// [`ExtLanguageProvenance`]'s own doc comment for why: every caller sets
/// all four fields unconditionally.
#[derive(Debug, Clone, PartialEq, Eq, Provenance, derive_new::new)]
#[provenance(crate = "amenable_core")]
pub struct ExtProvenance {
    /// The target crate's own language provenance this type-specific
    /// record relies on.
    #[entry(nested)]
    language: ExtLanguageProvenance,
    /// The canonical documentation URL for the type.
    #[entry(flatten)]
    #[new(into)]
    source_url: SourceUrl,
    /// The fully-qualified Rust type name being certified.
    #[entry(flatten)]
    #[new(into)]
    type_name: TypeName,
    /// Concise summary of the semantic promise made by the target crate.
    #[entry(flatten)]
    #[new(into)]
    semantic_summary: SemanticSummary,
}

impl Display for ExtProvenance {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", OwnedMetadataReport::new(self.clone()))
    }
}

/// Explicit standard-role wrapper for a third-party-crate-backed type.
///
/// `T: ?Sized` for the same reason `RustStdStandard<T>` is — see that
/// type's own doc comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new()",
    provenance = "<T as ExtType>::provenance()",
    provenance_type = "ExtProvenance",
    bound = "T: ExtType"
)]
pub struct ExtStandard<T: ?Sized> {
    _marker: PhantomData<*const T>,
}

impl<T: ?Sized> ExtStandard<T> {
    /// Promote a third-party-crate-backed type into the standard role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
