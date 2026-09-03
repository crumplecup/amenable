//! The rest of `std::io` (pipe/split/chain/cursor/error/vectored-slice/take,
//! via their own macros where the shape repeats). The
//! `std::sync::atomic::AtomicT` instantiations live in `sync_atomic`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::str_more_and_io_a::VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

macro_rules! impl_io_pipe_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_pipe_model_delivers_written_bytes_to_the_paired_reader".to_owned(),
                    VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_io_pipe_verus_witness!(std::io::PipeReader);
impl_io_pipe_verus_witness!(std::io::PipeWriter);

const VERIFY_SPLIT_MODEL_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_split_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Split<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_split_model_segments_on_the_given_byte_and_drops_it".to_owned(),
            VERIFY_SPLIT_MODEL_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Split<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Split<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// The shared pairwise-distinctness precondition `amenable_std::
// verus_witness` registers for several accommodation models that build
// a symbolic non-overlapping match/split window.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::Split<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
    "values_are_distinct"
);

const VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_writer_panicked_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_writer_panicked_model_recovers_the_buffered_data".to_owned(),
            VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::WriterPanicked>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::WriterPanicked>",
        "verus",
        || {
            <RustStdStandard<std::io::WriterPanicked> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_io_empty_repeat_sink_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../../amenable_verus/src/rust_std/io/io_empty_repeat_sink_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_io_empty_repeat_sink_verus_witness!(
    std::io::Empty,
    "verify_empty_model_read_reports_end_of_file",
    VERIFY_EMPTY_MODEL_READ_REPORTS_END_OF_FILE_SRC
);

// Singleton contract: `Empty::read` always reports the literal `0`
// bytes read.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Empty>,
    "amenable_std::rust_std::RustStdStandard<std::io::Empty>",
    "empty_read_reports_zero_bytes"
);
impl_io_empty_repeat_sink_verus_witness!(
    std::io::Repeat,
    "verify_repeat_model_fills_the_buffer_with_the_given_byte",
    VERIFY_REPEAT_MODEL_FILLS_THE_BUFFER_WITH_THE_GIVEN_BYTE_SRC
);

// The four-element counterpart to `observed_pair_matches_input`/
// `observed_triple_matches_input`, registered once here for all its
// real call sites.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Repeat>,
    "amenable_std::rust_std::RustStdStandard<std::io::Repeat>",
    "observed_quad_matches_input"
);
impl_io_empty_repeat_sink_verus_witness!(
    std::io::Sink,
    "verify_sink_model_write_reports_full_length_and_discards_content",
    VERIFY_SINK_MODEL_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC
);

const VERIFY_SEEK_FROM_MODEL_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_seek_from_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_seek_from_model_round_trips_each_variants_offset".to_owned(),
            VERIFY_SEEK_FROM_MODEL_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::SeekFrom>",
        "verus",
        || {
            <RustStdStandard<std::io::SeekFrom> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_chain_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chain_model_reads_the_first_source_then_the_second".to_owned(),
            VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_cursor_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Cursor<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cursor_model_read_advances_position_and_seek_repositions_it".to_owned(),
            VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Cursor<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Cursor<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Cursor<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: reading two bytes from position zero advances the
// position to exactly 2, and seeking back to `Start(0)` resets it to
// exactly 0.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Cursor<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Cursor<&'static [u8]>>",
    "cursor_positions_after_read_then_seek"
);

const VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_error_model_from_error_kind_preserves_the_kind".to_owned(),
            VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Error>",
        "verus",
        || {
            <RustStdStandard<std::io::Error> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: this model represents exactly the four
// representative `ErrorKind` variants as a tag `0..4`.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::Error>,
    "amenable_std::rust_std::RustStdStandard<std::io::Error>",
    "error_kind_index_is_representative"
);

const VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSlice<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_io_slice_model_derefs_to_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSlice<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IoSlice<'static>>",
        "verus",
        || {
            <RustStdStandard<std::io::IoSlice<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSliceMut<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSliceMut<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IoSliceMut<'static>>",
        "verus",
        || {
            <RustStdStandard<std::io::IoSliceMut<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/io/io_take_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Take<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_model_caps_reads_at_the_remaining_limit".to_owned(),
            VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Take<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Take<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: the allowance is always exactly exhausted (0)
// after a read that consumes the whole limit.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Take<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
    "take_allowance_is_exhausted"
);
