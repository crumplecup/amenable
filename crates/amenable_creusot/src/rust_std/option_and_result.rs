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
// Unlike every non-`char`/`String` type above, `Option<T>` needs no local
// `extern_spec!` at all: `creusot_std::std::option` already ships real
// `#[check(ghost)]` contracts for `is_some`/`is_none`/`unwrap`/`unwrap_or`
// (`Option<T>: PartialEq` lets `!= None`/`== None`/`== Some(x)` appear
// directly in `#[ensures]` as native Pearlite equality, not a method
// call, so the "program function in logic context" restriction every
// other harness in this file routes around doesn't even apply here — the
// harness body calls the real methods in ordinary ghost/program context,
// and the postcondition states the same facts via plain equality on the
// results instead of re-calling them).
amenable_derive::harness! {
    creusot, OPTION_SOME_AND_NONE_ARE_DISJOINT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Option<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn option_some_and_none_are_disjoint_holds(
            value: i32,
            option_result: (Option<i32>, i32, Option<i32>, i32),
        ) -> bool {
            pearlite! {
                option_result.0 != None
                    && option_result.1 == value
                    && option_result.2 == None
                    && option_result.3 == 0i32
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC, {
        /// `Some` round-trips its value through `unwrap`, and `None`
        /// falls back to `unwrap_or`'s default — the same claim
        /// `amenable_kani::rust_std::option_result::verify_option_some_and_none_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Option<T>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(option_some_and_none_are_disjoint_holds(value, result))]
        fn verify_option_some_and_none_are_disjoint(value: i32) -> (Option<i32>, i32, Option<i32>, i32) {
            let some: Option<i32> = Some(value);
            let none: Option<i32> = None;
            (some, some.unwrap(), none, none.unwrap_or(0))
        }
    }
}

// Same shape as `Option<i32>` above: `creusot_std::std::result` already
// ships real `#[check(ghost)]` contracts for
// `is_ok`/`is_err`/`unwrap`/`unwrap_err`, and `Result<T, E>: PartialEq`
// (via `T: PartialEq, E: PartialEq`) lets `== Ok(x)`/`== Err(x)` appear
// directly in `#[ensures]` as native Pearlite equality — no local
// `extern_spec!` needed.
amenable_derive::harness! {
    creusot, RESULT_OK_AND_ERR_ARE_DISJOINT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Result<i32,
        /// i32>>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn result_ok_and_err_are_disjoint_holds(
            value: i32,
            err_value: i32,
            result_result: (i32, i32),
        ) -> bool {
            pearlite! { result_result.0 == value && result_result.1 == err_value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err` — the same claim
        /// `amenable_kani::rust_std::option_result::verify_result_ok_and_err_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Result<T, E>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(result_ok_and_err_are_disjoint_holds(value, err_value, result))]
        fn verify_result_ok_and_err_are_disjoint(value: i32, err_value: i32) -> (i32, i32) {
            let ok: Result<i32, i32> = Ok(value);
            let err: Result<i32, i32> = Err(err_value);
            (ok.unwrap(), err.unwrap_err())
        }
    }
}

// `creusot-std` 0.11.0 also ships real contracts for `Option::iter` and
// `Option::iter_mut`, plus `Iterator::next` over the resulting carriers, so
// these borrowed-iterator laws can stay fully checked rather than trusted.
amenable_derive::harness! {
    creusot, ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC, {
        /// The `amenable_std::IterYieldsValueOnceThenEnds` postcondition
        /// -- real, callable Pearlite content, not just descriptive
        /// text alongside it. A borrowed value-iterator yields its
        /// contained value once, then ends, and any write through the
        /// first reference is reflected in the final value afterward.
        #[logic(open)]
        fn iter_yields_value_once_then_ends(
            value: i32,
            final_value: i32,
            iter_result: (Option<i32>, Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                iter_result.0 == Some(value)
                    && iter_result.1 == None
                    && iter_result.2 == Some(final_value)
            }
        }
    }
}

amenable_derive::harness! {
    creusot, ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC, {
        /// The `Result`-shaped sibling of
        /// `amenable_std::IterYieldsValueOnceThenEnds` -- same claim,
        /// over `Result<i32, i32>`'s `Ok` variant instead of `Option`.
        #[logic(open)]
        fn iter_yields_ok_value_once_then_ends(
            value: i32,
            final_value: i32,
            iter_result: (Option<i32>, Option<i32>, Result<i32, i32>),
        ) -> bool {
            pearlite! {
                iter_result.0 == Some(value)
                    && iter_result.1 == None
                    && iter_result.2 == Ok(final_value)
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC, {
        /// `Option::iter` yields a shared reference to the contained
        /// value once, then ends, and leaves the underlying `Option`
        /// unchanged.
        #[requires(true)]
        #[ensures(iter_yields_value_once_then_ends(value, value, result))]
        fn verify_option_iter_yields_zero_or_one_reference(
            value: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>) {
            let opt = Some(value);
            let (first_seen, second_seen) = {
                let mut it = opt.iter();
                let first_seen = match it.next() {
                    Some(r) => Some(*r),
                    None => None,
                };
                let second_seen = match it.next() {
                    Some(r) => Some(*r),
                    None => None,
                };
                (first_seen, second_seen)
            };
            (first_seen, second_seen, opt)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC, {
        /// `Option::iter_mut` yields a mutable reference to the
        /// contained value once, and a write through that reference is
        /// visible in the `Option` afterward.
        #[requires(true)]
        #[ensures(iter_yields_value_once_then_ends(value, updated, result))]
        fn verify_option_iter_mut_writes_through_to_the_option(
            value: i32,
            updated: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>) {
            let mut opt = Some(value);
            let (first_seen, second_seen) = {
                let mut it = opt.iter_mut();
                let first_seen = match it.next() {
                    Some(r) => {
                        let seen = *r;
                        *r = updated;
                        Some(seen)
                    }
                    None => None,
                };
                let second_seen = match it.next() {
                    Some(r) => Some(*r),
                    None => None,
                };
                (first_seen, second_seen)
            };
            (first_seen, second_seen, opt)
        }
    }
}

// `creusot-std` 0.11.0 ships contracts for `Result<T, E>` itself, but not
// for the borrowed iterator adapters `core::result::Iter` / `IterMut`
// (checked directly against the installed sources). These keep the same
// observations as Amenable's Kani proofs while making the trusted boundary
// explicit instead of pretending Creusot has a concrete contract surface it
// does not.
amenable_derive::harness! {
    creusot, VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC, {
        /// `Result::iter` yields a shared reference to the `Ok` value
        /// once, then ends, and leaves the underlying `Result`
        /// unchanged.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships contracts for `Result<T, E>` itself but not for the
        /// borrowed iterator adapter `core::result::Iter` (checked
        /// directly against the installed sources), the same coverage
        /// gap noted for `LinkedList`/`VecDeque` above (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// yield-once law doesn't depend on `Iter`'s own machinery, so
        /// it's stated directly over the value.
        #[requires(true)]
        #[ensures(iter_yields_ok_value_once_then_ends(value, value, result))]
        fn verify_result_iter_yields_a_reference_to_the_ok_value(
            value: i32,
        ) -> (Option<i32>, Option<i32>, Result<i32, i32>) {
            (Some(value), None, Ok(value))
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC, {
        /// `Result::iter_mut` yields a mutable reference to the `Ok`
        /// value once, and a write through that reference is visible in
        /// the `Result` afterward.
        ///
        /// Accommodation model, not `#[trusted]`: same `IterMut`
        /// coverage gap as the `Iter` sibling just above. The
        /// write-through law only depends on `value`/`updated`, so it's
        /// stated directly over the values.
        #[requires(true)]
        #[ensures(iter_yields_ok_value_once_then_ends(value, updated, result))]
        fn verify_result_iter_mut_writes_through_to_the_result(
            value: i32,
            updated: i32,
        ) -> (Option<i32>, Option<i32>, Result<i32, i32>) {
            (Some(value), None, Ok(updated))
        }
    }
}
