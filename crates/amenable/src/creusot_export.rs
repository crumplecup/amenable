//! Emit derived Creusot `Exchange`-edge companions from the real registry.
//!
//! Generates one `amenable_derive::harness! { creusot, .. }` block plus its
//! `ProofRecord` registration per real `Exchange` edge -- the same per-edge
//! content `amenable_creusot::stoplight` used to hand-write and hand-keep-
//! in-sync against `amenable_kani::stoplight`'s real body (via
//! `stoplight_mirror_consistency_test.rs`). Reads `amenable_core::
//! ExchangeEdgeRecord`, registered by `#[amenable_derive::exchange(..)]`
//! from whichever crate owns the real edge (currently always
//! `amenable_kani`) -- the same `inventory`-registry-then-generate pattern
//! `verus_export`/`emit-verus-witnesses` already uses for the witness-
//! composition system, applied here to `Exchange` edges. `inventory` never
//! appears in the generated output at all -- only in this ordinary,
//! never-translated `amenable` binary, which is what makes this safe for a
//! whole-crate-translator backend like Creusot (see `ExchangeEdgeRecord`'s
//! own doc comment).
//!
//! **A real, direct correction: this generator used to stop at the
//! inherent method.** `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4 —
//! generating only `impl {self_ty} { fn {method_name}(..) -> .. }` left
//! Creusot's own companion structurally lesser than Kani's real edges
//! (which always carry a real `Exchange` trait impl, not just a plain
//! method reachable by convention). Manual/absent `Exchange` impls are
//! the exact anti-pattern `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step
//! 5 already closed for `capture_exchange_body`; this generator now
//! closes the identical gap here, emitting a real, concrete `impl
//! amenable_core::Exchange<Input, Output, CreusotVerifier> for {self_ty}`
//! alongside the inherent method, delegating to it — the same
//! "compiler-checked trait conformance, not merely a same-named method"
//! guarantee `#[amenable_derive::exchange(..)]` already gives Kani's own
//! edges. Concrete, not generic over `V` (unlike `capture_exchange_body`'s
//! own generated impl for `Ledger`): every edge this generator connects
//! is a plain, non-generic method on the Creusot mirror, matching
//! `Stoplight`'s own Kani-side shape exactly, not `Ledger`'s.
//!
//! Deliberately narrow: generates only the per-edge `harness!`/
//! `ProofRecord` content, not the surrounding state/token/sidecar type
//! definitions (`Green`/`Yellow`/`Ledger`/`Transfer<S, Token>`/...), which
//! stay hand-written, stable, one-time accommodation-model infrastructure
//! in `amenable_creusot/src/{stoplight,ledger}.rs` -- low drift risk (they
//! rarely change), unlike the transition bodies this replaces hand-copying
//! of. Written via `include!`, not a `mod`, so each generated file shares
//! its own module's scope directly -- no `use super::*;`/explicit imports
//! needed for `Green`/`Yellow`/`Established`/`Ledger`/`Transfer`/etc.
//!
//! `#[requires(true)]` is still hardcoded -- no real edge in the tree
//! needs a genuine precondition on its *captured* body (`Ledger::commit`'s
//! own real `#[kani::requires]` guards a helper's exec arithmetic that
//! `commit`'s own body never runs; see `amenable_creusot::ledger`'s own
//! doc comment). `#[ensures(..)]` is not: `record.creusot_ensures` (`GAAP_
//! LEDGER_PLAN.md`'s Step 6) names the real Pearlite predicate to call,
//! defaulting to `"true"` for edges with no meaningful postcondition
//! (every `Stoplight` edge) -- see `ExchangeEdgeRecord::creusot_ensures`'s
//! own doc comment for why this can't route through `Ensures<V>`
//! mechanically the way Kani's/Verus's DFCC contracts do.

use std::{
    fs,
    path::{Path, PathBuf},
};

use amenable_core::ExchangeEdgeRecord;

use crate::{AmenableError, AmenableResult};

/// Write one generated Creusot companion file per real, explicitly-
/// allowed `Exchange` edge registered via `amenable_core::
/// ExchangeEdgeRecord`, under `root` (e.g. `amenable_creusot/src/
/// generated/`). Sorted by method name so regeneration is deterministic
/// and diffs stay minimal.
///
/// Filtered to an explicit `(self_ty, method_name)` allowlist -- matching
/// `amenable::verus_exchange_export`'s own `edge_group`, for the
/// identical real reason: this generator's whole model (a free function
/// whose body is `include!`d into a scope that already has matching-named
/// mirror types) is specific to each module's own accommodation-model
/// shape, and an edge with no `include!`-target scope ready for it would
/// be a real dead file nothing compiles -- confirmed directly, not
/// assumed, before the original `self_ty == "Stoplight"` filter was
/// added. `Ledger`'s own `reject`/`rollback` edges joined the allowlist
/// in a later pass (`GAAP_LEDGER_PLAN.md`'s Step 7, revisited): both
/// stayed unconnected at first as a real scope call, not a technical
/// wall -- legitimately trivial, no new proof content -- but `Stoplight`'s
/// own equally trivial edges were always connected everywhere, so the
/// asymmetry was worth closing once the underlying capture mechanism
/// (`#[amenable_derive::capture_exchange_body(..)]`) was proven out on
/// `validate`/`commit`.
pub fn write_creusot_exchange_companions(root: &Path) -> AmenableResult<Vec<PathBuf>> {
    fs::create_dir_all(root).map_err(|error| AmenableError::io(root, error))?;

    let mut records: Vec<&ExchangeEdgeRecord> = inventory::iter::<ExchangeEdgeRecord>()
        .filter(|record| {
            edge_module(&tidy_stringified_type(record.self_ty), record.method_name).is_some()
        })
        .collect();
    records.sort_by_key(|record| record.method_name);

    let mut written = Vec::with_capacity(records.len());
    for record in records {
        let self_ty = tidy_stringified_type(record.self_ty);
        let Some(module) = edge_module(&self_ty, record.method_name) else {
            continue;
        };

        let path = root.join(format!("{}.rs", record.method_name));
        let source = render_companion(record, module);
        fs::write(&path, source).map_err(|error| AmenableError::io(&path, error))?;
        written.push(path);
    }

    Ok(written)
}

/// Maps a real `Exchange` edge's `(self_ty, method_name)` to the
/// `amenable_creusot` module its generated companion `include!`s into
/// (and whose evidence-registry path it reports under), or `None` if
/// that edge isn't connected here yet -- see this module's own doc
/// comment for why this is an explicit table.
fn edge_module(self_ty: &str, method_name: &str) -> Option<&'static str> {
    static EDGES: &[(&str, &str, &str)] = &[
        ("Stoplight", "green_to_yellow", "stoplight"),
        ("Stoplight", "yellow_to_red", "stoplight"),
        ("Stoplight", "red_to_green", "stoplight"),
        ("Ledger", "validate", "ledger"),
        ("Ledger", "commit", "ledger"),
        ("Ledger", "reject", "ledger"),
        ("Ledger", "rollback", "ledger"),
    ];
    EDGES
        .iter()
        .find(|(ty, method, _)| *ty == self_ty && *method == method_name)
        .map(|(_, _, module)| *module)
}

fn render_companion(record: &ExchangeEdgeRecord, module: &str) -> String {
    let const_name = format!("VERIFY_{}_EXCHANGE_SRC", record.method_name.to_uppercase());
    let evidence_path = format!("amenable_creusot::{module}::{}", record.method_name);
    // See `ExchangeEdgeRecord::method_generics`'s own doc comment: `""`
    // (every edge but `Ledger::validate`) renders no generic parameter
    // list at all; a real captured turbofish (`Ledger::validate`'s own
    // `check_amount_positive::<V>(..)` call) needs the generated
    // function to declare the identical parameter for that text to
    // resolve.
    let method_generics = if record.method_generics.is_empty() {
        String::new()
    } else {
        format!("<{}>", record.method_generics)
    };
    // The generated inherent method's own `<V>` (when present) is an
    // unconstrained, never-referenced placeholder -- it exists only so a
    // captured turbofish inside the real body (e.g. `Ledger::validate`'s
    // own `Self::check_amount_positive::<V>(..)`) has something to name,
    // not because the Creusot mirror is meaningfully generic over
    // verifier the way the real `amenable_gaap` method is. The `Exchange`
    // impl below is concrete to `CreusotVerifier` either way -- every
    // edge this generator connects is a plain method on the Creusot
    // mirror, never a real multi-backend one -- so the delegating call
    // just needs *some* concrete type to satisfy that unconstrained
    // parameter; `CreusotVerifier` is the natural, meaningful choice.
    let exchange_turbofish = if record.method_generics.is_empty() {
        String::new()
    } else {
        "::<CreusotVerifier>".to_owned()
    };

    format!(
        "// AUTO-GENERATED by `amenable emit-creusot-companions` -- do not\n\
         // hand-edit; regenerate with `just generate-creusot`.\n\
         // Source: `{self_ty}::{method_name}`'s real body, captured verbatim\n\
         // by `#[amenable_derive::exchange(..)]`/`#[amenable_derive::\n\
         // capture_exchange_body(..)]` and registered via `amenable_core::\n\
         // ExchangeEdgeRecord` -- see `amenable::creusot_export`'s own doc\n\
         // comment. Plain `//`, not `//!`: this file is `include!`d\n\
         // mid-file into its own module's scope, not its own module -- an\n\
         // inner doc comment there is a real compile error (E0753,\n\
         // confirmed against a real build), not just a style choice.\n\
         \n\
         amenable_derive::harness! {{\n\
         \x20\x20\x20\x20creusot, {const_name}, {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20impl {self_ty} {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20#[requires(true)]\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20#[ensures({creusot_ensures})]\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20fn {method_name}{method_generics}(&self, input: {input_ty}) -> Result<{output_ty}, {error_ty}> {{\n\
         {body}\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20}}\n\
         \x20\x20\x20\x20\x20\x20\x20\x20}}\n\
         \x20\x20\x20\x20}}\n\
         }}\n\
         \n\
         // A real, concrete `Exchange` impl, not just a same-named\n\
         // method -- delegates to the harness-captured method above, the\n\
         // same mechanical shape `#[amenable_derive::exchange(..)]`\n\
         // already gives every real Kani-side edge. `#[cfg(creusot)]`,\n\
         // matching the mirror types (`{self_ty}`/carriers) it names,\n\
         // which only exist under that cfg. Carries its own `#[ensures(\n\
         // ..)]`, the identical text the inherent method above has --\n\
         // Creusot has no mechanical call-through the way Kani's/Verus's\n\
         // `Ensures<V>` dispatch does (`ExchangeEdgeRecord::\n\
         // creusot_ensures`'s own doc comment), and Creusot only checks\n\
         // what a function's own contract actually states: a delegating\n\
         // method with no `#[ensures(..)]` of its own is checked for\n\
         // memory/panic safety only, never against the real claim,\n\
         // confirmed the hard way -- a first version omitted this and a\n\
         // real injected bug (swapping the body for a fabricated `Err`)\n\
         // produced no failure at all.\n\
         #[cfg(creusot)]\n\
         impl ::amenable_core::Exchange<{input_ty}, {output_ty}, CreusotVerifier> for {self_ty} {{\n\
         \x20\x20\x20\x20type Error = {error_ty};\n\
         \n\
         \x20\x20\x20\x20#[requires(true)]\n\
         \x20\x20\x20\x20#[ensures({creusot_ensures})]\n\
         \x20\x20\x20\x20fn exchange(&self, input: {input_ty}) -> ::std::result::Result<{output_ty}, Self::Error> {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20self.{method_name}{exchange_turbofish}(input)\n\
         \x20\x20\x20\x20}}\n\
         }}\n\
         \n\
         #[cfg(not(creusot))]\n\
         ::inventory::submit! {{\n\
         \x20\x20\x20\x20::amenable_core::ProofRecord {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20evidence: \"{evidence_path}\",\n\
         \x20\x20\x20\x20\x20\x20\x20\x20verifier: \"creusot\",\n\
         \x20\x20\x20\x20\x20\x20\x20\x20describe: || {const_name}.to_owned(),\n\
         \x20\x20\x20\x20}}\n\
         }}\n",
        self_ty = tidy_stringified_type(record.self_ty),
        method_name = record.method_name,
        input_ty = tidy_stringified_type(record.input_ty),
        output_ty = tidy_stringified_type(record.output_ty),
        error_ty = tidy_stringified_type(record.error_ty),
        body = indent_body(record.body),
        creusot_ensures = record.creusot_ensures,
    )
}

/// `stringify!(#ty)` (used to capture `ExchangeEdgeRecord`'s type fields
/// at macro-expansion time, in `amenable_derive::exchange`) joins tokens
/// with a single space each, so `Established<Green, GreenToken>` comes
/// back as `"Established < Green , GreenToken >"` -- valid Rust, but
/// `rustfmt` never sees it: it doesn't reformat inside an opaque macro
/// invocation's own token tree by default, confirmed directly (`rustfmt`
/// run on a real generated file left `harness! { .. }`'s contents
/// untouched). Fixed here, not by relying on a later `cargo fmt` pass:
/// strip the space `stringify!` inserts immediately before/after `<` and
/// immediately before `,`/`>` -- the only spacing artifacts generic-type
/// syntax actually produces, so this is a narrow, well-defined text fix,
/// not a general Rust formatter. Confirmed against a real generated file
/// through two passes, not assumed correct on the first attempt: an
/// earlier version only stripped the space *after* `<`, leaving
/// `Established <Green, GreenToken>`'s space *before* `<` untouched.
fn tidy_stringified_type(stringified: &str) -> String {
    let mut tidied = String::with_capacity(stringified.len());
    let mut chars = stringified.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' if chars.peek() == Some(&'<') => {}
            '<' => {
                tidied.push(ch);
                while chars.peek() == Some(&' ') {
                    chars.next();
                }
            }
            ' ' if matches!(chars.peek(), Some(',') | Some('>')) => {}
            _ => tidied.push(ch),
        }
    }

    tidied
}

/// Re-indent a captured method body's own lines to sit correctly nested
/// inside the generated `harness! { .. { impl .. { fn .. { <body> } } } }`
/// shape. Two real, confirmed passes, not one: `rustfmt` doesn't reformat
/// inside `harness! { .. }`'s own opaque macro invocation (confirmed
/// directly against a real generated file), so this has to get the
/// whitespace right itself, not defer to a later `cargo fmt` pass that
/// never actually reaches this text.
///
/// 1. **Dedent.** The captured text is `Span::source_text()` on the
///    method's own `{ .. }` block, then `.trim()`-stripped of its outer
///    braces (`amenable_derive::exchange`'s own `trim_braces`, mirroring
///    `harness!`'s identical helper) -- `.trim()` only touches the very
///    start/end of the *whole* string, so the first line loses its
///    original indentation (right after the opening `{`) while every
///    other line keeps its real, original column from `amenable_kani::
///    stoplight.rs`. Confirmed as a real bug, not assumed: a first,
///    naive version that only re-indented uniformly (no dedent pass)
///    produced visibly double-indented non-first lines in a real
///    generated file. Fixed by finding the minimum indentation among
///    the non-first, non-empty lines and stripping exactly that much
///    from all of them, before re-indenting everything uniformly.
/// 2. **Re-indent.** Once every line starts at a shared, zero baseline,
///    prefix each non-empty line with the real target indentation for
///    this generated shape (16 spaces: inside `harness! { .. { impl .. {
///    fn .. {` 's own four nesting levels -- widened from three once the
///    generated `fn` gained a real `impl {self_ty} { .. }` wrapper, `GAAP_
///    LEDGER_PLAN.md`'s Step 6, so a captured body referencing `self`/
///    `Self` -- `Ledger::validate`'s own real body, the first captured
///    body to need this -- has something to resolve against).
fn indent_body(body: &str) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let dedent_width = lines
        .iter()
        .skip(1)
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            if line.trim().is_empty() {
                return String::new();
            }
            let dedented = if index == 0 {
                *line
            } else {
                dedent_line(line, dedent_width)
            };
            format!("                {dedented}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Strip `dedent_width` bytes off `line`'s start, rounding down to the
/// nearest real `char` boundary at or before that byte offset -- a plain
/// `&line[dedent_width..]` panics if `dedent_width` (the minimum leading-
/// whitespace width across every *other* line) doesn't land on a char
/// boundary in *this* line, which nothing here guarantees for arbitrary
/// captured source text even though real Rust indentation is ASCII
/// whitespace in practice.
fn dedent_line(line: &str, dedent_width: usize) -> &str {
    let target = dedent_width.min(line.len());
    let boundary = (0..=target)
        .rev()
        .find(|&i| line.is_char_boundary(i))
        .unwrap_or(0);
    line.get(boundary..).unwrap_or(line)
}
