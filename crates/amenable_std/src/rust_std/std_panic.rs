//! `RustStdType` registrations for `std::panic`.
//!
//! `PanicInfo` here is a deprecated `TypeAlias` for `PanicHookInfo`,
//! already covered — nothing separate to impl. `BacktraceStyle` is
//! deliberately not covered either — unstable (`panic_backtrace_config`).

use std::panic::PanicHookInfo;

use crate::rust_std::macros::impl_rust_std_type_lifetime0;

impl_rust_std_type_lifetime0!(
    PanicHookInfo,
    "std",
    "std::panic",
    "https://doc.rust-lang.org/std/panic/struct.PanicHookInfo.html",
    "The PanicHookInfo carrier describes an in-progress panic as seen by a panic hook: its message, location, and payload."
);
