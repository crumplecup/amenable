//! One root-level enum export: a synthetic local selector enum (one
//! unit variant per real artifact variant, chooses which arm's claim is
//! being proved) and a synthetic local result enum (one variant per
//! real artifact variant, payload = that variant's own checked-call
//! tuple, no payload if it has none) replace the flat struct path's
//! single parameter list/tuple return — see Design E in
//! `docs/VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md` for the rejected
//! alternatives and why this shape was chosen. The body and `ensures`
//! are both a real `match selector { ... }`; `ensures`'s arms further
//! `match result { ... }`, citing exactly the selected variant's own
//! composed claim and `false` for every other (structurally
//! unreachable, but syntactically required for exhaustiveness) result
//! shape.

use super::header::write_module_header;
use super::identifiers::{normalize_identifier, to_pascal_case};
use super::model::{NameAllocator, RenderedNode};
use super::route::RouteSegment;
use super::tree_walk::render_node;
use crate::{AmenableError, AmenableResult};
use amenable_core::WitnessExportSnapshot;
use tracing::instrument;

#[instrument(level = "debug", skip(export, names))]
pub(super) fn render_enum_module(
    export: &WitnessExportSnapshot,
    module_stem: &str,
    names: &mut NameAllocator,
) -> AmenableResult<String> {
    if export.artifact().variants().is_empty() {
        return Err(AmenableError::invariant(format!(
            "enum-shaped export {} has no variants -- nothing to compose",
            export.evidence()
        )));
    }

    let type_prefix = to_pascal_case(module_stem);
    let selector_ty = format!("{type_prefix}Selector");
    let result_ty = format!("{type_prefix}Result");

    let per_variant = render_variant_subtrees(export, names)?;

    let mut imports = Vec::new();
    let mut comments = Vec::new();
    for (_, rendered) in &per_variant {
        imports.extend(rendered.imports().iter().cloned());
        comments.extend(rendered.comments().iter().cloned());
    }

    let mut source = String::new();
    write_module_header(&mut source, export, &imports, &comments);

    let selector_variants = per_variant
        .iter()
        .map(|(name, _)| format!("    /// Selects `{name}`'s own composed claim.\n    {name},"))
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(&format!(
        "/// Selects which of `{module_stem}`'s composed variant claims a call to\n/// `verify_{module_stem}` proves.\npub enum {selector_ty} {{\n{selector_variants}\n}}\n\n"
    ));

    let arms = build_variant_arms(&selector_ty, &result_ty, &per_variant);
    let result_variant_decls = per_variant
        .iter()
        .zip(arms.result_variant_decls.iter())
        .map(|((name, _), decl)| format!("    /// `{name}`'s own result shape.\n{decl}"))
        .collect::<Vec<_>>()
        .join("\n");
    source.push_str(&format!(
        "/// The result of the `{selector_ty}` variant a `verify_{module_stem}` call\n/// selected.\npub enum {result_ty} {{\n{result_variant_decls}\n}}\n\n"
    ));

    let param_lists = render_param_lists(&selector_ty, &per_variant);

    // Named once, called from `ensures` below, rather than restated
    // inline: `cordial`'s own unnamed-contract-bound scanner only
    // recognizes a whole-clause bare call `name(...)`, never a raw
    // `match` expression, regardless of how genuinely composed its own
    // arms are -- naming this the same way every hand-written carrier's
    // own multi-arm postcondition already does.
    source.push_str(&format!(
        "/// `verify_{module_stem}`'s whole postcondition: the selected variant's\n/// own composed claim.\npub open spec fn {module_stem}_ensures_holds(selector: {selector_ty}, result: {result_ty}{}) -> bool {{\n    match selector {{\n{}\n    }}\n}}\n\n",
        param_lists.params_suffix,
        arms.ensures_arms.join("\n")
    ));

    if arms.any_requires {
        source.push_str(&format!(
            "/// `verify_{module_stem}`'s whole precondition: the selected variant's\n/// own composed requirement.\npub open spec fn {module_stem}_requires_holds(selector: {selector_ty}{}) -> bool {{\n    match selector {{\n{}\n    }}\n}}\n\n",
            param_lists.params_suffix,
            arms.requires_arms.join("\n")
        ));
    }

    source.push_str(&format!(
        "/// Proves `{module_stem}`'s own selected-variant claim -- see this crate's\n/// own doc comment.\npub fn verify_{module_stem}({}) -> (result: {result_ty})\n",
        param_lists.full_params
    ));

    if arms.any_requires {
        source.push_str(&format!(
            "    requires\n        {module_stem}_requires_holds(selector{}),\n",
            param_lists.param_names_suffix
        ));
    }

    source.push_str(&format!(
        "    ensures\n        {module_stem}_ensures_holds(selector, result{}),\n",
        param_lists.param_names_suffix
    ));

    source.push_str("{\n    match selector {\n");
    source.push_str(&arms.body_arms.join("\n"));
    source.push_str("\n    }\n}\n");
    source.push_str("\n} // verus!\n");

    Ok(source)
}

/// Render each real artifact variant's own subtree, naming it with a
/// route-scoped [`RouteSegment::Variant`] so sibling variants' same-named
/// params disambiguate the same way sibling struct members already do.
/// The artifact's own variant name may carry a provenance rename (e.g.
/// `fallback`, all-lowercase) rather than the real source identifier's
/// casing -- fine for audit labels/comments (the route itself keeps the
/// original), but not a valid Rust enum variant identifier on its own,
/// so the returned name is normalized to `PascalCase` for the synthetic
/// selector/result enums.
#[instrument(level = "debug", skip(export, names))]
fn render_variant_subtrees(
    export: &WitnessExportSnapshot,
    names: &mut NameAllocator,
) -> AmenableResult<Vec<(String, RenderedNode)>> {
    let mut per_variant = Vec::with_capacity(export.artifact().variants().len());
    for variant in export.artifact().variants() {
        let route = vec![RouteSegment::Variant(variant.name().clone())];
        let rendered = render_node(variant.artifact(), &route, names)?;
        let variant_ident = to_pascal_case(&normalize_identifier(variant.name()));
        per_variant.push((variant_ident, rendered));
    }

    Ok(per_variant)
}

/// The per-variant pieces of a rendered enum module: one result-enum
/// variant declaration, one `match selector` body arm, and one
/// `ensures_holds`/`requires_holds` arm, per real artifact variant.
struct VariantArms {
    result_variant_decls: Vec<String>,
    body_arms: Vec<String>,
    ensures_arms: Vec<String>,
    requires_arms: Vec<String>,
    any_requires: bool,
}

#[instrument(level = "debug", skip(per_variant))]
fn build_variant_arms(
    selector_ty: &str,
    result_ty: &str,
    per_variant: &[(String, RenderedNode)],
) -> VariantArms {
    let mut result_variant_decls = Vec::new();
    let mut body_arms = Vec::new();
    let mut ensures_arms = Vec::new();
    let mut requires_arms = Vec::new();
    let mut any_requires = false;

    for (name, rendered) in per_variant {
        let call_count = rendered.checked_calls().len();
        let bind_names: Vec<String> = match call_count {
            0 => Vec::new(),
            1 => vec!["r".to_owned()],
            _ => (0..call_count).map(|index| format!("r{index}")).collect(),
        };

        let (result_decl, result_pattern, result_ctor) = if call_count == 0 {
            (name.clone(), name.clone(), name.clone())
        } else {
            let types = rendered
                .checked_calls()
                .iter()
                .map(|call| call.ty().clone())
                .collect::<Vec<_>>()
                .join(", ");
            let exprs = rendered
                .checked_calls()
                .iter()
                .map(|call| call.expr().clone())
                .collect::<Vec<_>>()
                .join(", ");
            (
                format!("{name}({types})"),
                format!("{name}({})", bind_names.join(", ")),
                format!("{name}({exprs})"),
            )
        };
        result_variant_decls.push(format!("    {result_decl},"));
        body_arms.push(format!(
            "        {selector_ty}::{name} => {result_ty}::{result_ctor},"
        ));

        let claim = if rendered.ensures().is_empty() {
            "true".to_owned()
        } else {
            rendered
                .ensures()
                .iter()
                .map(|clause| clause.render_with(&bind_names[*clause.result_index()]))
                .collect::<Vec<_>>()
                .join(" && ")
        };
        ensures_arms.push(format!(
            "            {selector_ty}::{name} => match result {{\n                \
             {result_ty}::{result_pattern} => {claim},\n                _ => false,\n            }},"
        ));

        if !rendered.requires().is_empty() {
            any_requires = true;
        }
        let requires_claim = if rendered.requires().is_empty() {
            "true".to_owned()
        } else {
            rendered
                .requires()
                .iter()
                .map(|clause| clause.render_with("result"))
                .collect::<Vec<_>>()
                .join(" && ")
        };
        requires_arms.push(format!(
            "            {selector_ty}::{name} => {requires_claim},"
        ));
    }

    VariantArms {
        result_variant_decls,
        body_arms,
        ensures_arms,
        requires_arms,
        any_requires,
    }
}

/// The composite `verify_` function's own parameter lists: every real
/// variant's params flattened together (a real value only occupies one
/// variant at a time, but the generated function must accept whichever
/// variant's params the caller is about to select), plus the two
/// comma-prefixed suffixes `ensures_holds`/`requires_holds` need to
/// forward them -- empty when the export has no per-variant params at
/// all (the canary enum's `Fallback`/`Closed` case).
struct ParamLists {
    full_params: String,
    params_suffix: String,
    param_names_suffix: String,
}

#[instrument(level = "debug", skip(per_variant))]
fn render_param_lists(selector_ty: &str, per_variant: &[(String, RenderedNode)]) -> ParamLists {
    let param_names = per_variant
        .iter()
        .flat_map(|(_, rendered)| rendered.params().iter())
        .map(|param| param.local_name().clone())
        .collect::<Vec<_>>();
    let params = per_variant
        .iter()
        .flat_map(|(_, rendered)| rendered.params().iter())
        .map(|param| format!("{}: {}", param.local_name(), param.ty()))
        .collect::<Vec<_>>()
        .join(", ");

    let full_params = if params.is_empty() {
        format!("selector: {selector_ty}")
    } else {
        format!("selector: {selector_ty}, {params}")
    };
    let params_suffix = if params.is_empty() {
        String::new()
    } else {
        format!(", {params}")
    };
    let param_names_suffix = if param_names.is_empty() {
        String::new()
    } else {
        format!(", {}", param_names.join(", "))
    };

    ParamLists {
        full_params,
        params_suffix,
        param_names_suffix,
    }
}
