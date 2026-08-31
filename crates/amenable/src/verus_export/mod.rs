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
//! doc comments). An export's own root shape may be enum-shaped — a real
//! value only occupies one variant at a time, so `render_enum_module`
//! composes it as one function taking a synthetic selector param and
//! returning a synthetic result enum, with a real `match selector { ...
//! }` in both the body and `ensures`, proving only the selected variant's
//! own composed claim in its arm (see Design E in
//! `docs/VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`). A nested enum shape
//! — a member inside a struct, or inside another variant — is still
//! rejected; no real nested-enum type is registered anywhere in this
//! codebase yet.
//!
//! Split by role: [`module_tree`] lays generated files out on disk and
//! drives the export sweep; [`model`] is the shared intermediate
//! representation a subtree composes into (`RenderedNode` and its
//! pieces) plus the placeholder-substitution machinery; [`identifiers`]
//! is pure identifier-casing; [`route`] names a position in the artifact
//! tree for error messages and local-name hints; [`tree_walk`] walks an
//! artifact tree into a `RenderedNode`; [`header`] renders the module
//! preamble shared by both render strategies; [`flat`] composes a
//! struct/tuple-struct-shaped export (and dispatches to [`enum_render`]
//! for an enum-shaped one).
//!
//! Only [`write_verus_witness_modules`] is visible outside this module
//! -- everything else is `pub(super)`, confined to this subtree the same
//! way it was implicitly confined to one file before the split.

mod enum_render;
mod flat;
mod header;
mod identifiers;
mod model;
mod module_tree;
mod route;
mod tree_walk;

pub use module_tree::write_verus_witness_modules;
