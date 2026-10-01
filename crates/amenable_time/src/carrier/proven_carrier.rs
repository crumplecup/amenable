//! The generic `ProvenTemporalCarrier<T, STok>` sidecar wrapper.
//!
//! Phase 5 carrier surface. Each aggregate proof bundle
//! (`proof_composition::semantic_bundles`) gets one `<Bundle>Token`,
//! `#[establish]`-minted from the token that produced it -- the
//! bundle proposition is the *named aggregate*, the token *stands for
//! it*. [`ProvenTemporalCarrier`] pairs a backend-native carrier `T`
//! with one such token; keyed on the token so the token stays the
//! single source of truth (the proposition is
//! `<STok as ProofToken>::Proposition`).
//!
//! The `#[proof_token]` / `#[establish]` / `#[sidecar]` attributes
//! re-parse their `crate::...` string arguments as paths, so no `use`
//! is needed for the token structs themselves.

/// Generic user-facing wrapper: a backend-native carrier `T` paired
/// with one proof token `STok` standing for its whole semantic
/// bundle. A [`Sidecar`](amenable_core::Sidecar) — `carrier` is the
/// primary payload, `semantics` the sidecar token, and the
/// proposition is recovered from the token as
/// `<STok as ProofToken>::Proposition`. A `T` only ever exists
/// because a native factory produced it under proof, so this pairing
/// is lawful by construction.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition_from_token, constructor = "pub")]
pub struct ProvenTemporalCarrier<T, STok>
where
    T: ::amenable_core::Evidence,
    STok: ::amenable_core::ProofToken,
{
    #[sidecar(primary)]
    carrier: T,
    #[sidecar(token)]
    semantics: STok,
}
