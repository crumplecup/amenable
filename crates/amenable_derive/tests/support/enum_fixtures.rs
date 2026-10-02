use amenable_core::WitnessSupportSummary;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use super::shared::{DeriveFixtureKind, FixtureCase, FixtureInstance, WitnessLeaf, owned_entries};

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "authority_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub enum UnitEnumFixture {
    #[default]
    InternalOnly,
    ExternalStandard,
}

impl UnitEnumFixture {
    pub fn external_standard() -> Self {
        Self::ExternalStandard
    }
}

impl FixtureCase for UnitEnumFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::UnitEnum;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "internal_only".to_owned(),
                value: Self::InternalOnly,
                expected_entries: owned_entries(&[("authority_kind", "InternalOnly")]),
            },
            FixtureInstance {
                label: "external_standard".to_owned(),
                value: Self::external_standard(),
                expected_entries: owned_entries(&[("authority_kind", "ExternalStandard")]),
            },
        ]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::trivial_leaf(),
            WitnessSupportSummary::trivial_leaf(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "authority_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub enum NamedEnumFixture {
    RustProject {
        authority: WitnessLeaf,
        source_url: WitnessLeaf,
    },
    #[provenance(rename = "local_design")]
    Local { owner: WitnessLeaf },
    #[default]
    InternalOnly,
}

impl NamedEnumFixture {
    pub fn rust_project(authority: impl Into<String>, source_url: impl Into<String>) -> Self {
        Self::RustProject {
            authority: WitnessLeaf::new(authority),
            source_url: WitnessLeaf::new(source_url),
        }
    }

    pub fn local(owner: impl Into<String>) -> Self {
        Self::Local {
            owner: WitnessLeaf::new(owner),
        }
    }
}

impl FixtureCase for NamedEnumFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::NamedEnum;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "rust_project".to_owned(),
                value: Self::rust_project(
                    "Rust Project Developers",
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
                value: Self::local("UI Working Group"),
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "authority_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub enum TupleEnumFixture {
    RustProject(#[provenance(rename = "authority")] WitnessLeaf, WitnessLeaf),
    #[provenance(rename = "local_design")]
    Local(
        #[provenance(rename = "owner")] WitnessLeaf,
        #[provenance(skip)] WitnessLeaf,
    ),
    #[default]
    InternalOnly,
}

impl TupleEnumFixture {
    pub fn rust_project(authority: impl Into<String>, source_url: impl Into<String>) -> Self {
        Self::RustProject(WitnessLeaf::new(authority), WitnessLeaf::new(source_url))
    }

    pub fn local(owner: impl Into<String>, internal_note: impl Into<String>) -> Self {
        Self::Local(WitnessLeaf::new(owner), WitnessLeaf::new(internal_note))
    }
}

impl FixtureCase for TupleEnumFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::TupleEnum;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "rust_project".to_owned(),
                value: Self::rust_project(
                    "Rust Project Developers",
                    "https://doc.rust-lang.org/std/primitive.i32.html",
                ),
                expected_entries: owned_entries(&[
                    ("authority_kind", "RustProject"),
                    ("authority", "Rust Project Developers"),
                    ("1", "https://doc.rust-lang.org/std/primitive.i32.html"),
                ]),
            },
            FixtureInstance {
                label: "local".to_owned(),
                value: Self::local("UI Working Group", "not for metadata projection"),
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
