use amenable_core::WitnessSupportSummary;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use super::shared::{
    DeriveFixtureKind, FixtureCase, FixtureInstance, FixtureWitnessMember, WitnessLeaf,
    owned_entries,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct GenericStructFixture<TAuthority: FixtureWitnessMember, TDecision: FixtureWitnessMember> {
    authority: TAuthority,
    #[provenance(rename = "decision_id")]
    design_decision: TDecision,
}

impl<TAuthority: FixtureWitnessMember, TDecision: FixtureWitnessMember>
    GenericStructFixture<TAuthority, TDecision>
{
    pub fn new(authority: TAuthority, design_decision: TDecision) -> Self {
        Self {
            authority,
            design_decision,
        }
    }
}

impl FixtureCase for GenericStructFixture<WitnessLeaf, WitnessLeaf> {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::InstantiatedGenericStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "generic_named".to_owned(),
            value: Self::new(
                WitnessLeaf::new("UI Working Group"),
                WitnessLeaf::new("layout-12"),
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
pub struct GenericTupleStructFixture<
    TAuthority: FixtureWitnessMember,
    TDecision: FixtureWitnessMember,
>(
    #[provenance(rename = "authority")] TAuthority,
    #[provenance(rename = "decision_id")] TDecision,
);

impl<TAuthority: FixtureWitnessMember, TDecision: FixtureWitnessMember>
    GenericTupleStructFixture<TAuthority, TDecision>
{
    pub fn new(authority: TAuthority, design_decision: TDecision) -> Self {
        Self(authority, design_decision)
    }
}

impl FixtureCase for GenericTupleStructFixture<WitnessLeaf, WitnessLeaf> {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::InstantiatedGenericTupleStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![FixtureInstance {
            label: "generic_tuple".to_owned(),
            value: Self::new(
                WitnessLeaf::new("UI Working Group"),
                WitnessLeaf::new("layout-12"),
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
#[provenance(crate = "amenable_core", tag = "authority_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub enum GenericEnumFixture<TAuthority: FixtureWitnessMember, TOwner: FixtureWitnessMember> {
    RustProject {
        authority: TAuthority,
        source_url: WitnessLeaf,
    },
    #[provenance(rename = "local_design")]
    Local(
        #[provenance(rename = "owner")] TOwner,
        #[provenance(skip)] WitnessLeaf,
    ),
    #[default]
    InternalOnly,
}

impl<TAuthority: FixtureWitnessMember, TOwner: FixtureWitnessMember>
    GenericEnumFixture<TAuthority, TOwner>
{
    pub fn rust_project(authority: TAuthority, source_url: impl Into<String>) -> Self {
        Self::RustProject {
            authority,
            source_url: WitnessLeaf::new(source_url),
        }
    }

    pub fn local(owner: TOwner, internal_note: impl Into<String>) -> Self {
        Self::Local(owner, WitnessLeaf::new(internal_note))
    }
}

impl FixtureCase for GenericEnumFixture<WitnessLeaf, WitnessLeaf> {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::InstantiatedGenericEnum;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "rust_project".to_owned(),
                value: Self::rust_project(
                    WitnessLeaf::new("Rust Project Developers"),
                    "https://doc.rust-lang.org/std/primitive.i32.html",
                ),
                expected_entries: owned_entries(&[
                    ("authority_kind", "RustProject"),
                    ("authority", "Rust Project Developers"),
                    (
                        "source_url",
                        "https://doc.rust-lang.org/std/primitive.i32.html",
                    ),
                ]),
            },
            FixtureInstance {
                label: "local".to_owned(),
                value: Self::local(
                    WitnessLeaf::new("UI Working Group"),
                    "not for metadata projection",
                ),
                expected_entries: owned_entries(&[
                    ("authority_kind", "local_design"),
                    ("owner", "UI Working Group"),
                ]),
            },
            FixtureInstance {
                label: "internal_only".to_owned(),
                value: Self::InternalOnly,
                expected_entries: owned_entries(&[("authority_kind", "InternalOnly")]),
            },
        ]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::compose(&[
                WitnessSupportSummary::checked_leaf(),
                WitnessSupportSummary::checked_leaf(),
            ]),
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::trivial_leaf(),
        ])
    }
}

pub type ConcreteGenericStructFixture = GenericStructFixture<WitnessLeaf, WitnessLeaf>;
pub type ConcreteGenericTupleStructFixture = GenericTupleStructFixture<WitnessLeaf, WitnessLeaf>;
pub type ConcreteGenericEnumFixture = GenericEnumFixture<WitnessLeaf, WitnessLeaf>;
