use amenable_core::WitnessSupportSummary;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use super::shared::{DeriveFixtureKind, FixtureCase, FixtureInstance, WitnessLeaf, owned_entries};

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct UnitStructFixture;

impl FixtureCase for UnitStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::UnitStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "unit".to_owned(),
            value: Self,
            expected_entries: owned_entries(&[]),
        }]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::trivial_leaf()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct NamedStructFixture {
    authority: WitnessLeaf,
    #[provenance(rename = "decision_id")]
    design_decision: WitnessLeaf,
    #[provenance(skip)]
    internal_note: WitnessLeaf,
}

impl NamedStructFixture {
    pub fn new(
        authority: impl Into<String>,
        design_decision: impl Into<String>,
        internal_note: impl Into<String>,
    ) -> Self {
        Self {
            authority: WitnessLeaf::new(authority),
            design_decision: WitnessLeaf::new(design_decision),
            internal_note: WitnessLeaf::new(internal_note),
        }
    }
}

impl FixtureCase for NamedStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::NamedStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "named".to_owned(),
            value: Self::new(
                "UI Working Group",
                "layout-12",
                "not for metadata projection",
            ),
            expected_entries: owned_entries(&[
                ("authority", "UI Working Group"),
                ("decision_id", "layout-12"),
            ]),
        }]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::checked_leaf(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct TupleStructFixture(
    #[provenance(rename = "authority")] WitnessLeaf,
    WitnessLeaf,
    #[provenance(skip)] WitnessLeaf,
);

impl TupleStructFixture {
    pub fn new(
        authority: impl Into<String>,
        design_decision: impl Into<String>,
        internal_note: impl Into<String>,
    ) -> Self {
        Self(
            WitnessLeaf::new(authority),
            WitnessLeaf::new(design_decision),
            WitnessLeaf::new(internal_note),
        )
    }
}

impl FixtureCase for TupleStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::TupleStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "tuple".to_owned(),
            value: Self::new(
                "UI Working Group",
                "layout-12",
                "not for metadata projection",
            ),
            expected_entries: owned_entries(&[
                ("authority", "UI Working Group"),
                ("1", "layout-12"),
            ]),
        }]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::checked_leaf(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct CheckedPlusTrivialStructFixture {
    authority: WitnessLeaf,
    marker: UnitStructFixture,
}

impl CheckedPlusTrivialStructFixture {
    pub fn new(authority: impl Into<String>) -> Self {
        Self {
            authority: WitnessLeaf::new(authority),
            marker: UnitStructFixture,
        }
    }
}

impl FixtureCase for CheckedPlusTrivialStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::CheckedPlusTrivialStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "checked_plus_trivial".to_owned(),
            value: Self::new("UI Working Group"),
            expected_entries: owned_entries(&[("authority", "UI Working Group")]),
        }]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::trivial_leaf(),
        ])
    }
}
