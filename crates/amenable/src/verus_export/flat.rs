//! Compose a struct/tuple-struct-shaped export's single flat return
//! value (dispatching to [`super::enum_render`] for an enum-shaped root
//! instead).

use super::enum_render::render_enum_module;
use super::header::write_module_header;
use super::identifiers::normalize_identifier;
use super::model::{CheckedCall, NameAllocator, PendingClause};
use super::tree_walk::render_node;
use crate::AmenableResult;
use amenable_core::{WitnessArtifactShape, WitnessExportSnapshot};
use tracing::instrument;

#[instrument(level = "debug", skip(export))]
pub(super) fn render_verus_module(
    export: &WitnessExportSnapshot,
    module_name: &str,
) -> AmenableResult<String> {
    let module_stem = normalize_identifier(module_name);
    let mut names = NameAllocator::default();

    // A real value only occupies one variant at a time, so an enum root
    // needs match-per-variant composition -- structurally different
    // enough (selector/result types, no single flat return tuple) that
    // it gets its own render path rather than folding into the one
    // below. Nested `Enum` shapes (a member inside a struct, or inside
    // another variant) still fall through to `render_node`'s own
    // rejection -- no real nested-enum type is registered anywhere in
    // this codebase yet.
    if export.artifact().shape() == WitnessArtifactShape::Enum {
        return render_enum_module(export, &module_stem, &mut names);
    }

    let rendered = render_node(export.artifact(), &[], &mut names)?;
    let checked_call_count = rendered.checked_calls().len();

    let mut source = String::new();
    write_module_header(&mut source, export, rendered.imports(), rendered.comments());

    let params = rendered
        .params()
        .iter()
        .map(|param| format!("{}: {}", param.local_name(), param.ty()))
        .collect::<Vec<_>>()
        .join(", ");
    let return_ty = render_return_type(rendered.checked_calls());
    source.push_str(&format!(
        "/// Proves `{module_stem}`'s own composed claim -- see this file's own\n/// header comment.\npub fn verify_{module_stem}({params}){return_ty}\n"
    ));

    push_clause_block(
        &mut source,
        "requires",
        rendered.requires(),
        checked_call_count,
    );
    push_clause_block(
        &mut source,
        "ensures",
        rendered.ensures(),
        checked_call_count,
    );

    let body = render_body(rendered.checked_calls());
    source.push_str(&format!("{{\n    {body}\n}}\n"));
    source.push_str("\n} // verus!\n");
    Ok(source)
}

/// The composite's own `-> (result: T)` clause: nothing for zero checked
/// calls, the lone call's real type for one, or a tuple of every call's
/// real type for more than one.
#[instrument(level = "debug", skip(checked_calls))]
fn render_return_type(checked_calls: &[CheckedCall]) -> String {
    match checked_calls {
        [] => String::new(),
        [only] => format!(" -> (result: {})", only.ty()),
        many => format!(
            " -> (result: ({}))",
            many.iter()
                .map(|call| call.ty().clone())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// The composite's own body: nothing for zero checked calls, the lone
/// call's real expression for one, or a tuple of every call's real
/// expression for more than one -- mirrors [`render_return_type`]'s own
/// zero/one/many split, since the body and the return type describe the
/// same value.
#[instrument(level = "debug", skip(checked_calls))]
fn render_body(checked_calls: &[CheckedCall]) -> String {
    match checked_calls {
        [] => String::new(),
        [only] => only.expr().clone(),
        many => format!(
            "({})",
            many.iter()
                .map(|call| call.expr().clone())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Push a `requires`/`ensures` block onto `source`, or nothing if the
/// subtree contributed no clauses of that kind.
#[instrument(level = "debug", skip(source, clauses))]
fn push_clause_block(
    source: &mut String,
    keyword: &str,
    clauses: &[PendingClause],
    checked_call_count: usize,
) {
    if clauses.is_empty() {
        return;
    }

    source.push_str(&format!("    {keyword}\n"));
    for clause in clauses {
        source.push_str(&format!("        {},\n", clause.render(checked_call_count)));
    }
}
