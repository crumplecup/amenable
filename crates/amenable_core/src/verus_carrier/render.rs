//! Render parsed Verus carrier fragments back into canonical text.

use std::collections::HashSet;

/// The bound identifier a simple `pat` names, if it's a plain
/// identifier pattern (not a tuple/struct destructure).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(pat)))]
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

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(last, next)))]
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
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(tokens, placeholders))
)]
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
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(tokens, placeholders, start))
)]
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
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(item_fn)))]
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
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(item_fn), err(level = "warn"))
)]
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

/// Render a real predicate's own `fn NAME(...) -> ReturnType` signature
/// as literal text -- real, derived from the same parsed `item_fn` as
/// [`predicate_body`], never hand-typed. Exists so a registered
/// `ContractRecord` fragment can carry a literal `fn` token immediately
/// followed by the predicate's real name: `cordial`'s own Creusot/Verus
/// call-shape recognition (`fragment_fn_name`, scoped to this workspace's
/// `~/repos/cordial`) scans a registered fragment's text for exactly that
/// pair to confirm a fragment is a real function definition, not a raw
/// restated clause coincidentally resembling one -- `predicate_body`
/// alone (the bare clause, `observed == input`) can never contain it,
/// which is why every `verus_ensures_predicate!`/`verus_requires_`
/// predicate!` real site went on recognizing nothing until this existed.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(item_fn)))]
pub fn predicate_signature(item_fn: &verus_syn::ItemFn) -> String {
    let signature = &item_fn.sig;
    walk_tokens(quote::quote!(#signature), &HashSet::new(), &mut Vec::new())
}
