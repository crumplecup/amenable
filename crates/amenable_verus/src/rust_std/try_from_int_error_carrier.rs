//! Verus spec for `std::num::TryFromIntError`.
//!
//! Matches Kani's own claim scope exactly: Kani checks this over every
//! possible `i32` via symbolic execution (`kani::any()`), not a
//! representative sample. This proof does the same via a real universal
//! parameter, not a handful of concrete cases — the same strength, just
//! reached through `verus`'s SMT-based reasoning instead of Kani's
//! model-checking.
//!
//! No local `assume_specification` for `u8::try_from` here — `vstd`
//! already declares one (`vstd::std_specs::convert`'s
//! `impl_int_try_from_spec!` macro instantiation for `i32 => [u8 ...]`),
//! with exactly the semantics this claim needs
//! (`Self::MIN <= v <= Self::MAX` then `Ok(v as Self)` else `Err`, and
//! `obeys_try_from_spec()` unconditionally `true` for this pair). A
//! second, local `assume_specification` for the identical trait-method
//! instantiation doesn't produce a diagnosed conflict — it crashes
//! `verus` outright (`thread 'rustc' panicked at vir/src/traits.rs:511:
//! assertion failed: !method_impls.contains(&p)`), confirmed by removing
//! it and switching to relying on `vstd`'s existing spec, which fixed it.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `u8::try_from` fails with `TryFromIntError` exactly when the source
/// value doesn't fit in `u8`, and succeeds with the same value otherwise
/// — the same claim the Kani harness checks, for every possible `i32`.
pub fn verify_try_from_int_error_occurs_exactly_when_out_of_range(value: i32) -> (result: bool)
    ensures
        result,
{
    match <u8 as std::convert::TryFrom<i32>>::try_from(value) {
        Ok(converted) => (0 <= value && value <= u8::MAX as i32) && converted == value as u8,
        Err(_) => value < 0 || value > u8::MAX as i32,
    }
}

} // verus!
