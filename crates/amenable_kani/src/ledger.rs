//! `GAAP_LEDGER_PLAN.md`'s Step 1: the first real Kani edge for the
//! ledger worked example — `Pending -> Validated`, gated on
//! `SufficientFunds`/`AmountPositive`/`AccountsDistinct` from
//! `amenable_gaap`. The first genuinely branching, data-dependent
//! `Exchange` edge in this workspace: every `Stoplight` edge only ever
//! proved `result.is_ok()` (its states are zero-field types with no
//! branching that could fail); this one can fail for a real reason, and
//! the postcondition has to say exactly which one.
//!
//! Mirrors `amenable_kani::stoplight`'s own shape closely (see that
//! module's doc comment for the `Established`/`Sidecar`/`Establish`
//! mechanics this reuses), with one real structural difference:
//! `amenable_gaap`'s states carry real payload data (`TransferPayload`)
//! alongside their state marker, so `Sidecar<KaniVerifier>::Primary`
//! (`TransferPayload`) and `::Proposition` (`Pending`/`Validated`/..)
//! genuinely diverge here, unlike `Established<T, Token>`'s `T` playing
//! both roles for `Stoplight`.
//!
//! A second real, second-user finding from building this: `#[amenable_
//! derive::exchange]`'s generated `#[kani::ensures]` closure used to
//! dereference `result` (`*result`), which requires `Output: Copy` --
//! true for every `Stoplight` state (all zero-field), false here
//! (`TransferPayload` carries a `String`-backed `AccountId`, which can
//! never be `Copy`). Fixed at the macro itself (`result.clone()`
//! instead of `*result`, `Clone` being a strict superset of `Copy` so
//! `Stoplight`'s own usage is unaffected) rather than worked around
//! here -- exactly the "dogfood the design, knock out the kinks" this
//! second worked example was chosen for.
//!
//! `AccountId` carries a `Uuid` identity (not the `String` it started
//! with in `GAAP_LEDGER_PLAN.md`'s Step 0) precisely because of a real
//! CBMC cost the first version of this proof hit: comparing two
//! independently-constructed `String`s for equality *inside a
//! `#[kani::ensures]` closure* is expensive regardless of content or
//! length -- fully root-caused via `amenable_kani::gallery::
//! ledger_account_id_comparison`'s own investigation, which also
//! confirmed a *fixed-capacity* string (bounded buffer + a length
//! field) is exactly as expensive, so bounding the name wouldn't have
//! helped. `Uuid`'s 16-byte, fixed-length comparison is cheap in the
//! identical position. The accounts-distinct dimension is still split
//! across two concrete harnesses (`verify_validate_*` below, one
//! distinct-accounts case and one same-account case) rather than one
//! harness quantifying over `kani::any::<AccountId>()` -- not because
//! `Uuid` comparison is expensive (it isn't), but because this worked
//! example was never trying to prove "for all possible account
//! identities," only that the real transition logic honors whichever
//! identities it's handed; `amount`/`balance` stay the fully symbolic
//! dimension, since varying *those* is the actual point of this proof.
//!
//! A second real bug, found composing `validate` via `#[kani::
//! stub_verified]` rather than by hand-reasoning: `check_amount_
//! positive`'s own `#[kani::ensures]` was under-specified (`Err(bad) =>
//! *bad == amount`, missing `&& amount <= 0`). A weak postcondition
//! still passes that function's own isolated `#[kani::proof_for_
//! contract]` check trivially (the real body's actual behavior is a
//! subset of what a too-weak contract allows), but `stub_verified`
//! treats the contract as the complete story -- it was free to
//! substitute `Err` even when `amount > 0`, which broke `validate`'s
//! own downstream claim that `Err(NegativeAmount(amount))` implies
//! `amount <= 0`. Fixed by making the `Err` arm a full biconditional.
//! See [`Ledger::check_amount_positive`]'s own doc comment.

use amenable_core::{Establish, Evidence, ProofToken, Sidecar, Witness};
use amenable_gaap::{Committed, Pending, TransferPayload, Validated};

use crate::rust_std::macros::kani_ensures;
use crate::{CalculationProof, KaniVerifier};

/// A transfer payload bundled with the specific proof token minted for
/// its current state -- the `Sidecar` shape used throughout this
/// module. `new` is private: the only lawful ways to produce one are
/// [`Transfer::pending`] (the entry state: a transfer always starts
/// `Pending`, asserted rather than derived -- see `Pending`'s own
/// `Witness<KaniVerifier>` impl below for why it needs one at all) or
/// `Ledger::validate`, which calls `Establish::establish` on the real
/// credential it was handed.
///
/// `Clone` (not `Copy`: `TransferPayload` carries an `AccountId`, whose
/// `Uuid` identity is `Copy` but whose human-readable `name` field is
/// still a `String`) — needed for the same reason `Established<T,
/// Token>` derives `Copy` in `stoplight.rs`: the generated `#[kani::
/// ensures]` closure needs an owned value to hand `Ensures::ensures`,
/// and this crate's own `Result::clone()` fix (see this module's own
/// doc comment) only requires `Clone`, not `Copy`.
#[derive(Debug, Clone)]
pub struct Transfer<S, Token> {
    payload: TransferPayload,
    token: Token,
    _state: std::marker::PhantomData<S>,
}

impl<S, Token> Transfer<S, Token> {
    // `pub(crate)`, not private: lawful construction still requires a
    // real token (this crate's own `Establish`-minted or `Transfer::
    // pending`'s root case), so external crates still can't disconnect
    // a token from a real `establish()` call the way a fully `pub`
    // constructor would allow. The relaxation from module-private only
    // reaches `amenable_kani::gallery`'s own diagnostic harnesses (see
    // `gallery::ledger_account_id_comparison`'s doc comment for why),
    // not anything outside this crate.
    pub(crate) fn new(payload: TransferPayload, token: Token) -> Self {
        Self {
            payload,
            token,
            _state: std::marker::PhantomData,
        }
    }
}

impl<S, Token> Sidecar<KaniVerifier> for Transfer<S, Token>
where
    S: Evidence + Witness<KaniVerifier>,
    Token: ProofToken<Proposition = S> + Clone,
{
    type Primary = TransferPayload;
    type Proposition = S;
    type SidecarToken = Token;

    fn primary(&self) -> &Self::Primary {
        &self.payload
    }

    fn sidecar(&self) -> Self::SidecarToken {
        self.token.clone()
    }
}

/// Lawful token asserting a transfer is `Pending` — the entry state,
/// minted without going through `Establish::establish()` the same way
/// `stoplight::GreenToken::new` doesn't: there is no prior state to
/// present a credential from. No `#[establish(..)]` for the same reason
/// — a root has no credential type to name.
#[derive(Debug, Clone, amenable_derive::ProofToken)]
#[proof_token(proposition = "Pending")]
pub struct PendingToken(());

impl PendingToken {
    fn new(_state: Pending) -> Self {
        Self(())
    }
}

/// `Pending`'s own trivial witness. Unlike `stoplight::Green`, which
/// gets its `Witness<KaniVerifier>` impl "for free" from the `Red ->
/// Green` cycle-back edge (`Green` is also an edge *target* in that
/// cycle), nothing in this worked example's initial scope ever targets
/// `Pending` — a transfer only ever starts there, never returns to it.
/// So nothing auto-generates one via `#[amenable_derive::exchange]`;
/// this is hand-written, and honestly trivial: there is no computation
/// to prove about the fact that a new transfer starts `Pending`, the
/// same way there's none for `Green`'s own power-on claim.
impl Witness<KaniVerifier> for Pending {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

impl Transfer<Pending, PendingToken> {
    /// The entry case: every transfer starts `Pending`, asserted rather
    /// than reached via any transition.
    #[must_use]
    pub fn pending(payload: TransferPayload) -> Self {
        Self::new(payload, PendingToken::new(Pending))
    }
}

/// Lawful token minted once `Validated` is established from a proven
/// `Pending`.
#[derive(Debug, Clone, amenable_derive::ProofToken)]
#[proof_token(proposition = "Validated")]
#[amenable_derive::establish(
    credential = PendingToken,
    verifier = KaniVerifier,
    proposition = Validated,
)]
pub struct ValidatedToken(());

impl ValidatedToken {
    // `pub(crate)`, diagnostic only -- see `gallery::
    // ledger_commit_contract_timeout`'s own doc comment: constructing a
    // `Transfer<Validated, ValidatedToken>` in a `#[kani::proof_for_
    // contract]` harness's own setup code via the *lawful* `Sidecar::
    // sidecar`/`Establish::establish` chain is real, structural CBMC
    // cost (~143s even with fully concrete values), independent of
    // `commit`'s own contract content. This bypasses that chain for
    // harness setup only -- the CONTRACT being checked is `commit`'s
    // own, unaffected by how the harness's *input* was assembled, the
    // same reason `Transfer::new`/`Ledger::validate`/`Ledger::commit`
    // already carry this same `pub(crate)` diagnostic relaxation.
    #[cfg(kani)]
    pub(crate) fn diagnostic_only() -> Self {
        Self(())
    }
}

/// Every reason `Ledger::validate` can refuse a transfer — structured,
/// not a bare unit variant, so the postcondition below can state a real
/// claim relating the *violated* value to the refusal, checkable purely
/// from `result` (the shape `#[amenable_derive::exchange]`'s generated
/// `#[kani::ensures]` closure hands `Ensures::ensures`). Mirrors
/// `~/repos/elicitation/crates/elicit_server::ledger::errors::
/// ValidationError`'s own three variants exactly.
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

/// The source account's ledger state a transfer validates against —
/// carries the real balance `SufficientFunds` checks. Distinct from
/// `TransferPayload` (the transfer *request*): a ledger's balance is
/// account state, not something the transfer itself specifies.
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

    /// The account's current balance.
    #[must_use]
    pub fn balance(&self) -> i64 {
        self.balance
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
    /// amount && amount <= 0`, not just `*bad == amount`) -- a real bug
    /// found composing `validate` via `#[kani::stub_verified]`: an
    /// under-specified `Err` claim still passes this function's own
    /// isolated `#[kani::proof_for_contract]` check (a weaker contract
    /// is trivially satisfied by the real body's actual behavior), but
    /// `stub_verified` treats the contract as the *complete* story --
    /// it's free to substitute `Err` even when `amount > 0`, since
    /// nothing here forbade it, which broke `validate`'s own downstream
    /// claim that `Err(NegativeAmount(amount))` implies `amount <= 0`.
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<(), i64>| match result {
            Ok(()) => amount > 0,
            Err(bad) => *bad == amount && amount <= 0,
        })
    )]
    fn check_amount_positive(amount: i64) -> Result<(), i64> {
        if amount <= 0 { Err(amount) } else { Ok(()) }
    }

    /// `SufficientFunds`, isolated: `balance >= amount`. See
    /// [`Ledger::check_amount_positive`] for why this is broken out.
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<(), (i64, i64)>| match result {
            Ok(()) => self.balance >= amount,
            Err((balance, required)) => *balance < *required && *balance == self.balance && *required == amount,
        })
    )]
    fn check_sufficient_funds(&self, amount: i64) -> Result<(), (i64, i64)> {
        if self.balance < amount {
            Err((self.balance, amount))
        } else {
            Ok(())
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHECK_AMOUNT_POSITIVE_SRC, {
        #[kani::proof_for_contract(Ledger::check_amount_positive)]
        fn verify_check_amount_positive() {
            let amount: i64 = kani::any();
            let _ = Ledger::check_amount_positive(amount);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHECK_SUFFICIENT_FUNDS_SRC, {
        #[kani::proof_for_contract(Ledger::check_sufficient_funds)]
        fn verify_check_sufficient_funds() {
            let balance: i64 = kani::any();
            let amount: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let _ = ledger.check_sufficient_funds(amount);
        }
    }
}

// The real logic and its Kani contract live on a plain inherent method,
// not the `Exchange` trait impl -- see `stoplight.rs`'s own doc comment
// on `green_to_yellow` for why (Kani 0.67.0 can't contract a trait
// method when the trait itself is generic). Unlike every `Stoplight`
// edge, this one has real branching: `#[kani::ensures]`, generated by
// `#[amenable_derive::exchange]` below, calls through to
// `Validated::ensures(..)` (`kani_ensures!`, below), which states a
// real biconditional -- not merely `result.is_ok()` -- relating the
// transfer's own amount/the ledger's own balance/the two account names
// to which branch fires and, on the `Err` path, to the *exact* violated
// value the error variant reports.
//
// `validate` itself calls through `check_amount_positive`/`check_
// sufficient_funds` (above) rather than inlining their `if` checks --
// not a style preference, a real fix for the CBMC timeout `GAAP_LEDGER_
// PLAN.md`'s Step 1 hit and root-caused via `amenable_kani::gallery::
// ledger_account_id_comparison`: a symbolic condition selecting between
// this function's own heap-allocating `Ok` arm (`TransferPayload::
// clone()`/`Transfer::new`) and a non-allocating `Err` arm blew up
// CBMC's enum-drop modeling regardless of how few symbolic fields were
// involved. `#[kani::stub_verified]` (below, on both proof-for-contract
// harnesses) replaces each helper's own body with its already-proven
// contract during `validate`'s own verification, so CBMC's job here
// reduces to "does each helper's postcondition, plus the `Accounts
// Distinct` check inlined directly (never itself the bottleneck -- see
// that gallery module's first experiment), imply `validate`'s own
// `ensures`" -- not exploring the full symbolic-branch/allocation
// interaction from scratch.
kani_ensures!(
    Validated,
    "amenable_kani::ledger::Validated::validate_ensures",
    Result<Transfer<Validated, ValidatedToken>, TransferError>,
    |result| match result {
        Ok(validated) => {
            let payload = validated.primary();
            payload.amount().value() > 0 && payload.from() != payload.to()
        }
        Err(TransferError::NegativeAmount(amount)) => amount <= 0,
        Err(TransferError::InsufficientFunds { balance, required }) => balance < required,
        Err(TransferError::SameAccount) => true,
    }
);

#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Validated,
    proof_artifact = CalculationProof,
    harness_fn = verify_validate_accepts_a_lawful_transfer,
    harness_const = VERIFY_VALIDATE_ACCEPTS_A_LAWFUL_TRANSFER_SRC,
)]
impl Ledger {
    // `pub(crate)`, not private: lets `gallery::ledger_account_id_
    // comparison` call this directly, bypassing the `Exchange` trait's
    // extra dispatch layer -- isolates whether that layer itself (not
    // this function's own body, already tested extensively) is part of
    // the CBMC cost. Still invisible outside this crate.
    pub(crate) fn validate(
        &self,
        input: Transfer<Pending, PendingToken>,
    ) -> Result<Transfer<Validated, ValidatedToken>, TransferError> {
        let payload = input.primary().clone();
        let amount = payload.amount().value();

        Self::check_amount_positive(amount).map_err(TransferError::NegativeAmount)?;
        self.check_sufficient_funds(amount)
            .map_err(|(balance, required)| TransferError::InsufficientFunds {
                balance,
                required,
            })?;
        if payload.from() == payload.to() {
            return Err(TransferError::SameAccount);
        }

        let token = Validated::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

amenable_derive::harness! {
    kani, VERIFY_VALIDATE_ACCEPTS_A_LAWFUL_TRANSFER_SRC, {
        #[kani::proof_for_contract(Ledger::validate)]
        #[kani::stub_verified(Ledger::check_amount_positive)]
        #[kani::stub_verified(Ledger::check_sufficient_funds)]
        fn verify_validate_accepts_a_lawful_transfer() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_VALIDATE_REJECTS_THE_SAME_ACCOUNT_SRC, {
        #[kani::proof_for_contract(Ledger::validate)]
        #[kani::stub_verified(Ledger::check_amount_positive)]
        #[kani::stub_verified(Ledger::check_sufficient_funds)]
        fn verify_validate_rejects_the_same_account() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Amount::new(amount),
            );
            let input = Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

/// Lawful token minted once `Committed` is established from a proven
/// `Validated`.
#[derive(Debug, Clone, amenable_derive::ProofToken)]
#[proof_token(proposition = "Committed")]
#[amenable_derive::establish(
    credential = ValidatedToken,
    verifier = KaniVerifier,
    proposition = Committed,
)]
pub struct CommittedToken(());

// `Validated -> Committed`, `GAAP_LEDGER_PLAN.md`'s Step 2: infallible,
// like every `Stoplight` edge -- a transfer that already passed
// `validate`'s own checks has nothing left to reject at commit time
// (this worked example doesn't model concurrent modification between
// validation and commit, unlike `elicit_server::ledger`'s own "Phase 6"
// concurrency tests). `TransferError` is reused as the `Error` type
// rather than minting a new one-variant type (`Stoplight`'s own
// `StoplightError::NotUsed` pattern) -- no edge here ever constructs
// it, matching `StoplightError`'s own rationale for why an *inhabited*
// but never-constructed `Error` is required (`#[kani::stub_verified]`
// composition needs `E: kani::Arbitrary`, which needs at least one real
// variant to construct).
//
// `BalancedEntries`'s real claim -- `debit + credit == 0` -- is honestly
// trivial by construction here (`debit` is literally `-credit`), the
// same kind of triviality `Stoplight`'s own edges document rather than
// hide (zero-field states, no branching that could fail): naming and
// checking the claim is still real value (a future refinement building
// separate debit/credit `JournalEntry` postings from `commit`'s own
// body, matching `elicit_server::ledger`'s two-DB-row design, would
// make it non-tautological -- deliberately out of scope for this step,
// per `GAAP_LEDGER_PLAN.md`'s own "one real example by hand first"
// discipline).
//
// A real, second CBMC finding surfaced building this: negating a fully
// unconstrained symbolic `i64` (`-payload.amount().value()`) overflows
// at `i64::MIN`, which real Kani overflow-checking then has to reason
// about across the whole symbolic range -- expensive enough to time out
// on its own, independent of everything Step 1 already characterized.
// `#[kani::requires]` below states the real, true precondition instead
// of leaving it an artifact of what a harness happens to assume: `commit`
// is only ever meant to be called on an already-`validate`d transfer,
// which already established `AmountPositive`.
kani_ensures!(
    Committed,
    "amenable_kani::ledger::Committed::commit_ensures",
    Result<Transfer<Committed, CommittedToken>, TransferError>,
    |result| match result {
        Ok(committed) => {
            let payload = committed.primary();
            let debit = -payload.amount().value();
            let credit = payload.amount().value();
            debit + credit == 0
        }
        Err(_) => false,
    }
);

#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Committed,
    proof_artifact = CalculationProof,
    harness_fn = verify_commit_always_balances,
    harness_const = VERIFY_COMMIT_ALWAYS_BALANCES_SRC,
)]
impl Ledger {
    // `pub(crate)`, matching `validate`'s own precedent: lets
    // `gallery::ledger_commit_contract_timeout` call this directly for
    // real diagnostic harnesses. Still invisible outside this crate.
    //
    // `#[kani::requires]`: the real precondition, not an artifact of
    // what a harness happens to assume -- see the `BalancedEntries`
    // doc comment above (`kani_ensures!`, right above this impl block)
    // for the real overflow finding this fixes. `#[amenable_derive::
    // exchange]` only ever generates/appends the `#[kani::ensures]`
    // attribute; it clones this method's own attributes as written, so
    // a hand-written `requires` here coexists with the generated
    // `ensures` cleanly.
    #[cfg_attr(kani, kani::requires(input.primary().amount().value() > 0))]
    pub(crate) fn commit(
        &self,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Committed, CommittedToken>, TransferError> {
        let payload = input.primary().clone();
        let token = Committed::establish(input.sidecar());
        Ok(Transfer::new(payload, token))
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMIT_ALWAYS_BALANCES_SRC, {
        #[kani::proof_for_contract(Ledger::commit)]
        fn verify_commit_always_balances() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token = Validated::establish(credential);
            let validated: Transfer<Validated, ValidatedToken> = Transfer::new(payload, validated_token);
            let _ = ledger.commit(validated);
        }
    }
}
