//! Verus accommodation model for `core::char::{DecodeUtf16, DecodeUtf16Error}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `char::decode_utf16` has no `vstd` spec support at all.
//! `VerusDecodeUtf16Model` models the two laws
//! `amenable_kani::rust_std::char`'s own harnesses check against a
//! single, symbolic code unit (matching that module's own documented
//! choice to check one code unit at a time, not a `.collect()`ed
//! sequence): a non-surrogate BMP code unit always decodes successfully
//! to the scalar value equal to the code unit itself, and a lone
//! surrogate always fails, reporting the exact unpaired code unit.
//! Returns the decoded scalar value (`u32`) rather than constructing a
//! real `char` — the same numeric claim
//! `verify_decode_utf16_round_trips_a_bmp_code_unit`'s own assertion
//! checks (`decoded as u32 == u32::from(unit)`), without pulling in
//! `char` construction machinery this model doesn't otherwise need.
//! `VerusDecodeUtf16Model` isn't `DecodeUtf16`/`DecodeUtf16Error` and
//! doesn't claim to be — the proof is conditional: sound if the real
//! types refine these laws, which `amenable_kani`'s own
//! `verify_decode_utf16_round_trips_a_bmp_code_unit`/
//! `verify_decode_utf16_error_reports_the_unpaired_surrogate` harnesses
//! (checking the real `char::decode_utf16` directly) already confirm
//! independently, for the identical claims.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Models the two laws `char::decode_utf16` is expected to satisfy for
/// a single code unit — not `DecodeUtf16`/`DecodeUtf16Error` themselves.
pub struct VerusDecodeUtf16Model;

pub open spec fn decode_utf16_unit_is_non_surrogate(unit: u16) -> bool {
    unit < 0xD800 || unit > 0xDFFF
}

pub open spec fn decode_utf16_bmp_unit_decodes_to_same_scalar(
    unit: u16,
    result: Option<u32>,
) -> bool {
    result == Some(unit as u32)
}

pub open spec fn decode_utf16_unit_is_surrogate(unit: u16) -> bool {
    0xD800 <= unit <= 0xDFFF
}

pub open spec fn decode_utf16_lone_surrogate_reports_same_unit(
    unit: u16,
    result: Result<u32, u16>,
) -> bool {
    result == Err(unit)
}

impl VerusDecodeUtf16Model {
    /// A non-surrogate BMP code unit always decodes successfully, to
    /// the scalar value equal to the code unit itself.
    pub fn decode_bmp_unit(unit: u16) -> (result: Option<u32>)
        requires
            decode_utf16_unit_is_non_surrogate(unit),
        ensures
            decode_utf16_bmp_unit_decodes_to_same_scalar(unit, result),
    {
        Some(unit as u32)
    }

    /// A lone surrogate always fails to decode, reporting the exact
    /// unpaired code unit.
    pub fn decode_lone_surrogate(unit: u16) -> (result: Result<u32, u16>)
        requires
            decode_utf16_unit_is_surrogate(unit),
        ensures
            decode_utf16_lone_surrogate_reports_same_unit(unit, result),
    {
        Err(unit)
    }
}

pub open spec fn decode_utf16_test_inputs_cover_both_cases(
    bmp_unit: u16,
    lone_surrogate: u16,
) -> bool {
    decode_utf16_unit_is_non_surrogate(bmp_unit)
        && decode_utf16_unit_is_surrogate(lone_surrogate)
}

/// A non-surrogate BMP code unit decodes to the scalar value equal to
/// the code unit itself, and a lone high surrogate fails, reporting the
/// exact unpaired code unit — the laws `char::decode_utf16` is expected
/// to refine.
pub fn verify_decode_utf16_model_round_trips_and_reports_lone_surrogates(bmp_unit: u16, lone_surrogate: u16) -> (result: (bool, bool))
    requires
        decode_utf16_test_inputs_cover_both_cases(bmp_unit, lone_surrogate),
    ensures
        result.0,
        result.1,
{
    let decoded = VerusDecodeUtf16Model::decode_bmp_unit(bmp_unit);
    let round_trips = decoded == Some(bmp_unit as u32);

    let failed = VerusDecodeUtf16Model::decode_lone_surrogate(lone_surrogate);
    let reports_unpaired = match failed {
        Ok(_) => false,
        Err(reported) => reported == lone_surrogate,
    };

    (round_trips, reports_unpaired)
}

} // verus!
