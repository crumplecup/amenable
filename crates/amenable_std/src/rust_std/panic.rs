//! `RustStdType` registrations for `core::panic`.

use std::panic::AssertUnwindSafe;

use core::panic::{Location, PanicInfo, PanicMessage};

use crate::rust_std::macros::{impl_rust_std_type_generic1, impl_rust_std_type_lifetime0};

impl_rust_std_type_generic1!(
    AssertUnwindSafe,
    "core",
    "core::panic",
    "https://doc.rust-lang.org/core/panic/struct.AssertUnwindSafe.html",
    "The AssertUnwindSafe carrier wraps a value and asserts it is safe to use across an unwind boundary, overriding the UnwindSafe auto trait."
);

impl_rust_std_type_lifetime0!(
    Location,
    "core",
    "core::panic",
    "https://doc.rust-lang.org/core/panic/struct.Location.html",
    "The Location carrier records a source file, line, and column, as captured by #[track_caller] or panic!."
);

impl_rust_std_type_lifetime0!(
    PanicInfo,
    "core",
    "core::panic",
    "https://doc.rust-lang.org/core/panic/struct.PanicInfo.html",
    "The PanicInfo carrier describes an in-progress panic: its message, location, and whether the payload can be downcast."
);

impl_rust_std_type_lifetime0!(
    PanicMessage,
    "core",
    "core::panic",
    "https://doc.rust-lang.org/core/panic/struct.PanicMessage.html",
    "The PanicMessage carrier holds the formatted message passed to a panic."
);
