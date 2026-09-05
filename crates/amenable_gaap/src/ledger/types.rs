//! The `Transfer<S, Token>` state-carrier struct and the `TransferError`
//! enum. `Ledger`'s state machine and edge methods live in `machine`.

use crate::{Pending, PendingToken, TransferPayload};

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
    /// (`GAAP_LEDGER_PLAN.md`'s Step 7) -- `#[cfg(kani)]` is the real
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
        Self::new(payload, PendingToken::new())
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
