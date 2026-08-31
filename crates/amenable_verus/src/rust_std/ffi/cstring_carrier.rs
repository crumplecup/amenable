//! Verus spec for `std::ffi::CString` / `std::ffi::NulError`.

use std::ffi::CString;
#[cfg(verus_keep_ghost)]
use std::ffi::NulError;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `#[verifier::external_type_specification]` marker binding `CString`
/// to Verus -- never constructed by real code, only discovered by
/// Verus's own attribute processing.
#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCString(CString);

/// `#[verifier::external_type_specification]` marker binding `NulError`
/// to Verus.
#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExNulError(NulError);

/// The byte sequence `T::into(bytes): Vec<u8>` produces — opaque
/// (uninterpreted) for a bare generic `T: Into<Vec<u8>>`, connected to
/// the concrete `Vec<u8>` case by the broadcast axiom below.
pub uninterp spec fn into_vec_u8_spec<T: Into<Vec<u8>>>(bytes: T) -> Seq<u8>;

/// [`into_vec_u8_spec`]'s concrete-`Vec<u8>` case: `into` is the
/// identity.
pub open spec fn into_vec_u8_spec_matches_input_vec(v: Vec<u8>) -> bool {
    into_vec_u8_spec(v) == v@
}

#[verifier::external_body]
pub broadcast proof fn axiom_vec_u8_into_vec_u8_is_identity(v: Vec<u8>)
    ensures
        #[trigger] into_vec_u8_spec(v) == v@,
        #[trigger] into_vec_u8_spec_matches_input_vec(v),
{
}

/// The byte content (excluding the appended nul terminator) a real
/// `CString` carries — opaque (uninterpreted), connected to
/// `CString::new`'s input below and to `CString::as_bytes`'s result in
/// `into_string_error_carrier.rs`.
pub uninterp spec fn cstring_bytes_spec(s: CString) -> Seq<u8>;

/// Whether `bytes` has a nul byte before its final position.
pub open spec fn cstring_input_has_a_preterminal_nul<T: Into<Vec<u8>>>(bytes: T) -> bool {
    exists|i: int| 0 <= i < into_vec_u8_spec(bytes).len() - 1 && into_vec_u8_spec(bytes)[i] == 0
}

/// Whether `bytes` has no nul byte before its final position.
pub open spec fn cstring_input_has_no_preterminal_nul<T: Into<Vec<u8>>>(bytes: T) -> bool {
    !cstring_input_has_a_preterminal_nul(bytes)
}

/// `CString::new`'s whole postcondition: succeeds (recovering the input
/// bytes via [`cstring_bytes_spec`]) unless a nul byte appears before the
/// final position, in which case it fails.
pub open spec fn cstring_new_result_matches_input_bytes<T: Into<Vec<u8>>>(
    bytes: T,
    result: Result<CString, NulError>,
) -> bool {
    (cstring_input_has_no_preterminal_nul(bytes)
        ==> (result is Ok && cstring_bytes_spec(result->Ok_0) == into_vec_u8_spec(bytes)))
        && (cstring_input_has_a_preterminal_nul(bytes) ==> result is Err)
}

pub assume_specification<T: Into<Vec<u8>>> [CString::new::<T>] (bytes: T) -> (result: Result<CString, NulError>)
    ensures
        cstring_new_result_matches_input_bytes(bytes, result),
;

/// Precondition shared by this file's test inputs: `byte` itself is never
/// the nul byte.
pub open spec fn cstring_test_byte_is_nonzero(byte: u8) -> bool {
    byte != 0
}

/// `CString::new` appends its own nul terminator, and rejects any input
/// that already contains an interior nul byte — the same claim the
/// Kani harness checks. Takes `byte` as a `requires`-constrained
/// parameter (the same "parameter, not inline literal" shape as
/// `option_carrier.rs`) rather than a `kani::any()` symbolic value.
pub fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul(byte: u8) -> (result: bool)
    requires
        // Canonical home: amenable_std::NonNulByte's Requires<VerusVerifier>
        // impl (rust_std::cstr_carrier) names this exact fragment.
        cstring_test_byte_is_nonzero(byte),
    ensures
        result,
{
    broadcast use axiom_vec_u8_into_vec_u8_is_identity;

    let single: Vec<u8> = vec![byte];
    assert(into_vec_u8_spec_matches_input_vec(single));
    assert(!exists|i: int| 0 <= i < into_vec_u8_spec(single).len() - 1 && into_vec_u8_spec(single)[i] == 0) by {
        assert(into_vec_u8_spec_matches_input_vec(single));
    }
    let new_result = CString::new(single);
    assert(new_result is Ok);
    let accepted = new_result.is_ok();

    let with_interior_nul: Vec<u8> = vec![byte, 0, byte];
    assert(into_vec_u8_spec_matches_input_vec(with_interior_nul));
    assert(into_vec_u8_spec(with_interior_nul)[1int] == 0) by {
        assert(into_vec_u8_spec_matches_input_vec(with_interior_nul));
    }
    let rejected_result = CString::new(with_interior_nul);
    assert(rejected_result is Err);
    let rejected = rejected_result.is_err();

    accepted && rejected
}

} // verus!
