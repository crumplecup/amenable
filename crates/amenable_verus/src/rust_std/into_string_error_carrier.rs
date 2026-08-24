//! Verus spec for `std::ffi::IntoStringError`.
//!
//! Calls `CString::new` directly rather than declaring its own axiom for
//! it — `cstring_carrier.rs` already does (same compilation unit, one
//! crate; a second declaration would hit the duplicate-
//! assume_specification wall the `TryFromIntError` gallery finding
//! documents), and also supplies `cstring_bytes_spec`, the opaque
//! per-value byte content this file's `as_bytes` axiom ties into.

use std::ffi::CString;
#[cfg(verus_keep_ghost)]
use std::ffi::IntoStringError;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use super::cstring_carrier::cstring_bytes_spec;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExIntoStringError(IntoStringError);

/// The `CString` `.into_cstring()` recovers from a given
/// `IntoStringError` — opaque (uninterpreted), giving `into_string`'s
/// and `into_cstring`'s axioms a shared value to agree on without
/// either needing to reconstruct a real `CString`.
pub uninterp spec fn into_cstring_spec(err: IntoStringError) -> CString;

pub assume_specification [CString::into_string] (s: CString) -> (result: Result<String, IntoStringError>)
    ensures
        // 0xFF is never a valid leading byte in UTF-8 (outside the
        // encoding's valid range entirely) -- a real, sufficient
        // condition for this claim's specific input, not a full UTF-8
        // validity check over arbitrary byte content.
        (cstring_bytes_spec(s).len() > 0 && cstring_bytes_spec(s)[0] == 0xFFu8) ==> result is Err,
        result is Err ==> cstring_bytes_spec(into_cstring_spec(result->Err_0)) == cstring_bytes_spec(s),
;

pub assume_specification [IntoStringError::into_cstring] (err: IntoStringError) -> (result: CString)
    ensures
        result == into_cstring_spec(err),
;

pub assume_specification [CString::as_bytes] (s: &CString) -> (result: &[u8])
    ensures
        result@ == cstring_bytes_spec(*s),
;

/// `CString::into_string` fails when the bytes aren't valid UTF-8, and
/// the error doesn't discard them: `.into_cstring()` recovers exactly
/// the original `CString` (the full byte content, not just its length)
/// — the same claim the Kani harness checks. Takes `bytes` as a
/// `requires`-constrained parameter (the same "parameter, not inline
/// literal" shape as `option_carrier.rs`) rather than the literal
/// `vec![0xFFu8, b'x']` Kani constructs inline.
pub fn verify_into_string_error_recovers_the_original_cstring(bytes: Vec<u8>) -> (result: bool)
    requires
        bytes@.len() == 2,
        bytes@[0] == 0xFFu8,
        bytes@[1] != 0,
    ensures
        result,
{
    #[cfg(verus_keep_ghost)]
    use super::cstring_carrier::{axiom_vec_u8_into_vec_u8_is_identity, into_vec_u8_spec, into_vec_u8_spec_matches_input_vec};
    broadcast use axiom_vec_u8_into_vec_u8_is_identity;

    assert(into_vec_u8_spec_matches_input_vec(bytes));
    assert(!exists|i: int| 0 <= i < into_vec_u8_spec(bytes).len() - 1 && into_vec_u8_spec(bytes)[i] == 0) by {
        assert(into_vec_u8_spec_matches_input_vec(bytes));
        assert(into_vec_u8_spec(bytes)[0int] != 0);
    }
    let new_result = CString::new(bytes);
    assert(new_result is Ok);
    let invalid = new_result.expect("proven Ok by the assert immediately above");
    let ghost original_view: Seq<u8> = cstring_bytes_spec(invalid);

    let into_string_result = invalid.into_string();
    assert(into_string_result is Err);
    // vstd has no spec for Result::expect_err (unlike expect, which it
    // does cover), so this falls back to the same real-verus-vs-plain-
    // rustc split int_error_kind_carrier.rs uses for its own proven-
    // impossible branches.
    let err = match into_string_result {
        Err(value) => value,
        #[cfg(verus_keep_ghost)]
        Ok(_) => unreached(),
        #[cfg(not(verus_keep_ghost))]
        Ok(_) => unreachable!(),
    };

    let recovered = err.into_cstring();
    let _recovered_bytes = recovered.as_bytes();
    assert(_recovered_bytes@ =~= original_view);
    true
}

} // verus!
