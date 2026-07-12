use amenable_core::{Provenance as ProvenanceTrait, Registry, Standard};
use amenable_derive::Provenance;
use amenable_std::{
    CertRegistry, RustStdProvenance, RustStdStandard, RustStdType,
    write_rust_std_certificate_artifacts,
};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Provenance)]
#[provenance(crate = "amenable_core")]
struct LocalDesignProvenance {
    authority: String,
    #[provenance(rename = "decision_id")]
    design_decision: String,
    #[provenance(skip)]
    internal_note: String,
}

impl LocalDesignProvenance {
    fn new(
        authority: impl Into<String>,
        design_decision: impl Into<String>,
        internal_note: impl Into<String>,
    ) -> Self {
        Self {
            authority: authority.into(),
            design_decision: design_decision.into(),
            internal_note: internal_note.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Provenance)]
#[provenance(crate = "amenable_core", tag = "authority_kind")]
enum AuthoritySource {
    RustProject {
        authority: String,
        source_url: String,
    },
    #[provenance(rename = "local_design")]
    Local {
        owner: String,
    },
    InternalOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Provenance)]
#[provenance(crate = "amenable_core")]
struct NestedAuthorityProvenance {
    authority_source: AuthoritySource,
    semantic_summary: String,
}

#[test]
fn derived_struct_provenance_projects_named_fields() {
    let provenance = LocalDesignProvenance::new(
        "UI Working Group",
        "layout-12",
        "not for metadata projection",
    );

    assert_eq!(
        provenance.keys().collect::<Vec<_>>(),
        vec!["authority", "decision_id"]
    );
    assert_eq!(
        provenance.values().collect::<Vec<_>>(),
        vec!["UI Working Group", "layout-12"]
    );
}

#[test]
fn derived_enum_provenance_emits_tag_and_payload() {
    let provenance = AuthoritySource::RustProject {
        authority: "Rust Project Developers".to_string(),
        source_url: "https://doc.rust-lang.org/std/primitive.i32.html".to_string(),
    };

    assert_eq!(
        provenance.keys().collect::<Vec<_>>(),
        vec!["authority_kind", "authority", "source_url"]
    );
    assert_eq!(
        provenance.values().collect::<Vec<_>>(),
        vec![
            "RustProject",
            "Rust Project Developers",
            "https://doc.rust-lang.org/std/primitive.i32.html",
        ]
    );
}

#[test]
fn derived_struct_namespaces_nested_provenance_metadata() {
    let provenance = NestedAuthorityProvenance {
        authority_source: AuthoritySource::Local {
            owner: "UI Working Group".to_string(),
        },
        semantic_summary: "Layout invariants are selected by the application author.".to_string(),
    };

    assert_eq!(
        provenance.keys().collect::<Vec<_>>(),
        vec![
            "authority_source.authority_kind",
            "authority_source.owner",
            "semantic_summary",
        ]
    );
    assert_eq!(
        provenance.values().collect::<Vec<_>>(),
        vec![
            "local_design",
            "UI Working Group",
            "Layout invariants are selected by the application author.",
        ]
    );
}

#[test]
fn derived_unit_variant_emits_only_the_tag_entry() {
    let provenance = AuthoritySource::InternalOnly;

    assert_eq!(
        provenance.keys().collect::<Vec<_>>(),
        vec!["authority_kind"]
    );
    assert_eq!(
        provenance.values().collect::<Vec<_>>(),
        vec!["InternalOnly"]
    );
}

#[test]
fn rust_std_types_emit_derived_provenance_records() {
    let provenance: RustStdProvenance = <i32 as RustStdType>::provenance();

    assert_eq!(
        provenance
            .get("rust.authority")
            .expect("shared authority fact present")
            .value(),
        "Rust Project Developers"
    );
    assert_eq!(
        provenance
            .get("rust.source_module")
            .expect("shared source module fact present")
            .value(),
        "core::primitive"
    );
    assert_eq!(
        provenance
            .get("type_name")
            .expect("type name fact present")
            .value(),
        "i32"
    );
    assert_eq!(
        provenance
            .get("semantic_summary")
            .expect("semantic summary fact present")
            .value(),
        "The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
}

#[test]
fn fixed_width_integer_types_share_rust_language_provenance_but_vary_type_specific_facts() {
    let i8_provenance = <i8 as RustStdType>::provenance();
    let i32_provenance = <i32 as RustStdType>::provenance();

    assert_eq!(
        i8_provenance
            .get("rust.authority")
            .expect("i8 shared authority fact present")
            .value(),
        i32_provenance
            .get("rust.authority")
            .expect("i32 shared authority fact present")
            .value(),
    );
    assert_eq!(
        i8_provenance
            .get("rust.source_crate")
            .expect("i8 shared crate fact present")
            .value(),
        i32_provenance
            .get("rust.source_crate")
            .expect("i32 shared crate fact present")
            .value(),
    );
    assert_ne!(
        i8_provenance
            .get("source_url")
            .expect("i8 source url fact present")
            .value(),
        i32_provenance
            .get("source_url")
            .expect("i32 source url fact present")
            .value(),
    );
    assert_ne!(
        i8_provenance
            .get("type_name")
            .expect("i8 type name fact present")
            .value(),
        i32_provenance
            .get("type_name")
            .expect("i32 type name fact present")
            .value(),
    );
}

#[test]
fn rust_std_type_report_renders_a_default_human_readable_audit_surface() {
    assert_eq!(
        <i32 as RustStdType>::report().to_string(),
        "rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
}

#[test]
fn rust_std_type_certification_is_issued_through_the_registry() {
    let mut registry = CertRegistry::new();

    assert_eq!(
        <i32 as RustStdType>::certification(&mut registry).to_string(),
        "Provenance certificate 1 for i32\n\
rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
    assert_eq!(registry.len(), 1);
}

#[test]
fn rust_std_standard_wrapper_implements_standard() {
    let standard = RustStdStandard::<i32>::new();
    let mut registry = CertRegistry::new();

    assert_eq!(
        standard.report().to_string(),
        "rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
    assert_eq!(
        standard.certification(&mut registry, "i32").to_string(),
        "Provenance certificate 1 for i32\n\
rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
}

#[test]
fn rust_std_certificate_artifacts_are_emitted_to_the_workspace_artifacts_directory() {
    let artifact_directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/std-certificates");
    let paths =
        write_rust_std_certificate_artifacts(&artifact_directory).expect("std artifacts emitted");

    assert_eq!(paths.len(), 17);
    assert_eq!(
        std::fs::read_to_string(artifact_directory.join("i32.provenance.txt"))
            .expect("i32 certificate artifact present"),
        "Provenance certificate 5 for i32\n\
rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
}
