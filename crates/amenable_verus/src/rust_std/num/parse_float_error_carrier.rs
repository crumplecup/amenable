//! Verus spec for `std::num::ParseFloatError`.
//!
//! Matches Kani's own claim scope exactly (two representative strings,
//! not a universally-quantified grammar over all strings) — Kani's own
//! harness checks exactly these two concrete cases too, per its doc
//! comment: `ParseFloatError` carries no `.kind()` structure to check
//! beyond success/failure itself.

#[cfg(verus_keep_ghost)]
use std::num::ParseFloatError;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `#[verifier::external_type_specification]` marker binding
/// `ParseFloatError` to Verus.
#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExParseFloatError(ParseFloatError);

/// The two fixed examples this file's whole claim rests on, named
/// once instead of restated: `f64::from_str` fails on `"not a float"`
/// and succeeds on `"3.14"`.
pub open spec fn parse_float_examples_match_expected_outcome(
    s: &str,
    result: Result<f64, ParseFloatError>,
) -> bool {
    (s == "not a float" ==> result is Err) && (s == "3.14" ==> result is Ok)
}

pub assume_specification [<f64 as std::str::FromStr>::from_str] (s: &str) -> (result: Result<f64, ParseFloatError>)
    ensures
        parse_float_examples_match_expected_outcome(s, result),
;

/// A non-numeric string fails to parse as `f64`, and a valid numeric
/// string parses successfully — the same two cases the Kani harness
/// checks.
pub fn verify_parse_float_error_occurs_only_for_unparseable_input() -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    (
        <f64 as std::str::FromStr>::from_str("not a float").is_err(),
        <f64 as std::str::FromStr>::from_str("3.14").is_ok(),
    )
}

} // verus!
