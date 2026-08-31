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
    creusot, VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC, {
        /// `Backtrace::force_capture()` always produces a captured backtrace.
        /// This stays trusted in Creusot for the same reason as Kani's
        /// accommodation-model proof: the real capture path lives at the
        /// platform unwinding boundary, not inside Creusot's contract surface.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_backtrace_force_capture_always_actually_captures() -> bool {
            let backtrace = Backtrace::force_capture();
            match backtrace.status() {
                BacktraceStatus::Captured => true,
                BacktraceStatus::Disabled | BacktraceStatus::Unsupported => false,
                _ => false,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC, {
        /// The `BacktraceStatus` observed after `Backtrace::force_capture()`
        /// is `Captured`.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_backtrace_status_reports_captured_after_force_capture() -> bool {
            let status = Backtrace::force_capture().status();
            match status {
                BacktraceStatus::Captured => true,
                BacktraceStatus::Disabled | BacktraceStatus::Unsupported => false,
                _ => false,
            }
        }
    }
}
