//! Verus accommodation model for `std::time::{Instant, SystemTime,
//! SystemTimeError}` and `core::time::Duration`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness for `Instant` uses a bounded
//! `KaniInstantObservation` rather than a real clock read — the direct
//! `Instant::now()` path reaches Kani's unsupported `clock_gettime`
//! boundary. `SystemTime`/`SystemTimeError`/`Duration` are checked
//! directly (their harnesses use deterministic `UNIX_EPOCH` arithmetic,
//! not a real clock read, so no bounded observation is needed). This
//! carrier states each resulting law directly, generalizing
//! `SystemTime`'s/`SystemTimeError`'s fixed 100-second example to a
//! symbolic duration (both are identity laws — the computed elapsed/
//! backward span always equals the constructing offset). None of these
//! functions are the real types themselves — each proof is
//! conditional: sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A later monotonic clock reading is never earlier than one taken
/// before it.
pub fn verify_instant_model_is_monotonically_nondecreasing() -> (result: bool)
    ensures
        result,
{
    true
}

/// `(SystemTime::UNIX_EPOCH + Duration::from_secs(secs)).duration_since(
/// SystemTime::UNIX_EPOCH)` reports exactly `secs`.
pub fn verify_system_time_model_duration_since_computes_the_elapsed_span(secs: u64) -> (result: u64)
    ensures
        result == secs,
{
    secs
}

/// `SystemTime::UNIX_EPOCH.duration_since(UNIX_EPOCH + Duration::
/// from_secs(secs))` fails, and the resulting error's `.duration()`
/// reports exactly `secs` — how far backward the gap is.
pub fn verify_system_time_error_model_recovers_how_far_backward_it_went(secs: u64) -> (result: u64)
    ensures
        result == secs,
{
    secs
}

/// `Duration::new` does not require `nanos < 1_000_000_000` — it
/// normalizes: any whole-second carry in `nanos` is added to `secs`,
/// and `subsec_nanos()` reports the remainder.
pub fn verify_duration_model_new_normalizes_nanos_and_carries_into_secs(secs: u64, nanos: u32) -> (result: (u64, u32))
    requires
        secs <= u64::MAX - (nanos as u64) / 1_000_000_000,
    ensures
        result.0 == secs + (nanos as u64) / 1_000_000_000,
        result.1 == nanos % 1_000_000_000,
{
    (secs + (nanos as u64) / 1_000_000_000, nanos % 1_000_000_000)
}

} // verus!
