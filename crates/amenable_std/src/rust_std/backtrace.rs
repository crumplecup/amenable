//! `RustStdType` registrations for `std::backtrace`.
//!
//! `BacktraceFrame` is deliberately not covered here — unstable
//! (`backtrace_frames`).

use std::backtrace::{Backtrace, BacktraceStatus};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    Backtrace,
    "std",
    "std::backtrace",
    "https://doc.rust-lang.org/std/backtrace/struct.Backtrace.html",
    "The Backtrace carrier captures a representation of the call stack at the point it was created."
);

impl_rust_std_type!(
    BacktraceStatus,
    "std",
    "std::backtrace",
    "https://doc.rust-lang.org/std/backtrace/enum.BacktraceStatus.html",
    "The BacktraceStatus carrier reports whether a Backtrace was actually captured, and why not if not."
);

register_rust_std_standard_evidence!(Backtrace, BacktraceStatus);
