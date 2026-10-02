//! `GAAP_LEDGER_PLAN.md`'s Step 7: `Ledger`/`Transfer`/`TransferError` and
//! every one of `Ledger`'s real methods now live in `amenable_gaap`, with
//! Kani contracts attached directly there (see `amenable_gaap::ledger`'s
//! own doc comment for the confirmed "direct contract, no delegating
//! wrapper" pattern, and `gaap_ledger.rs` for the real harnesses proving
//! each method). This module keeps only what's genuinely Kani-specific
//! and can't move: each atomic contract type's own `Ensures<KaniVerifier>`
//! impl in [`atomic_claims`], and `Pending`'s own trivial
//! `Witness<KaniVerifier>` impl in [`pending`]. The `KaniCompose` impls
//! for these same domain types (another Kani-only concern) live in
//! [`mirror`]; the composite `validate`/`commit`/`reject`/`rollback` state
//! claims that call through these atomic contracts live in
//! [`state_claims`].
//!
//! `AccountId` is a bare `Uuid` identity (not the `String` it started
//! with in `GAAP_LEDGER_PLAN.md`'s Step 0, and not the combined id+name
//! struct it became after that: see `amenable_gaap::transfer::AccountId`'s
//! own doc comment for why it's now split from `Account`) precisely
//! because of a real CBMC cost the first version of this proof hit:
//! comparing two independently-constructed `String`s for equality
//! *inside a `#[kani::ensures]` closure* is expensive regardless of
//! content or length -- fully root-caused via `amenable_kani::gallery::
//! ledger_account_id_comparison`'s own investigation, which also
//! confirmed a *fixed-capacity* string (bounded buffer + a length
//! field) is exactly as expensive, so bounding the name wouldn't have
//! helped. `Uuid`'s 16-byte, fixed-length comparison is cheap in the
//! identical position.

mod atomic_claims;
#[cfg(kani)]
mod mirror;
mod pending;
mod state_claims;
