//! `KaniWitness` impls for `std::os::unix`.
//!
//! `#[cfg(unix)]`-gated, mirroring `amenable_std::rust_std::os_unix` —
//! compiles as a no-op on Windows (this crate's primary development
//! host). Real, checked harnesses, same as `os_windows` — but unlike
//! there, this host cannot execute Unix code at all (no working WSL
//! distribution, no linker for a Unix cross-target), so the runtime
//! claim below was never actually run here, only cross-target
//! type-checked via `cargo check -p amenable_kani --target
//! x86_64-unknown-linux-gnu`. The claim itself (a raw fd survives an
//! ownership-transferring conversion unchanged, since `From<File> for
//! OwnedFd` moves the same fd rather than reopening it) mirrors exactly
//! what was scratch-verified for `OwnedHandle` on Windows, and rests on
//! long-established, exhaustively documented POSIX/io-safety
//! guarantees. Run `cargo kani` against this harness on a real Unix
//! host to confirm it directly.

#![cfg(unix)]

use std::os::unix::io::{BorrowedFd, OwnedFd};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<BorrowedFd<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_borrowed_fd_reports_the_same_raw_value_as_the_owner",
            claim: VERIFY_BORROWED_FD_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BorrowedFd<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BorrowedFd<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<BorrowedFd<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BORROWED_FD_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC, {
        /// Borrowing a real file's descriptor reports exactly the same
        /// raw value the file itself reports, and a valid descriptor is
        /// always non-negative (the POSIX contract for a successfully
        /// opened fd).
        #[kani::proof]
        fn verify_borrowed_fd_reports_the_same_raw_value_as_the_owner() {
            use std::os::unix::io::{AsFd, AsRawFd};

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_os_unix_fd_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file = std::fs::File::create(base.join("data.txt")).unwrap();

            let borrowed = file.as_fd();
            assert_eq!(borrowed.as_raw_fd(), file.as_raw_fd());
            assert!(file.as_raw_fd() >= 0, "a valid fd is never negative");

            drop(file);
            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}

impl KaniWitness for RustStdStandard<OwnedFd> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_owned_fd_preserves_the_raw_value_across_conversion",
            claim: VERIFY_OWNED_FD_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OwnedFd>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OwnedFd>",
        verifier: "kani",
        describe: || <RustStdStandard<OwnedFd> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OWNED_FD_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC, {
        /// Converting a `File` into an `OwnedFd` transfers ownership of
        /// the very same descriptor — no re-opening happens under the
        /// hood, so the raw value is identical before and after.
        #[kani::proof]
        fn verify_owned_fd_preserves_the_raw_value_across_conversion() {
            use std::os::unix::io::AsRawFd;

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_os_unix_owned_fd_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file = std::fs::File::create(base.join("data.txt")).unwrap();

            let raw_before = file.as_raw_fd();
            let owned: OwnedFd = file.into();
            assert_eq!(owned.as_raw_fd(), raw_before);

            drop(owned);
            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}
