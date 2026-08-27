//! `harness!`/`gallery_harness!`: define a `#[cfg(...)]`-gated proof
//! harness item and, right alongside it, an always-available `&'static
//! str` constant holding the harness's verbatim source — whitespace and
//! all — so an audit report can show a proof exactly as its author wrote
//! it, not a machine-reflowed approximation. Both come from the same
//! braced group of tokens, so they can never drift apart.
//!
//! `harness!`'s `kani` invocations also emit an `inventory` record for
//! the contained function, registering it as an executable, *tracked*
//! Kani proof (`amenable_kani::KaniProofRegistration`) -- discoverable by
//! `amenable verify kani`'s own registry-driven sweep with no source
//! scanning needed. `gallery_harness!` has the identical grammar and
//! `#[cfg(...)]`-gating behavior but never emits that record: proof-
//! gallery cases already separately register their own
//! `KaniGalleryRegistration` (disposition/expectation metadata, run only
//! via the dedicated `amenable gallery` subcommand) and are explicitly
//! *not* supposed to be part of the tracked, "does the suite still pass"
//! sweep -- confirmed the hard way: before this split existed, every
//! gallery case (many with deliberately expected `timeout`/`failed`
//! outcomes) also silently ended up in `KaniProofRegistration` purely
//! because it happened to be written through `harness!`, and `amenable
//! verify kani`'s full sweep ran all of them.

use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

/// Whether an expanded harness should also register itself as a tracked,
/// executable [`amenable_kani::KaniProof`] -- see this module's own doc
/// comment for why gallery harnesses deliberately don't.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessRegistration {
    Tracked,
    GalleryOnly,
}

/// Expand `harness!(cfg_name, CONST_NAME, { item })` into the `#[cfg(...)]`
/// gated `item` plus a `const CONST_NAME: &str = "...";` holding `item`'s
/// verbatim source text. When `registration` is [`HarnessRegistration::
/// Tracked`], a `kani` invocation additionally registers the contained
/// function as a tracked [`amenable_kani::KaniProof`].
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(input, registration))
)]
pub fn expand_harness(
    input: TokenStream,
    registration: HarnessRegistration,
) -> syn::Result<TokenStream> {
    let mut tokens = input.into_iter().peekable();

    let cfg_name = expect_ident(&mut tokens)?;
    expect_comma(&mut tokens)?;
    let const_name = expect_ident(&mut tokens)?;
    expect_comma(&mut tokens)?;
    let group = expect_brace_group(&mut tokens)?;

    if tokens.peek().is_some() {
        return Err(Error::new_spanned(
            TokenStream::from_iter(tokens),
            "harness! takes exactly three arguments: cfg_name, CONST_NAME, { item }",
        ));
    }

    let source = group
        .span()
        .source_text()
        .map(|text| trim_braces(&text).to_owned())
        .unwrap_or_else(|| group.stream().to_string());
    let item = group.stream();
    let kani_record = if cfg_name == "kani" && registration == HarnessRegistration::Tracked {
        let harness = syn::parse2::<syn::ItemFn>(item.clone()).map_err(|err| {
            let mut context = Error::new_spanned(
                &item,
                "a `kani` harness! invocation must contain one function item",
            );
            context.combine(err);
            context
        })?;
        let name = harness.sig.ident;

        quote! {
            ::inventory::submit! {
                ::amenable_kani::KaniProofRegistration::new(
                    || ::amenable_kani::KaniProof::new(
                        concat!(module_path!(), "::", stringify!(#name)).to_owned(),
                        module_path!()
                            .split_once("::")
                            .map_or_else(
                                || stringify!(#name).to_owned(),
                                |(_, module)| format!("{module}::{}", stringify!(#name)),
                            ),
                        "amenable_kani".to_owned(),
                    ),
                )
            }
        }
    } else {
        TokenStream::new()
    };

    Ok(quote! {
        #[cfg(#cfg_name)]
        #item

        /// Verbatim source of this harness, whitespace and all, captured
        /// at macro-expansion time (via `amenable_derive::harness!`) so it
        /// can never drift from the real contract.
        pub const #const_name: &str = #source;

        #kani_record
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tokens)))]
fn expect_ident(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>,
) -> syn::Result<Ident> {
    match tokens.next() {
        Some(TokenTree::Ident(ident)) => Ok(ident),
        Some(other) => Err(Error::new_spanned(other, "expected an identifier")),
        None => Err(Error::new(
            proc_macro2::Span::call_site(),
            "expected an identifier",
        )),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tokens)))]
fn expect_comma(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>,
) -> syn::Result<()> {
    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => Ok(()),
        Some(other) => Err(Error::new_spanned(other, "expected `,`")),
        None => Err(Error::new(proc_macro2::Span::call_site(), "expected `,`")),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tokens)))]
fn expect_brace_group(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>,
) -> syn::Result<proc_macro2::Group> {
    match tokens.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => Ok(group),
        Some(other) => Err(Error::new_spanned(other, "expected `{ ... }`")),
        None => Err(Error::new(
            proc_macro2::Span::call_site(),
            "expected `{ ... }`",
        )),
    }
}

/// Strip the outer `{`/`}` a brace group's own `source_text()` includes,
/// along with the whitespace immediately inside them.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn trim_braces(text: &str) -> &str {
    text.trim()
        .strip_prefix('{')
        .and_then(|text| text.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(text)
}
