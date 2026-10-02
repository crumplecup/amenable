use amenable_core::{
    Evidence, Metadata, OwnedEntry, Provenance, Standard, Verifier, Witness, WitnessSupportSummary,
};
use amenable_derive::Standard as StandardDerive;
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

impl Metadata for WitnessLeaf {
    fn snapshot(&self) -> Vec<OwnedEntry> {
        self.0.snapshot()
    }
}

impl Provenance for WitnessLeaf {}

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

impl Metadata for FixtureVerifierMetadata {
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![OwnedEntry::new("verifier", "fixture")]
    }
}

impl Provenance for FixtureVerifierMetadata {}

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
pub(super) fn owned_entries(entries: &[(&str, &str)]) -> Vec<(String, String)> {
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
        return "(no metadata)".to_string();
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
