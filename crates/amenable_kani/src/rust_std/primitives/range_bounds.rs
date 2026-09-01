use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::ascii::ValueIsWithinInclusiveRange;
use crate::CheckedProof;
use crate::KaniWitness;

impl<T> KaniWitness for ValueIsWithinInclusiveRange<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            crate::rust_std::iter::VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsWithinInclusiveRange<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier>
    for ValueIsWithinInclusiveRange<T>
{
    type Input = (T, T, T);
    type Bound = bool;

    fn requires((value, low, high): (T, T, T)) -> bool {
        low <= value && value <= high
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsWithinInclusiveRange",
        "kani",
        "requires",
        || stringify!(low <= value && value <= high),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsWithinInclusiveRange",
        "kani",
        || <ValueIsWithinInclusiveRange<i32> as KaniWitness>::proof().to_string(),
    )
}

/// The negation of [`ValueIsWithinInclusiveRange`]: a `(value, low,
/// high)` triple known to satisfy the precondition a proof over a
/// symbolic value assumes when it must fall *outside* an excluded
/// inclusive range -- e.g. a UTF-16 code unit that isn't a surrogate.
/// Generic and hand-written for the same reason.
///
/// Independently hand-written as
/// `kani::assume(!(0xD800..=0xDFFF).contains(&unit))` at 1 real site
/// (`rust_std::char`'s non-surrogate UTF-16 code unit bound).
pub struct ValueIsOutsideInclusiveRange<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsOutsideInclusiveRange<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsOutsideInclusiveRange<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsOutsideInclusiveRange<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_decode_utf16_round_trips_a_bmp_code_unit".to_owned(),
            crate::rust_std::char::VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsOutsideInclusiveRange<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier>
    for ValueIsOutsideInclusiveRange<T>
{
    type Input = (T, T, T);
    type Bound = bool;

    fn requires((value, low, high): (T, T, T)) -> bool {
        value < low || value > high
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsOutsideInclusiveRange",
        "kani",
        "requires",
        || stringify!(value < low || value > high),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsOutsideInclusiveRange",
        "kani",
        || <ValueIsOutsideInclusiveRange<i32> as KaniWitness>::proof().to_string(),
    )
}

/// A `(value, minimum)` pair known to satisfy the precondition every
/// proof over a symbolic value with a one-sided lower bound assumes:
/// the value is at least the given minimum.
///
/// Independently hand-written as `kani::assume(value >= minimum)` at 1
/// real site (`rust_std::str`'s UTF-8 lead-byte lower bound, `0xF5`) --
/// a singleton today, named for the same reason every other bound in
/// this worklist is: it makes the assumption explicit and auditable,
/// not because it's shared across multiple sites. Generic over the
/// element type and hand-written for the same reason
/// `ValueIsWithinInclusiveRange` is.
pub struct ValueIsAtLeast<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsAtLeast<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsAtLeast<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsAtLeast<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_error_reports_the_valid_prefix_length_and_error_span".to_owned(),
            crate::rust_std::str::VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type Input = (T, T);
    type Bound = bool;

    fn requires((value, minimum): (T, T)) -> bool {
        value >= minimum
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        "requires",
        || stringify!(value >= minimum),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        || <ValueIsAtLeast<i32> as KaniWitness>::proof().to_string(),
    )
}

/// The postcondition counterpart to `ValueIsAtLeast`'s own
/// `Requires` impl, same body, for real sites that assert this shape
/// as an `Ensures` claim rather than assume it as a `Requires`
/// precondition -- both directions register their own `ContractRecord`
/// (Kani's `(verifier, kind)` lookup is keyed separately for
/// `"requires"` vs `"ensures"` clauses), so one type can carry both.
impl<T: PartialOrd> amenable_core::Ensures<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((value, minimum): (T, T)) -> bool {
        value >= minimum
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        "ensures",
        || stringify!(value >= minimum),
    )
}

/// A `(value, bound)` pair known to satisfy the precondition every
/// proof over a symbolic value with a one-sided upper bound assumes:
/// the value is strictly below the given bound. The mirror image of
/// `ValueIsAtLeast`, generic and hand-written for the same reason.
///
/// Independently hand-written as `kani::assume((c as u32) < 0x10000)`
/// at 1 real site (`os_windows_model`'s BMP-character bound for
/// `EncodeWide`).
pub struct ValueIsBelow<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsBelow<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsBelow<T> {
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

impl<T> KaniWitness for ValueIsBelow<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_encode_wide_encodes_a_bmp_char_as_one_code_unit".to_owned(),
            crate::os_windows_model::VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CHAR_AS_ONE_CODE_UNIT_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsBelow<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier> for ValueIsBelow<T> {
    type Input = (T, T);
    type Bound = bool;

    fn requires((value, bound): (T, T)) -> bool {
        value < bound
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsBelow",
        "kani",
        "requires",
        || stringify!(value < bound),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsBelow",
        "kani",
        || <ValueIsBelow<i32> as KaniWitness>::proof().to_string(),
    )
}
