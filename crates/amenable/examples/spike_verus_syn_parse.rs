//! Spike: can `verus_syn` (the real parser Verus's own `verus!` macro
//! uses internally) extract a real carrier's `requires`/`ensures` clauses
//! as structured data, rather than a human hand-typing them into
//! `register_verus_call_shape!`?
//!
//! Parses the real, checked-in `char_carrier.rs` and prints
//! `verify_char_roundtrip`'s real `ensures` clause, extracted from the
//! actual source rather than retyped -- for comparison against the
//! `$placeholder` template already hand-typed for this harness in
//! `amenable_std::verus_witness`.
//!
//! Run with: `cargo run --example spike_verus_syn_parse --features verus`

use verus_syn::{Item, parse::Parse};

/// Mirrors `verus_builtin_macros::syntax::Items` (private to that crate):
/// a bare sequence of items, exactly what sits inside a `verus! { ... }`
/// macro body.
struct Items {
    items: Vec<Item>,
}

impl Parse for Items {
    fn parse(input: verus_syn::parse::ParseStream) -> verus_syn::parse::Result<Items> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Items { items })
    }
}

fn main() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../amenable_verus/src/rust_std/char_carrier.rs"
    );
    let source = std::fs::read_to_string(path).expect("real carrier file should be readable");

    let file: syn::File = syn::parse_file(&source).expect("ordinary Rust syntax up to verus! {}");

    let verus_macro = file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Macro(item_macro) if item_macro.mac.path.is_ident("verus") => {
                Some(&item_macro.mac.tokens)
            }
            _ => None,
        })
        .expect("char_carrier.rs should contain a verus! { ... } block");

    let items: Items = verus_syn::parse2(verus_macro.clone())
        .expect("verus! body should parse as verus_syn items");

    let harness = items
        .items
        .iter()
        .find_map(|item| match item {
            Item::Fn(item_fn) if item_fn.sig.ident == "verify_char_roundtrip" => Some(item_fn),
            _ => None,
        })
        .expect("verify_char_roundtrip should be defined in char_carrier.rs");

    println!("parsed real signature for: {}", harness.sig.ident);

    let params = harness
        .sig
        .inputs
        .iter()
        .map(|arg| quote::quote!(#arg).to_string())
        .collect::<Vec<_>>();
    println!("params: {params:?}");

    match &harness.sig.spec.requires {
        Some(requires) => {
            for clause in &requires.exprs.exprs {
                println!("requires (real, parsed): {}", quote::quote!(#clause));
            }
        }
        None => println!("requires: none"),
    }

    match &harness.sig.spec.ensures {
        Some(ensures) => {
            for clause in &ensures.exprs.exprs {
                println!("ensures (real, parsed):  {}", quote::quote!(#clause));
            }
        }
        None => println!("ensures: none"),
    }

    println!();
    println!("hand-typed template in amenable_std::verus_witness today:");
    println!("  ensures = [");
    println!("      \"char_roundtrip_preserves_value($result, $c)\",");
    println!("      \"char_is_valid_unicode_scalar($c)\",");
    println!("  ]");
}
