use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::Evidence;
// `#[cfg(verus_keep_ghost)]`-gated, matching `amenable_core::evidence`'s
// own precedent: `AmountPositive::ensures(..)` (etc., below) resolves
// fine under ordinary `cargo check`/clippy without this import (`Type::
// trait_fn()` path syntax doesn't require the trait in scope the way
// `.method()` calls do), but real `verus`'s own driver -- which
// unconditionally sets `--cfg verus_keep_ghost` -- needs it, confirmed
// against the real toolchain: a real "function or associated item
// `ensures` not found" error without it.
#[cfg(verus_keep_ghost)]
use crate::Ensures;

verus! {

/// Sanitized mirror of `amenable_gaap::Amount` -- the real captured body
/// calls `.amount().value()`, so this needs the same two-method chain,
/// not just a bare `i64`.
#[derive(Clone, Copy)]
pub struct Amount(pub i64);

impl Amount {
    /// `ensures`, same real reason as every other accessor in this
    /// module: ordinary modular verification only exposes what a
    /// function's own `ensures` promises to its callers, so without
    /// this, nothing downstream could learn `.value()`'s actual result
    /// from `self`'s own field.
    pub fn value(&self) -> (result: i64)
        ensures
            result == self.0,
    {
        self.0
    }
}

/// Sanitized mirror of `amenable_gaap::TransferPayload` -- see this
/// file's own doc comment for why `from`/`to` are bare `u64`, not a
/// wrapping `AccountId`. `Clone`, not `Copy`, even though nothing here
/// stops it from being `Copy`: the real `TransferPayload` genuinely
/// isn't (its `from`/`to` are a `String`-backed `Account`, not the bare
/// `Copy` `AccountId` identity alone), and the real captured body's
/// own `.clone()` call needs a `Clone` type to land on -- deriving
/// `Copy` here too would make that same, legitimate `.clone()` trip a
/// real, un-`#[allow]`-able `clippy::clone_on_copy` failure.
pub struct TransferPayload {
    /// The paying account's identity.
    pub from: u64,
    /// The receiving account's identity.
    pub to: u64,
    /// The transfer amount.
    pub amount: Amount,
}

impl Clone for TransferPayload {
    /// Same real reason every accessor in this file carries an `ensures`:
    /// a hand-written impl, not `#[derive(Clone)]` -- Verus does not
    /// (yet) support adding a specification to an autoderived `Clone`
    /// impl when the type isn't also `Copy` (confirmed against the real
    /// toolchain: a real, if benign, compiler warning without this),
    /// leaving the real captured body's `.clone()` call opaque to any
    /// caller that needs to know the clone's fields match the original.
    fn clone(&self) -> (result: Self)
        ensures
            result.from == self.from,
            result.to == self.to,
            result.amount.0 == self.amount.0,
    {
        Self {
            from: self.from,
            to: self.to,
            amount: self.amount,
        }
    }
}

impl TransferPayload {
    /// `ensures` throughout this impl: same real reason as [`Amount::
    /// value`].
    pub fn from(&self) -> (result: u64)
        ensures
            result == self.from,
    {
        self.from
    }

    /// `ensures`: same real reason as [`Amount::value`].
    pub fn to(&self) -> (result: u64)
        ensures
            result == self.to,
    {
        self.to
    }

    /// `ensures`: same real reason as [`Amount::value`].
    pub fn amount(&self) -> (result: Amount)
        ensures
            result.0 == self.amount.0,
    {
        self.amount
    }
}

/// Matches the real `amenable_gaap::TransferPayload`'s own `Evidence`
/// impl -- required for `Sidecar::Primary: Evidence`, below. No
/// `#[derive(Default)]`/`Self::default()` here: a `derive`d impl expands
/// *outside* the `verus! {}` block's own processing (derive macros run
/// before Verus ever sees the tokens), so Verus treats it as an
/// "external" function no in-block body can call -- confirmed against
/// the real toolchain, not assumed. A literal struct constructor sidesteps
/// the question entirely.
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

/// Sanitized mirror of `amenable_kani::ledger::TransferError`.
#[derive(Debug, Clone, Copy)]
pub enum TransferError {
    /// The transfer amount wasn't positive.
    NegativeAmount(i64),
    /// The paying account's balance can't cover the transfer.
    InsufficientFunds {
        /// The paying account's actual balance.
        balance: i64,
        /// The amount that was required.
        required: i64,
    },
    /// The paying and receiving accounts were the same.
    SameAccount,
}

/// Sanitized mirror of `amenable_kani::ledger::Ledger` -- the real
/// captured `validate`/`commit` bodies below call `self.check_
/// sufficient_funds(..)`/`Self::check_amount_positive(..)`, so this
/// needs the same two helper methods, each carrying the real
/// `ensures` clause `amenable_kani::ledger::Ledger::check_amount_
/// positive`/`::check_sufficient_funds` separately prove on the Kani
/// side and `amenable_creusot::ledger::check_amount_positive`/
/// `::check_sufficient_funds` separately prove on the Creusot side --
/// a real, hand-written claim, not generated: `verus_exchange!`'s
/// generated `Exchange::exchange` body only ever gets to use what a
/// called function's own `ensures` promises, never its actual body
/// (ordinary modular verification), so without these, `validate`'s
/// own `ensures` (`validated_ensures_spec`, below) could not be
/// proven at all.
pub struct Ledger {
    /// The ledger's current balance.
    pub balance: i64,
}

impl Ledger {
    /// Builds a ledger starting at `balance`.
    pub fn new(balance: i64) -> Self {
        Self { balance }
    }

    /// `<V>`: unconstrained, no bound -- exists purely so `validate`'s
    /// own real captured body (`generated/ledger_exchange/validate.rs`,
    /// `Self::check_amount_positive::<V>(amount)`) has a generic
    /// parameter to name. Matches `amenable_creusot::ledger::Ledger::
    /// check_amount_positive`'s own identical fix and identical
    /// reasoning (`GAAP_LEDGER_PLAN.md`'s Step 7): the real counterpart's
    /// own `Ensures<V>` bound doesn't translate to this mirror at all
    /// (there is no separate isolated `Ensures<GalleryVerifier>` impl
    /// this helper's own postcondition needs to route through -- the
    /// `ensures` clause states the claim directly). The body's own
    /// `let _phantom: PhantomData<V> = ..` line has no verification
    /// content -- it exists only so `V` is genuinely used, not just
    /// declared, matching a real clippy lint this project can't
    /// `#[allow]` away (`clippy::extra_unused_type_parameters`).
    pub fn check_amount_positive<V>(amount: i64) -> (result: Result<(), i64>)
        ensures
            match result {
                Ok(()) => amount > 0,
                Err(bad) => bad == amount && amount <= 0,
            },
    {
        let _phantom: core::marker::PhantomData<V> = core::marker::PhantomData;
        if amount <= 0 { Err(amount) } else { Ok(()) }
    }

    /// `<V>`: see [`Ledger::check_amount_positive`]'s own doc comment.
    pub fn check_sufficient_funds<V>(&self, amount: i64) -> (result: Result<(), (i64, i64)>)
        ensures
            match result {
                Ok(()) => self.balance >= amount,
                Err((balance, required)) => balance < required && balance == self.balance && required == amount,
            },
    {
        let _phantom: core::marker::PhantomData<V> = core::marker::PhantomData;
        if self.balance < amount { Err((self.balance, amount)) } else { Ok(()) }
    }

    /// Matches `amenable_kani::ledger::Ledger::negative_amount`'s own
    /// doc comment: the real captured `validate` body below calls
    /// `Self::negative_amount` rather than the bare `TransferError::
    /// NegativeAmount` tuple-variant constructor, since Verus does not
    /// (yet) support "using a datatype constructor as a function value"
    /// (confirmed against the real toolchain). `ensures`, unlike the
    /// real Kani copy (plain `fn`, no Verus-specific syntax at all): this
    /// gets called through `Result::map_err`, whose own `vstd` spec
    /// states its postcondition purely in terms of `op.ensures(..)` --
    /// with no `ensures` declared here, Verus knows nothing at all about
    /// what `map_err`'s own result actually contains, and `validate`'s
    /// own postcondition could not be proven at its `Err` exit points
    /// (confirmed the hard way: a real `postcondition not satisfied`
    /// failure pointing at exactly this `?` exit, before this was added).
    pub fn negative_amount(bad: i64) -> (result: TransferError)
        ensures
            result == TransferError::NegativeAmount(bad),
    {
        TransferError::NegativeAmount(bad)
    }

    /// Matches `amenable_kani::ledger::Ledger::insufficient_funds`'s own
    /// doc comment: the real captured `validate` body calls `Self::
    /// insufficient_funds` rather than a destructuring closure (`|
    /// (balance, required)| ..`), since Verus does not (yet) support a
    /// general pattern in a closure parameter position either. `ensures`
    /// for the same real reason `negative_amount`'s own doc comment
    /// explains.
    pub fn insufficient_funds(bad: (i64, i64)) -> (result: TransferError)
        ensures
            result == (TransferError::InsufficientFunds { balance: bad.0, required: bad.1 }),
    {
        TransferError::InsufficientFunds {
            balance: bad.0,
            required: bad.1,
        }
    }
}

} // verus!
