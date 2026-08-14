//! Emit derived Verus witness modules from registered artifact trees.

use std::{
    fs,
    path::{Path, PathBuf},
};

use amenable_core::{
    MetadataEntry, WitnessArtifactNode, WitnessArtifactShape, WitnessExportSnapshot,
    WitnessSupportKind, witness_exports,
};

use crate::{AmenableError, AmenableResult};

/// Write one Verus proof module per registered `verus` witness export.
pub fn write_verus_witness_modules(root: &Path) -> AmenableResult<Vec<PathBuf>> {
    fs::create_dir_all(root).map_err(|error| AmenableError::io(root, error))?;

    let mut written_paths = Vec::new();

    for export in witness_exports()
        .into_iter()
        .filter(|record| record.verifier == "verus")
    {
        let module_segments = parse_destination_module(&export.destination_module)?;
        let output_path = ensure_module_tree(root, &module_segments)?;
        let source = render_verus_module(&export, &module_segments);

        fs::write(&output_path, source).map_err(|error| AmenableError::io(&output_path, error))?;
        written_paths.push(output_path);
    }

    Ok(written_paths)
}

fn parse_destination_module(destination_module: &str) -> AmenableResult<Vec<String>> {
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

    let segments = parts.map(str::to_owned).collect::<Vec<_>>();
    if segments.is_empty() {
        return Err(AmenableError::invariant(format!(
            "invalid Verus destination module {destination_module:?}: missing module segments"
        )));
    }

    for segment in &segments {
        if !is_valid_module_segment(segment) {
            return Err(AmenableError::invariant(format!(
                "invalid Verus destination module {destination_module:?}: bad Rust module segment {segment:?}"
            )));
        }
    }

    Ok(segments)
}

fn is_valid_module_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn ensure_module_tree(root: &Path, module_segments: &[String]) -> AmenableResult<PathBuf> {
    let mut current_dir = root.to_path_buf();
    let mut module_file = root.join("lib.rs");

    for segment in &module_segments[..module_segments.len() - 1] {
        ensure_module_declaration(&module_file, segment)?;
        current_dir.push(segment);
        fs::create_dir_all(&current_dir).map_err(|error| AmenableError::io(&current_dir, error))?;
        module_file = current_dir.join("mod.rs");
    }

    let final_segment = module_segments
        .last()
        .expect("module path parsing guarantees at least one segment");
    ensure_module_declaration(&module_file, final_segment)?;

    Ok(current_dir.join(format!("{final_segment}.rs")))
}

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

fn render_verus_module(export: &WitnessExportSnapshot, module_segments: &[String]) -> String {
    let module_name = module_segments
        .last()
        .expect("module path parsing guarantees at least one segment");
    let module_stem = normalize_identifier(module_name);
    let rendered = render_node(&export.artifact, &module_stem, &[]);
    let premise_comments = rendered
        .premises
        .iter()
        .map(render_premise_comment)
        .collect::<Vec<_>>()
        .join("\n");
    let proof_signature = render_proof_signature(&module_stem, &rendered.premises);
    let proof_body = render_proof_body(&module_stem, &rendered.premises);

    let mut source = String::new();
    source.push_str(&format!(
        "//! Derived Verus closure for `{}`.\n\n",
        export.evidence
    ));
    source.push_str("use verus_builtin_macros::verus;\n");
    source.push_str("#[allow(unused_imports)]\n");
    source.push_str("use vstd::prelude::*;\n\n");
    source.push_str("verus! {\n\n");
    source.push_str(&format!("// evidence: {}\n", export.evidence));
    source.push_str(&format!("// destination: {}\n", export.destination_module));
    source.push_str(&format!("// support: {}\n\n", export.support));

    for definition in &rendered.definitions {
        source.push_str(definition);
        source.push('\n');
    }

    if !premise_comments.is_empty() {
        source.push_str(&premise_comments);
        source.push('\n');
    }

    source.push_str(&proof_signature);
    source.push_str(&proof_body);
    source.push_str("\n} // verus!\n");
    source
}

#[derive(Debug, Clone)]
struct RenderedNode {
    definitions: Vec<String>,
    premises: Vec<LeafPremise>,
    expression: String,
}

#[derive(Debug, Clone)]
struct LeafPremise {
    name: String,
    kind: WitnessSupportKind,
    route: Vec<RouteSegment>,
    metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
enum RouteSegment {
    Member(String),
    Variant(String),
}

fn render_node(
    node: &WitnessArtifactNode,
    module_stem: &str,
    route: &[RouteSegment],
) -> RenderedNode {
    if node.shape == WitnessArtifactShape::Leaf {
        return render_leaf_node(node, module_stem, route);
    }

    let mut definitions = Vec::new();
    let mut premises = Vec::new();
    let mut child_expressions = Vec::new();

    for member in &node.members {
        let mut child_route = route.to_vec();
        child_route.push(RouteSegment::Member(member.label.clone()));
        let rendered = render_node(&member.artifact, module_stem, &child_route);
        definitions.extend(rendered.definitions);
        extend_premises(&mut premises, rendered.premises);
        child_expressions.push(rendered.expression);
    }

    for variant in &node.variants {
        let mut child_route = route.to_vec();
        child_route.push(RouteSegment::Variant(variant.name.clone()));
        let rendered = render_node(&variant.artifact, module_stem, &child_route);
        definitions.extend(rendered.definitions);
        extend_premises(&mut premises, rendered.premises);
        child_expressions.push(rendered.expression);
    }

    let definition = render_spec_definition(module_stem, route, &premises, &child_expressions);
    let expression = render_spec_call(module_stem, route, &premises);
    definitions.push(definition);

    RenderedNode {
        definitions,
        premises,
        expression,
    }
}

fn render_leaf_node(
    node: &WitnessArtifactNode,
    module_stem: &str,
    route: &[RouteSegment],
) -> RenderedNode {
    let expression = if node.kind == WitnessSupportKind::Trivial {
        "true".to_owned()
    } else {
        leaf_premise_name(module_stem, route, node.kind)
    };

    let premises = if node.kind == WitnessSupportKind::Trivial {
        Vec::new()
    } else {
        vec![LeafPremise {
            name: expression.clone(),
            kind: node.kind,
            route: route.to_vec(),
            metadata: filtered_metadata(&node.metadata),
        }]
    };

    if route.is_empty() {
        let definition = render_spec_definition(
            module_stem,
            route,
            &premises,
            std::slice::from_ref(&expression),
        );
        let root_expression = render_spec_call(module_stem, route, &premises);
        return RenderedNode {
            definitions: vec![definition],
            premises,
            expression: root_expression,
        };
    }

    RenderedNode {
        definitions: Vec::new(),
        premises,
        expression,
    }
}

fn filtered_metadata(entries: &[MetadataEntry]) -> Vec<(String, String)> {
    entries
        .iter()
        .filter(|entry| entry.key() != "claim")
        .map(|entry| (entry.key().to_owned(), entry.value().to_owned()))
        .collect()
}

fn extend_premises(target: &mut Vec<LeafPremise>, additions: Vec<LeafPremise>) {
    for premise in additions {
        if target.iter().all(|existing| existing.name != premise.name) {
            target.push(premise);
        }
    }
}

fn render_spec_definition(
    module_stem: &str,
    route: &[RouteSegment],
    premises: &[LeafPremise],
    child_expressions: &[String],
) -> String {
    let spec_name = spec_name(module_stem, route);
    let arguments = render_bool_arguments(premises);
    let body = if child_expressions.is_empty() {
        "true".to_owned()
    } else {
        child_expressions.join("\n        && ")
    };

    format!("pub open spec fn {spec_name}({arguments}) -> bool {{\n    {body}\n}}\n")
}

fn render_spec_call(module_stem: &str, route: &[RouteSegment], premises: &[LeafPremise]) -> String {
    let spec_name = spec_name(module_stem, route);
    let arguments = premises
        .iter()
        .map(|premise| premise.name.as_str())
        .collect::<Vec<_>>();

    if arguments.is_empty() {
        format!("{spec_name}()")
    } else {
        format!("{spec_name}({})", arguments.join(", "))
    }
}

fn render_bool_arguments(premises: &[LeafPremise]) -> String {
    premises
        .iter()
        .map(|premise| format!("{}: bool", premise.name))
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_premise_comment(premise: &LeafPremise) -> String {
    let mut line = format!(
        "// premise {}: {} leaf at {}",
        premise.name,
        premise.kind.as_str(),
        route_display(&premise.route)
    );

    if !premise.metadata.is_empty() {
        let metadata = premise
            .metadata
            .iter()
            .map(|(key, value)| format!("{key} = {value}"))
            .collect::<Vec<_>>()
            .join(", ");
        line.push_str("; ");
        line.push_str(&metadata);
    }

    line
}

fn route_display(route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return "root".to_owned();
    }

    route
        .iter()
        .map(|segment| match segment {
            RouteSegment::Member(label) => format!("member {label}"),
            RouteSegment::Variant(name) => format!("variant {name}"),
        })
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn render_proof_signature(module_stem: &str, premises: &[LeafPremise]) -> String {
    let arguments = render_bool_arguments(premises);
    let signature = if arguments.is_empty() {
        format!("pub proof fn verify_{module_stem}()")
    } else {
        format!("pub proof fn verify_{module_stem}({arguments})")
    };

    let requires = if premises.is_empty() {
        String::new()
    } else {
        format!(
            "\n    requires\n{}\n",
            premises
                .iter()
                .map(|premise| format!("        {},", premise.name))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    format!(
        "{signature}{requires}    ensures\n        {},\n",
        render_spec_call(module_stem, &[], premises)
    )
}

fn render_proof_body(module_stem: &str, premises: &[LeafPremise]) -> String {
    let call = render_spec_call(module_stem, &[], premises);
    format!("{{\n    assert({call});\n}}\n")
}

fn spec_name(module_stem: &str, route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return format!("{module_stem}_holds");
    }

    format!(
        "{module_stem}_{}_holds",
        route
            .iter()
            .map(route_segment_name)
            .collect::<Vec<_>>()
            .join("_")
    )
}

fn leaf_premise_name(
    module_stem: &str,
    route: &[RouteSegment],
    kind: WitnessSupportKind,
) -> String {
    let route_name = if route.is_empty() {
        "leaf".to_owned()
    } else {
        route
            .iter()
            .map(route_segment_name)
            .collect::<Vec<_>>()
            .join("_")
    };
    format!("{module_stem}_{route_name}_{}_holds", kind.as_str())
}

fn route_segment_name(segment: &RouteSegment) -> String {
    match segment {
        RouteSegment::Member(label) => format!("member_{}", normalize_identifier(label)),
        RouteSegment::Variant(name) => format!("variant_{}", normalize_identifier(name)),
    }
}

fn normalize_identifier(value: &str) -> String {
    let mut normalized = String::new();
    let mut previous_was_underscore = false;

    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() {
                if index > 0 && !previous_was_underscore {
                    normalized.push('_');
                }
                normalized.push(ch.to_ascii_lowercase());
            } else {
                normalized.push(ch.to_ascii_lowercase());
            }
            previous_was_underscore = false;
        } else if !previous_was_underscore {
            normalized.push('_');
            previous_was_underscore = true;
        }
    }

    let normalized = normalized.trim_matches('_').to_owned();
    if normalized.is_empty() {
        "_".to_owned()
    } else {
        normalized
    }
}
