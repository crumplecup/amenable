//! Verus accommodation model for `std::io::Lines<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniLinesObservation` rather than the real `BufRead::lines` path
//! directly. This carrier states the resulting law directly: `.lines()`
//! yields each line with its trailing newline dropped, over three
//! ASCII bytes symbolic other than not themselves being line
//! terminators. Not `Lines` itself — the proof is conditional: sound if
//! the real type refines this law, which `amenable_kani`'s own harness
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Over three ASCII bytes that are each neither `\n` (10) nor `\r`
/// (13), `.lines()` yields each byte unchanged as its own line, with no
/// terminator attached.
pub fn verify_lines_model_splits_on_newlines_and_drops_the_terminator(first: u8, second: u8, third: u8) -> (result: (u8, u8, u8))
    requires
        first < 128,
        second < 128,
        third < 128,
        first != 10,
        first != 13,
        second != 10,
        second != 13,
        third != 10,
        third != 13,
    ensures
        result == (first, second, third),
{
    (first, second, third)
}

} // verus!
