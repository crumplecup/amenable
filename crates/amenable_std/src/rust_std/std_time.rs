//! `RustStdType` registrations for `std::time`.

use std::time::{Instant, SystemTime, SystemTimeError};

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    Instant,
    "std",
    "std::time",
    "https://doc.rust-lang.org/std/time/struct.Instant.html",
    "The Instant carrier is a monotonically nondecreasing point in time, suitable for measuring elapsed durations."
);

impl_rust_std_type!(
    SystemTime,
    "std",
    "std::time",
    "https://doc.rust-lang.org/std/time/struct.SystemTime.html",
    "The SystemTime carrier is a point in time as reported by the system clock, which may jump backward or forward."
);

impl_rust_std_type!(
    SystemTimeError,
    "std",
    "std::time",
    "https://doc.rust-lang.org/std/time/struct.SystemTimeError.html",
    "The SystemTimeError carrier reports that a SystemTime subtraction went backward in time."
);
