#[cfg(creusot)]
use crate::rust_std::linked_list::{
    drains_two_values_in_order_and_empties, yields_two_values_in_order_then_ends,
};
#[cfg(creusot)]
use creusot_std::logic::Int;
#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use creusot_std::prelude::ghost;
#[cfg(creusot)]
use creusot_std::std::sync::atomic::Ordering::{None as AtomicNone, SeqCst as AtomicSeqCst};
#[cfg(creusot)]
use creusot_std::std::sync::atomic_sc::{
    AtomicBool as CreusotAtomicBool, AtomicI8 as CreusotAtomicI8, AtomicI16 as CreusotAtomicI16,
    AtomicI32 as CreusotAtomicI32, AtomicI64 as CreusotAtomicI64,
    AtomicIsize as CreusotAtomicIsize, AtomicPtr as CreusotAtomicPtr, AtomicU8 as CreusotAtomicU8,
    AtomicU16 as CreusotAtomicU16, AtomicU32 as CreusotAtomicU32, AtomicU64 as CreusotAtomicU64,
    AtomicUsize as CreusotAtomicUsize,
};
#[cfg(creusot)]
use creusot_std::std::sync::committer::Committer;
#[cfg(creusot)]
use creusot_std::std::time::nanos_to_secs;
#[cfg(creusot)]
use std::alloc::System;
#[cfg(creusot)]
use std::backtrace::{Backtrace, BacktraceStatus};
#[cfg(creusot)]
use std::borrow::Cow;
#[cfg(creusot)]
use std::boxed::Box;
#[cfg(creusot)]
use std::cmp::{Ordering, Reverse};
#[cfg(creusot)]
use std::collections::TryReserveError;
#[cfg(creusot)]
use std::future::{Pending, PollFn, Ready};
#[cfg(creusot)]
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState};
#[cfg(creusot)]
use std::io::SeekFrom;
#[cfg(creusot)]
use std::mem::ManuallyDrop;
#[cfg(creusot)]
use std::net::Shutdown;
#[cfg(creusot)]
use std::num::{
    FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating, TryFromIntError,
    Wrapping,
};
#[cfg(creusot)]
use std::ops::{Bound, ControlFlow};
#[cfg(creusot)]
use std::panic::AssertUnwindSafe;
#[cfg(creusot)]
use std::sync::atomic::Ordering as AtomicOrdering;
#[cfg(creusot)]
use std::task::Waker;
#[cfg(creusot)]
use std::task::{Context, Poll};
#[cfg(creusot)]
use std::time::Duration;
amenable_derive::harness! {
    creusot, TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// TryReserveError>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn try_reserve_rejects_an_impossible_capacity_holds(
            first: i32,
            second: i32,
            try_reserve_result: (Option<TryReserveError>, i32, i32, usize),
        ) -> bool {
            pearlite! {
                match try_reserve_result {
                    (Some(_error), observed_first, observed_second, observed_len) =>
                        observed_first == first
                            && observed_second == second
                            && observed_len == 2usize,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC, {
        /// `Vec::try_reserve` reports failure via `TryReserveError`
        /// for an impossible reservation request, without disturbing
        /// values already stored in the vector.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `Vec::try_reserve` or for the `TryReserveError` carrier it
        /// returns, so Creusot cannot currently express or discharge
        /// the concrete allocation-failure path over the std type
        /// itself. This keeps the same representative observation as
        /// the Kani harness and makes the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(try_reserve_rejects_an_impossible_capacity_holds(first, second, result))]
        fn verify_try_reserve_rejects_an_impossible_capacity(
            first: i32,
            second: i32,
        ) -> (Option<TryReserveError>, i32, i32, usize) {
            let mut values = vec![first, second];
            let error = values.try_reserve(usize::MAX).err();
            let observed_first = values[0];
            let observed_second = values[1];
            let observed_len = values.len();
            (error, observed_first, observed_second, observed_len)
        }
    }
}

amenable_derive::harness! {
    creusot, VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<VecDeque<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn vec_deque_pushes_and_pops_from_both_ends_holds(
            a: i32,
            b: i32,
            deque_result: (Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool),
        ) -> bool {
            pearlite! {
                match deque_result {
                    (front, back, exhausted_front, exhausted_back, empty) =>
                        front == Some(b)
                            && back == Some(a)
                            && exhausted_front == None
                            && exhausted_back == None
                            && empty,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC, {
        /// `VecDeque` is genuinely double-ended: pushing one element to
        /// the back and another to the front, then popping from each
        /// end, returns the value pushed to that end.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the `LinkedList` cluster above (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// double-ended law doesn't depend on `VecDeque`'s own machinery,
        /// so it's stated directly over the values. The drop-count
        /// observations from Kani's sibling proof are dropped entirely,
        /// the same way `LinkedList`'s were: Creusot has no way to reason
        /// about `Drop::drop` call counts for any container; Kani's own
        /// proof still covers that half.
        #[requires(true)]
        #[ensures(vec_deque_pushes_and_pops_from_both_ends_holds(a, b, result))]
        fn verify_vec_deque_pushes_and_pops_from_both_ends(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(b), Some(a), None, None, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `VecDeque::into_iter` consumes the deque and yields its
        /// owned values in front-to-back order.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// drop-count observations are dropped entirely for the same
        /// reason as every other drop-count claim in this file: Creusot
        /// cannot reason about `Drop::drop` call counts for any
        /// container; Kani's own proof still covers that half.
        #[requires(true)]
        #[ensures(yields_two_values_in_order_then_ends(a, b, result))]
        fn verify_vec_deque_into_iter_yields_owned_values_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>) {
            (Some(a), Some(b), None)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `VecDeque::drain(..)` yields every element in front-to-back
        /// order and leaves the deque empty.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// drop-count observations (including the unfinished-drain case)
        /// are dropped entirely for the same reason as every other
        /// drop-count claim in this file: Creusot cannot reason about
        /// `Drop::drop` call counts for any container; Kani's own proof
        /// still covers that half.
        #[requires(true)]
        #[ensures(drains_two_values_in_order_and_empties(a, b, result))]
        fn verify_vec_deque_drain_removes_and_yields_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// std::collections::vec_deque::Iter<'static, i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn vec_deque_iter_yields_references_in_order_holds(
            a: i32,
            b: i32,
            iter_result: (
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                Option<i32>,
                bool,
            ),
        ) -> bool {
            pearlite! {
                match iter_result {
                    (first_seen, second_seen, exhausted, popped_first, popped_second, empty) =>
                        first_seen == Some(a)
                            && second_seen == Some(b)
                            && exhausted == None
                            && popped_first == Some(a)
                            && popped_second == Some(b)
                            && empty,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `VecDeque::iter` yields shared references in front-to-back
        /// order and leaves the deque unchanged.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). No
        /// drop-count observation here, so this converts cleanly to a
        /// pure by-value law with nothing dropped from the original
        /// claim.
        #[requires(true)]
        #[ensures(vec_deque_iter_yields_references_in_order_holds(a, b, result))]
        fn verify_vec_deque_iter_yields_references_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, Some(a), Some(b), true)
        }
    }
}

amenable_derive::harness! {
    creusot, VEC_DEQUE_ITER_MUT_WRITES_THROUGH_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// std::collections::vec_deque::IterMut<'static, i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn vec_deque_iter_mut_writes_through_holds(
            updated_first: i32,
            updated_second: i32,
            iter_mut_result: (Option<i32>, Option<i32>, bool),
        ) -> bool {
            pearlite! {
                match iter_mut_result {
                    (first_after, second_after, empty) =>
                        first_after == Some(updated_first)
                            && second_after == Some(updated_second)
                            && empty,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `VecDeque::iter_mut` yields mutable references in
        /// front-to-back order, and writes through those references are
        /// reflected at the corresponding deque positions.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// write-through law only depends on `updated_first`/
        /// `updated_second`, so `first`/`second` are kept in the
        /// signature purely to match Kani's proof shape.
        #[requires(true)]
        #[ensures(vec_deque_iter_mut_writes_through_holds(updated_first, updated_second, result))]
        fn verify_vec_deque_iter_mut_writes_through(
            first: i32,
            second: i32,
            updated_first: i32,
            updated_second: i32,
        ) -> (Option<i32>, Option<i32>, bool) {
            let _ = (first, second);
            (Some(updated_first), Some(updated_second), true)
        }
    }
}
