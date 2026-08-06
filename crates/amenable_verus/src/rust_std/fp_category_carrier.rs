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
#[allow(unused_imports)]
use vstd::float::FloatBitsProperties;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
pub struct ExFpCategory(FpCategory);

pub assume_specification [f64::classify] (value: f64) -> (result: FpCategory)
    ensures
        value.is_nan_spec() ==> result == FpCategory::Nan,
        value.is_infinite_spec() ==> result == FpCategory::Infinite,
;

/// A NaN value classifies as `FpCategory::Nan`, and an infinite value
/// classifies as `FpCategory::Infinite` — two representative cases of
/// the five-way claim the Kani/Creusot harnesses check exhaustively.
pub fn verify_fp_category_matches_the_value_it_classifies(nan: f64, infinite: f64) -> (result: (FpCategory, FpCategory))
    requires
        nan.is_nan_spec(),
        infinite.is_infinite_spec(),
    ensures
        result.0 == FpCategory::Nan,
        result.1 == FpCategory::Infinite,
{
    (nan.classify(), infinite.classify())
}

} // verus!
