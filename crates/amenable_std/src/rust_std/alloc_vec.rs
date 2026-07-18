//! `RustStdType` registrations for `alloc::vec`.
//!
//! All default-allocator-only, same tradeoff as `Box` (see
//! `rust_std::alloc_boxed`'s doc comment). `PeekMut` is deliberately not
//! covered — unstable (`vec_peek_mut`).

use std::vec::{Drain as VecDrain, ExtractIf as VecExtractIf, IntoIter as VecIntoIter, Vec};

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    Vec,
    "alloc",
    "alloc::vec",
    "https://doc.rust-lang.org/alloc/vec/struct.Vec.html",
    "The Vec carrier is a growable, heap-allocated, contiguous array."
);

impl<T> crate::RustStdType for VecDrain<'_, T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::vec")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/vec/struct.Drain.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Drain carrier removes and yields a range of a Vec's elements in order."
    }
}

impl_rust_std_type_generic1!(
    VecIntoIter,
    "alloc",
    "alloc::vec",
    "https://doc.rust-lang.org/alloc/vec/struct.IntoIter.html",
    "The IntoIter carrier consumes a Vec, yielding its elements in order."
);

impl<T, F: FnMut(&mut T) -> bool> crate::RustStdType for VecExtractIf<'_, T, F> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::vec")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/vec/struct.ExtractIf.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The ExtractIf carrier lazily removes and yields a Vec's elements matching a predicate."
    }
}

impl<I: Iterator> crate::RustStdType for std::vec::Splice<'_, I> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::vec")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/vec/struct.Splice.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Splice carrier removes a range from a Vec and replaces it with a given iterator's elements, yielding the removed elements."
    }
}

// `i32` (and `'static` for the borrowing carriers) is the representative
// element type/lifetime this module's proof batch covers.
// `IntoIter<i32>` stands in for Splice's replacement-iterator parameter —
// the one concrete iterator type this batch exercises it with.
// `fn(&mut i32) -> bool` stands in for ExtractIf's predicate closure,
// which has no nameable type of its own.
register_rust_std_standard_evidence!(
    Vec<i32>,
    VecDrain<'static, i32>,
    VecIntoIter<i32>,
    VecExtractIf<'static, i32, fn(&mut i32) -> bool>,
    Splice<'static, VecIntoIter<i32>>,
);
