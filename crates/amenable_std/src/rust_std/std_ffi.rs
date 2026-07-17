//! `RustStdType` registrations for `std::ffi` (`OsStr`/`OsString`).
//!
//! `os_str::Display` is deliberately not covered here — unstable
//! (`os_str_display`).

use std::ffi::{OsStr, OsString};

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    OsStr,
    "std",
    "std::ffi",
    "https://doc.rust-lang.org/std/ffi/struct.OsStr.html",
    "The OsStr carrier borrows a platform-native string slice without taking ownership of it."
);

impl_rust_std_type!(
    OsString,
    "std",
    "std::ffi",
    "https://doc.rust-lang.org/std/ffi/struct.OsString.html",
    "The OsString carrier owns a platform-native string, capable of representing what the platform's real filenames/env vars allow."
);
