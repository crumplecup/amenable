use std::io::{BufReader, BufWriter, IntoInnerError};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::EmptiedContainerReportsEmpty;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniBufferedReadObservation, KaniFlushErrorObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_buf_reader_reads_the_underlying_bytes".to_owned(),
            VERIFY_BUF_READER_READS_THE_UNDERLYING_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BufReader<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BufReader<&'static [u8]>>",
        "kani",
        || <RustStdStandard<BufReader<&'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniBufferedReadObservation` instance actually
/// demonstrated its read-through, minted only by
/// [`KaniBufferedReadObservation::demonstrate_read_through`].
pub struct KaniBufferedReadWitnessToken(());

impl ProofToken for KaniBufferedReadWitnessToken {
    type Proposition = KaniBufferedReadObservation;
}

impl KaniBufferedReadObservation {
    /// Assert `.read_to_end()` reads through to exactly the wrapped
    /// bytes. Consumes `self`: the only way to obtain the token is to
    /// have run this check against a real observation instance, not to
    /// assert it independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, payload)))]
    #[must_use]
    pub fn demonstrate_read_through(self, payload: [u8; 2]) -> KaniBufferedReadWitnessToken {
        assert_eq!(self.read_to_end(), payload);
        KaniBufferedReadWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<BufReader<&'static [u8]>>`'s
/// read-through claim has been established from a
/// `KaniBufferedReadObservation`.
pub struct RustStdBufReaderToken(());

impl ProofToken for RustStdBufReaderToken {
    type Proposition = RustStdStandard<BufReader<&'static [u8]>>;
}

impl Establish<KaniBufferedReadWitnessToken, KaniVerifier>
    for RustStdStandard<BufReader<&'static [u8]>>
{
    type Token = RustStdBufReaderToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniBufferedReadWitnessToken) -> Self::Token {
        RustStdBufReaderToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_BUF_READER_READS_THE_UNDERLYING_BYTES_SRC, {
        /// A `BufReader` reads through to exactly the bytes of the reader
        /// it wraps.
        /// This proof uses the Amenable-owned bounded buffered-read model:
        /// if the real `BufReader` path refines this observation, the
        /// Rust-facing read-through claim follows. The claim is established
        /// through `Establish<KaniBufferedReadObservation, KaniVerifier> for
        /// RustStdStandard<BufReader<&'static [u8]>>` from the observation
        /// instance that actually demonstrated the read-through.
        #[kani::proof]
        fn verify_buf_reader_reads_the_underlying_bytes() {
            let payload = [kani::any(), kani::any()];
            let observation = crate::KaniBufferedReadObservation::new(payload);
            let demonstration = observation.demonstrate_read_through(payload);

            let _token = RustStdStandard::<BufReader<&'static [u8]>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<BufWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_buf_writer_flushes_to_the_underlying_writer".to_owned(),
            VERIFY_BUF_WRITER_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BufWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BufWriter<Vec<u8>>>",
        "kani",
        || <RustStdStandard<BufWriter<Vec<u8>>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BUF_WRITER_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC, {
        /// Written bytes reach the underlying writer once flushed.
        #[kani::proof]
        fn verify_buf_writer_flushes_to_the_underlying_writer() {
            use std::io::Write;

            let mut writer = BufWriter::new(Vec::new());
            writer.write_all(b"hello").unwrap();
            assert!(
                EmptiedContainerReportsEmpty::ensures(writer.get_ref().is_empty()),
                "a small write remains buffered before flush"
            );
            writer.flush().unwrap();
            assert!(CollectedSequenceMatchesExpected::ensures((
                writer.into_inner().unwrap(),
                b"hello".to_vec()
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Bytes<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bytes_yields_one_byte_at_a_time".to_owned(),
            VERIFY_BYTES_YIELDS_ONE_BYTE_AT_A_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Bytes<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Bytes<&'static [u8]>>",
        "kani",
        || <RustStdStandard<std::io::Bytes<&'static [u8]>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BYTES_YIELDS_ONE_BYTE_AT_A_TIME_SRC, {
        /// `.bytes()` yields each byte of the reader individually, in
        /// order.
        #[kani::proof]
        fn verify_bytes_yields_one_byte_at_a_time() {
            use std::io::Read;

            let collected: Vec<u8> = (b"abc"[..]).bytes().map(|b| b.unwrap()).collect();
            assert!(CollectedSequenceMatchesExpected::ensures((
                collected,
                vec![b'a', b'b', b'c']
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_into_inner_error_recovers_the_writer_and_the_flush_error".to_owned(),
            VERIFY_INTO_INNER_ERROR_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>",
        "kani",
        || <RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniFlushErrorObservation` instance actually
/// demonstrated recovering both the flush failure and the buffered data,
/// minted only by [`KaniFlushErrorObservation::demonstrate_recovery`].
pub struct KaniFlushErrorWitnessToken(());

impl ProofToken for KaniFlushErrorWitnessToken {
    type Proposition = KaniFlushErrorObservation;
}

impl KaniFlushErrorObservation {
    /// Assert the flush failed and the buffered bytes are recoverable
    /// unchanged. Consumes `self` for the same reason
    /// [`KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_recovery(self, buffered: [u8; 2]) -> KaniFlushErrorWitnessToken {
        assert!(self.flush_failed());
        assert_eq!(self.recovered_buffer(), buffered);
        KaniFlushErrorWitnessToken(())
    }
}

/// Lawful token minted once
/// `RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>`'s recovery claim has
/// been established from a `KaniFlushErrorObservation`.
pub struct RustStdIntoInnerErrorToken(());

impl ProofToken for RustStdIntoInnerErrorToken {
    type Proposition = RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>;
}

impl Establish<KaniFlushErrorWitnessToken, KaniVerifier>
    for RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>
{
    type Token = RustStdIntoInnerErrorToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniFlushErrorWitnessToken) -> Self::Token {
        RustStdIntoInnerErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_INTO_INNER_ERROR_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC, {
        /// `BufWriter::into_inner()` fails when flushing fails, and the
        /// resulting error recovers both the underlying `io::Error` and
        /// the writer itself.
        /// This proof uses the Amenable-owned bounded flush-failure model:
        /// if the real `into_inner` recovery path refines this observation,
        /// the Rust-facing recovery claim follows. The claim is established
        /// through `Establish<KaniFlushErrorObservation, KaniVerifier> for
        /// RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>` from the
        /// observation instance that actually demonstrated the recovery.
        #[kani::proof]
        fn verify_into_inner_error_recovers_the_writer_and_the_flush_error() {
            let buffered = [kani::any(), kani::any()];
            let observation = crate::KaniFlushErrorObservation::new(buffered);
            let demonstration = observation.demonstrate_recovery(buffered);

            let _token =
                RustStdStandard::<IntoInnerError<BufWriter<Vec<u8>>>>::establish(demonstration);
        }
    }
}
