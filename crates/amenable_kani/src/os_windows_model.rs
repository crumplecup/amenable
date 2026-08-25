//! Kani-only accommodation model for `std::os::windows`'s HANDLE/SOCKET
//! carriers (`BorrowedHandle`, `OwnedHandle`, `HandleOrInvalid`,
//! `BorrowedSocket`, `OwnedSocket`) and `EncodeWide`.
//!
//! This module is structurally different from every other accommodation
//! model in this crate, for one reason: **the real types can't be named
//! here at all**. `amenable_kani` must compile on Linux (Kani/CBMC don't
//! run on Windows), and `std::os::windows::io::BorrowedHandle` and friends
//! are `#[cfg(windows)]`-gated in std itself — the type simply doesn't
//! exist in a Linux compilation, not even as dead code. There is no
//! `impl KaniWitness for RustStdStandard<BorrowedHandle<'static>>` anywhere
//! in this crate, and there cannot be one.
//!
//! `amenable_std::rust_std::os_windows` (a cross-platform crate) registers
//! `RustStdType` + evidence for the real types under `#[cfg(windows)]`,
//! mirroring `rust_std::os_unix`'s pattern exactly. A `windows-latest` CI
//! job (`.github/workflows/ci.yml`) validates that registration compiles
//! for real. What's missing is a `kani` witness for it, and the only way to
//! supply one from a Linux-only crate is to prove a property of a *model*
//! of the real type's documented contract, then manually connect that
//! proof to the real type's evidence entry by string, bypassing
//! `KaniWitness`/`bridge_kani_witness!` entirely: `amenable_core::ProofRecord`
//! is just `{ evidence: &'static str, verifier: &'static str, describe: fn()
//! -> String }` (see `amenable_core::link`) — nothing requires `evidence` to
//! have been produced by `stringify!`-ing a type that actually compiled in
//! this crate. Each `ProofRecord` below hand-writes the exact evidence
//! string `amenable_std`'s `#[cfg(windows)]` registration produces, and its
//! `describe` closure builds a `CheckedProof` by hand from a real, run
//! proof of the model, referencing the model's own harness source as
//! `claim` and a `RustStdProvenance` describing the real Windows type it's
//! conditional on.
//!
//! This is the concrete form of "prove it against a model on Linux;
//! reference the model from the real type's evidence; if Windows refines
//! the model, the proof is valid" -- the evidence string is the only place
//! the two sides ever touch. Every contract modeled here is read directly
//! from this project's own pinned toolchain's `std::os::windows` source
//! (`~/.rustup/.../library/std/src/os/windows/io/{handle,socket}.rs`), not
//! guessed:
//!
//! - `RawHandle = *mut c_void` (`std::os::windows::raw`): an opaque,
//!   never-dereferenced OS-assigned identifier. Modeled as a plain `isize`
//!   bit pattern -- nothing about handle semantics depends on pointer
//!   arithmetic or memory layout, and representing it as an actual pointer
//!   would add nothing but risk (see the project rule against `unsafe` in
//!   proofs, even where the real API needs it).
//! - `BorrowedHandle`/`OwnedHandle`: both wrap a raw handle and expose it
//!   unchanged via `.as_raw_handle()`.
//! - `HandleOrInvalid`: wraps a raw handle that may be the sentinel
//!   `INVALID_HANDLE_VALUE` (`-1`); `TryFrom<HandleOrInvalid> for
//!   OwnedHandle` succeeds and preserves the value unless it equals that
//!   exact sentinel (`is_valid()` in `handle.rs`: `self.0 !=
//!   sys::c::INVALID_HANDLE_VALUE`).
//! - `RawSocket = u64` (on the 64-bit targets `windows-latest` runs):
//!   `BorrowedSocket`/`OwnedSocket` wrap one and expose it unchanged via
//!   `.as_raw_socket()`, the same shape as the handle carriers.
//! - `EncodeWide`: encodes an `OsStr` as UTF-16 code units. Modeled for a
//!   single BMP Unicode scalar value (code point `< 0x10000`, no surrogate
//!   pair needed), mirroring the `byte as char` convention `rust_std::str`
//!   already uses elsewhere in this crate: encoding is exactly the code
//!   point cast to `u16`.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::{
    RustLanguageProvenance, RustStdProvenance, RustStdProvenanceBuilder,
    WindowsHandleOrInvalidRejectsOnlyTheSentinel,
};

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// Modeled raw HANDLE value -- see the module doc for why this is a plain
/// `isize`, not a real pointer.
pub type KaniRawHandle = isize;

/// The `INVALID_HANDLE_VALUE` sentinel, per Win32's own definition.
pub const KANI_INVALID_HANDLE_VALUE: KaniRawHandle = -1;

/// Modeled `BorrowedHandle`/`OwnedHandle` semantics: wraps a raw handle
/// value, `.as_raw_handle()` returns exactly what it was constructed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniWindowsHandle(KaniRawHandle);

impl KaniWindowsHandle {
    /// Construct a modeled handle wrapping a raw value.
    #[must_use]
    pub fn new(value: KaniRawHandle) -> Self {
        Self(value)
    }

    /// Model `.as_raw_handle()`.
    #[must_use]
    pub fn as_raw_handle(&self) -> KaniRawHandle {
        self.0
    }
}

/// Modeled `InvalidHandleError`: reports that a `HandleOrInvalid` could not
/// convert because it held exactly the `INVALID_HANDLE_VALUE` sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniWindowsInvalidHandleError(());

/// Modeled `HandleOrInvalid` semantics: wraps a raw handle that may be the
/// `INVALID_HANDLE_VALUE` sentinel, deferring that check to conversion time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniWindowsHandleOrInvalid(KaniRawHandle);

impl KaniWindowsHandleOrInvalid {
    /// Construct a modeled `HandleOrInvalid` wrapping a raw value.
    #[must_use]
    pub fn new(value: KaniRawHandle) -> Self {
        Self(value)
    }

    /// Model `TryFrom<HandleOrInvalid> for OwnedHandle`: succeeds and
    /// preserves the value unless it's exactly `INVALID_HANDLE_VALUE`.
    pub fn try_into_owned(self) -> Result<KaniWindowsHandle, KaniWindowsInvalidHandleError> {
        if self.0 == KANI_INVALID_HANDLE_VALUE {
            Err(KaniWindowsInvalidHandleError(()))
        } else {
            Ok(KaniWindowsHandle(self.0))
        }
    }
}

/// Modeled `BorrowedSocket`/`OwnedSocket` semantics: wraps a raw socket
/// value, `.as_raw_socket()` returns exactly what it was constructed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniWindowsSocket(u64);

impl KaniWindowsSocket {
    /// Construct a modeled socket wrapping a raw value.
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Model `.as_raw_socket()`.
    #[must_use]
    pub fn as_raw_socket(&self) -> u64 {
        self.0
    }
}

/// Modeled `EncodeWide` semantics for a single BMP Unicode scalar value:
/// encodes to exactly one `u16` equal to the code point.
#[must_use]
pub fn kani_encode_wide_bmp_char(c: char) -> u16 {
    u16::try_from(u32::from(c)).expect("c is a BMP scalar value by this fn's own precondition")
}

/// Build the `RustStdProvenance` for a real, `#[cfg(windows)]`-only
/// `std::os::windows` carrier that this crate can't name directly -- see
/// the module doc for why. Mirrors exactly what `amenable_std`'s own
/// `#[cfg(windows)]` registration for the same type produces.
fn windows_provenance(
    source_module: &str,
    url: &str,
    type_name: &str,
    summary: &str,
) -> RustStdProvenance {
    RustStdProvenanceBuilder::default()
        .rust(RustLanguageProvenance::for_source("std", source_module))
        .source_url(url)
        .type_name(type_name)
        .semantic_summary(summary)
        .build()
        .expect("all fields set")
}

amenable_derive::harness! {
    kani, VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `BorrowedHandle`/`OwnedHandle` both expose exactly the raw
        /// handle value they were constructed with, unchanged.
        #[kani::proof]
        fn verify_windows_handle_as_raw_handle_recovers_the_wrapped_value() {
            let value: isize = kani::any();
            let handle = crate::KaniWindowsHandle::new(value);
            assert_eq!(handle.as_raw_handle(), value, "as_raw_handle recovers the wrapped value");
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "kani",
        || CheckedProof {
            harness: "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value".to_owned(),
            claim: VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html",
                "std::os::windows::io::BorrowedHandle<'static>",
                "The BorrowedHandle carrier borrows a raw Windows HANDLE without taking ownership of it.",
            ),
        }
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "kani",
        || CheckedProof {
            harness: "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value".to_owned(),
            claim: VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html",
                "std::os::windows::io::OwnedHandle",
                "The OwnedHandle carrier owns a raw Windows HANDLE, closing it on drop.",
            ),
        }
        .to_string(),
    )
}

/// [`WindowsHandleOrInvalidRejectsOnlyTheSentinel`] reuses the same
/// harness rather than adding a new Kani proof — it names the
/// postcondition this model's one real proof already checks, it doesn't
/// prove anything new.
impl KaniWitness for WindowsHandleOrInvalidRejectsOnlyTheSentinel {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_windows_handle_or_invalid_rejects_only_the_sentinel".to_owned(),
            claim: VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(WindowsHandleOrInvalidRejectsOnlyTheSentinel);

kani_ensures!(
    WindowsHandleOrInvalidRejectsOnlyTheSentinel,
    "amenable_std::WindowsHandleOrInvalidRejectsOnlyTheSentinel",
    isize,
    |value| crate::KaniWindowsHandleOrInvalid::new(value)
        .try_into_owned()
        .is_err()
);

amenable_derive::harness! {
    kani, VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC, {
        /// `HandleOrInvalid` converts to an owned handle, preserving the
        /// wrapped value, unless that value is exactly the
        /// `INVALID_HANDLE_VALUE` sentinel (`-1`), in which case
        /// conversion fails. The assertion calls
        /// `WindowsHandleOrInvalidRejectsOnlyTheSentinel::ensures`
        /// directly rather than restating the comparison.
        #[kani::proof]
        fn verify_windows_handle_or_invalid_rejects_only_the_sentinel() {
            let value: isize = kani::any();
            let wrapped = crate::KaniWindowsHandleOrInvalid::new(value);
            let result = wrapped.try_into_owned();
            if value == crate::KANI_INVALID_HANDLE_VALUE {
                assert!(
                    WindowsHandleOrInvalidRejectsOnlyTheSentinel::ensures(value),
                    "the sentinel value is rejected"
                );
            } else {
                assert_eq!(
                    result,
                    Ok(crate::KaniWindowsHandle::new(value)),
                    "any non-sentinel value converts, preserving itself"
                );
            }
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "kani",
        || CheckedProof {
            harness: "verify_windows_handle_or_invalid_rejects_only_the_sentinel".to_owned(),
            claim: VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html",
                "std::os::windows::io::HandleOrInvalid",
                "The HandleOrInvalid carrier owns a Windows HANDLE that may be the sentinel INVALID_HANDLE_VALUE, deferring that check to conversion time.",
            ),
        }
        .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `BorrowedSocket`/`OwnedSocket` both expose exactly the raw
        /// socket value they were constructed with, unchanged.
        #[kani::proof]
        fn verify_windows_socket_as_raw_socket_recovers_the_wrapped_value() {
            let value: u64 = kani::any();
            let socket = crate::KaniWindowsSocket::new(value);
            assert_eq!(socket.as_raw_socket(), value, "as_raw_socket recovers the wrapped value");
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "kani",
        || CheckedProof {
            harness: "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value".to_owned(),
            claim: VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html",
                "std::os::windows::io::BorrowedSocket<'static>",
                "The BorrowedSocket carrier borrows a raw Windows SOCKET without taking ownership of it.",
            ),
        }
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "kani",
        || CheckedProof {
            harness: "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value".to_owned(),
            claim: VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedSocket.html",
                "std::os::windows::io::OwnedSocket",
                "The OwnedSocket carrier owns a raw Windows SOCKET, closing it on drop.",
            ),
        }
        .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CHAR_AS_ONE_CODE_UNIT_SRC, {
        /// `EncodeWide` encodes a Basic Multilingual Plane character (code
        /// point below the surrogate range, needing no surrogate pair) as
        /// exactly one UTF-16 code unit equal to its code point.
        #[kani::proof]
        fn verify_encode_wide_encodes_a_bmp_char_as_one_code_unit() {
            let c: char = kani::any();
            kani::assume((c as u32) < 0x10000);
            assert_eq!(
                crate::kani_encode_wide_bmp_char(c),
                c as u32 as u16,
                "a BMP character encodes to one code unit equal to its code point"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "kani",
        || CheckedProof {
            harness: "verify_encode_wide_encodes_a_bmp_char_as_one_code_unit".to_owned(),
            claim: VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CHAR_AS_ONE_CODE_UNIT_SRC.to_owned(),
            provenance: windows_provenance(
                "std::os::windows::ffi",
                "https://doc.rust-lang.org/std/os/windows/ffi/struct.EncodeWide.html",
                "std::os::windows::ffi::EncodeWide<'static>",
                "The EncodeWide carrier lazily encodes an OsStr as UTF-16 code units, as Windows APIs expect.",
            ),
        }
        .to_string(),
    )
}
