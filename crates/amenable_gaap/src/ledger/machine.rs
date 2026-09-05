//! The `Ledger` state machine and its edge methods (`check_amount_positive`/
//! `check_sufficient_funds`, `validate`, `commit`, `reject`, `rollback`).
//! The `Transfer<S, Token>` carrier and `TransferError` live in `types`.
//!
//! Kani has no `extern_spec!`-style external-contract mechanism the way
//! Creusot does, so a real `#[kani::requires]`/`#[kani::ensures]` contract
//! has to sit directly on a function `amenable_kani` itself owns. Confirmed
//! three ways via `amenable_kani::gallery::ledger_gaap_free_function_contract`:
//! a Kani-contracted function whose body *delegates* to one of these methods
//! from a separate wrapper hits a genuine Kani 0.67.0 DFCC scaffolding
//! failure (`free.frees.1`), regardless of the wrapper's shape or the
//! contract's content. The fix, confirmed clean: attach the contract
//! *directly* to the method, zero delegation, with a fully generic
//! `Ensures<V>` bound rather than naming any concrete verifier -- see
//! `commit`'s own doc comment below.

use amenable_core::{Establish, Sidecar};

use crate::{
    AmountPositive, Committed, CommittedToken, Pending, PendingToken, Rejected,
    RejectedFromPendingToken, RejectedFromValidatedToken, SufficientFunds, Transfer, TransferError,
    TransferPayload, Validated, ValidatedToken,
};

/// The source account's ledger state a transfer validates against.
///
/// The second design canary for `docs/STATE_MACHINE_DERIVATION_PLAN.md`,
/// after `Stoplight` — and the one that forced a real fix rather than a
/// special case. `Ledger`'s methods had no `Exchange` trait impl at all
/// (generic over `V`, registered only via `#[capture_exchange_body(..)]`,
/// which deliberately didn't generate one), so `#[derive(StateMachine)]`'s
/// static assertion had nothing to check — not a limitation to work
/// around in the derive, but the derive correctly catching a real gap
/// this crate's own methods had. `capture_exchange_body` now generates a
/// real `impl<V: Verifier> Exchange<Input, Output, V> for Self`
/// unconditionally, generic rather than tied to one backend (`#[exchange(
/// ..)]`'s own concrete-verifier bundle can't apply here — `Ledger`
/// stays neutral, no dependency on any backend crate). `#[state_machine(
/// generic_over_verifier, ..)]` below matches that shape: no concrete
/// verifier named anywhere (this crate can't name one), a genuinely
/// `for<V: Verifier>`-checked static assertion instead of a
/// per-instantiation one, and a single blanket `impl<V: Verifier>
/// StateMachine<V> for Ledger`.
///
/// `Pending`'s fourth and fifth `state(..)` arguments name its real
/// root constructor and the real seed type it needs: unlike
/// `Stoplight`'s `Green` (`Established::<Green, GreenToken>::root()`,
/// zero arguments), `Transfer::pending` takes a real `TransferPayload`
/// -- a downstream consumer reading `root_entries()` sees `Pending` is
/// root-enterable *and* exactly what it needs to supply to enter it,
/// rather than `Pending` being silently absent from the audit surface
/// the way an earlier, zero-argument-only version of this mechanism
/// left it (`RootEntry::seed`'s own doc comment in `amenable_core::
/// state_machine` has the full account).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    amenable_derive::StateMachine,
    derive_getters::Getters,
    derive_new::new,
)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[state_machine(
    generic_over_verifier,
    state(
        "Pending",
        "Transfer<Pending, PendingToken>",
        "Transfer::pending",
        "TransferPayload"
    ),
    state("Validated", "Transfer<Validated, ValidatedToken>"),
    state("Committed", "Transfer<Committed, CommittedToken>"),
    state(
        "Rejected<Pending>",
        "Transfer<Rejected<Pending>, RejectedFromPendingToken>"
    ),
    state(
        "Rejected<Validated>",
        "Transfer<Rejected<Validated>, RejectedFromValidatedToken>"
    ),
    edge("Pending", "Validated"),
    edge("Validated", "Committed"),
    edge("Pending", "Rejected<Pending>"),
    edge("Validated", "Rejected<Validated>")
)]
pub struct Ledger {
    /// The account's current balance. `derive_new`/`derive_getters` have
    /// no way to carry the `#[must_use]` this constructor/getter used to
    /// have -- confirmed against both crates' own docs -- but neither
    /// clippy lint that would care (`must_use_candidate`) is enabled in
    /// this workspace, so nothing actually stops warning either way.
    #[getter(copy)]
    balance: i64,
}

impl Ledger {
    /// `AmountPositive`, isolated: `amount > 0`. Broken out from
    /// `validate`'s own body specifically so this can be proven cheaply
    /// on its own -- see `GAAP_LEDGER_PLAN.md`'s Step 1: the combined
    /// body times out under CBMC once `amount` is symbolic, because a
    /// symbolic condition then selects between `validate`'s own
    /// heap-allocating `Ok` arm and a non-allocating `Err` arm. Neither
    /// arm *here* allocates, so there's nothing for that cost to attach
    /// to.
    ///
    /// The `Err` arm's own claim must be a full biconditional (`*bad ==
    /// amount && !AmountPositive::ensures(amount)`, not just `*bad ==
    /// amount`) -- a real bug found composing `validate` via `#[kani::
    /// stub_verified]`: an under-specified `Err` claim still passes this
    /// function's own isolated `#[kani::proof_for_contract]` check (a
    /// weaker contract is trivially satisfied by the real body's actual
    /// behavior), but `stub_verified` treats the contract as the
    /// *complete* story -- it's free to substitute `Err` even when
    /// `amount > 0`, since nothing here forbade it, which broke
    /// `validate`'s own downstream claim that `Err(NegativeAmount(
    /// amount))` implies the amount wasn't positive.
    ///
    /// Generic over `V`, direct contract, no delegating wrapper -- see
    /// `commit`'s own doc comment for the confirmed reasoning.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<(), i64>| match result {
            Ok(()) => <AmountPositive as amenable_core::Ensures<V>>::ensures(amount),
            Err(bad) => {
                *bad == amount && !<AmountPositive as amenable_core::Ensures<V>>::ensures(amount)
            }
        })
    )]
    pub fn check_amount_positive<V: amenable_core::Verifier>(amount: i64) -> Result<(), i64>
    where
        AmountPositive: amenable_core::Ensures<V, Input = i64, Bound = bool>,
    {
        if amount <= 0 { Err(amount) } else { Ok(()) }
    }

    /// `SufficientFunds`, isolated: `balance >= amount`. See
    /// [`Ledger::check_amount_positive`] for why this is broken out.
    /// Generic over `V`, direct contract, no delegating wrapper -- see
    /// `commit`'s own doc comment for the confirmed reasoning.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<(), (i64, i64)>| match result {
            Ok(()) => <SufficientFunds as amenable_core::Ensures<V>>::ensures((self.balance, amount)),
            Err((balance, required)) => {
                !<SufficientFunds as amenable_core::Ensures<V>>::ensures((self.balance, amount))
                    && *balance == self.balance
                    && *required == amount
            }
        })
    )]
    pub fn check_sufficient_funds<V: amenable_core::Verifier>(
        &self,
        amount: i64,
    ) -> Result<(), (i64, i64)>
    where
        SufficientFunds: amenable_core::Ensures<V, Input = (i64, i64), Bound = bool>,
    {
        if self.balance < amount {
            Err((self.balance, amount))
        } else {
            Ok(())
        }
    }

    /// A real function, not the bare `TransferError::NegativeAmount`
    /// tuple-variant constructor -- `validate`'s own `.map_err(..)` call
    /// below is captured verbatim into a real Verus companion, and
    /// Verus's own translator does not (yet) support "using a datatype
    /// constructor as a function value" (confirmed against the real
    /// toolchain), even though ordinary `rustc`/Kani/Creusot all accept
    /// it. A closure would dodge that error too, but trades it for a
    /// real `clippy::redundant_closure` failure this project cannot
    /// `#[allow]` away -- this wrapper is the one form both toolchains
    /// accept.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn negative_amount(bad: i64) -> TransferError {
        TransferError::NegativeAmount(bad)
    }

    /// Same real reason as [`Ledger::negative_amount`] -- a real
    /// function, not a destructuring closure: Verus's translator
    /// separately does not (yet) support "only variables .. not general
    /// patterns" in a closure parameter position either.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn insufficient_funds(bad: (i64, i64)) -> TransferError {
        TransferError::InsufficientFunds {
            balance: bad.0,
            required: bad.1,
        }
    }
}

// `validate`/`commit` each sit in their own, single-method `impl Ledger {
// .. }` block below, decorated with `#[amenable_derive::
// capture_exchange_body(..)]` -- not folded into the main block above,
// the same real reason `#[amenable_derive::exchange(..)]` (this macro's
// concrete-verifier-bundling sibling) has always required it: `Span::
// source_text()`'s own verbatim capture only works when the captured
// item sits directly at the attribute's own call site (`exchange.rs`'s
// own doc comment). `GAAP_LEDGER_PLAN.md`'s Step 7's own follow-up:
// registers a real `ExchangeEdgeRecord` from each method's real body --
// orphaned when `#[exchange(..)]` was removed from these methods (moving
// them here, generic over `V`, left no concrete verifier for that
// macro's own bundle to name) -- so `amenable_creusot`'s/`amenable_
// verus`'s generated companions (`just generate-creusot`/`just
// generate-verus-exchange`) have a real source to regenerate from again,
// instead of silently going stale.

#[amenable_derive::capture_exchange_body(
    evidence = "Validated",
    creusot_ensures = "validated_result_holds(result)",
    method_generics = "V",
    kani_ensures = "true"
)]
impl Ledger {
    /// `Pending -> Validated`: the first genuinely branching, data-
    /// dependent claim in this worked example. Calls through `check_
    /// amount_positive`/`check_sufficient_funds` (above) rather than
    /// inlining their checks -- not a style preference, a real fix for
    /// the CBMC timeout `GAAP_LEDGER_PLAN.md`'s Step 1 hit and root-
    /// caused via `amenable_kani::gallery::ledger_account_id_comparison`:
    /// a symbolic condition selecting between this function's own
    /// heap-allocating `Ok` arm and a non-allocating `Err` arm blew up
    /// CBMC's enum-drop modeling regardless of how few symbolic fields
    /// were involved.
    ///
    /// Generic over `V`, direct contract, no delegating wrapper -- see
    /// `commit`'s own doc comment for the confirmed reasoning. Calls
    /// through `check_amount_positive::<V>`/`check_sufficient_funds::<V>`
    /// with an explicit turbofish: nothing in either call's own
    /// arguments/return type structurally names `V`, so Rust has no
    /// other way to resolve which instantiation this call targets.
    ///
    /// `kani_ensures = "true"` on the attribute above generates this
    /// method's own contract: `<Validated as Ensures<V>>::ensures(
    /// result.clone())`, calling through the target evidence type's own
    /// registered claim rather than restating the combined biconditional
    /// inline -- manually re-deriving a composite bound (`AmountPositive`/
    /// `SufficientFunds`/`AccountsDistinct`, each named separately) is
    /// the same anti-pattern this project already rejects everywhere
    /// else: `Validated`'s own `Ensures<KaniVerifier>` impl (`amenable_
    /// kani::ledger`'s `kani_ensures!(Validated, ..)`) already IS the
    /// one real, registered claim this transition proves -- restating it
    /// here a second time would be two hand-typed copies of the same
    /// logic with nothing enforcing they stay in sync, exactly what
    /// `#[amenable_derive::exchange(..)]`'s own generated contracts
    /// (`<Evidence as Ensures<V>>::ensures(result.clone())`, never
    /// restated) always avoided for `Stoplight`. `Validated` here plays
    /// the identical role `Yellow`/`Red` do there: the evidence a
    /// transition establishes doubles as its own postcondition's real
    /// `Ensures<V>` carrier, not a separate, redundant contract type. As
    /// of `GAAP_LEDGER_PLAN.md`'s Step 7's own follow-up ("manual bounds
    /// are an anti-pattern"), this exact call-through shape is generated
    /// mechanically rather than hand-typed a fourth time, since
    /// `validate`/`commit`/`reject`/`rollback` all converged on the
    /// identical pattern once each stopped restating its own claim.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, input)))]
    pub fn validate<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Pending, PendingToken>,
    ) -> Result<Transfer<Validated, ValidatedToken>, TransferError>
    where
        Pending: amenable_core::Evidence + amenable_core::Witness<V>,
        Validated: amenable_core::Evidence
            + amenable_core::Witness<V>
            + amenable_core::Ensures<
                V,
                Input = Result<Transfer<Validated, ValidatedToken>, TransferError>,
                Bound = bool,
            >,
        PendingToken: amenable_core::ProofToken<Proposition = Pending> + Clone,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        AmountPositive: amenable_core::Ensures<V, Input = i64, Bound = bool>,
        SufficientFunds: amenable_core::Ensures<V, Input = (i64, i64), Bound = bool>,
    {
        let payload = input.primary().clone();
        let amount = payload.amount().value();

        Self::check_amount_positive::<V>(amount).map_err(Self::negative_amount)?;
        self.check_sufficient_funds::<V>(amount)
            .map_err(Self::insufficient_funds)?;
        if payload.from() == payload.to() {
            return Err(TransferError::SameAccount);
        }

        let token = Validated::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

#[amenable_derive::capture_exchange_body(
    evidence = "Committed",
    creusot_ensures = "committed_result_holds(result)",
    kani_ensures = "true",
    kani_requires_evidence = "Validated"
)]
impl Ledger {
    /// `Validated -> Committed`: infallible -- a transfer that already
    /// passed `validate`'s own checks has nothing left to reject at
    /// commit time.
    ///
    /// Generic over `V`, not fixed to any backend: nothing in `Transfer<
    /// Validated, ValidatedToken>`'s own type structurally names a
    /// verifier (`V` only ever appears in a `Sidecar<V>`/`Establish<C,
    /// V>` impl's own bounds), so calling `Committed::establish(..)`/
    /// `input.sidecar()` needs *some* `V` in scope to resolve at all --
    /// real, not decorative, confirmed by the compiler rejecting a
    /// non-generic signature outright (`Committed: Witness<_>` unsolved).
    /// Each backend's own caller supplies its own concrete `V` at the
    /// call site (`ledger.commit::<KaniVerifier>(input)`), the same way
    /// the backend-generic `Establish`/`Sidecar` blanket impls this body
    /// relies on are themselves only ever *used* concretely, never
    /// defined concretely.
    ///
    /// `#[cfg_attr(kani, ..)]` carries a real Kani contract *directly* --
    /// `GAAP_LEDGER_PLAN.md`'s Step 7 gallery investigation
    /// (`amenable_kani::gallery::ledger_gaap_free_function_contract`)
    /// confirmed, three ways, that a Kani-contracted function whose body
    /// *delegates* to this method from a separate wrapper hits a real
    /// Kani 0.67.0 DFCC scaffolding failure (`free.frees.1`), regardless
    /// of the wrapper's shape or the contract's content -- but attaching
    /// the contract directly here, with zero delegation, verifies clean
    /// (`commit_contract_no_wrapper`, `0 of 287 failed`).
    ///
    /// `kani_ensures = "true"`/`kani_requires_evidence = "Validated"` on
    /// the attribute above generate this method's own contract: a real
    /// precondition, `<Validated as Requires<V>>::requires(input.clone())`,
    /// plus `<Committed as Ensures<V>>::ensures(result.clone())` rather
    /// than restating either bound inline -- see `validate`'s own doc
    /// comment for the full "manual bounds are an anti-pattern, call
    /// through the evidence type's own registered claim instead"
    /// reasoning. The precondition itself is not a fresh claim: `Validated`'s
    /// own `Requires<KaniVerifier>` impl (`amenable_kani::ledger`'s
    /// `kani_requires!(Validated, ..)`) delegates through the identical
    /// `AmountPositive` claim `validate`'s own postcondition already
    /// calls through -- a `Validated`-carrying `Transfer` is exactly the
    /// value flowing from `validate`'s output position into `commit`'s
    /// input position, so the same real fact serves both roles instead of
    /// two independently hand-typed copies (this used to be a raw
    /// `input.primary().amount().value() > 0` expression, restating what
    /// `AmountPositive` already states once). `Committed`'s own `Ensures<
    /// KaniVerifier>` impl (`amenable_kani::ledger`'s `kani_ensures!(
    /// Committed, ..)`) already calls through `BalancedEntries` itself, so
    /// restating that here too would just be a second, unsynchronized
    /// copy. This still compiles from `amenable_gaap` with no dependency
    /// on any backend crate -- satisfied at the call site by whichever
    /// backend's own real `Ensures<V>`/`Requires<V>` impls already exist
    /// there. Same `cargo kani`-sets-`--cfg kani`-globally mechanism
    /// `TransferError`'s/`Ledger`'s own `#[cfg_attr(kani, derive(kani::
    /// Arbitrary))]` already relies on.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, input)))]
    pub fn commit<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Committed, CommittedToken>, TransferError>
    where
        Validated: amenable_core::Evidence
            + amenable_core::Witness<V>
            + amenable_core::Requires<V, Input = Transfer<Validated, ValidatedToken>, Bound = bool>,
        Committed: amenable_core::Evidence
            + amenable_core::Witness<V>
            + amenable_core::Ensures<
                V,
                Input = Result<Transfer<Committed, CommittedToken>, TransferError>,
                Bound = bool,
            >,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        CommittedToken: amenable_core::ProofToken<Proposition = Committed> + Clone,
    {
        let payload = input.primary().clone();
        let token = Committed::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

#[amenable_derive::capture_exchange_body(evidence = "Rejected<Pending>", kani_ensures = "true")]
impl Ledger {
    /// `Pending -> Rejected<Pending>`: infallible, like `commit` --
    /// rejecting a still-pending transfer (an operator cancelling a
    /// request, or an external process auto-rejecting it, before
    /// validation is ever attempted) has no failure mode of its own. The
    /// claim is legitimately trivial (`result.is_ok()`), the same shape
    /// every `Stoplight` edge documents rather than hides -- `validate`/
    /// `commit` are where this worked example's real, non-trivial claims
    /// live. Generic over `V`, direct contract, no delegating wrapper --
    /// see `commit`'s own doc comment for the confirmed reasoning. No
    /// `creusot_ensures` override -- defaults to the literal `"true"`,
    /// matching every `Stoplight` edge's own identically trivial claim
    /// on Creusot (`green_to_yellow`/etc.), not a real biconditional the
    /// way `validate`'s/`commit`'s own claims need.
    ///
    /// `kani_ensures = "true"` on the attribute above generates this
    /// method's own contract, calling through `<Rejected<Pending> as
    /// Ensures<V>>::ensures(result.clone())` rather than restating
    /// `result.is_ok()` inline -- even a trivial claim is still a claim
    /// `Rejected<Pending>`'s own `Ensures<KaniVerifier>` impl already
    /// states once (`amenable_kani::ledger`'s `kani_ensures!(Rejected<
    /// Pending>, ..)`); restating it here too is the identical
    /// anti-pattern `validate`'s own doc comment explains, just with a
    /// shorter claim.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, input)))]
    pub fn reject<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Pending, PendingToken>,
    ) -> Result<Transfer<Rejected<Pending>, RejectedFromPendingToken>, TransferError>
    where
        Pending: amenable_core::Evidence + amenable_core::Witness<V>,
        Rejected<Pending>: amenable_core::Evidence
            + amenable_core::Witness<V>
            + amenable_core::Ensures<
                V,
                Input = Result<
                    Transfer<Rejected<Pending>, RejectedFromPendingToken>,
                    TransferError,
                >,
                Bound = bool,
            >,
        PendingToken: amenable_core::ProofToken<Proposition = Pending> + Clone,
        RejectedFromPendingToken:
            amenable_core::ProofToken<Proposition = Rejected<Pending>> + Clone,
    {
        let payload = input.primary().clone();
        let token = Rejected::<Pending>::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

#[amenable_derive::capture_exchange_body(evidence = "Rejected<Validated>", kani_ensures = "true")]
impl Ledger {
    /// `Validated -> Rejected<Validated>`: infallible, like [`Ledger::
    /// reject`] above -- see its own doc comment for why the claim is
    /// legitimately trivial (including why no `creusot_ensures`
    /// override, and why `kani_ensures = "true"` generates a contract
    /// calling through `Rejected<Validated>`'s own `Ensures<V>` impl
    /// rather than restating `result.is_ok()` inline). Generic over `V`,
    /// direct contract, no delegating wrapper -- see `commit`'s own doc
    /// comment for the confirmed reasoning.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, input)))]
    pub fn rollback<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Rejected<Validated>, RejectedFromValidatedToken>, TransferError>
    where
        Validated: amenable_core::Evidence + amenable_core::Witness<V>,
        Rejected<Validated>: amenable_core::Evidence
            + amenable_core::Witness<V>
            + amenable_core::Ensures<
                V,
                Input = Result<
                    Transfer<Rejected<Validated>, RejectedFromValidatedToken>,
                    TransferError,
                >,
                Bound = bool,
            >,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        RejectedFromValidatedToken:
            amenable_core::ProofToken<Proposition = Rejected<Validated>> + Clone,
    {
        let payload = input.primary().clone();
        let token = Rejected::<Validated>::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}
