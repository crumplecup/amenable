//! The `AsciiByte` gallery marker, the remaining `str` match-index iterators,
//! and the first half of the `std::io` buffered-reader/writer family.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::str_family::VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC;
use crate::{AsciiByte, RustStdStandard};
use amenable_core::Evidence;

/// [`AsciiByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition the harness already requires, it
/// doesn't prove anything new. The precondition recurs across four
/// carrier files (`str_ascii_iter_carrier`, `str_pattern_match_carrier`,
/// `str_pattern_reverse_carrier`, `str_pattern_terminator_carrier`) —
/// every real site now calls the one shared spec fn,
/// `primitive_shapes_carrier::is_ascii_byte`. (An earlier version of this
/// registration hand-typed one inert, non-`fn` string per carrier's
/// local variable spelling instead of a real shared predicate — that
/// text could never satisfy the call-shape recognition rule and never
/// actually named any of these sites; replaced rather than kept
/// alongside a real fragment constant, which is why none exists here to
/// name.)
impl VerusWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_matches_model_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(AsciiByte);

amenable_derive::verus_requires_predicate!(AsciiByte, "amenable_std::AsciiByte", "is_ascii_byte");

const VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::MatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_match_indices_model_pairs_each_match_with_its_byte_offset".to_owned(),
            VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::MatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::MatchIndices<'static, char>>",
        "verus",
        || {
            <RustStdStandard<std::str::MatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC:
    &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::RMatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back"
                .to_owned(),
            VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::RMatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RMatchIndices<'static, char>>",
        "verus",
        || {
            <RustStdStandard<std::str::RMatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_buffered_read_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_buf_reader_model_reads_the_underlying_bytes".to_owned(),
            VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufReader<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::BufReader<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::BufReader<&'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_buf_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_buf_writer_model_flushes_to_the_underlying_writer".to_owned(),
            VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::BufWriter<Vec<u8>>>",
        "verus",
        || {
            <RustStdStandard<std::io::BufWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_bytes_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Bytes<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_bytes_model_yields_one_byte_at_a_time".to_owned(),
            VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Bytes<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Bytes<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Bytes<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_into_inner_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_into_inner_error_model_recovers_the_writer_and_the_flush_error".to_owned(),
            VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>",
        "verus",
        || {
            <RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_line_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_line_writer_model_flushes_on_a_newline_but_not_before_one".to_owned(),
            VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::LineWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
        "verus",
        || {
            <RustStdStandard<std::io::LineWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::LineWriter<Vec<u8>>>,
    "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
    "is_not_a_newline_byte"
);

// The three-element counterpart to `observed_pair_matches_input`,
// registered once here for all its real call sites.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::LineWriter<Vec<u8>>>,
    "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
    "observed_triple_matches_input"
);

const VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Lines<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lines_model_splits_on_newlines_and_drops_the_terminator".to_owned(),
            VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Lines<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Lines<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::Lines<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
    "is_not_a_line_terminator_byte"
);

pub(super) const VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_pipe_carrier.rs");
