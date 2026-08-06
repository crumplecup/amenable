//! Verus spec for `std::num::IntErrorKind` / `std::num::ParseIntError`.

use std::num::IntErrorKind;
#[cfg(verus_keep_ghost)]
use std::num::ParseIntError;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
pub struct ExIntErrorKind(IntErrorKind);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExParseIntError(ParseIntError);

/// Connects `ParseIntError::kind()`'s real (but otherwise opaque to
/// `verus`) classification to the trusted axioms below: an `uninterp`
/// spec function, not a reimplementation of `kind()`'s real logic — its
/// only content is what the two `assume_specification`s below assert
/// about it.
pub uninterp spec fn parse_int_error_kind_spec(err: ParseIntError) -> IntErrorKind;

pub assume_specification [ParseIntError::kind] (err: &ParseIntError) -> (result: &IntErrorKind)
    ensures
        *result == parse_int_error_kind_spec(*err),
;

pub assume_specification [<i32 as std::str::FromStr>::from_str] (s: &str) -> (result: Result<i32, ParseIntError>)
    ensures
        s@.len() == 0 ==> result is Err && parse_int_error_kind_spec(result->Err_0) == IntErrorKind::Empty,
        (s@.len() > 0 && s@[0] as u8 >= 97 && s@[0] as u8 <= 122) ==>
            result is Err && parse_int_error_kind_spec(result->Err_0) == IntErrorKind::InvalidDigit,
;

/// An empty string fails to parse as `i32` with exactly
/// `IntErrorKind::Empty` — the same claim the Kani/Creusot harnesses
/// check for this case (one of the five variants they check
/// exhaustively; the other four all require reasoning about `i32`'s
/// numeric range during parsing, which isn't stated here). Calls
/// `FromStr::from_str` directly rather than through `str::parse`'s
/// generic wrapper: `verus` has no spec for `str::parse` itself (only
/// the underlying `from_str` it delegates to), and both exercise the
/// identical real conversion. Takes `s` as a `requires`-constrained
/// parameter (the same "parameter, not inline literal" shape as
/// `option_carrier.rs`) rather than the literal `""`, which `verus`
/// doesn't connect to `s@.len() == 0` automatically.
pub fn verify_int_error_kind_classifies_parse_failures(s: &str) -> (result: IntErrorKind)
    requires
        s@.len() == 0,
    ensures
        result == IntErrorKind::Empty,
{
    match <i32 as std::str::FromStr>::from_str(s) {
        #[cfg(verus_keep_ghost)]
        Ok(_) => unreached(),
        #[cfg(not(verus_keep_ghost))]
        Ok(_) => unreachable!(),
        Err(err) => *err.kind(),
    }
}

/// A lowercase ASCII letter can never start a valid integer literal, so
/// any such string fails to parse as `i32` with exactly
/// `IntErrorKind::InvalidDigit` — the same claim
/// `verify_parse_int_error_reports_the_kind_of_the_failure`'s Kani/
/// Creusot harnesses check for the literal `"not a number"`, generalized
/// to any string starting with a lowercase letter. Same "parameter, not
/// inline literal" shape as `verify_int_error_kind_classifies_parse_
/// failures` above, for the same reason.
pub fn verify_parse_int_error_model_reports_the_kind_of_the_failure(s: &str) -> (result: IntErrorKind)
    requires
        s@.len() > 0,
        s@[0] as u8 >= 97,
        s@[0] as u8 <= 122,
    ensures
        result == IntErrorKind::InvalidDigit,
{
    match <i32 as std::str::FromStr>::from_str(s) {
        #[cfg(verus_keep_ghost)]
        Ok(_) => unreached(),
        #[cfg(not(verus_keep_ghost))]
        Ok(_) => unreachable!(),
        Err(err) => *err.kind(),
    }
}

} // verus!
