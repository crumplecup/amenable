//! `RustStdType` registrations for `core::time`.

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    core::time::Duration,
    "core",
    "core::time",
    "https://doc.rust-lang.org/core/time/struct.Duration.html",
    "The Duration carrier stores a span of time as whole seconds plus a nanosecond fraction."
);

impl_rust_std_type!(
    core::time::TryFromFloatSecsError,
    "core",
    "core::time",
    "https://doc.rust-lang.org/core/time/struct.TryFromFloatSecsError.html",
    "The TryFromFloatSecsError carrier reports that a floating-point seconds value could not be converted into a Duration."
);
