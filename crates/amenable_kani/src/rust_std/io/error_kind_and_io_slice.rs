use std::io::{IoSlice, IoSliceMut, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::cursor_error::ErrorKindMatchesExpected;
use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};

    pub(super) use crate::DerefReflectsTheStoredValue;
    pub(super) use crate::IndexRecoversTheStoredElement;
    pub(super) use crate::ValueIsAtLeast;
    pub(super) use crate::ValueIsWithinInclusiveRange;
}
#[cfg(kani)]
use mirror::{
    DerefReflectsTheStoredValue, Ensures, IndexRecoversTheStoredElement, Requires, ValueIsAtLeast,
    ValueIsWithinInclusiveRange,
};

impl KaniWitness for ErrorKindMatchesExpected {
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

bridge_kani_witness!(ErrorKindMatchesExpected);

kani_ensures!(
    ErrorKindMatchesExpected,
    "amenable_kani::ErrorKindMatchesExpected",
    (std::io::ErrorKind, std::io::ErrorKind),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ErrorKindMatchesExpected",
        "kani",
        || <ErrorKindMatchesExpected as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(ValueIsWithinInclusiveRange::requires((index, 0, kinds.len() - 1)));
            let kind = kinds[index];

            let error = std::io::Error::from(kind);
            assert!(
                ErrorKindMatchesExpected::ensures((error.kind(), kind)),
                "Error::from(kind).kind() recovers the given kind"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IoSlice<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_io_slice_derefs_to_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_DEREFS_TO_THE_WRAPPED_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IoSlice<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IoSlice<'static>>",
        "kani",
        || <RustStdStandard<IoSlice<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_IO_SLICE_DEREFS_TO_THE_WRAPPED_BYTES_SRC, {
        /// `IoSlice::new` borrows a byte slice without copying it:
        /// dereferencing the `IoSlice` yields exactly the wrapped bytes.
        #[kani::proof]
        fn verify_io_slice_derefs_to_the_wrapped_bytes() {
            let bytes: [u8; 4] = kani::any();
            let slice = IoSlice::new(&bytes);
            assert!(
                DerefReflectsTheStoredValue::ensures((&*slice, &bytes)),
                "IoSlice derefs to exactly the wrapped bytes"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IoSliceMut<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_io_slice_mut_derefs_to_and_permits_mutating_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_MUT_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IoSliceMut<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IoSliceMut<'static>>",
        "kani",
        || <RustStdStandard<IoSliceMut<'static>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(
                DerefReflectsTheStoredValue::ensures((&*slice, &original)),
                "IoSliceMut derefs to exactly the wrapped bytes"
            );
            slice[0] = new_value;
            drop(slice);
            assert!(
                IndexRecoversTheStoredElement::ensures((bytes[0], new_value)),
                "mutating through IoSliceMut mutates the wrapped bytes"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Take<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_take_caps_reads_at_the_remaining_limit".to_owned(),
            VERIFY_TAKE_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Take<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
        "kani",
        || <RustStdStandard<std::io::Take<&'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(ValueIsAtLeast::requires((data.len() as u64, limit)));

            let mut reader = (&data[..]).take(limit);
            let mut buffer = [0u8; 4];
            let read = reader
                .read(&mut buffer)
                .expect("Take::read over an in-memory slice never errors");

            assert!(
                RustStdStandard::<u64>::ensures((read as u64, limit)),
                "Take::read yields exactly the remaining limit when the source has enough bytes"
            );
            assert!(
                RustStdStandard::<u64>::ensures((reader.limit(), 0)),
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
