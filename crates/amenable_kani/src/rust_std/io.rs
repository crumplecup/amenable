//! `KaniWitness` impls for `std::io`.
//!
//! `BufReader`/`Bytes`/`Lines`/`Split` are proved over `&'static [u8]`, and
//! `BufWriter`/`LineWriter`/`IntoInnerError` over `Vec<u8>` — in-memory
//! `Read`/`Write`/`BufRead` implementors, so every harness runs without
//! real filesystem or OS-handle I/O. `Stdin`/`Stdout`/`Stderr` and their
//! lock guards are process-attached global handles with no checkable
//! invariant beyond what the type system already guarantees (exercising
//! them for real would mean writing to, or blocking on, the actual
//! process's standard streams during the proof), so those six stay
//! "trusted." `PipeReader`/`PipeWriter` create a genuinely fresh OS pipe
//! each time, so they get real behavioral harnesses like the in-memory
//! types.

use std::io::{
    BufReader, BufWriter, IntoInnerError, LineWriter, PipeReader, PipeWriter, Stderr, StderrLock,
    Stdin, StdinLock, Stdout, StdoutLock, WriterPanicked,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_buf_reader_reads_the_underlying_bytes",
            claim: VERIFY_BUF_READER_READS_THE_UNDERLYING_BYTES_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_BUF_READER_READS_THE_UNDERLYING_BYTES_SRC, {
        /// A `BufReader` reads through to exactly the bytes of the reader
        /// it wraps.
        #[kani::proof]
        fn verify_buf_reader_reads_the_underlying_bytes() {
            use std::io::Read;

            let mut reader = BufReader::new(&b"hello"[..]);
            let mut collected = String::new();
            reader.read_to_string(&mut collected).unwrap();
            assert_eq!(collected, "hello");
        }
    }
}

impl KaniWitness for RustStdStandard<BufWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_buf_writer_flushes_to_the_underlying_writer",
            claim: VERIFY_BUF_WRITER_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC,
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
            harness: "verify_bytes_yields_one_byte_at_a_time",
            claim: VERIFY_BYTES_YIELDS_ONE_BYTE_AT_A_TIME_SRC,
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
            harness: "verify_into_inner_error_recovers_the_writer_and_the_flush_error",
            claim: VERIFY_INTO_INNER_ERROR_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_INTO_INNER_ERROR_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC, {
        /// `BufWriter::into_inner()` fails when flushing fails, and the
        /// resulting error recovers both the underlying `io::Error` and
        /// the writer itself.
        #[kani::proof]
        fn verify_into_inner_error_recovers_the_writer_and_the_flush_error() {
            use std::io::Write;

            struct FailingWriter;
            impl Write for FailingWriter {
                fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                    Err(std::io::Error::other("always fails"))
                }
                fn flush(&mut self) -> std::io::Result<()> {
                    Err(std::io::Error::other("always fails"))
                }
            }

            let mut failing = BufWriter::new(FailingWriter);
            failing.write_all(b"buffered, not yet flushed").unwrap();
            match failing.into_inner() {
                Err(err) => {
                    assert_eq!(err.error().to_string(), "always fails");
                    let _recovered_writer: BufWriter<FailingWriter> = err.into_inner();
                }
                Ok(_) => panic!("expected into_inner to fail when flushing fails"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_line_writer_flushes_on_a_newline_but_not_before_one",
            claim: VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC, {
        /// A line ending in `\n` reaches the underlying writer
        /// immediately, but a trailing partial line stays buffered until
        /// the next newline or an explicit flush.
        #[kani::proof]
        fn verify_line_writer_flushes_on_a_newline_but_not_before_one() {
            use std::io::Write;

            let mut writer = LineWriter::new(Vec::new());
            writer.write_all(b"abc\n").unwrap();
            assert_eq!(writer.get_ref().as_slice(), b"abc\n");

            writer.write_all(b"def").unwrap();
            assert_eq!(
                writer.get_ref().as_slice(),
                b"abc\n",
                "the partial line stays buffered until a newline or flush"
            );

            writer.flush().unwrap();
            assert_eq!(writer.get_ref().as_slice(), b"abc\ndef");
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Lines<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_lines_splits_on_newlines_and_drops_the_terminator",
            claim: VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC, {
        /// `.lines()` yields each line without its trailing `\n`.
        #[kani::proof]
        fn verify_lines_splits_on_newlines_and_drops_the_terminator() {
            use std::io::BufRead;

            let lines: Vec<String> = (b"a\nb\nc"[..]).lines().map(|l| l.unwrap()).collect();
            assert_eq!(lines, vec!["a", "b", "c"]);
        }
    }
}

impl KaniWitness for RustStdStandard<PipeReader> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pipe_reader_reads_what_the_paired_writer_wrote",
            claim: VERIFY_PIPE_READER_READS_WHAT_THE_PAIRED_WRITER_WROTE_SRC,
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
        /// the paired reader half.
        #[kani::proof]
        fn verify_pipe_reader_reads_what_the_paired_writer_wrote() {
            use std::io::{Read, Write};

            let (mut reader, mut writer) = std::io::pipe().unwrap();
            writer.write_all(b"piped").unwrap();
            drop(writer);
            let mut collected = Vec::new();
            reader.read_to_end(&mut collected).unwrap();
            assert_eq!(collected, b"piped");
        }
    }
}

impl KaniWitness for RustStdStandard<PipeWriter> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pipe_writer_writes_arrive_at_the_paired_reader",
            claim: VERIFY_PIPE_WRITER_WRITES_ARRIVE_AT_THE_PAIRED_READER_SRC,
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
        /// recoverable.
        #[kani::proof]
        fn verify_pipe_writer_writes_arrive_at_the_paired_reader() {
            use std::io::{Read, Write};

            let (mut reader, mut writer) = std::io::pipe().unwrap();
            writer.write_all(b"piped").unwrap();
            drop(writer);
            let mut collected = Vec::new();
            reader.read_to_end(&mut collected).unwrap();
            assert_eq!(collected, b"piped");
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Split<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_split_segments_on_the_given_byte_and_drops_it",
            claim: VERIFY_SPLIT_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_SPLIT_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC, {
        /// `.split()` yields the segments between a given byte, dropping
        /// the separator itself. The inherent slice `split` shadows
        /// `BufRead::split` in method-call syntax, so it's invoked via
        /// its fully-qualified trait path.
        #[kani::proof]
        fn verify_split_segments_on_the_given_byte_and_drops_it() {
            use std::io::BufRead;

            let pieces: Vec<Vec<u8>> = BufRead::split(&b"a,b,c"[..], b',')
                .map(|piece| piece.unwrap())
                .collect();
            assert_eq!(pieces, vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()]);
        }
    }
}

impl KaniWitness for RustStdStandard<WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_writer_panicked_recovers_the_buffered_data",
            claim: VERIFY_WRITER_PANICKED_RECOVERS_THE_BUFFERED_DATA_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_WRITER_PANICKED_RECOVERS_THE_BUFFERED_DATA_SRC, {
        /// When a `BufWriter`'s inner writer panics mid-write, the panic
        /// is caught rather than corrupting the buffer, and
        /// `.into_parts()` afterward reports `WriterPanicked` while still
        /// recovering exactly the data that was buffered.
        #[kani::proof]
        fn verify_writer_panicked_recovers_the_buffered_data() {
            use std::io::Write;

            struct PanickingWriter;
            impl Write for PanickingWriter {
                fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                    panic!("writer panicked");
                }
                fn flush(&mut self) -> std::io::Result<()> {
                    Ok(())
                }
            }

            let mut writer = BufWriter::new(PanickingWriter);
            writer.write_all(b"data").unwrap();
            let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                writer.flush().unwrap();
            }));
            assert!(caught.is_err(), "the inner writer's panic propagates out");
            match writer.into_parts().1 {
                Err(writer_panicked) => assert_eq!(writer_panicked.into_inner(), b"data"),
                Ok(_) => panic!("expected WriterPanicked after a caught panic"),
            }
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
