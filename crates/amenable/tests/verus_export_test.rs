mod support;

use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use amenable::{
    ClassifiedWitness, Evidence, MetadataEntry, Provenance, Verifier, Witness, WitnessArtifact,
    WitnessArtifactMember, WitnessArtifactNode, WitnessArtifactShape, WitnessArtifactVariant,
    WitnessModulePath, WitnessSupportKind, WitnessSupportSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct LocalVerusVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LocalVerusVerifierMetadata;

impl Provenance for LocalVerusVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new("backend", "local-verus")].into_iter()
    }
}

impl Verifier for LocalVerusVerifier {
    type Metadata = LocalVerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LocalEnumEvidence;

impl Evidence for LocalEnumEvidence {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalEnumProofArtifact;

impl std::fmt::Display for LocalEnumProofArtifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "verifier: verus\nshape: enum\nsupport: {}",
            WitnessSupportSummary::compose(&[
                WitnessSupportSummary::compose(&[
                    WitnessSupportSummary::checked_leaf(),
                    WitnessSupportSummary::trusted_leaf(),
                ]),
                WitnessSupportSummary::trusted_leaf(),
                WitnessSupportSummary::trivial_leaf(),
            ])
        )
    }
}

impl WitnessModulePath for LocalEnumProofArtifact {
    const MODULE_PATH: &'static str = "crate::derived_witness::local_shape_witness";
}

impl WitnessArtifact for LocalEnumProofArtifact {
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::enum_variants(
            WitnessSupportSummary::compose(&[
                WitnessSupportSummary::compose(&[
                    WitnessSupportSummary::checked_leaf(),
                    WitnessSupportSummary::trusted_leaf(),
                ]),
                WitnessSupportSummary::trusted_leaf(),
                WitnessSupportSummary::trivial_leaf(),
            ]),
            "entry_kind",
            vec![
                WitnessArtifactVariant {
                    name: "Balanced".to_owned(),
                    artifact: Box::new(WitnessArtifactNode::members(
                        WitnessArtifactShape::NamedVariant,
                        WitnessSupportSummary::compose(&[
                            WitnessSupportSummary::checked_leaf(),
                            WitnessSupportSummary::trusted_leaf(),
                        ]),
                        Some("Balanced".to_owned()),
                        vec![
                            WitnessArtifactMember {
                                label: "checked".to_owned(),
                                artifact: Box::new(WitnessArtifactNode::leaf_with_metadata(
                                    WitnessSupportKind::Checked,
                                    WitnessSupportSummary::checked_leaf(),
                                    "harness: verify_char_roundtrip",
                                    [
                                        MetadataEntry::new("verifier", "verus"),
                                        MetadataEntry::new("harness", "verify_char_roundtrip"),
                                    ],
                                )),
                            },
                            WitnessArtifactMember {
                                label: "trusted".to_owned(),
                                artifact: Box::new(WitnessArtifactNode::leaf_with_metadata(
                                    WitnessSupportKind::Trusted,
                                    WitnessSupportSummary::trusted_leaf(),
                                    "authority: Rust Project Developers",
                                    [MetadataEntry::new("authority", "Rust Project Developers")],
                                )),
                            },
                        ],
                    )),
                },
                WitnessArtifactVariant {
                    name: "fallback".to_owned(),
                    artifact: Box::new(WitnessArtifactNode::members(
                        WitnessArtifactShape::TupleVariant,
                        WitnessSupportSummary::trusted_leaf(),
                        Some("Adjustment".to_owned()),
                        vec![WitnessArtifactMember {
                            label: "field_0".to_owned(),
                            artifact: Box::new(WitnessArtifactNode::leaf_with_metadata(
                                WitnessSupportKind::Trusted,
                                WitnessSupportSummary::trusted_leaf(),
                                "authority: Rust Project Developers",
                                [MetadataEntry::new("authority", "Rust Project Developers")],
                            )),
                        }],
                    )),
                },
                WitnessArtifactVariant {
                    name: "Closed".to_owned(),
                    artifact: Box::new(WitnessArtifactNode::members(
                        WitnessArtifactShape::UnitVariant,
                        WitnessSupportSummary::trivial_leaf(),
                        Some("Closed".to_owned()),
                        vec![],
                    )),
                },
            ],
        )
    }
}

impl Witness<LocalVerusVerifier> for LocalEnumEvidence {
    type SupportingEvidence = Self;
    type ProofArtifact = LocalEnumProofArtifact;

    fn proof() -> Self::ProofArtifact {
        LocalEnumProofArtifact
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::compose(&[
                WitnessSupportSummary::checked_leaf(),
                WitnessSupportSummary::trusted_leaf(),
            ]),
            WitnessSupportSummary::trusted_leaf(),
            WitnessSupportSummary::trivial_leaf(),
        ])
    }
}

impl ClassifiedWitness<LocalVerusVerifier> for LocalEnumEvidence {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LocalLeafEvidence;

impl Evidence for LocalLeafEvidence {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalLeafProofArtifact;

impl std::fmt::Display for LocalLeafProofArtifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "verifier: verus\nshape: leaf\nsupport: {}",
            WitnessSupportSummary::checked_leaf()
        )
    }
}

impl WitnessModulePath for LocalLeafProofArtifact {
    const MODULE_PATH: &'static str = "crate::custom::proofs::shape_override_witness";
}

impl WitnessArtifact for LocalLeafProofArtifact {
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Checked,
            WitnessSupportSummary::checked_leaf(),
            "harness: verify_shape_override",
            [MetadataEntry::new("harness", "verify_shape_override")],
        )
    }
}

impl Witness<LocalVerusVerifier> for LocalLeafEvidence {
    type SupportingEvidence = Self;
    type ProofArtifact = LocalLeafProofArtifact;

    fn proof() -> Self::ProofArtifact {
        LocalLeafProofArtifact
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<LocalVerusVerifier> for LocalLeafEvidence {}

// A local, test-only call shape for LocalLeafEvidence's harness --
// doesn't need to correspond to any real Verus source, since this test
// only inspects the renderer's generated text, never runs the real
// `verus` tool on it.
amenable_std::register_verus_call_shape! {
    harness = "verify_shape_override",
    module_path = "crate::custom::shape_override_carrier",
    params = [("value", "i32")],
    returns = "i32",
    requires = [],
    ensures = [("shape_override_holds", ["result", "value"])],
}

amenable_core::register_witness_exports!(verifier = LocalVerusVerifier; LocalEnumEvidence, LocalLeafEvidence);

fn read_file(path: &Path) -> miette::Result<String> {
    fs::read_to_string(path)
        .map_err(|error| miette::miette!("failed to read {}: {error}", path.display()))
}

/// `LocalEnumEvidence` (registered alongside `LocalLeafEvidence` above)
/// is enum-shaped, which the renderer deliberately doesn't support yet
/// (see `amenable::verus_export`'s own doc comment) -- so this call
/// returns an overall `Err` naming it. That must not stop
/// `LocalLeafEvidence`'s own, unrelated, working export from still
/// being rendered and written to disk: `write_verus_witness_modules`
/// renders every export independently and reports every failure
/// together, rather than aborting the whole batch at the first one
/// (confirmed as a real problem while building this -- `inventory`
/// registration is process-global, so a single broken registration
/// anywhere would otherwise silently starve every other, working export
/// in the same process).
#[test]
fn write_verus_witness_modules_materializes_shape_specific_modules() -> miette::Result<()> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the Unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("amenable-verus-export-test-{stamp}"));

    if root.exists() {
        fs::remove_dir_all(&root).expect("stale temp directory should be removable");
    }

    let report = support::library(amenable::write_verus_witness_modules(&root))
        .expect_err("LocalEnumEvidence's enum shape is not supported yet");
    let error_text = report.to_string();
    assert!(error_text.contains("enum"), "{error_text}");
    assert!(error_text.contains("LocalEnumEvidence"), "{error_text}");

    assert!(
        !root.join("derived_witness/local_shape_witness.rs").exists(),
        "the unsupported enum export must not produce a file"
    );

    let lib_rs = read_file(&root.join("lib.rs"))?;
    assert!(lib_rs.contains("pub mod custom;"), "{lib_rs}");

    let custom_mod = read_file(&root.join("custom/mod.rs"))?;
    assert!(custom_mod.contains("pub mod proofs;"), "{custom_mod}");

    let nested_mod = read_file(&root.join("custom/proofs/mod.rs"))?;
    assert!(
        nested_mod.contains("pub mod shape_override_witness;"),
        "{nested_mod}"
    );

    let leaf_module = read_file(&root.join("custom/proofs/shape_override_witness.rs"))?;
    assert!(
        leaf_module.contains("crate::custom::shape_override_carrier::verify_shape_override(value)"),
        "{leaf_module}"
    );
    assert!(
        leaf_module.contains("shape_override_holds(result, value)"),
        "{leaf_module}"
    );
    assert!(
        leaf_module.contains("pub fn verify_shape_override_witness(value: i32) -> (result: i32)"),
        "{leaf_module}"
    );
    assert!(
        leaf_module.contains("use crate::custom::shape_override_carrier::shape_override_holds;"),
        "{leaf_module}"
    );

    fs::remove_dir_all(&root)
        .map_err(|error| miette::miette!("failed to remove {}: {error}", root.display()))?;
    Ok(())
}
