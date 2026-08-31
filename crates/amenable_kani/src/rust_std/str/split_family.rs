use std::str::{RSplit, RSplitN, Split, SplitInclusive, SplitN};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
#[cfg(kani)]
use amenable_std::AsciiByte;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use super::lines_and_markers::CollectedSequenceMatchesExpected;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
#[cfg(kani)]
use crate::{
    EmptiedContainerReportsEmpty, FourBytesAreEachAscii, IteratorYieldsNoneWhenExhausted,
    SplitOperandsAreDistinctFromThePattern, ThreeBytesAreEachAscii,
    ThreeSplitOperandsAreDistinctFromThePattern, ValueIsAtLeast,
};

impl KaniWitness for RustStdStandard<Split<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_yields_substrings_between_pattern_matches".to_owned(),
            VERIFY_SPLIT_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Split<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Split<'static, char>>",
        "kani",
        || <RustStdStandard<Split<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC, {
        /// `.split(pat)` yields the substrings between matches, in
        /// forward order.
        #[kani::proof]
        fn verify_split_yields_substrings_between_pattern_matches() {
            let parts: Vec<&str> = "a,b,c".split(',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b", "c"])),
                "split yields substrings between matches, forward"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitN<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_splitn_limits_to_n_substrings".to_owned(),
            VERIFY_SPLITN_LIMITS_TO_N_SUBSTRINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitN<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::SplitN<'static, char>>",
        "kani",
        || <RustStdStandard<SplitN<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLITN_LIMITS_TO_N_SUBSTRINGS_SRC, {
        /// `.splitn(n, pat)` stops after `n` substrings, leaving the
        /// remainder of the str unsplit in the final item.
        #[kani::proof]
        fn verify_splitn_limits_to_n_substrings() {
            let parts: Vec<&str> = "a,b,c".splitn(2, ',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b,c"])),
                "splitn stops after n items, leaving the remainder unsplit"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitInclusive<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_inclusive_keeps_the_delimiter_attached".to_owned(),
            VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_DELIMITER_ATTACHED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitInclusive<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::SplitInclusive<'static, char>>",
        "kani",
        || <RustStdStandard<SplitInclusive<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_DELIMITER_ATTACHED_SRC, {
        /// `.split_inclusive(pat)` keeps each matched delimiter attached
        /// to the end of the substring that precedes it.
        #[kani::proof]
        fn verify_split_inclusive_keeps_the_delimiter_attached() {
            let parts: Vec<&str> = "a,b,c".split_inclusive(',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a,", "b,", "c"])),
                "split_inclusive keeps the delimiter attached to each substring"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplit<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_yields_substrings_from_the_back".to_owned(),
            VERIFY_RSPLIT_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplit<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RSplit<'static, char>>",
        "kani",
        || <RustStdStandard<RSplit<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC, {
        /// `.rsplit(pat)` yields the piece after the match, then the
        /// piece before it. This proof uses the Amenable-owned bounded
        /// str-pattern accommodation model (`KaniStrRSplitObservation`,
        /// symbolic over all three ASCII positions): if the real
        /// `rsplit` path refines this one-occurrence
        /// `[before, pattern, after]` window, the Rust-facing claim
        /// follows. The direct path times out under Kani even for a
        /// single `.next()` call on a fixed five-byte str -- see
        /// `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
        #[kani::proof]
        fn verify_rsplit_yields_substrings_from_the_back() {
            let before: u8 = kani::any();
            let pattern: u8 = kani::any();
            let after: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((before, pattern, after)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                before, pattern, after,
            )));
            let observation =
                crate::KaniStrRSplitObservation::new(before as char, pattern as char, after as char);
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    observation.rsplit(),
                    [after as char, before as char]
                )),
                "rsplit yields the piece after the match, then the piece before it"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitN<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplitn_limits_to_n_substrings_from_the_back".to_owned(),
            VERIFY_RSPLITN_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitN<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RSplitN<'static, char>>",
        "kani",
        || <RustStdStandard<RSplitN<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLITN_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC, {
        /// `.rsplitn(2, pat)` yields the piece after the last match,
        /// then everything before it uncut. This proof uses the
        /// Amenable-owned bounded str-pattern accommodation model
        /// (`KaniStrRSplitNObservation`, symbolic over all four ASCII
        /// positions): if the real `rsplitn` path refines this
        /// two-occurrence `[a, pattern, b, pattern, c]` window, the
        /// Rust-facing claim follows. The direct path hits the same
        /// reverse-search timeout as `RSplit` -- see
        /// `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
        #[kani::proof]
        fn verify_rsplitn_limits_to_n_substrings_from_the_back() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((a, pattern, b, c)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                a, b, c, pattern,
            )));
            let observation =
                crate::KaniStrRSplitNObservationBuilder::default().a(a as char).pattern(pattern as char).b(b as char).c(c as char).build().expect("all fields set");
            let (first, rest) = observation.rsplitn_two();
            assert!(
                RustStdStandard::<char>::ensures((first, c as char)),
                "rsplitn's first piece is everything after the last match"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    rest,
                    [a as char, pattern as char, b as char]
                )),
                "rsplitn's second piece is everything before the last match, uncut"
            );
        }
    }
}
