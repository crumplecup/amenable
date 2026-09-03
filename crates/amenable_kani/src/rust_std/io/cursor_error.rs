use std::io::Cursor;
#[cfg(kani)]
use std::io::SeekFrom;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::error_kind_and_io_slice::VERIFY_ERROR_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Cursor<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cursor_read_advances_position_and_seek_repositions_it".to_owned(),
            VERIFY_CURSOR_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cursor<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cursor<&'static [u8]>>",
        "kani",
        || <RustStdStandard<Cursor<&'static [u8]>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(
                RustStdStandard::<usize>::ensures((read, buffer.len())),
                "Cursor::read fills the requested buffer"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures((buffer, [data[0], data[1]])),
                "Cursor::read yields bytes starting from position zero"
            );
            assert!(
                RustStdStandard::<u64>::ensures((cursor.position(), 2)),
                "Cursor::read advances position by the number of bytes read"
            );

            cursor
                .seek(SeekFrom::Start(0))
                .expect("Cursor::seek to a valid offset never errors");
            assert!(
                RustStdStandard::<u64>::ensures((cursor.position(), 0)),
                "Cursor::seek(Start(0)) resets position to zero"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_error_from_error_kind_preserves_the_kind".to_owned(),
            VERIFY_ERROR_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Error>",
        "kani",
        || <RustStdStandard<std::io::Error> as KaniWitness>::proof().to_string(),
    )
}

/// An `(actual, expected)` pair of `io::ErrorKind` values known to
/// agree -- `std::io::ErrorKind` has no `amenable_std::RustStdType`
/// registration to hang an `Ensures` impl on `RustStdStandard<ErrorKind>`
/// from (unlike `core::num::FpCategory`/`IntErrorKind`), so this is a
/// local, `amenable_kani`-only marker type instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct ErrorKindMatchesExpected;
