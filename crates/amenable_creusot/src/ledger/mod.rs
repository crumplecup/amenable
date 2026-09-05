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
//! *local* item does (per this crate's own `rust_std_witness`'s own doc
//! comment) turned out not to generalize to an ordinary dependency
//! crate's own items, only to items local to the crate `cargo creusot`
//! actually translates.
//!
//! Split by the worked example's own narrative phases: [`transfer_and_types`]
//! (imports, `extern_spec!`s, `Transfer<S, Token>`, `Amount`,
//! `TransferPayload`, the `Pending` witness, `TransferError`),
//! [`ledger_validate`] (the `Ledger` struct, `impl Ledger`'s real
//! `check_*` method bodies, and the logic functions those bodies call),
//! [`contract_bounds`] (the `AmountPositive`/`SufficientFunds`/
//! `AccountsDistinct` contract-type wiring), and
//! [`validated_committed_rejected`] (`BalancedEntries`, the `Validated`/
//! `Committed`/`Rejected<T>` witnesses, and the `generated/*.rs`
//! `include!`s).

mod contract_bounds;
mod ledger_validate;
mod transfer_and_types;
mod validated_committed_rejected;

pub use contract_bounds::{
    ACCOUNTS_DISTINCT_HOLDS_SRC, SUFFICIENT_FUNDS_HOLDS_SRC, VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC,
    VERIFY_CHECK_SUFFICIENT_FUNDS_SRC,
};
pub use ledger_validate::{AMOUNT_POSITIVE_HOLDS_SRC, VERIFY_CHECK_AMOUNT_POSITIVE_SRC};
pub use validated_committed_rejected::{
    BALANCED_ENTRIES_HOLDS_SRC, VERIFY_CHECK_COMMIT_BALANCES_SRC,
};
