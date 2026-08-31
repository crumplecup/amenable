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
    creusot, RANGE_TO_CONTAINS_MATCHES_BOUND_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// std::ops::RangeTo<i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn range_to_contains_matches_bound_holds(end: i32, x: i32, range_to_result: bool) -> bool {
            pearlite! { range_to_result == (x < end) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC, {
        /// `RangeTo` is unbounded below, so membership reduces to its
        /// single exclusive upper bound.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 does not currently expose a
        /// contract surface for `RangeBounds::contains` over the concrete
        /// std range carriers, so this keeps the same law Kani checks as a
        /// named trusted boundary instead of dropping to provenance-only
        /// coverage.
        #[trusted]
        #[requires(true)]
        #[ensures(range_to_contains_matches_bound_holds(end, x, result))]
        fn verify_range_to_contains_matches_bound(end: i32, x: i32) -> bool {
            std::ops::RangeBounds::contains(&(..end), &x)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC, {
        /// `RangeFull` carries no fields, but its `..` interval contains
        /// every value.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 does not currently expose a
        /// contract surface for `RangeBounds::contains` over `RangeFull`,
        /// so this keeps the carrier's semantics explicit at the trusted
        /// boundary rather than pretending missing library contracts were
        /// discharged.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_range_full_contains_everything(x: i32) -> bool {
            std::ops::RangeBounds::contains(&(..), &x)
        }
    }
}

amenable_derive::harness! {
    creusot, BOUND_ROUND_TRIPS_ITS_ENDPOINT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Bound<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn bound_round_trips_its_endpoint_holds(
            value: Bound<i32>,
            bound_result: (bool, bool, Option<i32>),
        ) -> bool {
            pearlite! {
                match value {
                    Bound::Included(inner) => bound_result == (true, false, Some(inner)),
                    Bound::Excluded(inner) => bound_result == (false, true, Some(inner)),
                    Bound::Unbounded => bound_result == (false, false, None),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC, {
        /// `Bound` has exactly three inhabitants; the endpoint variants
        /// round-trip their payload and `Unbounded` carries no endpoint.
        #[requires(true)]
        #[ensures(bound_round_trips_its_endpoint_holds(value, result))]
        fn verify_bound_round_trips_its_endpoint(
            value: Bound<i32>,
        ) -> (bool, bool, Option<i32>) {
            match value {
                Bound::Included(inner) => (true, false, Some(inner)),
                Bound::Excluded(inner) => (false, true, Some(inner)),
                Bound::Unbounded => (false, false, None),
            }
        }
    }
}

amenable_derive::harness! {
    creusot, CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// ControlFlow<i32, i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn control_flow_continue_and_break_are_disjoint_holds(
            value: ControlFlow<i32, i32>,
            control_flow_result: (bool, bool, Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                match value {
                    ControlFlow::Continue(inner) =>
                        control_flow_result == (true, false, Some(inner), None),
                    ControlFlow::Break(inner) =>
                        control_flow_result == (false, true, None, Some(inner)),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC, {
        /// `Continue` and `Break` are mutually exclusive, and each
        /// variant carries exactly the payload the other lacks.
        #[requires(true)]
        #[ensures(control_flow_continue_and_break_are_disjoint_holds(value, result))]
        fn verify_control_flow_continue_and_break_are_disjoint(
            value: ControlFlow<i32, i32>,
        ) -> (bool, bool, Option<i32>, Option<i32>) {
            match value {
                ControlFlow::Continue(inner) => (true, false, Some(inner), None),
                ControlFlow::Break(inner) => (false, true, None, Some(inner)),
            }
        }
    }
}
