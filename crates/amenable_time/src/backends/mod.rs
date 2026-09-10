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
//! can honestly back — durations, fixed instants, and endpoint ordering.
//! Two things make it a useful canary rather than a stub:
//!
//! - the trait-bound assertions in `tests/std_backend_test.rs`
//!   (`StdTimeBackend: TemporalIntervalFactory<CanaryVerifier>`, …) fail
//!   to compile if the interface drifts;
//! - its `Exchange` bodies *execute* the contracts — `order_offset_endpoints`
//!   resolves both endpoints through real Gregorian calendar arithmetic
//!   and returns `Err` for a reversed interval; the duration bridge
//!   round-trips a span through an actual `std::time::Duration` and
//!   rejects year/month components. It is a runtime oracle against the
//!   formal backends over the slice it covers.
//!
//! Real coverage of the rest (week/ordinal dates, time zones, ISO 8601 /
//! RFC 3339 parsing and formatting) needs a date-time library; a `jiff`
//! (or `chrono`) backend module would sit alongside this one.

mod std_time;

pub use std_time::{
    CanaryVerifier, CanaryVerifierMetadata, StdDuration, StdSystemTime, StdTimeBackend,
    StdUtcOffset,
};
