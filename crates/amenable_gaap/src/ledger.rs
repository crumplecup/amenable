//! Real `Transfer<S, Token>`/`TransferError`/`Ledger` types and logic --
//! `GAAP_LEDGER_PLAN.md`'s Step 9: closing the `Ledger` mirror on Creusot/
//! Verus the same way the token mirror was closed (Step 7/8) -- move the
//! real type *and* its real logic here, and let each backend attach its
//! own proof separately rather than needing its own copy of the struct or
//! the logic.
//!
//! **Scoped to `commit` only, so far.** A deliberate, minimal slice --
//! `commit` is the simplest real edge (infallible, one real claim) --
//! chosen to prove the pattern before extending it to `validate`/`check_
//! amount_positive`/`check_sufficient_funds`/`reject`/`rollback`. Kani has
//! no `extern_spec!`-style external-contract mechanism the way Creusot
//! does, so a real `#[kani::requires]`/`#[kani::ensures]` contract has to
//! sit directly on a function `amenable_kani` itself owns -- the first,
//! wrong assumption here was that this meant a *wrapper*, since inherent
//! impls can't reach across a crate boundary the way trait impls can. Real
//! finding, confirmed three ways via `amenable_kani::gallery::
//! ledger_gaap_free_function_contract`: a Kani-contracted function whose
//! body *delegates* to this method from a separate wrapper hits a genuine
//! Kani 0.67.0 DFCC scaffolding failure (`free.frees.1`), regardless of
//! the wrapper's shape (bare free function, associated function on a
//! local type) or the contract's content (real claim, trivial `true`).
//! The actual fix, confirmed clean (`commit_contract_no_wrapper`, `0 of
//! 287 failed`): attach the contract *directly* to this method, with zero
//! delegation, using a fully generic `Ensures<V>` bound rather than
//! naming any concrete verifier -- see `commit`'s own doc comment below.

use amenable_core::{Establish, Sidecar};

use crate::{
    AccountId, AccountsDistinct, AmountPositive, BalancedEntries, Committed, CommittedToken,
    Pending, PendingToken, Rejected, RejectedFromPendingToken, RejectedFromValidatedToken,
    SufficientFunds, TransferPayload, Validated, ValidatedToken,
};

/// A transfer payload bundled with the specific proof token minted for
/// its current state. `constructor = "pub(crate)"`: lawful construction
/// still requires a real token (this crate's own `Establish`-minted, or
/// [`Transfer::pending`]'s root case), so external crates can't
/// disconnect a token from a real `establish()` call the way a fully
/// `pub` constructor would allow.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "S", constructor = "pub(crate)")]
pub struct Transfer<S, Token> {
    #[sidecar(primary)]
    payload: TransferPayload,
    #[sidecar(token)]
    token: Token,
    _state: std::marker::PhantomData<S>,
}

impl<S, Token> Transfer<S, Token> {
    /// Diagnostic-only construction, bypassing the lawful `Establish`/
    /// `Sidecar` chain -- matches `ValidatedToken::diagnostic_only`'s own
    /// precedent and reasoning (`GAAP_LEDGER_PLAN.md`'s Step 1): real,
    /// structural CBMC cost from constructing a `Transfer` via the
    /// lawful chain in a harness's own setup code, independent of the
    /// contract actually being checked. `#[cfg(kani)]`, not privacy-
    /// gated further: the crate calling this (`amenable_kani`'s own
    /// experiments/gallery) is no longer the crate defining `Transfer`
    /// (`GAAP_LEDGER_PLAN.md`'s Step 9) -- `#[cfg(kani)]` is the real
    /// gate instead, relying on the same global-`--cfg` scoping
    /// `ValidatedToken::diagnostic_only`'s own doc comment explains.
    #[cfg(kani)]
    pub fn diagnostic_new(payload: TransferPayload, token: Token) -> Self {
        Self::new(payload, token)
    }
}

impl Transfer<Pending, PendingToken> {
    /// The entry case: every transfer starts `Pending`, asserted rather
    /// than reached via any transition.
    #[must_use]
    pub fn pending(payload: TransferPayload) -> Self {
        Self::new(payload, PendingToken::new(Pending))
    }
}

/// Every reason `Ledger::validate` can refuse a transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub enum TransferError {
    /// The transfer amount was not positive.
    NegativeAmount(i64),
    /// The source account's balance was less than the transfer amount.
    InsufficientFunds {
        /// The source account's actual balance.
        balance: i64,
        /// The amount that was required.
        required: i64,
    },
    /// The source and destination accounts were the same.
    SameAccount,
}

/// The source account's ledger state a transfer validates against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct Ledger {
    balance: i64,
}

impl Ledger {
    /// Construct a ledger view of an account holding `balance`.
    #[must_use]
    pub fn new(balance: i64) -> Self {
        Self { balance }
    }

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
    fn negative_amount(bad: i64) -> TransferError {
        TransferError::NegativeAmount(bad)
    }

    /// Same real reason as [`Ledger::negative_amount`] -- a real
    /// function, not a destructuring closure: Verus's translator
    /// separately does not (yet) support "only variables .. not general
    /// patterns" in a closure parameter position either.
    fn insufficient_funds(bad: (i64, i64)) -> TransferError {
        TransferError::InsufficientFunds {
            balance: bad.0,
            required: bad.1,
        }
    }

    /// The account's current balance.
    #[must_use]
    pub fn balance(&self) -> i64 {
        self.balance
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
    creusot_ensures = "match result { Ok(validated) => amount_positive_holds(validated.payload.amount.0, true) && accounts_distinct_holds(validated.payload.from, validated.payload.to, true), Err(TransferError::NegativeAmount(bad)) => amount_positive_holds(bad, false), Err(TransferError::InsufficientFunds { balance, required }) => sufficient_funds_holds(balance, required, false), Err(TransferError::SameAccount) => true, }",
    method_generics = "V"
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
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Transfer<Validated, ValidatedToken>, TransferError>| match result {
            Ok(validated) => {
                let payload = validated.primary();
                <AmountPositive as amenable_core::Ensures<V>>::ensures(payload.amount().value())
                    && <AccountsDistinct as amenable_core::Ensures<V>>::ensures((
                        payload.from().clone(),
                        payload.to().clone(),
                    ))
            }
            Err(TransferError::NegativeAmount(amount)) => {
                !<AmountPositive as amenable_core::Ensures<V>>::ensures(*amount)
            }
            Err(TransferError::InsufficientFunds { balance, required }) => {
                !<SufficientFunds as amenable_core::Ensures<V>>::ensures((*balance, *required))
            }
            Err(TransferError::SameAccount) => true,
        })
    )]
    pub fn validate<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Pending, PendingToken>,
    ) -> Result<Transfer<Validated, ValidatedToken>, TransferError>
    where
        Pending: amenable_core::Evidence + amenable_core::Witness<V>,
        Validated: amenable_core::Evidence + amenable_core::Witness<V>,
        PendingToken: amenable_core::ProofToken<Proposition = Pending> + Clone,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        AmountPositive: amenable_core::Ensures<V, Input = i64, Bound = bool>,
        SufficientFunds: amenable_core::Ensures<V, Input = (i64, i64), Bound = bool>,
        AccountsDistinct: amenable_core::Ensures<V, Input = (AccountId, AccountId), Bound = bool>,
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
    creusot_ensures = "match result { Ok(committed) => committed_amount_holds(committed.payload.amount.0), Err(_) => false, }"
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
    /// `GAAP_LEDGER_PLAN.md`'s Step 9 gallery investigation
    /// (`amenable_kani::gallery::ledger_gaap_free_function_contract`)
    /// confirmed, three ways, that a Kani-contracted function whose body
    /// *delegates* to this method from a separate wrapper hits a real
    /// Kani 0.67.0 DFCC scaffolding failure (`free.frees.1`), regardless
    /// of the wrapper's shape or the contract's content -- but attaching
    /// the contract directly here, with zero delegation, verifies clean
    /// (`commit_contract_no_wrapper`, `0 of 287 failed`). `BalancedEntries:
    /// Ensures<V, ..>` is an extra bound calling through the trait
    /// generically, never naming a concrete verifier, so this still
    /// compiles from `amenable_gaap` with no dependency on any backend
    /// crate -- satisfied at the call site by whichever backend's own
    /// real `Ensures<V>` impl for `BalancedEntries` already exists there.
    /// Same `cargo kani`-sets-`--cfg kani`-globally mechanism `TransferError`'s/
    /// `Ledger`'s own `#[cfg_attr(kani, derive(kani::Arbitrary))]` already
    /// relies on.
    #[cfg_attr(kani, kani::requires(input.primary().amount().value() > 0))]
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Transfer<Committed, CommittedToken>, TransferError>| match result {
            Ok(committed) => <BalancedEntries as amenable_core::Ensures<V>>::ensures(committed.primary().amount().value()),
            Err(_) => false,
        })
    )]
    pub fn commit<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Committed, CommittedToken>, TransferError>
    where
        Validated: amenable_core::Evidence + amenable_core::Witness<V>,
        Committed: amenable_core::Evidence + amenable_core::Witness<V>,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        CommittedToken: amenable_core::ProofToken<Proposition = Committed> + Clone,
        BalancedEntries: amenable_core::Ensures<V, Input = i64, Bound = bool>,
    {
        let payload = input.primary().clone();
        let token = Committed::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

#[amenable_derive::capture_exchange_body(evidence = "Rejected<Pending>")]
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
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Transfer<Rejected<Pending>, RejectedFromPendingToken>, TransferError>| result.is_ok())
    )]
    pub fn reject<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Pending, PendingToken>,
    ) -> Result<Transfer<Rejected<Pending>, RejectedFromPendingToken>, TransferError>
    where
        Pending: amenable_core::Evidence + amenable_core::Witness<V>,
        Rejected<Pending>: amenable_core::Evidence + amenable_core::Witness<V>,
        PendingToken: amenable_core::ProofToken<Proposition = Pending> + Clone,
        RejectedFromPendingToken:
            amenable_core::ProofToken<Proposition = Rejected<Pending>> + Clone,
    {
        let payload = input.primary().clone();
        let token = Rejected::<Pending>::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

#[amenable_derive::capture_exchange_body(evidence = "Rejected<Validated>")]
impl Ledger {
    /// `Validated -> Rejected<Validated>`: infallible, like [`Ledger::
    /// reject`] above -- see its own doc comment for why the claim is
    /// legitimately trivial (including why no `creusot_ensures`
    /// override). Generic over `V`, direct contract, no delegating
    /// wrapper -- see `commit`'s own doc comment for the confirmed
    /// reasoning.
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Transfer<Rejected<Validated>, RejectedFromValidatedToken>, TransferError>| result.is_ok())
    )]
    pub fn rollback<V: amenable_core::Verifier>(
        &self,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Rejected<Validated>, RejectedFromValidatedToken>, TransferError>
    where
        Validated: amenable_core::Evidence + amenable_core::Witness<V>,
        Rejected<Validated>: amenable_core::Evidence + amenable_core::Witness<V>,
        ValidatedToken: amenable_core::ProofToken<Proposition = Validated> + Clone,
        RejectedFromValidatedToken:
            amenable_core::ProofToken<Proposition = Rejected<Validated>> + Clone,
    {
        let payload = input.primary().clone();
        let token = Rejected::<Validated>::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}
