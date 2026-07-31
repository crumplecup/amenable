//! `KaniWitness` impls for `std::io`.
//!
//! `Bytes` and the simple `BufWriter` flush law still verify directly over
//! in-memory `&'static [u8]` / `Vec<u8>` readers and writers. The rest of the
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
    BufReader, BufWriter, IntoInnerError, LineWriter, PipeReader, PipeWriter, Stderr, StderrLock,
    Stdin, StdinLock, Stdout, StdoutLock, WriterPanicked,
};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
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
            assert_eq!(collected, expected);
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
            assert_eq!(collected, expected);
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

impl_kani_witness_trusted!(
    Stderr,
    StderrLock<'static>,
    Stdin,
    StdinLock<'static>,
    Stdout,
    StdoutLock<'static>,
);
