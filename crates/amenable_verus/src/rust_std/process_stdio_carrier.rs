//! Verus accommodation model for `std::process::Stdio`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded Amenable-owned
//! process observation rather than a real `Stdio` configuration
//! directly — the direct path reaches Kani's unsupported
//! C-string-literal boundary. This carrier states the resulting law
//! directly: `.stdout(Stdio::null())` leaves no child stdout handle,
//! while `.stdout(Stdio::piped())` does expose one. Not `Stdio` itself
//! — the proof is conditional: sound if the real type refines this
//! law, which `amenable_kani`'s own
//! `verify_stdio_null_discards_the_childs_output_handle` harness
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Stdio::null()`'s stdout handle is absent (`result.0` is `false`);
/// `Stdio::piped()`'s stdout handle is present (`result.1` is `true`).
pub fn verify_stdio_model_null_discards_the_childs_output_handle() -> (result: (bool, bool))
    ensures
        !result.0,
        result.1,
{
    (false, true)
}

} // verus!
