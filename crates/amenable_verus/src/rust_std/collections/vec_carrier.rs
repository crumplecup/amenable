//! Verus spec for `alloc::vec::Vec<i32>`.
//!
//! Covers the push/pop/len/index half of Kani's claim only — the same
//! genuine category difference `box_carrier.rs` documents for `Box<i32>`:
//! Kani's harness also checks Drop-glue timing (a separate witness type
//! confirms `pop` transfers a value without dropping it, and that
//! dropping the `Vec` drops every remaining element exactly once), a
//! property about the order/count of `Drop::drop` invocations, not the
//! kind of pre/postcondition Verus states.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_option_matches_input, observed_value_matches_input,
};

verus! {

/// A singleton claim: a `Vec`'s length after exactly one push is always
/// the literal `1`. Named, not inlined, so the assumption has an
/// explicit source even though nothing else calls it.
pub open spec fn vec_len_after_one_push_is_one(len: usize) -> bool {
    len == 1
}

/// `push` appends, `len`/indexing observe the pushed value, `pop`
/// returns the last pushed value and leaves the `Vec` empty, and
/// popping an exhausted `Vec` returns `None` — the push/pop/len/index
/// half of the claim the Kani harness checks.
pub fn verify_vec_push_pop_round_trips(value: i32) -> (result: (usize, i32, Option<i32>, bool, Option<i32>))
    ensures
        vec_len_after_one_push_is_one(result.0),
        observed_value_matches_input(result.1 as int, value as int),
        observed_option_matches_input(result.2, value),
        result.3,
        result.4 is None,
{
    let mut v: Vec<i32> = Vec::new();
    v.push(value);
    let len_after_push = v.len();
    let indexed = v[0];
    let popped = v.pop();
    let empty_after_pop = v.is_empty();
    let popped_again = v.pop();

    (len_after_push, indexed, popped, empty_after_pop, popped_again)
}

} // verus!
