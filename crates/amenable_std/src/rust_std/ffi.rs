//! `RustStdType` registrations for `core::ffi`.
//!
//! `VaList` is deliberately not covered here — unstable (`c_variadic`) and
//! platform-specific besides. The `c_*` items (`c_char`, `c_int`, ...) are
//! all type aliases to primitives already covered in `rust_std::primitives`
//! — nothing separate to impl for the alias name itself. `c_str::Bytes` is
//! also unstable (`cstr_bytes`, rust-lang/rust#112115).

use core::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError, c_void};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    CStr,
    "core",
    "core::ffi",
    "https://doc.rust-lang.org/core/ffi/struct.CStr.html",
    "The CStr carrier borrows a nul-terminated, C-compatible string without taking ownership of it."
);

impl_rust_std_type!(
    FromBytesUntilNulError,
    "core",
    "core::ffi",
    "https://doc.rust-lang.org/core/ffi/struct.FromBytesUntilNulError.html",
    "The FromBytesUntilNulError carrier reports that a byte slice contained no nul byte when one was required."
);

impl_rust_std_type!(
    FromBytesWithNulError,
    "core",
    "core::ffi",
    "https://doc.rust-lang.org/core/ffi/enum.FromBytesWithNulError.html",
    "The FromBytesWithNulError carrier reports that a byte slice was not exactly nul-terminated as CStr::from_bytes_with_nul requires."
);

impl_rust_std_type!(
    c_void,
    "core",
    "core::ffi",
    "https://doc.rust-lang.org/core/ffi/enum.c_void.html",
    "The c_void carrier is Rust's FFI-compatible representation of C's incomplete void type, used only behind a pointer."
);

// CStr is unsized (a DST): RustStdStandard<T> now supports T: ?Sized (see
// its own doc comment), so CStr gets a proof registered directly, no
// reference wrapper needed. c_void is intentionally excluded here — it is
// only ever used behind a raw pointer and, unlike CStr, has no safe
// constructor or accessor to write a checked proof against.
register_rust_std_standard_evidence!(CStr, FromBytesUntilNulError, FromBytesWithNulError, c_void,);
