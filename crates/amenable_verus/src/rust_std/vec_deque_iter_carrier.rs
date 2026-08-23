//! Verus spec for `alloc::collections::vec_deque::Iter<'_, i32>`.
//!
//! `vstd` gives `VecDeque::iter()` real spec support (`std_specs/
//! vecdeque.rs`): `spec_iter` ties the returned `Iter`'s `remaining()` to
//! the deque's own `@` view via `axiom_spec_iter`, through the same
//! generic `Iterator`/`FromIterator` trait-extension machinery
//! `vec_into_iter_carrier.rs` documents — so, matching that carrier,
//! order preservation is proved via `.collect()`, not a raw `.next()`
//! call (unsupported for concrete iterator types today; see that
//! carrier's module doc comment for the confirming evidence). `VecDeque`
//! has no owned `IntoIter` spec support at all (only the borrowing
//! `iter()`), so this covers `vec_deque::Iter` specifically, not
//! `vec_deque::IntoIter`.

use std::collections::VecDeque;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;
#[cfg(verus_keep_ghost)]
use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

verus! {

/// `VecDeque::iter().collect()` round-trips the deque's elements by
/// reference, in order.
pub fn verify_vec_deque_iter_round_trips_via_collect(a: i32, b: i32) -> (result: bool)
    ensures
        result,
{
    broadcast use group_vec_dequeue_axioms;

    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(a);
    dq.push_back(b);

    let collected: Vec<&i32> = dq.iter().collect();
    assert(collected@.len() == 2);
    assert(*collected[0] == a);
    assert(*collected[1] == b);

    collected.len() == 2 && *collected[0] == a && *collected[1] == b
}

} // verus!
