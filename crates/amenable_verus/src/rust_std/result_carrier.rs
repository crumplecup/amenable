//! Verus spec for `Result<i32, i32>`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `unwrap` returns exactly the value an `Ok` wraps — the same claim the
/// Kani/Creusot harnesses check. Same "parameter, not inline literal"
/// shape as `option_carrier.rs`'s proofs, for the same reason: avoids a
/// real `clippy::unnecessary_literal_unwrap` trip while proving an
/// equally real, more general claim.
pub fn verify_result_unwrap_returns_the_ok_value(res: Result<i32, i32>, _value: i32) -> (result: i32)
    requires
        res == Result::<i32, i32>::Ok(_value),
    ensures
        result == _value,
{
    res.unwrap()
}

/// `unwrap_err` returns exactly the value an `Err` wraps — the same claim
/// the Kani/Creusot harnesses check.
pub fn verify_result_unwrap_err_returns_the_err_value(res: Result<i32, i32>, _err_value: i32) -> (result: i32)
    requires
        res == Result::<i32, i32>::Err(_err_value),
    ensures
        result == _err_value,
{
    res.unwrap_err()
}

} // verus!
