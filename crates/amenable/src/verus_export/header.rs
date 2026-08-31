//! The module preamble shared by both render strategies: doc comment,
//! `use`s (incl. gated predicate imports), and the `verus! {}` opener
//! with its evidence/destination/support/audit comments — identical for
//! the flat (struct/tuple-struct) and enum-composite render paths, so
//! both build onto the same `source` buffer via [`write_module_header`]
//! rather than duplicating it.

use amenable_core::WitnessExportSnapshot;
use std::collections::HashSet;
use tracing::instrument;

#[instrument(level = "info", skip(source, export))]
pub(super) fn write_module_header(
    source: &mut String,
    export: &WitnessExportSnapshot,
    imports: &[(String, String)],
    comments: &[String],
) {
    source.push_str(&format!(
        "//! Derived Verus closure for `{}`.\n\n",
        export.evidence()
    ));
    source.push_str("use verus_builtin_macros::verus;\n");
    source.push_str(
        "#[allow(\n    unused_imports,\n    reason = \"vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly\"\n)]\n",
    );
    source.push_str("use vstd::prelude::*;\n");

    let predicate_imports = predicate_import_lines(imports);
    if !predicate_imports.is_empty() {
        source.push('\n');
        for import in &predicate_imports {
            // Spec fns have no runtime representation and don't exist
            // under plain `cargo check` -- gate every predicate import
            // the same way every hand-written carrier in this crate
            // already does, or `cargo check`/`clippy-verus` (which don't
            // set `verus_keep_ghost`) fail with an unresolved import.
            source.push_str("#[cfg(verus_keep_ghost)]\n");
            source.push_str(import);
            source.push('\n');
        }
    }
    source.push('\n');

    source.push_str("verus! {\n\n");
    source.push_str(&format!("// evidence: {}\n", export.evidence()));
    source.push_str(&format!(
        "// destination: {}\n",
        export.destination_module()
    ));
    source.push_str(&format!("// support: {}\n", export.support()));

    if !comments.is_empty() {
        source.push('\n');
        for comment in comments {
            source.push_str(comment);
            source.push('\n');
        }
    }
    source.push('\n');
}

/// Verus spec predicates need an explicit `use`, unlike ordinary
/// functions (which resolve fine via a fully qualified call path) —
/// confirmed against the real `verus` tool while building this.
#[instrument(level = "debug")]
fn predicate_import_lines(imports: &[(String, String)]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut lines = Vec::new();

    for (module_path, name) in imports {
        if seen.insert((module_path.clone(), name.clone())) {
            lines.push(format!("use {module_path}::{name};"));
        }
    }

    // rustfmt sorts `use` groups alphabetically; sorting here too keeps
    // every regeneration byte-for-byte rustfmt-clean, so `emit-verus-
    // witnesses` never leaves a dirty `cargo fmt --all --check`.
    lines.sort();
    lines
}
