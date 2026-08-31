//! Verus spec for `Result<i32, i32>`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_value_matches_input;

verus! {

/// The precondition `verify_result_unwrap_returns_the_ok_value` assumes:
/// `res` is exactly `Ok(value)`.
pub open spec fn result_wraps_the_given_ok_value(res: Result<i32, i32>, value: i32) -> bool {
    res == Result::<i32, i32>::Ok(value)
}

/// The precondition `verify_result_unwrap_err_returns_the_err_value`
/// assumes: `res` is exactly `Err(err_value)`.
pub open spec fn result_wraps_the_given_err_value(res: Result<i32, i32>, err_value: i32) -> bool {
    res == Result::<i32, i32>::Err(err_value)
}

/// `unwrap` returns exactly the value an `Ok` wraps — the same claim the
/// Kani/Creusot harnesses check. Same "parameter, not inline literal"
/// shape as `option_carrier.rs`'s proofs, for the same reason: avoids a
/// real `clippy::unnecessary_literal_unwrap` trip while proving an
/// equally real, more general claim.
pub fn verify_result_unwrap_returns_the_ok_value(res: Result<i32, i32>, _value: i32) -> (result: i32)
    requires
        result_wraps_the_given_ok_value(res, _value),
    ensures
        observed_value_matches_input(result as int, _value as int),
{
    res.expect("requires guarantees res is Ok")
}

/// `unwrap_err` returns exactly the value an `Err` wraps — the same claim
/// the Kani/Creusot harnesses check. `vstd` has no spec for
/// `Result::expect_err` (unlike `expect`, which it does cover), so the
/// `Ok` arm falls back to the same real-verus-vs-plain-rustc split
/// `int_error_kind_carrier.rs` uses for its own proven-impossible
/// branches: `unreached()` under the real `verus` toolchain (backed by
/// the `requires` clause above, checked by the SMT solver), `
/// unreachable!()` as the ordinary-rustc fallback `verus_keep_ghost`
/// doesn't set.
pub fn verify_result_unwrap_err_returns_the_err_value(res: Result<i32, i32>, _err_value: i32) -> (result: i32)
    requires
        result_wraps_the_given_err_value(res, _err_value),
    ensures
        observed_value_matches_input(result as int, _err_value as int),
{
    match res {
        Err(value) => value,
        #[cfg(verus_keep_ghost)]
        Ok(_) => unreached(),
        #[cfg(not(verus_keep_ghost))]
        Ok(_) => unreachable!(),
    }
}

} // verus!
