//! Verus spec for `char`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `char` is constrained to Unicode scalar values (excludes the surrogate
/// range `0xD800..=0xDFFF`) and round-trips through itself — the same
/// claim the Kani harness checks by symbolic exploration and the Creusot
/// harness checks via the `@` View operator, restated here as a real,
/// `verus`-checked postcondition. `c as u32` is an ordinary Verus spec
/// cast for a Rust primitive (confirmed empirically against a real
/// `verus` install: `verus --crate-type=lib` on this exact function
/// reports `verification results:: 1 verified, 0 errors`), unlike
/// `String`'s content, which needs the `@` View operator instead (see
/// `string_carrier.rs`).
pub fn verify_char_roundtrip(c: char) -> (result: char)
    ensures
        result == c,
        (c as u32) <= 0xD7FFu32 || ((c as u32) >= 0xE000u32 && (c as u32) <= 0x10FFFFu32),
{
    c
}

} // verus!
