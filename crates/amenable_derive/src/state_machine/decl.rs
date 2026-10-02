use syn::{LitStr, Type};

#[derive(derive_getters::Getters, derive_new::new)]
pub(super) struct StateDecl {
    name: LitStr,
    carrier: Type,
    root: Option<RootDecl>,
}

/// A declared root constructor: the real, compile-time-checked path
/// (for the `const _: fn(..) -> Carrier = #path;` assertion), its
/// original literal text (for `root_entries()`'s `constructor` string
/// -- kept alongside the parsed `syn::Path` rather than re-stringifying
/// it via `quote!`, which normalizes token spacing, e.g. `Established::
/// <Green, GreenToken>::root` becomes `Established :: < Green,
/// GreenToken > :: root`, a technically-equivalent but uglier string
/// than the one actually written in the declaration), and an optional
/// seed: the real argument type a data-needing root's constructor
/// requires, parsed and stringified the same paired way.
#[derive(derive_getters::Getters, derive_new::new)]
pub(super) struct RootDecl {
    path: syn::Path,
    path_lit: LitStr,
    seed: Option<(Type, LitStr)>,
}

#[derive(derive_getters::Getters, derive_new::new)]
pub(super) struct EdgeDecl {
    from: LitStr,
    to: LitStr,
}

pub(super) enum VerifierMode {
    Concrete(Box<Type>),
    Generic,
}

#[derive(derive_getters::Getters, derive_new::new)]
pub(super) struct StateMachineBlock {
    verifier: VerifierMode,
    states: Vec<StateDecl>,
    edges: Vec<EdgeDecl>,
    translator_cfg: Option<LitStr>,
}
