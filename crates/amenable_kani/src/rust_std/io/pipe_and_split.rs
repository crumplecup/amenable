use std::io::{PipeReader, PipeWriter};

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use super::line_writer_and_lines::ByteIsDistinctFromTheMarker;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{KaniBufReadSplitObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<PipeReader> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_pipe_reader_reads_what_the_paired_writer_wrote".to_owned(),
            VERIFY_PIPE_READER_READS_WHAT_THE_PAIRED_WRITER_WROTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PipeReader>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PipeReader>",
        "kani",
        || <RustStdStandard<PipeReader> as KaniWitness>::proof().to_string(),
    )
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
            assert!(RustStdStandard::<u64>::ensures((
                reader.resource_id(),
                pipe.resource_id()
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<PipeWriter> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_pipe_writer_writes_arrive_at_the_paired_reader".to_owned(),
            VERIFY_PIPE_WRITER_WRITES_ARRIVE_AT_THE_PAIRED_READER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PipeWriter>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PipeWriter>",
        "kani",
        || <RustStdStandard<PipeWriter> as KaniWitness>::proof().to_string(),
    )
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
            assert!(RustStdStandard::<u64>::ensures((
                writer_resource,
                reader.resource_id()
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Split<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_segments_on_the_given_byte_and_drops_it".to_owned(),
            VERIFY_SPLIT_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Split<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
        "kani",
        || <RustStdStandard<std::io::Split<&'static [u8]>> as KaniWitness>::proof().to_string(),
    )
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
    /// [`crate::KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
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
            kani::assume(ByteIsDistinctFromTheMarker::requires((first, delimiter)));
            kani::assume(ByteIsDistinctFromTheMarker::requires((second, delimiter)));
            kani::assume(ByteIsDistinctFromTheMarker::requires((third, delimiter)));
            let observation = crate::KaniBufReadSplitObservationBuilder::default()
                .first(first)
                .delimiter(delimiter)
                .second(second)
                .third(third)
                .build()
                .expect("all fields set");
            let demonstration = observation.demonstrate_segments(first, second, third);

            let _token =
                RustStdStandard::<std::io::Split<&'static [u8]>>::establish(demonstration);
        }
    }
}
