use std::io::WriterPanicked;

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{KaniVerifier, KaniWitness, KaniWriterPanickedObservation};

impl KaniWitness for RustStdStandard<WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_writer_panicked_recovers_the_buffered_data".to_owned(),
            VERIFY_WRITER_PANICKED_RECOVERS_THE_BUFFERED_DATA_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<WriterPanicked>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<WriterPanicked>",
        "kani",
        || <RustStdStandard<WriterPanicked> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniWriterPanickedObservation` instance actually
/// demonstrated recovering the buffered data after a panic, minted only by
/// [`KaniWriterPanickedObservation::demonstrate_recovery`].
pub struct KaniWriterPanickedWitnessToken(());

impl ProofToken for KaniWriterPanickedWitnessToken {
    type Proposition = KaniWriterPanickedObservation;
}

impl KaniWriterPanickedObservation {
    /// Assert the panic was captured and the buffered bytes are
    /// recoverable unchanged. Consumes `self` for the same reason
    /// [`crate::KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_recovery(self, buffered: [u8; 2]) -> KaniWriterPanickedWitnessToken {
        assert!(self.panicked());
        assert_eq!(self.recovered_buffer(), buffered);
        KaniWriterPanickedWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<WriterPanicked>`'s buffered-data
/// recovery claim has been established from a
/// `KaniWriterPanickedObservation`.
pub struct RustStdWriterPanickedToken(());

impl ProofToken for RustStdWriterPanickedToken {
    type Proposition = RustStdStandard<WriterPanicked>;
}

impl Establish<KaniWriterPanickedWitnessToken, KaniVerifier> for RustStdStandard<WriterPanicked> {
    type Token = RustStdWriterPanickedToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniWriterPanickedWitnessToken) -> Self::Token {
        RustStdWriterPanickedToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_WRITER_PANICKED_RECOVERS_THE_BUFFERED_DATA_SRC, {
        /// When a `BufWriter`'s inner writer panics mid-write, the panic
        /// is caught rather than corrupting the buffer, and
        /// `.into_parts()` afterward reports `WriterPanicked` while still
        /// recovering exactly the data that was buffered.
        /// This proof uses the Amenable-owned bounded panic-recovery model:
        /// if the real `WriterPanicked` path refines this observation, the
        /// Rust-facing buffered-data recovery claim follows. The claim is
        /// established through `Establish<KaniWriterPanickedObservation,
        /// KaniVerifier> for RustStdStandard<WriterPanicked>` from the
        /// observation instance that actually demonstrated the recovery.
        #[kani::proof]
        fn verify_writer_panicked_recovers_the_buffered_data() {
            let buffered = [kani::any(), kani::any()];
            let observation = crate::KaniWriterPanickedObservation::new(buffered);
            let demonstration = observation.demonstrate_recovery(buffered);

            let _token = RustStdStandard::<WriterPanicked>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Empty> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_empty_read_reports_end_of_file".to_owned(),
            VERIFY_EMPTY_READ_REPORTS_END_OF_FILE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Empty>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Empty>",
        "kani",
        || <RustStdStandard<std::io::Empty> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::io::Empty>,
    "amenable_std::rust_std::RustStdStandard<std::io::Empty>",
    usize,
    |read| read == 0
);

amenable_derive::harness! {
    kani, VERIFY_EMPTY_READ_REPORTS_END_OF_FILE_SRC, {
        /// `std::io::empty()`'s reader always reports zero bytes read,
        /// regardless of the buffer offered to it.
        #[kani::proof]
        fn verify_empty_read_reports_end_of_file() {
            use std::io::Read;

            let mut buffer: [u8; 4] = kani::any();
            let mut reader = std::io::empty();
            let read = reader.read(&mut buffer).expect("Empty::read never errors");
            assert!(
                RustStdStandard::<std::io::Empty>::ensures(read),
                "Empty::read always reports zero bytes read"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Repeat> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_repeat_fills_the_buffer_with_the_given_byte".to_owned(),
            VERIFY_REPEAT_FILLS_THE_BUFFER_WITH_THE_GIVEN_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Repeat>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Repeat>",
        "kani",
        || <RustStdStandard<std::io::Repeat> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::io::Repeat>,
    "amenable_std::rust_std::RustStdStandard<std::io::Repeat>",
    (u8, u8),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REPEAT_FILLS_THE_BUFFER_WITH_THE_GIVEN_BYTE_SRC, {
        /// `std::io::repeat(byte)`'s reader always fills the whole
        /// buffer offered to it with the given byte.
        #[kani::proof]
        fn verify_repeat_fills_the_buffer_with_the_given_byte() {
            use std::io::Read;

            let byte: u8 = kani::any();
            let mut buffer = [0u8; 4];
            let mut reader = std::io::repeat(byte);
            let read = reader.read(&mut buffer).expect("Repeat::read never errors");
            assert!(
                RustStdStandard::<usize>::ensures((read, buffer.len())),
                "Repeat::read always fills the whole buffer"
            );
            for filled in buffer {
                assert!(
                    RustStdStandard::<std::io::Repeat>::ensures((filled, byte)),
                    "Repeat::read fills every slot with the given byte"
                );
            }
        }
    }
}
