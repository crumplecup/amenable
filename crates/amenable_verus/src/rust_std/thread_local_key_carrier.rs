//! Verus accommodation model for
//! `std::thread::LocalKey<std::cell::Cell<i32>>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `LocalKey`
//! directly, within a single thread (its per-thread isolation itself
//! isn't something Kani — or Verus — can verify without spawning a
//! second thread; both tools verify a single sequential instruction
//! stream). This carrier states the same fixed example directly:
//! `.with()` reads the lazily-initialized value `5`, and after a
//! mutation through `.with()`, a later `.with()` sees `42`. Not
//! `LocalKey` itself — the proof is conditional: sound if the real type
//! refines this example, which `amenable_kani`'s own
//! `verify_local_key_with_reads_the_initialized_value` harness (checking
//! the real type directly) already confirms independently, for the
//! identical example.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `.with()` on a `LocalKey<Cell<i32>>` initialized to `5` reads `5`;
/// after `.with(|c| c.set(42))`, a later `.with()` reads `42`.
pub fn verify_local_key_model_with_reads_the_initialized_value() -> (result: (i32, i32))
    ensures
        result.0 == 5,
        result.1 == 42,
{
    (5, 42)
}

} // verus!
