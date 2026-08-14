use amenable_core::{Provenance, WitnessSupportSummary};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct DerivedWitnessCheckedPlusTrivialStruct<
    TChecked: Provenance + Clone + Default,
    TTrivial: Provenance + Clone + Default,
> {
    pub checked: TChecked,
    pub marker: TTrivial,
}

impl<TChecked: Provenance + Clone + Default, TTrivial: Provenance + Clone + Default>
    DerivedWitnessCheckedPlusTrivialStruct<TChecked, TTrivial>
{
    pub fn new(checked: TChecked) -> Self {
        Self {
            checked,
            marker: TTrivial::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub enum DerivedWitnessGenericEnum<
    TChecked: Provenance + Clone,
    TTrusted: Provenance + Clone,
    TSkipped: Provenance + Clone,
> {
    Balanced {
        checked: TChecked,
        trusted: TTrusted,
    },
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] TSkipped,
    ),
    #[default]
    Closed,
}

impl<TChecked: Provenance + Clone, TTrusted: Provenance + Clone, TSkipped: Provenance + Clone>
    DerivedWitnessGenericEnum<TChecked, TTrusted, TSkipped>
{
    pub fn balanced(checked: TChecked, trusted: TTrusted) -> Self {
        Self::Balanced { checked, trusted }
    }

    pub fn adjustment(trusted: TTrusted, skipped_checked: TSkipped) -> Self {
        Self::Adjustment(trusted, skipped_checked)
    }
}

pub fn checked_plus_trivial_support() -> WitnessSupportSummary {
    WitnessSupportSummary::compose(&[
        WitnessSupportSummary::checked_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ])
}

pub fn balanced_variant_support() -> WitnessSupportSummary {
    WitnessSupportSummary::compose(&[
        WitnessSupportSummary::checked_leaf(),
        WitnessSupportSummary::trusted_leaf(),
    ])
}

pub fn mixed_support() -> WitnessSupportSummary {
    WitnessSupportSummary::compose(&[
        balanced_variant_support(),
        WitnessSupportSummary::trusted_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ])
}

pub fn assert_generic_enum_report(
    proof_type: &str,
    report: &str,
    closed_report: &str,
    verifier_label: &str,
    expected_support: WitnessSupportSummary,
) {
    assert!(
        proof_type.contains("DerivedWitnessGenericEnumWitnessProof"),
        "{proof_type}"
    );
    assert!(
        report.contains(&format!("verifier: {verifier_label}")),
        "{report}"
    );
    assert!(report.contains("shape: enum"), "{report}");
    assert!(
        report.contains(&format!("support: {expected_support}")),
        "{report}"
    );
    assert!(report.contains("tag: entry_kind"), "{report}");
    assert!(
        report.contains("variant Balanced: shape: named_variant"),
        "{report}"
    );
    assert!(
        report.contains("member checked: harness: verify_char_roundtrip"),
        "{report}"
    );
    assert!(
        report.contains("variant fallback: shape: tuple_variant"),
        "{report}"
    );
    assert!(
        closed_report.contains("shape: unit_variant"),
        "{closed_report}"
    );
}

pub fn assert_checked_plus_trivial_report(
    report: &str,
    verifier_label: &str,
    expected_support: WitnessSupportSummary,
) {
    assert!(
        report.contains(&format!("verifier: {verifier_label}")),
        "{report}"
    );
    assert!(report.contains("shape: named_struct"), "{report}");
    assert!(
        report.contains(&format!("support: {expected_support}")),
        "{report}"
    );
    assert!(
        report.contains("member checked: harness: verify_char_roundtrip"),
        "{report}"
    );
    assert!(
        report.contains(&format!("member marker: verifier: {verifier_label}")),
        "{report}"
    );
}
