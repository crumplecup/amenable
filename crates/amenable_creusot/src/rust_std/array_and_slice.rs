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
    creusot, INDEXING_AND_LENGTH_HOLDS_SRC, {
        /// The `amenable_std::IndexingAndLength` postcondition -- real,
        /// callable Pearlite content, not just descriptive text
        /// alongside it. A three-element container's reported length
        /// is `3`, and each index recovers the element it was
        /// constructed with.
        #[logic(open)]
        fn indexing_and_length_holds(
            a: i32,
            b: i32,
            c: i32,
            container_result: (usize, i32, i32, i32),
        ) -> bool {
            pearlite! {
                container_result.0 == 3usize
                    && container_result.1 == a
                    && container_result.2 == b
                    && container_result.3 == c
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC, {
        /// A fixed-size array's length is its compile-time-known element
        /// count, and each index recovers the element stored there.
        #[requires(true)]
        #[ensures(indexing_and_length_holds(a, b, c, result))]
        fn verify_array_indexing_and_length(a: i32, b: i32, c: i32) -> (usize, i32, i32, i32) {
            let arr = [a, b, c];
            (arr.len(), arr[0], arr[1], arr[2])
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SLICE_INDEXING_AND_LENGTH_SRC, {
        /// A slice's `.len()` reports the number of elements it views,
        /// and indexing recovers the underlying elements in order.
        #[requires(true)]
        #[ensures(indexing_and_length_holds(a, b, c, result))]
        fn verify_slice_indexing_and_length(a: i32, b: i32, c: i32) -> (usize, i32, i32, i32) {
            let arr = [a, b, c];
            let slice: &[i32] = &arr;
            (slice.len(), slice[0], slice[1], slice[2])
        }
    }
}

// `creusot-std` 0.11.0 ships enough contracts to keep the core borrowed
// slice iterators below fully checked. The other slice carriers we cover in
// `amenable_std` still need trusted boundaries today: predicate-driven
// `ChunkBy`/`ChunkByMut` and `Split`/`RSplit`/`*N`/`*Mut` all hit the same
// "contractless external + missing `IteratorSpec`" boundary, chunk/window and
// reverse-chunk iterators still lack borrowed-carrier iterator contracts (and
// the forward family also hit slice-pattern translation ICEs while being
// reduced), `EscapeAscii` hits the same missing iterator contracts, and
// `get_disjoint_mut` remains contractless. Those laws stay behind explicit
// trusted boundaries instead of pretending to be checked here.
amenable_derive::harness! {
    creusot, SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<std::slice::
        /// Iter<'static, i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn slice_iter_yields_shared_references_in_order(
            a: i32,
            b: i32,
            c: i32,
            slice_iter_result: (Option<i32>, Option<i32>, Option<i32>, bool),
        ) -> bool {
            pearlite! {
                match slice_iter_result {
                    (first_seen, second_seen, third_seen, exhausted) =>
                        first_seen == Some(a)
                            && second_seen == Some(b)
                            && third_seen == Some(c)
                            && exhausted,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, {
        /// `slice::Iter` yields shared references to each element in
        /// order, then ends.
        #[requires(true)]
        #[ensures(slice_iter_yields_shared_references_in_order(a, b, c, result))]
        fn verify_slice_iter_yields_shared_references_in_order(
            a: i32,
            b: i32,
            c: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            let data = [a, b, c];
            let mut it = data.iter();
            let first_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let second_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let third_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let exhausted = match it.next() {
                Some(_) => false,
                None => true,
            };
            (first_seen, second_seen, third_seen, exhausted)
        }
    }
}

amenable_derive::harness! {
    creusot, SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<std::slice::
        /// IterMut<'static, i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn slice_iter_mut_yields_mutable_references_that_write_through(
            a: i32,
            b: i32,
            updated_a: i32,
            updated_b: i32,
            slice_iter_mut_result: (Option<i32>, Option<i32>, bool, i32, i32),
        ) -> bool {
            pearlite! {
                match slice_iter_mut_result {
                    (first_seen, second_seen, exhausted, final_first, final_second) =>
                        first_seen == Some(a)
                            && second_seen == Some(b)
                            && exhausted
                            && final_first == updated_a
                            && final_second == updated_b,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC, {
        /// `slice::IterMut` yields mutable references in order, and
        /// writes through them update the underlying slice.
        #[requires(true)]
        #[ensures(slice_iter_mut_yields_mutable_references_that_write_through(a, b, updated_a, updated_b, result))]
        fn verify_slice_iter_mut_yields_mutable_references_that_write_through(
            a: i32,
            b: i32,
            updated_a: i32,
            updated_b: i32,
        ) -> (Option<i32>, Option<i32>, bool, i32, i32) {
            let mut data = [a, b];
            let (first_seen, second_seen, exhausted) = {
                let mut it = data.iter_mut();
                let first_seen = match it.next() {
                    Some(r) => {
                        let seen = *r;
                        *r = updated_a;
                        Some(seen)
                    }
                    None => None,
                };
                let second_seen = match it.next() {
                    Some(r) => {
                        let seen = *r;
                        *r = updated_b;
                        Some(seen)
                    }
                    None => None,
                };
                let exhausted = match it.next() {
                    Some(_) => false,
                    None => true,
                };
                (first_seen, second_seen, exhausted)
            };
            (first_seen, second_seen, exhausted, data[0], data[1])
        }
    }
}
