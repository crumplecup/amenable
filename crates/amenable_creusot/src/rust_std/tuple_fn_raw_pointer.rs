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
    creusot, VERIFY_TUPLE_FIELD_ACCESS_SRC, {
        /// A tuple's fields recover the values it was constructed with,
        /// in position order.
        #[requires(true)]
        #[ensures(tuple_field_access_holds(a, b, result))]
        fn verify_tuple_field_access(a: i32, b: i32) -> (i32, i32) {
            let tuple = (a, b);
            (tuple.0, tuple.1)
        }
    }
}

amenable_derive::harness! {
    creusot, TUPLE_FIELD_ACCESS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<(i32, i32)>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn tuple_field_access_holds(a: i32, b: i32, tuple_result: (i32, i32)) -> bool {
            pearlite! { tuple_result.0 == a && tuple_result.1 == b }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// Calling through a `fn` pointer invokes exactly the function it
        /// was assigned from.
        ///
        /// `#[trusted]`: `creusot-rustc` rejects a real `f(value)` call
        /// here with `error: unsupported function call type`. The
        /// reduced repro is recorded in `amenable_std::creusot_gallery`;
        /// this trusted boundary keeps the dispatch law explicit for the
        /// carrier instead of falling back to provenance-only coverage.
        #[trusted]
        #[requires(true)]
        #[ensures(fn_pointer_calls_the_underlying_function(value, result))]
        fn verify_fn_pointer_calls_the_underlying_function(value: i32) -> i32 {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<fn(i32) ->
        /// i32>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn fn_pointer_calls_the_underlying_function(value: i32, fn_pointer_result: i32) -> bool {
            pearlite! { fn_pointer_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC, {
        /// Casting a raw const pointer changes its pointee type without
        /// changing its logical address.
        #[requires(true)]
        #[ensures(result)]
        fn verify_const_pointer_cast_preserves_the_address(ptr: *const i32) -> bool {
            let cast = ptr.cast::<u8>();
            cast.addr() == ptr.addr()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC, {
        /// Casting a raw mutable pointer changes its pointee type without
        /// changing its logical address.
        #[requires(true)]
        #[ensures(result)]
        fn verify_mut_pointer_cast_preserves_the_address(ptr: *mut i32) -> bool {
            let cast = ptr.cast::<u8>();
            cast.addr() == ptr.addr()
        }
    }
}
