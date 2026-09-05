//! Verus spec for `std::os::windows::{ffi::EncodeWide, io::{BorrowedHandle,
//! BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket}}`.
//!
//! `#[cfg(windows)]`-gated at the `pub mod` declaration in `rust_std/mod.rs`
//! — this file only compiles on Windows, and Verus/CBMC don't run here
//! (this crate's primary development host is Linux), so nothing in this
//! file has been checked by `verus` locally. It is checked by the
//! `verus-windows` GitHub Actions workflow (`workflow_dispatch`-triggered,
//! `windows-latest`), the only place `verus` can actually see these types.
//!
//! Unlike `amenable_kani::os_windows_model` (a synthetic Linux-compilable
//! stand-in, forced by Kani/CBMC never running on Windows at all), these
//! are real `assume_specification` axioms against the real
//! `std::os::windows` types, matching real Kani/Creusot proofs elsewhere
//! in this project — possible here specifically because `verus` itself
//! runs natively on Windows, not just under WSL. Each axiom's `ensures`
//! is read directly from this project's own pinned toolchain's
//! `std::os::windows` source
//! (`~/.rustup/.../library/std/src/os/windows/io/{handle,socket}.rs`),
//! the same source `amenable_kani::os_windows_model`'s own doc comment
//! cites, not guessed.
//!
//! No executable `verify_...` functions accompany these axioms: every
//! safe construction path for these types (`OwnedHandle: From<fs::File>`,
//! `AsHandle for io::Stdout`, ...) either needs a real OS-backed I/O call
//! `vstd` has no spec for (the same gap `fs_content_carrier.rs`/
//! `std_ffi_carrier.rs` document for `std::fs`/`std::ffi::OsStr`), or an
//! `unsafe fn` (`BorrowedHandle::borrow_raw`, `HandleOrInvalid::
//! from_raw_handle`), which this project's proofs never use even where
//! the real API requires it. The axioms below stand alone, checked only
//! by `verus` type-checking their signatures against the real types on
//! the `windows-latest` runner.
//!
//! `EncodeWide`'s axiom covers only the structural half of its contract
//! (front-to-back, deterministic code-unit consumption) — not the
//! semantic half (`amenable_kani`'s "for a single BMP scalar value,
//! encoding is exactly the code point cast to `u16`"), which would
//! require relating `EncodeWide`'s state back to `OsStr`'s content, and
//! `vstd` has no spec support for `OsStr` at all (see
//! `std_ffi_carrier.rs`). Partial coverage of a real type's full
//! documented law, not the whole thing — the same tradeoff
//! `int_error_kind_carrier.rs` already makes for `IntErrorKind`.
//!
//! `RawHandle`/`std::os::windows::raw::HANDLE` need no separate coverage
//! here: both are aliases for `*mut c_void`, already covered by the
//! `pointer` primitive doc page (`primitive_shapes_carrier.rs`'s
//! `*const i32`/`*mut i32` axioms) — confirmed via a fresh `cordial
//! run`, not assumed; an earlier cached checklist run (predating that
//! primitive-shapes cluster) incorrectly still listed them, and
//! `rust_std/os_windows.rs`'s own doc comment is imprecise about why
//! (says "aliases to isize/u64", but `RawHandle` is a pointer, not an
//! integer — `RawSocket = u64` is the only one of the pair that's
//! actually an integer alias).

#[cfg(verus_keep_ghost)]
use std::os::windows::ffi::EncodeWide;
#[cfg(verus_keep_ghost)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, BorrowedHandle, BorrowedSocket, HandleOrInvalid, InvalidHandleError,
    OwnedHandle, OwnedSocket, RawHandle, RawSocket,
};

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExBorrowedHandle<'a>(BorrowedHandle<'a>);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExOwnedHandle(OwnedHandle);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExHandleOrInvalid(HandleOrInvalid);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExInvalidHandleError(InvalidHandleError);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExBorrowedSocket<'a>(BorrowedSocket<'a>);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExOwnedSocket(OwnedSocket);

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExEncodeWide<'a>(EncodeWide<'a>);

/// The raw handle value a `BorrowedHandle` carries — opaque
/// (uninterpreted), since the real field is private; connected to
/// `as_raw_handle()`'s result by the axiom below, mirroring
/// `int_error_kind_carrier.rs`'s `parse_int_error_kind_spec` pattern.
pub uninterp spec fn borrowed_handle_addr_spec(h: BorrowedHandle) -> usize;

pub open spec fn as_raw_handle_addr_matches(result: RawHandle, h: BorrowedHandle) -> bool {
    result.addr() == borrowed_handle_addr_spec(h)
}

pub assume_specification<'a> [<BorrowedHandle<'a> as AsRawHandle>::as_raw_handle] (h: &BorrowedHandle<'a>) -> (result: RawHandle)
    ensures
        as_raw_handle_addr_matches(result, *h),
;

/// Same shape as `borrowed_handle_addr_spec`, for `OwnedHandle`'s
/// private raw value.
pub uninterp spec fn owned_handle_addr_spec(h: OwnedHandle) -> usize;

pub open spec fn owned_as_raw_handle_addr_matches(result: RawHandle, h: OwnedHandle) -> bool {
    result.addr() == owned_handle_addr_spec(h)
}

pub assume_specification [<OwnedHandle as AsRawHandle>::as_raw_handle] (h: &OwnedHandle) -> (result: RawHandle)
    ensures
        owned_as_raw_handle_addr_matches(result, *h),
;

/// Same shape, for `HandleOrInvalid`'s private raw value.
pub uninterp spec fn handle_or_invalid_addr_spec(h: HandleOrInvalid) -> usize;

/// `TryFrom<HandleOrInvalid> for OwnedHandle`'s whole postcondition:
/// succeeds and preserves the wrapped value exactly unless it equals the
/// sentinel `INVALID_HANDLE_VALUE` (`-1` as `HANDLE`, whose exposed
/// address is `usize::MAX` under strict provenance) — `handle.rs`'s own
/// `is_valid()`: `self.0 != sys::c::INVALID_HANDLE_VALUE`.
pub open spec fn handle_or_invalid_try_from_matches(handle_or_invalid: HandleOrInvalid, result: Result<OwnedHandle, <OwnedHandle as core::convert::TryFrom<HandleOrInvalid>>::Error>) -> bool {
    (handle_or_invalid_addr_spec(handle_or_invalid) == usize::MAX <==> result is Err)
        && (result is Ok ==> owned_handle_addr_spec(result->Ok_0) == handle_or_invalid_addr_spec(handle_or_invalid))
}

pub assume_specification [<OwnedHandle as core::convert::TryFrom<HandleOrInvalid>>::try_from] (handle_or_invalid: HandleOrInvalid) -> (result: Result<OwnedHandle, <OwnedHandle as core::convert::TryFrom<HandleOrInvalid>>::Error>)
    ensures
        handle_or_invalid_try_from_matches(handle_or_invalid, result),
;

/// The raw socket value a `BorrowedSocket` carries — opaque
/// (uninterpreted), same shape as the handle carriers above.
/// `RawSocket` is a plain `u64` (on the 64-bit targets `windows-latest`
/// runs), so no pointer-address indirection is needed here.
pub uninterp spec fn borrowed_socket_value_spec(s: BorrowedSocket) -> RawSocket;

pub open spec fn as_raw_socket_matches(result: RawSocket, s: BorrowedSocket) -> bool {
    result == borrowed_socket_value_spec(s)
}

pub assume_specification<'a> [<BorrowedSocket<'a> as AsRawSocket>::as_raw_socket] (s: &BorrowedSocket<'a>) -> (result: RawSocket)
    ensures
        as_raw_socket_matches(result, *s),
;

/// Same shape, for `OwnedSocket`'s private raw value.
pub uninterp spec fn owned_socket_value_spec(s: OwnedSocket) -> RawSocket;

pub open spec fn owned_as_raw_socket_matches(result: RawSocket, s: OwnedSocket) -> bool {
    result == owned_socket_value_spec(s)
}

pub assume_specification [<OwnedSocket as AsRawSocket>::as_raw_socket] (s: &OwnedSocket) -> (result: RawSocket)
    ensures
        owned_as_raw_socket_matches(result, *s),
;

/// The code units an `EncodeWide` iterator has left to yield — opaque
/// (uninterpreted): `vstd` has no spec support for `OsStr`, so this
/// carrier states only the structural half of `EncodeWide`'s contract
/// (front-to-back, deterministic consumption), not how the sequence
/// relates to the `OsStr` it was built from. See the module doc comment.
pub uninterp spec fn encode_wide_view(iter: EncodeWide) -> Seq<u16>;

/// `EncodeWide::next`'s whole postcondition: an exhausted view yields
/// `None` and leaves the view unchanged; a non-empty view yields its
/// first code unit and advances past it.
pub open spec fn encode_wide_next_matches(before: Seq<u16>, after: Seq<u16>, result: Option<u16>) -> bool {
    (before.len() == 0 ==> result is None && after == before)
        && (before.len() > 0 ==> result == Some(before[0]) && after == before.subrange(1, before.len() as int))
}

pub assume_specification<'a> [<EncodeWide<'a> as Iterator>::next] (iter: &mut EncodeWide<'a>) -> (result: Option<u16>)
    ensures
        encode_wide_next_matches(encode_wide_view(*old(iter)), encode_wide_view(*iter), result),
;

} // verus!
