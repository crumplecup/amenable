//! `RustStdType` registrations for `core::hash`.

use std::hash::BuildHasherDefault;

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    BuildHasherDefault,
    "core",
    "core::hash",
    "https://doc.rust-lang.org/core/hash/struct.BuildHasherDefault.html",
    "The BuildHasherDefault carrier constructs a Hasher of type H via H::default() each time one is needed."
);

// Hand-written rather than via impl_rust_std_type!: #[expect(deprecated)]
// needs to attach to the actual impl item, not a macro invocation.
#[expect(
    deprecated,
    reason = "SipHasher itself is stable (only deprecated as a recommendation to use DefaultHasher instead); covering it is a coverage-completeness question, not a call to use it"
)]
impl crate::RustStdType for std::hash::SipHasher {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::hash")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/hash/struct.SipHasher.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The SipHasher carrier implements the SipHash 2-4 algorithm; deprecated in favor of DefaultHasher, but still a real, stable carrier."
    }
}

// `DefaultHasher` is the representative `H` this module's proof batch
// covers `BuildHasherDefault<H>` for. Neither `DefaultHasher` nor
// `SipHasher` needs an import here: `register_rust_std_standard_evidence!`
// only ever stringifies its argument, never uses it in a real type
// position, so nothing gets resolved — and unresolved names don't trigger
// `SipHasher`'s deprecation lint either, confirmed empirically.
register_rust_std_standard_evidence!(BuildHasherDefault<DefaultHasher>, SipHasher,);
