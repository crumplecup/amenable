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
    creusot, DURATION_NEW_HEADROOM_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Duration>`
        /// precondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn duration_new_headroom_holds(secs: u64, nanos: u32) -> bool {
            pearlite! { secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@ }
        }
    }
}

amenable_derive::harness! {
    creusot, DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Duration>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it. Not `open`: it calls the
        /// opaque `creusot_std` logic fn `nanos_to_secs`, and an `open`
        /// wrapper around an opaque callee would leak that opacity
        /// boundary (same real `creusot-rustc` "less-visible item"
        /// error `string_roundtrips_and_preserves_length` hit above).
        #[logic]
        fn duration_new_normalizes_nanos_and_carries_into_secs_holds(
            secs: u64,
            nanos: u32,
            duration_result: Duration,
        ) -> bool {
            pearlite! {
                nanos_to_secs(duration_result@) == secs@ + (nanos@ / 1_000_000_000)
                    && duration_result@ % 1_000_000_000 == nanos@ % 1_000_000_000
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `Duration::new` does not require `nanos < 1_000_000_000` — it
        /// normalizes: any whole-second carry in `nanos` is added to
        /// `secs`, and `subsec_nanos()` reports the remainder. Same claim
        /// as the Kani harness (`amenable_kani::rust_std::time`), restated
        /// as a Creusot postcondition — right down to the `secs.checked_add
        /// (carry).is_some()` precondition Kani assumes, here expressed as
        /// `secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@` (Pearlite's `@`
        /// operator lifts to arbitrary-precision `Int`, so this is exactly
        /// "the real u64 addition wouldn't overflow", not an approximation).
        ///
        /// `creusot-std` ships its own trusted `extern_spec!` for
        /// `Duration::new`/`as_secs`/`subsec_nanos` (`creusot_std::std::
        /// time`) — but `#[check(ghost)]` extern-spec methods are still
        /// *program* functions, not `#[logic]` ones, so `result.as_secs()`
        /// can't be called directly inside `#[ensures]` any more than
        /// `String::len()` could — confirmed by a real translation error,
        /// not a guess: `error: called program function 'std::time::
        /// Duration::as_secs' in logic context`. Unlike `String::len`,
        /// no local `#[trusted]` wrapper is needed to route around it:
        /// `creusot_std::std::time` already exports `nanos_to_secs`/
        /// `secs_to_nanos` as plain `#[logic(open)]` functions (the exact
        /// terms `as_secs`/`subsec_nanos`'s own postconditions are stated
        /// in), so the claim below is expressed directly in terms of
        /// `result@` (the `View` operator, Duration's total nanosecond
        /// count as Pearlite's arbitrary-precision `Int`) and those
        /// existing logic functions instead.
        ///
        /// This also means this harness proves less than the Kani one:
        /// Kani exercises the real `std::time::Duration` implementation,
        /// symbolically; this proves only that `creusot-std`'s OWN trusted
        /// axiom for `Duration::new`'s total nanosecond count decomposes
        /// the way `as_secs`/`subsec_nanos`'s OWN trusted axioms claim it
        /// should — internal consistency between two independently-trusted
        /// specifications, not agreement with the real implementation.
        #[requires(duration_new_headroom_holds(secs, nanos))]
        #[ensures(duration_new_normalizes_nanos_and_carries_into_secs_holds(secs, nanos, result))]
        fn verify_duration_new_normalizes_nanos_and_carries_into_secs(
            secs: u64,
            nanos: u32,
        ) -> Duration {
            Duration::new(secs, nanos)
        }
    }
}
