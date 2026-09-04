use std::fs::{File, FileTimes};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniFileContentObservation, KaniFileTimesObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<File> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_file_write_then_read_round_trips_the_bytes".to_owned(),
            VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<File>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<File>",
        "kani",
        || <RustStdStandard<File> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileContentObservation` instance actually
/// demonstrated its byte-preserving round trip, minted only by
/// [`KaniFileContentObservation::demonstrate_round_trip`].
pub struct KaniFileContentWitnessToken(());

impl ProofToken for KaniFileContentWitnessToken {
    type Proposition = KaniFileContentObservation;
}

impl KaniFileContentObservation {
    /// Assert the bytes read back match what was written. Consumes
    /// `self` for the same reason
    /// [`crate::KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, bytes)))]
    #[must_use]
    pub fn demonstrate_round_trip(self, bytes: [u8; 4]) -> KaniFileContentWitnessToken {
        assert_eq!(
            self.read(),
            bytes,
            "bytes written to a file are read back unchanged through a fresh handle"
        );
        KaniFileContentWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<File>`'s write/read round-trip
/// claim has been established from a `KaniFileContentObservation` that has
/// itself demonstrated the byte-preserving round trip.
pub struct RustStdFileContentToken(());

impl ProofToken for RustStdFileContentToken {
    type Proposition = RustStdStandard<File>;
}

impl Establish<KaniFileContentWitnessToken, KaniVerifier> for RustStdStandard<File> {
    type Token = RustStdFileContentToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniFileContentWitnessToken) -> Self::Token {
        RustStdFileContentToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC, {
        /// Bytes written to a file and flushed by `Drop` are read back
        /// unchanged through a fresh handle.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::write_all` / re-open path preserves written bytes this
        /// way, the Rust-facing claim follows. The claim is established
        /// through `Establish<KaniFileContentObservation, KaniVerifier> for
        /// RustStdStandard<File>` from the observation instance that
        /// actually demonstrated the round trip, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_file_write_then_read_round_trips_the_bytes() {
            let bytes: [u8; 4] = kani::any();
            let observation = crate::KaniFileContentObservation::write(bytes);
            let demonstration = observation.demonstrate_round_trip(bytes);

            let _token = RustStdStandard::<File>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<FileTimes> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_file_times_sets_the_recorded_modification_time".to_owned(),
            VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FileTimes>",
        "kani",
        || <RustStdStandard<FileTimes> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileTimesObservation` instance actually
/// demonstrated the exact time reflection, minted only by
/// [`KaniFileTimesObservation::demonstrate_modification_time`].
pub struct KaniFileTimesWitnessToken(());

impl ProofToken for KaniFileTimesWitnessToken {
    type Proposition = KaniFileTimesObservation;
}

impl KaniFileTimesObservation {
    /// Assert the recorded modification time matches the target exactly.
    /// Consumes `self` for the same reason
    /// [`crate::KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_modification_time(
        self,
        target_unix_seconds: u64,
    ) -> KaniFileTimesWitnessToken {
        assert_eq!(
            self.modified(),
            target_unix_seconds,
            "the target modification time is reflected exactly in metadata"
        );
        KaniFileTimesWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<FileTimes>`'s modification-time
/// claim has been established from a `KaniFileTimesObservation` that has
/// itself demonstrated the exact time reflection.
pub struct RustStdFileTimesToken(());

impl ProofToken for RustStdFileTimesToken {
    type Proposition = RustStdStandard<FileTimes>;
}

impl Establish<KaniFileTimesWitnessToken, KaniVerifier> for RustStdStandard<FileTimes> {
    type Token = RustStdFileTimesToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniFileTimesWitnessToken) -> Self::Token {
        RustStdFileTimesToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC, {
        /// `.set_modified()` on a `FileTimes`, applied via
        /// `File::set_times()`, is reflected exactly in the file's
        /// metadata. The target time is whole-second-precision, since
        /// some filesystems truncate sub-second components.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_times` / `metadata().modified()` path preserves the target
        /// time this way, the Rust-facing claim follows. The claim is
        /// established through `Establish<KaniFileTimesObservation,
        /// KaniVerifier> for RustStdStandard<FileTimes>` from the
        /// observation instance that actually demonstrated the reflection,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_file_times_sets_the_recorded_modification_time() {
            let target_unix_seconds: u64 = kani::any();
            let observation = crate::KaniFileTimesObservation::set_modified(target_unix_seconds);
            let demonstration = observation.demonstrate_modification_time(target_unix_seconds);

            let _token = RustStdStandard::<FileTimes>::establish(demonstration);
        }
    }
}
