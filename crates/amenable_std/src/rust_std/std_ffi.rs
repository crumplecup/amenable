//! `RustStdType` registrations for `std::ffi` (`OsStr`/`OsString`).
//!
//! `os_str::Display` was previously (incorrectly) documented here as
//! unstable (`os_str_display`) — confirmed stable via a standalone
//! compile check with no `#[feature]` gate; it's covered below.

use std::ffi::os_str::Display;
use std::ffi::{OsStr, OsString};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

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

impl_rust_std_type_lifetime0!(
    Display,
    "std",
    "std::ffi::os_str",
    "https://doc.rust-lang.org/std/ffi/os_str/struct.Display.html",
    "The Display carrier renders an OsStr for user-facing output, lossily substituting invalid UTF-8 where necessary."
);

register_rust_std_standard_evidence!(OsStr, OsString, std::ffi::os_str::Display<'static>);
