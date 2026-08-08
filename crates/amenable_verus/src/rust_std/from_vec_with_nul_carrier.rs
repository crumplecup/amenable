//! Verus spec for `std::ffi::FromVecWithNulError`.

use std::ffi::CString;
#[cfg(verus_keep_ghost)]
use std::ffi::FromVecWithNulError;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExFromVecWithNulError(FromVecWithNulError);

pub assume_specification [CString::from_vec_with_nul] (bytes: Vec<u8>) -> (result: Result<CString, FromVecWithNulError>)
    ensures
        (bytes@.len() > 0 && bytes@[bytes@.len() - 1] == 0
            && !(exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0)) ==> result is Ok,
        (!exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0) ==> result is Err,
        (exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0) ==> result is Err,
;

/// `CString::from_vec_with_nul` requires the nul to be exactly the last
/// byte — the same claim the Kani harness checks. Takes `byte` as a
/// `requires`-constrained parameter (the same "parameter, not inline
/// literal" shape as `option_carrier.rs`) rather than a `kani::any()`
/// symbolic value.
pub fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end(byte: u8) -> (result: (bool, bool, bool))
    requires
        // Canonical home: amenable_std::NonNulByte's Requires<VerusVerifier>
        // impl (rust_std::cstr_carrier) names this exact fragment.
        byte != 0,
    ensures
        result.0,
        result.1,
        result.2,
{
    let nul_last: Vec<u8> = vec![byte, 0];
    assert(!exists|i: int| 0 <= i < nul_last@.len() - 1 && nul_last@[i] == 0);
    let accepted_result = CString::from_vec_with_nul(nul_last);
    assert(accepted_result is Ok);
    let accepted = accepted_result.is_ok();

    let no_nul: Vec<u8> = vec![byte, byte];
    assert(!exists|i: int| 0 <= i < no_nul@.len() && no_nul@[i] == 0);
    let no_nul_result = CString::from_vec_with_nul(no_nul);
    assert(no_nul_result is Err);
    let rejected_no_nul = no_nul_result.is_err();

    let nul_before_end: Vec<u8> = vec![byte, 0, byte];
    assert(nul_before_end@[1int] == 0);
    let nul_before_end_result = CString::from_vec_with_nul(nul_before_end);
    assert(nul_before_end_result is Err);
    let rejected_early_nul = nul_before_end_result.is_err();

    (accepted, rejected_no_nul, rejected_early_nul)
}

} // verus!
