//! `harness!`: defines a `#[cfg(...)]`-gated proof harness item and, right
//! alongside it, an always-available `&'static str` constant holding the
//! harness's verbatim source — whitespace and all — so an audit report can
//! show a proof exactly as its author wrote it, not a machine-reflowed
//! approximation. Both come from the same braced group of tokens, so they
//! can never drift apart.

use proc_macro2::{Delimiter, Ident, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

/// Expand `harness!(cfg_name, CONST_NAME, { item })` into the `#[cfg(...)]`
/// gated `item` plus a `const CONST_NAME: &str = "...";` holding `item`'s
/// verbatim source text.
pub fn expand_harness(input: TokenStream) -> syn::Result<TokenStream> {
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

    Ok(quote! {
        #[cfg(#cfg_name)]
        #item

        const #const_name: &str = #source;
    })
}

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

fn expect_comma(
    tokens: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>,
) -> syn::Result<()> {
    match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => Ok(()),
        Some(other) => Err(Error::new_spanned(other, "expected `,`")),
        None => Err(Error::new(proc_macro2::Span::call_site(), "expected `,`")),
    }
}

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
fn trim_braces(text: &str) -> &str {
    text.trim()
        .strip_prefix('{')
        .and_then(|text| text.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(text)
}
