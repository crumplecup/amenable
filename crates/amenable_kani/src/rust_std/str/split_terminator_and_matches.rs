use std::str::{Matches, RMatches, RSplitTerminator, SplitTerminator};

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
use crate::{
    FourBytesAreEachAscii, SplitOperandsAreDistinctFromThePattern, ThreeBytesAreEachAscii,
    ThreeSplitOperandsAreDistinctFromThePattern,
};

impl KaniWitness for RustStdStandard<SplitTerminator<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_terminator_suppresses_a_trailing_empty_substring".to_owned(),
            VERIFY_SPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitTerminator<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitTerminator<'static, char>>",
        "kani",
        || <RustStdStandard<SplitTerminator<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC, {
        /// `.split_terminator(pat)` treats the pattern as a terminator
        /// rather than a separator: a match at the very end of the str
        /// does not produce a trailing empty substring. This proof uses
        /// the Amenable-owned bounded str-pattern accommodation model
        /// (`KaniStrSplitTerminatorObservation`, symbolic over all three
        /// ASCII positions): if the real `split_terminator` path refines
        /// this two-occurrence, nothing-after-the-last-match
        /// `[a, pattern, b, pattern]` window, the Rust-facing claim
        /// follows. The direct path times out for real inside this
        /// crate despite passing in an isolated probe crate -- see
        /// `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
        #[kani::proof]
        fn verify_split_terminator_suppresses_a_trailing_empty_substring() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((a, pattern, b)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                a, pattern, b,
            )));
            let observation =
                crate::KaniStrSplitTerminatorObservation::new(a as char, pattern as char, b as char);
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    observation.split_terminator(),
                    [a as char, b as char]
                )),
                "no trailing empty substring after the terminal match"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitTerminator<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_terminator_suppresses_a_trailing_empty_substring_from_the_back"
                .to_owned(),
            VERIFY_RSPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitTerminator<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RSplitTerminator<'static, char>>",
        "kani",
        || <RustStdStandard<RSplitTerminator<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC, {
        /// Same terminator-suppression behavior as `SplitTerminator`,
        /// traversed from the back. This proof uses the same
        /// `KaniStrSplitTerminatorObservation` model as `SplitTerminator`
        /// -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rsplit_terminator_suppresses_a_trailing_empty_substring_from_the_back() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((a, pattern, b)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                a, pattern, b,
            )));
            let observation =
                crate::KaniStrSplitTerminatorObservation::new(a as char, pattern as char, b as char);
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    observation.rsplit_terminator(),
                    [b as char, a as char]
                )),
                "no trailing empty substring after the terminal match, traversed from the back"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Matches<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_matches_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_MATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Matches<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Matches<'static, char>>",
        "kani",
        || <RustStdStandard<Matches<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC, {
        /// `.matches(pat)` yields every non-overlapping occurrence of
        /// the pattern. This proof uses the Amenable-owned bounded
        /// str-pattern accommodation model (`KaniStrMatchObservation`,
        /// symbolic over all three filler ASCII positions and the
        /// pattern itself): if the real `matches` path refines this
        /// two-occurrence `[f0, pattern, f1, pattern, f2]` window, the
        /// Rust-facing claim follows. The direct path times out for
        /// real inside this crate despite passing in an isolated probe
        /// crate -- see
        /// `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
        #[kani::proof]
        fn verify_matches_yields_every_non_overlapping_occurrence() {
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
                    observation.matches(),
                    [pattern as char, pattern as char]
                )),
                "matches finds every non-overlapping occurrence"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RMatches<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rmatches_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_RMATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RMatches<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RMatches<'static, char>>",
        "kani",
        || <RustStdStandard<RMatches<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RMATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC, {
        /// Same non-overlapping-occurrence behavior as `Matches`,
        /// traversed from the back; content alone can't distinguish
        /// traversal order for a `char` pattern (see this module's own
        /// doc). This proof uses the same `KaniStrMatchObservation`
        /// model as `Matches` -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rmatches_yields_every_non_overlapping_occurrence() {
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
                    observation.rmatches(),
                    [pattern as char, pattern as char]
                )),
                "rmatches finds every non-overlapping occurrence"
            );
        }
    }
}
