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

pub assume_specification<'a> [CStr::from_bytes_until_nul] (bytes: &'a [u8]) -> (result: Result<&'a CStr, std::ffi::FromBytesUntilNulError>)
    ensures
        (exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0) ==> result is Ok,
        (!exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0) ==> result is Err,
;

/// `to_bytes`'s abstract content for a `&CStr` — opaque (uninterpreted),
/// connected to the bytes a `from_bytes_with_nul` call was constructed
/// from (minus the terminating nul) by that function's own `ensures`
/// below, mirroring `cstring_carrier.rs`'s `cstring_bytes_spec` pattern.
pub uninterp spec fn cstr_bytes_spec(cstr: &CStr) -> Seq<u8>;

pub assume_specification<'a> [CStr::from_bytes_with_nul] (bytes: &'a [u8]) -> (result: Result<&'a CStr, std::ffi::FromBytesWithNulError>)
    ensures
        (bytes@.len() > 0 && bytes@[bytes@.len() - 1] == 0
            && !exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0) ==> {
            &&& result is Ok
            &&& cstr_bytes_spec(result->Ok_0) == bytes@.subrange(0, bytes@.len() - 1)
        },
        (!exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0) ==> result is Err,
        (exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0) ==> result is Err,
;

pub assume_specification<'a> [CStr::to_bytes] (cstr: &'a CStr) -> (result: &'a [u8])
    ensures
        result@ == cstr_bytes_spec(cstr),
;

/// `CStr::from_bytes_until_nul` succeeds whenever a nul byte appears
/// anywhere in the input, and fails only when none is present at all —
/// the same claim the Kani harness checks. Takes `with_nul`/`without_nul`
/// as `requires`-constrained parameters (the same "parameter, not
/// inline literal" shape as `option_carrier.rs`) rather than the array
/// literals Kani constructs inline.
pub fn verify_from_bytes_until_nul_requires_a_nul_byte_somewhere(with_nul: &[u8], without_nul: &[u8]) -> (result: (bool, bool))
    requires
        exists|i: int| 0 <= i < with_nul@.len() && with_nul@[i] == 0,
        !exists|i: int| 0 <= i < without_nul@.len() && without_nul@[i] == 0,
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
        // Canonical home: amenable_std::NonNulByte's Requires<VerusVerifier>
        // impl (this file's own verify_cstr_excludes_the_terminating_nul_from_to_bytes) names this exact fragment.
        byte != 0,
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
        // Canonical home: amenable_std::NonNulByte's Requires<VerusVerifier>
        // impl (this file's own verify_cstr_excludes_the_terminating_nul_from_to_bytes) names this exact fragment.
        byte != 0,
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
