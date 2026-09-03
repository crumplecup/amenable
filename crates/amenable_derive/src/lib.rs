//! Proc macros for the `amenable` constitutional trait family.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod attr_options;
mod calculation;
mod capture_exchange_body;
mod establish;
mod evidence;
mod exchange;
mod harness;
mod kani_compose;
mod proof_token;
mod provenance;
mod sidecar;
mod standard;
mod state_machine;
#[cfg(feature = "verus")]
mod verus_contract;
#[cfg(feature = "verus")]
mod verus_fragment;
mod witness;

use proc_macro::TokenStream;

use syn::{DeriveInput, ItemFn, ItemImpl, ItemStruct, parse_macro_input};

use calculation::{CalculationArgs, expand_calculation};
use capture_exchange_body::{CaptureExchangeBodyArgs, expand_capture_exchange_body};
use establish::{EstablishArgs, expand_establish};
use evidence::{expand_evidence, expand_evidence_derive};
use exchange::{ExchangeArgs, expand_exchange};
use harness::{HarnessRegistration, expand_harness};
use kani_compose::expand_kani_compose;
use proof_token::expand_proof_token;
use provenance::expand_provenance;
use sidecar::expand_sidecar;
use standard::expand_standard;
use state_machine::expand_state_machine;
use witness::expand_witness;

/// Define a `#[cfg(...)]`-gated proof harness item and, alongside it, an
/// always-available `&'static str` constant holding the harness's verbatim
/// source — whitespace and all.
///
/// `harness!(cfg_name, CONST_NAME, { item })` expands to `#[cfg(cfg_name)]
/// item` plus `const CONST_NAME: &str = "...";`. Both come from the same
/// braced group of tokens (captured via `Span::source_text`, which needs a
/// contiguous, human-authored span — this is why the item must be written
/// directly at the call site, not threaded through an intermediate
/// `macro_rules!` layer), so an audit report can show a proof exactly as
/// its author wrote it, and the two can never drift apart the way a
/// hand-maintained description could. Falls back to reconstructing the
/// source from tokens (losing original formatting) if `source_text` is
/// unavailable for the span.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro]
pub fn harness(input: TokenStream) -> TokenStream {
    match expand_harness(input.into(), HarnessRegistration::Tracked) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Identical grammar and `#[cfg(...)]`-gating to [`harness!`], for
/// proof-gallery cases specifically: never registers the contained
/// function as a tracked `amenable_kani::KaniProof`, since gallery
/// cases are explicitly *not* part of the tracked "does the suite still
/// pass" sweep (`amenable verify kani`) -- they get their own,
/// separately-registered `KaniGalleryRegistration` and run only via the
/// dedicated `amenable gallery` subcommand. See `harness.rs`'s own doc
/// comment for the real incident this split exists to prevent.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro]
pub fn gallery_harness(input: TokenStream) -> TokenStream {
    match expand_harness(input.into(), HarnessRegistration::GalleryOnly) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate a `Provenance` impl (`metadata()` walking every non-`#[provenance(skip)]`
/// field's own `Provenance::metadata()`), from a `#[derive(Provenance)]` on
/// a struct or enum.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(Provenance, attributes(provenance))]
pub fn derive_provenance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_provenance(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Turn a method into a chain link in the evidence graph: it knows it has a
/// method, knows it yields a token (named here), and registers itself.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr, item)))]
#[proc_macro_attribute]
pub fn calculation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CalculationArgs);
    let input = parse_macro_input!(item as ItemFn);

    match expand_calculation(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generalizes the by-hand `Exchange` transition pattern proven in
/// `amenable_kani::stoplight` (see `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
/// Step 3 and Step 6): given `impl SelfType { fn method(&self, input:
/// Input) -> Result<Output, Error> { .. } }` — the real inherent method,
/// its body left exactly as authored — generates the `Witness<V>` impl for
/// `evidence` naming the given harness, the `ProofRecord` registration
/// backing it, the `Exchange<Input, Output, V>` impl that delegates to
/// `method`, and (Step 6) `method`'s own `#[cfg_attr(.., kani::ensures(..))]`
/// contract, calling through `evidence`'s own, separately-registered
/// `Ensures<V>` impl (`kani_ensures!`, still hand-written — the actual
/// predicate lives there, not here) rather than requiring it re-typed at
/// every call site. Deliberately does not touch or generate the
/// `harness! { .. }` invocation itself, since that macro's verbatim-source
/// capture only works when written directly at its own call site.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr, item)))]
#[proc_macro_attribute]
pub fn exchange(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as ExchangeArgs);
    let input = parse_macro_input!(item as ItemImpl);

    match expand_exchange(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `#[capture_exchange_body(evidence = "..", creusot_ensures = "..")]` on
/// `impl SelfType { fn method(&self, input: Input) -> Result<Output,
/// Error> { .. } }` — registers a real `ExchangeEdgeRecord` from the
/// method's own real body, verbatim, leaving the method itself
/// completely untouched. See `capture_exchange_body.rs`'s own doc
/// comment for why this is a separate, narrower macro from `#[exchange(
/// ..)]` rather than a mode of it: `GAAP_LEDGER_PLAN.md`'s Step 7 moved
/// `Ledger`'s own methods to a neutral crate with a fully generic
/// `Ensures<V>` bound, so there is no concrete verifier left for
/// `#[exchange(..)]`'s own contract/`Witness<V>`/`Exchange<..>` bundle to
/// name.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr, item)))]
#[proc_macro_attribute]
pub fn capture_exchange_body(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as CaptureExchangeBodyArgs);
    let input = parse_macro_input!(item as ItemImpl);

    match expand_capture_exchange_body(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Compute `is_root` for a hand-written `impl Evidence` block from its own
/// `Basis` declaration, at compile time — no `TypeId`, no `'static`. For a
/// hand-written impl with real, non-trivial `basis()`/`audit()` bodies; see
/// [`derive_evidence`] for the common trivial-root case, which needs no
/// hand-written impl at all.
///
/// Takes no arguments -- rejected with a real compile error rather than
/// silently ignored, since `#[derive(Evidence)]`'s own, separate helper
/// attribute shares this exact name with real arguments (`#[evidence(
/// basis = "..", ..)]`), a realistic mistake to make on this macro's own
/// bare `#[evidence]` form.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr, item)))]
#[proc_macro_attribute]
pub fn evidence(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as syn::parse::Nothing);
    let input = parse_macro_input!(item as ItemImpl);

    match expand_evidence(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate a real, provable-not-asserted root `Evidence` impl (`Basis` as
/// named, `Audit = ()`, `is_root()` computed the same way [`evidence`]
/// does) from a `#[evidence(basis = "..", basis_ctor = "..", bound =
/// "..")]` attribute, plus the same `EvidenceLink` auto-registration
/// [`derive_standard`] already does for its own root, non-generic case —
/// see `evidence.rs`'s own doc comment for the full rationale and the real
/// duplication (and missing registry entries) this closes.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(Evidence, attributes(evidence))]
pub fn derive_evidence(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_evidence_derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate both `Standard` and `Evidence` impls from a `#[standard(...)]`
/// attribute, since they always share the same provenance value.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(Standard, attributes(standard))]
pub fn derive_standard(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_standard(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate `impl ProofToken for X { type Proposition = Y; }` from a
/// `#[proof_token(proposition = "Y")]` attribute -- see `proof_token`'s
/// own doc comment.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(ProofToken, attributes(proof_token))]
pub fn derive_proof_token(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_proof_token(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate the trivial-token-minting half of `impl Establish<C, V> for Y`
/// from a `#[establish(credential = .., verifier = .., proposition = ..)]`
/// attribute on the token struct -- see [`establish`]'s own doc comment
/// for why this is an attribute macro, not a derive.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attr, item)))]
#[proc_macro_attribute]
pub fn establish(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EstablishArgs);
    let input = parse_macro_input!(item as ItemStruct);

    match expand_establish(&args, &input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate `impl Sidecar<V> for X { .. }` plus a `new(..)` constructor
/// from `#[sidecar(verifier = .., proposition = .., constructor = ..)]`
/// and `#[sidecar(primary)]`/`#[sidecar(token)]` field markers -- see
/// `sidecar`'s own doc comment.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(Sidecar, attributes(sidecar))]
pub fn derive_sidecar(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_sidecar(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `#[derive(StateMachine)]` -- see `state_machine`'s own doc comment
/// for the full `#[state_machine(verifier = .., state(..), edge(..))]`
/// syntax. Step 1 of `docs/STATE_MACHINE_DERIVATION_PLAN.md`: emits only
/// the compiler-enforced static assertions, one per declared edge.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(StateMachine, attributes(state_machine))]
pub fn derive_state_machine(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_state_machine(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate bounded Kani-facing constructors by delegating every field to
/// its own `KaniCompose` implementation -- see `kani_compose`'s own doc
/// comment for the full rationale.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(KaniCompose)]
pub fn derive_kani_compose(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_kani_compose(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Generate a structural closure over already-witnessed members -- see
/// `witness`'s own doc comment for the full rationale.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[proc_macro_derive(Witness, attributes(provenance, witness))]
pub fn derive_witness(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_witness(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Expand a real Verus harness name to a `&'static [&'static str]` array
/// literal of its real `ensures` clauses, extracted from the real
/// carrier source at compile time -- a missing harness or clause is a
/// real compile error here, not a runtime failure discovered later.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_fragments(input: TokenStream) -> TokenStream {
    match verus_fragment::expand_verus_fragments(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_fragments!`], for a harness's real `requires`
/// clauses instead.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_fragments(input: TokenStream) -> TokenStream {
    match verus_fragment::expand_verus_fragments(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `verus_ensures_witness!(Type, evidence_expr, "harness")` generates a
/// real `impl Ensures<crate::VerusVerifier> for Type` (`Bound =
/// &'static [&'static str]`, one real clause per element) plus one
/// `ContractRecord` registration per clause -- see
/// `verus_contract`'s own doc comment for why `Bound` is a slice
/// and why the registration is generated rather than hand-written.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_witness(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_witness(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_witness!`], for `Requires` instead.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_witness(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_witness(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// `verus_ensures_predicate!(Type, evidence_expr, "predicate_name")` --
/// like [`verus_ensures_witness!`], but for a claim that's a real,
/// named `pub open spec fn`'s own declaration (shared across several
/// different harnesses/carrier files) rather than any one harness's own
/// clause list. See `verus_contract`'s own doc comment.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_ensures_predicate(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_predicate_witness(input.into(), true) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Like [`verus_ensures_predicate!`], for `Requires` instead.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
#[cfg(feature = "verus")]
#[proc_macro]
pub fn verus_requires_predicate(input: TokenStream) -> TokenStream {
    match verus_contract::expand_verus_predicate_witness(input.into(), false) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
