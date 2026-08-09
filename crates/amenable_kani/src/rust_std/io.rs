//! `KaniWitness` impls for `std::io`.
//!
//! `Bytes` and the simple `BufWriter` flush law still verify directly over
//! in-memory `&'static [u8]` / `Vec<u8>` readers and writers, as do the
//! zero-state adapters `Empty`, `Repeat`, `SeekFrom`, and `Sink`: none of
//! them carry internal buffering state, so there's nothing for Kani to
//! time out on. The rest of the
//! buffered `std::io` family (`BufReader`, `IntoInnerError`, `LineWriter`,
//! `Lines`, `Split`, `WriterPanicked`) uses Amenable-owned bounded
//! observations instead: these direct std paths are pure in-memory, but still
//! time out or hit unsupported panic capture under Kani. `Stdin`/`Stdout`/
//! `Stderr` and their lock guards are process-attached global handles with no
//! checkable invariant beyond what the type system already guarantees
//! (exercising them for real would mean writing to, or blocking on, the
//! actual process's standard streams during the proof), so those six stay
//! "trusted." The direct `PipeReader`/`PipeWriter` setup path reaches
//! unsupported `pipe2` under Kani, so the production proofs use an
//! Amenable-owned anonymous-pipe model instead; the direct std path remains
//! preserved in the gallery as a false trail.

use std::io::{
    BufReader, BufWriter, Cursor, IntoInnerError, IoSlice, IoSliceMut, LineWriter, PipeReader,
    PipeWriter, SeekFrom, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, WriterPanicked,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};
use crate::{
    KaniBufReadSplitObservation, KaniBufferedReadObservation, KaniFlushErrorObservation,
    KaniLineWriterObservation, KaniLinesObservation, KaniVerifier, KaniWitness,
    KaniWriterPanickedObservation,
};

impl KaniWitness for RustStdStandard<BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_buf_reader_reads_the_underlying_bytes".to_owned(),
            claim: VERIFY_BUF_READER_READS_THE_UNDERLYING_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BufReader<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BufReader<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<BufReader<&'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_buf_writer_flushes_to_the_underlying_writer".to_owned(),
            claim: VERIFY_BUF_WRITER_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BufWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BufWriter<Vec<u8>>>",
        verifier: "kani",
        describe: || <RustStdStandard<BufWriter<Vec<u8>>> as KaniWitness>::proof().to_string(),
    }
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
                writer.get_ref().is_empty(),
                "a small write remains buffered before flush"
            );
            writer.flush().unwrap();
            assert_eq!(writer.into_inner().unwrap(), b"hello");
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Bytes<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_bytes_yields_one_byte_at_a_time".to_owned(),
            claim: VERIFY_BYTES_YIELDS_ONE_BYTE_AT_A_TIME_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Bytes<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Bytes<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Bytes<&'static [u8]>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BYTES_YIELDS_ONE_BYTE_AT_A_TIME_SRC, {
        /// `.bytes()` yields each byte of the reader individually, in
        /// order.
        #[kani::proof]
        fn verify_bytes_yields_one_byte_at_a_time() {
            use std::io::Read;

            let collected: Vec<u8> = (b"abc"[..]).bytes().map(|b| b.unwrap()).collect();
            assert_eq!(collected, vec![b'a', b'b', b'c']);
        }
    }
}

impl KaniWitness for RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_into_inner_error_recovers_the_writer_and_the_flush_error".to_owned(),
            claim: VERIFY_INTO_INNER_ERROR_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence:
            "amenable_std::rust_std::RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>",
        verifier: "kani",
        describe: || <RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>> as KaniWitness>::proof()
            .to_string(),
    }
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

impl KaniWitness for RustStdStandard<LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_line_writer_flushes_on_a_newline_but_not_before_one".to_owned(),
            claim: VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<LineWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LineWriter<Vec<u8>>>",
        verifier: "kani",
        describe: || <RustStdStandard<LineWriter<Vec<u8>>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniLineWriterObservation` instance actually
/// demonstrated the buffering and flush-on-newline behavior, minted only
/// by [`KaniLineWriterObservation::demonstrate_flush_behavior`].
pub struct KaniLineWriterWitnessToken(());

impl ProofToken for KaniLineWriterWitnessToken {
    type Proposition = KaniLineWriterObservation;
}

impl KaniLineWriterObservation {
    /// Assert a newline-terminated write reaches the underlying writer
    /// immediately, a trailing partial line stays buffered until flush,
    /// and flush then delivers it. Consumes `self` for the same reason
    /// [`KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[must_use]
    pub fn demonstrate_flush_behavior(
        self,
        line_byte: u8,
        trailing_byte: u8,
    ) -> KaniLineWriterWitnessToken {
        assert_eq!(self.after_newline_write(), [line_byte, b'\n']);
        assert_eq!(
            self.buffered_before_flush(),
            [line_byte, b'\n'],
            "the trailing partial line stays buffered until flush"
        );
        assert_eq!(self.after_flush(), [line_byte, b'\n', trailing_byte]);
        KaniLineWriterWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<LineWriter<Vec<u8>>>`'s
/// line-buffering claim has been established from a
/// `KaniLineWriterObservation`.
pub struct RustStdLineWriterToken(());

impl ProofToken for RustStdLineWriterToken {
    type Proposition = RustStdStandard<LineWriter<Vec<u8>>>;
}

impl Establish<KaniLineWriterWitnessToken, KaniVerifier> for RustStdStandard<LineWriter<Vec<u8>>> {
    type Token = RustStdLineWriterToken;

    fn establish(_credential: KaniLineWriterWitnessToken) -> Self::Token {
        RustStdLineWriterToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC, {
        /// A line ending in `\n` reaches the underlying writer
        /// immediately, but a trailing partial line stays buffered until
        /// the next newline or an explicit flush.
        /// This proof uses the Amenable-owned bounded line-buffer model:
        /// if the real `LineWriter` path refines this observation, the
        /// Rust-facing flush-on-newline claim follows. The claim is
        /// established through `Establish<KaniLineWriterObservation,
        /// KaniVerifier> for RustStdStandard<LineWriter<Vec<u8>>>` from the
        /// observation instance that actually demonstrated the buffering and
        /// flush behavior.
        #[kani::proof]
        fn verify_line_writer_flushes_on_a_newline_but_not_before_one() {
            let line_byte: u8 = kani::any();
            let trailing_byte: u8 = kani::any();
            kani::assume(line_byte != b'\n');
            kani::assume(trailing_byte != b'\n');
            let observation = crate::KaniLineWriterObservation::new(line_byte, trailing_byte);
            let demonstration = observation.demonstrate_flush_behavior(line_byte, trailing_byte);

            let _token = RustStdStandard::<LineWriter<Vec<u8>>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Lines<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_lines_splits_on_newlines_and_drops_the_terminator".to_owned(),
            claim: VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Lines<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Lines<&'static [u8]>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniLinesObservation` instance actually demonstrated
/// its terminator-dropping line split, minted only by
/// [`KaniLinesObservation::demonstrate_line_split`].
pub struct KaniLinesWitnessToken(());

impl ProofToken for KaniLinesWitnessToken {
    type Proposition = KaniLinesObservation;
}

impl KaniLinesObservation {
    /// Assert `.lines()` yields the three lines with their terminators
    /// dropped. Consumes `self` for the same reason
    /// [`KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[must_use]
    pub fn demonstrate_line_split(self, first: u8, second: u8, third: u8) -> KaniLinesWitnessToken {
        assert_eq!(self.lines(), ([first], [second], [third]));
        KaniLinesWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::io::Lines<&'static [u8]>>`'s
/// line-splitting claim has been established from a `KaniLinesObservation`.
pub struct RustStdLinesToken(());

impl ProofToken for RustStdLinesToken {
    type Proposition = RustStdStandard<std::io::Lines<&'static [u8]>>;
}

impl Establish<KaniLinesWitnessToken, KaniVerifier>
    for RustStdStandard<std::io::Lines<&'static [u8]>>
{
    type Token = RustStdLinesToken;

    fn establish(_credential: KaniLinesWitnessToken) -> Self::Token {
        RustStdLinesToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC, {
        /// `.lines()` yields each line without its trailing `\n`.
        /// This proof uses the Amenable-owned bounded line-splitting model:
        /// if the real `BufRead::lines` path refines this observation, the
        /// Rust-facing terminator-dropping claim follows. The claim is
        /// established through `Establish<KaniLinesObservation,
        /// KaniVerifier> for RustStdStandard<std::io::Lines<&'static [u8]>>`
        /// from the observation instance that actually demonstrated the
        /// line split.
        #[kani::proof]
        fn verify_lines_splits_on_newlines_and_drops_the_terminator() {
            let first: u8 = kani::any();
            let second: u8 = kani::any();
            let third: u8 = kani::any();
            kani::assume(first.is_ascii() && first != b'\n' && first != b'\r');
            kani::assume(second.is_ascii() && second != b'\n' && second != b'\r');
            kani::assume(third.is_ascii() && third != b'\n' && third != b'\r');
            let observation = crate::KaniLinesObservation::new(first, second, third);
            let demonstration = observation.demonstrate_line_split(first, second, third);

            let _token =
                RustStdStandard::<std::io::Lines<&'static [u8]>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<PipeReader> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pipe_reader_reads_what_the_paired_writer_wrote".to_owned(),
            claim: VERIFY_PIPE_READER_READS_WHAT_THE_PAIRED_WRITER_WROTE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PipeReader>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PipeReader>",
        verifier: "kani",
        describe: || <RustStdStandard<PipeReader> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<PipeReader>,
    "amenable_std::rust_std::RustStdStandard<PipeReader>",
    (Vec<u8>, Vec<u8>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_PIPE_READER_READS_WHAT_THE_PAIRED_WRITER_WROTE_SRC, {
        /// Bytes written to a pipe's writer half arrive, unaltered, on
        /// the paired reader half. This proof uses the Amenable-owned pipe
        /// accommodation model: if the real std/libc path refines these
        /// modeled laws, the Rust-facing delivery claim follows.
        #[kani::proof]
        fn verify_pipe_reader_reads_what_the_paired_writer_wrote() {
            let mut pipe = <crate::KaniPipe as crate::KaniCompose>::kani_depth0();
            let reader = pipe.reader();
            let writer = pipe.writer();
            let payload = <[u8; 2] as crate::KaniCompose>::kani_any();
            let expected = payload.to_vec();

            pipe.write_all(writer.clone(), expected.clone());
            pipe.close_writer(writer);

            let collected = pipe.read_to_end(reader.clone());
            assert!(RustStdStandard::<PipeReader>::ensures((collected, expected)));
            assert_eq!(reader.resource_id(), pipe.resource_id());
        }
    }
}

impl KaniWitness for RustStdStandard<PipeWriter> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pipe_writer_writes_arrive_at_the_paired_reader".to_owned(),
            claim: VERIFY_PIPE_WRITER_WRITES_ARRIVE_AT_THE_PAIRED_READER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PipeWriter>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PipeWriter>",
        verifier: "kani",
        describe: || <RustStdStandard<PipeWriter> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<PipeWriter>,
    "amenable_std::rust_std::RustStdStandard<PipeWriter>",
    (Vec<u8>, Vec<u8>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_PIPE_WRITER_WRITES_ARRIVE_AT_THE_PAIRED_READER_SRC, {
        /// The same delivery contract as `PipeReader`, checked from the
        /// writer's side: `.write_all()` succeeds and the bytes are
        /// recoverable. This proof uses the Amenable-owned pipe
        /// accommodation model: if the real std/libc path refines these
        /// modeled laws, the Rust-facing delivery claim follows.
        #[kani::proof]
        fn verify_pipe_writer_writes_arrive_at_the_paired_reader() {
            let mut pipe = <crate::KaniPipe as crate::KaniCompose>::kani_depth0();
            let reader = pipe.reader();
            let writer = pipe.writer();
            let payload = <[u8; 2] as crate::KaniCompose>::kani_any();
            let expected = payload.to_vec();
            let writer_resource = writer.resource_id();

            pipe.write_all(writer.clone(), expected.clone());
            pipe.close_writer(writer);

            let collected = pipe.read_to_end(reader.clone());
            assert!(RustStdStandard::<PipeWriter>::ensures((collected, expected)));
            assert_eq!(writer_resource, reader.resource_id());
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Split<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_segments_on_the_given_byte_and_drops_it".to_owned(),
            claim: VERIFY_SPLIT_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Split<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Split<&'static [u8]>> as KaniWitness>::proof().to_string(),
    }
}

/// Witness that a `KaniBufReadSplitObservation` instance actually
/// demonstrated its delimiter-dropping split, minted only by
/// [`KaniBufReadSplitObservation::demonstrate_segments`].
pub struct KaniBufReadSplitWitnessToken(());

impl ProofToken for KaniBufReadSplitWitnessToken {
    type Proposition = KaniBufReadSplitObservation;
}

impl KaniBufReadSplitObservation {
    /// Assert `.segments()` yields the three segments with the delimiter
    /// dropped. Consumes `self` for the same reason
    /// [`KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[must_use]
    pub fn demonstrate_segments(
        self,
        first: u8,
        second: u8,
        third: u8,
    ) -> KaniBufReadSplitWitnessToken {
        assert_eq!(self.segments(), ([first], [second], [third]));
        KaniBufReadSplitWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::io::Split<&'static [u8]>>`'s
/// delimiter-dropping split claim has been established from a
/// `KaniBufReadSplitObservation`.
pub struct RustStdBufReadSplitToken(());

impl ProofToken for RustStdBufReadSplitToken {
    type Proposition = RustStdStandard<std::io::Split<&'static [u8]>>;
}

impl Establish<KaniBufReadSplitWitnessToken, KaniVerifier>
    for RustStdStandard<std::io::Split<&'static [u8]>>
{
    type Token = RustStdBufReadSplitToken;

    fn establish(_credential: KaniBufReadSplitWitnessToken) -> Self::Token {
        RustStdBufReadSplitToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC, {
        /// `.split()` yields the segments between a given byte, dropping
        /// the separator itself. The inherent slice `split` shadows
        /// `BufRead::split` in method-call syntax, so it's invoked via
        /// its fully-qualified trait path.
        /// This proof uses the Amenable-owned bounded split model: if the
        /// real `BufRead::split` path refines this observation, the
        /// Rust-facing delimiter-dropping claim follows. The claim is
        /// established through `Establish<KaniBufReadSplitObservation,
        /// KaniVerifier> for RustStdStandard<std::io::Split<&'static
        /// [u8]>>` from the observation instance that actually
        /// demonstrated the split.
        #[kani::proof]
        fn verify_split_segments_on_the_given_byte_and_drops_it() {
            let first: u8 = kani::any();
            let delimiter: u8 = kani::any();
            let second: u8 = kani::any();
            let third: u8 = kani::any();
            kani::assume(first != delimiter);
            kani::assume(second != delimiter);
            kani::assume(third != delimiter);
            let observation = crate::KaniBufReadSplitObservation::new(
                first, delimiter, second, third,
            );
            let demonstration = observation.demonstrate_segments(first, second, third);

            let _token =
                RustStdStandard::<std::io::Split<&'static [u8]>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_writer_panicked_recovers_the_buffered_data".to_owned(),
            claim: VERIFY_WRITER_PANICKED_RECOVERS_THE_BUFFERED_DATA_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<WriterPanicked>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<WriterPanicked>",
        verifier: "kani",
        describe: || <RustStdStandard<WriterPanicked> as KaniWitness>::proof().to_string(),
    }
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
    /// [`KaniBufferedReadObservation::demonstrate_read_through`] does.
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
        CheckedProof {
            harness: "verify_empty_read_reports_end_of_file".to_owned(),
            claim: VERIFY_EMPTY_READ_REPORTS_END_OF_FILE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Empty>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Empty>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Empty> as KaniWitness>::proof().to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_repeat_fills_the_buffer_with_the_given_byte".to_owned(),
            claim: VERIFY_REPEAT_FILLS_THE_BUFFER_WITH_THE_GIVEN_BYTE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Repeat>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Repeat>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Repeat> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(read, buffer.len(), "Repeat::read always fills the whole buffer");
            for filled in buffer {
                assert!(
                    RustStdStandard::<std::io::Repeat>::ensures((filled, byte)),
                    "Repeat::read fills every slot with the given byte"
                );
            }
        }
    }
}

impl KaniWitness for RustStdStandard<SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_seek_from_round_trips_each_variants_offset".to_owned(),
            claim: VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        verifier: "kani",
        describe: || <RustStdStandard<SeekFrom> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<SeekFrom>,
    "amenable_std::rust_std::RustStdStandard<SeekFrom>",
    (i64, i64),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// Each `SeekFrom` variant preserves the offset it was
        /// constructed with, and stays its own variant. The assertions
        /// call `RustStdStandard::<SeekFrom>::ensures` directly rather
        /// than restating the comparison.
        #[kani::proof]
        fn verify_seek_from_round_trips_each_variants_offset() {
            let start_offset: u64 = kani::any();
            let end_offset: i64 = kani::any();
            let current_offset: i64 = kani::any();

            match SeekFrom::Start(start_offset) {
                SeekFrom::Start(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value as i64, start_offset as i64)),
                        "SeekFrom::Start preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::Start must construct the Start variant"),
            }
            match SeekFrom::End(end_offset) {
                SeekFrom::End(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value, end_offset)),
                        "SeekFrom::End preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::End must construct the End variant"),
            }
            match SeekFrom::Current(current_offset) {
                SeekFrom::Current(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value, current_offset)),
                        "SeekFrom::Current preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::Current must construct the Current variant"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Sink> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_sink_write_reports_full_length_and_discards_content".to_owned(),
            claim: VERIFY_SINK_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Sink>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Sink>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Sink> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SINK_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC, {
        /// `std::io::sink()`'s writer always reports the full length
        /// written and never errors, regardless of the content offered.
        #[kani::proof]
        fn verify_sink_write_reports_full_length_and_discards_content() {
            use std::io::Write;

            let data: [u8; 4] = kani::any();
            let mut writer = std::io::sink();
            let written = writer.write(&data).expect("Sink::write never errors");
            assert_eq!(written, data.len(), "Sink::write reports the full length written");
            writer.flush().expect("Sink::flush never errors");
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chain_reads_the_first_source_then_the_second".to_owned(),
            claim: VERIFY_CHAIN_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence:
            "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
    ([u8; 2], [u8; 2]),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAIN_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC, {
        /// `.chain()` reads its first source to exhaustion before it
        /// starts reading the second.
        #[kani::proof]
        fn verify_chain_reads_the_first_source_then_the_second() {
            use std::io::Read;

            let first: [u8; 2] = kani::any();
            let second: [u8; 2] = kani::any();
            let mut chain = (&first[..]).chain(&second[..]);
            let mut buffer = [0u8; 2];

            let read_first = chain
                .read(&mut buffer)
                .expect("Chain::read over in-memory slices never errors");
            assert_eq!(
                read_first,
                buffer.len(),
                "Chain::read drains the first source fully before touching the second"
            );
            assert!(
                RustStdStandard::<std::io::Chain<&'static [u8], &'static [u8]>>::ensures((
                    buffer, first
                )),
                "Chain::read yields the first source's bytes first"
            );

            let read_second = chain
                .read(&mut buffer)
                .expect("Chain::read over in-memory slices never errors");
            assert_eq!(
                read_second,
                buffer.len(),
                "Chain::read continues into the second source once the first is exhausted"
            );
            assert!(
                RustStdStandard::<std::io::Chain<&'static [u8], &'static [u8]>>::ensures((
                    buffer, second
                )),
                "Chain::read yields the second source's bytes once the first is drained"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Cursor<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cursor_read_advances_position_and_seek_repositions_it".to_owned(),
            claim: VERIFY_CURSOR_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Cursor<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cursor<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<Cursor<&'static [u8]>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CURSOR_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC, {
        /// `Cursor::read` yields bytes from the current position and
        /// advances it by the amount read; `Cursor::seek` repositions it.
        #[kani::proof]
        fn verify_cursor_read_advances_position_and_seek_repositions_it() {
            use std::io::{Read, Seek};

            let data: [u8; 4] = kani::any();
            let mut cursor = Cursor::new(&data[..]);
            let mut buffer = [0u8; 2];

            let read = cursor
                .read(&mut buffer)
                .expect("Cursor::read over an in-memory slice never errors");
            assert_eq!(read, buffer.len(), "Cursor::read fills the requested buffer");
            assert_eq!(
                buffer,
                [data[0], data[1]],
                "Cursor::read yields bytes starting from position zero"
            );
            assert_eq!(
                cursor.position(),
                2,
                "Cursor::read advances position by the number of bytes read"
            );

            cursor
                .seek(SeekFrom::Start(0))
                .expect("Cursor::seek to a valid offset never errors");
            assert_eq!(
                cursor.position(),
                0,
                "Cursor::seek(Start(0)) resets position to zero"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_error_from_error_kind_preserves_the_kind".to_owned(),
            claim: VERIFY_ERROR_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Error>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ERROR_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC, {
        /// `Error::from(kind)` preserves the given `ErrorKind`, recoverable
        /// unchanged via `.kind()`. `ErrorKind` has no `kani::Arbitrary`
        /// impl (it's a large foreign enum with no way to derive one), so
        /// this checks a representative, bounded-exhaustive subset rather
        /// than a fully symbolic kind.
        #[kani::proof]
        fn verify_error_from_error_kind_preserves_the_kind() {
            use std::io::ErrorKind;

            let kinds = [
                ErrorKind::NotFound,
                ErrorKind::PermissionDenied,
                ErrorKind::AlreadyExists,
                ErrorKind::InvalidInput,
            ];
            let index: usize = kani::any();
            kani::assume(index < kinds.len());
            let kind = kinds[index];

            let error = std::io::Error::from(kind);
            assert_eq!(
                error.kind(),
                kind,
                "Error::from(kind).kind() recovers the given kind"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IoSlice<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_io_slice_derefs_to_the_wrapped_bytes".to_owned(),
            claim: VERIFY_IO_SLICE_DEREFS_TO_THE_WRAPPED_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IoSlice<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IoSlice<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<IoSlice<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_IO_SLICE_DEREFS_TO_THE_WRAPPED_BYTES_SRC, {
        /// `IoSlice::new` borrows a byte slice without copying it:
        /// dereferencing the `IoSlice` yields exactly the wrapped bytes.
        #[kani::proof]
        fn verify_io_slice_derefs_to_the_wrapped_bytes() {
            let bytes: [u8; 4] = kani::any();
            let slice = IoSlice::new(&bytes);
            assert_eq!(&*slice, &bytes, "IoSlice derefs to exactly the wrapped bytes");
        }
    }
}

impl KaniWitness for RustStdStandard<IoSliceMut<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_io_slice_mut_derefs_to_and_permits_mutating_the_wrapped_bytes"
                .to_owned(),
            claim: VERIFY_IO_SLICE_MUT_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IoSliceMut<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IoSliceMut<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<IoSliceMut<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_IO_SLICE_MUT_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC, {
        /// `IoSliceMut::new` mutably borrows a byte slice without copying
        /// it: dereferencing yields exactly the wrapped bytes, and writing
        /// through the `IoSliceMut` mutates the original slice.
        #[kani::proof]
        fn verify_io_slice_mut_derefs_to_and_permits_mutating_the_wrapped_bytes() {
            let mut bytes: [u8; 4] = kani::any();
            let original = bytes;
            let new_value: u8 = kani::any();

            let mut slice = IoSliceMut::new(&mut bytes);
            assert_eq!(&*slice, &original, "IoSliceMut derefs to exactly the wrapped bytes");
            slice[0] = new_value;
            drop(slice);
            assert_eq!(
                bytes[0], new_value,
                "mutating through IoSliceMut mutates the wrapped bytes"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Take<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_take_caps_reads_at_the_remaining_limit".to_owned(),
            claim: VERIFY_TAKE_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Take<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::io::Take<&'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TAKE_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC, {
        /// `.take(limit)` caps a single read at the remaining limit, and
        /// that limit is exhausted by exactly the bytes read.
        #[kani::proof]
        fn verify_take_caps_reads_at_the_remaining_limit() {
            use std::io::Read;

            let data: [u8; 4] = kani::any();
            let limit: u64 = kani::any();
            kani::assume(limit <= data.len() as u64);

            let mut reader = (&data[..]).take(limit);
            let mut buffer = [0u8; 4];
            let read = reader
                .read(&mut buffer)
                .expect("Take::read over an in-memory slice never errors");

            assert_eq!(
                read as u64, limit,
                "Take::read yields exactly the remaining limit when the source has enough bytes"
            );
            assert_eq!(
                reader.limit(),
                0,
                "Take::limit reaches zero once a read consumes the whole allowance"
            );
        }
    }
}

impl_kani_witness_trusted!(
    Stderr,
    StderrLock<'static>,
    Stdin,
    StdinLock<'static>,
    Stdout,
    StdoutLock<'static>,
);
