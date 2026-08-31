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
    creusot, SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<SeekFrom>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn seek_from_round_trips_each_variants_offset(
            start_offset: u64,
            end_offset: i64,
            current_offset: i64,
            seek_result: (u64, i64, i64),
        ) -> bool {
            pearlite! {
                seek_result.0 == start_offset
                    && seek_result.1 == end_offset
                    && seek_result.2 == current_offset
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// Each `SeekFrom` variant preserves the offset it was constructed
        /// with and remains its own variant.
        #[requires(true)]
        #[ensures(seek_from_round_trips_each_variants_offset(start_offset, end_offset, current_offset, result))]
        fn verify_seek_from_round_trips_each_variants_offset(
            start_offset: u64,
            end_offset: i64,
            current_offset: i64,
        ) -> (u64, i64, i64) {
            let start_value = match SeekFrom::Start(start_offset) {
                SeekFrom::Start(value) => value,
                SeekFrom::End(_) | SeekFrom::Current(_) => start_offset,
            };
            let end_value = match SeekFrom::End(end_offset) {
                SeekFrom::End(value) => value,
                SeekFrom::Start(_) | SeekFrom::Current(_) => end_offset,
            };
            let current_value = match SeekFrom::Current(current_offset) {
                SeekFrom::Current(value) => value,
                SeekFrom::Start(_) | SeekFrom::End(_) => current_offset,
            };

            (start_value, end_value, current_value)
        }
    }
}
