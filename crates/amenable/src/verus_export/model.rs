//! The shared intermediate representation one artifact subtree composes
//! into, and the placeholder-substitution machinery that keeps a leaf's
//! `requires`/`ensures` templates honest until they're merged into a
//! parent's own clause list.

use std::collections::HashSet;
use tracing::instrument;

/// One real typed parameter a composite's generated proof function
/// needs, already collision-resolved to a unique local name.
#[derive(Debug, Clone)]
pub(super) struct RenderedParam {
    pub(super) local_name: String,
    pub(super) ty: String,
}

/// One checked leaf's real call expression and its real return type —
/// becomes one component of the composite's own return value.
#[derive(Debug, Clone)]
pub(super) struct CheckedCall {
    pub(super) expr: String,
    pub(super) ty: String,
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
pub(super) struct PendingClause {
    pub(super) template: String,
    pub(super) result_index: usize,
}

impl PendingClause {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn rebase(self, base: usize) -> Self {
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
    #[instrument(level = "debug", skip(self))]
    pub(super) fn render(&self, checked_call_count: usize) -> String {
        let result_ref = if checked_call_count <= 1 {
            "result".to_owned()
        } else {
            format!("result.{}", self.result_index)
        };
        self.render_with(&result_ref)
    }

    /// Resolve to real Verus source given an explicit reference string
    /// for this clause's `$result` occurrence, rather than deriving one
    /// from a top-level `result`/`result.N` tuple projection. Used by
    /// enum composition, where `$result` resolves to a locally bound
    /// name (`r`, or `r0`/`r1`/... for a multi-call variant) inside a
    /// `match result { ... }` arm instead.
    #[instrument(level = "trace", skip(self))]
    pub(super) fn render_with(&self, result_ref: &str) -> String {
        substitute_placeholder(&self.template, "result", result_ref)
    }
}

/// The composed contribution of one artifact subtree: real parameters,
/// real checked-leaf calls, real cited predicates, and audit comments —
/// never an assumed boolean.
#[derive(Debug, Clone, Default)]
pub(super) struct RenderedNode {
    pub(super) params: Vec<RenderedParam>,
    pub(super) checked_calls: Vec<CheckedCall>,
    pub(super) requires: Vec<PendingClause>,
    pub(super) ensures: Vec<PendingClause>,
    /// `(module_path, name)` pairs needing a `use` — a leaf's own
    /// `VerusCallShape::imports`, carried through unchanged (no need to
    /// rebase; imports aren't indexed by checked-call position).
    pub(super) imports: Vec<(String, String)>,
    pub(super) comments: Vec<String>,
}

impl RenderedNode {
    #[instrument(level = "debug", skip(self, other))]
    pub(super) fn merge(&mut self, other: RenderedNode) {
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
#[instrument(level = "debug")]
pub(super) fn substitute_placeholder(template: &str, name: &str, replacement: &str) -> String {
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

/// Allocates unique local identifiers across an entire composite,
/// reusing a leaf's own real parameter names whenever nothing else
/// already claimed them, and falling back to a route-qualified name
/// only on an actual collision between sibling leaves.
#[derive(Debug, Default)]
pub(super) struct NameAllocator {
    used: HashSet<String>,
}

impl NameAllocator {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn allocate(&mut self, preferred: &str, route_hint: &str) -> String {
        if self.used.insert(preferred.to_owned()) {
            return preferred.to_owned();
        }

        let qualified = format!("{route_hint}_{preferred}");
        self.used.insert(qualified.clone());
        qualified
    }
}
