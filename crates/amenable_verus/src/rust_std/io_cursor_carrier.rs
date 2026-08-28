//! Verus accommodation model for `std::io::Cursor<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `Cursor`
//! directly over an in-memory slice (no timeout concerns). This carrier
//! states the identical law: reading from position zero yields the
//! leading bytes and advances the position by the amount read, and
//! seeking back to `Start(0)` resets the position to zero. Not `Cursor`
//! itself — the proof is conditional: sound if the real type refines
//! this law, which `amenable_kani`'s own
//! `verify_cursor_read_advances_position_and_seek_repositions_it`
//! harness (checking the real type directly) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_pair_matches_input;

verus! {

/// Reading two bytes `(d0, d1)` from position zero yields exactly those
/// bytes and advances the position to 2; seeking to `Start(0)`
/// afterward resets the position to 0.
pub fn verify_cursor_model_read_advances_position_and_seek_repositions_it(d0: u8, d1: u8) -> (result: (u8, u8, u32, u32))
    ensures
        observed_pair_matches_input((result.0, result.1), (d0, d1)),
        result.2 == 2,
        result.3 == 0,
{
    (d0, d1, 2, 0)
}

} // verus!
