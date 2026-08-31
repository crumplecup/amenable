//! Lay generated Verus modules out on disk (`mod` declarations, nested
//! directories) and drive the export sweep that writes them.

use super::flat::render_verus_module;
use crate::{AmenableError, AmenableResult};
use amenable_core::{WitnessExportSnapshot, witness_exports};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::instrument;

/// Write one Verus proof module per registered `verus` witness export.
#[instrument(level = "info")]
pub fn write_verus_witness_modules(root: &Path) -> AmenableResult<Vec<PathBuf>> {
    fs::create_dir_all(root).map_err(|error| AmenableError::io(root, error))?;

    let mut written_paths = Vec::new();
    let mut failures = Vec::new();

    // Each export is rendered independently: one unsupported or
    // misregistered export (e.g. an enum-shaped composite, or a checked
    // leaf whose harness has no registered call shape yet) must not
    // block every *other*, unrelated export from being written --
    // confirmed as a real problem while building this, not a
    // hypothetical: with `inventory`-based registration being
    // process-global, a single broken registration anywhere silently
    // starved every working export in the same process under the
    // original fail-fast version.
    for export in witness_exports()
        .into_iter()
        .filter(|record| record.verifier() == "verus")
    {
        match render_and_write_export(root, &export) {
            Ok(path) => written_paths.push(path),
            Err(error) => failures.push(format!("{}: {error}", export.evidence())),
        }
    }

    if !failures.is_empty() {
        return Err(AmenableError::invariant(format!(
            "failed to render {} of {} Verus witness export(s):\n{}",
            failures.len(),
            written_paths.len() + failures.len(),
            failures.join("\n")
        )));
    }

    Ok(written_paths)
}

#[instrument(level = "debug", skip(export))]
fn render_and_write_export(root: &Path, export: &WitnessExportSnapshot) -> AmenableResult<PathBuf> {
    let (parent_segments, final_segment) = parse_destination_module(export.destination_module())?;
    let output_path = ensure_module_tree(root, &parent_segments, &final_segment)?;
    let source = render_verus_module(export, &final_segment)?;

    fs::write(&output_path, source).map_err(|error| AmenableError::io(&output_path, error))?;
    Ok(output_path)
}

/// Splits a validated `crate::a::b::c` destination into its ancestor
/// segments (`a`, `b`) and final segment (`c`), so callers that only need
/// the final segment carry that guarantee in the type instead of
/// re-deriving it with `.last().expect(...)`.
#[instrument(level = "debug")]
fn parse_destination_module(destination_module: &str) -> AmenableResult<(Vec<String>, String)> {
    let mut parts = destination_module.split("::");
    let crate_root = parts.next().ok_or_else(|| {
        AmenableError::invariant(format!(
            "invalid Verus destination module {destination_module:?}: expected crate-relative path"
        ))
    })?;

    if crate_root != "crate" {
        return Err(AmenableError::invariant(format!(
            "invalid Verus destination module {destination_module:?}: expected path to start with \"crate::\""
        )));
    }

    let mut segments = parts.map(str::to_owned).collect::<Vec<_>>();
    for segment in &segments {
        if !is_valid_module_segment(segment) {
            return Err(AmenableError::invariant(format!(
                "invalid Verus destination module {destination_module:?}: bad Rust module segment {segment:?}"
            )));
        }
    }

    let Some(final_segment) = segments.pop() else {
        return Err(AmenableError::invariant(format!(
            "invalid Verus destination module {destination_module:?}: missing module segments"
        )));
    };

    Ok((segments, final_segment))
}

#[instrument(level = "trace")]
fn is_valid_module_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

#[instrument(level = "debug")]
fn ensure_module_tree(
    root: &Path,
    parent_segments: &[String],
    final_segment: &str,
) -> AmenableResult<PathBuf> {
    let mut current_dir = root.to_path_buf();
    let mut module_file = root.join("lib.rs");

    for segment in parent_segments {
        ensure_module_declaration(&module_file, segment)?;
        current_dir.push(segment);
        fs::create_dir_all(&current_dir).map_err(|error| AmenableError::io(&current_dir, error))?;
        module_file = current_dir.join("mod.rs");
    }

    ensure_module_declaration(&module_file, final_segment)?;

    Ok(current_dir.join(format!("{final_segment}.rs")))
}

#[instrument(level = "debug")]
fn ensure_module_declaration(module_file: &Path, module_name: &str) -> AmenableResult<()> {
    if let Some(parent) = module_file.parent() {
        fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
    }

    let public_decl = format!("pub mod {module_name};");
    let private_decl = format!("mod {module_name};");
    let mut content = match fs::read_to_string(module_file) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(AmenableError::io(module_file, error)),
    };

    if content.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == public_decl || trimmed == private_decl
    }) {
        return Ok(());
    }

    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    content.push_str(&public_decl);
    content.push('\n');

    fs::write(module_file, content).map_err(|error| AmenableError::io(module_file, error))
}
