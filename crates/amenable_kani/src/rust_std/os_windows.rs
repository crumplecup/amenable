//! `KaniWitness` impls for `std::os::windows`.
//!
//! `#[cfg(windows)]`-gated, mirroring `amenable_std::rust_std::os_windows`
//! — compiles as a no-op on every other platform. Verified empirically on
//! this machine (Windows), unlike `os_unix`.

#![cfg(windows)]

use std::os::windows::ffi::EncodeWide;
use std::os::windows::io::{
    BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<EncodeWide<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_encode_wide_maps_ascii_to_matching_utf16_units".to_owned(),
            VERIFY_ENCODE_WIDE_MAPS_ASCII_TO_MATCHING_UTF16_UNITS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<EncodeWide<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "kani",
        || <RustStdStandard<EncodeWide<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ENCODE_WIDE_MAPS_ASCII_TO_MATCHING_UTF16_UNITS_SRC, {
        /// ASCII text encodes to UTF-16 code units with the same
        /// numeric values, one per character.
        #[kani::proof]
        fn verify_encode_wide_maps_ascii_to_matching_utf16_units() {
            use std::os::windows::ffi::OsStrExt;

            let wide: Vec<u16> = std::ffi::OsStr::new("hi").encode_wide().collect();
            assert!(CollectedSequenceMatchesExpected::ensures((wide, vec![0x68, 0x69])));
        }
    }
}

impl KaniWitness for RustStdStandard<BorrowedHandle<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_borrowed_handle_reports_the_same_raw_value_as_the_owner".to_owned(),
            VERIFY_BORROWED_HANDLE_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BorrowedHandle<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "kani",
        || <RustStdStandard<BorrowedHandle<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BORROWED_HANDLE_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC, {
        /// Borrowing a real file's handle reports exactly the same raw
        /// value the file itself reports.
        #[kani::proof]
        fn verify_borrowed_handle_reports_the_same_raw_value_as_the_owner() {
            use std::os::windows::io::{AsHandle, AsRawHandle};

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_os_windows_handle_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file = std::fs::File::create(base.join("data.txt")).unwrap();

            let borrowed = file.as_handle();
            assert_eq!(borrowed.as_raw_handle(), file.as_raw_handle());

            drop(file);
            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}

impl KaniWitness for RustStdStandard<BorrowedSocket<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_borrowed_socket_reports_the_same_raw_value_as_the_owner".to_owned(),
            VERIFY_BORROWED_SOCKET_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BorrowedSocket<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "kani",
        || <RustStdStandard<BorrowedSocket<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BORROWED_SOCKET_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC, {
        /// Borrowing a real socket's handle reports exactly the same
        /// raw value the socket itself reports.
        #[kani::proof]
        fn verify_borrowed_socket_reports_the_same_raw_value_as_the_owner() {
            use std::os::windows::io::{AsRawSocket, AsSocket};

            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let borrowed = listener.as_socket();
            assert_eq!(borrowed.as_raw_socket(), listener.as_raw_socket());
        }
    }
}

impl KaniWitness for RustStdStandard<HandleOrInvalid> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_handle_or_invalid_distinguishes_a_real_handle_from_the_sentinel".to_owned(),
            VERIFY_HANDLE_OR_INVALID_DISTINGUISHES_A_REAL_HANDLE_FROM_THE_SENTINEL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<HandleOrInvalid>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "kani",
        || <RustStdStandard<HandleOrInvalid> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_HANDLE_OR_INVALID_DISTINGUISHES_A_REAL_HANDLE_FROM_THE_SENTINEL_SRC, {
        /// A `HandleOrInvalid` built from a real, open handle converts
        /// cleanly into an `OwnedHandle`; one built from the sentinel
        /// `INVALID_HANDLE_VALUE` (`-1`) does not.
        #[kani::proof]
        fn verify_handle_or_invalid_distinguishes_a_real_handle_from_the_sentinel() {
            use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_os_windows_handle_or_invalid_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file = std::fs::File::create(base.join("data.txt")).unwrap();
            let raw = file.as_raw_handle();

            let real = unsafe { HandleOrInvalid::from_raw_handle(raw) };
            let converted: Result<OwnedHandle, _> = real.try_into();
            assert!(converted.is_ok());

            let sentinel = unsafe { HandleOrInvalid::from_raw_handle(-1isize as RawHandle) };
            let converted_sentinel: Result<OwnedHandle, _> = sentinel.try_into();
            assert!(FallibleOperationReportsFailure::ensures(converted_sentinel.is_err()));

            drop(file);
            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}

impl KaniWitness for RustStdStandard<OwnedHandle> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_owned_handle_preserves_the_raw_value_across_conversion".to_owned(),
            VERIFY_OWNED_HANDLE_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OwnedHandle>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "kani",
        || <RustStdStandard<OwnedHandle> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OWNED_HANDLE_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC, {
        /// Converting a `File` into an `OwnedHandle` transfers ownership
        /// of the very same handle — no re-opening happens under the
        /// hood, so the raw value is identical before and after.
        #[kani::proof]
        fn verify_owned_handle_preserves_the_raw_value_across_conversion() {
            use std::os::windows::io::AsRawHandle;

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_os_windows_owned_handle_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file = std::fs::File::create(base.join("data.txt")).unwrap();

            let raw_before = file.as_raw_handle();
            let owned: OwnedHandle = file.into();
            assert_eq!(owned.as_raw_handle(), raw_before);

            drop(owned);
            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}

impl KaniWitness for RustStdStandard<OwnedSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_owned_socket_preserves_the_raw_value_across_conversion".to_owned(),
            VERIFY_OWNED_SOCKET_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OwnedSocket>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "kani",
        || <RustStdStandard<OwnedSocket> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OWNED_SOCKET_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC, {
        /// Same preservation claim as `OwnedHandle`, for a real TCP
        /// socket converted into an `OwnedSocket`.
        #[kani::proof]
        fn verify_owned_socket_preserves_the_raw_value_across_conversion() {
            use std::os::windows::io::AsRawSocket;

            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let raw_before = listener.as_raw_socket();
            let owned: OwnedSocket = listener.into();
            assert_eq!(owned.as_raw_socket(), raw_before);
        }
    }
}
