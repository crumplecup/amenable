//! `RustStdType` registrations for `alloc::ffi`.

use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    CString,
    "alloc",
    "alloc::ffi",
    "https://doc.rust-lang.org/alloc/ffi/struct.CString.html",
    "The CString carrier owns a nul-terminated, C-compatible string."
);

impl_rust_std_type!(
    FromVecWithNulError,
    "alloc",
    "alloc::ffi",
    "https://doc.rust-lang.org/alloc/ffi/struct.FromVecWithNulError.html",
    "The FromVecWithNulError carrier reports that a byte vector was not exactly nul-terminated as CString::from_vec_with_nul requires."
);

impl_rust_std_type!(
    IntoStringError,
    "alloc",
    "alloc::ffi",
    "https://doc.rust-lang.org/alloc/ffi/struct.IntoStringError.html",
    "The IntoStringError carrier reports that a CString's bytes were not valid UTF-8 when converting it into a String."
);

impl_rust_std_type!(
    NulError,
    "alloc",
    "alloc::ffi",
    "https://doc.rust-lang.org/alloc/ffi/struct.NulError.html",
    "The NulError carrier reports that a byte vector contained an interior nul byte when constructing a CString."
);
