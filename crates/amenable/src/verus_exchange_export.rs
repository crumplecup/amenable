//! Emit derived Verus `Exchange`-edge companions from the real registry.
//!
//! Generates one `crate::exchange_support::verus_exchange!(..)` call per
//! real `Exchange` edge -- the mechanical `Witness<V>`/`Exchange<..>`
//! wiring `amenable_verus::gallery::stoplight_exchange` used to hand-copy
//! from `amenable_kani::stoplight`'s real body, with nothing guarding
//! drift between the two (unlike Creusot's own former mirror, which at
//! least had `stoplight_mirror_consistency_test.rs`). Reads `amenable_
//! core::ExchangeEdgeRecord`, the same registry `creusot_export`/`emit-
//! creusot-companions` already reads (`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
//! Step 9) -- registered by `#[amenable_derive::exchange(..)]` from
//! whichever crate owns the real edge (currently always `amenable_kani`),
//! safe to read from this ordinary, never-translated `amenable` binary.
//!
//! Structurally simpler than `creusot_export` in one respect: `verus_
//! exchange!`'s own macro definition already threads the real predicate
//! through automatically (`ensures <Evidence as Ensures<V>>::ensures(
//! result)`, resolved via whatever `verus_ensures!` call already
//! registered for that evidence type), so there is no trivial-contract
//! placeholder to hardcode here at all -- the generated call only ever
//! needs real type names and the real body, nothing else. `verus_
//! exchange!`'s own macro *definition* stays in `exchange_support.rs`,
//! untouched -- this only ever generates *call sites*.
//!
//! Deliberately narrow, matching `creusot_export`'s own scope exactly:
//! generates only the per-edge `verus_exchange!(..)` call, not the
//! surrounding state/token/sidecar type definitions or the `verus_
//! ensures!` calls carrying the real predicate, both of which stay
//! hand-written in each gallery module (`gallery::stoplight_exchange`,
//! `gallery::ledger_exchange`) -- stable, one-time infrastructure with
//! far lower drift risk than a transition body's own evolving logic.
//!
//! **Grouped by `(self_ty, method_name)`, not filtered to one `self_ty`.**
//! `creusot_export` still filters to `self_ty == "Stoplight"` (Creusot's
//! own connection for `Ledger::validate`/`::commit` isn't built yet);
//! this generator instead routes each explicitly-allowed edge into a
//! subdirectory named for its own group (`stoplight_exchange/` for
//! `Stoplight`'s three edges, `ledger_exchange/` for `Ledger`'s
//! `validate`/`commit`), so a new gallery module's edges land in their
//! own, separate `include!`-target directory automatically rather than
//! silently writing dead files into an unrelated one -- the exact
//! `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 9 mistake this crate's own
//! doc comment already warns about. `method_name`, not just `self_ty`,
//! has to gate this too: `Ledger` also registers `reject`/`rollback`
//! (`Rejected<Pending>`/`Rejected<Validated>`), which `gallery::
//! ledger_exchange` deliberately doesn't `include!` yet (see that
//! module's own doc comment for why) -- confirmed the hard way, not
//! assumed: an earlier version of this filter keyed on `self_ty` alone
//! and silently wrote `reject.rs`/`rollback.rs` into `ledger_exchange/`
//! as real dead files nothing `include!`s. [`edge_group`] is a real,
//! explicit lookup (not a mechanical `self_ty`-to-`snake_case`
//! transform, and not a blanket per-`self_ty` allow): the directory name
//! doesn't derive from `self_ty` mechanically (`stoplight_exchange` vs.
//! `Stoplight`) and which methods are covered doesn't derive from
//! `self_ty` alone either, so an honest table beats a formula that would
//! happen to produce the wrong answer on both counts.

use std::{
    fs,
    path::{Path, PathBuf},
};

use amenable_core::ExchangeEdgeRecord;

use crate::{AmenableError, AmenableResult};

/// Write one generated Verus `Exchange`-edge companion file per real,
/// explicitly-allowed edge registered via `amenable_core::
/// ExchangeEdgeRecord`, under `root/{group}/{method_name}.rs` (`root`
/// e.g. `amenable_verus/src/gallery/generated/`). Sorted by group then
/// method name so regeneration is deterministic and diffs stay minimal
/// -- the same convention `creusot_export` already uses. An edge with no
/// entry in [`edge_group`] is skipped, not written somewhere arbitrary
/// -- the same "no dead files, no silent guesses" discipline `creusot_
/// export`'s own filter already applies.
pub fn write_verus_exchange_companions(root: &Path) -> AmenableResult<Vec<PathBuf>> {
    let mut records: Vec<&ExchangeEdgeRecord> = inventory::iter::<ExchangeEdgeRecord>()
        .filter(|record| {
            edge_group(&tidy_stringified_type(record.self_ty), record.method_name).is_some()
        })
        .collect();
    records.sort_by_key(|record| (record.self_ty, record.method_name));

    let mut written = Vec::with_capacity(records.len());
    for record in records {
        let self_ty = tidy_stringified_type(record.self_ty);
        let Some(group) = edge_group(&self_ty, record.method_name) else {
            continue;
        };
        let dir = root.join(group);
        fs::create_dir_all(&dir).map_err(|error| AmenableError::io(&dir, error))?;

        let path = dir.join(format!("{}.rs", record.method_name));
        let source = render_companion(record);
        fs::write(&path, source).map_err(|error| AmenableError::io(&path, error))?;
        written.push(path);
    }

    Ok(written)
}

/// Maps a real `Exchange` edge's `(self_ty, method_name)` to the gallery
/// subdirectory its generated companion belongs in, or `None` if that
/// edge isn't connected here yet -- see this module's own doc comment
/// for why this is an explicit table, not a mechanical transform or a
/// per-`self_ty` blanket allow.
fn edge_group(self_ty: &str, method_name: &str) -> Option<&'static str> {
    static EDGES: &[(&str, &str, &str)] = &[
        ("Stoplight", "green_to_yellow", "stoplight_exchange"),
        ("Stoplight", "yellow_to_red", "stoplight_exchange"),
        ("Stoplight", "red_to_green", "stoplight_exchange"),
        ("Ledger", "validate", "ledger_exchange"),
        ("Ledger", "commit", "ledger_exchange"),
    ];
    EDGES
        .iter()
        .find(|(ty, method, _)| *ty == self_ty && *method == method_name)
        .map(|(_, _, dir)| *dir)
}

/// `verus_exchange!`'s own macro signature (`exchange_support.rs`):
/// `($self_ty, $self_param, $input_param: $input_ty, $output_ty,
/// $error_ty, $evidence, $verifier, $body)`. The gallery's own local
/// verifier marker is always `GalleryVerifier` (every gallery case
/// defines its own, matching `gallery::evidence_self_referential_root`'s
/// own doc comment for why), not derivable from `ExchangeEdgeRecord` --
/// `record.self_ty` is Kani's own `Stoplight`/`Ledger`, which happens to
/// match the gallery's own identically-named local type by convention,
/// the same way every other type name here does. `$self_param` is always
/// literally `self`, not a variable name -- unlike `$input_param`
/// (Kani's own real parameter names vary edge to edge), every real Kani
/// method this captures from is `fn method(&self, ..)`, so the captured
/// body's own uses of `self` are always the literal keyword. Passing it
/// through as an explicit macro argument still matters, even though its
/// spelling never varies: a real mixed-site hygiene bug, confirmed the
/// hard way building `gallery::ledger_exchange` (the first captured body
/// to actually call a `self.method(..)`, since every `Stoplight` edge's
/// real body happens not to) -- `self` hardcoded inside `verus_
/// exchange!`'s own template bound to the macro *definition's* hygiene
/// context, a different one from `self` as written in this generated
/// file's own `$body`, producing a real "self value is a keyword only
/// available in methods with a self parameter" error even though the
/// generated function plainly has a `&self` parameter. Fixed the same
/// way `$input_param` already was: the receiver name is now itself a
/// macro argument, so the declaration and `$body`'s own uses come from
/// the same call-site tokens.
fn render_companion(record: &ExchangeEdgeRecord) -> String {
    format!(
        "// AUTO-GENERATED by `amenable emit-verus-exchange-companions` --\n\
         // do not hand-edit; regenerate with `just generate-verus-exchange`.\n\
         // Source: `{self_ty}::{method_name}`'s real body, captured verbatim\n\
         // by `#[amenable_derive::exchange(..)]` and registered via\n\
         // `amenable_core::ExchangeEdgeRecord` -- see `amenable::\n\
         // verus_exchange_export`'s own doc comment. Plain `//`, not `//!`:\n\
         // this file is `include!`d mid-file into its own gallery module's\n\
         // scope, not its own module -- an inner doc comment there is a\n\
         // real compile error (E0753), the identical reason `creusot_export`\n\
         // uses plain `//` too.\n\
         \n\
         crate::exchange_support::verus_exchange!(\n\
         \x20\x20\x20\x20{self_ty},\n\
         \x20\x20\x20\x20self,\n\
         \x20\x20\x20\x20input: {input_ty},\n\
         \x20\x20\x20\x20{output_ty},\n\
         \x20\x20\x20\x20{error_ty},\n\
         \x20\x20\x20\x20{evidence},\n\
         \x20\x20\x20\x20GalleryVerifier,\n\
         \x20\x20\x20\x20{{\n\
         {body}\n\
         \x20\x20\x20\x20}}\n\
         );\n",
        self_ty = tidy_stringified_type(record.self_ty),
        method_name = record.method_name,
        input_ty = tidy_stringified_type(record.input_ty),
        output_ty = tidy_stringified_type(record.output_ty),
        error_ty = tidy_stringified_type(record.error_ty),
        evidence = tidy_stringified_type(record.evidence),
        body = indent_body(record.body),
    )
}

/// `stringify!(#ty)`'s one-space-per-token join (`Established < Green ,
/// GreenToken >`) survives untouched into the generated output --
/// confirmed for the identical reason `creusot_export`'s own version of
/// this function documents (`rustfmt` doesn't reformat inside a macro
/// invocation's own token tree by default). Byte-for-byte the same fix,
/// duplicated rather than shared: these two generators produce
/// structurally different output (a `fn` signature vs. a bare macro
/// argument list) and depend on different crates' own `AmenableResult`
/// plumbing, so sharing this one small helper isn't worth a new shared
/// module yet.
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
/// inside `verus_exchange!(.. { <body> })`'s own argument position -- the
/// identical dedent-then-reindent fix `creusot_export`'s own version of
/// this function documents in full (the captured text's first line loses
/// its original indentation to `.trim()` while every other line keeps
/// it), just at a shallower target depth (one argument position, not
/// three nested nesting levels inside `harness! { .. { fn .. {` ).
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
                &line[dedent_width.min(line.len())..]
            };
            format!("        {dedented}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
