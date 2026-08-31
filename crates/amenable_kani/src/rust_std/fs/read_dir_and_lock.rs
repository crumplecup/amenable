use std::fs::ReadDir;

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniLockObservation, KaniReadDirObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<ReadDir> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_read_dir_iterates_every_entry_in_the_directory".to_owned(),
            VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ReadDir>",
        "kani",
        || <RustStdStandard<ReadDir> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniReadDirObservation` instance actually demonstrated
/// exactly the created entries, minted only by
/// [`KaniReadDirObservation::demonstrate_completeness`].
pub struct KaniReadDirWitnessToken(());

impl ProofToken for KaniReadDirWitnessToken {
    type Proposition = KaniReadDirObservation;
}

impl KaniReadDirObservation {
    /// Assert `.entries()` yields exactly the two created entries, in
    /// order. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, first_name, second_name))
    )]
    #[must_use]
    pub fn demonstrate_completeness(
        self,
        first_name: crate::KaniFsLabel,
        second_name: crate::KaniFsLabel,
    ) -> KaniReadDirWitnessToken {
        let entries = self.entries();
        assert_eq!(
            entries.len(),
            2,
            "read_dir yields exactly the created entries"
        );
        assert_eq!(entries[0].file_name(), Some(first_name));
        assert_eq!(entries[1].file_name(), Some(second_name));
        KaniReadDirWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ReadDir>`'s entry-completeness
/// claim has been established from a `KaniReadDirObservation` that has
/// itself demonstrated exactly the created entries.
pub struct RustStdReadDirToken(());

impl ProofToken for RustStdReadDirToken {
    type Proposition = RustStdStandard<ReadDir>;
}

impl Establish<KaniReadDirWitnessToken, KaniVerifier> for RustStdStandard<ReadDir> {
    type Token = RustStdReadDirToken;

    fn establish(_credential: KaniReadDirWitnessToken) -> Self::Token {
        RustStdReadDirToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC, {
        /// `.read_dir()` yields exactly the files that were created in
        /// that directory, no more and no fewer.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` path preserves entry identity this way, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniReadDirObservation, KaniVerifier> for
        /// RustStdStandard<ReadDir>` from the observation instance that
        /// actually demonstrated the entry completeness, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_read_dir_iterates_every_entry_in_the_directory() {
            let base = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('d'));
            let one = crate::KaniFsLabel::new('1');
            let two = crate::KaniFsLabel::new('2');
            let observation = crate::KaniReadDirObservation::new(base, one, two);
            let demonstration = observation.demonstrate_completeness(one, two);

            let _token = RustStdStandard::<ReadDir>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_lock_error_reports_a_lock_already_held".to_owned(),
            VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        "kani",
        || {
            <RustStdStandard<std::fs::TryLockError> as KaniWitness>::proof().to_string()
        },
    )
}

/// Witness that a `KaniLockObservation` instance actually demonstrated a
/// second handle failing to acquire an already-held lock, minted only by
/// [`KaniLockObservation::demonstrate_exclusion`].
pub struct KaniLockWitnessToken(());

impl ProofToken for KaniLockWitnessToken {
    type Proposition = KaniLockObservation;
}

impl KaniLockObservation {
    /// Assert the first `.try_lock()` succeeds and a second fails while
    /// the lock is still held. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_exclusion(mut self) -> KaniLockWitnessToken {
        assert!(
            self.try_lock().is_ok(),
            "the first handle acquires the lock"
        );

        assert!(
            self.try_lock().is_err(),
            "a second handle can't also lock the same file"
        );
        KaniLockWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::fs::TryLockError>`'s
/// mutual-exclusion claim has been established from a
/// `KaniLockObservation` that has itself demonstrated a second handle
/// failing to acquire an already-held lock.
pub struct RustStdTryLockErrorToken(());

impl ProofToken for RustStdTryLockErrorToken {
    type Proposition = RustStdStandard<std::fs::TryLockError>;
}

impl Establish<KaniLockWitnessToken, KaniVerifier> for RustStdStandard<std::fs::TryLockError> {
    type Token = RustStdTryLockErrorToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniLockWitnessToken) -> Self::Token {
        RustStdTryLockErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC, {
        /// A second handle's `.try_lock()` fails while the first handle
        /// still holds the file lock.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::try_lock` path preserves this exclusion, the Rust-facing
        /// claim follows. The claim is established through
        /// `Establish<KaniLockObservation, KaniVerifier> for
        /// RustStdStandard<std::fs::TryLockError>` from the observation
        /// instance that actually demonstrated the exclusion, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_try_lock_error_reports_a_lock_already_held() {
            let file = crate::KaniLockObservation::new();
            let demonstration = file.demonstrate_exclusion();

            let _token = RustStdStandard::<std::fs::TryLockError>::establish(demonstration);
        }
    }
}
