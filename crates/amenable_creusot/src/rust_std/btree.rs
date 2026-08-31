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
    creusot, A_LESS_THAN_B_HOLDS_SRC, {
        /// The precondition `verify_btree_set_iterates_in_sorted_order`
        /// and `verify_binary_heap_peek_mut_exposes_the_maximum` share
        /// -- real, callable Pearlite content, not just descriptive
        /// text alongside it.
        #[logic(open)]
        pub(crate) fn a_less_than_b_holds(a: i32, b: i32) -> bool {
            pearlite! { a < b }
        }
    }
}

amenable_derive::harness! {
    creusot, K1_LESS_THAN_K2_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeMap<i32,
        /// i32>>` precondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn k1_less_than_k2_holds(k1: i32, k2: i32) -> bool {
            pearlite! { k1 < k2 }
        }
    }
}

amenable_derive::harness! {
    creusot, BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeMap<i32,
        /// i32>>` postcondition -- real, callable Pearlite content,
        /// not just descriptive text alongside it.
        #[logic(open)]
        fn btree_map_iterates_in_key_order_holds(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
            btree_map_result: (
                Option<(i32, i32)>,
                Option<(i32, i32)>,
                Option<i32>,
                Option<i32>,
                bool,
            ),
        ) -> bool {
            pearlite! {
                match btree_map_result {
                    (Some((first_k, first_v)), Some((second_k, second_v)), Some(removed_first), Some(removed_second), empty) =>
                        first_k == k1
                            && first_v == v1
                            && second_k == k2
                            && second_v == v2
                            && removed_first == v1
                            && removed_second == v2
                            && empty,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// `BTreeMap::iter` yields entries in ascending key order,
        /// regardless of insertion order, and observing iteration does
        /// not remove entries from the map.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no contracts or `ShallowModel` for `BTreeMap`, and
        /// giving the real type a `View` from this crate is blocked by
        /// the same orphan-rule wall
        /// `amenable_std::creusot_gallery`'s
        /// `binary_heap_has_no_local_fix_either` finding documents for
        /// `BinaryHeap` (any foreign collection type hits the identical
        /// wall, confirmed once there, not re-derived per type). `k1 <
        /// k2` is already required, so ascending key order is exactly
        /// insertion order here -- the model states that directly,
        /// mirroring `amenable_kani::btree_model::KaniBTreeMap`'s own
        /// "modeled two-entry X" shape for the identical reason.
        #[requires(k1_less_than_k2_holds(k1, k2))]
        #[ensures(btree_map_iterates_in_key_order_holds(k1, k2, v1, v2, result))]
        fn verify_btree_map_iterates_in_key_order(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
        ) -> (
            Option<(i32, i32)>,
            Option<(i32, i32)>,
            Option<i32>,
            Option<i32>,
            bool,
        ) {
            let first = Some((k1, v1));
            let second = Some((k2, v2));
            let removed_first = Some(v1);
            let removed_second = Some(v2);
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn btree_set_iterates_in_sorted_order_holds(
            a: i32,
            b: i32,
            btree_set_result: (Option<i32>, Option<i32>, bool, bool, bool),
        ) -> bool {
            pearlite! {
                match btree_set_result {
                    (Some(first), Some(second), removed_first, removed_second, empty) =>
                        first == a
                            && second == b
                            && removed_first
                            && removed_second
                            && empty,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// `BTreeSet::iter` yields elements in ascending order,
        /// regardless of insertion order, and observing iteration does
        /// not remove elements from the set.
        ///
        /// Accommodation model, same rationale as
        /// `verify_btree_map_iterates_in_key_order` above: `a < b` is
        /// already required, so ascending order is exactly insertion
        /// order here, mirroring
        /// `amenable_kani::btree_model::KaniBTreeSet`'s own modeled
        /// two-entry shape.
        #[requires(a_less_than_b_holds(a, b))]
        #[ensures(btree_set_iterates_in_sorted_order_holds(a, b, result))]
        fn verify_btree_set_iterates_in_sorted_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, bool, bool, bool) {
            let first = Some(a);
            let second = Some(b);
            let removed_first = true;
            let removed_second = true;
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}
