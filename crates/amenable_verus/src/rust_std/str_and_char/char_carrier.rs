//! Verus spec for `char`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `verify_char_roundtrip`'s whole postcondition: the round trip
/// preserves the value verbatim.
pub open spec fn char_roundtrip_preserves_value(result: char, input: char) -> bool {
    result == input
}

/// Whether `value` is a valid Unicode scalar value: not in the surrogate
/// range.
pub open spec fn char_is_valid_unicode_scalar(value: char) -> bool {
    (value as u32) <= 0xD7FFu32
        || ((value as u32) >= 0xE000u32 && (value as u32) <= 0x10FFFFu32)
}

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
///
/// The second `ensures` clause is the canonical home
/// `amenable_std::ValidUnicodeScalar` names — see that type for the same
/// bound stated once, and its `Ensures<VerusVerifier>` impl for this exact
/// fragment held as a reusable, backend-checkable claim.
pub fn verify_char_roundtrip(c: char) -> (result: char)
    ensures
        char_roundtrip_preserves_value(result, c),
        char_is_valid_unicode_scalar(c),
{
    c
}

} // verus!
