//! Verus spec for `alloc::string::FromUtf16Error`.

#[cfg(verus_keep_ghost)]
use std::string::FromUtf16Error;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExFromUtf16Error(FromUtf16Error);

pub open spec fn from_utf16_result_matches_single_unit_examples(
    units: &[u16],
    result: Result<String, FromUtf16Error>,
) -> bool {
    ((units@.len() == 1 && units@[0] == 0x61) ==> result is Ok)
        && ((units@.len() == 1 && units@[0] == 0xD800) ==> result is Err)
        && ((units@.len() == 1 && units@[0] == 0xDC00) ==> result is Err)
}

pub open spec fn from_utf16_inputs_cover_valid_and_lone_surrogate_cases(
    valid: &[u16],
    lone_surrogate: &[u16],
    lone_low_surrogate: &[u16],
) -> bool {
    valid@.len() == 1
        && valid@[0] == 0x61
        && lone_surrogate@.len() == 1
        && lone_surrogate@[0] == 0xD800
        && lone_low_surrogate@.len() == 1
        && lone_low_surrogate@[0] == 0xDC00
}

pub open spec fn from_utf16_case_results_match_accept_reject_triple(result: (bool, bool, bool)) -> bool {
    result.0 && result.1 && result.2
}

pub assume_specification [String::from_utf16] (units: &[u16]) -> (result: Result<String, FromUtf16Error>)
    ensures
        from_utf16_result_matches_single_unit_examples(units, result),
;

/// A valid code unit is accepted, and a lone surrogate half (high or
/// low, with no pair) is rejected — the same three cases the Kani
/// harness checks. Takes `valid`/`lone_surrogate`/`lone_low_surrogate`
/// as `requires`-constrained parameters (the same "parameter, not
/// inline literal" shape as `option_carrier.rs`) rather than the array
/// literals Kani constructs inline.
pub fn verify_from_utf16_rejects_a_lone_surrogate(valid: &[u16], lone_surrogate: &[u16], lone_low_surrogate: &[u16]) -> (result: (bool, bool, bool))
    requires
        from_utf16_inputs_cover_valid_and_lone_surrogate_cases(
            valid,
            lone_surrogate,
            lone_low_surrogate,
        ),
    ensures
        from_utf16_case_results_match_accept_reject_triple(result),
{
    let accepted = String::from_utf16(valid).is_ok();
    let rejected_high = String::from_utf16(lone_surrogate).is_err();
    let rejected_low = String::from_utf16(lone_low_surrogate).is_err();
    (accepted, rejected_high, rejected_low)
}

} // verus!
