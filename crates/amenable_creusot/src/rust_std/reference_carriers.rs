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
    creusot, VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// Dereferencing a shared reference recovers exactly the value it
        /// borrows.
        #[requires(true)]
        #[ensures(shared_reference_dereferences_to_the_referent(value, result))]
        fn verify_shared_reference_dereferences_to_the_referent(value: i32) -> i32 {
            let reference = &value;
            *reference
        }
    }
}

amenable_derive::harness! {
    creusot, SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<&'static i32>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn shared_reference_dereferences_to_the_referent(
            value: i32,
            reference_result: i32,
        ) -> bool {
            pearlite! { reference_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// Dereferencing a mutable reference recovers the borrowed value,
        /// and writing through it updates the referent.
        #[requires(true)]
        #[ensures(mutable_reference_dereferences_to_and_updates_the_referent(initial, next, result))]
        fn verify_mutable_reference_dereferences_to_and_updates_the_referent(
            initial: i32,
            next: i32,
        ) -> (i32, i32) {
            let mut value = initial;
            let reference = &mut value;
            let before = *reference;
            *reference = next;
            (before, *reference)
        }
    }
}

amenable_derive::harness! {
    creusot, MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<&'static mut
        /// i32>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn mutable_reference_dereferences_to_and_updates_the_referent(
            initial: i32,
            next: i32,
            reference_result: (i32, i32),
        ) -> bool {
            pearlite! { reference_result.0 == initial && reference_result.1 == next }
        }
    }
}
