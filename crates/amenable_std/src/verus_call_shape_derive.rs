//! Derive a harness's real [`VerusCallShape`] by parsing its defining
//! carrier source, instead of a human hand-typing a second copy of its
//! real `requires`/`ensures` clauses.
//!
//! The actual parsing (locating the carrier file, extracting its real
//! signature/clauses) lives in `amenable_core::verus_carrier` -- shared
//! with `amenable_derive`'s compile-time `verus_ensures_fragments!`/
//! `verus_requires_fragments!` macros, so there's exactly one
//! implementation of "how do you read a real harness's real signature,"
//! not two. This module is just the `VerusCallShape`-shaped view over
//! it: building the `$placeholder` templates a compositional renderer
//! needs (as opposed to the literal text those macros want), and
//! discovering which real module each cited predicate/spec-fn needs an
//! import from.

use std::collections::HashSet;

use amenable_core::{verus_find_fn, verus_param_name, verus_walk_tokens};

use crate::{VerusCallKind, VerusCallShape, VerusImport, VerusParam};

/// Derive one harness's real [`VerusCallShape`] by locating and parsing
/// its defining carrier file -- `None` if no real carrier defines a
/// public function with this exact name.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(crate) fn derive_call_shape(harness: &str) -> Option<VerusCallShape> {
    let (_path, module_path, item_fn) = verus_find_fn(harness)?;

    let params = item_fn
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match &arg.kind {
            verus_syn::FnArgKind::Typed(pat_type) => {
                let name = verus_param_name(&pat_type.pat)?;
                let ty = &pat_type.ty;
                let ty = verus_walk_tokens(quote::quote!(#ty), &HashSet::new(), &mut Vec::new());
                Some(VerusParam::new(name, ty))
            }
            verus_syn::FnArgKind::Receiver(_) => None,
        })
        .collect::<Vec<_>>();

    let returns = match &item_fn.sig.output {
        verus_syn::ReturnType::Type(_, _, pattern, ty) => {
            if let Some(pattern_box) = pattern {
                let (_, pat, _) = pattern_box.as_ref();
                assert_eq!(
                    verus_param_name(pat).as_deref(),
                    Some("result"),
                    "harness `{harness}`'s named return binding must be `result`, matching the \
                     renderer's own fixed `$result` placeholder convention"
                );
            }
            verus_walk_tokens(quote::quote!(#ty), &HashSet::new(), &mut Vec::new())
        }
        verus_syn::ReturnType::Default => "()".to_owned(),
    };

    let mut placeholders: HashSet<String> =
        params.iter().map(|param| param.name().clone()).collect();
    placeholders.insert("result".to_owned());

    // A `Vec`, not a `HashSet`: iteration order must be stable (the
    // discovery/appearance order in the clauses) so `imports` doesn't
    // vary between runs -- `HashSet`'s iteration order is randomized
    // per-process, confirmed the hard way by a real flaky test failure.
    let mut calls = Vec::new();

    let requires = item_fn
        .sig
        .spec
        .requires
        .as_ref()
        .map(|requires| {
            requires
                .exprs
                .exprs
                .iter()
                .map(|expr| verus_walk_tokens(quote::quote!(#expr), &placeholders, &mut calls))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let ensures = item_fn
        .sig
        .spec
        .ensures
        .as_ref()
        .map(|ensures| {
            ensures
                .exprs
                .exprs
                .iter()
                .map(|expr| verus_walk_tokens(quote::quote!(#expr), &placeholders, &mut calls))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let imports = calls
        .into_iter()
        .filter(|name| !placeholders.contains(name))
        .filter_map(|name| {
            let (_, import_module_path, _) = verus_find_fn(&name)?;
            Some(VerusImport::new(import_module_path, name))
        })
        .collect();

    Some(VerusCallShape::new(
        module_path,
        harness.to_owned(),
        params,
        requires,
        ensures,
        imports,
        VerusCallKind::Function { returns },
    ))
}
