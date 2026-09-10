//! Backend implementations of the temporal trait surface.
//!
//! A backend is a concrete type that performs temporal transitions as
//! [`Exchange`](amenable_core::Exchange)s and exposes its native carrier
//! types through the `Temporal*Props` families. The `Exchange` impls live
//! *here* rather than in a downstream crate because every descriptor,
//! sidecar and token they touch is `amenable_time`'s own — the impl is
//! `impl ForeignTrait for LocalType`, which the orphan rule allows.
//!
//! [`std_time`] is a **canary**, not a usable backend: it implements only
//! the slice of the interface that `std::time::{Duration, SystemTime}`
//! can honestly back — durations, fixed instants, and endpoint ordering —
//! against a [`CanaryVerifier`](std_time::CanaryVerifier) that proves
//! nothing. Its purpose is to catch interface breakage at `cargo check`
//! time. Real coverage (calendar dates, week/ordinal dates, time zones,
//! ISO 8601 / RFC 3339 parsing and formatting) needs a date-time library;
//! a `jiff` (or `chrono`) backend module would sit alongside this one.

mod std_time;

pub use std_time::{
    CanaryVerifier, CanaryVerifierMetadata, StdDuration, StdSystemTime, StdTimeBackend,
    StdUtcOffset,
};
