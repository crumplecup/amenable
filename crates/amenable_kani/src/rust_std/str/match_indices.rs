use std::str::{MatchIndices, RMatchIndices};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use super::lines_and_markers::CollectedSequenceMatchesExpected;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::bridge_kani_witness;
#[cfg(kani)]
use crate::{FourBytesAreEachAscii, ThreeSplitOperandsAreDistinctFromThePattern};

impl KaniWitness for RustStdStandard<MatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_match_indices_pairs_each_match_with_its_byte_offset".to_owned(),
            VERIFY_MATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<MatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<MatchIndices<'static, char>>",
        "kani",
        || <RustStdStandard<MatchIndices<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC, {
        /// `.match_indices(pat)` pairs each match with its byte offset,
        /// in forward (left-to-right) order. This proof uses the same
        /// `KaniStrMatchObservation` model as `Matches` -- see that
        /// proof's doc comment. The window's fixed byte offsets (1 and
        /// 3) hold because every field is exactly one ASCII byte.
        #[kani::proof]
        fn verify_match_indices_pairs_each_match_with_its_byte_offset() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    observation.match_indices(),
                    [(1, pattern as char), (3, pattern as char)]
                )),
                "match_indices pairs each match with its byte offset, forward"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RMatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rmatch_indices_pairs_each_match_with_its_byte_offset_from_the_back".to_owned(),
            VERIFY_RMATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RMatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RMatchIndices<'static, char>>",
        "kani",
        || <RustStdStandard<RMatchIndices<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RMATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC, {
        /// `.rmatch_indices(pat)` pairs each match with its byte offset,
        /// same set as `match_indices` but in reverse (right-to-left)
        /// order -- the one place in this cluster where forward/reverse
        /// traversal is directly assertable by value, since the byte
        /// offset (unlike the matched substring itself) differs per
        /// occurrence. This proof uses the same `KaniStrMatchObservation`
        /// model as `Matches` -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rmatch_indices_pairs_each_match_with_its_byte_offset_from_the_back() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    observation.rmatch_indices(),
                    [(3, pattern as char), (1, pattern as char)]
                )),
                "rmatch_indices pairs each match with its byte offset, in reverse"
            );
        }
    }
}
