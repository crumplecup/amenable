#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::array_slice_str::VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC;
use super::tuple_fn_ptr::VERIFY_TUPLE_FIELD_ACCESS_SRC;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "kani",
        || <RustStdStandard<&'static i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// Dereferencing a shared reference recovers exactly the value
        /// it borrows. `Box::leak` gives a genuinely `'static` reference
        /// to symbolic heap data without needing a `const`/`static` item
        /// (which can't hold a `kani::any()` value) -- ordinary safe
        /// Rust, not a workaround for anything unsafe. Calls
        /// `DerefReflectsTheStoredValue::ensures` directly rather than
        /// restating the comparison -- see that type for why this is the
        /// one harness its registration reuses as a witness.
        #[kani::proof]
        fn verify_shared_reference_dereferences_to_the_referent() {
            let value: i32 = kani::any();
            let leaked: &'static i32 = Box::leak(Box::new(value));
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, value)),
                "dereferencing recovers the referent"
            );
        }
    }
}

/// A `(dereferenced, expected)` pair known to agree: dereferencing a
/// smart pointer, guard, or reference recovers exactly the value stored
/// in (or borrowed by) it.
///
/// Independently hand-written as `assert_eq!(*wrapper, expected, ...)` at
/// 28 real sites spanning `Cow`, `Box`, `BinaryHeap::PeekMut`, `Rc`,
/// `Arc`, `RefCell`'s `Ref`/`RefMut`, `ManuallyDrop`, `Option`/`Result`'s
/// `IterMut`, `AssertUnwindSafe`, `Pin<Box<_>>`, shared/mutable
/// references, `slice::IterMut`, and `Mutex`/`RwLock`'s guards -- the
/// identical claim regardless of which wrapper type derefs. Generic over
/// the pointee type rather than one registration per wrapper type, the
/// same reasoning (and the same reason it needs a hand-written
/// `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `IteratorYieldsNoneWhenExhausted` in `rust_std::iter` and
/// `AtomicLoadReflectsTheLastWrite` in `rust_std::sync_atomic`.
pub struct DerefReflectsTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for DerefReflectsTheStoredValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for DerefReflectsTheStoredValue<T> {
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

impl<T> KaniWitness for DerefReflectsTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for DerefReflectsTheStoredValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier> for DerefReflectsTheStoredValue<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((dereferenced, expected): (T, T)) -> bool {
        dereferenced == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::DerefReflectsTheStoredValue",
        "kani",
        "ensures",
        || stringify!(dereferenced == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::DerefReflectsTheStoredValue",
        "kani",
        || <DerefReflectsTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    )
}

/// An `(actual, expected)` pair known to agree: indexing a fixed-length
/// container at a position recovers exactly the element known to be
/// stored there.
///
/// Independently hand-written as `assert_eq!(container[i], expected,
/// ...)` at 9 real sites spanning `Vec<i32>`, a `[u8; 4]` array indexed
/// through `IoSliceMut`, `[i32; 3]`/`[i32; 1]` arrays, and a `&[i32]`
/// slice -- the identical claim regardless of container kind or element
/// type. This is the Kani-side sibling of
/// `amenable_std::IndexingAndLength`'s Creusot postcondition, not a
/// reuse of that type directly: `IndexingAndLength` is a fixed,
/// non-generic wrapper bundling a length check together with three
/// specific indices in one Pearlite predicate, which cannot vary its
/// `Input` type per real site's element type the way a Kani `Ensures`
/// impl needs to (`i32` here, `u8` for the `IoSliceMut` site). Generic
/// over the element type instead, same reasoning (and the same reason
/// it needs a hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `DerefReflectsTheStoredValue` just above.
pub struct IndexRecoversTheStoredElement<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IndexRecoversTheStoredElement<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IndexRecoversTheStoredElement<T> {
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

impl<T> KaniWitness for IndexRecoversTheStoredElement<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_owned(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IndexRecoversTheStoredElement<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for IndexRecoversTheStoredElement<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::IndexRecoversTheStoredElement",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IndexRecoversTheStoredElement",
        "kani",
        || <IndexRecoversTheStoredElement<i32> as KaniWitness>::proof().to_string(),
    )
}

/// An `(actual, expected)` pair known to agree: a struct or tuple
/// field access recovers exactly the value known to be stored there.
///
/// Independently hand-written as `assert_eq!(value.field, expected,
/// ...)` at 5 real sites: `verify_tuple_field_access` (`(a, b)`'s `.0`/
/// `.1` projections, 2 sites), `calculator::Debit`/`Credit`'s own
/// `.value` field access constructors (2 sites), and
/// `verify_assert_unwind_safe_derefs_transparently`'s `.0` projection
/// after a `DerefMut` write-through (1 site) -- the identical claim
/// regardless of whether the access is a named field or a tuple index.
/// A distinct access pattern from `IndexRecoversTheStoredElement`
/// (`[i]`) and `DerefReflectsTheStoredValue` (`*x`) even though the
/// `Ensures` impl body is identical trivial equality either way --
/// same reasoning as keeping `CollectedSequenceMatchesExpected`
/// separate from `DerefReflectsTheStoredValue` despite type-level
/// overlap.
pub struct FieldAccessRecoversTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for FieldAccessRecoversTheStoredValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for FieldAccessRecoversTheStoredValue<T> {
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

impl<T> KaniWitness for FieldAccessRecoversTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tuple_field_access".to_owned(),
            VERIFY_TUPLE_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for FieldAccessRecoversTheStoredValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for FieldAccessRecoversTheStoredValue<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::FieldAccessRecoversTheStoredValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FieldAccessRecoversTheStoredValue",
        "kani",
        || <FieldAccessRecoversTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mutable_reference_dereferences_to_and_updates_the_referent".to_owned(),
            VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "kani",
        || <RustStdStandard<&'static mut i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// Dereferencing a mutable reference recovers the value it
        /// borrows, and writing through it updates the referent visibly
        /// through the same reference.
        #[kani::proof]
        fn verify_mutable_reference_dereferences_to_and_updates_the_referent() {
            let initial: i32 = kani::any();
            let next: i32 = kani::any();
            let leaked: &'static mut i32 = Box::leak(Box::new(initial));
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, initial)),
                "dereferencing recovers the referent"
            );
            *leaked = next;
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, next)),
                "writing through the reference updates the referent"
            );
        }
    }
}
