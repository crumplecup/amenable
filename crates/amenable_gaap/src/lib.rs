//! GAAP ledger worked example: trait-interface and domain types for
//! double-entry bookkeeping, the next `Stoplight`-succeeding worked
//! example in the Exchange proof derivation lineage. See
//! `docs/GAAP_LEDGER_PLAN.md` for the full design rationale and the
//! crate-hierarchy decision (mirrors `amenable_std`'s own real,
//! asymmetric shape: Kani-side proofs will live in `amenable_kani::
//! ledger`, which depends on this crate, not the reverse).

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contracts;
mod transfer;

pub use contracts::{
    AccountingEquationHolds, AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds,
};
pub use transfer::{AccountId, Amount, Committed, Pending, Rejected, TransferPayload, Validated};
