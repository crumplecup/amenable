//! Verus spec for `std::ffi::CString` / `std::ffi::NulError`.

use std::ffi::CString;
#[cfg(verus_keep_ghost)]
use std::ffi::NulError;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCString(CString);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExNulError(NulError);

/// The byte sequence `T::into(bytes): Vec<u8>` produces — opaque
/// (uninterpreted) for a bare generic `T: Into<Vec<u8>>`, connected to
/// the concrete `Vec<u8>` case by the broadcast axiom below.
pub uninterp spec fn into_vec_u8_spec<T: Into<Vec<u8>>>(bytes: T) -> Seq<u8>;

#[verifier::external_body]
pub broadcast proof fn axiom_vec_u8_into_vec_u8_is_identity(v: Vec<u8>)
    ensures
        #[trigger] into_vec_u8_spec(v) == v@,
{
}

pub assume_specification<T: Into<Vec<u8>>> [CString::new::<T>] (bytes: T) -> (result: Result<CString, NulError>)
    ensures
        (!exists|i: int| 0 <= i < into_vec_u8_spec(bytes).len() - 1 && into_vec_u8_spec(bytes)[i] == 0)
            ==> result is Ok,
        (exists|i: int| 0 <= i < into_vec_u8_spec(bytes).len() - 1 && into_vec_u8_spec(bytes)[i] == 0)
            ==> result is Err,
;

/// `CString::new` appends its own nul terminator, and rejects any input
/// that already contains an interior nul byte — the same claim the
/// Kani harness checks. Takes `byte` as a `requires`-constrained
/// parameter (the same "parameter, not inline literal" shape as
/// `option_carrier.rs`) rather than a `kani::any()` symbolic value.
pub fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul(byte: u8) -> (result: bool)
    requires
        byte != 0,
    ensures
        result,
{
    broadcast use axiom_vec_u8_into_vec_u8_is_identity;

    let single: Vec<u8> = vec![byte];
    let new_result = CString::new(single);
    assert(new_result is Ok);
    let accepted = new_result.is_ok();

    let with_interior_nul: Vec<u8> = vec![byte, 0, byte];
    assert(into_vec_u8_spec(with_interior_nul)[1int] == 0);
    let rejected_result = CString::new(with_interior_nul);
    assert(rejected_result is Err);
    let rejected = rejected_result.is_err();

    accepted && rejected
}

} // verus!
