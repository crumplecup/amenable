use std::str::{SplitAsciiWhitespace, SplitWhitespace, Utf8Chunks};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use super::lines_and_markers::CollectedSequenceMatchesExpected;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
#[cfg(kani)]
use crate::{EmptiedContainerReportsEmpty, IteratorYieldsNoneWhenExhausted};

impl KaniWitness for RustStdStandard<SplitAsciiWhitespace<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_ascii_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitAsciiWhitespace<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitAsciiWhitespace<'static>>",
        "kani",
        || <RustStdStandard<SplitAsciiWhitespace<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC, {
        /// `.split_ascii_whitespace()` collapses runs of whitespace and
        /// drops leading/trailing whitespace entirely.
        #[kani::proof]
        fn verify_split_ascii_whitespace_collapses_runs_of_whitespace() {
            let s = " a  b ";
            let parts: Vec<&str> = s.split_ascii_whitespace().collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b"])),
                "runs of whitespace collapse to a single split point"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitWhitespace<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitWhitespace<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitWhitespace<'static>>",
        "kani",
        || <RustStdStandard<SplitWhitespace<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC, {
        /// Same collapsing behavior as `SplitAsciiWhitespace`, over
        /// Unicode whitespace.
        #[kani::proof]
        fn verify_split_whitespace_collapses_runs_of_whitespace() {
            let s = " a  b ";
            let parts: Vec<&str> = s.split_whitespace().collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b"])),
                "runs of whitespace collapse to a single split point"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Utf8Chunks<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunks_yields_one_chunk_for_wholly_valid_input".to_owned(),
            VERIFY_UTF8_CHUNKS_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Chunks<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Chunks<'static>>",
        "kani",
        || <RustStdStandard<Utf8Chunks<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_CHUNKS_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC, {
        /// Re-validating wholly valid UTF-8 bytes yields exactly one
        /// chunk, with no trailing invalid bytes and nothing left over.
        #[kani::proof]
        fn verify_utf8_chunks_yields_one_chunk_for_wholly_valid_input() {
            let bytes = b"ab";
            let mut chunks = bytes.utf8_chunks();
            let first = chunks.next().unwrap();
            assert!(AccessorRecoversTheExpectedValue::ensures((first.valid(), "ab")));
            assert!(
                EmptiedContainerReportsEmpty::ensures(first.invalid().is_empty()),
                "wholly valid input has no invalid bytes"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(chunks.next()),
                "wholly valid input is exactly one chunk"
            );
        }
    }
}

/// An `(actual, expected)` pair known to agree: a plain accessor
/// method call recovers exactly the expected value -- distinct from a
/// field access (`FieldAccessRecoversTheStoredValue`), an index
/// (`IndexRecoversTheStoredElement`), or an iterator's `.next()`
/// (`IteratorYieldsAReferenceToTheStoredValue`) even though the
/// `Ensures` impl body is identical trivial equality either way, same
/// reasoning as keeping those types separate from each other.
///
/// Independently hand-written as `assert_eq!(chunk.valid(), "ab", ...)`
/// / `assert_eq!(chunk.invalid(), &[0xFFu8][..], ...)` at 2 real sites
/// in `Utf8Chunk`'s own `.valid()`/`.invalid()` accessors.
pub struct AccessorRecoversTheExpectedValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for AccessorRecoversTheExpectedValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for AccessorRecoversTheExpectedValue<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}
