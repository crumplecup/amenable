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
    creusot, VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `Cow` stores either a borrowed or owned value, and
        /// destructuring the enum recovers that wrapped `i32`
        /// unchanged.
        ///
        /// `creusot-std` 0.11.0 ships no contracts for
        /// `alloc::borrow::Cow`, and calling uncontracted external
        /// methods such as `Deref::deref` or `Cow::into_owned` would
        /// poison the whole goal. So this uses only local construction
        /// and pattern matching on the enum itself.
        #[requires(true)]
        #[ensures(cow_destructure_recovers_the_wrapped_value(value, result))]
        fn verify_cow_destructure_recovers_the_wrapped_value(value: Cow<'static, i32>) -> i32 {
            match value {
                Cow::Borrowed(borrowed) => *borrowed,
                Cow::Owned(owned) => owned,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Cow<'static,
        /// i32>>` postcondition -- real, callable Pearlite content,
        /// not just descriptive text alongside it.
        #[logic(open)]
        fn cow_destructure_recovers_the_wrapped_value(
            value: Cow<'static, i32>,
            cow_result: i32,
        ) -> bool {
            pearlite! {
                match value {
                    Cow::Borrowed(borrowed) => cow_result == *borrowed,
                    Cow::Owned(owned) => cow_result == owned,
                }
            }
        }
    }
}
