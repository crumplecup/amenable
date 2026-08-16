//! The five real, provable claims this worked example exists to prove —
//! not citation-only `Standard` placeholders. Each becomes a real
//! `Requires<V>`/`Ensures<V>` impl per verifier as the later steps of
//! `GAAP_LEDGER_PLAN.md` land (`Requires<V>`/`Ensures<V>` both require
//! `Self: Witness<V>`, so none of that can exist yet); this file only
//! establishes the bare `Evidence` shape each one needs first.
//!
//! `#[amenable_derive::evidence]`, not `#[derive(Standard)]`: these are
//! claims that get *proven*, not asserted-and-audited roots, so they
//! don't carry a `Provenance` record the way `transfer::Pending` and its
//! sibling states do — see `GAAP_LEDGER_PLAN.md`'s own "Motivation"
//! section for the correction this distinction came from (elicitation's
//! `Prop` maps to `Standard` only for its citation-only, unprovable
//! members; these five have real proof bodies instead).

use amenable_core::Evidence;

/// The transfer amount is positive (`amount > 0`).
///
/// Source: pre-ASC foundational ledger invariant — amounts must be
/// non-zero and non-negative. Precondition on the `Pending -> Validated`
/// edge (`GAAP_LEDGER_PLAN.md`'s Step 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AmountPositive;

#[amenable_derive::evidence]
impl Evidence for AmountPositive {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

/// The source account holds at least the transfer amount
/// (`balance >= amount`).
///
/// Source: pre-ASC foundational ledger invariant — no overdraft without
/// an explicit credit facility. The first genuinely branching,
/// data-dependent predicate this gallery proves: unlike `Stoplight`'s
/// `result.is_ok()`, this can fail for a real, non-degenerate reason.
/// Precondition on the `Pending -> Validated` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SufficientFunds;

#[amenable_derive::evidence]
impl Evidence for SufficientFunds {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

/// The source and destination accounts are distinct (`from != to`).
///
/// Source: ASC 230 — Statement of Cash Flows; gross vs. net
/// presentation. Precondition on the `Pending -> Validated` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccountsDistinct;

#[amenable_derive::evidence]
impl Evidence for AccountsDistinct {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

/// The debit entry and credit entry balance (`debit + credit == 0`).
///
/// Source: pre-ASC foundational double-entry requirement. Postcondition
/// on the `Validated -> Committed` edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BalancedEntries;

#[amenable_derive::evidence]
impl Evidence for BalancedEntries {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}

/// Assets = Liabilities + Stockholders' Equity, at every period-end.
///
/// Source: double-entry bookkeeping foundational equation; ASC 210 —
/// Balance Sheet. The one multi-variable arithmetic identity in the
/// initial contract set — where it attaches (`Committed` directly, or a
/// separate period-close state) is an open question, deferred to Step 2
/// of `GAAP_LEDGER_PLAN.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AccountingEquationHolds;

#[amenable_derive::evidence]
impl Evidence for AccountingEquationHolds {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) {}
}
