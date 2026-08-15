//! Verifies the Creusot `Stoplight` accommodation model's exchange
//! bodies stay in real correspondence with the real
//! `amenable_kani::stoplight` bodies they mirror — a doc comment saying
//! "mirrors X" is not evidence; this is. See `EXCHANGE_PROOF_DERIVATION_
//! PLAN.md`'s Step 2 for why this is a consistency check, not a code
//! generator: the real body (`Result`-returning, `&self`-taking) and the
//! mirror body (bare return, free function) are not literally the same
//! tokens with names substituted — the two representations necessarily
//! differ in one well-defined way (the real body's trailing `Ok(...)`
//! wrapper, which the mirror's un-modeled `Result` has no equivalent
//! for), so "derive B from A" isn't well-defined the way it is for
//! Verus's verbatim predicate-text extraction. What *is* well-defined,
//! and is exactly what this test checks: strip that one documented
//! difference and the two bodies must be otherwise identical.
//!
//! Reads `amenable_kani::stoplight`'s real source directly off disk
//! (`fs::read_to_string`, not a Cargo dependency — the whole reason a
//! dependency edge isn't available is the subject of this plan's
//! "Creusot compilation model" design discussion) and compares it
//! against `amenable_creusot`'s own already-exported, `harness!`-
//! captured verbatim source constants (`VERIFY_*_EXCHANGE_SRC`) — the
//! same mechanism `amenable_std::creusot_witness` already relies on to
//! keep a reported claim from drifting from the real contract.

use std::{fs, path::Path};

use syn::{Expr, ImplItem, Item, Stmt};

/// Real `amenable_kani::stoplight` source, read directly off disk.
fn real_kani_stoplight_source() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../amenable_kani/src/stoplight.rs");
    fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "failed to read real Kani source at {}: {error}",
            path.display()
        )
    })
}

/// Find `impl Stoplight { fn NAME(...) { ... } }`'s real body — there is
/// exactly one such inherent-impl block per method in the real source
/// (see `stoplight.rs`'s own doc comment on why each transition got its
/// own `impl Stoplight` block).
fn extract_real_method_body(file: &syn::File, method_name: &str) -> syn::Block {
    for item in &file.items {
        let Item::Impl(item_impl) = item else {
            continue;
        };
        if item_impl.trait_.is_some() {
            continue;
        }
        for impl_item in &item_impl.items {
            if let ImplItem::Fn(method) = impl_item
                && method.sig.ident == method_name
            {
                return method.block.clone();
            }
        }
    }
    panic!("no `impl Stoplight` method named `{method_name}` found in the real Kani source");
}

/// The one documented, well-defined difference between the real body and
/// the mirror body: the real body's `Result`-returning shape ends in a
/// trailing `Ok(...)` call that the mirror (which doesn't model `Result`
/// at all) has no equivalent for. Strip it, keep everything else.
fn unwrap_trailing_ok(mut block: syn::Block, method_name: &str) -> syn::Block {
    match block.stmts.pop() {
        Some(Stmt::Expr(Expr::Call(call), None))
            if matches!(&*call.func, Expr::Path(path) if path.path.is_ident("Ok"))
                && call.args.len() == 1 =>
        {
            block
                .stmts
                .push(Stmt::Expr(call.args.into_iter().next().unwrap(), None));
            block
        }
        Some(other) => panic!(
            "`{method_name}`'s real body no longer ends in a trailing `Ok(...)` call \
             (found: {:?}) -- update `unwrap_trailing_ok` or the mirror body in \
             `amenable_creusot/src/stoplight.rs` to match the real shape",
            quote::quote!(#other).to_string()
        ),
        None => panic!("`{method_name}`'s real body is empty"),
    }
}

/// Parse a `harness!`-captured verbatim source constant back into its
/// function body.
fn mirror_body_from_harness_src(src: &str) -> syn::Block {
    let item_fn: syn::ItemFn = syn::parse_str(src).unwrap_or_else(|error| {
        panic!("failed to parse harness source as a function: {error}\n{src}")
    });
    *item_fn.block
}

fn assert_bodies_match(real_block: syn::Block, mirror_block: syn::Block, edge: &str) {
    let real_tokens = quote::quote!(#real_block).to_string();
    let mirror_tokens = quote::quote!(#mirror_block).to_string();
    assert_eq!(
        real_tokens, mirror_tokens,
        "the Creusot accommodation model for `{edge}` has drifted from the real \
         amenable_kani::stoplight body it mirrors -- update the body in \
         amenable_creusot/src/stoplight.rs to match the real transition logic \
         (after re-applying the documented trailing-`Ok(...)`-unwrap difference)"
    );
}

#[test]
fn green_to_yellow_mirror_matches_the_real_kani_body() {
    let file: syn::File =
        syn::parse_file(&real_kani_stoplight_source()).expect("valid Rust source");
    let real_block = unwrap_trailing_ok(
        extract_real_method_body(&file, "green_to_yellow"),
        "green_to_yellow",
    );
    let mirror_block =
        mirror_body_from_harness_src(amenable_creusot::VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC);
    assert_bodies_match(real_block, mirror_block, "green_to_yellow");
}

#[test]
fn yellow_to_red_mirror_matches_the_real_kani_body() {
    let file: syn::File =
        syn::parse_file(&real_kani_stoplight_source()).expect("valid Rust source");
    let real_block = unwrap_trailing_ok(
        extract_real_method_body(&file, "yellow_to_red"),
        "yellow_to_red",
    );
    let mirror_block =
        mirror_body_from_harness_src(amenable_creusot::VERIFY_YELLOW_TO_RED_EXCHANGE_SRC);
    assert_bodies_match(real_block, mirror_block, "yellow_to_red");
}

#[test]
fn red_to_green_mirror_matches_the_real_kani_body() {
    let file: syn::File =
        syn::parse_file(&real_kani_stoplight_source()).expect("valid Rust source");
    let real_block = unwrap_trailing_ok(
        extract_real_method_body(&file, "red_to_green"),
        "red_to_green",
    );
    let mirror_block =
        mirror_body_from_harness_src(amenable_creusot::VERIFY_RED_TO_GREEN_EXCHANGE_SRC);
    assert_bodies_match(real_block, mirror_block, "red_to_green");
}
