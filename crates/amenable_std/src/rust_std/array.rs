//! `RustStdType` registrations for `core::array`.
//!
//! `array::IntoIter<T, const N: usize>` is deliberately not covered here —
//! none of this module's macros model a const generic parameter.

use std::array::TryFromSliceError;

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    TryFromSliceError,
    "core",
    "core::array",
    "https://doc.rust-lang.org/core/array/struct.TryFromSliceError.html",
    "The TryFromSliceError carrier reports that a slice's length did not match the fixed-size array being converted into."
);

register_rust_std_standard_evidence!(TryFromSliceError);
