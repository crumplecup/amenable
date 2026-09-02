use amenable_core::{
    Evidence, MetadataEntry, Provenance, Standard, Verifier, Witness, WitnessSupportSummary,
};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use strum::EnumIter;

pub trait FixtureCase:
    Clone
    + Default
    + PartialEq
    + Eq
    + std::fmt::Debug
    + Provenance
    + Standard<Provenance = Self>
    + Evidence<Basis = Self, Audit = Self>
    + 'static
{
    const KIND: DeriveFixtureKind;

    fn instances() -> Vec<FixtureInstance<Self>>;

    fn expected_support() -> WitnessSupportSummary;
}

pub trait FixtureWitnessMember:
    Clone
    + Default
    + PartialEq
    + Eq
    + std::fmt::Debug
    + Provenance
    + Standard<Provenance = Self>
    + Evidence<Basis = Self, Audit = Self>
    + Witness<FixtureVerifier>
    + 'static
{
}

#[derive(Debug, Clone, PartialEq, Eq, Default, StandardDerive)]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct WitnessLeaf(String);

impl WitnessLeaf {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Provenance for WitnessLeaf {
    type MetadataIter = <String as Provenance>::MetadataIter;

    fn metadata(&self) -> Self::MetadataIter {
        self.0.metadata()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessLeafProof {
    pub evidence: String,
}

impl std::fmt::Display for WitnessLeafProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "leaf: {}", self.evidence)
    }
}

pub struct FixtureVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FixtureVerifierMetadata;

impl Provenance for FixtureVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new("verifier", "fixture")].into_iter()
    }
}

impl Verifier for FixtureVerifier {
    type Metadata = FixtureVerifierMetadata;

    fn name() -> &'static str {
        "fixture"
    }
}

impl Witness<FixtureVerifier> for WitnessLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = WitnessLeafProof;

    fn proof() -> Self::ProofArtifact {
        WitnessLeafProof {
            evidence: std::any::type_name::<Self>().to_owned(),
        }
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum DeriveFixtureKind {
    UnitStruct,
    NamedStruct,
    TupleStruct,
    CheckedPlusTrivialStruct,
    UnitEnum,
    NamedEnum,
    TupleEnum,
    NestedStruct,
    NestedTupleStruct,
    InstantiatedGenericStruct,
    InstantiatedGenericTupleStruct,
    InstantiatedGenericEnum,
}

pub struct FixtureInstance<F> {
    pub label: String,
    pub value: F,
    pub expected_entries: Vec<(String, String)>,
}

/// Convert a literal `(&str, &str)` array into the owned pairs
/// `FixtureInstance::expected_entries` stores.
fn owned_entries(entries: &[(&str, &str)]) -> Vec<(String, String)> {
    entries
        .iter()
        .map(|&(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
}

impl<T> FixtureWitnessMember for T where
    T: Clone
        + Default
        + PartialEq
        + Eq
        + std::fmt::Debug
        + Provenance
        + Standard<Provenance = Self>
        + Evidence<Basis = Self, Audit = Self>
        + Witness<FixtureVerifier>
        + 'static
{
}

pub fn expected_report(entries: &[(String, String)]) -> String {
    if entries.is_empty() {
        return "(no provenance metadata)".to_string();
    }

    entries
        .iter()
        .map(|(key, value)| format!("{key}: {value}"))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn expected_keys(entries: &[(String, String)]) -> Vec<String> {
    entries.iter().map(|(key, _)| key.clone()).collect()
}

pub fn expected_values(entries: &[(String, String)]) -> Vec<String> {
    entries.iter().map(|(_, value)| value.clone()).collect()
}

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

macro_rules! for_each_fixture_type {
    ($callback:ident) => {
        $callback!(crate::support::UnitStructFixture);
        $callback!(crate::support::NamedStructFixture);
        $callback!(crate::support::TupleStructFixture);
        $callback!(crate::support::CheckedPlusTrivialStructFixture);
        $callback!(crate::support::UnitEnumFixture);
        $callback!(crate::support::NamedEnumFixture);
        $callback!(crate::support::TupleEnumFixture);
        $callback!(crate::support::NestedStructFixture);
        $callback!(crate::support::NestedTupleStructFixture);
        $callback!(crate::support::ConcreteGenericStructFixture);
        $callback!(crate::support::ConcreteGenericTupleStructFixture);
        $callback!(crate::support::ConcreteGenericEnumFixture);
    };
}

pub(crate) use for_each_fixture_type;
