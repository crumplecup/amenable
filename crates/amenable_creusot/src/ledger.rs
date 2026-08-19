//! Creusot proof-function content for `GAAP_LEDGER_PLAN.md`'s Step 3 —
//! the ledger worked example's own claims, the first genuinely
//! non-trivial Creusot predicates anywhere in this workspace (every
//! `Stoplight` claim proven so far is `result.is_ok()`; that never
//! exercised anything beyond translator plumbing).
//!
//! **A real structural difference from `stoplight.rs`.** `Stoplight`'s
//! evidence types (`Green`/`Yellow`/`Red`) live in `amenable_kani`, a
//! crate `amenable_creusot` can never depend on (real cycle risk: it
//! would optionally depend back), so `amenable_creusot::stoplight`
//! needs a full accommodation-model mirror. GAAP's own evidence types
//! (`Pending`/`Validated`/`Committed`/`Rejected<T>`) live in
//! `amenable_gaap` instead, which has no dependency back on
//! `amenable_creusot` at all — so `amenable_creusot` can take a real,
//! ordinary Cargo dependency on `amenable_gaap` and implement
//! `Witness<CreusotVerifier>` directly on the *real* types, no mirror
//! needed. Confirmed empirically, not assumed: `cargo creusot -- -p
//! amenable_creusot` and `cargo creusot prove -- -p amenable_creusot`
//! both succeed with `amenable_gaap` as a real dependency (`Proved (114
//! files) ✔`), and a real injected bug in `check_amount_positive`'s
//! body produced a precise, real failure (`Goal Coma.vc_check_amount_
//! positive: ✘`) before being reverted — the earlier assumption that
//! *any* dependency beyond `amenable_core` would risk an ICE the way a
//! *local* item does (per `amenable_std::creusot_witness`'s own doc
//! comment) turned out not to generalize to an ordinary dependency
//! crate's own items, only to items local to the crate `cargo creusot`
//! actually translates.
//!
//! The proof functions' own signatures are still a *sanitized mirror*
//! of the real Kani-side claims (`amenable_kani::ledger`), not a
//! byte-for-byte structural copy — matching `Stoplight`'s own precedent
//! of dropping the real body's `Result` wrapper: `amount`/`balance`
//! stay plain `i64`, and an account identity is mirrored as a plain
//! `u64` rather than `amenable_gaap::AccountId`'s real `Uuid`-backed
//! struct — the real claim (`from != to`) only needs *some* comparable
//! identity type, and Creusot's support for a hand-rolled `Uuid`-backed
//! equality type is untested territory not worth risking here. `Ledger`/
//! `Transfer<S, Token>` themselves still live only in `amenable_kani`
//! (a crate this one still can't depend on), so the checks in this file
//! stay isolated predicates over primitive arguments, the same way
//! `Ledger::check_amount_positive`/`check_sufficient_funds` are isolated
//! on the Kani side. The `Ledger`/`Transfer<S, Token>`/`TransferError`
//! mirror types further down (`GAAP_LEDGER_PLAN.md`'s Step 6) connect
//! `Ledger::validate`'s/`::commit`'s real captured bodies to these same
//! predicates — not by depending on `amenable_kani`, but via the
//! `generated/validate.rs`/`generated/commit.rs` companions this file
//! `include!`s at the bottom, produced by the same `ExchangeEdgeRecord`
//! codegen Step 4 already used for Verus.
//!
//! **`GAAP_LEDGER_PLAN.md`'s Step 5**: `amenable_gaap::contracts::
//! {AmountPositive, SufficientFunds, AccountsDistinct, BalancedEntries}`
//! — real `Evidence` types since Step 0, previously dead code here too
//! (this module's own `_holds` predicates matched their names by
//! convention only, never touched the real types) — now back real
//! `Witness<CreusotVerifier>`/`Ensures<CreusotVerifier>` impls, one per
//! `_holds` predicate, `#[cfg(not(creusot))]`-only (no `#[cfg(creusot)]`
//! counterpart needed: nothing establishes a token *from* any of these
//! four, so real translation never needs to see either impl — unlike
//! `Validated`/`Committed`'s own `Witness<CreusotVerifier>` impls,
//! further down, which do). `Bound = &'static str`, not `bool`: Pearlite
//! predicates have no exec representation at all, so `Ensures::ensures`
//! just exposes the real `_holds` source text for audit purposes — the
//! actual checking is `amount_positive_holds`/etc. themselves, called
//! directly from Pearlite composition (inside `check_amount_positive`'s
//! own `#[ensures(..)]` and `validated_holds`'s own `match`), never
//! through this trait.

#[cfg(creusot)]
use amenable_core::{Establish, Evidence, Sidecar, Witness};
#[cfg(creusot)]
use amenable_gaap::{
    Committed, CommittedToken, Pending, PendingToken, Rejected, RejectedFromPendingToken,
    RejectedFromValidatedToken, Validated, ValidatedToken,
};
#[cfg(creusot)]
use creusot_std::macros::{ensures, extern_spec, logic, requires};
#[cfg(creusot)]
use creusot_std::std::ops::FnOnceExt;

#[cfg(creusot)]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use amenable_gaap::{AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds};

// A real, confirmed gap in `creusot_std` itself, not this crate's own:
// `Result::map_err` carries no contract anywhere in `creusot-std`'s own
// source (confirmed by reading `creusot-std/src/std/result.rs` directly
// -- `map`/`and`/`or`/`unwrap_or`/etc. are all specified there, `map_err`
// is not), so calling it with no `extern_spec!` of our own produces a
// real "calling external function `map_err` with no contract will yield
// an impossible precondition" warning and a genuine unprovable goal --
// confirmed against the real toolchain, not assumed: `Ledger::validate`'s
// own real body captured below (`GAAP_LEDGER_PLAN.md`'s Step 6) is the
// first proof in this crate to call `.map_err(..)` at all. Modeled
// directly on `creusot-std`'s own `Option::map`'s real `extern_spec!`
// entry (`existential postcondition_once`, the same shape every other
// `FnOnce`-taking `Result`/`Option` method there uses).
//
// Plain `//` comments, not `///`: rustdoc doesn't generate documentation
// for items produced by a macro invocation, so a doc comment here is
// real, silently-dropped dead weight -- confirmed by `cargo creusot`'s
// own real "unused doc comment" warning, invisible to ordinary `cargo
// check`/clippy the same way every `#[cfg(creusot)]`-gated content is.
#[cfg(creusot)]
extern_spec! {
    impl<T, E> Result<T, E> {
        #[requires(match self {
            Ok(_) => true,
            Err(e) => op.precondition((e,)),
        })]
        #[ensures(match self {
            Ok(t) => result == Ok(t),
            Err(e) => exists<r> result == Err(r) && op.postcondition_once((e,), r),
        })]
        fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F>;
    }
}

// `GAAP_LEDGER_PLAN.md`'s Step 7: the same real, confirmed gap as
// `map_err`'s own extern_spec, above, one layer up. `Establish::
// establish` now lives in `amenable_gaap` (one real, backend-generic
// blanket impl, not a per-verifier one hand-written locally here) --
// `creusot-rustc`'s translator only fully analyzes items *local* to the
// crate it's translating, so a call through a dependency's impl is
// "external" to it the same way a std method is, and needs its own
// explicit contract. Confirmed against the real toolchain: without
// this, `cargo creusot prove` reports "calling external function
// `establish` with no contract will yield an impossible precondition"
// and a genuine unprovable `vc_validate`/`vc_commit` goal.
//
// `#[ensures(true)]`, not a richer claim: `establish`'s own real body
// (every hand-written impl in this workspace, and the blanket impl
// alike) ignores its credential and mints a bare unit token whose only
// field is private -- there is no observable postcondition to state
// about the *result* from Pearlite, matching this project's own
// "tautological model" precedent (a real, honest `true` for a
// genuinely content-free claim, not a stand-in for a stronger one this
// crate is avoiding).
//
// `-> ValidatedToken`/`-> CommittedToken`, not `-> Self::Token`:
// `extern_spec!` real-toolchain-confirmed rejects an associated-type
// return position ("Cannot use Self here") -- spelled out as the
// concrete type each blanket impl instantiation actually resolves to
// instead.
//
// `impl<V: Verifier> .. for Validated`, not `impl .. for Validated`
// fixed to `CreusotVerifier`: `extern_spec!` real-toolchain-confirmed
// ("extern spec generics don't match") requires matching the *real*
// item's own generic shape exactly -- the real impl is the one, generic
// blanket impl in `amenable_gaap` (`impl<V: Verifier> Establish<C, V>
// for Y where Y: Witness<V>`), not a per-verifier concrete one, so the
// extern_spec has to be written the same way, `V` and all.
#[cfg(creusot)]
extern_spec! {
    impl<V: amenable_core::Verifier> Establish<PendingToken, V> for Validated
    where
        Validated: Witness<V>,
    {
        #[ensures(true)]
        fn establish(credential: PendingToken) -> ValidatedToken;
    }

    impl<V: amenable_core::Verifier> Establish<ValidatedToken, V> for Committed
    where
        Committed: Witness<V>,
    {
        #[ensures(true)]
        fn establish(credential: ValidatedToken) -> CommittedToken;
    }

    // `reject`'s/`rollback`'s own real captured bodies (`generated/
    // reject.rs`/`generated/rollback.rs`, below) call the identical
    // external `establish` -- revisited scope, `GAAP_LEDGER_PLAN.md`'s
    // Step 7: same real gap, same fix, same reasoning as `Validated`'s/
    // `Committed`'s own entries above.
    impl<V: amenable_core::Verifier> Establish<PendingToken, V> for Rejected<Pending>
    where
        Rejected<Pending>: Witness<V>,
    {
        #[ensures(true)]
        fn establish(credential: PendingToken) -> RejectedFromPendingToken;
    }

    impl<V: amenable_core::Verifier> Establish<ValidatedToken, V> for Rejected<Validated>
    where
        Rejected<Validated>: Witness<V>,
    {
        #[ensures(true)]
        fn establish(credential: ValidatedToken) -> RejectedFromValidatedToken;
    }
}

/// `GAAP_LEDGER_PLAN.md`'s Step 6: an accommodation-model mirror for
/// `amenable_kani::ledger::{Ledger, Transfer<S, Token>}` and their tokens
/// -- needed for the same real reason `amenable_creusot::stoplight`'s own
/// mirror is, and only now, since `Ledger::validate`'s/`::commit`'s real
/// bodies are captured verbatim below for the first time (`validate.rs`/
/// `commit.rs`, generated by `amenable::creusot_export`): `Ledger`/
/// `Transfer<S, Token>` live only in `amenable_kani`, a crate this one
/// cannot depend on (verifier backends never depend on each other), and
/// their real constructors are deliberately private (`Transfer::new` is
/// `pub(crate)` there), so a real dependency wouldn't let the captured
/// bodies compile against the real types even if the dependency itself
/// were legal. Matches `amenable_gaap`'s own real evidence markers
/// (`Pending`/`Validated`/`Committed`) directly -- no mirror needed for
/// those, since they're not privacy-gated and this crate already depends
/// on `amenable_gaap` for real.
///
/// `Transfer<S, Token>`'s own `Sidecar<CreusotVerifier>` impl is derived,
/// not hand-written -- `#[derive(amenable_derive::Sidecar)]`, the same
/// derive `amenable_kani::ledger::Transfer`'s real definition and
/// `amenable_creusot::stoplight::Established`'s own mirror both use for
/// the identical shape (`GAAP_LEDGER_PLAN.md`'s Step 5).
#[cfg(creusot)]
#[derive(amenable_derive::Sidecar)]
#[sidecar(
    verifier = "CreusotVerifier",
    proposition = "S",
    constructor = "pub(crate)"
)]
pub struct Transfer<S, Token> {
    // Fully `pub`, not `pub(crate)`: Creusot's own proof-transparency
    // check requires an `#[ensures(..)]` clause to only ever reach items
    // *at least* as visible as the function stating it -- and `Sidecar`'s
    // own `primary()` method (`amenable_core::Sidecar` is a `pub` trait)
    // is as visible as the trait impl itself, not merely `pub(crate)`
    // like `new`'s own constructor. Confirmed the hard way in two steps:
    // a real "cannot make `.. payload` transparent in `.. new`" error
    // once `#[derive(amenable_derive::Sidecar)]`'s generated constructor
    // gained a real `#[ensures(result.payload == payload)]` clause
    // (`GAAP_LEDGER_PLAN.md`'s Step 6), fixed by widening to `pub(crate)`
    // -- then the *same* error again, this time naming `Sidecar::
    // primary()` instead of `new`, once that method *also* gained a real
    // `ensures` clause. No privacy invariant to protect here either way,
    // unlike the real `amenable_kani::ledger::Transfer`'s own private
    // fields.
    #[sidecar(primary)]
    pub payload: TransferPayload,
    #[sidecar(token)]
    pub token: Token,
    _state: std::marker::PhantomData<S>,
}

/// Sanitized mirror of `amenable_gaap::Amount` -- the real captured
/// `validate`/`commit` bodies call `.amount().value()`, so this needs
/// the same two-method chain, not just a bare `i64`. The field is `pub`,
/// not private: `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4 gave
/// `Ledger`'s edges a real `Exchange` trait impl, whose `exchange()`
/// method is necessarily as visible as the (public) `Exchange` trait
/// itself -- more visible than the private inherent method its
/// `#[ensures(..)]` clause used to sit on alone. Creusot's
/// proof-transparency check requires everything an `#[ensures(..)]`
/// clause touches to be at least as visible as the function carrying it
/// (the same real constraint `GAAP_LEDGER_PLAN.md`'s Step 6 already hit
/// and fixed the same way, confirmed again here rather than assumed
/// still true).
#[cfg(creusot)]
#[derive(Clone, Copy)]
pub struct Amount(pub i64);

#[cfg(creusot)]
impl Amount {
    // `#[ensures(..)]`: ordinary modular verification only exposes what
    // a function's own `ensures` promises -- without this, nothing
    // downstream could learn `.value()`'s actual result from `self`'s
    // own field, the identical real reason every accessor below needs
    // one too (`GAAP_LEDGER_PLAN.md`'s Step 6, confirmed the hard way:
    // `validate`'s own success-path postcondition could not be proven
    // without these, even after `Transfer::new`'s own `#[derive(Sidecar)]`
    // -generated `ensures` was fixed).
    #[requires(true)]
    #[ensures(result == self.0)]
    fn value(&self) -> i64 {
        self.0
    }
}

/// Sanitized mirror of `amenable_gaap::TransferPayload` -- `from`/`to`
/// are bare `u64`, matching this file's own existing sanitized-mirror
/// choice for `check_accounts_distinct`'s own isolated proof, above:
/// the real claim only needs *some* comparable identity type, and the
/// real captured body never names `AccountId` as a type, only ever
/// compares what `.from()`/`.to()` return. `from`/`to`/`amount` are all
/// `pub`, not private -- see [`Amount`]'s own doc comment for why: the real
/// `Exchange::exchange` impl's `#[ensures(..)]` clause (`docs/
/// STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4) accesses them directly,
/// and needs to be at least as visible as that (public trait) method.
#[cfg(creusot)]
#[derive(Clone, Copy)]
pub struct TransferPayload {
    pub from: u64,
    pub to: u64,
    pub amount: Amount,
}

#[cfg(creusot)]
impl TransferPayload {
    // `#[ensures(..)]` throughout this impl -- same real reason as
    // [`Amount::value`], above.
    #[requires(true)]
    #[ensures(result == self.from)]
    fn from(&self) -> u64 {
        self.from
    }

    #[requires(true)]
    #[ensures(result == self.to)]
    fn to(&self) -> u64 {
        self.to
    }

    #[requires(true)]
    #[ensures(result.0 == self.amount.0)]
    fn amount(&self) -> Amount {
        self.amount
    }
}

#[cfg(creusot)]
impl Evidence for TransferPayload {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        TransferPayload {
            from: 0,
            to: 0,
            amount: Amount(0),
        }
    }

    fn audit(&self) {}
}

/// `GAAP_LEDGER_PLAN.md`'s Step 7: `PendingToken`/`ValidatedToken`/
/// `CommittedToken` are no longer mirrored here -- they're real,
/// backend-neutral types imported directly from `amenable_gaap`
/// (`amenable_creusot` already depends on it for real), each minted via
/// one backend-generic `impl<V: Verifier> Establish<C, V> for Y where Y:
/// Witness<V>` living in `amenable_gaap` itself. That blanket impl
/// becomes a real, usable `Establish<PendingToken, CreusotVerifier> for
/// Validated` for free, the moment `Validated: Witness<CreusotVerifier>`
/// holds -- which it already does, further down this file. No token
/// mirror, no hand-written `Establish` impl, needed here at all anymore.
///
/// Trivial, matching `amenable_kani::ledger::Pending`'s own `Witness<
/// KaniVerifier>` impl exactly (see its own doc comment): nothing in
/// this worked example's scope ever targets `Pending`, so nothing
/// generates one -- needed here only because `Transfer<Pending,
/// PendingToken>: Sidecar<CreusotVerifier>` (`validate`'s own input
/// type) requires `Pending: Evidence + Witness<CreusotVerifier>`.
#[cfg(creusot)]
impl Witness<CreusotVerifier> for Pending {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Sanitized mirror of `amenable_kani::ledger::TransferError` -- a
/// *different*, concrete type from this file's own `TransferOutcome`
/// (above), which backs the pre-existing isolated `validate` proof
/// function's own Pearlite claim, unrelated to and unaffected by this
/// mirror. Real captured bodies below construct `TransferError::
/// NegativeAmount(..)`/`::InsufficientFunds { .. }`/`::SameAccount`
/// directly, by name, so the type has to be named exactly this, with
/// exactly these variants -- no `PartialEq`/`Eq` derive needed, matching
/// `TransferOutcome`'s own precedent (pattern matching, not `==`).
#[cfg(creusot)]
#[derive(Clone, Copy)]
pub enum TransferError {
    NegativeAmount(i64),
    InsufficientFunds { balance: i64, required: i64 },
    SameAccount,
}

/// Sanitized mirror of `amenable_kani::ledger::Ledger` -- the real
/// captured `validate`/`commit` bodies below call `Self::check_amount_
/// positive`/`self.check_sufficient_funds`/`Self::negative_amount`/
/// `Self::insufficient_funds`, all as real methods on a real receiver
/// (the first captured Creusot body to reference `self`/`Self` at all --
/// `amenable::creusot_export`'s own generator needed a real `&self`
/// wrapper added for exactly this, `GAAP_LEDGER_PLAN.md`'s Step 6). A
/// *different*, concrete type from this file's own bare `check_amount_
/// positive`/`check_sufficient_funds` free functions (above), which back
/// the pre-existing isolated `validate` proof, unrelated to and
/// unaffected by this mirror -- both exist in the same module without
/// conflict, since `Ledger::check_amount_positive`/`self.check_
/// sufficient_funds` and the bare free-function calls resolve through
/// entirely different paths.
#[cfg(creusot)]
pub struct Ledger {
    balance: i64,
}

#[cfg(creusot)]
impl Ledger {
    /// `<V>`: unconstrained, no bound, never referenced in this body --
    /// exists purely so `validate`'s own real captured body (`generated/
    /// validate.rs`, `Self::check_amount_positive::<V>(amount)`) has a
    /// generic parameter to name. `Ledger::check_amount_positive`'s real
    /// counterpart in `amenable_gaap` is generic over `V: amenable_core::
    /// Verifier` with a real `Ensures<V>` bound (`GAAP_LEDGER_PLAN.md`'s
    /// Step 7) -- Creusot's own `Ensures<CreusotVerifier>` impl has a
    /// structurally different shape (`Input = ()`, `Bound = &'static
    /// str`, purely descriptive text; the real checking happens through
    /// `amount_positive_holds` directly, never through this trait, see
    /// this module's own doc comment), so mirroring the real bound here
    /// would be a type mismatch, not a stronger proof -- an unconstrained
    /// `V` is the honest mirror of "the real call needs some type in this
    /// position," nothing more.
    #[requires(true)]
    #[ensures(match result {
        Ok(()) => amount_positive_holds(amount, true),
        Err(bad) => bad == amount && amount_positive_holds(amount, false),
    })]
    fn check_amount_positive<V>(amount: i64) -> Result<(), i64> {
        if amount > 0 { Ok(()) } else { Err(amount) }
    }

    /// `<V>`: see [`Ledger::check_amount_positive`]'s own doc comment.
    #[requires(true)]
    #[ensures(match result {
        Ok(()) => sufficient_funds_holds(self.balance, amount, true),
        Err((balance, required)) => balance == self.balance && required == amount && sufficient_funds_holds(self.balance, amount, false),
    })]
    fn check_sufficient_funds<V>(&self, amount: i64) -> Result<(), (i64, i64)> {
        if self.balance < amount {
            Err((self.balance, amount))
        } else {
            Ok(())
        }
    }

    /// Matches `amenable_kani::ledger::Ledger::negative_amount`'s own
    /// doc comment: a real function, not the bare `TransferError::
    /// NegativeAmount` tuple-variant constructor or a destructuring
    /// closure -- unlike Verus, Creusot places no such restriction on
    /// either form, but the real captured body already calls through
    /// this real method (the Kani source both backends capture from is
    /// the same, single source of truth), so this mirrors it exactly
    /// rather than inlining an equivalent that would silently diverge
    /// from what's actually captured.
    #[requires(true)]
    #[ensures(match result {
        TransferError::NegativeAmount(actual) => actual == bad,
        _ => false,
    })]
    fn negative_amount(bad: i64) -> TransferError {
        TransferError::NegativeAmount(bad)
    }

    #[requires(true)]
    #[ensures(match result {
        TransferError::InsufficientFunds { balance, required } => balance == bad.0 && required == bad.1,
        _ => false,
    })]
    fn insufficient_funds(bad: (i64, i64)) -> TransferError {
        TransferError::InsufficientFunds {
            balance: bad.0,
            required: bad.1,
        }
    }
}

amenable_derive::harness! {
    creusot, AMOUNT_POSITIVE_HOLDS_SRC, {
        /// The `Validated` postcondition's `AmountPositive` conjunct —
        /// real, callable Pearlite content, named and reused rather
        /// than restated wherever it's checked.
        #[logic(open)]
        pub fn amount_positive_holds(amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (amount@ > 0) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_AMOUNT_POSITIVE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_amount_positive`'s
        /// own claim, isolated the same way that function is isolated on
        /// the Kani side.
        #[requires(true)]
        #[ensures(amount_positive_holds(amount, result))]
        fn check_amount_positive(amount: i64) -> bool {
            amount > 0
        }
    }
}

// Ties the real `amenable_gaap::AmountPositive` contract type to this
// module's own Pearlite content -- matching `amenable_kani::ledger`'s
// own `kani_ensures!` wiring for the identical type (`GAAP_LEDGER_
// PLAN.md`'s Step 5), previously done on Kani only. `Bound = &'static
// str`, not `bool`: unlike Kani, Pearlite predicates have no exec
// representation at all, so `amenable_core::contract`'s own doc comment
// already anticipates this fallback -- a description of the real bound
// for audit purposes, not a checked value (the actual checking is
// `amount_positive_holds` itself, called directly from Pearlite
// composition above and in `validated_holds` below; nothing in the
// translated proof ever calls through this trait). No `#[cfg(creusot)]`
// counterpart needed at all, unlike `Validated`/`Committed`'s own
// `Witness<CreusotVerifier>` impls: nothing establishes a token *from*
// `AmountPositive` (it's not part of an `Establish<C, V>` chain), so
// real Creusot translation never needs to see either impl.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AmountPositive {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_amount_positive".to_owned(),
                VERIFY_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AmountPositive {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        AMOUNT_POSITIVE_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, SUFFICIENT_FUNDS_HOLDS_SRC, {
        /// The `Validated` postcondition's `SufficientFunds` conjunct.
        #[logic(open)]
        pub fn sufficient_funds_holds(balance: i64, amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (balance@ >= amount@) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_SUFFICIENT_FUNDS_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_sufficient_funds`'s
        /// own claim.
        #[requires(true)]
        #[ensures(sufficient_funds_holds(balance, amount, result))]
        fn check_sufficient_funds(balance: i64, amount: i64) -> bool {
            balance >= amount
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for SufficientFunds {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_sufficient_funds".to_owned(),
                VERIFY_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for SufficientFunds {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        SUFFICIENT_FUNDS_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, ACCOUNTS_DISTINCT_HOLDS_SRC, {
        /// The `Validated` postcondition's `AccountsDistinct` conjunct.
        /// `from`/`to` are a sanitized `u64` mirror of `amenable_gaap::
        /// AccountId`'s real `Uuid`-backed identity — see this module's
        /// own doc comment for why.
        #[logic(open)]
        pub fn accounts_distinct_holds(from: u64, to: u64, outcome: bool) -> bool {
            pearlite! { outcome == (from != to) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own
        /// `payload.from() != payload.to()` check, isolated the same
        /// way `AmountPositive`/`SufficientFunds` are.
        #[requires(true)]
        #[ensures(accounts_distinct_holds(from, to, result))]
        fn check_accounts_distinct(from: u64, to: u64) -> bool {
            from != to
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AccountsDistinct {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_accounts_distinct".to_owned(),
                VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AccountsDistinct {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        ACCOUNTS_DISTINCT_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, TRANSFER_OUTCOME_MIRROR_SRC, {
        /// Sanitized mirror of `amenable_kani::ledger::TransferError`,
        /// for the combined claim below — real Rust enum, destructured
        /// via Pearlite `match`, which needs no `PartialEq`/`Eq` derive
        /// at all (pattern matching, not an `==` comparison) — dropped
        /// after a real translation error: `creusot-rustc` requires a
        /// `DeepModel` impl for any type deriving `PartialEq`, matching
        /// `CreusotVerifierMetadata`'s own precedent in this crate's
        /// `witness.rs`.
        enum TransferOutcome {
            Ok,
            NegativeAmount(i64),
            InsufficientFunds { balance: i64, required: i64 },
            SameAccount,
        }
    }
}

amenable_derive::harness! {
    creusot, VALIDATED_HOLDS_SRC, {
        /// The real, combined `Validated` postcondition — matches
        /// `amenable_kani::ledger::Validated::validate_ensures`'s exact
        /// match-arm shape (including the real claim's own asymmetry:
        /// the `Ok` arm doesn't restate `SufficientFunds`, only
        /// `AmountPositive`/`AccountsDistinct` — `SufficientFunds` gates
        /// whether `Ok` is reached at all, but isn't re-asserted once
        /// it has been), composing the four isolated conjuncts above
        /// rather than restating their content.
        #[logic(open)]
        fn validated_holds(amount: i64, from: u64, to: u64, outcome: TransferOutcome) -> bool {
            pearlite! {
                match outcome {
                    TransferOutcome::Ok => amount_positive_holds(amount, true) && accounts_distinct_holds(from, to, true),
                    TransferOutcome::NegativeAmount(bad) => bad == amount && amount@ <= 0,
                    TransferOutcome::InsufficientFunds { balance, required } => balance < required,
                    TransferOutcome::SameAccount => true,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VALIDATE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own real
        /// body -- calls the same three isolated checks in the same
        /// order, short-circuiting the same way.
        #[requires(true)]
        #[ensures(validated_holds(amount, from, to, result))]
        fn validate(balance: i64, amount: i64, from: u64, to: u64) -> TransferOutcome {
            if !check_amount_positive(amount) {
                return TransferOutcome::NegativeAmount(amount);
            }
            if !check_sufficient_funds(balance, amount) {
                return TransferOutcome::InsufficientFunds { balance, required: amount };
            }
            if !check_accounts_distinct(from, to) {
                return TransferOutcome::SameAccount;
            }
            TransferOutcome::Ok
        }
    }
}

/// Proof artifact for a `Validated`/`Committed` claim that currently
/// rests on more than one isolated Creusot check (`Validated` on
/// `AmountPositive`/`SufficientFunds`/`AccountsDistinct`, each proven
/// separately — see this module's own doc comment for why there is no
/// single composed claim yet, matching `Ledger::check_amount_positive`/
/// `::check_sufficient_funds` being proven separately on the Kani side
/// too). Uses `crate::witness::MultiCheckProof` — see that type's own
/// doc comment for why it's `#[cfg(not(creusot))]` at its definition
/// site, not here (a real internal compiler panic, confirmed the hard
/// way, building this very module).
#[cfg(not(creusot))]
fn validated_proof() -> crate::witness::MultiCheckProof {
    crate::witness::MultiCheckProof {
        checks: vec![
            (
                "check_amount_positive".to_owned(),
                VERIFY_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
            ),
            (
                "check_sufficient_funds".to_owned(),
                VERIFY_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
            ),
            (
                "check_accounts_distinct".to_owned(),
                VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC.to_owned(),
            ),
        ],
    }
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Validated {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_gaap::Validated",
        verifier: "creusot",
        describe: || validated_proof().to_string(),
    }
}

amenable_derive::harness! {
    creusot, BALANCED_ENTRIES_HOLDS_SRC, {
        /// The `Committed` postcondition's `BalancedEntries` conjunct
        /// (`debit + credit == 0`) — honestly tautological by
        /// construction here (`debit` is literally `-credit`), matching
        /// the real Kani-side claim's own documented triviality.
        /// `@`-lifted throughout: `-amount` on a fully unconstrained
        /// `i64` overflows at `i64::MIN` in ordinary Rust arithmetic
        /// (the exact real CBMC timeout `GAAP_LEDGER_PLAN.md`'s Step 2
        /// hit on the Kani side), but Pearlite's `Int` is
        /// arbitrary-precision, so the *claim itself* has no such
        /// overflow — `check_commit_balances` below still carries the
        /// same real `amount > 0` precondition Kani's own `Ledger::
        /// commit` needed, since the *function body*'s ordinary `i64`
        /// negation is not overflow-safe merely because the logical
        /// claim about it is.
        #[logic(open)]
        pub fn balanced_entries_holds(debit: i64, credit: i64, outcome: bool) -> bool {
            pearlite! { outcome == (debit@ + credit@ == 0) }
        }
    }
}

amenable_derive::harness! {
    creusot, COMMITTED_AMOUNT_HOLDS_SRC, {
        /// Real captured `commit` companion's own postcondition (`GAAP_
        /// LEDGER_PLAN.md`'s Step 6) -- unlike `check_commit_balances`
        /// above, whose own `#[requires(amount@ > 0)]` guards its
        /// *exec* body's real `-amount` negation, the generated
        /// `commit` companion carries no precondition at all (its real
        /// body never computes `-amount`, only `check_commit_balances`'s
        /// own isolated proof does) -- so this needs to be provable for
        /// *every* `i64`, not just positive ones. `@`-lifts `amount`
        /// *before* negating, inside this function's own body, rather
        /// than computing `-amount` at `i64` width at the ensures
        /// clause's own call site the way `balanced_entries_holds(
        /// -amount, amount, result)` does above -- confirmed the hard
        /// way: that shape needs `amount > 0` to avoid a real overflow
        /// obligation at `i64::MIN`, and this call site has no such
        /// precondition to rely on.
        #[logic(open)]
        pub fn committed_amount_holds(amount: i64) -> bool {
            pearlite! { (-amount@) + amount@ == 0 }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_COMMIT_BALANCES_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::commit`'s own claim
        /// -- see the real edge's own doc comment for why `amount > 0`
        /// is a genuine precondition, not an artifact of what a harness
        /// happens to assume.
        #[requires(amount@ > 0)]
        #[ensures(balanced_entries_holds(-amount, amount, result))]
        fn check_commit_balances(amount: i64) -> bool {
            let debit = -amount;
            let credit = amount;
            debit + credit == 0
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for BalancedEntries {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_commit_balances".to_owned(),
                VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for BalancedEntries {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        BALANCED_ENTRIES_HOLDS_SRC
    }
}

#[cfg(not(creusot))]
fn committed_proof() -> crate::witness::MultiCheckProof {
    crate::witness::MultiCheckProof {
        checks: vec![(
            "check_commit_balances".to_owned(),
            VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
        )],
    }
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Committed {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_gaap::Committed",
        verifier: "creusot",
        describe: || committed_proof().to_string(),
    }
}

// `reject`'s/`rollback`'s own claims are legitimately trivial (`result.
// is_ok()`, matching every `Stoplight` edge's own shape) -- no isolated
// Pearlite predicate to name here the way `Validated`'s/`Committed`'s
// own combined claims do, matching `Pending`'s/`AccountsDistinct`'s/
// `BalancedEntries`'s own trivial `Witness<CreusotVerifier>` impls
// above. `GAAP_LEDGER_PLAN.md`'s Step 7, revisited: connected on
// Creusot/Verus for the first time, closing the asymmetry `Stoplight`'s
// own equally trivial edges never had.
#[cfg(creusot)]
impl Witness<CreusotVerifier> for Rejected<Pending> {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Rejected<Validated> {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

// `Ledger::validate`'s/`::commit`'s/`::reject`'s/`::rollback`'s real
// bodies -- generated by `amenable::creusot_export` from `amenable_
// core::ExchangeEdgeRecord`, not hand-written or hand-copied, matching
// `amenable_creusot::stoplight`'s own three edges exactly (`GAAP_LEDGER_
// PLAN.md`'s Step 6). `include!`, not `mod`: shares this file's own
// scope directly (`Ledger`/`Transfer`/`TransferError`/`Pending`/
// `Validated`/`Committed`/`Rejected`/tokens above, already in scope).
// Regenerate with `just generate-creusot` after changing a real
// Kani-side transition; do not hand-edit the included files.
include!("generated/validate.rs");
include!("generated/commit.rs");
include!("generated/reject.rs");
include!("generated/rollback.rs");
