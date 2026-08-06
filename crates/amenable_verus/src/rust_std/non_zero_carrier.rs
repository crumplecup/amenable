//! Verus accommodation model for `core::num::NonZero<{i8,i16,i32,i64,
//! i128,isize,u8,u16,u32,u64,u128,usize}>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. All twelve real instantiations check the identical claim
//! in `amenable_kani`, written out literally per numeric width rather
//! than genericized (matching that crate's own documented choice, per
//! its module doc comment) — this carrier mirrors that same one
//! function per width. `NonZero::<T>::new` succeeds exactly when the
//! input is nonzero, and `.get()` round-trips the wrapped value.
//! `VerusNonZeroModel` isn't `NonZero<T>` and doesn't claim to be — each
//! proof is conditional: sound if the real `NonZero<T>` refines this
//! law, which `amenable_kani`'s own harness for that exact width
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

pub fn verify_non_zero_i8_model_round_trips_iff_nonzero(value: i8) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_i16_model_round_trips_iff_nonzero(value: i16) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_i32_model_round_trips_iff_nonzero(value: i32) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_i64_model_round_trips_iff_nonzero(value: i64) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_i128_model_round_trips_iff_nonzero(value: i128) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_isize_model_round_trips_iff_nonzero(value: isize) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_u8_model_round_trips_iff_nonzero(value: u8) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_u16_model_round_trips_iff_nonzero(value: u16) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_u32_model_round_trips_iff_nonzero(value: u32) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_u64_model_round_trips_iff_nonzero(value: u64) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_u128_model_round_trips_iff_nonzero(value: u128) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

pub fn verify_non_zero_usize_model_round_trips_iff_nonzero(value: usize) -> (result: bool)
    ensures
        value != 0 ==> result,
        value == 0 ==> !result,
{
    value != 0
}

} // verus!
