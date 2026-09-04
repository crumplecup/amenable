#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::whitespace_utf8::VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC;
use crate::CheckedProof;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<std::str::Lines<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lines_splits_on_line_endings".to_owned(),
            VERIFY_LINES_SPLITS_ON_LINE_ENDINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::Lines<'static>>);

kani_ensures!(
    RustStdStandard<std::str::Lines<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
    (Option<&'static str>, Option<&'static str>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
        "kani",
        || <RustStdStandard<std::str::Lines<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LINES_SPLITS_ON_LINE_ENDINGS_SRC, {
        /// `.lines()` splits on `\n`, without yielding a trailing empty
        /// line.
        #[kani::proof]
        fn verify_lines_splits_on_line_endings() {
            let s = "a\nb";
            let mut it = s.lines();
            assert!(RustStdStandard::<std::str::Lines<'static>>::ensures((it.next(), Some("a"))));
            assert!(RustStdStandard::<std::str::Lines<'static>>::ensures((it.next(), Some("b"))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

/// A collected sequence known to match exactly the expected sequence.
///
/// Independently hand-written as `assert_eq!(collected, vec![...],
/// ...)` at 8 real sites spanning `Vec<i32>` (a drained `VecDeque`, a
/// rejected `try_reserve`'s untouched content, `Vec::extract_if`'s two
/// halves), `Vec<u16>` (`encode_wide`'s UTF-16 units), and `Vec<&str>`
/// (`split_ascii_whitespace`/`split_whitespace`/`splitn`'s parts), plus
/// 3 more sites in `rust_std::alloc_string` over `Vec<u8>`
/// (`FromUtf8Error::into_bytes` recovering the original owned bytes) --
/// the identical claim regardless of collected element type. Generic over
/// the collected sequence type rather than one registration per
/// producer, the same reasoning (and the same reason it needs a
/// hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `DerefReflectsTheStoredValue` (`rust_std::primitives`) -- not a
/// reuse of that type directly, since its name states a deref claim
/// this isn't.
pub struct CollectedSequenceMatchesExpected<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for CollectedSequenceMatchesExpected<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for CollectedSequenceMatchesExpected<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for CollectedSequenceMatchesExpected<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_ascii_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for CollectedSequenceMatchesExpected<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for CollectedSequenceMatchesExpected<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::CollectedSequenceMatchesExpected",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CollectedSequenceMatchesExpected",
        "kani",
        || <CollectedSequenceMatchesExpected<i32> as KaniWitness>::proof().to_string(),
    )
}
