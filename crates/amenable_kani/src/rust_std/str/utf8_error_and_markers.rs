use std::str::{ParseBoolError, Utf8Chunk, Utf8Error};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;

use super::whitespace_utf8::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};
#[cfg(kani)]
use crate::{IteratorYieldsNoneWhenExhausted, ValueIsAtLeast};

impl<T> KaniWitness for AccessorRecoversTheExpectedValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes".to_owned(),
            VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for AccessorRecoversTheExpectedValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for AccessorRecoversTheExpectedValue<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::AccessorRecoversTheExpectedValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::AccessorRecoversTheExpectedValue",
        "kani",
        || <AccessorRecoversTheExpectedValue<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Utf8Chunk<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes".to_owned(),
            VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Chunk<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Chunk<'static>>",
        "kani",
        || <RustStdStandard<Utf8Chunk<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC, {
        /// A `Utf8Chunk`'s `.valid()`/`.invalid()` split a byte
        /// sequence containing one bad byte into its valid-UTF-8 prefix
        /// and the trailing invalid byte.
        #[kani::proof]
        fn verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes() {
            let bytes = b"ab\xFFcd";
            let mut chunks = bytes.utf8_chunks();
            let first = chunks.next().unwrap();
            assert!(
                AccessorRecoversTheExpectedValue::ensures((first.valid(), "ab")),
                "the chunk's valid() is the UTF-8 prefix before the bad byte"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((first.invalid(), &[0xFFu8][..])),
                "the chunk's invalid() is the bad byte itself"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Utf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_error_reports_the_valid_prefix_length_and_error_span".to_owned(),
            VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Error>",
        "kani",
        || <RustStdStandard<Utf8Error> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC, {
        /// `str::from_utf8`'s error reports exactly how much of the
        /// input was valid (`valid_up_to`) and the width of the single
        /// bad byte (`error_len`).
        /// This proof uses the Amenable-owned UTF-8 accommodation model
        /// (`utf8_model.rs`'s `KaniUtf8::error_position`, which exposes the
        /// same `valid_up_to`/`error_len` position information as
        /// `std::str::Utf8Error`): the direct `std::str::from_utf8` path
        /// times out even for fixed concrete literals (see
        /// `gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector`).
        /// `0xF5..=0xFF` are never valid UTF-8 lead bytes anywhere, so any
        /// byte in that range is a lone one-byte error regardless of its
        /// neighbors.
        #[kani::proof]
        fn verify_utf8_error_reports_the_valid_prefix_length_and_error_span() {
            let invalid: u8 = kani::any();
            kani::assume(ValueIsAtLeast::requires((invalid, 0xF5)));
            let bytes = [b'a', b'b', invalid, b'c'];
            let err = crate::KaniUtf8::error_position(&bytes).unwrap_err();
            assert!(
                RustStdStandard::<usize>::ensures((err.valid_up_to(), 2)),
                "two leading bytes were valid UTF-8"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((err.error_len(), Some(1))),
                "the single bad byte has error_len 1"
            );
        }
    }
}

impl_kani_witness_trusted!(ParseBoolError);

// `LinesAny` is deprecated (in favor of `Lines`) but still stable and real;
// covering it is a coverage-completeness question, not a call to use it.
// `#[expect(deprecated)]` on a macro invocation is silently ignored by
// rustc (confirmed empirically) rather than suppressing warnings from
// inside its expansion, so `bridge_kani_witness!`/`inventory::submit!`
// below would otherwise warn on every mention of the type name. Routing
// through this locally `#[expect(deprecated)]`-attributed alias hides the
// deprecation at every downstream use site instead, confirmed the same way.
#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
type LinesAnyStatic = std::str::LinesAny<'static>;

impl KaniWitness for RustStdStandard<LinesAnyStatic> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lines_any_splits_on_any_line_ending".to_owned(),
            VERIFY_LINES_ANY_SPLITS_ON_ANY_LINE_ENDING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LinesAnyStatic>);

kani_ensures!(
    RustStdStandard<LinesAnyStatic>,
    "amenable_std::rust_std::RustStdStandard<LinesAnyStatic>",
    (Option<&'static str>, Option<&'static str>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinesAny<'static>>",
        "kani",
        || <RustStdStandard<LinesAnyStatic> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LINES_ANY_SPLITS_ON_ANY_LINE_ENDING_SRC, {
        /// Unlike `Lines` (which splits on `\n` alone, treating a
        /// preceding `\r` as part of the line's own trimming),
        /// `LinesAny` is documented to split on either `\n` or `\r\n` —
        /// deprecated in favor of `Lines`, but a real, distinct, still-
        /// stable carrier worth covering. Deprecated: only the call
        /// site needs `#[expect(deprecated)]`, not the whole module.
        #[expect(
            deprecated,
            reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
        )]
        #[kani::proof]
        fn verify_lines_any_splits_on_any_line_ending() {
            let s = "a\r\nb";
            let mut it = s.lines_any();
            assert!(RustStdStandard::<LinesAnyStatic>::ensures((it.next(), Some("a"))));
            assert!(RustStdStandard::<LinesAnyStatic>::ensures((it.next(), Some("b"))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}
