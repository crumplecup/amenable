#[path = "support_reporting.rs"]
mod support;

use amenable_core::{Provenance as ProvenanceTrait, Registry, Standard};
use amenable_std::{
    CertRegistry, RustStdProvenance, RustStdStandard, RustStdType,
    write_rust_std_certificate_artifacts,
};
use miette::{IntoDiagnostic, WrapErr};
use std::path::Path;

#[test]
fn rust_std_types_emit_derived_provenance_records() -> miette::Result<()> {
    let provenance: RustStdProvenance = <i32 as RustStdType>::provenance();

    assert_eq!(
        provenance
            .get("rust.authority")
            .ok_or_else(|| miette::miette!("shared authority fact present"))?
            .value(),
        "Rust Project Developers"
    );
    assert_eq!(
        provenance
            .get("rust.source_module")
            .ok_or_else(|| miette::miette!("shared source module fact present"))?
            .value(),
        "core::primitive"
    );
    assert_eq!(
        provenance
            .get("type_name")
            .ok_or_else(|| miette::miette!("type name fact present"))?
            .value(),
        "i32"
    );
    assert_eq!(
        provenance
            .get("semantic_summary")
            .ok_or_else(|| miette::miette!("semantic summary fact present"))?
            .value(),
        "The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
    Ok(())
}

#[test]
fn fixed_width_integer_types_share_rust_language_provenance_but_vary_type_specific_facts()
-> miette::Result<()> {
    let i8_provenance = <i8 as RustStdType>::provenance();
    let i32_provenance = <i32 as RustStdType>::provenance();

    assert_eq!(
        i8_provenance
            .get("rust.authority")
            .ok_or_else(|| miette::miette!("i8 shared authority fact present"))?
            .value(),
        i32_provenance
            .get("rust.authority")
            .ok_or_else(|| miette::miette!("i32 shared authority fact present"))?
            .value(),
    );
    assert_eq!(
        i8_provenance
            .get("rust.source_crate")
            .ok_or_else(|| miette::miette!("i8 shared crate fact present"))?
            .value(),
        i32_provenance
            .get("rust.source_crate")
            .ok_or_else(|| miette::miette!("i32 shared crate fact present"))?
            .value(),
    );
    assert_ne!(
        i8_provenance
            .get("source_url")
            .ok_or_else(|| miette::miette!("i8 source url fact present"))?
            .value(),
        i32_provenance
            .get("source_url")
            .ok_or_else(|| miette::miette!("i32 source url fact present"))?
            .value(),
    );
    assert_ne!(
        i8_provenance
            .get("type_name")
            .ok_or_else(|| miette::miette!("i8 type name fact present"))?
            .value(),
        i32_provenance
            .get("type_name")
            .ok_or_else(|| miette::miette!("i32 type name fact present"))?
            .value(),
    );
    Ok(())
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
fn rust_std_certificate_artifacts_are_emitted_to_the_workspace_artifacts_directory()
-> miette::Result<()> {
    let artifact_directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/std-certificates");
    let paths = support::library(write_rust_std_certificate_artifacts(&artifact_directory))?;

    assert_eq!(paths.len(), 17);
    assert_eq!(
        std::fs::read_to_string(artifact_directory.join("i32.provenance.txt"))
            .into_diagnostic()
            .wrap_err("i32 certificate artifact present")?,
        "Provenance certificate 5 for i32\n\
rust.authority_kind: external_standard\n\
rust.authority: Rust Project Developers\n\
rust.source_crate: core\n\
rust.source_module: core::primitive\n\
source_url: https://doc.rust-lang.org/std/primitive.i32.html\n\
type_name: i32\n\
semantic_summary: The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
    );
    Ok(())
}
