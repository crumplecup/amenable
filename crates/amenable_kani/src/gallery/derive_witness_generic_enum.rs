//! Gallery canary for `#[derive(Witness)]` on concretely instantiated generic enums.
//!
//! This isolates the Kani-facing question for the derive work:
//!
//! - can a generic enum derive `Witness` once its concrete members already
//!   carry Kani witnesses?
//! - does the generated proof artifact preserve the enclosing enum shape,
//!   rather than collapsing to an unstructured bag of child proofs?
//!
//! The fixture intentionally mixes named, tuple, and unit variants so one
//! explicit instantiation exercises the full enum surface.

#[cfg(kani)]
use amenable_core::Witness;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_core::Provenance;

type ConcreteDerivedWitnessEnum = DerivedWitnessGenericEnum<crate::Debit, crate::Credit>;

#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive,
)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
enum DerivedWitnessGenericEnum<TDebit: Provenance + Clone, TAdjustment: Provenance + Clone> {
    Balanced {
        debit: TDebit,
        credit: crate::Credit,
    },
    #[provenance(rename = "adjustment")]
    Adjustment(
        #[provenance(rename = "credit")] TAdjustment,
        #[provenance(skip)] crate::Credit,
    ),
    #[default]
    Closed,
}

impl<TDebit: Provenance + Clone, TAdjustment: Provenance + Clone>
    DerivedWitnessGenericEnum<TDebit, TAdjustment>
{
    fn balanced(debit: TDebit) -> Self {
        Self::Balanced {
            debit,
            credit: crate::Credit::new(1),
        }
    }

    fn adjustment(credit: TAdjustment) -> Self {
        Self::Adjustment(credit, crate::Credit::new(2))
    }
}

fn concrete_variants() -> (ConcreteDerivedWitnessEnum, ConcreteDerivedWitnessEnum) {
    (
        ConcreteDerivedWitnessEnum::balanced(crate::Debit::new(3)),
        ConcreteDerivedWitnessEnum::adjustment(crate::Credit::new(4)),
    )
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || {
            let _ = concrete_variants();

            ::amenable_kani::KaniGalleryCase {
                id: "amenable_kani::gallery::derive_witness_generic_enum::derive_witness_supports_concrete_generic_enums".to_owned(),
                harness: "gallery::derive_witness_generic_enum::derive_witness_supports_concrete_generic_enums".to_owned(),
                package: "amenable_kani".to_owned(),
                title: "derive(Witness) builds a structured Kani artifact for a concrete generic enum".to_owned(),
                disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
                expected: ::amenable_kani::KaniGalleryExpectation::Passed,
            }
        },
    }
}

amenable_derive::harness! {
    kani, DERIVE_WITNESS_SUPPORTS_CONCRETE_GENERIC_ENUMS_SRC, {
        /// One concrete generic-enum instantiation is enough for Kani to
        /// check the derive strategy: the outer proof artifact exists as
        /// one nominal enum proof, each reachable member proof comes from
        /// the correct leaf witness, and the unit variant still appears in
        /// the enclosing shape even though it carries no child witnesses.
        #[kani::proof]
        fn derive_witness_supports_concrete_generic_enums() {
            let _ = concrete_variants();

            let proof = <ConcreteDerivedWitnessEnum as Witness<crate::KaniVerifier>>::proof();
            let balanced = proof.variant_balanced;
            let adjustment = proof.variant_adjustment;
            let closed = proof.variant_closed;
            let balanced_size = core::mem::size_of_val(&balanced);
            let adjustment_size = core::mem::size_of_val(&adjustment);
            let closed_size = core::mem::size_of_val(&closed);

            let crate::CalculationProof {
                harness: debit_harness,
                claim: debit_claim,
            } = balanced.debit;
            let crate::CalculationProof {
                harness: named_credit_harness,
                claim: named_credit_claim,
            } = balanced.credit;
            let crate::CalculationProof {
                harness: tuple_credit_harness,
                claim: tuple_credit_claim,
            } = adjustment.field_0;

            let _ = (
                debit_harness,
                debit_claim,
                named_credit_harness,
                named_credit_claim,
                tuple_credit_harness,
                tuple_credit_claim,
            );

            assert!(
                balanced_size > adjustment_size,
                "the named variant carries more witness structure than the tuple variant"
            );
            assert!(
                adjustment_size > closed_size,
                "the tuple variant carries more witness structure than the unit variant"
            );
        }
    }
}
