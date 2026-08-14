//! Emit derived Verus witness modules from registered artifact trees.
//!
//! Each generated module composes its leaves' *real* Verus proofs: a
//! `Checked` leaf's real harness gets called (looked up by harness name
//! in `amenable_std::verus_call_shape`, never assumed), its call
//! expression becomes part of the composite's own return value, and its
//! real `ensures`/`requires` predicates are cited verbatim against that
//! return value — never restated, never assumed as a free boolean.
//! (Verus `ensures` clauses can only reference a function's own
//! parameters and its declared `-> (result: T)` binding, never an
//! arbitrary body-local `let` — confirmed against the real `verus` tool
//! while building this: an earlier version tried `let result = call();`
//! with a bodyless-of-return-type function and got `cannot find value
//! "result" in this scope` at the `ensures` clause. The fix is to make
//! each checked leaf's call expression part of the composite's own
//! return value, exactly like a real hand-written carrier does.)
//! `Trivial` leaves contribute nothing (there is nothing to prove).
//! `Trusted` leaves contribute nothing checkable either — Verus has no
//! way to verify an externally-provenance-backed claim — but their
//! trust boundary is rendered as an explicit, auditable comment rather
//! than silently smuggled into the proof as an assumed premise.
//! `Opaque` leaves cannot reach this code at all:
//! `amenable_core::ClassifiedWitness` blocks them from ever being
//! exported, at `cargo check` time (see `amenable_core::witness`'s own
//! doc comments). Enum-shaped composites are not supported yet — a real
//! value only occupies one variant at a time, so composing one needs a
//! `match`-per-variant proof structure, not this file's flat
//! conjunction; that lands in a later phase.

use std::{
    fs,
    path::{Path, PathBuf},
};

use amenable_core::{
    MetadataEntry, WitnessArtifactNode, WitnessArtifactShape, WitnessExportSnapshot,
    WitnessSupportKind, witness_exports,
};
use amenable_std::verus_call_shape;

use crate::{AmenableError, AmenableResult};

/// Write one Verus proof module per registered `verus` witness export.
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
        .filter(|record| record.verifier == "verus")
    {
        match render_and_write_export(root, &export) {
            Ok(path) => written_paths.push(path),
            Err(error) => failures.push(format!("{}: {error}", export.evidence)),
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

fn render_and_write_export(root: &Path, export: &WitnessExportSnapshot) -> AmenableResult<PathBuf> {
    let module_segments = parse_destination_module(&export.destination_module)?;
    let output_path = ensure_module_tree(root, &module_segments)?;
    let source = render_verus_module(export, &module_segments)?;

    fs::write(&output_path, source).map_err(|error| AmenableError::io(&output_path, error))?;
    Ok(output_path)
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

/// One real typed parameter a composite's generated proof function
/// needs, already collision-resolved to a unique local name.
#[derive(Debug, Clone)]
struct RenderedParam {
    local_name: String,
    ty: String,
}

/// One checked leaf's real call expression and its real return type —
/// becomes one component of the composite's own return value.
#[derive(Debug, Clone)]
struct CheckedCall {
    expr: String,
    ty: String,
}

/// One not-yet-finalized `requires`/`ensures` clause: the harness's own
/// real clause text, with every `$paramname` placeholder already
/// substituted for this leaf's chosen local names, but `$result` left
/// literal until the final checked-call count is known. `result_index`
/// is relative to the owning subtree until merged into a parent, at
/// which point [`RenderedNode::merge`] rebases it — by the time
/// rendering reaches the module root, it's a direct, global index into
/// the root's own `checked_calls`.
#[derive(Debug, Clone)]
struct PendingClause {
    template: String,
    result_index: usize,
}

impl PendingClause {
    fn rebase(self, base: usize) -> Self {
        PendingClause {
            template: self.template,
            result_index: base + self.result_index,
        }
    }

    /// Resolve to real Verus source, once the final checked-call count
    /// is known: a single checked call's result is just `result`; with
    /// more than one, the composite returns a tuple and each call's
    /// result is `result.N`. A no-op if the template never referenced
    /// `$result` at all (some don't -- e.g. a precondition purely about
    /// a parameter).
    fn render(&self, checked_call_count: usize) -> String {
        let result_ref = if checked_call_count <= 1 {
            "result".to_owned()
        } else {
            format!("result.{}", self.result_index)
        };
        substitute_placeholder(&self.template, "result", &result_ref)
    }
}

/// The composed contribution of one artifact subtree: real parameters,
/// real checked-leaf calls, real cited predicates, and audit comments —
/// never an assumed boolean.
#[derive(Debug, Clone, Default)]
struct RenderedNode {
    params: Vec<RenderedParam>,
    checked_calls: Vec<CheckedCall>,
    requires: Vec<PendingClause>,
    ensures: Vec<PendingClause>,
    /// `(module_path, name)` pairs needing a `use` — a leaf's own
    /// `VerusCallShape::imports`, carried through unchanged (no need to
    /// rebase; imports aren't indexed by checked-call position).
    imports: Vec<(String, String)>,
    comments: Vec<String>,
}

impl RenderedNode {
    fn merge(&mut self, other: RenderedNode) {
        let base = self.checked_calls.len();
        self.params.extend(other.params);
        self.checked_calls.extend(other.checked_calls);
        self.requires
            .extend(other.requires.into_iter().map(|clause| clause.rebase(base)));
        self.ensures
            .extend(other.ensures.into_iter().map(|clause| clause.rebase(base)));
        self.imports.extend(other.imports);
        self.comments.extend(other.comments);
    }
}

/// Replace every `$name` occurrence in `template` with `replacement`,
/// leaving anything else (including other `$other` placeholders)
/// untouched. Placeholders are `$` followed by ASCII alphanumeric/`_`
/// characters; a bare `$` not followed by an identifier is left as-is.
fn substitute_placeholder(template: &str, name: &str, replacement: &str) -> String {
    let chars: Vec<char> = template.chars().collect();
    let mut output = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            let start = i + 1;
            let mut end = start;
            while end < chars.len() && (chars[end].is_ascii_alphanumeric() || chars[end] == '_') {
                end += 1;
            }
            let token: String = chars[start..end].iter().collect();
            if token == name {
                output.push_str(replacement);
                i = end;
                continue;
            }
        }
        output.push(chars[i]);
        i += 1;
    }

    output
}

// Only `Member` exists today: enum-shaped composites are rejected
// before any of their variants are ever walked (see `render_node`), so
// there's nothing that would construct a `Variant` segment yet. Add it
// back alongside the match-per-variant rendering that needs it.
#[derive(Debug, Clone)]
enum RouteSegment {
    Member(String),
}

/// Allocates unique local identifiers across an entire composite,
/// reusing a leaf's own real parameter names whenever nothing else
/// already claimed them, and falling back to a route-qualified name
/// only on an actual collision between sibling leaves.
#[derive(Debug, Default)]
struct NameAllocator {
    used: std::collections::HashSet<String>,
}

impl NameAllocator {
    fn allocate(&mut self, preferred: &str, route_hint: &str) -> String {
        if self.used.insert(preferred.to_owned()) {
            return preferred.to_owned();
        }

        let qualified = format!("{route_hint}_{preferred}");
        self.used.insert(qualified.clone());
        qualified
    }
}

fn render_verus_module(
    export: &WitnessExportSnapshot,
    module_segments: &[String],
) -> AmenableResult<String> {
    let module_name = module_segments
        .last()
        .expect("module path parsing guarantees at least one segment");
    let module_stem = normalize_identifier(module_name);
    let mut names = NameAllocator::default();
    let rendered = render_node(&export.artifact, &[], &mut names)?;
    let checked_call_count = rendered.checked_calls.len();

    let mut source = String::new();
    source.push_str(&format!(
        "//! Derived Verus closure for `{}`.\n\n",
        export.evidence
    ));
    source.push_str("use verus_builtin_macros::verus;\n");
    source.push_str("#[allow(unused_imports)]\n");
    source.push_str("use vstd::prelude::*;\n");

    let predicate_imports = predicate_import_lines(&rendered);
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
    source.push_str(&format!("// evidence: {}\n", export.evidence));
    source.push_str(&format!("// destination: {}\n", export.destination_module));
    source.push_str(&format!("// support: {}\n", export.support));

    if !rendered.comments.is_empty() {
        source.push('\n');
        for comment in &rendered.comments {
            source.push_str(comment);
            source.push('\n');
        }
    }
    source.push('\n');

    let params = rendered
        .params
        .iter()
        .map(|param| format!("{}: {}", param.local_name, param.ty))
        .collect::<Vec<_>>()
        .join(", ");
    let return_ty = match checked_call_count {
        0 => String::new(),
        1 => format!(" -> (result: {})", rendered.checked_calls[0].ty),
        _ => format!(
            " -> (result: ({}))",
            rendered
                .checked_calls
                .iter()
                .map(|call| call.ty.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    source.push_str(&format!(
        "pub fn verify_{module_stem}({params}){return_ty}\n"
    ));

    if !rendered.requires.is_empty() {
        source.push_str("    requires\n");
        for requires in &rendered.requires {
            source.push_str(&format!(
                "        {},\n",
                requires.render(checked_call_count)
            ));
        }
    }

    if !rendered.ensures.is_empty() {
        source.push_str("    ensures\n");
        for ensures in &rendered.ensures {
            source.push_str(&format!(
                "        {},\n",
                ensures.render(checked_call_count)
            ));
        }
    }

    let body = match checked_call_count {
        0 => String::new(),
        1 => rendered.checked_calls[0].expr.clone(),
        _ => format!(
            "({})",
            rendered
                .checked_calls
                .iter()
                .map(|call| call.expr.clone())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    };
    source.push_str(&format!("{{\n    {body}\n}}\n"));
    source.push_str("\n} // verus!\n");
    Ok(source)
}

/// Verus spec predicates need an explicit `use`, unlike ordinary
/// functions (which resolve fine via a fully qualified call path) —
/// confirmed against the real `verus` tool while building this.
fn predicate_import_lines(rendered: &RenderedNode) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut lines = Vec::new();

    for (module_path, name) in &rendered.imports {
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

fn render_node(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    match node.shape {
        WitnessArtifactShape::Leaf => render_leaf_node(node, route, names),
        WitnessArtifactShape::Enum => Err(AmenableError::invariant(format!(
            "Verus composition for enum-shaped witnesses is not supported yet (route: {}); \
             a real value only occupies one variant at a time, so this needs a match-per-variant \
             proof structure this renderer doesn't build yet -- exclude this type from Verus \
             export until it does",
            route_display(route)
        ))),
        WitnessArtifactShape::NamedStruct
        | WitnessArtifactShape::TupleStruct
        | WitnessArtifactShape::UnitStruct
        | WitnessArtifactShape::NamedVariant
        | WitnessArtifactShape::TupleVariant
        | WitnessArtifactShape::UnitVariant => {
            let mut combined = RenderedNode::default();
            for member in &node.members {
                let mut child_route = route.to_vec();
                child_route.push(RouteSegment::Member(member.label.clone()));
                combined.merge(render_node(&member.artifact, &child_route, names)?);
            }
            Ok(combined)
        }
    }
}

fn render_leaf_node(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    match node.kind {
        WitnessSupportKind::Trivial => Ok(RenderedNode::default()),
        WitnessSupportKind::Trusted => Ok(RenderedNode {
            comments: vec![render_trust_comment(node, route)],
            ..RenderedNode::default()
        }),
        WitnessSupportKind::Checked => render_checked_leaf(node, route, names),
        WitnessSupportKind::Mixed | WitnessSupportKind::Opaque => {
            Err(AmenableError::invariant(format!(
                "leaf at {} reports an impossible per-leaf support kind {:?} -- Mixed only \
                 arises from composing multiple leaves, and Opaque leaves cannot reach this \
                 renderer (amenable_core::ClassifiedWitness blocks them at cargo check time)",
                route_display(route),
                node.kind
            )))
        }
    }
}

fn render_checked_leaf(
    node: &WitnessArtifactNode,
    route: &[RouteSegment],
    names: &mut NameAllocator,
) -> AmenableResult<RenderedNode> {
    let harness = metadata_value(&node.metadata, "harness").ok_or_else(|| {
        AmenableError::invariant(format!(
            "checked leaf at {} has no \"harness\" metadata entry",
            route_display(route)
        ))
    })?;

    let shape = verus_call_shape(harness).ok_or_else(|| {
        AmenableError::invariant(format!(
            "no registered Verus call shape for harness `{harness}` (route: {}) -- register one \
             via amenable_std::register_verus_call_shape! before exporting this composite",
            route_display(route)
        ))
    })?;

    let route_hint = route_hint_name(route);
    let mut local_names: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    let mut params = Vec::new();

    for param in &shape.params {
        let local = names.allocate(&param.name, &route_hint);
        local_names.insert(param.name.clone(), local.clone());
        params.push(RenderedParam {
            local_name: local,
            ty: param.ty.clone(),
        });
    }

    let call_args = shape
        .params
        .iter()
        .map(|param| local_names[&param.name].clone())
        .collect::<Vec<_>>()
        .join(", ");
    let call = CheckedCall {
        expr: format!("{}::{}({call_args})", shape.module_path, shape.name),
        ty: match &shape.kind {
            amenable_std::VerusCallKind::Function { returns } => returns.clone(),
            amenable_std::VerusCallKind::Predicate => {
                return Err(AmenableError::invariant(format!(
                    "harness `{harness}` (route: {}) is registered as a bare predicate, not a \
                     callable function -- predicate-kind composition isn't wired up yet",
                    route_display(route)
                )));
            }
        },
    };

    let requires = shape
        .requires
        .iter()
        .map(|template| pending_clause(template, &local_names))
        .collect();
    let ensures = shape
        .ensures
        .iter()
        .map(|template| pending_clause(template, &local_names))
        .collect();
    let imports = shape
        .imports
        .iter()
        .map(|import| (import.module_path.clone(), import.name.clone()))
        .collect();

    let comment = format!(
        "// checked leaf at {}: calls {}::{}",
        route_display(route),
        shape.module_path,
        shape.name
    );

    Ok(RenderedNode {
        params,
        checked_calls: vec![call],
        requires,
        ensures,
        imports,
        comments: vec![comment],
    })
}

/// Substitute every `$paramname` placeholder (i.e. every placeholder
/// except `$result`) with this leaf's own chosen local name -- fully
/// resolved already, no rebasing needed. `$result` is left literal for
/// [`PendingClause::render`] to resolve once the final checked-call
/// count is known.
fn pending_clause(
    template: &str,
    local_names: &std::collections::HashMap<String, String>,
) -> PendingClause {
    let mut resolved = template.to_owned();
    for (name, local) in local_names {
        if name != "result" {
            resolved = substitute_placeholder(&resolved, name, local);
        }
    }

    PendingClause {
        template: resolved,
        result_index: 0,
    }
}

fn render_trust_comment(node: &WitnessArtifactNode, route: &[RouteSegment]) -> String {
    let mut line = format!("// trusted leaf at {}", route_display(route));

    let metadata = node
        .metadata
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

fn metadata_value<'a>(metadata: &'a [MetadataEntry], key: &str) -> Option<&'a str> {
    metadata
        .iter()
        .find(|entry| entry.key() == key)
        .map(MetadataEntry::value)
}

fn route_display(route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return "root".to_owned();
    }

    route
        .iter()
        .map(|segment| {
            let RouteSegment::Member(label) = segment;
            format!("member {label}")
        })
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn route_hint_name(route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return "root".to_owned();
    }

    route
        .iter()
        .map(route_segment_name)
        .collect::<Vec<_>>()
        .join("_")
}

fn route_segment_name(segment: &RouteSegment) -> String {
    let RouteSegment::Member(label) = segment;
    normalize_identifier(label)
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
