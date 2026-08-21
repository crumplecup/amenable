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

use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use verus_syn::parse::{Parse, ParseStream};

/// Mirrors `verus_builtin_macros::syntax::Items` (private to that
/// crate): a bare sequence of items, exactly what sits inside a
/// `verus! { ... }` macro body.
struct Items {
    items: Vec<verus_syn::Item>,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> verus_syn::parse::Result<Items> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Items { items })
    }
}

/// Root directory real carrier files live under, resolved relative to
/// this crate's own manifest -- the same relative depth
/// `amenable::paths::verus_source_directory` already uses for the
/// sibling crate (`amenable_core`, like `amenable`, sits directly under
/// `crates/`).
fn carrier_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_verus/src")
}

/// Every `.rs` file under `root`, recursively.
fn rust_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return files;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            files.push(path);
        }
    }

    files
}

/// Parse one real carrier file's `verus! { ... }` body into a sequence
/// of `verus_syn` items.
fn parse_carrier_items(path: &Path) -> Option<Vec<verus_syn::Item>> {
    let source = fs::read_to_string(path).ok()?;
    let file: syn::File = syn::parse_file(&source).ok()?;

    let verus_macro_tokens = file.items.iter().find_map(|item| match item {
        syn::Item::Macro(item_macro) if item_macro.mac.path.is_ident("verus") => {
            Some(item_macro.mac.tokens.clone())
        }
        _ => None,
    })?;

    let items: Items = verus_syn::parse2(verus_macro_tokens).ok()?;
    Some(items.items)
}

/// Find the real, public `pub fn`/`pub open spec fn` named `name`
/// anywhere under `amenable_verus/src`, returning its defining file and
/// crate-relative module path.
pub fn find_fn(name: &str) -> Option<(PathBuf, String, verus_syn::ItemFn)> {
    let root = carrier_root();

    for path in rust_files(&root) {
        let Some(items) = parse_carrier_items(&path) else {
            continue;
        };

        for item in items {
            if let verus_syn::Item::Fn(item_fn) = item
                && item_fn.sig.ident == name
                && matches!(item_fn.vis, verus_syn::Visibility::Public(_))
            {
                // `path` came from walking `root` itself (`rust_files(&root)`
                // above), so `module_path_for` can only fail here if that
                // walk is broken -- in which case there's no lawful module
                // path to report, so keep searching rather than trust an
                // unreachable branch never to fire.
                let Ok(module_path) = module_path_for(&root, &path) else {
                    continue;
                };
                return Some((path, module_path, item_fn));
            }
        }
    }

    None
}

/// `.../amenable_verus/src/rust_std/char_carrier.rs` (relative to
/// `root`) to `crate::rust_std::char_carrier`.
fn module_path_for(root: &Path, path: &Path) -> Result<String, std::path::StripPrefixError> {
    let relative = path.strip_prefix(root)?.with_extension("");

    let segments = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    Ok(format!("crate::{}", segments.join("::")))
}

/// The bound identifier a simple `pat` names, if it's a plain
/// identifier pattern (not a tuple/struct destructure).
pub fn param_name(pat: &verus_syn::Pat) -> Option<String> {
    match pat {
        verus_syn::Pat::Ident(pat_ident) => Some(pat_ident.ident.to_string()),
        _ => None,
    }
}

/// A token-stream walk serving two purposes at once: building a
/// `$placeholder` template string (any identifier exactly matching a
/// known parameter name or `result` gets a `$` prefix) and collecting
/// "call-like" identifiers (an identifier immediately followed by a
/// parenthesized group) that aren't themselves placeholders -- these
/// are real predicate/spec-fn references the generated `use`s need. An
/// empty `placeholders` set produces literal (non-templated) text.
///
/// Not a general pretty-printer -- just enough spacing awareness to
/// match ordinary Rust call/field-access/operator formatting for the
/// narrow expression grammar real `requires`/`ensures` clauses actually
/// use (calls, tuple projections, casts, unary `!`, comparisons,
/// `&&`/`==>`), so the generated text reads the same as a human would
/// have hand-typed it. Real `Punct` spacing (`Joint`/`Alone`) already
/// tells us when adjacent punctuation forms one compound operator
/// (`&&`, `==`, `<=`) versus two independent tokens.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PrintKind {
    Start,
    IdentOrLit,
    OpenDelim,
    CloseDelim,
    Dot,
    Comma,
    Bang,
    JointPunct,
    OtherPunct,
}

fn needs_space_before(last: PrintKind, next: PrintKind) -> bool {
    !matches!(
        (last, next),
        (PrintKind::Start, _)
            | (_, PrintKind::CloseDelim | PrintKind::Comma | PrintKind::Dot)
            | (
                PrintKind::OpenDelim | PrintKind::Dot | PrintKind::Bang | PrintKind::JointPunct,
                _
            )
            | (
                PrintKind::IdentOrLit | PrintKind::CloseDelim,
                PrintKind::OpenDelim
            )
    )
}

/// Render `tokens` back to text, `$`-prefixing any identifier in
/// `placeholders` and collecting call-like identifiers into `calls` --
/// see this module's own doc comment for the full rationale.
pub fn walk_tokens(
    tokens: proc_macro2::TokenStream,
    placeholders: &HashSet<String>,
    calls: &mut Vec<String>,
) -> String {
    let mut out = String::new();
    walk_tokens_into(tokens, placeholders, calls, &mut out, PrintKind::Start);
    out
}

/// Returns the [`PrintKind`] of the last token actually emitted, so a
/// caller resuming after a nested group knows what came before it.
fn walk_tokens_into(
    tokens: proc_macro2::TokenStream,
    placeholders: &HashSet<String>,
    calls: &mut Vec<String>,
    out: &mut String,
    start: PrintKind,
) -> PrintKind {
    let mut last = start;
    let mut previous_ident: Option<String> = None;

    for tt in tokens {
        let kind = match &tt {
            proc_macro2::TokenTree::Ident(_) | proc_macro2::TokenTree::Literal(_) => {
                PrintKind::IdentOrLit
            }
            proc_macro2::TokenTree::Group(group) => match group.delimiter() {
                proc_macro2::Delimiter::None => last,
                _ => PrintKind::OpenDelim,
            },
            proc_macro2::TokenTree::Punct(punct) => match punct.as_char() {
                '.' => PrintKind::Dot,
                ',' => PrintKind::Comma,
                '!' => PrintKind::Bang,
                _ if punct.spacing() == proc_macro2::Spacing::Joint => PrintKind::JointPunct,
                _ => PrintKind::OtherPunct,
            },
        };

        if needs_space_before(last, kind) {
            out.push(' ');
        }

        match tt {
            proc_macro2::TokenTree::Ident(ident) => {
                let name = ident.to_string();
                if placeholders.contains(&name) {
                    out.push('$');
                    previous_ident = None;
                } else {
                    previous_ident = Some(name.clone());
                }
                out.push_str(&name);
            }
            proc_macro2::TokenTree::Literal(literal) => {
                out.push_str(&literal.to_string());
                previous_ident = None;
            }
            proc_macro2::TokenTree::Punct(punct) => {
                out.push(punct.as_char());
                previous_ident = None;
            }
            proc_macro2::TokenTree::Group(group) => {
                let (open, close) = match group.delimiter() {
                    proc_macro2::Delimiter::Parenthesis => ("(", ")"),
                    proc_macro2::Delimiter::Bracket => ("[", "]"),
                    proc_macro2::Delimiter::Brace => ("{", "}"),
                    proc_macro2::Delimiter::None => ("", ""),
                };
                if group.delimiter() == proc_macro2::Delimiter::Parenthesis
                    && let Some(name) = previous_ident.take()
                    && !calls.contains(&name)
                {
                    calls.push(name);
                }
                out.push_str(open);
                let inner_last = walk_tokens_into(
                    group.stream(),
                    placeholders,
                    calls,
                    out,
                    PrintKind::OpenDelim,
                );
                out.push_str(close);
                last = match group.delimiter() {
                    proc_macro2::Delimiter::None => inner_last,
                    _ => PrintKind::CloseDelim,
                };
                previous_ident = None;
                continue;
            }
        }

        last = kind;
    }

    last
}

/// Extract a harness's real `requires` or `ensures` clauses as literal
/// text (no `$placeholder` substitution) -- for the compile-time
/// `verus_ensures_fragments!`/`verus_requires_fragments!` macros.
pub fn literal_clauses(item_fn: &verus_syn::ItemFn, ensures: bool) -> Vec<String> {
    let spec = if ensures {
        item_fn.sig.spec.ensures.as_ref().map(|e| &e.exprs)
    } else {
        item_fn.sig.spec.requires.as_ref().map(|r| &r.exprs)
    };

    spec.map(|specification| {
        specification
            .exprs
            .iter()
            .map(|expr| walk_tokens(quote::quote!(#expr), &HashSet::new(), &mut Vec::new()))
            .collect()
    })
    .unwrap_or_default()
}

/// Why [`predicate_body`] couldn't extract a real predicate's own body as
/// a single literal claim. Deliberately *not* a `std::error::Error`
/// citizen (no `derive_more::Error`, just `Display`): neither caller
/// ever holds, matches, or propagates this value as an error -- both
/// `map_err` it away immediately into their own real error (`syn::
/// Error`, `miette::Report`), touching it only through `Display`. It's
/// a descriptive reason consumed at one call site, not chainable data
/// like `ChainErrorKind`'s variants (which real callers hold, match on,
/// and need real location for) -- so it doesn't need `ChainError`'s
/// source-wrapping + location-tracking shape either.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum PredicateBodyError {
    /// The predicate's body has no statements at all.
    #[display("has an empty body")]
    Empty,
    /// The predicate's body isn't exactly one trailing expression.
    #[display("has a body that isn't a single trailing expression")]
    NotASingleExpression,
}

/// Extract a real `pub open spec fn NAME(...) -> bool { EXPR }`
/// predicate's own body as literal text -- its real, canonical
/// declaration, not any one caller's argument-substituted instance of
/// it. `Err` if the body isn't exactly one trailing expression (spec
/// fns are pure by Verus's own rules, so this covers every real one in
/// this codebase; a body with intermediate `let`s or other statements
/// would need a human decision about which part *is* the claim, which
/// this deliberately doesn't guess at).
pub fn predicate_body(item_fn: &verus_syn::ItemFn) -> Result<String, PredicateBodyError> {
    match item_fn.block.stmts.as_slice() {
        [verus_syn::Stmt::Expr(expr, None)] => Ok(walk_tokens(
            quote::quote!(#expr),
            &HashSet::new(),
            &mut Vec::new(),
        )),
        [] => Err(PredicateBodyError::Empty),
        _ => Err(PredicateBodyError::NotASingleExpression),
    }
}
