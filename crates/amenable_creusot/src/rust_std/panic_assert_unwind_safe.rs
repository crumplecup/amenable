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
    creusot, VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// `AssertUnwindSafe` is a transparent tuple wrapper around the
        /// carried value; Creusot can prove the wrapper-level round trip
        /// directly through its public field.
        #[requires(true)]
        #[ensures(assert_unwind_safe_derefs_transparently(value, updated, result))]
        fn verify_assert_unwind_safe_derefs_transparently(
            value: i32,
            updated: i32,
        ) -> (i32, i32) {
            let mut wrapped = AssertUnwindSafe(value);
            let first = wrapped.0;
            wrapped.0 = updated;
            (first, wrapped.0)
        }
    }
}

amenable_derive::harness! {
    creusot, ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// AssertUnwindSafe<i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn assert_unwind_safe_derefs_transparently(
            value: i32,
            updated: i32,
            wrapper_result: (i32, i32),
        ) -> bool {
            pearlite! { wrapper_result.0 == value && wrapper_result.1 == updated }
        }
    }
}
