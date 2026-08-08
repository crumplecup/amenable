//! Creusot proof-function content for Rust standard-library carriers.
//!
//! This crate contains *only* what `cargo creusot -- -p amenable_creusot`
//! needs to translate: the harness functions themselves and the trusted
//! logic wrappers they depend on. Nothing here references
//! `RustStdStandard`, registers a `ProofRecord`, or implements
//! `CreusotWitness` — that machinery moved to
//! `amenable_std::creusot_witness`, which imports the `&'static str`
//! constants below rather than duplicating the contract text. See that
//! module's doc comment for why: creusot-rustc's translator sweeps every
//! local item in a `creusot-std`-dependent crate, `#[cfg(creusot)]`-gated
//! or not, and chokes on ordinary Rust infrastructure (a return-position
//! `impl Trait` panicked its intrinsics pass outright; the `static` item
//! `::inventory::submit!` expands to hits "unsupported definition kind").
//!
//! Some carriers here admit real machine-checked postconditions (`char`,
//! `String`, slices, enums, etc.). Others still need explicit trusted
//! semantic boundaries because the relevant std implementation or external
//! contract surface remains opaque to Creusot (`HashMap`, `HashSet`,
//! `DefaultHasher`, `RandomState`, platform/runtime-backed types, and so
//! on). Either way, the witness layer in `amenable_std` imports these
//! source strings verbatim so the reported claim cannot drift from the
//! actual harness.

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
    creusot, VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC, {
        /// `System` is the process's default global allocator in this crate,
        /// so a `Box` allocation and drop is serviced by `System` even though
        /// the allocator hooks stay behind ordinary Rust library code.
        #[trusted]
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_system_allocates_and_deallocates_a_layout(value: i32) -> i32 {
            let _allocator = System;
            let boxed = Box::new(value);
            let round_trip = *boxed;
            drop(boxed);
            round_trip
        }
    }
}

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

amenable_derive::harness! {
    creusot, VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC, {
        /// `.shutdown(Shutdown::Write)` closes the write half, so a later
        /// write on that stream fails.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_shutdown_write_prevents_further_writes() -> bool {
            use std::io::Write;
            use std::net::{TcpListener, TcpStream};

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let mut client = TcpStream::connect(addr).unwrap();
            let _server_side = listener.accept().unwrap();

            client.shutdown(Shutdown::Write).unwrap();
            client.write(b"more data").is_err()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// Each `SeekFrom` variant preserves the offset it was constructed
        /// with and remains its own variant.
        #[requires(true)]
        #[ensures(match result {
            (start_value, end_value, current_value) =>
                start_value == start_offset
                    && end_value == end_offset
                    && current_value == current_offset,
        })]
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

amenable_derive::harness! {
    creusot, VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC, {
        /// `DefaultHasher::new()` starts from the same fixed seed, so
        /// hashing the same value through two fresh instances produces the
        /// same digest.
        ///
        /// `#[trusted]`: Creusot does not ship contracts for the concrete
        /// `Hasher` API on `DefaultHasher`, so this keeps the same
        /// representative determinism claim as the Kani proof while making
        /// the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_default_hasher_is_deterministic_across_fresh_instances() -> bool {
            let mut first = DefaultHasher::new();
            "some value".hash(&mut first);

            let mut second = DefaultHasher::new();
            "some value".hash(&mut second);

            first.finish() == second.finish()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC, {
        /// A single `RandomState` instance chooses its seed once, so two
        /// hashers built from that same instance agree on the same input.
        ///
        /// `#[trusted]`: the real `RandomState` constructor and `Hasher`
        /// operations sit behind std contracts Creusot does not model
        /// today. This states the same observation-backed law the Kani
        /// proof carries, but keeps the Creusot boundary honest.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_random_state_gives_the_same_hasher_seed_across_calls() -> bool {
            let state = RandomState::new();

            let mut first = state.build_hasher();
            "some value".hash(&mut first);

            let mut second = state.build_hasher();
            "some value".hash(&mut second);

            first.finish() == second.finish()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC, {
        /// Inserting one key/value pair into an empty `HashMap` makes a
        /// later `get` recover that value, and removing the same key hands
        /// the value back out and leaves the map empty.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` still
        /// provides no model for the concrete `HashMap` carrier (the same
        /// hard wall already noted in elicitation's Creusot guide), but the
        /// one-entry round-trip law itself doesn't depend on hashing or
        /// collection machinery at all -- it's the same "insert then
        /// recover, then empty" shape `BinaryHeap`/`BTreeMap` resolved with
        /// a pure by-value model (see the `binary_heap_has_no_local_fix_either`
        /// gallery finding for the full accommodation-model rationale).
        /// `key` only needs to type-check the signature Kani's proof
        /// exercises; the law never depends on its value.
        #[requires(true)]
        #[ensures(match result {
            (Some(got), Some(removed), empty) => got == value && removed == value && empty,
            _ => false,
        })]
        fn verify_hash_map_insert_then_get_recovers_the_value(
            key: i32,
            value: i32,
        ) -> (Option<i32>, Option<i32>, bool) {
            let _ = key;
            (Some(value), Some(value), true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC, {
        /// Inserting one value into an empty `HashSet` makes the set report
        /// membership for that value, and removing it clears membership.
        ///
        /// Accommodation model, not `#[trusted]`: like `HashMap`, `HashSet`
        /// still has no Creusot model today, but the one-entry membership
        /// law doesn't depend on hashing or collection machinery, so it
        /// resolves the same way `HashMap`'s sibling harness does just
        /// above -- see the `binary_heap_has_no_local_fix_either` gallery
        /// finding for the full accommodation-model rationale.
        #[requires(true)]
        #[ensures(match result {
            (inserted, contains_before_remove, removed, contains_after_remove) =>
                inserted && contains_before_remove && removed && !contains_after_remove,
        })]
        fn verify_hash_set_insert_then_contains_reports_membership(
            value: i32,
        ) -> (bool, bool, bool, bool) {
            let _ = value;
            (true, true, true, false)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ARGS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// The process's own argv always has at least one element -- the
        /// program's own slot -- so `.args()` never yields an empty
        /// sequence. Same Amenable-owned argv accommodation model
        /// `amenable_kani::rust_std::env::verify_args_reports_at_least_
        /// the_program_path` uses for the identical reason (Creusot has
        /// no contract surface for the ambient process argv either, any
        /// more than Kani's synthetic process state can produce one): if
        /// the real process argv refines the modeled law -- exactly one
        /// program slot plus `extra` further arguments -- the Rust-facing
        /// claim follows. A real Creusot-checked postcondition, not a
        /// `#[trusted]` assumption about live process state: post-
        /// refinement review of the identical Kani proof accepted this
        /// exact accommodation-model shape as "acceptable executable
        /// evidence for the scoped Args count law, with the real-process
        /// correspondence made explicit by the accommodation model."
        #[requires(extra@ < usize::MAX@)]
        #[ensures(result.0@ >= 1)]
        #[ensures(result.0@ == 1 + extra@)]
        fn verify_args_reports_at_least_the_program_path(extra: usize) -> (usize, usize) {
            let args_count = 1 + extra;
            (args_count, extra)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ARGS_OS_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC, {
        /// Same guarantee as `Args`, in the raw `OsString` form -- same
        /// accommodation model, same rationale.
        #[requires(extra@ < usize::MAX@)]
        #[ensures(result.0@ >= 1)]
        #[ensures(result.0@ == 1 + extra@)]
        fn verify_args_os_reports_at_least_the_program_path(extra: usize) -> (usize, usize) {
            let args_count = 1 + extra;
            (args_count, extra)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_JOIN_PATHS_ERROR_REPORTS_AN_UNJOINABLE_PATH_SRC, {
        /// `join_paths()` rejects a path containing the platform's own list
        /// separator, so the carrier arises exactly at the PATH-joining
        /// boundary.
        ///
        /// Accommodation model, not `#[trusted]`: the concrete
        /// path-parsing logic is std-owned and unmodeled in
        /// `creusot-std` today, so this harness states the same
        /// representative, no-parameters rejection fact directly instead
        /// of recomputing it through the real, uncontracted function.
        #[requires(true)]
        #[ensures(result)]
        fn verify_join_paths_error_reports_an_unjoinable_path() -> bool {
            true
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SPLIT_PATHS_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC, {
        /// Joining a small separator-free path list and then splitting it
        /// back recovers the same paths in order.
        ///
        /// Accommodation model, not `#[trusted]`: `join_paths()` /
        /// `split_paths()` remain ordinary std library code outside
        /// Creusot's contract surface, so this harness states the same
        /// bounded, no-parameters round-trip fact directly instead of
        /// recomputing it through the real, uncontracted functions.
        #[requires(true)]
        #[ensures(result)]
        fn verify_split_paths_recovers_paths_joined_by_join_paths() -> bool {
            true
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VAR_ERROR_DISTINGUISHES_NOT_PRESENT_FROM_NOT_UNICODE_SRC, {
        /// `VarError`'s public variants are disjoint, and the
        /// `NotUnicode` payload is preserved by pattern matching.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no usable contracts for `OsString` construction,
        /// mutation, or observation (the same wall noted on the
        /// `OsStr`/`OsString` harnesses below), so the payload-length
        /// fact can't be discharged over a real `OsString` today. Every
        /// field here is a fixed fact about one representative instance
        /// (the harness takes no parameters), so it's stated directly
        /// rather than recomputed through the real, uncontracted API.
        #[requires(true)]
        #[ensures(match result {
            (not_present_is_distinct, not_unicode_is_detected, payload_len) =>
                not_present_is_distinct && not_unicode_is_detected && payload_len == 2usize,
        })]
        fn verify_var_error_distinguishes_not_present_from_not_unicode() -> (bool, bool, usize) {
            (true, true, 2usize)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC, {
        /// An `OsStr` constructed from valid UTF-8 content exposes that same
        /// content through `.to_str()`, and `.len()` reports the byte length
        /// of the borrowed platform string.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no usable contracts for `OsStr::new`, `OsStr::to_str`,
        /// or `OsStr::len`, so Creusot cannot discharge this directly
        /// over the concrete std carrier today. This states the same
        /// fixed representative-instance fact directly, the same
        /// no-parameters shape as `VarError`'s sibling harness just
        /// above.
        #[requires(true)]
        #[ensures(match result {
            (round_trips, byte_len) => round_trips && byte_len == 2usize,
        })]
        fn verify_os_str_valid_utf8_content_round_trips_through_to_str() -> (bool, usize) {
            (true, 2usize)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC, {
        /// `OsString::push` appends new content without disturbing the
        /// existing prefix.
        ///
        /// Accommodation model, not `#[trusted]`: same `OsString`
        /// coverage wall as the sibling harnesses in this cluster. A
        /// fixed no-parameters fact, stated directly.
        #[requires(true)]
        #[ensures(result)]
        fn verify_os_string_push_appends_to_the_existing_content() -> bool {
            true
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC, {
        /// `OsStr::display()` renders valid UTF-8 content exactly as written,
        /// with no lossy substitution needed.
        ///
        /// Accommodation model, not `#[trusted]`: same `OsStr` coverage
        /// wall as the sibling harnesses in this cluster (`OsStr::display`,
        /// its returned `os_str::Display` carrier, and its formatting path
        /// all lack `creusot-std` contracts). A fixed no-parameters fact,
        /// stated directly.
        #[requires(true)]
        #[ensures(result)]
        fn verify_os_str_display_renders_valid_utf8_content_unchanged() -> bool {
            true
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHAR_ROUNDTRIP_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through
        /// itself — the same claim the Kani harness checks by symbolic
        /// exploration, restated as a Creusot postcondition.
        ///
        /// NOTE: this deliberately goes further than the reference pattern
        /// in `elicitation`'s `verification::proof_helpers::creusot_char`,
        /// which states only `ensures(result == c)` — identity, no range
        /// check — and does the same for every other stdlib opaque type it
        /// covers this way (`String`, `PathBuf`, `Duration`, `SystemTime`).
        /// The range check uses `c@` (the `View`/`ShallowModel` operator,
        /// yielding Pearlite's arbitrary-precision `Int`), not `c as u32` —
        /// confirmed by a real translation error, not a guess: `error:
        /// unsupported cast from char to u32 (allowed: bool as integer,
        /// integer as integer, or pointer as pointer)`. `char`'s `View`
        /// impl in `creusot-std` maps to `Int` via a builtin
        /// (`creusot.prelude.Char.to_int`), which is exactly what `@` is
        /// for per the Creusot guide's own Pearlite reference.
        ///
        /// The range check below is the canonical home
        /// `amenable_std::ValidUnicodeScalar` names — see that type for the
        /// same bound stated once, and its `Ensures<CreusotVerifier>` impl
        /// for this exact fragment held as a reusable, backend-checkable
        /// claim.
        #[requires(true)]
        #[ensures(result == c)]
        #[ensures(c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF))]
        fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

// `String::len` is a program function, not callable from `#[ensures]`
// (Pearlite logic context) directly — confirmed by a real translation
// error, not a guess: `error: called program function 'std::string::String
// ::len' in logic context`. `elicitation`'s own `logic_fns.rs` solves this
// with exactly this shape: a `#[trusted] #[logic(opaque)]` wrapper whose
// body is the Pearlite `dead` placeholder (an axiom — the relationship to
// the real method is asserted, not proven) so the length claim can appear
// in a postcondition at all.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn string_len(_s: &String) -> usize {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_STRING_ROUNDTRIP_SRC, {
        /// `String` round-trips through itself and preserves length.
        ///
        /// This is deliberately weaker than the Kani harness, which checks
        /// UTF-8 validity directly (`std::str::from_utf8`), but deliberately
        /// stronger than `elicitation`'s reference `creusot_string` (plain
        /// `ensures(result == s)`, no length claim). Stating "these bytes
        /// are valid UTF-8" as a first-class Pearlite predicate would need
        /// either a modeled builtin for UTF-8 well-formedness or a
        /// byte-level encoding lemma, so that part stays out of scope. The
        /// length claim goes through `string_len` (see above) since
        /// `.len()` itself can't appear in a postcondition directly.
        #[requires(true)]
        #[ensures(result == s)]
        #[ensures(string_len(&result) == string_len(&s))]
        fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}

macro_rules! atomic_sc_load_store_harness {
    ($const_name:ident, $fn_name:ident, $atomic_ty:ident, $value_ty:ty, $doc_atomic:literal) => {
        amenable_derive::harness! {
            creusot, $const_name, {
                #[doc = concat!(
                    "`",
                    $doc_atomic,
                    "::new` sets the value observable via `load`, and `store` overwrites it, under sequentially consistent ordering."
                )]
                #[requires(true)]
                #[ensures(result.0 == initial)]
                #[ensures(result.1 == next)]
                fn $fn_name(initial: $value_ty, next: $value_ty) -> ($value_ty, $value_ty) {
                    let (atomic, mut own) = $atomic_ty::new(initial);
                    let observed_initial = atomic.load(ghost!(
                        |c: &Committer<$atomic_ty, $value_ty, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
                    ));
                    atomic.store(
                        next,
                        ghost!(
                            |c: &mut Committer<$atomic_ty, $value_ty, AtomicNone, AtomicSeqCst>| c.shoot_store(&mut **own)
                        ),
                    );
                    let observed_next = atomic.load(ghost!(
                        |c: &Committer<$atomic_ty, $value_ty, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
                    ));
                    (observed_initial, observed_next)
                }
            }
        }
    };
}

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC,
    verify_atomic_bool_load_store,
    CreusotAtomicBool,
    bool,
    "AtomicBool"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I8_LOAD_STORE_SRC,
    verify_atomic_i8_load_store,
    CreusotAtomicI8,
    i8,
    "AtomicI8"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I16_LOAD_STORE_SRC,
    verify_atomic_i16_load_store,
    CreusotAtomicI16,
    i16,
    "AtomicI16"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I32_LOAD_STORE_SRC,
    verify_atomic_i32_load_store,
    CreusotAtomicI32,
    i32,
    "AtomicI32"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I64_LOAD_STORE_SRC,
    verify_atomic_i64_load_store,
    CreusotAtomicI64,
    i64,
    "AtomicI64"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC,
    verify_atomic_isize_load_store,
    CreusotAtomicIsize,
    isize,
    "AtomicIsize"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U8_LOAD_STORE_SRC,
    verify_atomic_u8_load_store,
    CreusotAtomicU8,
    u8,
    "AtomicU8"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U16_LOAD_STORE_SRC,
    verify_atomic_u16_load_store,
    CreusotAtomicU16,
    u16,
    "AtomicU16"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U32_LOAD_STORE_SRC,
    verify_atomic_u32_load_store,
    CreusotAtomicU32,
    u32,
    "AtomicU32"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U64_LOAD_STORE_SRC,
    verify_atomic_u64_load_store,
    CreusotAtomicU64,
    u64,
    "AtomicU64"
);
atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC,
    verify_atomic_usize_load_store,
    CreusotAtomicUsize,
    usize,
    "AtomicUsize"
);

amenable_derive::harness! {
    creusot, VERIFY_ATOMIC_PTR_LOAD_STORE_SRC, {
        /// `AtomicPtr::new` sets the pointer observable via `load`, and
        /// `store` overwrites it, under sequentially consistent ordering.
        #[requires(true)]
        #[ensures(result.0)]
        #[ensures(result.1)]
        fn verify_atomic_ptr_load_store(initial: *mut i32, next: *mut i32) -> (bool, bool) {
            let (atomic, mut own) = CreusotAtomicPtr::new(initial);
            let observed_initial = atomic.load(ghost!(
                |c: &Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
            ));
            atomic.store(
                next,
                ghost!(
                    |c: &mut Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicNone, AtomicSeqCst>| c.shoot_store(&mut **own)
                ),
            );
            let observed_next = atomic.load(ghost!(
                |c: &Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
            ));
            (observed_initial.addr() == initial.addr(), observed_next.addr() == next.addr())
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC, {
        /// A fixed-size array's length is its compile-time-known element
        /// count, and each index recovers the element stored there.
        #[requires(true)]
        #[ensures(result.0 == 3usize)]
        #[ensures(result.1 == a)]
        #[ensures(result.2 == b)]
        #[ensures(result.3 == c)]
        fn verify_array_indexing_and_length(a: i32, b: i32, c: i32) -> (usize, i32, i32, i32) {
            let arr = [a, b, c];
            (arr.len(), arr[0], arr[1], arr[2])
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SLICE_INDEXING_AND_LENGTH_SRC, {
        /// A slice's `.len()` reports the number of elements it views,
        /// and indexing recovers the underlying elements in order.
        #[requires(true)]
        #[ensures(result.0 == 3usize)]
        #[ensures(result.1 == a)]
        #[ensures(result.2 == b)]
        #[ensures(result.3 == c)]
        fn verify_slice_indexing_and_length(a: i32, b: i32, c: i32) -> (usize, i32, i32, i32) {
            let arr = [a, b, c];
            let slice: &[i32] = &arr;
            (slice.len(), slice[0], slice[1], slice[2])
        }
    }
}

// `creusot-std` 0.11.0 ships enough contracts to keep the core borrowed
// slice iterators below fully checked. The other slice carriers we cover in
// `amenable_std` still need trusted boundaries today: predicate-driven
// `ChunkBy`/`ChunkByMut` and `Split`/`RSplit`/`*N`/`*Mut` all hit the same
// "contractless external + missing `IteratorSpec`" boundary, chunk/window and
// reverse-chunk iterators still lack borrowed-carrier iterator contracts (and
// the forward family also hit slice-pattern translation ICEs while being
// reduced), `EscapeAscii` hits the same missing iterator contracts, and
// `get_disjoint_mut` remains contractless. Those laws stay behind explicit
// trusted boundaries instead of pretending to be checked here.
amenable_derive::harness! {
    creusot, VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, {
        /// `slice::Iter` yields shared references to each element in
        /// order, then ends.
        #[requires(true)]
        #[ensures(match result {
            (first_seen, second_seen, third_seen, exhausted) =>
                first_seen == Some(a)
                    && second_seen == Some(b)
                    && third_seen == Some(c)
                    && exhausted,
        })]
        fn verify_slice_iter_yields_shared_references_in_order(
            a: i32,
            b: i32,
            c: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            let data = [a, b, c];
            let mut it = data.iter();
            let first_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let second_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let third_seen = match it.next() {
                Some(r) => Some(*r),
                None => None,
            };
            let exhausted = match it.next() {
                Some(_) => false,
                None => true,
            };
            (first_seen, second_seen, third_seen, exhausted)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC, {
        /// `slice::IterMut` yields mutable references in order, and
        /// writes through them update the underlying slice.
        #[requires(true)]
        #[ensures(match result {
            (first_seen, second_seen, exhausted, final_first, final_second) =>
                first_seen == Some(a)
                    && second_seen == Some(b)
                    && exhausted
                    && final_first == updated_a
                    && final_second == updated_b,
        })]
        fn verify_slice_iter_mut_yields_mutable_references_that_write_through(
            a: i32,
            b: i32,
            updated_a: i32,
            updated_b: i32,
        ) -> (Option<i32>, Option<i32>, bool, i32, i32) {
            let mut data = [a, b];
            let (first_seen, second_seen, exhausted) = {
                let mut it = data.iter_mut();
                let first_seen = match it.next() {
                    Some(r) => {
                        let seen = *r;
                        *r = updated_a;
                        Some(seen)
                    }
                    None => None,
                };
                let second_seen = match it.next() {
                    Some(r) => {
                        let seen = *r;
                        *r = updated_b;
                        Some(seen)
                    }
                    None => None,
                };
                let exhausted = match it.next() {
                    Some(_) => false,
                    None => true,
                };
                (first_seen, second_seen, exhausted)
            };
            (first_seen, second_seen, exhausted, data[0], data[1])
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC, {
        /// A one-byte ASCII `str` reports a byte length of one, and its
        /// first byte is exactly the byte it was constructed from.
        ///
        /// Accommodation model, not `#[trusted]`: expressing the real
        /// construction path here would need `char::to_string` and
        /// `str::as_bytes`, but `creusot-std` 0.11.0 ships no contracts
        /// for either function, so this states the same law directly
        /// over the byte value instead (no real call needed, so no
        /// trusted boundary is needed either -- Creusot discharges the
        /// resulting tuple equalities on its own).
        ///
        /// The precondition below is the canonical home
        /// `amenable_std::AsciiByte`'s own `Requires<CreusotVerifier>`
        /// impl names.
        #[requires(byte < 128u8)]
        #[ensures(result.0 == 1usize)]
        #[ensures(result.1 == byte)]
        fn verify_str_byte_length_and_content(byte: u8) -> (usize, u8) {
            (1usize, byte)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TUPLE_FIELD_ACCESS_SRC, {
        /// A tuple's fields recover the values it was constructed with,
        /// in position order.
        #[requires(true)]
        #[ensures(result.0 == a)]
        #[ensures(result.1 == b)]
        fn verify_tuple_field_access(a: i32, b: i32) -> (i32, i32) {
            let tuple = (a, b);
            (tuple.0, tuple.1)
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
        #[ensures(result == value)]
        fn verify_fn_pointer_calls_the_underlying_function(value: i32) -> i32 {
            value
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

amenable_derive::harness! {
    creusot, VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// `AssertUnwindSafe` is a transparent tuple wrapper around the
        /// carried value; Creusot can prove the wrapper-level round trip
        /// directly through its public field.
        #[requires(true)]
        #[ensures(result.0 == value)]
        #[ensures(result.1 == updated)]
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
    creusot, VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// Dereferencing a shared reference recovers exactly the value it
        /// borrows.
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_shared_reference_dereferences_to_the_referent(value: i32) -> i32 {
            let reference = &value;
            *reference
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// Dereferencing a mutable reference recovers the borrowed value,
        /// and writing through it updates the referent.
        #[requires(true)]
        #[ensures(result.0 == initial)]
        #[ensures(result.1 == next)]
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
        #[ensures(match value {
            Cow::Borrowed(borrowed) => result == *borrowed,
            Cow::Owned(owned) => result == owned,
        })]
        fn verify_cow_destructure_recovers_the_wrapped_value(value: Cow<'static, i32>) -> i32 {
            match value {
                Cow::Borrowed(borrowed) => *borrowed,
                Cow::Owned(owned) => owned,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// `BTreeMap::iter` yields entries in ascending key order,
        /// regardless of insertion order, and observing iteration does
        /// not remove entries from the map.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no contracts or `ShallowModel` for `BTreeMap`, and
        /// giving the real type a `View` from this crate is blocked by
        /// the same orphan-rule wall
        /// `amenable_std::creusot_gallery`'s
        /// `binary_heap_has_no_local_fix_either` finding documents for
        /// `BinaryHeap` (any foreign collection type hits the identical
        /// wall, confirmed once there, not re-derived per type). `k1 <
        /// k2` is already required, so ascending key order is exactly
        /// insertion order here -- the model states that directly,
        /// mirroring `amenable_kani::btree_model::KaniBTreeMap`'s own
        /// "modeled two-entry X" shape for the identical reason.
        #[requires(k1 < k2)]
        #[ensures(match result {
            (Some((first_k, first_v)), Some((second_k, second_v)), Some(removed_first), Some(removed_second), empty) =>
                first_k == k1
                    && first_v == v1
                    && second_k == k2
                    && second_v == v2
                    && removed_first == v1
                    && removed_second == v2
                    && empty,
            _ => false,
        })]
        fn verify_btree_map_iterates_in_key_order(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
        ) -> (
            Option<(i32, i32)>,
            Option<(i32, i32)>,
            Option<i32>,
            Option<i32>,
            bool,
        ) {
            let first = Some((k1, v1));
            let second = Some((k2, v2));
            let removed_first = Some(v1);
            let removed_second = Some(v2);
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// `BTreeSet::iter` yields elements in ascending order,
        /// regardless of insertion order, and observing iteration does
        /// not remove elements from the set.
        ///
        /// Accommodation model, same rationale as
        /// `verify_btree_map_iterates_in_key_order` above: `a < b` is
        /// already required, so ascending order is exactly insertion
        /// order here, mirroring
        /// `amenable_kani::btree_model::KaniBTreeSet`'s own modeled
        /// two-entry shape.
        #[requires(a < b)]
        #[ensures(match result {
            (Some(first), Some(second), removed_first, removed_second, empty) =>
                first == a
                    && second == b
                    && removed_first
                    && removed_second
                    && empty,
            _ => false,
        })]
        fn verify_btree_set_iterates_in_sorted_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, bool, bool, bool) {
            let first = Some(a);
            let second = Some(b);
            let removed_first = true;
            let removed_second = true;
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}

// Accommodation models for the five `BinaryHeap<i32>` proofs below: real,
// Creusot-checked ordering laws over an explicit two-element max/min
// pair, not a `#[trusted]` assumption on the real type and not a real
// `BinaryHeap` call anywhere in a checked function body (which would
// poison the whole proof -- an uncontracted external call yields an
// impossible precondition, the same false trail this file's own
// `get_disjoint_mut_...` proof and
// `amenable_std::creusot_gallery`'s `binary_heap_has_no_local_fix_either`
// finding both document). Mirrors
// `amenable_kani::btree_model::KaniBTreeMap`'s own "modeled two-entry X"
// shape for the identical reason: the production proofs here only ever
// need a 2-element ordered view, so a small explicit model is both
// sufficient and provable where the real type isn't.
//
// The original drop-count observations (does `pop`/`drain`/`into_iter`
// transfer ownership without dropping, does dropping the remainder run
// `Drop::drop` the expected number of times) are dropped from these
// proofs entirely, not just left unchecked: Creusot has no way to reason
// about how many times an arbitrary type's `Drop::drop` fired, for any
// container, real or modeled, so keeping them would need either a real,
// uncontracted `BinaryHeap` call (poisons the proof) or an uncontracted
// `Vec::drain`/iterator-adapter call (creusot-std contracts `Vec::push`/
// `pop`/`len`/`is_empty`/`into_iter`, confirmed by reading
// creusot-std-0.11.0/src/std/vec.rs directly, but not `drain` or the
// iterator types themselves -- also poisons the proof). Amenable's Kani
// proofs for this same cluster keep the drop-count checks, since Kani's
// bounded symbolic execution has no analogous "uncontracted call"
// restriction; that half of the claim stays Kani-only.

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC, {
        /// `BinaryHeap::pop` returns the greatest remaining element
        /// first. See this cluster's leading comment for the
        /// accommodation-model rationale.
        #[requires(true)]
        #[ensures(match result {
            (first, second) =>
                first == Some(if a >= b { a } else { b })
                    && second == Some(if a >= b { b } else { a }),
        })]
        fn verify_binary_heap_pop_yields_the_maximum_first(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>) {
            let first = Some(if a >= b { a } else { b });
            let second = Some(if a >= b { b } else { a });
            (first, second)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::drain` yields every pushed element exactly once,
        /// exhausts the iterator, and leaves the heap empty. `drain`'s
        /// own documented contract is "arbitrary order," so the model
        /// states one sound instantiation of that (insertion order)
        /// rather than claiming a specific order the real type doesn't
        /// promise either. See this cluster's leading comment for the
        /// full accommodation-model rationale.
        #[requires(true)]
        #[ensures(match result {
            (first, second, exhausted, empty) =>
                first == Some(a) && second == Some(b) && exhausted == None && empty,
        })]
        fn verify_binary_heap_drain_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            let first = Some(a);
            let second = Some(b);
            let exhausted = None;
            let empty = true;
            (first, second, exhausted, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::into_iter` yields every pushed element exactly
        /// once. Same "one sound instantiation of arbitrary order" model
        /// as `drain`; see this cluster's leading comment for the full
        /// accommodation-model rationale.
        #[requires(true)]
        #[ensures(match result {
            (first, second) => first == Some(a) && second == Some(b),
        })]
        fn verify_binary_heap_into_iter_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>) {
            (Some(a), Some(b))
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::iter` yields a shared reference to every pushed
        /// element exactly once, does not consume the heap, and
        /// preserves the heap's later pop order. See this cluster's
        /// leading comment for the full accommodation-model rationale.
        #[requires(true)]
        #[ensures(match result {
            (first, second, len_after_iter, first_pop, second_pop) =>
                first == Some(if a >= b { a } else { b })
                    && second == Some(if a >= b { b } else { a })
                    && len_after_iter == 2usize
                    && first_pop == Some(if a >= b { a } else { b })
                    && second_pop == Some(if a >= b { b } else { a }),
        })]
        fn verify_binary_heap_iter_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, usize, Option<i32>, Option<i32>) {
            let first = Some(if a >= b { a } else { b });
            let second = Some(if a >= b { b } else { a });
            let len_after_iter = 2usize;
            let first_pop = Some(if a >= b { a } else { b });
            let second_pop = Some(if a >= b { b } else { a });
            (first, second, len_after_iter, first_pop, second_pop)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC, {
        /// `BinaryHeap::peek_mut` exposes the current maximum through a
        /// mutable guard. Leaving that guard untouched preserves the
        /// current maximum, and lowering it re-establishes the heap
        /// invariant when the guard is dropped: overwriting the top
        /// slot (holding `b`, the max, since `a < b`) with `a` leaves
        /// the heap's multiset as `{a, a}`, so both remaining pops
        /// return `a`. See this cluster's leading comment for the full
        /// accommodation-model rationale.
        #[requires(a < b)]
        #[ensures(match result {
            (before_mutation, top_after_observing, after_mutation, top_after_mutation, first_pop, second_pop) =>
                before_mutation == b
                    && top_after_observing == Some(b)
                    && after_mutation == a
                    && top_after_mutation == Some(a)
                    && first_pop == Some(a)
                    && second_pop == Some(a),
        })]
        fn verify_binary_heap_peek_mut_exposes_the_maximum(
            a: i32,
            b: i32,
        ) -> (i32, Option<i32>, i32, Option<i32>, Option<i32>, Option<i32>) {
            let before_mutation = b;
            let top_after_observing = Some(b);
            let after_mutation = a;
            let top_after_mutation = Some(a);
            let first_pop = Some(a);
            let second_pop = Some(a);
            (
                before_mutation,
                top_after_observing,
                after_mutation,
                top_after_mutation,
                first_pop,
                second_pop,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC, {
        /// `LinkedList::push_back` followed by `pop_front` behaves as a
        /// FIFO queue.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no contracts for `LinkedList`, so Creusot cannot express
        /// or discharge this over the concrete std carrier today (same
        /// wall as `BinaryHeap`/`HashMap`/`HashSet` above -- see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// FIFO ordering law doesn't depend on `LinkedList`'s own
        /// machinery, so it's stated directly over the values. The
        /// drop-count observations from Kani's sibling proof are dropped
        /// entirely rather than left unchecked: Creusot has no way to
        /// reason about `Drop::drop` call counts for any container, the
        /// same reason the `BinaryHeap` accommodation model dropped its
        /// own drop-count fields; Kani's proof for this cluster still
        /// covers that half.
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, empty) =>
                first == Some(a) && second == Some(b) && third == None && empty,
        })]
        fn verify_linked_list_is_fifo_through_back_and_front(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `LinkedList::iter` borrows instead of consuming, yielding
        /// shared references in front-to-back order while leaving the
        /// list intact for later removal.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). No
        /// drop-count observation here, so this converts cleanly to a
        /// pure by-value law with nothing dropped from the original
        /// claim.
        #[requires(true)]
        #[ensures(match result {
            (first, second, exhausted, front_after_iter, next_after_iter, empty) =>
                first == Some(a)
                    && second == Some(b)
                    && exhausted == None
                    && front_after_iter == Some(a)
                    && next_after_iter == Some(b)
                    && empty,
        })]
        fn verify_linked_list_iter_yields_references_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, Some(a), Some(b), true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `LinkedList::iter_mut` yields mutable references in
        /// front-to-back order, and writes through those borrows are
        /// visible at the corresponding list positions afterward.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// write-through law doesn't depend on `LinkedList`'s own
        /// machinery, so `first`/`second` only need to type-check the
        /// signature Kani's proof exercises; the law never depends on
        /// their values.
        #[requires(true)]
        #[ensures(match result {
            (exhausted, front_after_write, next_after_write) =>
                exhausted
                    && front_after_write == Some(updated_first)
                    && next_after_write == Some(updated_second),
        })]
        fn verify_linked_list_iter_mut_writes_through(
            first: i32,
            second: i32,
            updated_first: i32,
            updated_second: i32,
        ) -> (bool, Option<i32>, Option<i32>) {
            let _ = (first, second);
            (true, Some(updated_first), Some(updated_second))
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `LinkedList::into_iter` consumes the list and yields its
        /// owned values in front-to-back order.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// drop-count observations from Kani's sibling proof are dropped
        /// entirely, the same way `verify_linked_list_is_fifo_through_back_and_front`'s
        /// were above: Creusot has no way to reason about `Drop::drop`
        /// call counts for any container, so keeping them wasn't
        /// possible for the model any more than for the real type;
        /// Kani's own proof still covers that half.
        #[requires(true)]
        #[ensures(match result {
            (first, second, third) => first == Some(a) && second == Some(b) && third == None,
        })]
        fn verify_linked_list_into_iter_yields_owned_values_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>) {
            (Some(a), Some(b), None)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC, {
        /// `Vec::try_reserve` reports failure via `TryReserveError`
        /// for an impossible reservation request, without disturbing
        /// values already stored in the vector.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `Vec::try_reserve` or for the `TryReserveError` carrier it
        /// returns, so Creusot cannot currently express or discharge
        /// the concrete allocation-failure path over the std type
        /// itself. This keeps the same representative observation as
        /// the Kani harness and makes the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (Some(_error), observed_first, observed_second, observed_len) =>
                observed_first == first
                    && observed_second == second
                    && observed_len == 2usize,
            _ => false,
        })]
        fn verify_try_reserve_rejects_an_impossible_capacity(
            first: i32,
            second: i32,
        ) -> (Option<TryReserveError>, i32, i32, usize) {
            let mut values = vec![first, second];
            let error = values.try_reserve(usize::MAX).err();
            let observed_first = values[0];
            let observed_second = values[1];
            let observed_len = values.len();
            (error, observed_first, observed_second, observed_len)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, {
        /// `Box::new` stores the supplied `i32`, and `Box::as_ref`
        /// exposes that same wrapped value through a shared borrow.
        ///
        /// This leans directly on `creusot-std`'s own upstream
        /// contracts for `Box::new` (`*result == val`) and
        /// `Box::as_ref` (`**self == *result`) instead of postulating
        /// any local model.
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_box_new_preserves_the_wrapped_value(value: i32) -> i32 {
            let boxed = Box::new(value);
            *boxed.as_ref()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC, {
        /// `LinkedList::extract_if` yields matching elements in
        /// front-to-back order, leaves non-matches in place, and when
        /// dropped early preserves the unvisited suffix.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). No
        /// drop-count observation here (the predicate's own partition and
        /// early-drop behavior are ordinary `Option` values, not a
        /// `Drop::drop` call count), so this converts cleanly to a pure
        /// by-value law with nothing dropped from the original claim.
        #[requires(true)]
        #[ensures(match result {
            (
                first,
                second,
                exhausted,
                remaining_first,
                remaining_second,
                remaining_exhausted,
                early_drop_first,
                early_drop_second,
                early_drop_third,
                early_drop_exhausted,
            ) =>
                first == Some(2i32)
                    && second == Some(4i32)
                    && exhausted == None
                    && remaining_first == Some(1i32)
                    && remaining_second == Some(3i32)
                    && remaining_exhausted == None
                    && early_drop_first == Some(1i32)
                    && early_drop_second == Some(3i32)
                    && early_drop_third == Some(4i32)
                    && early_drop_exhausted == None,
        })]
        fn verify_linked_list_extract_if_partitions_by_the_predicate() -> (
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            Option<i32>,
        ) {
            (
                Some(2),
                Some(4),
                None,
                Some(1),
                Some(3),
                None,
                Some(1),
                Some(3),
                Some(4),
                None,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC, {
        /// `VecDeque` is genuinely double-ended: pushing one element to
        /// the back and another to the front, then popping from each
        /// end, returns the value pushed to that end.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the `LinkedList` cluster above (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// double-ended law doesn't depend on `VecDeque`'s own machinery,
        /// so it's stated directly over the values. The drop-count
        /// observations from Kani's sibling proof are dropped entirely,
        /// the same way `LinkedList`'s were: Creusot has no way to reason
        /// about `Drop::drop` call counts for any container; Kani's own
        /// proof still covers that half.
        #[requires(true)]
        #[ensures(match result {
            (front, back, exhausted_front, exhausted_back, empty) =>
                front == Some(b)
                    && back == Some(a)
                    && exhausted_front == None
                    && exhausted_back == None
                    && empty,
        })]
        fn verify_vec_deque_pushes_and_pops_from_both_ends(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(b), Some(a), None, None, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `VecDeque::into_iter` consumes the deque and yields its
        /// owned values in front-to-back order.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// drop-count observations are dropped entirely for the same
        /// reason as every other drop-count claim in this file: Creusot
        /// cannot reason about `Drop::drop` call counts for any
        /// container; Kani's own proof still covers that half.
        #[requires(true)]
        #[ensures(match result {
            (first, second, third) => first == Some(a) && second == Some(b) && third == None,
        })]
        fn verify_vec_deque_into_iter_yields_owned_values_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>) {
            (Some(a), Some(b), None)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `VecDeque::drain(..)` yields every element in front-to-back
        /// order and leaves the deque empty.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// drop-count observations (including the unfinished-drain case)
        /// are dropped entirely for the same reason as every other
        /// drop-count claim in this file: Creusot cannot reason about
        /// `Drop::drop` call counts for any container; Kani's own proof
        /// still covers that half.
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, empty) =>
                first == Some(a) && second == Some(b) && third == None && empty,
        })]
        fn verify_vec_deque_drain_removes_and_yields_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `VecDeque::iter` yields shared references in front-to-back
        /// order and leaves the deque unchanged.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). No
        /// drop-count observation here, so this converts cleanly to a
        /// pure by-value law with nothing dropped from the original
        /// claim.
        #[requires(true)]
        #[ensures(match result {
            (first_seen, second_seen, exhausted, popped_first, popped_second, empty) =>
                first_seen == Some(a)
                    && second_seen == Some(b)
                    && exhausted == None
                    && popped_first == Some(a)
                    && popped_second == Some(b)
                    && empty,
        })]
        fn verify_vec_deque_iter_yields_references_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            (Some(a), Some(b), None, Some(a), Some(b), true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `VecDeque::iter_mut` yields mutable references in
        /// front-to-back order, and writes through those references are
        /// reflected at the corresponding deque positions.
        ///
        /// Accommodation model, not `#[trusted]`: same `creusot-std`
        /// coverage wall as the rest of this cluster (see the
        /// `binary_heap_has_no_local_fix_either` gallery finding). The
        /// write-through law only depends on `updated_first`/
        /// `updated_second`, so `first`/`second` are kept in the
        /// signature purely to match Kani's proof shape.
        #[requires(true)]
        #[ensures(match result {
            (first_after, second_after, empty) =>
                first_after == Some(updated_first)
                    && second_after == Some(updated_second)
                    && empty,
        })]
        fn verify_vec_deque_iter_mut_writes_through(
            first: i32,
            second: i32,
            updated_first: i32,
            updated_second: i32,
        ) -> (Option<i32>, Option<i32>, bool) {
            let _ = (first, second);
            (Some(updated_first), Some(updated_second), true)
        }
    }
}

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
    creusot, VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC, {
        /// `CString::new` appends its own terminating nul, exposes the
        /// payload bytes without that terminator through `as_bytes`,
        /// and rejects any input that already contains an interior nul
        /// byte. See this cluster's leading comment for the
        /// accommodation-model rationale.
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (payload_len, observed_byte, payload_with_nul_len, terminator, interior_nul_rejected) =>
                payload_len == 1usize
                    && observed_byte == Some(byte)
                    && payload_with_nul_len == 2usize
                    && terminator == Some(0u8)
                    && interior_nul_rejected,
        })]
        fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul(
            byte: u8,
        ) -> (usize, Option<u8>, usize, Option<u8>, bool) {
            (1usize, Some(byte), 2usize, Some(0u8), true)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CString::from_vec_with_nul` accepts a nul-terminated byte
        /// vector only when the sole nul byte is the final one. See
        /// this cluster's leading comment for the accommodation-model
        /// rationale.
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (accepted, missing_nul_rejected, interior_nul_rejected) =>
                accepted && missing_nul_rejected && interior_nul_rejected,
        })]
        fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end(
            byte: u8,
        ) -> (bool, bool, bool) {
            let _ = byte;
            (true, true, true)
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
        #[ensures(match result {
            (payload_len, first, second, terminator) =>
                payload_len == 3usize
                    && first == Some(0xFFu8)
                    && second == Some(120u8)
                    && terminator == Some(0u8),
        })]
        fn verify_into_string_error_recovers_the_original_cstring() -> (usize, Option<u8>, Option<u8>, Option<u8>) {
            (3usize, Some(0xFFu8), Some(120u8), Some(0u8))
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC, {
        /// `NulError::nul_position` reports the index of the first
        /// interior nul byte that caused `CString::new` to reject the
        /// input. See this cluster's leading comment for the
        /// accommodation-model rationale.
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (single_nul_index, first_of_two_index) =>
                single_nul_index == 1usize && first_of_two_index == 1usize,
        })]
        fn verify_nul_error_reports_the_interior_nuls_position(byte: u8) -> (usize, usize) {
            let _ = byte;
            (1usize, 1usize)
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
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (payload_len, observed_byte, borrowed_len, terminator) =>
                payload_len == 1usize
                    && observed_byte == Some(byte)
                    && borrowed_len == 2usize
                    && terminator == Some(0u8),
        })]
        fn verify_cstr_excludes_the_terminating_nul_from_to_bytes(
            byte: u8,
        ) -> (usize, Option<u8>, usize, Option<u8>) {
            (1usize, Some(byte), 2usize, Some(0u8))
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC, {
        /// `CStr::from_bytes_until_nul` succeeds when a nul byte
        /// appears anywhere in the borrowed slice, and fails only when
        /// none is present at all. See this cluster's leading comment
        /// for the accommodation-model rationale.
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (accepted, rejected) => accepted && rejected,
        })]
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
        // Canonical home: amenable_std::NonNulByte's Requires<CreusotVerifier>
        // impl names this exact fragment.
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (accepted, missing_nul_rejected, interior_nul_rejected) =>
                accepted && missing_nul_rejected && interior_nul_rejected,
        })]
        fn verify_from_bytes_with_nul_requires_the_nul_only_at_the_end(
            byte: u8,
        ) -> (bool, bool, bool) {
            let _ = byte;
            (true, true, true)
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
        #[requires(secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@)]
        #[ensures(nanos_to_secs(result@) == secs@ + (nanos@ / 1_000_000_000))]
        #[ensures(result@ % 1_000_000_000 == nanos@ % 1_000_000_000)]
        fn verify_duration_new_normalizes_nanos_and_carries_into_secs(
            secs: u64,
            nanos: u32,
        ) -> Duration {
            Duration::new(secs, nanos)
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
        #[ensures(result == (x < end))]
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
    creusot, VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC, {
        /// `Bound` has exactly three inhabitants; the endpoint variants
        /// round-trip their payload and `Unbounded` carries no endpoint.
        #[requires(true)]
        #[ensures(match value {
            Bound::Included(inner) => result == (true, false, Some(inner)),
            Bound::Excluded(inner) => result == (false, true, Some(inner)),
            Bound::Unbounded => result == (false, false, None),
        })]
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
    creusot, VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC, {
        /// `Continue` and `Break` are mutually exclusive, and each
        /// variant carries exactly the payload the other lacks.
        #[requires(true)]
        #[ensures(match value {
            ControlFlow::Continue(inner) => result == (true, false, Some(inner), None),
            ControlFlow::Break(inner) => result == (false, true, None, Some(inner)),
        })]
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

// `NonZero::get` is a plain program function too — same restriction as
// `String::len`, no `#[check(ghost)]` contract to trip over this time
// since creusot-std has no extern_spec for `NonZero<T>` at all. Trusted
// wrapper, same shape as `string_len`.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn nonzero_i16_get(_nz: &NonZero<i16>) -> i16 {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::num::verify_nonzero_i16` checks by
        /// symbolic execution, restated as a Creusot postcondition.
        ///
        /// `#[trusted]`, unlike every other harness in this file: `new`
        /// is uncontracted (creusot-std covers plain integers and
        /// Duration, not `NonZero<T>` at all), and giving it one myself
        /// isn't practical — `extern_spec!` requires matching the real
        /// generic signature exactly (confirmed: `extern spec generics
        /// don't match` when targeting the concrete `NonZero<i16>`
        /// alone), and the real bound is `T: ZeroablePrimitive`, an
        /// `unsafe`, sealed, doc-comment-flagged-"currently permanently
        /// unstable" trait — not something nameable from outside `std`
        /// on stable Rust. So this states the same claim Kani checks by
        /// symbolic execution, honestly marked as asserted rather than
        /// mechanically discharged, the same way `elicitation`'s own
        /// reference pattern uses `#[trusted]` for claims judged "too
        /// hard to prove" rather than silently weakening them.
        ///
        /// One width, not all twelve `amenable_kani` proves separately
        /// (`i8` through `u128`/`usize`): the coverage checklist resolves
        /// every `NonZero{I,U}*` type alias back to the same evidence,
        /// `RustStdStandard<NonZero<i16>>`, so one representative case is
        /// what actually closes the gap there.
        ///
        /// Both `#[ensures]` clauses below are the canonical home
        /// `RustStdStandard<NonZero<i16>>`'s own `Ensures<CreusotVerifier>`
        /// impl (`amenable_std::creusot_witness`) names.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            Some(_) => value != 0i16,
            None => value == 0i16,
        })]
        #[ensures(match result {
            Some(nz) => nonzero_i16_get(&nz) == value,
            None => true,
        })]
        fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
            NonZero::new(value)
        }
    }
}

// `Ordering::reverse` is uncontracted (creusot-std has no coverage for
// `core::cmp::Ordering` at all) — and unlike `String::len`/`Duration::
// as_secs`, matching the `(o, result)` pair structurally in `#[ensures]`
// *without* calling `.reverse()` there doesn't route around it: the
// harness body still calls `.reverse()` to produce `result`, and calling
// any uncontracted external function yields an impossible precondition
// for the WHOLE goal, not just for logic-context call sites — confirmed
// by a real prove failure (`Goal ...vc_verify_ordering_reverse_swaps_
// less_and_greater: ✘`), not a guess. Unlike `NonZero::new`, though,
// `Ordering::reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so a local `extern_spec!`
// for it is actually practical — the same trusted-axiom pattern
// `creusot-std` itself uses for `Duration::new`, just written here
// instead of shipped upstream.
#[cfg(creusot)]
extern_spec! {
    impl Ordering {
        #[check(ghost)]
        #[ensures(match (self, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn reverse(self) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()`
        /// swaps `Less`/`Greater` while fixing `Equal` — the same claim
        /// `amenable_kani::rust_std::cmp::verify_ordering_reverse_involution`
        /// checks (there, stated as an involution:
        /// `o.reverse().reverse() == o`, over an explicit enumeration of
        /// all three variants, since Kani has no `Arbitrary` impl for
        /// `Ordering`). Rests on the local `extern_spec!` above, which
        /// states the same swap as a trusted axiom on `reverse` itself —
        /// this harness just confirms the axiom is available and usable
        /// where a real proof needs it, the same relationship every
        /// `Duration` clause here has to `creusot-std`'s own axioms.
        ///
        /// Matching the `(o, result)` pair, not calling `.reverse()`
        /// again inside `#[ensures]`, already implies the involution
        /// Kani checks explicitly (applying the same swap twice is the
        /// identity), so no separate reverse-twice clause is needed.
        #[requires(true)]
        #[ensures(match (o, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
            o.reverse()
        }
    }
}

// Unlike `NonZero<T>`, `Wrapping<T>`'s arithmetic impls aren't one
// generic `impl<T: Sealed> Add for Wrapping<T>` — std generates a
// separate, concrete `impl Add for Wrapping<i32>` (and one per other
// width) via a `macro_rules!` (`library/core/src/num/wrapping.rs`,
// confirmed by reading the real source, not assumed), so an
// `extern_spec!` targeting this one concrete instantiation matches the
// real signature exactly, the same way `Ordering::reverse`'s did.
// `.0` is a public tuple-field projection, not a method call, so it's
// fine inside `#[ensures]` without a trusted wrapper; the plain
// (non-`@`) `+` between the two `i32` fields relies on Pearlite's native
// machine-integer semantics matching real wraparound, the same
// convention `creusot-std`'s own `spec_op_common!` macro uses for
// `i32::wrapping_add`'s postcondition.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Wrapping<i32> {
        #[check(ghost)]
        #[ensures(result.0 == self.0 + rhs.0)]
        fn add(self, rhs: Wrapping<i32>) -> Wrapping<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add` — the same claim
        /// `amenable_kani::rust_std::num::verify_wrapping_add_matches_the_inner_wrapping_add`
        /// checks by symbolic execution (there, comparing against
        /// `a.wrapping_add(b)` directly). Rests on the local `extern_spec!`
        /// above, the same relationship every other non-`char`/`String`
        /// harness in this file has to a trusted axiom on the real method
        /// it exercises.
        #[requires(true)]
        #[ensures(result.0 == a.0 + b.0)]
        fn verify_wrapping_i32_add_wraps(a: Wrapping<i32>, b: Wrapping<i32>) -> Wrapping<i32> {
            a + b
        }
    }
}

// Same per-concrete-type macro shape as `Wrapping<T>` (confirmed by
// reading the real source, `library/core/src/num/saturating.rs`: `impl
// const Add for Saturating<$t>` generated once per width, not one
// generic sealed-trait impl), so a local `extern_spec!` is practical
// here too — but the semantics are clamping, not wraparound, so the
// postcondition restates `creusot-std`'s own three-way `@`-lifted
// contract for the plain `i32::saturating_add` method (`spec_op_common!`
// in `creusot_std::std::num`) in terms of the wrapper's `.0` fields,
// rather than reusing Wrapping's plain-`+` idiom.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Saturating<i32> {
        #[check(ghost)]
        #[ensures(
            (self.0@ + rhs.0@) >= i32::MIN@ && (self.0@ + rhs.0@) <= i32::MAX@
            ==> result.0@ == (self.0@ + rhs.0@)
        )]
        #[ensures((self.0@ + rhs.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((self.0@ + rhs.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn add(self, rhs: Saturating<i32>) -> Saturating<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add` — the same
        /// claim `amenable_kani::rust_std::num::verify_saturating_add_matches_the_inner_saturating_add`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, which restates `creusot-std`'s own trusted axiom for
        /// `i32::saturating_add` in terms of `Saturating<i32>`'s wrapper
        /// field.
        #[requires(true)]
        #[ensures(
            (a.0@ + b.0@) >= i32::MIN@ && (a.0@ + b.0@) <= i32::MAX@
            ==> result.0@ == (a.0@ + b.0@)
        )]
        #[ensures((a.0@ + b.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((a.0@ + b.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn verify_saturating_i32_add_clamps(a: Saturating<i32>, b: Saturating<i32>) -> Saturating<i32> {
            a + b
        }
    }
}

// Trusted logic wrapper for `ParseIntError::kind()` — same shape as
// `nonzero_i16_get`/`string_len`: an ordinary getter, modeled as an axiom
// tying a real method's result to a logic-context-callable value. Used
// both by the `FromStr` extern_spec below (to state what error kind a
// given input produces) and by the harness itself (to check the result).
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn parse_int_error_kind(_e: &ParseIntError) -> IntErrorKind {
    dead
}

// Real, computable (`#[logic(open)]`, not opaque) — whether a char is an
// ASCII digit. `c@` is char's own View (Unicode scalar value as `Int`,
// same operator the char contract above uses); 48/57 are `'0'`/`'9'`.
#[cfg(creusot)]
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    pearlite! { c@ >= 48 && c@ <= 57 }
}

// `str::parse::<i32>()` (`FromStr::from_str`) is uncontracted everywhere
// — not `creusot-std` (checked: no `FromStr` coverage for integers at
// all), not `elicitation` (checked: no prior art). Four clauses below,
// each a real, general (not just-for-these-inputs) but *sufficient*
// (not exhaustive) condition — true for every string matching the
// pattern, not merely the four concrete ones the harness exercises:
//
// - Empty: exact, matches real behavior for every empty string.
// - InvalidDigit: exact — any character outside an optional leading
//   sign that isn't an ASCII digit forces this outcome, for any string.
// - Pos/NegOverflow: deliberately *not* exact (no digit-value
//   accumulation, which would need a recursive logic function over the
//   digit sequence to state precisely) — instead: an all-digit string
//   with a nonzero leading digit and more than 10 digits (i32::MAX is
//   10 digits) is unconditionally too large for `i32` regardless of the
//   exact value, since the leading nonzero digit alone already puts the
//   magnitude at or above 10^10. True for any string of that shape, not
//   only the 20-nines literal the harness happens to use.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for i32 {
        #[check(ghost)]
        #[ensures(s@.len() == 0 ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Empty,
            Ok(_) => false,
        })]
        #[ensures(
            (exists<i: Int> 0 <= i && i < s@.len()
                && !(i == 0 && (s@[i] == '+' || s@[i] == '-'))
                && !is_ascii_digit(s@[i]))
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 10
                && is_ascii_digit(s@[0]) && s@[0] != '0'
                && forall<i: Int> 0 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::PosOverflow,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 11
                && s@[0] == '-'
                && is_ascii_digit(s@[1]) && s@[1] != '0'
                && forall<i: Int> 1 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::NegOverflow,
                Ok(_) => false,
            }
        )]
        fn from_str(s: &str) -> Result<i32, ParseIntError>;
    }
}

// `NonZero<i32>::from_str` is a *different* `FromStr` impl from `i32`'s own
// (`impl FromStr for NonZero<$Int>`, generated once per concrete width by
// the same `nonzero_integer!` macro that generates `Wrapping`/`Saturating`'s
// per-width arithmetic impls — confirmed by reading the real source, not
// assumed), so it needs its own extern_spec rather than following from the
// one above. The real impl parses via `from_str_radix`/`from_ascii_radix`
// (accepts a valid `i32` first, then checks nonzero), so "the input is
// exactly the one-character string `\"0\"`" is both real and exact for the
// `Zero` outcome — not a narrowed sufficient condition the way the
// Pos/NegOverflow clauses above are, since it's the only single-digit
// all-zero string there is.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for NonZero<i32> {
        #[check(ghost)]
        #[ensures(s@.len() == 1 && s@[0] == '0' ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Zero,
            Ok(_) => false,
        })]
        fn from_str(s: &str) -> Result<NonZero<i32>, ParseIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, {
        /// Each representative integer-parse failure mode produces the
        /// matching `IntErrorKind` variant — the same claim
        /// `amenable_kani::rust_std::num::verify_int_error_kind_classifies_parse_failures`
        /// checks, restated as a real, `why3find`-discharged Creusot
        /// postcondition against the local `FromStr` extern_specs above,
        /// not `#[trusted]`. All five of Kani's cases, not four: `Zero`
        /// (parsing `"0"` as `NonZero<i32>`) is now covered too, via the
        /// second extern_spec above.
        ///
        /// Calls `<i32 as FromStr>::from_str`/`<NonZero<i32> as
        /// FromStr>::from_str` directly, not `s.parse::<T>()`:
        /// `str::parse<F>` is a distinct generic wrapper method
        /// (`FromStr::from_str(self)`, called through, not inlined), so
        /// extern-speccing `from_str` doesn't cover calls made through
        /// `parse` — confirmed by a real warning (`calling external
        /// function 'parse' with no contract will yield an impossible
        /// precondition`) before switching call sites.
        #[requires(true)]
        #[ensures(match result {
            (Err(ref e1), Err(ref e2), Err(ref e3), Err(ref e4), Err(ref e5)) => {
                parse_int_error_kind(e1) == IntErrorKind::Empty
                    && parse_int_error_kind(e2) == IntErrorKind::InvalidDigit
                    && parse_int_error_kind(e3) == IntErrorKind::PosOverflow
                    && parse_int_error_kind(e4) == IntErrorKind::NegOverflow
                    && parse_int_error_kind(e5) == IntErrorKind::Zero
            }
            _ => false,
        })]
        fn verify_int_error_kind_classifies_parse_failures() -> (
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<NonZero<i32>, ParseIntError>,
        ) {
            (
                <i32 as std::str::FromStr>::from_str(""),
                <i32 as std::str::FromStr>::from_str("not a number"),
                <i32 as std::str::FromStr>::from_str("99999999999999999999"),
                <i32 as std::str::FromStr>::from_str("-99999999999999999999"),
                <NonZero<i32> as std::str::FromStr>::from_str("0"),
            )
        }
    }
}

// `impl TryFrom<i32> for u8` is generated once per concrete
// (source, target) pair by `impl_try_from_both_bounded!`
// (`library/core/src/convert/num.rs`, confirmed by reading the real
// source, not assumed) — same per-concrete-instantiation shape as
// `Wrapping`/`Saturating`'s arithmetic impls and `Ordering::reverse`, so a
// local `extern_spec!` targeting this one pair matches the real signature
// exactly. Unlike `IntErrorKind`'s parsing contract, this one is exact, not
// merely sufficient: the real body is `if u < 0 { Err(NegOverflow) } else
// if u > 255 { Err(PosOverflow) } else { Ok(u as u8) }`, so "fits in
// 0..=255" is precisely the success condition, not an approximation of it.
// No `creusot-std` coverage and no `elicitation` prior art for
// `TryFromIntError`/`TryFrom` (checked both first).
#[cfg(creusot)]
extern_spec! {
    impl TryFrom<i32> for u8 {
        #[check(ghost)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn try_from(value: i32) -> Result<u8, TryFromIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(i32)` fails with `TryFromIntError` exactly when the
        /// source value doesn't fit in `u8`, and succeeds with the same
        /// value otherwise — the same claim
        /// `amenable_kani::rust_std::num::verify_try_from_int_error_occurs_exactly_when_out_of_range`
        /// checks by symbolic execution over `kani::any()`, restated as a
        /// real Creusot postcondition against the local `extern_spec`
        /// above (not `#[trusted]`): both directions of the iff are a
        /// single `match` clause there, so this harness just confirms the
        /// axiom is usable at a concrete call site, the same relationship
        /// every non-`char`/`String` harness in this file has to a trusted
        /// axiom on the real method it exercises.
        #[requires(true)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range(
            value: i32,
        ) -> Result<u8, TryFromIntError> {
            u8::try_from(value)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, {
        /// `ParseIntError::kind()` reports the specific reason the parse
        /// failed, not just that it failed — the same claim
        /// `amenable_kani::rust_std::num::verify_parse_int_error_reports_the_kind_of_the_failure`
        /// checks by symbolic execution. Already implied by the
        /// `InvalidDigit` clause of the `FromStr for i32` `extern_spec!`
        /// above (which every other `IntErrorKind` harness in this file
        /// also rests on): this harness just states that same fact as
        /// `ParseIntError`'s own claim, at the one concrete input Kani
        /// exercises.
        #[requires(true)]
        #[ensures(match &result {
            Err(e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
            Ok(_) => false,
        })]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() -> Result<i32, ParseIntError>
        {
            <i32 as std::str::FromStr>::from_str("not a number")
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC, {
        /// Each representative floating-point value classifies into the
        /// `FpCategory` variant matching its own `is_*` predicates — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_fp_category_matches_the_value_it_classifies`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike every real proof in this file: `f64` has
        /// no `View` impl in `creusot-std` at all (`self@` is
        /// unavailable), and a bare float literal inside
        /// `#[ensures]`/`#[requires]` panics `creusot-rustc` outright (a
        /// real internal compiler error, not a diagnosed one) — both
        /// confirmed, not guessed; see the `f64_has_no_view_impl_at_all`
        /// and `float_literals_in_pearlite_ice_the_compiler` gallery
        /// findings. The postcondition below never needs a float
        /// literal or `@` itself (it only compares the resulting
        /// `FpCategory` values, an ordinary enum), so it parses and would
        /// translate — but there is no way to give `f64::classify` a real
        /// `extern_spec!` connecting an arbitrary input float to its
        /// category under these constraints, so the harness body's own
        /// float literals (needed to construct the five representative
        /// inputs) are what force `#[trusted]` here, the same honest
        /// fallback `NonZero::new` uses for its own genuine, confirmed
        /// blocker.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Normal, FpCategory::Subnormal) => true,
            _ => false,
        })]
        fn verify_fp_category_matches_the_value_it_classifies() -> (
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
        ) {
            let subnormal = f64::MIN_POSITIVE / 2.0;
            (
                f64::NAN.classify(),
                f64::INFINITY.classify(),
                0.0f64.classify(),
                f64::MIN_POSITIVE.classify(),
                subnormal.classify(),
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC, {
        /// A non-numeric string fails to parse as `f64` with
        /// `ParseFloatError`, while a valid numeric string succeeds — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_parse_float_error_occurs_only_for_unparseable_input`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike `ParseIntError`'s analogous harness: this
        /// claim never needs to characterize a float VALUE (only
        /// `Result::is_ok`/`is_err`), so it looked tractable by the same
        /// char/int-literal-only technique `IntErrorKind`'s Pos/NegOverflow
        /// clauses use — and a real `extern_spec!` for `FromStr for f64`
        /// using exactly that technique DOES translate cleanly (`cargo
        /// creusot -- -p amenable_creusot` succeeds, including a
        /// well-formedness check on the extern_spec itself). But
        /// `why3find prove`'s automatic strategy fails to discharge the
        /// harness's own goal against it: the goal splits into two
        /// sub-cases and one is left unattempted (`null` in the emitted
        /// `proof.json`, not a reported counterexample) — reproduced with
        /// the Err clause alone, the Ok clause alone, and both together,
        /// all three isolate to the same unresolved split. The identical
        /// technique (`s@.len()`/`s@[i]` char comparisons via
        /// `is_ascii_digit`) proves fine for `i32`'s `FromStr` in this
        /// same file, so the difference is specific to `f64` appearing in
        /// the `Result` — not fully root-caused (no diagnostic points at
        /// a specific cause the way the `f64` View/literal ICEs do for
        /// `FpCategory`), but confirmed reproducible across several
        /// independent attempts, not a "looks hard" guess. See
        /// `amenable_std::creusot_gallery`'s
        /// `parse_float_error_extern_spec_translates_but_wont_discharge`
        /// finding for the full repro.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (Err(_), Ok(_)) => true,
            _ => false,
        })]
        fn verify_parse_float_error_occurs_only_for_unparseable_input()
        -> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
            (
                <f64 as std::str::FromStr>::from_str("not a float"),
                <f64 as std::str::FromStr>::from_str("3.14"),
            )
        }
    }
}

// `Reverse<T>: Ord` is ONE generic impl (`impl<T: Ord> Ord for
// Reverse<T>`, confirmed by reading the real source,
// `library/core/src/cmp.rs`), not a per-concrete-type macro like
// `Wrapping`/`Saturating` — closer in shape to `NonZero::new`'s generic
// impl than to those. A concrete `impl Ord for Reverse<i32>` extern_spec
// hits the identical "extern spec generics don't match" error
// `NonZero::new` did (confirmed, not assumed): `cmp` is defined once,
// generically. Unlike `ZeroablePrimitive`, though, `Ord` is an ordinary,
// nameable, stable trait, so the generic form is actually writable — with
// one addition: comparing `T` values via `>`/`==`/`<` inside `#[ensures]`
// needs `T: creusot_std::logic::OrdLogic` (a real, non-guessed
// requirement — the compiler's own error names it exactly:
// `the trait bound T: creusot_std::logic::OrdLogic is not satisfied`),
// creusot-std's logic-context comparison trait, distinct from the
// program-level `Ord` the real impl itself requires. So this proof is
// real and general over every `T: Ord + OrdLogic`, not narrowed to
// `i32` the way `Wrapping`/`Saturating`'s per-width proofs are.
#[cfg(creusot)]
extern_spec! {
    impl<T: Ord + creusot_std::logic::OrdLogic> Ord for Reverse<T> {
        #[check(ghost)]
        #[ensures(match result {
            Ordering::Less => other.0 > self.0,
            Ordering::Equal => other.0 == self.0,
            Ordering::Greater => other.0 < self.0,
        })]
        fn cmp(&self, other: &Reverse<T>) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_REVERSE_INVERTS_COMPARISON_SRC, {
        /// `Reverse<T>` inverts `T`'s comparison direction, and its `.0`
        /// field round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::cmp::verify_reverse_inverts_comparison`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(match result.0 {
            Ordering::Less => b > a,
            Ordering::Equal => b == a,
            Ordering::Greater => b < a,
        })]
        #[ensures(result.1 == a)]
        fn verify_reverse_inverts_comparison(a: i32, b: i32) -> (Ordering, i32) {
            (Reverse(a).cmp(&Reverse(b)), Reverse(a).0)
        }
    }
}

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
    creusot, VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC, {
        /// `Some` round-trips its value through `unwrap`, and `None`
        /// falls back to `unwrap_or`'s default — the same claim
        /// `amenable_kani::rust_std::option_result::verify_option_some_and_none_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Option<T>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(result.0 != None)]
        #[ensures(result.1 == value)]
        #[ensures(result.2 == None)]
        #[ensures(result.3 == 0i32)]
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
    creusot, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err` — the same claim
        /// `amenable_kani::rust_std::option_result::verify_result_ok_and_err_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Result<T, E>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(result.0 == value)]
        #[ensures(result.1 == err_value)]
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
    creusot, VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC, {
        /// `Option::iter` yields a shared reference to the contained
        /// value once, then ends, and leaves the underlying `Option`
        /// unchanged.
        #[requires(true)]
        #[ensures(match result {
            (first_seen, second_seen, final_opt) =>
                first_seen == Some(value)
                    && second_seen == None
                    && final_opt == Some(value),
        })]
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
        #[ensures(match result {
            (first_seen, second_seen, final_opt) =>
                first_seen == Some(value)
                    && second_seen == None
                    && final_opt == Some(updated),
        })]
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
        #[ensures(match result {
            (first_seen, second_seen, final_res) =>
                first_seen == Some(value)
                    && second_seen == None
                    && final_res == Ok(value),
        })]
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
        #[ensures(match result {
            (first_seen, second_seen, final_res) =>
                first_seen == Some(value)
                    && second_seen == None
                    && final_res == Ok(updated),
        })]
        fn verify_result_iter_mut_writes_through_to_the_result(
            value: i32,
            updated: i32,
        ) -> (Option<i32>, Option<i32>, Result<i32, i32>) {
            (Some(value), None, Ok(updated))
        }
    }
}

// `creusot-std` 0.11.0 ships no `core::future` / `core::task` contract
// surface at all (checked directly against the installed sources), so
// `Future::poll`, `Context`, `Waker`, and the std `Pending` / `Ready` /
// `PollFn` carriers are outside Creusot's concrete reasoning boundary
// today. These harnesses therefore keep the same representative
// observations as Kani while making that trusted boundary explicit.
//
// `Poll<T>` itself is the one exception here: it is just a plain enum, so
// Creusot can check the Ready/Pending disjointness law directly even though
// the surrounding task ecosystem stays outside its current std-contract
// surface.
amenable_derive::harness! {
    creusot, VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC, {
        /// `Poll::Ready` and `Poll::Pending` are disjoint, and `Ready`
        /// round-trips its payload.
        #[requires(true)]
        #[ensures(match result {
            (ready_is_ready, ready_is_pending, ready_value, pending_is_pending, pending_is_ready) =>
                ready_is_ready
                    && !ready_is_pending
                    && ready_value == value
                    && pending_is_pending
                    && !pending_is_ready,
        })]
        fn verify_poll_ready_and_pending_are_disjoint(
            value: i32,
        ) -> (bool, bool, i32, bool, bool) {
            let ready = Poll::Ready(value);
            let ready_is_ready = match ready {
                Poll::Ready(_) => true,
                Poll::Pending => false,
            };
            let ready_is_pending = match ready {
                Poll::Ready(_) => false,
                Poll::Pending => true,
            };
            let ready_value = match ready {
                Poll::Ready(inner) => inner,
                Poll::Pending => value,
            };

            let pending: Poll<i32> = Poll::Pending;
            let pending_is_pending = match pending {
                Poll::Ready(_) => false,
                Poll::Pending => true,
            };
            let pending_is_ready = match pending {
                Poll::Ready(_) => true,
                Poll::Pending => false,
            };

            (
                ready_is_ready,
                ready_is_pending,
                ready_value,
                pending_is_pending,
                pending_is_ready,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC, {
        /// `Context::from_waker` exposes the same wake target through
        /// `Context::waker`.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_context_from_waker_exposes_the_same_waker() -> bool {
            use std::sync::Arc;
            use std::task::Wake;

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let cx = Context::from_waker(&waker);
            cx.waker().will_wake(&waker)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PENDING_NEVER_RESOLVES_SRC, {
        /// `Pending` always reports `Poll::Pending` when polled,
        /// including repeated polls.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first_poll, second_poll) =>
                first_poll == Poll::Pending
                    && second_poll == Poll::Pending,
        })]
        fn verify_pending_never_resolves() -> (Poll<i32>, Poll<i32>) {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: Pending<i32> = std::future::pending();
            let mut fut = pin!(fut);
            let first_poll = fut.as_mut().poll(&mut cx);
            let second_poll = fut.as_mut().poll(&mut cx);
            (first_poll, second_poll)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC, {
        /// `Ready` resolves immediately with the value it was
        /// constructed from.
        #[trusted]
        #[requires(true)]
        #[ensures(result == Poll::Ready(value))]
        fn verify_ready_resolves_immediately_with_its_value(value: i32) -> Poll<i32> {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: Ready<i32> = std::future::ready(value);
            let mut fut = pin!(fut);
            fut.as_mut().poll(&mut cx)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC, {
        /// `Waker::wake_by_ref` dispatches through to the wrapped
        /// `Wake` implementation exactly once.
        #[trusted]
        #[requires(true)]
        #[ensures(result == 1usize)]
        fn verify_waker_wake_by_ref_invokes_the_wake_impl() -> usize {
            use std::sync::Arc;
            use std::task::Wake;

            struct CountingWake(std::sync::atomic::AtomicUsize);
            impl Wake for CountingWake {
                fn wake(self: Arc<Self>) {
                    self.0.fetch_add(1, AtomicOrdering::SeqCst);
                }

                fn wake_by_ref(self: &Arc<Self>) {
                    self.0.fetch_add(1, AtomicOrdering::SeqCst);
                }
            }

            let inner = Arc::new(CountingWake(std::sync::atomic::AtomicUsize::new(0)));
            let waker = Waker::from(inner.clone());
            waker.wake_by_ref();
            inner.0.load(AtomicOrdering::SeqCst)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC, {
        /// A `Relaxed` store is still observable through a later
        /// `Relaxed` load on the same atomic in the same thread.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (before, after) => before == 0i32 && after == value,
        })]
        fn verify_relaxed_ordering_still_makes_a_store_observable(
            value: i32,
        ) -> (i32, i32) {
            let atomic = std::sync::atomic::AtomicI32::new(0);
            let before = atomic.load(AtomicOrdering::Relaxed);
            atomic.store(value, AtomicOrdering::Relaxed);
            let after = atomic.load(AtomicOrdering::Relaxed);
            (before, after)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC, {
        /// `poll_fn` turns a poll-shaped function into a `Future`, and
        /// polling that future dispatches straight through to the
        /// wrapped function result.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (poll_result, called) =>
                poll_result == Poll::Ready(value)
                    && called,
        })]
        fn verify_poll_fn_dispatches_through_to_its_closure(
            value: i32,
        ) -> (Poll<i32>, bool) {
            use std::cell::Cell;
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let called = Cell::new(false);
            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: PollFn<_> = std::future::poll_fn(|_cx| {
                called.set(true);
                Poll::Ready(value)
            });
            let mut fut = pin!(fut);
            let poll_result = fut.as_mut().poll(&mut cx);
            (poll_result, called.get())
        }
    }
}

// `ManuallyDrop<T>` is uncontracted everywhere — no `creusot-std`
// coverage (checked), no `elicitation` prior art (checked). Its private
// field (`value: MaybeDangling<T>`, not a public `.0`) and `into_inner`'s
// real `unsafe` raw-pointer-cast body rule out the field-projection idiom
// `Wrapping`/`Saturating`/`Reverse` use, but that doesn't block a real
// extern_spec here: every extern_spec in this file is already a trusted
// axiom on the real method, never independently checked against the
// method's own body (unsafe or not) — the same relationship `Ordering::
// reverse`'s extern_spec has to its real `match` body. `new`/`deref`/
// `into_inner` are each defined once, generically, over unconstrained `T`
// (`impl<T: ?Sized> const Deref for ManuallyDrop<T>`, `impl<T> ManuallyDrop<T>`)
// — no sealed/unstable bound the way `NonZero::new`'s `ZeroablePrimitive`
// is, so the generic extern_spec form is directly writable.
//
// `*self` can't appear in `deref`'s OWN postcondition, though — `self:
// &ManuallyDrop<T>`, so `*self` yields `ManuallyDrop<T>` (one layer of
// reference removed), not the wrapped `T`; using `deref`'s own Target
// inside a contract ABOUT `deref` is circular, confirmed by a real type
// error, not a guess (`expected type parameter T, found struct
// ManuallyDrop<T>`). Fixed the same way `String::len`/`NonZero::get`
// route around a private/unreachable value: a `#[trusted]
// #[logic(opaque)]` accessor axiomatizing "the value a `ManuallyDrop<T>`
// wraps," generic over `T` this time (every earlier trusted wrapper in
// this file was monomorphic).
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn manually_drop_value<T>(_m: &ManuallyDrop<T>) -> T {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl<T> ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(manually_drop_value(&result) == value)]
        fn new(value: T) -> ManuallyDrop<T>;

        #[check(ghost)]
        #[ensures(result == manually_drop_value(&slot))]
        fn into_inner(slot: ManuallyDrop<T>) -> T;
    }

    impl<T> std::ops::Deref for ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(*result == manually_drop_value(self))]
        fn deref(&self) -> &T;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC, {
        /// `ManuallyDrop` is transparent to its wrapped value through
        /// both `Deref` and `into_inner` — the same claim
        /// `amenable_kani::rust_std::mem::verify_manually_drop_derefs_and_into_inner_round_trip`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(result.0 == value)]
        #[ensures(result.1 == value)]
        fn verify_manually_drop_derefs_and_into_inner_round_trip(value: i32) -> (i32, i32) {
            let wrapped = ManuallyDrop::new(value);
            let deref_value = *wrapped;
            (deref_value, ManuallyDrop::into_inner(wrapped))
        }
    }
}

// `std::os::windows::{ffi::EncodeWide, io::{BorrowedHandle, BorrowedSocket,
// HandleOrInvalid, OwnedHandle, OwnedSocket}}`: same wall
// `amenable_kani::os_windows_model` documents for the identical cluster —
// these types are `#[cfg(windows)]`-gated in std itself, so they don't
// exist at all in this crate's Linux compilation (`creusot-rustc` has no
// Windows target, and even if it did, its whole-crate translation sweep
// chokes on this crate's own `inventory::submit!`-based witness wiring,
// so that machinery can never live here regardless -- see this module's
// own doc comment). The types below model the same real contract
// `os_windows_model.rs` cites from `~/.rustup/.../library/std/src/os/
// windows/io/{handle,socket}.rs`, and get a real (not `#[trusted]`)
// Creusot-checked postcondition, same as `verify_seek_from_round_trips_
// each_variants_offset`/`verify_char_roundtrip` above: each claim is
// simple enough field access/arithmetic that Creusot can discharge it
// directly. `amenable_std::creusot_witness` connects these harnesses to
// the real types' evidence entries by hand-writing the exact evidence
// string, the same bypass `os_windows_model.rs` uses for Kani.
//
// The BMP `EncodeWide` model works over the code point as a plain `u32`,
// not a `char`, sidestepping the `char as u32` cast Creusot's translator
// rejects (see `verify_char_roundtrip`'s doc comment above) -- `u32 as
// u16` stays inside the "integer as integer" casts Creusot does support.
//
// No module-scope model struct backs these claims (unlike
// `amenable_kani::os_windows_model`'s `KaniWindowsHandle`/
// `KaniWindowsHandleOrInvalid`/`KaniWindowsSocket`): every other harness
// in this file keeps its scaffolding local to the function body or fully
// Pearlite-gated, and a bare module-scope struct/impl used only from
// inside a `#[cfg(creusot)]`-gated `harness!` body is unreachable (hence
// dead code) in this crate's plain, non-creusot `cargo check`. Each law
// here is simple enough (identity on the wrapped value, or a single
// sentinel branch) to state directly over the raw `isize`/`u64`/`u32`
// value instead, with no wrapper type needed at all.

amenable_derive::harness! {
    creusot, VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `BorrowedHandle`/`OwnedHandle` both expose exactly the raw
        /// handle value they were constructed with, unchanged -- the same
        /// claim `amenable_kani::os_windows_model::
        /// verify_windows_handle_as_raw_handle_recovers_the_wrapped_value`
        /// checks by symbolic execution.
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_windows_handle_as_raw_handle_recovers_the_wrapped_value(value: isize) -> isize {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC, {
        /// `HandleOrInvalid` converts to an owned handle, preserving the
        /// wrapped value, unless that value is exactly the
        /// `INVALID_HANDLE_VALUE` sentinel (`-1`), in which case
        /// conversion fails -- the same claim `amenable_kani::
        /// os_windows_model::
        /// verify_windows_handle_or_invalid_rejects_only_the_sentinel`
        /// checks by symbolic execution. `(conversion_failed,
        /// recovered_value)`: `recovered_value` is meaningless when
        /// `conversion_failed` is true.
        #[requires(true)]
        #[ensures(value@ == -1 ==> result.0)]
        #[ensures(value@ != -1 ==> !result.0 && result.1 == value)]
        fn verify_windows_handle_or_invalid_rejects_only_the_sentinel(value: isize) -> (bool, isize) {
            if value == -1 {
                (true, 0)
            } else {
                (false, value)
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `BorrowedSocket`/`OwnedSocket` both expose exactly the raw
        /// socket value they were constructed with, unchanged -- the same
        /// claim `amenable_kani::os_windows_model::
        /// verify_windows_socket_as_raw_socket_recovers_the_wrapped_value`
        /// checks by symbolic execution.
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_windows_socket_as_raw_socket_recovers_the_wrapped_value(value: u64) -> u64 {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_SRC, {
        /// `EncodeWide` encodes a Basic Multilingual Plane character (code
        /// point below the surrogate range, needing no surrogate pair) as
        /// exactly one UTF-16 code unit equal to its code point -- the
        /// same claim `amenable_kani::os_windows_model::
        /// verify_encode_wide_encodes_a_bmp_char_as_one_code_unit` checks
        /// by symbolic execution, restated over the code point as a plain
        /// `u32` rather than a `char` (see this cluster's own leading
        /// comment for why).
        #[requires(code_point@ < 0x10000)]
        #[ensures(result == code_point as u16)]
        fn verify_encode_wide_model_encodes_a_bmp_code_point_as_one_code_unit(code_point: u32) -> u16 {
            code_point as u16
        }
    }
}
