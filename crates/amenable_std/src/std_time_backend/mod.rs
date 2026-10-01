//! The `std::time` canary backend for `amenable_time`'s temporal contract
//! interface.
//!
//! Lives in `amenable_std`, not `amenable_time`: `amenable_time` is the
//! trait/contract interface crate, kept dependency-light so it can be an
//! optional dep of `amenable_ext`'s jiff/chrono backends without dragging
//! in `amenable_std`'s whole std-lib registration surface (see
//! `docs/AMENABLE_EXT_PLAN.md`). Backend implementations instead live
//! alongside the type registrations they're built from — this one here,
//! future jiff/chrono ones in `amenable_ext`.
//!
//! A backend is a concrete type that performs temporal transitions as
//! [`Exchange`](amenable_core::Exchange)s and exposes its native carrier
//! types through the `Temporal*Props` families. The `Exchange` impls live
//! *here* rather than in `amenable_time` itself because every descriptor,
//! sidecar and token they touch is `amenable_time`'s own — the impl is
//! `impl ForeignTrait for LocalType`, which the orphan rule allows.
//!
//! This is a **canary**, not a usable backend: it implements only the
//! slice of the interface that `std::time::{Duration, SystemTime}` can
//! honestly back — durations, fixed instants, and endpoint ordering. Two
//! things make it a useful canary rather than a stub:
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
//! (or `chrono`) backend in `amenable_ext` would sit alongside this one.
//!
//! Split by real concern: [`verifier`] (the canary's identity as a trust
//! boundary into the `Witness<V>` system), [`backend`] (the native
//! carriers and the backend struct's own property/reporter declarations),
//! [`interval_exchange`] (calendar arithmetic and the interval-factory
//! `Exchange`s), and [`duration_bridge`] (duration arithmetic and the
//! duration-native-bridge `Exchange`s).

mod backend;
mod duration_bridge;
mod interval_exchange;
mod verifier;

pub use backend::{StdDuration, StdSystemTime, StdTimeBackend, StdUtcOffset};
pub use verifier::{CanaryVerifier, CanaryVerifierMetadata};
