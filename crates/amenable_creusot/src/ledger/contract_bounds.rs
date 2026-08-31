#[cfg(creusot)]
use amenable_core::{Establish, Evidence, Sidecar, Witness};
#[cfg(creusot)]
use amenable_gaap::{
    Committed, CommittedToken, Pending, PendingToken, Rejected, RejectedFromPendingToken,
    RejectedFromValidatedToken, Validated, ValidatedToken,
};
#[cfg(creusot)]
use creusot_std::macros::{ensures, extern_spec, logic, requires};
#[cfg(creusot)]
use creusot_std::std::ops::FnOnceExt;

use super::ledger_validate::{AMOUNT_POSITIVE_HOLDS_SRC, VERIFY_CHECK_AMOUNT_POSITIVE_SRC};
#[cfg(creusot)]
use super::ledger_validate::{amount_positive_holds, check_amount_positive};
#[cfg(creusot)]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use amenable_gaap::{AccountsDistinct, AmountPositive, SufficientFunds};
// Ties the real `amenable_gaap::AmountPositive` contract type to this
// module's own Pearlite content -- matching `amenable_kani::ledger`'s
// own `kani_ensures!` wiring for the identical type (`GAAP_LEDGER_
// PLAN.md`'s Step 5), previously done on Kani only. `Bound = &'static
// str`, not `bool`: unlike Kani, Pearlite predicates have no exec
// representation at all, so `amenable_core::contract`'s own doc comment
// already anticipates this fallback -- a description of the real bound
// for audit purposes, not a checked value (the actual checking is
// `amount_positive_holds` itself, called directly from Pearlite
// composition above and in `validated_holds` below; nothing in the
// translated proof ever calls through this trait). No `#[cfg(creusot)]`
// counterpart needed at all, unlike `Validated`/`Committed`'s own
// `Witness<CreusotVerifier>` impls: nothing establishes a token *from*
// `AmountPositive` (it's not part of an `Establish<C, V>` chain), so
// real Creusot translation never needs to see either impl.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AmountPositive {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof::new(vec![(
            "check_amount_positive".to_owned(),
            VERIFY_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
        )])
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AmountPositive {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        AMOUNT_POSITIVE_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, SUFFICIENT_FUNDS_HOLDS_SRC, {
        /// The `Validated` postcondition's `SufficientFunds` conjunct.
        #[logic(open)]
        pub fn sufficient_funds_holds(balance: i64, amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (balance@ >= amount@) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::sufficient_funds_holds",
        "creusot",
        "ensures",
        || SUFFICIENT_FUNDS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_SUFFICIENT_FUNDS_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_sufficient_funds`'s
        /// own claim.
        #[requires(true)]
        #[ensures(sufficient_funds_holds(balance, amount, result))]
        fn check_sufficient_funds(balance: i64, amount: i64) -> bool {
            balance >= amount
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for SufficientFunds {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof::new(vec![(
            "check_sufficient_funds".to_owned(),
            VERIFY_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
        )])
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for SufficientFunds {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        SUFFICIENT_FUNDS_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, ACCOUNTS_DISTINCT_HOLDS_SRC, {
        /// The `Validated` postcondition's `AccountsDistinct` conjunct.
        /// `from`/`to` are a sanitized `u64` mirror of `amenable_gaap::
        /// AccountId`'s real `Uuid`-backed identity — see this module's
        /// own doc comment for why.
        #[logic(open)]
        pub fn accounts_distinct_holds(from: u64, to: u64, outcome: bool) -> bool {
            pearlite! { outcome == (from != to) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::accounts_distinct_holds",
        "creusot",
        "ensures",
        || ACCOUNTS_DISTINCT_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own
        /// `payload.from() != payload.to()` check, isolated the same
        /// way `AmountPositive`/`SufficientFunds` are.
        #[requires(true)]
        #[ensures(accounts_distinct_holds(from, to, result))]
        fn check_accounts_distinct(from: u64, to: u64) -> bool {
            from != to
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AccountsDistinct {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof::new(vec![(
            "check_accounts_distinct".to_owned(),
            VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC.to_owned(),
        )])
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AccountsDistinct {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        ACCOUNTS_DISTINCT_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, TRANSFER_OUTCOME_MIRROR_SRC, {
        /// Sanitized mirror of `amenable_kani::ledger::TransferError`,
        /// for the combined claim below — real Rust enum, destructured
        /// via Pearlite `match`, which needs no `PartialEq`/`Eq` derive
        /// at all (pattern matching, not an `==` comparison) — dropped
        /// after a real translation error: `creusot-rustc` requires a
        /// `DeepModel` impl for any type deriving `PartialEq`, matching
        /// `CreusotVerifierMetadata`'s own precedent in this crate's
        /// `witness.rs`.
        enum TransferOutcome {
            Ok,
            NegativeAmount(i64),
            InsufficientFunds { balance: i64, required: i64 },
            SameAccount,
        }
    }
}

amenable_derive::harness! {
    creusot, VALIDATED_HOLDS_SRC, {
        /// The real, combined `Validated` postcondition — matches
        /// `amenable_kani::ledger::Validated::validate_ensures`'s exact
        /// match-arm shape (including the real claim's own asymmetry:
        /// the `Ok` arm doesn't restate `SufficientFunds`, only
        /// `AmountPositive`/`AccountsDistinct` — `SufficientFunds` gates
        /// whether `Ok` is reached at all, but isn't re-asserted once
        /// it has been), composing the four isolated conjuncts above
        /// rather than restating their content.
        #[logic(open)]
        fn validated_holds(amount: i64, from: u64, to: u64, outcome: TransferOutcome) -> bool {
            pearlite! {
                match outcome {
                    TransferOutcome::Ok => amount_positive_holds(amount, true) && accounts_distinct_holds(from, to, true),
                    TransferOutcome::NegativeAmount(bad) => bad == amount && amount@ <= 0,
                    TransferOutcome::InsufficientFunds { balance, required } => balance < required,
                    TransferOutcome::SameAccount => true,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::validated_holds",
        "creusot",
        "ensures",
        || VALIDATED_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_VALIDATE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own real
        /// body -- calls the same three isolated checks in the same
        /// order, short-circuiting the same way.
        #[requires(true)]
        #[ensures(validated_holds(amount, from, to, result))]
        fn validate(balance: i64, amount: i64, from: u64, to: u64) -> TransferOutcome {
            if !check_amount_positive(amount) {
                return TransferOutcome::NegativeAmount(amount);
            }
            if !check_sufficient_funds(balance, amount) {
                return TransferOutcome::InsufficientFunds { balance, required: amount };
            }
            if !check_accounts_distinct(from, to) {
                return TransferOutcome::SameAccount;
            }
            TransferOutcome::Ok
        }
    }
}
