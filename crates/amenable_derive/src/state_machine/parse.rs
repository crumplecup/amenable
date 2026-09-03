//! `#[derive(StateMachine)]`'s `#[state_machine(..)]` attribute parsing
//! into the shared `StateMachineBlock` / `StateDecl` / `EdgeDecl` decls.

use syn::{Attribute, Error, LitStr, Type};

use super::{EdgeDecl, RootDecl, StateDecl, StateMachineBlock, VerifierMode};

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr)))]
pub(super) fn parse_state_machine_block(attr: &Attribute) -> syn::Result<StateMachineBlock> {
    let mut verifier: Option<Type> = None;
    let mut generic_over_verifier = false;
    let mut states: Vec<StateDecl> = Vec::new();
    let mut edges: Vec<EdgeDecl> = Vec::new();
    let mut translator_cfg: Option<LitStr> = None;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("verifier") {
            let value: LitStr = meta.value()?.parse()?;
            verifier = Some(value.parse()?);
            return Ok(());
        }

        if meta.path.is_ident("generic_over_verifier") {
            generic_over_verifier = true;
            return Ok(());
        }

        if meta.path.is_ident("state") {
            states.push(parse_state_decl(&meta)?);
            return Ok(());
        }

        if meta.path.is_ident("edge") {
            edges.push(parse_edge_decl(&meta)?);
            return Ok(());
        }

        if meta.path.is_ident("translator_cfg") {
            translator_cfg = Some(meta.value()?.parse()?);
            return Ok(());
        }

        Err(meta.error("unsupported state_machine container attribute"))
    })?;

    let verifier = match (verifier, generic_over_verifier) {
        (Some(verifier), false) => VerifierMode::Concrete(Box::new(verifier)),
        (None, true) => VerifierMode::Generic,
        (Some(_), true) => {
            return Err(Error::new_spanned(
                attr,
                "state_machine accepts at most one of `verifier`/`generic_over_verifier`, not both",
            ));
        }
        (None, false) => {
            return Err(Error::new_spanned(
                attr,
                "state_machine requires either `verifier = \"...\"` or `generic_over_verifier`",
            ));
        }
    };

    Ok(StateMachineBlock {
        verifier,
        states,
        edges,
        translator_cfg,
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(meta)))]
fn parse_state_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<StateDecl> {
    let content;
    syn::parenthesized!(content in meta.input);

    let name: LitStr = content.parse()?;
    content.parse::<syn::Token![,]>()?;
    let carrier_lit: LitStr = content.parse()?;

    // Optional third positional arg: the real path to this state's root
    // constructor (`Established::<Green, GreenToken>::root`, not a call
    // -- the derive checks and invokes it, the caller never types the
    // call itself). Absent for every state with no lawful root worth
    // declaring this way.
    let root = if content.peek(syn::Token![,]) {
        content.parse::<syn::Token![,]>()?;
        let root_lit: LitStr = content.parse()?;
        let root_path: syn::Path = root_lit.parse()?;

        // Optional fourth positional arg: the real seed type a data-
        // needing root's constructor requires as its one real argument
        // (`Transfer::pending`'s own `TransferPayload`). Absent for a
        // zero-argument root -- `RootEntry::seed` reports `"()"` in
        // that case instead, see its own doc comment.
        let seed = if content.peek(syn::Token![,]) {
            content.parse::<syn::Token![,]>()?;
            let seed_lit: LitStr = content.parse()?;
            let seed_ty: Type = seed_lit.parse()?;
            Some((seed_ty, seed_lit))
        } else {
            None
        };

        Some(RootDecl {
            path: root_path,
            path_lit: root_lit,
            seed,
        })
    } else {
        None
    };

    Ok(StateDecl {
        name,
        carrier: carrier_lit.parse()?,
        root,
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(meta)))]
fn parse_edge_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<EdgeDecl> {
    let content;
    syn::parenthesized!(content in meta.input);

    let from: LitStr = content.parse()?;
    content.parse::<syn::Token![,]>()?;
    let to: LitStr = content.parse()?;

    Ok(EdgeDecl { from, to })
}
