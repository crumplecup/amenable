//! Shared logic for locating and parsing real Verus carrier source
//! (`crates/amenable_verus/src/**/*.rs`) -- used both by `amenable_derive`'s
//! compile-time `verus_ensures_fragments!`/`verus_requires_fragments!`
//! macros and by `amenable_std`'s runtime `VerusCallShape` derivation, so
//! there is exactly one implementation of "how do you read a real
//! harness's real signature and clauses," not two. Lives here (not in
//! `amenable_derive` itself) because a `proc-macro = true` crate cannot
//! export anything but `#[proc_macro]`/`#[proc_macro_derive]`/
//! `#[proc_macro_attribute]` items -- confirmed by a real compiler
//! error, not assumed -- so the shared logic needs an ordinary crate
//! both `amenable_derive` and `amenable_std` can depend on;
//! `amenable_std` already depends on `amenable_core`, and
//! `amenable_derive` gaining a new dependency on `amenable_core` (never
//! the reverse) is an ordinary, acyclic edge.
//!
//! `verus_syn` -- the real parser `verus_builtin_macros` itself uses to
//! read a `verus! { ... }` block -- exposes a genuine `Signature.spec.
//! requires`/`.ensures` AST, not just opaque tokens, so this doesn't need
//! to hand-roll any part of Verus's own grammar. What it *does* need to
//! hand-roll: extracting the inner token stream a `verus! { ... }` macro
//! invocation wraps (`verus_syn` expects to parse that content directly,
//! not a whole ordinary Rust file with an embedded macro call --
//! confirmed by reading `verus_builtin_macros::syntax::rewrite_items`,
//! which does exactly this same two-step parse), and converting each
//! extracted clause `Expr` back to text by walking its token stream
//! directly rather than `Expr`'s own printer (so this doesn't need the
//! `visit`/`fold` `syn`/`verus_syn` features).
//!
//! Discovery has no registration to keep in sync either: given a harness
//! name, every `.rs` file under `amenable_verus/src` is scanned for a
//! matching `pub fn`. Cheap (a few dozen files) and correct by
//! construction -- there is no second list of "where harnesses live"
//! that could fall out of sync with where they actually live.

mod discovery;
mod module_path;
mod render;

pub use discovery::find_fn;
pub use render::{
    PredicateBodyError, literal_clauses, param_name, predicate_body, predicate_signature,
    walk_tokens,
};
