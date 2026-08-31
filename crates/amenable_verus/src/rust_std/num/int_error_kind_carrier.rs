//! Verus spec for `std::num::IntErrorKind` / `std::num::ParseIntError`.

use std::num::IntErrorKind;
#[cfg(verus_keep_ghost)]
use std::num::ParseIntError;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{has_length, values_are_equal};

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

/// `ParseIntError::kind()`'s own real postcondition: the accessor
/// recovers exactly the classification `parse_int_error_kind_spec`
/// (its own uninterpreted, axiom-backed spec) assigns the error.
pub open spec fn parse_int_error_kind_matches(result: IntErrorKind, err: ParseIntError) -> bool {
    result == parse_int_error_kind_spec(err)
}

pub assume_specification [ParseIntError::kind] (err: &ParseIntError) -> (result: &IntErrorKind)
    ensures
        parse_int_error_kind_matches(*result, *err),
;

/// A precondition shared by this file's own lowercase-start claim below
/// and `verify_parse_int_error_model_reports_the_kind_of_the_failure`'s
/// own `requires` clause: the string is non-empty and its first
/// character is a lowercase ASCII letter (`a`..=`z`).
pub open spec fn starts_with_lowercase_ascii_letter(s: Seq<char>) -> bool {
    s.len() > 0 && s[0] as u8 >= 97 && s[0] as u8 <= 122
}

/// The `i32::from_str` real postcondition's `Empty` conjunct: an
/// empty string fails to parse, classified exactly `IntErrorKind::Empty`.
pub open spec fn from_str_empty_reports_empty_kind(s: Seq<char>, result: Result<i32, ParseIntError>) -> bool {
    s.len() == 0 ==> result is Err && parse_int_error_kind_spec(result->Err_0) == IntErrorKind::Empty
}

/// The `i32::from_str` real postcondition's `InvalidDigit` conjunct: a
/// string starting with a lowercase ASCII letter fails to parse,
/// classified exactly `IntErrorKind::InvalidDigit`.
pub open spec fn from_str_lowercase_reports_invalid_digit_kind(s: Seq<char>, result: Result<i32, ParseIntError>) -> bool {
    starts_with_lowercase_ascii_letter(s) ==>
        result is Err && parse_int_error_kind_spec(result->Err_0) == IntErrorKind::InvalidDigit
}

pub assume_specification [<i32 as std::str::FromStr>::from_str] (s: &str) -> (result: Result<i32, ParseIntError>)
    ensures
        from_str_empty_reports_empty_kind(s@, result),
        from_str_lowercase_reports_invalid_digit_kind(s@, result),
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
        has_length(s@, 0),
    ensures
        values_are_equal(result, IntErrorKind::Empty),
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
        starts_with_lowercase_ascii_letter(s@),
    ensures
        values_are_equal(result, IntErrorKind::InvalidDigit),
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
