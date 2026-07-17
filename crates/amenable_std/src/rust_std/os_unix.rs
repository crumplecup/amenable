//! `RustStdType` registrations for `std::os::unix`.
//!
//! `#[cfg(unix)]`-gated, mirroring how real std itself gates this module —
//! compiles as a no-op on Windows (this crate's primary development host),
//! and only activates on a real `unix` target. Verified with `cargo check
//! -p amenable_std --target x86_64-unknown-linux-gnu` (not a native check —
//! no Linux host was available in this session — but a real cross-target
//! type-check against `rust-std-x86_64-unknown-linux-gnu`, which caught a
//! real error on `std::os::linux::process::PidFd` before it was pruned; see
//! `git log` for that removal).
//!
//! `dev_t`/`gid_t`/`mode_t`/`pid_t`/`pthread_t`/`uid_t`/`RawFd`/
//! `RawPthread` are all type aliases to primitives, already covered via
//! `rust_std::primitives` — nothing separate to impl. `XdgDirsIter` is
//! deliberately not covered — likely unstable (very recent addition).

#![cfg(unix)]

use std::os::unix::io::{BorrowedFd, OwnedFd};

use crate::rust_std::macros::{impl_rust_std_type, impl_rust_std_type_lifetime0};

impl_rust_std_type_lifetime0!(
    BorrowedFd,
    "std",
    "std::os::unix::io",
    "https://doc.rust-lang.org/std/os/unix/io/struct.BorrowedFd.html",
    "The BorrowedFd carrier borrows a raw Unix file descriptor without taking ownership of it."
);

impl_rust_std_type!(
    OwnedFd,
    "std",
    "std::os::unix::io",
    "https://doc.rust-lang.org/std/os/unix/io/struct.OwnedFd.html",
    "The OwnedFd carrier owns a raw Unix file descriptor, closing it on drop."
);
