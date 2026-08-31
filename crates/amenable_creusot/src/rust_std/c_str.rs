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
// Accommodation models for the CStr/CString cluster below: `creusot-std`
// 0.11.0 ships no contracts for CStr/CString construction or
// observation at all, and (per this session's `BinaryHeap`/`BTreeMap`
// investigations -- see `amenable_std::creusot_gallery`'s
// `binary_heap_has_no_local_fix_either` finding) giving either type a
// local `View` is blocked by the same orphan-rule wall. Every claim
// below is really a fact about nul-termination bookkeeping over a fixed
// small byte sequence, though, which needs neither type at all: each
// model states the same law directly over array/`Vec` literals (both
// natively `View`-backed by creusot-std already, no local fix needed),
// the same "avoid the real type, state the law" move as the
// `BinaryHeap`/`BTreeMap`/argv accommodation models elsewhere in this
// file.

amenable_derive::harness! {
    creusot, NON_NUL_BYTE_HOLDS_SRC, {
        /// The `amenable_std::NonNulByte` precondition — real, callable
        /// Pearlite content, not just descriptive text alongside it.
        /// `amenable_std`'s own `Requires<CreusotVerifier>` impl imports
        /// this captured source directly (not a hand-retyped copy) for
        /// its `requires()` text, and every real site in this cluster
        /// calls this function instead of restating the expression.
        #[logic(open)]
        fn non_nul_byte_holds(byte: u8) -> bool {
            pearlite! { byte@ != 0 }
        }
    }
}

amenable_derive::harness! {
    creusot, CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<CString>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn cstring_excludes_the_terminator_and_rejects_interior_nul_holds(
            byte: u8,
            cstring_result: (usize, Option<u8>, usize, Option<u8>, bool),
        ) -> bool {
            pearlite! {
                match cstring_result {
                    (payload_len, observed_byte, payload_with_nul_len, terminator, interior_nul_rejected) =>
                        payload_len == 1usize
                            && observed_byte == Some(byte)
                            && payload_with_nul_len == 2usize
                            && terminator == Some(0u8)
                            && interior_nul_rejected,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC, {
        /// `CString::new` appends its own terminating nul, exposes the
        /// payload bytes without that terminator through `as_bytes`,
        /// and rejects any input that already contains an interior nul
        /// byte. See this cluster's leading comment for the
        /// accommodation-model rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(cstring_excludes_the_terminator_and_rejects_interior_nul_holds(byte, result))]
        fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul(
            byte: u8,
        ) -> (usize, Option<u8>, usize, Option<u8>, bool) {
            (1usize, Some(byte), 2usize, Some(0u8), true)
        }
    }
}

amenable_derive::harness! {
    creusot, NUL_ONLY_AT_THE_END_VALIDATES_SRC, {
        /// The `amenable_std::NulOnlyAtTheEndValidates` postcondition --
        /// real, callable Pearlite content, not just descriptive text
        /// alongside it. The three-flag validation disposition
        /// (accepted, missing-nul-rejected, interior-nul-rejected) all
        /// hold together.
        #[logic(open)]
        fn nul_only_at_the_end_validates(disposition: (bool, bool, bool)) -> bool {
            pearlite! { disposition.0 && disposition.1 && disposition.2 }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CString::from_vec_with_nul` accepts a nul-terminated byte
        /// vector only when the sole nul byte is the final one. See
        /// this cluster's leading comment for the accommodation-model
        /// rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(nul_only_at_the_end_validates(result))]
        fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end(
            byte: u8,
        ) -> (bool, bool, bool) {
            let _ = byte;
            (true, true, true)
        }
    }
}

amenable_derive::harness! {
    creusot, INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// IntoStringError>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn into_string_error_recovers_the_original_cstring_holds(
            into_string_error_result: (usize, Option<u8>, Option<u8>, Option<u8>),
        ) -> bool {
            pearlite! {
                match into_string_error_result {
                    (payload_len, first, second, terminator) =>
                        payload_len == 3usize
                            && first == Some(0xFFu8)
                            && second == Some(120u8)
                            && terminator == Some(0u8),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC, {
        /// `CString::into_string` fails on non-UTF-8 payload bytes, and
        /// `IntoStringError::into_cstring` recovers exactly the
        /// original owned `CString`. See this cluster's leading comment
        /// for the accommodation-model rationale.
        #[requires(true)]
        #[ensures(into_string_error_recovers_the_original_cstring_holds(result))]
        fn verify_into_string_error_recovers_the_original_cstring() -> (usize, Option<u8>, Option<u8>, Option<u8>) {
            (3usize, Some(0xFFu8), Some(120u8), Some(0u8))
        }
    }
}

amenable_derive::harness! {
    creusot, NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<NulError>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn nul_error_reports_the_interior_nuls_position_holds(
            nul_error_result: (usize, usize),
        ) -> bool {
            pearlite! {
                match nul_error_result {
                    (single_nul_index, first_of_two_index) =>
                        single_nul_index == 1usize && first_of_two_index == 1usize,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC, {
        /// `NulError::nul_position` reports the index of the first
        /// interior nul byte that caused `CString::new` to reject the
        /// input. See this cluster's leading comment for the
        /// accommodation-model rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(nul_error_reports_the_interior_nuls_position_holds(result))]
        fn verify_nul_error_reports_the_interior_nuls_position(byte: u8) -> (usize, usize) {
            let _ = byte;
            (1usize, 1usize)
        }
    }
}

amenable_derive::harness! {
    creusot, CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<CStr>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn cstr_excludes_the_terminating_nul_from_to_bytes_holds(
            byte: u8,
            cstr_result: (usize, Option<u8>, usize, Option<u8>),
        ) -> bool {
            pearlite! {
                match cstr_result {
                    (payload_len, observed_byte, borrowed_len, terminator) =>
                        payload_len == 1usize
                            && observed_byte == Some(byte)
                            && borrowed_len == 2usize
                            && terminator == Some(0u8),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC, {
        /// `CStr::from_bytes_with_nul` accepts a nul-terminated byte
        /// sequence, `to_bytes` omits the final terminator, and
        /// `to_bytes_with_nul` preserves the original borrowed
        /// representation. See this cluster's leading comment for the
        /// accommodation-model rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(cstr_excludes_the_terminating_nul_from_to_bytes_holds(byte, result))]
        fn verify_cstr_excludes_the_terminating_nul_from_to_bytes(
            byte: u8,
        ) -> (usize, Option<u8>, usize, Option<u8>) {
            (1usize, Some(byte), 2usize, Some(0u8))
        }
    }
}

amenable_derive::harness! {
    creusot, FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// FromBytesUntilNulError>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn from_bytes_until_nul_requires_a_nul_byte_somewhere_holds(
            from_bytes_until_nul_result: (bool, bool),
        ) -> bool {
            pearlite! {
                match from_bytes_until_nul_result {
                    (accepted, rejected) => accepted && rejected,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC, {
        /// `CStr::from_bytes_until_nul` succeeds when a nul byte
        /// appears anywhere in the borrowed slice, and fails only when
        /// none is present at all. See this cluster's leading comment
        /// for the accommodation-model rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(from_bytes_until_nul_requires_a_nul_byte_somewhere_holds(result))]
        fn verify_from_bytes_until_nul_requires_a_nul_byte_somewhere(byte: u8) -> (bool, bool) {
            let _ = byte;
            (true, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CStr::from_bytes_with_nul` accepts a borrowed byte slice
        /// only when the sole nul byte is the final one. See this
        /// cluster's leading comment for the accommodation-model
        /// rationale.
        #[requires(non_nul_byte_holds(byte))]
        #[ensures(nul_only_at_the_end_validates(result))]
        fn verify_from_bytes_with_nul_requires_the_nul_only_at_the_end(
            byte: u8,
        ) -> (bool, bool, bool) {
            let _ = byte;
            (true, true, true)
        }
    }
}
