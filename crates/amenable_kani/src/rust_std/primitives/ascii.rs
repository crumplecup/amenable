use amenable_core::Evidence;
use amenable_std::{AsciiByte, RustStdStandard};

use super::array_slice_str::VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_requires};

/// [`AsciiByte`] reuses `verify_str_byte_length_and_content` rather than
/// adding a new Kani harness — it names the precondition the harness
/// already assumes, it doesn't prove anything new.
impl KaniWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(AsciiByte);

kani_requires!(AsciiByte, "amenable_std::AsciiByte", u8, |byte| byte < 128);

/// Four bytes each known to satisfy [`AsciiByte`]'s own precondition
/// (`< 128`), combined into a single callable predicate.
///
/// Independently hand-written as `kani::assume(a < 128 && pattern < 128
/// && b < 128 && c < 128)` at 5 real sites in `rust_std::str`'s
/// `*n`/`matches`/`match_indices` family -- the same four-way ASCII
/// bound `AsciiByte` already names for a single byte, just applied to
/// all four symbolic bytes a real site needs at once. A separate type
/// rather than four individual `AsciiByte::requires(...)` calls joined
/// by `&&` at the call site: the call-shape scanner only recognizes a
/// `kani::assume(EXPR)` clause as compliant when `EXPR` itself is a
/// single call, not a `&&`-combined expression of several real calls
/// (confirmed the hard way -- see `ThreeSplitOperandsAreDistinctFromThePattern`
/// for the same lesson applied to a `!=` combination).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FourBytesAreEachAscii;

impl KaniWitness for FourBytesAreEachAscii {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FourBytesAreEachAscii);

kani_requires!(
    FourBytesAreEachAscii,
    "amenable_kani::FourBytesAreEachAscii",
    (u8, u8, u8, u8),
    |(a, pattern, b, c)| a < 128 && pattern < 128 && b < 128 && c < 128
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FourBytesAreEachAscii",
        "kani",
        || <FourBytesAreEachAscii as KaniWitness>::proof().to_string(),
    )
}

/// Three bytes each known to satisfy [`AsciiByte`]'s own precondition
/// (`< 128`), combined into a single callable predicate — the
/// three-operand sibling of [`FourBytesAreEachAscii`], same reasoning.
///
/// Independently hand-written as `kani::assume(before < 128 && pattern
/// < 128 && after < 128)` at 3 real sites in `rust_std::str`'s
/// `rsplit`/`split_terminator`/`rsplit_terminator` family -- the same
/// three-way ASCII bound applied to all three symbolic bytes a real
/// site needs at once, for the same call-shape-scanner reason
/// `FourBytesAreEachAscii` documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct ThreeBytesAreEachAscii;

impl KaniWitness for ThreeBytesAreEachAscii {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ThreeBytesAreEachAscii);

kani_requires!(
    ThreeBytesAreEachAscii,
    "amenable_kani::ThreeBytesAreEachAscii",
    (u8, u8, u8),
    |(before, pattern, after)| before < 128 && pattern < 128 && after < 128
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ThreeBytesAreEachAscii",
        "kani",
        || <ThreeBytesAreEachAscii as KaniWitness>::proof().to_string(),
    )
}

/// A `(value, low, high)` triple known to satisfy the precondition
/// every proof over a small, symbolic-but-bounded value assumes: the
/// value falls within the inclusive range `low..=high`.
///
/// Independently hand-written as `kani::assume((low..=high).contains(&value))`
/// at 2 real sites (`rust_std::iter`'s `FlatMap` over `0..=4`,
/// `rust_std::slice`'s printable-ASCII bound over `0x20..=0x7e`) --
/// the identical range-membership precondition regardless of the
/// concrete bounds or element type. Generic over the element type
/// rather than one registration per bound, the same reasoning (and the
/// same reason it needs a hand-written `Witness`/`Requires` impl
/// instead of the `bridge_kani_witness!`/`kani_requires!` macros) as
/// `SplitOperandsAreDistinctFromThePattern` (`rust_std::slice`).
pub struct ValueIsWithinInclusiveRange<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsWithinInclusiveRange<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsWithinInclusiveRange<T> {
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
