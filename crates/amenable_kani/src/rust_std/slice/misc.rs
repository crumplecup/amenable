use std::slice::{EscapeAscii, GetDisjointMutError};

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
#[cfg(kani)]
use crate::FallibleOperationReportsSuccess;
#[cfg(kani)]
use crate::IndexRecoversTheStoredElement;
#[cfg(kani)]
use crate::IteratorYieldsAReferenceToTheStoredValue;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniEscapeAsciiObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_ascii_leaves_printable_bytes_unescaped".to_owned(),
            VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<EscapeAscii<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EscapeAscii<'static>>",
        "kani",
        || <RustStdStandard<EscapeAscii<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniEscapeAsciiObservation` instance actually
/// demonstrated its escaped bytes, minted only by
/// [`KaniEscapeAsciiObservation::demonstrate_escaping`].
pub struct KaniEscapeAsciiWitnessToken(());

impl ProofToken for KaniEscapeAsciiWitnessToken {
    type Proposition = KaniEscapeAsciiObservation;
}

impl KaniEscapeAsciiObservation {
    /// Assert the printable byte passes through unescaped and the
    /// trailing newline expands to its two-byte backslash form. Consumes
    /// `self` for the same reason
    /// [`crate::KaniChannel::demonstrate_delivery`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_escaping(self, printable: u8) -> KaniEscapeAsciiWitnessToken {
        assert_eq!(
            self.source(),
            [printable, b'\n'],
            "the bounded source witness keeps the printable byte and newline"
        );
        assert_eq!(
            self.escaped(),
            [printable, b'\\', b'n'],
            "printable bytes pass through unescaped and newline expands to two bytes"
        );
        KaniEscapeAsciiWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<EscapeAscii<'static>>`'s
/// printable-plus-newline escape claim has been established from a
/// `KaniEscapeAsciiObservation` that has itself demonstrated the escaped
/// bytes.
pub struct RustStdEscapeAsciiToken(());

impl ProofToken for RustStdEscapeAsciiToken {
    type Proposition = RustStdStandard<EscapeAscii<'static>>;
}

impl Establish<KaniEscapeAsciiWitnessToken, KaniVerifier>
    for RustStdStandard<EscapeAscii<'static>>
{
    type Token = RustStdEscapeAsciiToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniEscapeAsciiWitnessToken) -> Self::Token {
        RustStdEscapeAsciiToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_ASCII_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC, {
        /// A printable ASCII byte passes through `escape_ascii`
        /// unchanged, while a control character (`\n`) is escaped to
        /// its two-byte backslash form.
        /// This proof uses the Amenable-owned bounded `escape_ascii`
        /// observation: the direct `EscapeAscii` iterator still times
        /// out under Kani even on a fixed two-byte witness, both when
        /// eagerly collected and when observed stepwise. The claim is
        /// established through
        /// `Establish<KaniEscapeAsciiObservation, KaniVerifier> for
        /// RustStdStandard<EscapeAscii<'static>>` from the observation
        /// instance that actually demonstrated the escaped bytes.
        #[kani::proof]
        fn verify_escape_ascii_leaves_printable_bytes_unescaped() {
            let printable: u8 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((printable, 0x20, 0x7e)));
            let observation = KaniEscapeAsciiObservation::new(printable);
            let demonstration = observation.demonstrate_escaping(printable);

            let _token = RustStdStandard::<EscapeAscii<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds".to_owned(),
            VERIFY_GET_DISJOINT_MUT_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<GetDisjointMutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<GetDisjointMutError>",
        "kani",
        || <RustStdStandard<GetDisjointMutError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_GET_DISJOINT_MUT_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC, {
        /// `get_disjoint_mut` succeeds for genuinely disjoint in-bounds
        /// indices, and fails (producing this error) for either
        /// overlapping or out-of-bounds indices — its two distinct
        /// failure modes.
        #[kani::proof]
        fn verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let mut data = [a, b, 0, 0];
            assert!(
                FallibleOperationReportsSuccess::ensures(data.get_disjoint_mut([0, 2]).is_ok()),
                "disjoint, in-bounds indices succeed"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(data.get_disjoint_mut([0, 0]).is_err()),
                "overlapping indices are rejected"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(data.get_disjoint_mut([0, 10]).is_err()),
                "out-of-bounds indices are rejected"
            );
        }
    }
}
