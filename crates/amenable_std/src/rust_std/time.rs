//! `RustStdType` registrations for `core::time`.

use std::time::{Duration, TryFromFloatSecsError};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    Duration,
    "core",
    "core::time",
    "https://doc.rust-lang.org/core/time/struct.Duration.html",
    "The Duration carrier stores a span of time as whole seconds plus a nanosecond fraction."
);

impl_rust_std_type!(
    TryFromFloatSecsError,
    "core",
    "core::time",
    "https://doc.rust-lang.org/core/time/struct.TryFromFloatSecsError.html",
    "The TryFromFloatSecsError carrier reports that a floating-point seconds value could not be converted into a Duration."
);

register_rust_std_standard_evidence!(Duration, TryFromFloatSecsError);
