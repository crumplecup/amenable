//! `#[derive(StateMachine)]`'s code emission: the per-edge static
//! assertions and the `impl amenable_core::StateMachine<V> for Self` body.
//! Parsing is in `parse`; the shared decl types and entry points are in
//! the module root.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, LitStr, Type};

use super::{RootDecl, StateDecl, StateMachineBlock, VerifierMode};

/// One shared, top-level generic checker function per block, referenced
/// (never called) once per edge via a plain `const _: fn() = ..;` item —
/// not the closure-wrapped, per-edge-nested-function shape this used to
/// generate. A real, confirmed correction: that shape (`const _: fn() =
/// || { fn assert(..) {} assert::<Self>(); };`) triggered a genuine
/// `creusot-rustc` ICE (`naming.rs`'s `ComaNames::get`, "no entry found
/// for key", during `translate_function` -- a compiler panic, not a
/// lint), isolated by temporarily emitting only the static assertions
/// with no trait impl (still panicked) and then only the trait impl
/// with no static assertions (compiled clean), confirming the assertion
/// shape itself was the cause, not `audit_surface()`'s `inventory` call
/// as first assumed. A bare item reference (function-item-to-`fn()`-
/// pointer coercion, no closure, no nested function definition) still
/// forces the identical compiler-enforced bound check -- instantiating
/// `#checker_fn::<Input, Output, Self>` as a value requires `Self:
/// Exchange<Input, Output, Verifier>` to hold, exactly like a direct
/// call would -- without whatever specific nesting shape `creusot-rustc`
/// can't translate.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self_ty, block)))]
pub(super) fn expand_block_assertions(
    self_ty: &syn::Ident,
    block: &StateMachineBlock,
) -> syn::Result<TokenStream> {
    // One flat `const` per declared root, not a shared generic checker
    // -- each root's own carrier (and seed, if any) type is already
    // fixed by its own declaration, so there's no genericity to share
    // across states the way the edge checker shares `In`/`Out`/`T`.
    // Independent of verifier mode: a root constructor's own signature
    // has nothing to do with `V` at all, unlike an edge's `Exchange`
    // bound. Same flat, no-closure shape regardless: a bare item
    // reference to a real path, checked by the compiler assigning it to
    // a precisely-typed `fn(..) -> Carrier` const -- if the named path
    // doesn't exist, doesn't accept exactly the declared seed (or any
    // argument at all, for a zero-argument root), or doesn't return
    // exactly the declared carrier, this fails to compile with a real,
    // precise error, not a silent gap.
    let root_checks: TokenStream = block
        .states
        .iter()
        .filter_map(|state| {
            let root_decl = state.root.as_ref()?;
            let path = &root_decl.path;
            let carrier = &state.carrier;
            Some(match &root_decl.seed {
                Some((seed_ty, _)) => quote! {
                    const _: fn(#seed_ty) -> #carrier = #path;
                },
                None => quote! {
                    const _: fn() -> #carrier = #path;
                },
            })
        })
        .collect();

    let VerifierMode::Concrete(verifier) = &block.verifier else {
        // No edge static assertion here -- see this module's own doc
        // comment for why a "for every V: Verifier" check is provably
        // too strong (real edges are only generic over V conditionally,
        // bounded by real Witness/Ensures/Requires impls this derive
        // has no way to know per edge) and why that's fine:
        // capture_exchange_body's own generated impl is already the
        // real compile-time check. Root checks above still apply.
        return Ok(root_checks);
    };

    let checker_fn = format_ident!(
        "__assert_{}_state_machine_edge",
        self_ty.to_string().to_lowercase()
    );
    let references = block
        .edges
        .iter()
        .map(|edge| {
            let from_carrier = find_state_carrier(&block.states, &edge.from)?;
            let to_carrier = find_state_carrier(&block.states, &edge.to)?;

            Ok(quote! {
                const _: fn() = #checker_fn::<#from_carrier, #to_carrier, #self_ty>;
            })
        })
        .collect::<syn::Result<TokenStream>>()?;

    Ok(quote! {
        #[doc(hidden)]
        fn #checker_fn<In, Out, T>()
        where
            In: ::amenable_core::Sidecar<#verifier>,
            Out: ::amenable_core::Sidecar<#verifier>,
            T: ::amenable_core::Exchange<In, Out, #verifier>,
        {
        }

        #references
        #root_checks
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self_ty, block)))]
pub(super) fn expand_block_state_machine_impl(
    self_ty: &syn::Ident,
    block: &StateMachineBlock,
) -> TokenStream {
    let self_ty_str = self_ty.to_string();

    let state_names = block.states.iter().map(|state| &state.name);
    let transitions = block.edges.iter().map(|edge| {
        let from = &edge.from;
        let to = &edge.to;
        quote! { ::amenable_core::Transition::new(#from, #to) }
    });

    let (impl_generics, verifier) = match &block.verifier {
        VerifierMode::Concrete(verifier) => (quote! {}, quote! { #verifier }),
        VerifierMode::Generic => (quote! { <V: ::amenable_core::Verifier> }, quote! { V }),
    };

    let audit_surface_body = quote! {
        let mut audits: ::std::vec::Vec<::amenable_core::TransitionAudit> =
            ::inventory::iter::<::amenable_core::ExchangeEdgeRecord>()
                .filter(|record| record.self_ty() == #self_ty_str)
                .map(|record| {
                    ::amenable_core::TransitionAudit::new(
                        record.evidence().to_string(),
                        record.method_name().to_string(),
                        record.body().to_string(),
                    )
                })
                .collect();

        audits.sort_by(|left, right| {
            (left.to(), left.method_name()).cmp(&(right.to(), right.method_name()))
        });

        audits
    };

    // `translator_cfg` is opt-in, per block -- not baked unconditionally
    // into every use of this derive. Only a block whose real `Exchange`
    // impls live *inside* a crate a translator-based backend (Creusot)
    // actually translates needs this at all (confirmed the hard way: a
    // real `creusot-rustc` ICE, compiler panic not a lint, from an
    // earlier version that always emitted an ungated `inventory::iter`
    // call here). Every other user of this derive so far (`amenable_
    // kani`/`amenable_gaap`, neither ever translated by anything) has no
    // reason to know a cfg named `creusot` exists at all -- baking the
    // split in unconditionally leaked that knowledge into their own
    // `Cargo.toml` `check-cfg` lists, a real, direct correction: cfg
    // awareness belongs only in the one crate that's actually
    // translated, matching this whole codebase's "verifier backends
    // never depend on each other, not even a cfg name" discipline.
    let audit_surface = match &block.translator_cfg {
        None => quote! {
            fn audit_surface() -> ::std::vec::Vec<::amenable_core::TransitionAudit> {
                #audit_surface_body
            }
        },
        Some(cfg_name) => {
            let cfg_ident = proc_macro2::Ident::new(&cfg_name.value(), cfg_name.span());
            quote! {
                // Two separate item-level `#[cfg(..)]`-gated definitions,
                // not one body with an inner `#[cfg(..)]` block -- cfg
                // only applies to items/statements, not arbitrary tail
                // expressions, and exactly one of these two survives
                // stripping either way.
                #[cfg(not(#cfg_ident))]
                fn audit_surface() -> ::std::vec::Vec<::amenable_core::TransitionAudit> {
                    #audit_surface_body
                }

                // Honest, not aspirational: `inventory` genuinely cannot
                // run inside real translation, so there is nothing real
                // to report from here, ever, matching every other real
                // `#[cfg(not(#cfg_ident))]`-only registry query in this
                // codebase (e.g. the old `Amenable::verus_surface()`).
                #[cfg(#cfg_ident)]
                fn audit_surface() -> ::std::vec::Vec<::amenable_core::TransitionAudit> {
                    ::std::vec::Vec::new()
                }
            }
        }
    };

    // Only overrides the trait's own empty default when at least one
    // state actually declared a root -- most blocks have none, and the
    // default already says exactly that honestly.
    let root_entries_states: Vec<(&StateDecl, &RootDecl)> = block
        .states
        .iter()
        .filter_map(|state| state.root.as_ref().map(|root| (state, root)))
        .collect();
    let root_entries = if root_entries_states.is_empty() {
        quote! {}
    } else {
        let entries = root_entries_states.iter().map(|(state, root_decl)| {
            let name = &state.name;
            let root_str = &root_decl.path_lit;
            let seed_str = match &root_decl.seed {
                Some((_, seed_lit)) => quote! { #seed_lit },
                None => quote! { "()" },
            };
            quote! {
                ::amenable_core::RootEntry::new(#name, #root_str, #seed_str)
            }
        });

        quote! {
            fn root_entries() -> &'static [::amenable_core::RootEntry] {
                const ROOT_ENTRIES: &[::amenable_core::RootEntry] = &[#(#entries),*];
                ROOT_ENTRIES
            }
        }
    };

    let state_machine_impl = quote! {
        impl #impl_generics ::amenable_core::StateMachine<#verifier> for #self_ty {
            fn states() -> &'static [&'static str] {
                &[#(#state_names),*]
            }

            fn transitions() -> &'static [::amenable_core::Transition] {
                const TRANSITIONS: &[::amenable_core::Transition] = &[#(#transitions),*];
                TRANSITIONS
            }

            #audit_surface
            #root_entries
        }
    };

    // Only `translator_cfg = Some(..)` ever splices a real `#[cfg(..)]`
    // token into `audit_surface`'s two-branch split above -- no
    // wrapper needed (and none added) for the common case with no cfg
    // tokens at all, matching elicitation's own `needs_compat_mod`
    // discipline: wrap only when there's something to suppress. When it
    // is set, `#[cfg(..)]` here is meaningless to a downstream consumer
    // that never declares that cfg name -- the `allow`+`const` wrapper
    // is the only placement that suppresses `unexpected_cfgs` there
    // without affecting what actually compiles under the real cfg
    // (confirmed against this exact "cfg-gated associated fns alongside
    // ungated ones, all inside one impl" shape in an isolated scratch
    // crate before landing here; see `docs/CFG_HYGIENE_PLAN.md`'s
    // Step 1).
    if block.translator_cfg.is_some() {
        quote! {
            #[allow(unexpected_cfgs)]
            const _: () = {
                #state_machine_impl
            };
        }
    } else {
        state_machine_impl
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(states, name)))]
fn find_state_carrier<'a>(states: &'a [StateDecl], name: &LitStr) -> syn::Result<&'a Type> {
    states
        .iter()
        .find(|state| state.name.value() == name.value())
        .map(|state| &state.carrier)
        .ok_or_else(|| {
            Error::new(
                name.span(),
                format!(
                    "state_machine edge references undeclared state \"{}\"",
                    name.value()
                ),
            )
        })
}
