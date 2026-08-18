//! The five real, provable claims this worked example exists to prove —
//! not citation-only `Standard` placeholders. Four of five now have a
//! real `Ensures<KaniVerifier>` impl (`amenable_kani::ledger`, since
//! `Requires<V>`/`Ensures<V>` both require `Self: Witness<V>`, which has
//! to live wherever the verifier marker does — orphan rules, the same
//! reason `Witness<KaniVerifier>` impls for `Pending`/`Validated`/
//! `Committed` live there too, not here): `AmountPositive`/
//! `SufficientFunds`/`AccountsDistinct`/`BalancedEntries` back
//! `Ledger::check_amount_positive`'s/`::check_sufficient_funds`'s own
//! DFCC contracts and `Validated`'s/`Committed`'s combined claims —
//! real code calling through these types now, closing a real gap where
//! these five sat as dead code workspace-wide since Step 0, referenced
//! only by doc comments that used their names without ever importing
//! them. `AccountingEquationHolds` and the Creusot/Verus `Ensures<V>`
//! impls for the other four are still open (`GAAP_LEDGER_PLAN.md`'s own
//! "Next step"). This file only establishes the bare `Evidence` shape
//! each one needs — the verifier-specific `Ensures<V>` impls live with
//! their own verifier markers, per orphan rules.
//!
//! `#[derive(Evidence)]`, not `#[derive(Standard)]`: these are claims
//! that get *proven*, not asserted-and-audited roots, so they don't
//! carry a `Provenance` record the way `transfer::Pending` and its
//! sibling states do — see `GAAP_LEDGER_PLAN.md`'s own "Motivation"
//! section for the correction this distinction came from (elicitation's
//! `Prop` maps to `Standard` only for its citation-only, unprovable
//! members; these five have real proof bodies instead). All five are
//! the identical trivial shape `#[derive(Evidence)] #[evidence(basis =
//! "Self")]` exists to collapse: no hand-written `impl Evidence` block,
//! and (unlike the hand-written form these used to be) a real
//! `EvidenceLink` registration each, so all five are actually
//! discoverable through the evidence-chain registry now.

/// The transfer amount is positive (`amount > 0`).
///
/// Source: pre-ASC foundational ledger invariant — amounts must be
/// non-zero and non-negative. Precondition on the `Pending -> Validated`
/// edge (`GAAP_LEDGER_PLAN.md`'s Step 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct AmountPositive;

/// The source account holds at least the transfer amount
/// (`balance >= amount`).
///
/// Source: pre-ASC foundational ledger invariant — no overdraft without
/// an explicit credit facility. The first genuinely branching,
/// data-dependent predicate this gallery proves: unlike `Stoplight`'s
/// `result.is_ok()`, this can fail for a real, non-degenerate reason.
/// Precondition on the `Pending -> Validated` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct SufficientFunds;

/// The source and destination accounts are distinct (`from != to`).
///
/// Source: ASC 230 — Statement of Cash Flows; gross vs. net
/// presentation. Precondition on the `Pending -> Validated` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct AccountsDistinct;

/// The debit entry and credit entry balance (`debit + credit == 0`).
///
/// Source: pre-ASC foundational double-entry requirement. Postcondition
/// on the `Validated -> Committed` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct BalancedEntries;

/// Assets = Liabilities + Stockholders' Equity, at every period-end.
///
/// Source: double-entry bookkeeping foundational equation; ASC 210 —
/// Balance Sheet. The one multi-variable arithmetic identity in the
/// initial contract set — where it attaches (`Committed` directly, or a
/// separate period-close state) is an open question, deferred to Step 2
/// of `GAAP_LEDGER_PLAN.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct AccountingEquationHolds;
