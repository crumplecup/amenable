//! Verus spec for `std::num::FpCategory`.
//!
//! `classify`'s result relates to `vstd::float::FloatBitsProperties`'s
//! real, checked `is_nan_spec`/`is_infinite_spec` predicates (not a
//! guessed relationship) — `vstd` ships genuine IEEE-754 bit-pattern
//! specs for `f64`, just no `classify`/`is_nan`/`is_infinite` EXEC-method
//! specs of its own, so this crate supplies those via
//! `assume_specification`, stated in terms of `vstd`'s own spec
//! predicates. Takes `value` as a `requires`-constrained parameter (the
//! same "parameter, not inline literal" shape as `option_carrier.rs`)
//! rather than constructing a NaN via the `f64::NAN` constant, which
//! `verus` doesn't support directly.

use std::num::FpCategory;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::float::FloatBitsProperties is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::float::FloatBitsProperties;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `#[verifier::external_type_specification]` marker binding
/// `FpCategory` to Verus.
#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
pub struct ExFpCategory(FpCategory);

/// `f64::classify`'s whole postcondition, for the two special values this
/// file tests: NaN classifies as `Nan`, an infinite value classifies as
/// `Infinite`.
pub open spec fn fp_category_classify_result_matches_special_value_categories(
    value: f64,
    result: FpCategory,
) -> bool {
    (value.is_nan_spec() ==> result == FpCategory::Nan)
        && (value.is_infinite_spec() ==> result == FpCategory::Infinite)
}

/// Precondition shared by this file's test inputs: `nan` genuinely is
/// NaN, `infinite` genuinely is infinite.
pub open spec fn fp_category_inputs_cover_nan_and_infinite_cases(nan: f64, infinite: f64) -> bool {
    nan.is_nan_spec() && infinite.is_infinite_spec()
}

/// Both of this file's example classifications resolved as expected.
pub open spec fn fp_category_results_match_nan_and_infinite_cases(
    result: (FpCategory, FpCategory),
) -> bool {
    result.0 == FpCategory::Nan && result.1 == FpCategory::Infinite
}

pub assume_specification [f64::classify] (value: f64) -> (result: FpCategory)
    ensures
        fp_category_classify_result_matches_special_value_categories(value, result),
;

/// A NaN value classifies as `FpCategory::Nan`, and an infinite value
/// classifies as `FpCategory::Infinite` — two representative cases of
/// the five-way claim the Kani/Creusot harnesses check exhaustively.
pub fn verify_fp_category_matches_the_value_it_classifies(nan: f64, infinite: f64) -> (result: (FpCategory, FpCategory))
    requires
        fp_category_inputs_cover_nan_and_infinite_cases(nan, infinite),
    ensures
        fp_category_results_match_nan_and_infinite_cases(result),
{
    (nan.classify(), infinite.classify())
}

} // verus!
