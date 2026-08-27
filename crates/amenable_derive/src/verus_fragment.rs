//! `verus_ensures_fragments!`/`verus_requires_fragments!`: expand a real
//! harness name to a `&'static [&'static str]` array literal of its real
//! `ensures`/`requires` clauses, extracted from the real carrier source
//! at compile time via [`crate::verus_carrier`].
//!
//! A missing harness or missing clause is a real `syn::Error` here, not
//! a runtime failure discovered later by whichever caller happens to
//! invoke `Ensures::ensures()`/`Requires::requires()` first -- the crate
//! either compiles with the real, current clause text baked in as a
//! literal, or it doesn't compile at all.

use amenable_core::{verus_find_fn, verus_literal_clauses};
use syn::LitStr;

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_verus_fragments(
    input: proc_macro2::TokenStream,
    ensures: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let harness: LitStr = syn::parse2(input)?;
    let name = harness.value();

    let (_, _, item_fn) = verus_find_fn(&name).ok_or_else(|| {
        syn::Error::new(
            harness.span(),
            format!(
                "no real, public `pub fn {name}` found under `crates/amenable_verus/src` -- \
                 this macro derives its fragment from the real carrier source, so the harness \
                 must exist there"
            ),
        )
    })?;

    let clauses = verus_literal_clauses(&item_fn, ensures);
    if clauses.is_empty() {
        let kind = if ensures { "ensures" } else { "requires" };
        return Err(syn::Error::new(
            harness.span(),
            format!("harness `{name}` has no real `{kind}` clause to derive a fragment from"),
        ));
    }

    Ok(quote::quote! {
        &[#(#clauses),*]
    })
}
