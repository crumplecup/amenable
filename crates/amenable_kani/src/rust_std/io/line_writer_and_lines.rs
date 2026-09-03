use std::io::LineWriter;

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_requires};
use crate::{KaniLineWriterObservation, KaniLinesObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_line_writer_flushes_on_a_newline_but_not_before_one".to_owned(),
            VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LineWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LineWriter<Vec<u8>>>",
        "kani",
        || <RustStdStandard<LineWriter<Vec<u8>>> as KaniWitness>::proof().to_string(),
    )
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
    /// [`crate::KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

/// A `(value, marker)` pair known to satisfy `value != marker`.
///
/// Independently hand-written as `kani::assume(a != b)` at 5 real
/// sites: `verify_line_writer_flushes_on_a_newline_but_not_before_one`
/// (a byte distinct from the fixed literal newline marker `b'\n'`, 2
/// sites) and `verify_split_segments_on_the_given_byte_and_drops_it`
/// (a byte distinct from the symbolic split delimiter, 3 sites) -- the
/// identical single-pair distinctness precondition regardless of
/// whether the marker is fixed or symbolic. Unlike
/// `SplitOperandsAreDistinctFromThePattern`, which combines two
/// distinctness checks against one pattern into a single call, every
/// real site here asserts exactly one pair at a time, so this needs
/// no type parameter and no combined `&&`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct ByteIsDistinctFromTheMarker;

impl KaniWitness for ByteIsDistinctFromTheMarker {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_line_writer_flushes_on_a_newline_but_not_before_one".to_owned(),
            VERIFY_LINE_WRITER_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ByteIsDistinctFromTheMarker);

kani_requires!(
    ByteIsDistinctFromTheMarker,
    "amenable_kani::ByteIsDistinctFromTheMarker",
    (u8, u8),
    |(value, marker)| value != marker
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ByteIsDistinctFromTheMarker",
        "kani",
        || <ByteIsDistinctFromTheMarker as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(ByteIsDistinctFromTheMarker::requires((line_byte, b'\n')));
            kani::assume(ByteIsDistinctFromTheMarker::requires((trailing_byte, b'\n')));
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
        CheckedProof::new(
            "verify_lines_splits_on_newlines_and_drops_the_terminator".to_owned(),
            VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Lines<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
        "kani",
        || <RustStdStandard<std::io::Lines<&'static [u8]>> as KaniWitness>::proof().to_string(),
    )
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
    /// [`crate::KaniBufferedReadObservation::demonstrate_read_through`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniLinesWitnessToken) -> Self::Token {
        RustStdLinesToken(())
    }
}

/// A `u8` known to be ASCII and neither `\n` nor `\r` -- ordinary line
/// content that can never itself be mistaken for a line terminator.
///
/// Independently hand-written as `kani::assume(byte.is_ascii() && byte
/// != b'\n' && byte != b'\r')` at 3 real sites in
/// `verify_lines_splits_on_newlines_and_drops_the_terminator` (one per
/// symbolic line-content byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct ByteIsAsciiAndNotALineTerminator;

impl KaniWitness for ByteIsAsciiAndNotALineTerminator {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lines_splits_on_newlines_and_drops_the_terminator".to_owned(),
            VERIFY_LINES_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ByteIsAsciiAndNotALineTerminator);

kani_requires!(
    ByteIsAsciiAndNotALineTerminator,
    "amenable_kani::ByteIsAsciiAndNotALineTerminator",
    u8,
    |byte| byte.is_ascii() && byte != b'\n' && byte != b'\r'
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ByteIsAsciiAndNotALineTerminator",
        "kani",
        || <ByteIsAsciiAndNotALineTerminator as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(ByteIsAsciiAndNotALineTerminator::requires(first));
            kani::assume(ByteIsAsciiAndNotALineTerminator::requires(second));
            kani::assume(ByteIsAsciiAndNotALineTerminator::requires(third));
            let observation = crate::KaniLinesObservation::new(first, second, third);
            let demonstration = observation.demonstrate_line_split(first, second, third);

            let _token =
                RustStdStandard::<std::io::Lines<&'static [u8]>>::establish(demonstration);
        }
    }
}
