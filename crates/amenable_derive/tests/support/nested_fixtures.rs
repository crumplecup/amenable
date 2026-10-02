use amenable_core::WitnessSupportSummary;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use super::enum_fixtures::{NamedEnumFixture, TupleEnumFixture};
use super::shared::{DeriveFixtureKind, FixtureCase, FixtureInstance, WitnessLeaf, owned_entries};

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct NestedStructFixture {
    authority_source: NamedEnumFixture,
    semantic_summary: WitnessLeaf,
}

impl NestedStructFixture {
    pub fn new(authority_source: NamedEnumFixture, semantic_summary: impl Into<String>) -> Self {
        Self {
            authority_source,
            semantic_summary: WitnessLeaf::new(semantic_summary),
        }
    }
}

impl FixtureCase for NestedStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::NestedStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "nested_local".to_owned(),
                value: Self::new(
                    NamedEnumFixture::local("UI Working Group"),
                    "Layout invariants are selected by the application author.",
                ),
                expected_entries: owned_entries(&[
                    ("authority_source.authority_kind", "local_design"),
                    ("authority_source.owner", "UI Working Group"),
                    (
                        "semantic_summary",
                        "Layout invariants are selected by the application author.",
                    ),
                ]),
            },
            FixtureInstance {
                label: "nested_rust_project".to_owned(),
                value: Self::new(
                    NamedEnumFixture::rust_project(
                        "Rust Project Developers",
                        "https://doc.rust-lang.org/std/primitive.i32.html",
                    ),
                    "Layout invariants defer to the upstream primitive contract.",
                ),
                expected_entries: owned_entries(&[
                    ("authority_source.authority_kind", "RustProject"),
                    ("authority_source.authority", "Rust Project Developers"),
                    (
                        "authority_source.source_url",
                        "https://doc.rust-lang.org/std/primitive.i32.html",
                    ),
                    (
                        "semantic_summary",
                        "Layout invariants defer to the upstream primitive contract.",
                    ),
                ]),
            },
        ]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            NamedEnumFixture::expected_support(),
            WitnessSupportSummary::checked_leaf(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct NestedTupleStructFixture(
    #[provenance(rename = "authority_source")] TupleEnumFixture,
    #[provenance(rename = "semantic_summary")] WitnessLeaf,
);

impl NestedTupleStructFixture {
    pub fn new(authority_source: TupleEnumFixture, semantic_summary: impl Into<String>) -> Self {
        Self(authority_source, WitnessLeaf::new(semantic_summary))
    }
}

impl FixtureCase for NestedTupleStructFixture {
    const KIND: DeriveFixtureKind = DeriveFixtureKind::NestedTupleStruct;

    fn instances() -> Vec<FixtureInstance<Self>> {
        vec![
            FixtureInstance {
                label: "nested_local".to_owned(),
                value: Self::new(
                    TupleEnumFixture::local("UI Working Group", "not for metadata projection"),
                    "Layout invariants are selected by the application author.",
                ),
                expected_entries: owned_entries(&[
                    ("authority_source.authority_kind", "local_design"),
                    ("authority_source.owner", "UI Working Group"),
                    (
                        "semantic_summary",
                        "Layout invariants are selected by the application author.",
                    ),
                ]),
            },
            FixtureInstance {
                label: "nested_rust_project".to_owned(),
                value: Self::new(
                    TupleEnumFixture::rust_project(
                        "Rust Project Developers",
                        "https://doc.rust-lang.org/std/primitive.i32.html",
                    ),
                    "Layout invariants defer to the upstream primitive contract.",
                ),
                expected_entries: owned_entries(&[
                    ("authority_source.authority_kind", "RustProject"),
                    ("authority_source.authority", "Rust Project Developers"),
                    (
                        "authority_source.1",
                        "https://doc.rust-lang.org/std/primitive.i32.html",
                    ),
                    (
                        "semantic_summary",
                        "Layout invariants defer to the upstream primitive contract.",
                    ),
                ]),
            },
        ]
    }

    fn expected_support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            TupleEnumFixture::expected_support(),
            WitnessSupportSummary::checked_leaf(),
        ])
    }
}
