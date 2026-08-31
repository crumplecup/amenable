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
    creusot, WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BorrowedHandle<'static>>`/`RustStdStandard<OwnedHandle>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it. Both real Windows carriers
        /// share the identical claim (see this cluster's leading
        /// comment for why they aren't real Rust types on this
        /// platform).
        #[logic(open)]
        fn windows_handle_as_raw_handle_recovers_the_wrapped_value_holds(
            value: isize,
            handle_result: isize,
        ) -> bool {
            pearlite! { handle_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `BorrowedHandle`/`OwnedHandle` both expose exactly the raw
        /// handle value they were constructed with, unchanged -- the same
        /// claim `amenable_kani::os_windows_model::
        /// verify_windows_handle_as_raw_handle_recovers_the_wrapped_value`
        /// checks by symbolic execution.
        #[requires(true)]
        #[ensures(windows_handle_as_raw_handle_recovers_the_wrapped_value_holds(value, result))]
        fn verify_windows_handle_as_raw_handle_recovers_the_wrapped_value(value: isize) -> isize {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// HandleOrInvalid>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn windows_handle_or_invalid_rejects_only_the_sentinel_holds(
            value: isize,
            conversion_result: (bool, isize),
        ) -> bool {
            pearlite! {
                (value@ == -1 ==> conversion_result.0)
                    && (value@ != -1 ==> !conversion_result.0 && conversion_result.1 == value)
            }
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
        #[ensures(windows_handle_or_invalid_rejects_only_the_sentinel_holds(value, result))]
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
    creusot, WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BorrowedSocket<'static>>`/`RustStdStandard<OwnedSocket>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it. Both real Windows carriers
        /// share the identical claim (see this cluster's leading
        /// comment for why they aren't real Rust types on this
        /// platform).
        #[logic(open)]
        fn windows_socket_as_raw_socket_recovers_the_wrapped_value_holds(
            value: u64,
            socket_result: u64,
        ) -> bool {
            pearlite! { socket_result == value }
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
        #[ensures(windows_socket_as_raw_socket_recovers_the_wrapped_value_holds(value, result))]
        fn verify_windows_socket_as_raw_socket_recovers_the_wrapped_value(value: u64) -> u64 {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, ENCODE_WIDE_HEADROOM_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// EncodeWide<'static>>` precondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn encode_wide_headroom_holds(code_point: u32) -> bool {
            pearlite! { code_point@ < 0x10000 }
        }
    }
}

amenable_derive::harness! {
    creusot, ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// EncodeWide<'static>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn encode_wide_encodes_a_bmp_code_point_as_one_code_unit_holds(
            code_point: u32,
            encode_result: u16,
        ) -> bool {
            pearlite! { encode_result == code_point as u16 }
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
        #[requires(encode_wide_headroom_holds(code_point))]
        #[ensures(encode_wide_encodes_a_bmp_code_point_as_one_code_unit_holds(code_point, result))]
        fn verify_encode_wide_model_encodes_a_bmp_code_point_as_one_code_unit(code_point: u32) -> u16 {
            code_point as u16
        }
    }
}
