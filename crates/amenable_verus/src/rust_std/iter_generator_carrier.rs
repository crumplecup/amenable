//! Verus accommodation model for `core::iter::{Once, OnceWith, Repeat,
//! RepeatWith, RepeatN, Empty}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Each of these six real types checks a "how many times, and
//! what, does this generator yield" claim in `amenable_kani` — this
//! carrier states each one directly rather than modeling repeated
//! `.next()` calls against iterator state. `RepeatWith`'s real claim
//! (checked in `amenable_kani` through a shared atomic call counter) is
//! "never caches, calls its closure fresh every time" — modeled here by
//! taking each call's already-computed value as a *separate* symbolic
//! parameter and proving each is returned exactly as given: if the
//! model cached, a later call would return an earlier parameter's value
//! instead, which the postcondition rules out, the same "prove no
//! caching by varying the input each call" shape
//! `lazy_cell_carrier.rs` uses (there, to prove caching *does* happen).
//! None of these functions are `Once`/`OnceWith`/`Repeat`/
//! `RepeatWith`/`RepeatN`/`Empty` themselves — each proof is
//! conditional: sound if the real type refines the stated behavior,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real type directly) already confirms independently, for the
//! identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_option_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `once(value)::next` yields the value once, then `None`.
pub fn verify_once_model_yields_exactly_one_value(value: i32) -> (result: (Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, value),
        result.1 is None,
{
    (Some(value), None)
}

/// `once_with(f)::next` yields `f()`'s one computed value, then `None`.
pub fn verify_once_with_model_calls_its_closure_exactly_once(computed: i32) -> (result: (Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, computed),
        result.1 is None,
{
    (Some(computed), None)
}

/// `repeat(value)::next` yields the same value on every call.
pub fn verify_repeat_model_yields_the_same_value_forever(value: i32) -> (result: (i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, value as int),
        observed_value_matches_input(result.1 as int, value as int),
        observed_value_matches_input(result.2 as int, value as int),
{
    (value, value, value)
}

/// `repeat_with(f)::next` calls `f` fresh on every call rather than
/// caching — modeled by taking each call's already-computed value as
/// its own symbolic parameter and returning each one exactly as given.
pub fn verify_repeat_with_model_calls_its_closure_once_per_item(first: i32, second: i32, third: i32) -> (result: (i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, first as int),
        observed_value_matches_input(result.1 as int, second as int),
        observed_value_matches_input(result.2 as int, third as int),
{
    (first, second, third)
}

/// `repeat_n(value, 2)::next` yields `value` exactly twice, then stops.
pub fn verify_repeat_n_model_yields_the_value_exactly_n_times(value: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, value),
        observed_option_matches_input(result.1, value),
        result.2 is None,
{
    (Some(value), Some(value), None)
}

/// `empty::<i32>()::next` never yields a value.
pub fn verify_empty_model_yields_nothing() -> (result: (Option<i32>, Option<i32>))
    ensures
        result.0 is None,
        result.1 is None,
{
    (None, None)
}

} // verus!
