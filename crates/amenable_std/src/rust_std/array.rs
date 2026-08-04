//! `RustStdType` registrations for `core::array`.
//!
//! `array::IntoIter<T, const N: usize>` needs a hand-written impl rather
//! than one of this module's macros: none of them model a const generic
//! parameter. Its own struct definition carries no bound on `T` or `N`,
//! so the impl itself is otherwise unremarkable.

use std::array::{IntoIter, TryFromSliceError};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    TryFromSliceError,
    "core",
    "core::array",
    "https://doc.rust-lang.org/core/array/struct.TryFromSliceError.html",
    "The TryFromSliceError carrier reports that a slice's length did not match the fixed-size array being converted into."
);

impl<T, const N: usize> crate::RustStdType for IntoIter<T, N> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::array")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/array/struct.IntoIter.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The IntoIter carrier owns a fixed-size array and yields its elements by value."
    }
}

// `3` is the representative array length this module's proof batch covers.
register_rust_std_standard_evidence!(TryFromSliceError, IntoIter<i32, 3>);
