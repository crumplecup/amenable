//! Gallery case: the real `Stoplight` worked example
//! (`amenable_kani::stoplight`/`amenable_creusot::stoplight`), full
//! three-edge cycle, under Verus -- using `exchange_support`'s
//! `external_trait_specification`s (confirmed compatible with
//! `gallery::proof_token_external_trait_bound`'s own, separately-
//! declared copies existing in the same crate at the same time: Verus
//! does not treat two independently-named specification traits for the
//! same external trait as a "duplicate specification" the way its own
//! test suite's same-name/same-module cases are).
//!
//! **Disposition: best practice, confirmed.** **Expected/actual
//! outcome: passes, for real -- `418 verified, 0 errors`.**
//!
//! ## A real, new finding: no Kani-style workaround needed
//!
//! Kani 0.67.0 cannot place `#[kani::proof_for_contract]` on a trait
//! method when the trait itself is generic, which is exactly what
//! `Exchange<Input, Output, V>` is -- forcing the real logic and its
//! contract onto a plain inherent method in `amenable_kani::stoplight`,
//! with the trait impl reduced to delegation (see `EXCHANGE_PROOF_
//! DERIVATION_PLAN.md`'s Step 1). That limitation doesn't carry over to
//! Verus, confirmed empirically rather than assumed: a real `ensures`
//! clause sits directly on `Exchange<.., GalleryVerifier>`'s own generic
//! `exchange` method for all three edges (via `verus_exchange!`, see
//! below), and verifies clean. Kani's contracts are a separate,
//! DFCC-checked attribute mechanism; Verus's `ensures` is ordinary
//! function syntax, so this isn't surprising in hindsight, but it was
//! checked rather than presumed.
//!
//! ## Every mechanical piece of each edge is macro-generated
//!
//! Each edge needs, beyond its own real transition body: an `Ensures<
//! GalleryVerifier>` impl carrying the postcondition (spec companion +
//! exec body + `#[verifier::when_used_as_spec]` bridge), a `Witness<
//! GalleryVerifier>` impl for the target evidence (without it, the
//! preceding `Establish` impl above does not compile, so the exchange
//! cannot exist until the transition it claims is proven), and the
//! `Exchange<Input, Output, GalleryVerifier>` impl itself, its `ensures`
//! clause calling through the registered `Ensures` impl rather than
//! restating the bound (the same single-source-of-truth pattern
//! `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6 wires up for Kani).
//! None of that is hand-written here: `exchange_support::verus_ensures!`
//! generates the first, `exchange_support::verus_exchange!` the other
//! two -- the real Verus-side counterparts to Kani's `kani_ensures!` and
//! `#[amenable_derive::exchange(..)]` respectively (`EXCHANGE_PROOF_
//! DERIVATION_PLAN.md`'s Step 6/7). A *proc*-macro from a separate crate
//! like `amenable_derive` is not available here at all -- `verus
//! --crate-type=lib` never resolves any extern crate beyond what the
//! real `verus` binary itself bakes in, regardless of what `Cargo.toml`
//! declares -- so both are `macro_rules!` macros instead, each invoked
//! *outside* the main `verus! {}` block below, not inside it: a `macro_
//! rules!` macro's plain output can't itself contain `spec`/`open`/
//! `ensures` syntax the way a directly-authored `verus! {}` body can, so
//! each wraps its own content in a fresh, nested `verus! {}` invocation
//! instead (confirmed the hard way, including a real macro-hygiene fix
//! for `verus_exchange!`'s generated parameter name, in `gallery::
//! ensures_macro_generated`/`gallery::exchange_macro_generated`).
//!
//! `Bound = bool` (not the weaker `Bound = &'static str` `amenable_
//! core::contract`'s own doc comment originally anticipated for any
//! non-Rust-DSL backend) works for Verus via `#[verifier::
//! when_used_as_spec]`: each `Ensures` impl pairs its real exec
//! `ensures()` body with a private `spec fn` companion of identical
//! logic, and Verus transparently substitutes the spec version wherever
//! the exec version is referenced from spec position -- the exact
//! mechanism `vstd::std_specs::result::is_ok` uses for `Result::is_ok()`
//! itself (see `gallery::ensures_contract_bound` for the isolated case
//! that found and confirmed this).
//!
//! ## Verified non-vacuous
//!
//! A real bug (`Err(())` swapped in for the `Green -> Yellow` edge's
//! real `Ok(..)` body, inside its `verus_exchange!` invocation) produced
//! a real, precise failure -- `error: postcondition not satisfied`,
//! pointing at the exact macro-generated `ensures` clause (inside
//! `exchange_support.rs`'s own `verus_exchange!` definition, correctly
//! attributed back to this file's call site) and the exact `Err(())`
//! return that violates it -- confirming the `ensures` clause is a real,
//! checked claim on this exact body, not a vacuous pass. Reverted and
//! re-verified clean afterward.
//!
//! `full_cycle` chains all three real `Exchange::exchange` calls
//! together (not a hand-rolled shortcut using the underlying `establish`
//! calls directly), matching the real `Stoplight`'s own full
//! `Green -> Yellow -> Red -> Green` cycle -- also verifies clean.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

// `Ensures`/`Witness`/`Exchange` impls for each edge are macro-generated
// (`verus_exchange!`, below) using fully-qualified `<Evidence as
// crate::Ensures<V>>::ensures(..)` syntax internally, so this file itself
// no longer references those trait names directly -- no import needed
// for them here at all (unlike the earlier hand-written version, which
// needed a `#[cfg(verus_keep_ghost)]`-gated `use crate::Ensures;` since
// plain `cargo clippy` erases `ensures(...)` clause content entirely).
use crate::exchange_support::{verus_ensures, verus_sidecar, verus_state_machine};
// `exchange_support`'s `external_trait_specification`s apply crate-wide
// once compiled in, via `lib.rs`'s own `pub mod exchange_support;` --
// no explicit import needed here for Verus to pick them up.
use crate::{Establish, Evidence, Exchange, ProofToken, Sidecar, Verifier};

verus! {

pub struct GalleryVerifier;

impl Verifier for GalleryVerifier {
    type Metadata = GalleryVerifierMetadata;

    fn name() -> &'static str {
        "gallery"
    }
}

#[derive(Default)]
pub struct GalleryVerifierMetadata;

#[verifier::external]
impl crate::Provenance for GalleryVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<crate::MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

pub struct Green;

impl Evidence for Green {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

pub struct Yellow;

impl Evidence for Yellow {
    type Basis = Green;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}
}

pub struct Red;

impl Evidence for Red {
    type Basis = Yellow;
    type Audit = ();

    fn basis() -> Self::Basis {
        Yellow
    }

    fn audit(&self) -> Self::Audit {}
}

#[derive(Clone, Copy)]
pub struct GreenToken;

impl ProofToken for GreenToken {
    type Proposition = Green;
}

#[derive(Clone, Copy)]
pub struct YellowToken;

impl ProofToken for YellowToken {
    type Proposition = Yellow;
}

#[derive(Clone, Copy)]
pub struct RedToken;

impl ProofToken for RedToken {
    type Proposition = Red;
}

impl Establish<GreenToken, GalleryVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken
    }
}

impl Establish<YellowToken, GalleryVerifier> for Red {
    type Token = RedToken;

    fn establish(_credential: YellowToken) -> Self::Token {
        RedToken
    }
}

impl Establish<RedToken, GalleryVerifier> for Green {
    type Token = GreenToken;

    fn establish(_credential: RedToken) -> Self::Token {
        GreenToken
    }
}

pub struct Stoplight;

/// Sanitized mirror of `amenable_kani::stoplight::StoplightError` --
/// needed for the same real reason `amenable_creusot::stoplight`'s own
/// copy is: `verus_exchange!`'s generated bodies below are `#[amenable_
/// derive::exchange(..)]`'s real, verbatim-captured Kani bodies (`Ok(..)`
/// wrapper included), not a hand-simplified stand-in, so the surrounding
/// `Exchange::Error` type has to be the real one, not `()`.
#[derive(Debug, Clone, Copy)]
pub enum StoplightError {
    /// The one variant. Exists so `StoplightError` is an ordinary
    /// constructible type, not so any edge below constructs it.
    NotUsed,
}

/// Chains all three real `Exchange` impls together through the actual
/// trait methods (not a hand-rolled shortcut) -- the full cycle a real
/// `Stoplight` runs, proven to round-trip back to a well-formed `Green`.
pub fn full_cycle(stoplight: &Stoplight, start: Established<Green, GreenToken>) -> (result: Established<Green, GreenToken>)
{
    let yellow = stoplight
        .exchange(start)
        .expect("exchange's own ensures clause guarantees Ok");
    let red = stoplight
        .exchange(yellow)
        .expect("exchange's own ensures clause guarantees Ok");
    stoplight
        .exchange(red)
        .expect("exchange's own ensures clause guarantees Ok")
}

} // verus!

// `Established<T, Token>` -- generated by `verus_sidecar!`, the
// `macro_rules!` counterpart to `amenable_derive::Sidecar`
// (`amenable_kani::stoplight`'s and `amenable_creusot::stoplight`'s own
// real/mirror `Established<T, Token>` now derive the identical shape),
// not hand-written here anymore. Invoked outside the `verus! {}` block
// above for the same structural reason `verus_ensures!`/`verus_exchange!`
// both are -- see their own doc comments.
verus_sidecar!(Established<T, Token>, GalleryVerifier);

// The contract type carries the bound; the generated `ensures(...)`
// clause (via `verus_exchange!`, included below) calls through it rather
// than restating it -- the same single-source-of-truth pattern
// `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6 wires up for Kani (`kani_
// ensures!`), confirmed to also work for Verus (not merely `Bound =
// &'static str` description text, contrary to `amenable_core::contract`'s
// own doc comment at the time it was written -- see `gallery::ensures_
// contract_bound` for where that was checked, not assumed). Still hand-
// written: `verus_ensures!` carries the real predicate itself, the same
// discipline Kani's `kani_ensures!`/Creusot's own generator keep --
// generation only ever covers the mechanical wiring around a claim, never
// the claim's own content.
verus_ensures!(
    Yellow,
    GalleryVerifier,
    yellow_ensures_spec,
    Result<Established<Yellow, YellowToken>, StoplightError>,
    |result| result.is_ok()
);

verus_ensures!(
    Red,
    GalleryVerifier,
    red_ensures_spec,
    Result<Established<Red, RedToken>, StoplightError>,
    |result| result.is_ok()
);

// Backs the cycle-back edge, same as the real `Stoplight`'s own
// `evidence_id = "cycle_back"` on this edge's `#[amenable_derive::
// exchange]` invocation: `Green` (a root) still needs a `Witness` impl
// here, `Sidecar<V>`'s own bound applies to every proposition, root or
// not, and this is the only `Exchange` edge that can generate it.
verus_ensures!(
    Green,
    GalleryVerifier,
    green_ensures_spec,
    Result<Established<Green, GreenToken>, StoplightError>,
    |result| result.is_ok()
);

// The three `Witness<GalleryVerifier>`/`Exchange<..>` impls -- generated
// by `amenable::emit-verus-exchange-companions` from `amenable_core::
// ExchangeEdgeRecord`, the same registry `emit-creusot-companions`
// already reads (`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 9), not
// hand-written or hand-copied. `include!`, not `mod`: shares this file's
// own scope directly (`Green`/`Yellow`/`Established`/`Stoplight`/
// `StoplightError`/`GalleryVerifier` above, already in scope), matching
// `amenable_creusot::stoplight`'s identical reason. Regenerate with
// `just generate-verus-exchange` after changing a real Kani-side
// transition; do not hand-edit the included files.
include!("generated/stoplight_exchange/green_to_yellow.rs");
include!("generated/stoplight_exchange/yellow_to_red.rs");
include!("generated/stoplight_exchange/red_to_green.rs");

// The Verus half of `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4 --
// works unmodified against this gallery's own `Stoplight` because the
// three `Exchange` impls above already exist for real (`verus_exchange!`
// generates them, not a same-named stand-in). See `verus_state_machine!`'s
// own doc comment for why this is a hand-built `macro_rules!` macro
// rather than a reuse of `#[derive(amenable_derive::StateMachine)]`.
verus_state_machine!(
    Stoplight,
    GalleryVerifier,
    states: ["Green", "Yellow", "Red"],
    edges: [
        ("Green", Established<Green, GreenToken>, "Yellow", Established<Yellow, YellowToken>),
        ("Yellow", Established<Yellow, YellowToken>, "Red", Established<Red, RedToken>),
        ("Red", Established<Red, RedToken>, "Green", Established<Green, GreenToken>),
    ],
);
