//! Verus spec for `Option<i32>`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;

verus! {

/// `unwrap` returns exactly the value a `Some` wraps — the same claim the
/// Kani/Creusot harnesses check, restated as a real `verus`-checked
/// postcondition. Takes `some`/`value` as parameters constrained by
/// `requires` rather than constructing `Some(value)` as a literal inline:
/// semantically identical (the same real `Option::unwrap` call, on the
/// same shape of input), but a genuinely more general claim — true for
/// any `Option<i32>` matching the precondition, not just one built inline
/// — and it sidesteps a real `clippy::unnecessary_literal_unwrap` trip
/// (confirmed: constructing `Some(value)` then immediately calling
/// `.unwrap()` on it in the same scope is exactly what that lint flags,
/// correctly, for ordinary code — this just isn't ordinary code, but
/// there's no need to fight the lint when an equally real, more general
/// contract avoids it instead).
pub fn verify_option_unwrap_returns_the_wrapped_value(some: Option<i32>, _value: i32) -> (result: i32)
    requires
        some == Some::<i32>(_value),
    ensures
        observed_value_matches_input(result as int, _value as int),
{
    some.expect("requires guarantees some is Some")
}

/// `unwrap_or` falls back to the supplied default exactly when the
/// receiver is `None` — the same claim the Kani/Creusot harnesses check.
pub fn verify_option_unwrap_or_falls_back_to_the_default(none: Option<i32>, default: i32) -> (result: i32)
    requires
        none is None,
    ensures
        observed_value_matches_input(result as int, default as int),
{
    none.unwrap_or(default)
}

} // verus!
