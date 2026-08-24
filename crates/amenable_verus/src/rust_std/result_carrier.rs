//! Verus spec for `Result<i32, i32>`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
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
        res == Result::<i32, i32>::Err(_err_value),
    ensures
        result == _err_value,
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
