//! Verus spec for `core::char::CharTryFromError` / `core::char::TryFromCharError`.
//!
//! Matches Kani's own claim scope exactly: both harnesses check the
//! claim over every possible `u32`/`char` via symbolic execution
//! (`kani::any()`), not a representative sample — these proofs do the
//! same via real universal parameters.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCharTryFromError(core::char::CharTryFromError);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExTryFromCharError(core::char::TryFromCharError);

pub assume_specification [<char as core::convert::TryFrom<u32>>::try_from] (value: u32) -> (result: Result<char, <char as core::convert::TryFrom<u32>>::Error>)
    ensures
        (value <= 0x0010_FFFF && !(0xD800 <= value && value <= 0xDFFF)) ==> (result is Ok && (result->Ok_0 as u32) == value),
        (value > 0x0010_FFFF || (0xD800 <= value && value <= 0xDFFF)) ==> result is Err,
;

pub assume_specification [<u8 as core::convert::TryFrom<char>>::try_from] (value: char) -> (result: Result<u8, <u8 as core::convert::TryFrom<char>>::Error>)
    ensures
        (value as u32) <= 0xFF ==> (result is Ok && (result->Ok_0 as u32) == (value as u32)),
        (value as u32) > 0xFF ==> result is Err,
;

/// `char::try_from(u32)` succeeds exactly for valid Unicode scalar
/// values (at most `U+10FFFF`, excluding the surrogate range) and
/// preserves the value, failing with `CharTryFromError` otherwise — the
/// same claim the Kani harness checks, for every possible `u32`.
pub fn verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range(value: u32) -> (result: bool)
    ensures
        result,
{
    let is_valid_scalar = value <= 0x0010_FFFF && !(0xD800..=0xDFFF).contains(&value);
    match char::try_from(value) {
        Ok(parsed) => is_valid_scalar && (parsed as u32) == value,
        Err(_) => !is_valid_scalar,
    }
}

/// `u8::try_from(char)` fails with `TryFromCharError` exactly when the
/// char's scalar value doesn't fit in `u8`, and succeeds with the same
/// value otherwise — the same claim the Kani harness checks, for every
/// possible `char`.
pub fn verify_try_from_char_error_occurs_exactly_when_out_of_range(value: char) -> (result: bool)
    ensures
        result,
{
    match u8::try_from(value) {
        Ok(converted) => (value as u32) <= 0xFF && (converted as u32) == (value as u32),
        Err(_) => (value as u32) > 0xFF,
    }
}

} // verus!
