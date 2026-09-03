//! Real `Transfer<S, Token>` / `TransferError` / `Ledger` types and logic --
//! `GAAP_LEDGER_PLAN.md`'s Step 9: the real type *and* its real logic live
//! here, and each backend attaches its own proof separately rather than
//! needing its own copy of the struct or the logic.
//!
//! Split into `types` (the `Transfer` carrier and `TransferError`) and
//! `machine` (the `Ledger` state machine plus its `validate`/`commit`/
//! `reject`/`rollback` edges).

mod machine;
mod types;

pub use machine::Ledger;
pub use types::{Transfer, TransferError};
