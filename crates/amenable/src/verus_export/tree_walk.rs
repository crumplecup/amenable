//! Walk an artifact tree into a [`RenderedNode`], one leaf at a time.

use super::model::substitute_placeholder;
use super::model::{CheckedCall, NameAllocator, PendingClause, RenderedNode, RenderedParam};
use super::route::{RouteSegment, route_display, route_hint_name};
use crate::{AmenableError, AmenableResult};
use amenable_core::{MetadataEntry, WitnessArtifactNode, WitnessArtifactShape, WitnessSupportKind};
use amenable_std::{VerusCallKind, VerusCallShape, verus_call_shape};
use std::collections::HashMap;
use tracing::instrument;

#[instrument(level = "debug", skip(node, route, names))]
pub(super) fn render_node(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    match node.shape() {
        WitnessArtifactShape::Leaf => render_leaf_node(node, route, names),
        WitnessArtifactShape::Enum => Err(AmenableError::invariant(format!(
            "Verus composition for a nested enum-shaped witness is not supported yet (route: {}); \
             match-per-variant composition (see render_enum_module) only handles an export's own \
             root shape -- no real nested-enum type is registered anywhere in this codebase yet, \
             so exclude this type from Verus export until one needs it",
            route_display(route)
        ))),
        WitnessArtifactShape::NamedStruct
        | WitnessArtifactShape::TupleStruct
        | WitnessArtifactShape::UnitStruct
        | WitnessArtifactShape::NamedVariant
        | WitnessArtifactShape::TupleVariant
        | WitnessArtifactShape::UnitVariant => {
            let mut combined = RenderedNode::default();
            for member in node.members() {
                let mut child_route = route.to_vec();
                child_route.push(RouteSegment::Member(member.label().clone()));
                combined.merge(render_node(member.artifact(), &child_route, names)?);
            }
            Ok(combined)
        }
    }
}

#[instrument(level = "debug", skip(node, route, names))]
fn render_leaf_node(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    match node.kind() {
        WitnessSupportKind::Trivial => Ok(RenderedNode::default()),
        WitnessSupportKind::Trusted => {
            Ok(RenderedNode::default().with_comments(vec![render_trust_comment(node, route)]))
        }
        WitnessSupportKind::Checked => render_checked_leaf(node, route, names),
        WitnessSupportKind::Mixed | WitnessSupportKind::Opaque => {
            Err(AmenableError::invariant(format!(
                "leaf at {} reports an impossible per-leaf support kind {:?} -- Mixed only \
                 arises from composing multiple leaves, and Opaque leaves cannot reach this \
                 renderer (amenable_core::ClassifiedWitness blocks them at cargo check time)",
                route_display(route),
                node.kind()
            )))
        }
    }
}

#[instrument(level = "debug", skip(node, route, names))]
fn render_checked_leaf(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    let harness = metadata_value(node.metadata(), "harness").ok_or_else(|| {
        AmenableError::invariant(format!(
            "checked leaf at {} has no \"harness\" metadata entry",
            route_display(route)
        ))
    })?;

    let shape = verus_call_shape(harness).ok_or_else(|| {
        AmenableError::invariant(format!(
            "no Verus call shape found for harness `{harness}` (route: {}) -- it should be a real, \
             public `pub fn` in some crates/amenable_verus/src/**/*.rs carrier file (call shapes \
             derive automatically from real source), or explicitly registered via \
             amenable_std::register_verus_call_shape! if it's a synthetic/test-only shape with no \
             real carrier file behind it",
            route_display(route)
        ))
    })?;

    let route_hint = route_hint_name(route);
    let (params, local_names) = allocate_params(&shape, &route_hint, names);
    let call = build_checked_call(&shape, harness, route, &local_names)?;

    let requires = shape
        .requires()
        .iter()
        .map(|template| pending_clause(template, &local_names))
        .collect();
    let ensures = shape
        .ensures()
        .iter()
        .map(|template| pending_clause(template, &local_names))
        .collect();
    let imports = shape
        .imports()
        .iter()
        .map(|import| (import.module_path().clone(), import.name().clone()))
        .collect();

    let comment = format!(
        "// checked leaf at {}: calls {}::{}",
        route_display(route),
        shape.module_path(),
        shape.name()
    );

    Ok(RenderedNode::default()
        .with_params(params)
        .with_checked_calls(vec![call])
        .with_requires(requires)
        .with_ensures(ensures)
        .with_imports(imports)
        .with_comments(vec![comment]))
}

/// Allocate a collision-resolved local name for each of a checked leaf's
/// real parameters, returning both the ordered [`RenderedParam`] list and
/// the real-name-to-local-name lookup later clauses substitute
/// `$paramname` placeholders through.
#[instrument(level = "debug", skip(shape, names))]
fn allocate_params(
    shape: &VerusCallShape,
    route_hint: &str,
    names: &mut NameAllocator,
) -> (Vec<RenderedParam>, HashMap<String, String>) {
    let mut local_names = HashMap::new();
    let mut params = Vec::new();

    for param in shape.params() {
        let local = names.allocate(param.name(), route_hint);
        local_names.insert(param.name().clone(), local.clone());
        params.push(RenderedParam::new(local, param.ty().clone()));
    }

    (params, local_names)
}

/// Build the leaf's real call expression against its already-allocated
/// local parameter names, and pair it with the harness's real return
/// type -- rejecting a bare-predicate shape, which isn't wired up as a
/// callable function yet.
#[instrument(level = "debug", skip(shape, route, local_names))]
fn build_checked_call(
    shape: &VerusCallShape,
    harness: &str,
    route: &[RouteSegment],
    local_names: &HashMap<String, String>,
) -> AmenableResult<CheckedCall> {
    let call_args = shape
        .params()
        .iter()
        .map(|param| local_names[param.name()].clone())
        .collect::<Vec<_>>()
        .join(", ");

    let ty = match &shape.kind() {
        VerusCallKind::Function { returns } => returns.clone(),
        VerusCallKind::Predicate => {
            return Err(AmenableError::invariant(format!(
                "harness `{harness}` (route: {}) is registered as a bare predicate, not a \
                 callable function -- predicate-kind composition isn't wired up yet",
                route_display(route)
            )));
        }
    };

    Ok(CheckedCall::new(
        format!("{}::{}({call_args})", shape.module_path(), shape.name()),
        ty,
    ))
}

/// Substitute every `$paramname` placeholder (i.e. every placeholder
/// except `$result`) with this leaf's own chosen local name -- fully
/// resolved already, no rebasing needed. `$result` is left literal for
/// [`super::model::PendingClause::render`] to resolve once the final
/// checked-call count is known.
#[instrument(level = "debug", skip(local_names))]
fn pending_clause(template: &str, local_names: &HashMap<String, String>) -> PendingClause {
    let mut resolved = template.to_owned();
    for (name, local) in local_names {
        if name != "result" {
            resolved = substitute_placeholder(&resolved, name, local);
        }
    }

    PendingClause::new(resolved, 0)
}

#[instrument(level = "debug", skip(node, route))]
fn render_trust_comment(node: &WitnessArtifactNode, route: &[RouteSegment]) -> String {
    let mut line = format!("// trusted leaf at {}", route_display(route));

    let metadata = node
        .metadata()
        .iter()
        .map(|entry| format!("{} = {}", entry.key(), entry.value()))
        .collect::<Vec<_>>()
        .join(", ");

    if !metadata.is_empty() {
        line.push_str("; ");
        line.push_str(&metadata);
    }

    line
}

#[instrument(level = "debug", skip(metadata))]
fn metadata_value<'a>(metadata: &'a [MetadataEntry], key: &str) -> Option<&'a str> {
    metadata
        .iter()
        .find(|entry| entry.key() == key)
        .map(MetadataEntry::value)
}
