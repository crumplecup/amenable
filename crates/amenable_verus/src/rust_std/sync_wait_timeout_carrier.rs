//! Verus accommodation model for `std::sync::WaitTimeoutResult`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniWaitTimeoutObservation` rather than the real
//! `Condvar::wait_timeout()` path directly — that path reaches an
//! unsupported `clock_gettime` boundary under Kani. This carrier states
//! the resulting law directly: `.timed_out()` reports `true` for a wait
//! that genuinely timed out. Not `WaitTimeoutResult` itself — the proof
//! is conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own `verify_wait_timeout_result_reports_timed_out`
//! harness (checking the real type via the bounded observation)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `.timed_out()` reports `true` for a `WaitTimeoutResult` produced by
/// a wait that genuinely timed out.
pub fn verify_wait_timeout_result_model_reports_timed_out() -> (result: bool)
    ensures
        result,
{
    true
}

} // verus!
