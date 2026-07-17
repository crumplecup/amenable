//! `RustStdType` registrations for `alloc::borrow`.

use std::borrow::{Cow, ToOwned};

impl<'a, T: ToOwned + ?Sized> crate::RustStdType for Cow<'a, T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::borrow")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/borrow/enum.Cow.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Cow carrier holds either a borrowed reference or an owned value, cloning to owned only when mutation is needed."
    }
}
