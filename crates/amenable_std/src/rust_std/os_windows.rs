//! `RustStdType` registrations for `std::os::windows`.
//!
//! The real `impl_rust_std_type*!` blocks below are `#[cfg(windows)]`-gated
//! individually, mirroring how real std itself gates this module — they
//! need the actual `std::os::windows::*` types to exist, which is only
//! true when compiled on Windows. Verified empirically on that platform
//! (this crate's primary development host).
//!
//! The `register_rust_std_standard_evidence!` call at the bottom is
//! deliberately NOT gated, unlike everything else here: its whole body is
//! `stringify!($ty)` — pure token-to-text conversion, no name resolution —
//! so it doesn't need the real types to exist to register the exact
//! evidence string the cfg(windows) impls above produce when actually
//! compiled on Windows (confirmed empirically: the identical macro shape
//! compiles fine even naming a type declared nowhere in the crate).
//! Registering it unconditionally means `evidence_link` is visible from
//! any host's coverage checklist, not just a Windows one — the same
//! "prove/register on Linux, reference the real type's evidence string"
//! move `amenable_kani::os_windows_model` already makes for the witness
//! layer, applied one layer up, to the evidence layer itself.
//! `RawHandle`/`std::os::windows::raw::HANDLE` are aliases to
//! `*mut c_void` (a pointer, not an integer), already covered via the
//! `pointer` primitive doc page's `*const i32`/`*mut i32` registrations
//! in `rust_std::primitives`; `RawSocket` is `u64` (on the 64-bit
//! targets `windows-latest` runs), covered by the `u64` primitive
//! directly. Nothing separate to impl for either.
//! `ProcThreadAttributeList`/`ProcThreadAttributeListBuilder` are
//! deliberately not covered — unstable
//! (`windows_process_extensions_main_thread_state` or a related gate).

#[cfg(windows)]
use std::os::windows::ffi::EncodeWide;
#[cfg(windows)]
use std::os::windows::io::{
    BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
};

use crate::rust_std::macros::register_rust_std_standard_evidence;
#[cfg(windows)]
use crate::rust_std::macros::{impl_rust_std_type, impl_rust_std_type_lifetime0};

#[cfg(windows)]
impl_rust_std_type_lifetime0!(
    EncodeWide,
    "std",
    "std::os::windows::ffi",
    "https://doc.rust-lang.org/std/os/windows/ffi/struct.EncodeWide.html",
    "The EncodeWide carrier lazily encodes an OsStr as UTF-16 code units, as Windows APIs expect."
);

#[cfg(windows)]
impl_rust_std_type_lifetime0!(
    BorrowedHandle,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html",
    "The BorrowedHandle carrier borrows a raw Windows HANDLE without taking ownership of it."
);

#[cfg(windows)]
impl_rust_std_type_lifetime0!(
    BorrowedSocket,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html",
    "The BorrowedSocket carrier borrows a raw Windows SOCKET without taking ownership of it."
);

#[cfg(windows)]
impl_rust_std_type!(
    HandleOrInvalid,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html",
    "The HandleOrInvalid carrier owns a Windows HANDLE that may be the sentinel INVALID_HANDLE_VALUE, deferring that check to conversion time."
);

#[cfg(windows)]
impl_rust_std_type!(
    OwnedHandle,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html",
    "The OwnedHandle carrier owns a raw Windows HANDLE, closing it on drop."
);

#[cfg(windows)]
impl_rust_std_type!(
    OwnedSocket,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedSocket.html",
    "The OwnedSocket carrier owns a raw Windows SOCKET, closing it on drop."
);

register_rust_std_standard_evidence!(
    EncodeWide<'static>,
    BorrowedHandle<'static>,
    BorrowedSocket<'static>,
    HandleOrInvalid,
    OwnedHandle,
    OwnedSocket,
);
