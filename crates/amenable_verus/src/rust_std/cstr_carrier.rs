//! Verus spec for `core::ffi::CStr` / `core::ffi::FromBytesUntilNulError` /
//! `core::ffi::FromBytesWithNulError`.

use std::ffi::CStr;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCStr(CStr);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExFromBytesUntilNulError(std::ffi::FromBytesUntilNulError);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExFromBytesWithNulError(std::ffi::FromBytesWithNulError);

pub open spec fn cstr_bytes_contain_a_nul(bytes: &[u8]) -> bool {
    exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0
}

pub open spec fn cstr_bytes_contain_no_nul(bytes: &[u8]) -> bool {
    !exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0
}

pub open spec fn cstr_bytes_have_an_interior_nul(bytes: &[u8]) -> bool {
    exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0
}

pub open spec fn cstr_bytes_have_only_a_trailing_nul(bytes: &[u8]) -> bool {
    bytes@.len() > 0
        && bytes@[bytes@.len() - 1] == 0
        && !cstr_bytes_have_an_interior_nul(bytes)
}

pub open spec fn cstr_from_bytes_until_nul_result_matches_nul_presence<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesUntilNulError>,
) -> bool {
    (cstr_bytes_contain_a_nul(bytes) ==> result is Ok)
        && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
}

pub assume_specification<'a> [CStr::from_bytes_until_nul] (bytes: &'a [u8]) -> (result: Result<&'a CStr, std::ffi::FromBytesUntilNulError>)
    ensures
        cstr_from_bytes_until_nul_result_matches_nul_presence(bytes, result),
;

/// `to_bytes`'s abstract content for a `&CStr` — opaque (uninterpreted),
/// connected to the bytes a `from_bytes_with_nul` call was constructed
/// from (minus the terminating nul) by that function's own `ensures`
/// below, mirroring `cstring_carrier.rs`'s `cstring_bytes_spec` pattern.
pub uninterp spec fn cstr_bytes_spec(cstr: &CStr) -> Seq<u8>;

pub open spec fn cstr_from_bytes_with_nul_result_matches_bytes<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesWithNulError>,
) -> bool {
    (cstr_bytes_have_only_a_trailing_nul(bytes) ==> {
        &&& result is Ok
        &&& cstr_bytes_spec(result->Ok_0) == bytes@.subrange(0, bytes@.len() - 1)
    }) && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
        && (cstr_bytes_have_an_interior_nul(bytes) ==> result is Err)
}

pub assume_specification<'a> [CStr::from_bytes_with_nul] (bytes: &'a [u8]) -> (result: Result<&'a CStr, std::ffi::FromBytesWithNulError>)
    ensures
        cstr_from_bytes_with_nul_result_matches_bytes(bytes, result),
;

pub open spec fn cstr_to_bytes_matches_model(cstr: &CStr, result: &[u8]) -> bool {
    result@ == cstr_bytes_spec(cstr)
}

pub assume_specification<'a> [CStr::to_bytes] (cstr: &'a CStr) -> (result: &'a [u8])
    ensures
        cstr_to_bytes_matches_model(cstr, result),
;

pub open spec fn cstr_until_nul_test_inputs_cover_both_cases(
    with_nul: &[u8],
    without_nul: &[u8],
) -> bool {
    cstr_bytes_contain_a_nul(with_nul) && cstr_bytes_contain_no_nul(without_nul)
}

pub open spec fn non_nul_byte_value_is_nonzero(byte: u8) -> bool {
    byte != 0
}

/// `CStr::from_bytes_until_nul` succeeds whenever a nul byte appears
/// anywhere in the input, and fails only when none is present at all —
/// the same claim the Kani harness checks. Takes `with_nul`/`without_nul`
/// as `requires`-constrained parameters (the same "parameter, not
/// inline literal" shape as `option_carrier.rs`) rather than the array
/// literals Kani constructs inline.
pub fn verify_from_bytes_until_nul_requires_a_nul_byte_somewhere(with_nul: &[u8], without_nul: &[u8]) -> (result: (bool, bool))
    requires
        cstr_until_nul_test_inputs_cover_both_cases(with_nul, without_nul),
    ensures
        result.0,
        result.1,
{
    let with_nul_result = CStr::from_bytes_until_nul(with_nul);
    assert(with_nul_result is Ok);
    let accepted = with_nul_result.is_ok();

    let without_nul_result = CStr::from_bytes_until_nul(without_nul);
    assert(without_nul_result is Err);
    let rejected = without_nul_result.is_err();

    (accepted, rejected)
}

/// `CStr::from_bytes_with_nul` requires the nul to be exactly the last
/// byte: no nul at all is rejected, and so is an interior nul with
/// trailing data after it — the same claim the Kani harness checks.
pub fn verify_from_bytes_with_nul_requires_the_nul_only_at_the_end(byte: u8) -> (result: (bool, bool, bool))
    requires
        non_nul_byte_value_is_nonzero(byte),
    ensures
        result.0,
        result.1,
        result.2,
{
    let nul_last: &[u8] = &[byte, 0];
    assert(!exists|i: int| 0 <= i < nul_last@.len() - 1 && nul_last@[i] == 0);
    let nul_last_result = CStr::from_bytes_with_nul(nul_last);
    assert(nul_last_result is Ok);
    let accepted = nul_last_result.is_ok();

    let no_nul: &[u8] = &[byte, byte];
    assert(!exists|i: int| 0 <= i < no_nul@.len() && no_nul@[i] == 0);
    let no_nul_result = CStr::from_bytes_with_nul(no_nul);
    assert(no_nul_result is Err);
    let rejected_no_nul = no_nul_result.is_err();

    let interior_nul: &[u8] = &[0, byte];
    assert(interior_nul@[0int] == 0);
    let interior_nul_result = CStr::from_bytes_with_nul(interior_nul);
    assert(interior_nul_result is Err);
    let rejected_interior_nul = interior_nul_result.is_err();

    (accepted, rejected_no_nul, rejected_interior_nul)
}

/// `CStr::from_bytes_with_nul` accepts a nul-terminated byte sequence,
/// and `.to_bytes()` reports its content without the terminator itself
/// — the same claim the Kani harness checks.
pub fn verify_cstr_excludes_the_terminating_nul_from_to_bytes(byte: u8) -> (result: bool)
    requires
        non_nul_byte_value_is_nonzero(byte),
    ensures
        result,
{
    let with_nul: &[u8] = &[byte, 0];
    let cstr_result = CStr::from_bytes_with_nul(with_nul);
    assert(cstr_result is Ok);
    let cstr = cstr_result.unwrap();

    let bytes = cstr.to_bytes();
    assert(bytes@.len() == 1);
    assert(bytes@[0] == byte);

    bytes.len() == 1 && bytes[0] == byte
}

} // verus!
