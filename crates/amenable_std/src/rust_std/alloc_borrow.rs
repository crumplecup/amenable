//! `RustStdType` registrations for `alloc::borrow`.

use std::borrow::{Cow, ToOwned};

use crate::rust_std::macros::register_rust_std_standard_evidence;

impl<'a, T: ToOwned + ?Sized> crate::RustStdType for Cow<'a, T> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::borrow")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/borrow/enum.Cow.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The Cow carrier holds either a borrowed reference or an owned value, cloning to owned only when mutation is needed."
    }
}

register_rust_std_standard_evidence!(Cow<'static, i32>);
