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
mod render;

pub use discovery::find_fn;
pub use render::{
    PredicateBodyError, literal_clauses, param_name, predicate_body, predicate_signature,
    walk_tokens,
};

use std::{fs, path::Path};

/// `.../amenable_verus/src/rust_std/str_and_char/char_carrier.rs`
/// (relative to `root`) to `crate::rust_std::str_and_char::char_carrier`
/// -- or, when the leaf module hides itself behind a private `mod` plus
/// a `pub use` re-export of `name` one level up (the standard
/// `mod core; pub use core::Item;` idiom this crate's own mod.rs files
/// follow), the real, shorter reachable path instead
/// (`crate::rust_std::str_and_char`). Climbs repeatedly in case more
/// than one level is hidden this way. `name` is the specific function
/// being resolved, not just the leaf module: a hidden leaf can
/// re-export some of its items and not others, so a level only counts
/// as climbable once its re-export names `name` itself -- otherwise the
/// raw, file-location-derived segment is kept as the best (if
/// unreachable) answer available, rather than silently guessing.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(path)))]
fn module_path_for(
    root: &Path,
    path: &Path,
    name: &str,
) -> Result<String, std::path::StripPrefixError> {
    let relative = path.strip_prefix(root)?.with_extension("");

    let mut segments = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    while segments.len() > 1 {
        let Some((leaf, parent_segments)) = segments.split_last() else {
            break;
        };
        let declaring_file = if parent_segments.is_empty() {
            root.join("lib.rs")
        } else {
            root.join(parent_segments.join(std::path::MAIN_SEPARATOR_STR))
                .join("mod.rs")
        };

        match leaf_reachability(&declaring_file, leaf, name) {
            LeafReachability::HiddenAndReexported => {
                segments.pop();
            }
            LeafReachability::Public | LeafReachability::Unknown => break,
        }
    }

    Ok(format!("crate::{}", segments.join("::")))
}

/// Whether `leaf` (a `mod`/`pub mod` declared in `declaring_file`) is
/// directly public, or -- if hidden behind a private `mod` -- whether a
/// `pub use leaf::{ .., name, .. };` in the same file re-exports `name`
/// specifically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LeafReachability {
    /// `pub mod leaf;` -- the raw file-location segment is already the
    /// real, reachable path.
    Public,
    /// `mod leaf;` plus a `pub use leaf::{ .., name, .. };` naming
    /// `name` specifically -- the real path drops this segment.
    HiddenAndReexported,
    /// Everything else: `declaring_file` couldn't be read/parsed,
    /// `leaf` isn't declared there at all, or it's hidden with no
    /// matching re-export. Callers treat this the same as `Public` --
    /// stop climbing and keep the segment as written, rather than
    /// produce a path that doesn't actually resolve.
    Unknown,
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(declaring_file)))]
fn leaf_reachability(declaring_file: &Path, leaf: &str, name: &str) -> LeafReachability {
    let Ok(source) = fs::read_to_string(declaring_file) else {
        return LeafReachability::Unknown;
    };
    let Ok(file) = syn::parse_file(&source) else {
        return LeafReachability::Unknown;
    };

    let mod_declaration = file.items.iter().find_map(|item| match item {
        syn::Item::Mod(item_mod) if item_mod.ident == leaf => Some(&item_mod.vis),
        _ => None,
    });
    match mod_declaration {
        None => return LeafReachability::Unknown,
        Some(syn::Visibility::Public(_)) => return LeafReachability::Public,
        Some(_) => {}
    }

    let reexports_name = file.items.iter().any(|item| match item {
        syn::Item::Use(item_use) if matches!(item_use.vis, syn::Visibility::Public(_)) => {
            use_tree_export_names(&item_use.tree, leaf).contains(&name.to_string())
        }
        _ => false,
    });

    if reexports_name {
        LeafReachability::HiddenAndReexported
    } else {
        LeafReachability::Unknown
    }
}

/// The names a `use` tree re-exports out of `expected_leaf`, if its
/// first path segment is exactly `expected_leaf` -- e.g. `leaf::{a, b}`
/// against `expected_leaf = "leaf"` yields `["a", "b"]`. Only `Name`
/// and `Group` trees are real re-export shapes this crate's own mod.rs
/// files ever produce (see `mod_hiding`-style generation); a rename
/// (`leaf::a as b`) intentionally isn't treated as exporting `a` by its
/// original name, since the real reachable path would need the alias,
/// not the original identifier this function is asked to resolve.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tree)))]
fn use_tree_export_names(tree: &syn::UseTree, expected_leaf: &str) -> Vec<String> {
    match tree {
        syn::UseTree::Path(use_path) if use_path.ident == expected_leaf => {
            collect_use_names(&use_path.tree)
        }
        _ => Vec::new(),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tree)))]
fn collect_use_names(tree: &syn::UseTree) -> Vec<String> {
    match tree {
        syn::UseTree::Name(use_name) => vec![use_name.ident.to_string()],
        syn::UseTree::Group(group) => group.items.iter().flat_map(collect_use_names).collect(),
        _ => Vec::new(),
    }
}
