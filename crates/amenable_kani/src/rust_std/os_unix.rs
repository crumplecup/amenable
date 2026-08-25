//! `KaniWitness` impls for `std::os::unix`.
//!
//! `#[cfg(unix)]`-gated, mirroring `amenable_std::rust_std::os_unix` —
//! compiles as a no-op on Windows (this crate's primary development
//! host). The direct `BorrowedFd::try_clone_to_owned()` path is preserved in
//! the proof gallery as an unsupported `fcntl` boundary. The production
//! `OwnedFd` proof therefore uses an Amenable-owned accommodation model:
//! if the real std/libc path refines the modeled borrow / duplicate / transfer
//! laws, then the proof result carries over to the Rust-facing claim.

#![cfg(unix)]

use std::os::unix::io::{BorrowedFd, OwnedFd};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::NonNegativeFd;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<BorrowedFd<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_borrowed_fd_reports_the_same_raw_value_as_the_owner".to_owned(),
            VERIFY_BORROWED_FD_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BorrowedFd<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedFd<'static>>",
        "kani",
        || <RustStdStandard<BorrowedFd<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BORROWED_FD_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC, {
        /// Borrowing an already-open Unix stream's descriptor reports
        /// exactly the same raw value the owner itself reports, and a
        /// live descriptor is always non-negative. The second assertion
        /// calls `NonNegativeFd::ensures` directly rather than restating
        /// the comparison.
        #[kani::proof]
        fn verify_borrowed_fd_reports_the_same_raw_value_as_the_owner() {
            use std::os::unix::io::{AsFd, AsRawFd};

            let stdout = std::io::stdout();
            let borrowed = stdout.as_fd();
            assert_eq!(borrowed.as_raw_fd(), stdout.as_raw_fd());
            assert!(
                NonNegativeFd::ensures(stdout.as_raw_fd()),
                "a live fd is never negative"
            );
        }
    }
}

/// The [`NonNegativeFd`] contract type reuses
/// `verify_borrowed_fd_reports_the_same_raw_value_as_the_owner` rather than
/// adding a new Kani harness — it names the bound the harness already
/// checks, it doesn't prove anything new.
impl KaniWitness for NonNegativeFd {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_borrowed_fd_reports_the_same_raw_value_as_the_owner".to_owned(),
            VERIFY_BORROWED_FD_REPORTS_THE_SAME_RAW_VALUE_AS_THE_OWNER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(NonNegativeFd);

kani_ensures!(
    NonNegativeFd,
    "amenable_kani::NonNegativeFd",
    i32,
    |value| value >= 0
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::NonNegativeFd",
        "kani",
        || <NonNegativeFd as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<OwnedFd> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_owned_fd_preserves_the_raw_value_across_conversion".to_owned(),
            VERIFY_OWNED_FD_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OwnedFd>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedFd>",
        "kani",
        || <RustStdStandard<OwnedFd> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OWNED_FD_PRESERVES_THE_RAW_VALUE_ACROSS_CONVERSION_SRC, {
        /// Modeled accommodation: if the real borrowed-fd duplication and
        /// `File` → `OwnedFd` transfer path refines `KaniFd` / `KaniFile`,
        /// then converting a `File` into an `OwnedFd` preserves the raw fd
        /// value and resource identity without reopening the underlying
        /// resource.
        #[kani::proof]
        fn verify_owned_fd_preserves_the_raw_value_across_conversion() {
            let seed_owned = crate::KaniFd::kani_live_any();
            let borrowed = seed_owned.borrow();
            let duplicated_raw_fd: u16 = kani::any();
            let duplicated_owned = borrowed.duplicate_as_owned_with_raw(i32::from(duplicated_raw_fd));
            let file = crate::KaniFile::from_owned_fd(duplicated_owned);
            let raw_before = file.raw_fd();
            let resource_before = file.resource_id();

            let owned = file.into_owned_fd();

            assert_eq!(owned.raw_fd(), raw_before);
            assert_eq!(owned.resource_id(), resource_before);
            assert!(owned.is_live(), "modeled ownership transfer keeps the fd live");
        }
    }
}
