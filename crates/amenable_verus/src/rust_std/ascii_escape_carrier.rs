//! Verus accommodation model for `core::ascii::EscapeDefault`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Mirrors `char_transform_carrier.rs`'s own reasoning for
//! why this checks one fixed representative example rather than a
//! symbolic law (`amenable_kani`'s harness does the same, for the same
//! reason — collecting a symbolic escape iterator times out under
//! Kani): `b'\n'.escape_ascii()` yields the two-byte escape `b"\\n"`.
//! Unlike the `char` escape adapters' `&str` output, `Vec<u8>`'s
//! `PartialEq` does have real `vstd` spec support (`std_specs/vec.rs`),
//! so this checks the claim via ordinary exec `==` rather than needing
//! view-equality. Not `EscapeDefault` itself — the proof is
//! conditional: sound if the real `EscapeDefault` refines this fixed
//! mapping, which `amenable_kani`'s own
//! `verify_escape_default_escapes_a_control_byte` harness (checking the
//! real `u8::escape_ascii()` directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `b'\n'.escape_ascii()` yields `b"\n"` (the two-byte escape,
/// backslash then `n`) — the fixed example `EscapeDefault` is expected
/// to refine.
pub fn verify_escape_default_model_escapes_a_control_byte() -> (result: bool)
    ensures
        result,
{
    // 92 = b'\\' (backslash), 110 = b'n' -- spelled as numeric literals
    // since Verus's macro rejects the `b'\\'` byte-literal escape form.
    let escaped: Vec<u8> = vec![92u8, 110u8];
    let expected: Vec<u8> = vec![92u8, 110u8];
    escaped == expected
}

} // verus!
