//! `RustStdType` registrations for `std::os::windows`.
//!
//! `#[cfg(windows)]`-gated, mirroring how real std itself gates this
//! module — compiles as a no-op on every other platform. Verified
//! empirically on this machine (Windows). `RawHandle`/`RawSocket` are
//! aliases to `isize`/`u64` respectively, already covered via
//! `rust_std::primitives` — nothing separate to impl. `ProcThreadAttributeList`/
//! `ProcThreadAttributeListBuilder` are deliberately not covered — unstable
//! (`windows_process_extensions_main_thread_state` or a related gate).

#![cfg(windows)]

use std::os::windows::ffi::EncodeWide;
use std::os::windows::io::{
    BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type_lifetime0!(
    EncodeWide,
    "std",
    "std::os::windows::ffi",
    "https://doc.rust-lang.org/std/os/windows/ffi/struct.EncodeWide.html",
    "The EncodeWide carrier lazily encodes an OsStr as UTF-16 code units, as Windows APIs expect."
);

impl_rust_std_type_lifetime0!(
    BorrowedHandle,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html",
    "The BorrowedHandle carrier borrows a raw Windows HANDLE without taking ownership of it."
);

impl_rust_std_type_lifetime0!(
    BorrowedSocket,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html",
    "The BorrowedSocket carrier borrows a raw Windows SOCKET without taking ownership of it."
);

impl_rust_std_type!(
    HandleOrInvalid,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html",
    "The HandleOrInvalid carrier owns a Windows HANDLE that may be the sentinel INVALID_HANDLE_VALUE, deferring that check to conversion time."
);

impl_rust_std_type!(
    OwnedHandle,
    "std",
    "std::os::windows::io",
    "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html",
    "The OwnedHandle carrier owns a raw Windows HANDLE, closing it on drop."
);

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
