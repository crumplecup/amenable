//! Verus spec for `alloc::collections::VecDeque<i32>`.
//!
//! Covers the push/pop-both-ends half of Kani's claim only — the same
//! genuine category difference `box_carrier.rs`/`vec_carrier.rs`
//! document: Kani's harness also checks Drop-glue timing (a separate
//! drop-witness type confirms `pop_front` transfers a value without
//! dropping it, and that dropping the deque drops every remaining
//! element exactly once), not the kind of pre/postcondition Verus
//! states.
//!
//! `is_empty` itself has no `vstd` spec support (unlike `len`, which does
//! — `std_specs/vecdeque.rs`'s `spec_vec_dequeue_len` — and unlike
//! `push_back`/`push_front`/`pop_back`/`pop_front`, all specced generically
//! over `VecDeque<T, A>`), and declaring a local axiom for it would require
//! naming the real `A: Allocator` generic explicitly, which needs the
//! unstable `allocator_api` feature — unusable here since this crate's
//! plain `cargo build`/`clippy` check path (required by CLAUDE.md's
//! pre-commit verification) compiles under stable, not the nightly Verus
//! bundles internally. `dq.len() == 0` expresses the same claim using only
//! already-specced `len`, so no axiom (and no unstable feature) is needed
//! at all.

use std::collections::VecDeque;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `push_back`/`push_front` add to their respective ends, `pop_front`/
/// `pop_back` return the front-pushed/back-pushed elements in order and
/// leave the deque empty, and popping an exhausted end returns `None` —
/// the push/pop-both-ends half of the claim the Kani harness checks.
pub fn verify_vec_deque_pushes_and_pops_from_both_ends(a: i32, b: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool))
    ensures
        result.0 == Some(b),
        result.1 == Some(a),
        result.2 is None,
        result.3 is None,
        result.4,
{
    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(a);
    dq.push_front(b);

    let popped_front = dq.pop_front();
    let popped_back = dq.pop_back();
    let exhausted_front = dq.pop_front();
    let exhausted_back = dq.pop_back();
    let remaining_len = dq.len();
    let empty_after = remaining_len == 0;

    (popped_front, popped_back, exhausted_front, exhausted_back, empty_after)
}

} // verus!
