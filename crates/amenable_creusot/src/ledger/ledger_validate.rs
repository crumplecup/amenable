/// The `#[cfg(creusot)]` imports, `TransferError`, `Ledger`, and `impl
/// Ledger` this file needs, consolidated into one gate on this `mod`
/// instead of one per item -- see `stoplight::mirror`'s own doc comment
/// for the general rationale. `Ledger`/`TransferError` are re-exported
/// `pub`, not `pub(super)`: `validated_committed_rejected.rs`'s own
/// mirror (a sibling module under `ledger/`, not a descendant of this
/// one) imports both by the same `super::ledger_validate::{Ledger,
/// TransferError, ..}` path it always has, and a re-export can never be
/// more visible than what it re-exports -- matching the same real
/// constraint `stoplight::mirror`'s own split re-export hit first.
/// `ensures`/`requires`/`sufficient_funds_holds` stay a separate, private
/// bridging `use`: the `harness! { .. }` blocks below (unconditional, at
/// this file's own top level) need them too, but nothing outside this
/// file ever does.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};

    pub(super) use super::super::contract_bounds::sufficient_funds_holds;
    use super::{
        check_amount_positive_result_holds, check_sufficient_funds_result_holds,
        insufficient_funds_holds, negative_amount_holds,
    };

    /// Sanitized mirror of `amenable_kani::ledger::TransferError` -- a
    /// *different*, concrete type from this file's own `TransferOutcome`
    /// (above), which backs the pre-existing isolated `validate` proof
    /// function's own Pearlite claim, unrelated to and unaffected by this
    /// mirror. Real captured bodies below construct `TransferError::
    /// NegativeAmount(..)`/`::InsufficientFunds { .. }`/`::SameAccount`
    /// directly, by name, so the type has to be named exactly this, with
    /// exactly these variants -- no `PartialEq`/`Eq` derive needed, matching
    /// `TransferOutcome`'s own precedent (pattern matching, not `==`).
    #[derive(Clone, Copy)]
    pub enum TransferError {
        NegativeAmount(i64),
        InsufficientFunds { balance: i64, required: i64 },
        SameAccount,
    }

    /// Sanitized mirror of `amenable_kani::ledger::Ledger` -- the real
    /// captured `validate`/`commit` bodies below call `Self::check_amount_
    /// positive`/`self.check_sufficient_funds`/`Self::negative_amount`/
    /// `Self::insufficient_funds`, all as real methods on a real receiver
    /// (the first captured Creusot body to reference `self`/`Self` at all --
    /// `amenable::creusot_export`'s own generator needed a real `&self`
    /// wrapper added for exactly this, `GAAP_LEDGER_PLAN.md`'s Step 6). A
    /// *different*, concrete type from this file's own bare `check_amount_
    /// positive`/`check_sufficient_funds` free functions (above), which back
    /// the pre-existing isolated `validate` proof, unrelated to and
    /// unaffected by this mirror -- both exist in the same module without
    /// conflict, since `Ledger::check_amount_positive`/`self.check_
    /// sufficient_funds` and the bare free-function calls resolve through
    /// entirely different paths.
    pub struct Ledger {
        pub(crate) balance: i64,
    }

    impl Ledger {
        /// `<V>`: unconstrained, no bound, never referenced in this body --
        /// exists purely so `validate`'s own real captured body (`generated/
        /// validate.rs`, `Self::check_amount_positive::<V>(amount)`) has a
        /// generic parameter to name. `Ledger::check_amount_positive`'s real
        /// counterpart in `amenable_gaap` is generic over `V: amenable_core::
        /// Verifier` with a real `Ensures<V>` bound (`GAAP_LEDGER_PLAN.md`'s
        /// Step 7) -- Creusot's own `Ensures<CreusotVerifier>` impl has a
        /// structurally different shape (`Input = ()`, `Bound = &'static
        /// str`, purely descriptive text; the real checking happens through
        /// `amount_positive_holds` directly, never through this trait, see
        /// this module's own doc comment), so mirroring the real bound here
        /// would be a type mismatch, not a stronger proof -- an unconstrained
        /// `V` is the honest mirror of "the real call needs some type in this
        /// position," nothing more.
        #[requires(true)]
        #[ensures(check_amount_positive_result_holds(amount, result))]
        pub(crate) fn check_amount_positive<V>(amount: i64) -> Result<(), i64> {
            if amount > 0 { Ok(()) } else { Err(amount) }
        }

        /// `<V>`: see [`Ledger::check_amount_positive`]'s own doc comment.
        #[requires(true)]
        #[ensures(check_sufficient_funds_result_holds(self.balance, amount, result))]
        pub(crate) fn check_sufficient_funds<V>(&self, amount: i64) -> Result<(), (i64, i64)> {
            if self.balance < amount {
                Err((self.balance, amount))
            } else {
                Ok(())
            }
        }

        /// Matches `amenable_kani::ledger::Ledger::negative_amount`'s own
        /// doc comment: a real function, not the bare `TransferError::
        /// NegativeAmount` tuple-variant constructor or a destructuring
        /// closure -- unlike Verus, Creusot places no such restriction on
        /// either form, but the real captured body already calls through
        /// this real method (the Kani source both backends capture from is
        /// the same, single source of truth), so this mirrors it exactly
        /// rather than inlining an equivalent that would silently diverge
        /// from what's actually captured.
        #[requires(true)]
        #[ensures(negative_amount_holds(bad, result))]
        pub(crate) fn negative_amount(bad: i64) -> TransferError {
            TransferError::NegativeAmount(bad)
        }

        #[requires(true)]
        #[ensures(insufficient_funds_holds(bad, result))]
        pub(crate) fn insufficient_funds(bad: (i64, i64)) -> TransferError {
            TransferError::InsufficientFunds {
                balance: bad.0,
                required: bad.1,
            }
        }
    }
}
#[cfg(creusot)]
pub use mirror::{Ledger, TransferError};
#[cfg(creusot)]
use mirror::{ensures, logic, requires, sufficient_funds_holds};

amenable_derive::harness! {
    creusot, CHECK_AMOUNT_POSITIVE_RESULT_HOLDS_SRC, {
        /// `Ledger::check_amount_positive`'s whole-`Result` postcondition,
        /// named once rather than restated inline in its own
        /// `#[ensures(..)]` clause -- composes `amount_positive_holds`
        /// (already named) with the `Err` arm's own extra `bad == amount`
        /// conjunct.
        #[logic(open)]
        pub(crate) fn check_amount_positive_result_holds(amount: i64, outcome: Result<(), i64>) -> bool {
            pearlite! {
                match outcome {
                    Ok(()) => amount_positive_holds(amount, true),
                    Err(bad) => bad == amount && amount_positive_holds(amount, false),
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::Ledger::check_amount_positive",
        "creusot",
        "ensures",
        || CHECK_AMOUNT_POSITIVE_RESULT_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, CHECK_SUFFICIENT_FUNDS_RESULT_HOLDS_SRC, {
        /// `Ledger::check_sufficient_funds`'s whole-`Result` postcondition
        /// -- same reasoning as `check_amount_positive_result_holds`,
        /// composing `sufficient_funds_holds` with the `Err` arm's own
        /// extra conjuncts. Takes `balance` as a plain argument rather
        /// than `&self`: a `#[logic(open)] fn` at module scope has no
        /// receiver to name, matching `sufficient_funds_holds`'s own
        /// free-function shape above.
        #[logic(open)]
        pub(crate) fn check_sufficient_funds_result_holds(
            balance: i64,
            amount: i64,
            outcome: Result<(), (i64, i64)>,
        ) -> bool {
            pearlite! {
                match outcome {
                    Ok(()) => sufficient_funds_holds(balance, amount, true),
                    Err((observed_balance, required)) => {
                        observed_balance == balance
                            && required == amount
                            && sufficient_funds_holds(balance, amount, false)
                    }
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::Ledger::check_sufficient_funds",
        "creusot",
        "ensures",
        || CHECK_SUFFICIENT_FUNDS_RESULT_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, NEGATIVE_AMOUNT_HOLDS_SRC, {
        /// `Ledger::negative_amount`'s postcondition: the constructed
        /// `TransferError` is exactly the `NegativeAmount` variant
        /// wrapping the given value, never any other variant.
        #[logic(open)]
        pub(crate) fn negative_amount_holds(bad: i64, outcome: TransferError) -> bool {
            pearlite! {
                match outcome {
                    TransferError::NegativeAmount(actual) => actual == bad,
                    _ => false,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::Ledger::negative_amount",
        "creusot",
        "ensures",
        || NEGATIVE_AMOUNT_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, INSUFFICIENT_FUNDS_HOLDS_SRC, {
        /// `Ledger::insufficient_funds`'s postcondition: the constructed
        /// `TransferError` is exactly the `InsufficientFunds` variant
        /// carrying the given `(balance, required)` pair, never any other
        /// variant.
        #[logic(open)]
        pub(crate) fn insufficient_funds_holds(bad: (i64, i64), outcome: TransferError) -> bool {
            pearlite! {
                match outcome {
                    TransferError::InsufficientFunds { balance, required } => {
                        balance == bad.0 && required == bad.1
                    }
                    _ => false,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::Ledger::insufficient_funds",
        "creusot",
        "ensures",
        || INSUFFICIENT_FUNDS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, AMOUNT_POSITIVE_HOLDS_SRC, {
        /// The `Validated` postcondition's `AmountPositive` conjunct —
        /// real, callable Pearlite content, named and reused rather
        /// than restated wherever it's checked.
        #[logic(open)]
        pub fn amount_positive_holds(amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (amount@ > 0) }
        }
    }
}

// Two call shapes reuse this same fn: `check_amount_positive`'s own
// `#[ensures(amount_positive_holds(amount, result))]` below, and
// `check_commit_balances`'s own `#[requires(amount_positive_holds(amount,
// true))]` further down -- separate `ContractRecord`s since Kani's
// `(verifier, kind)` lookup (mirrored here for Creusot/Verus by
// `cordial`'s own scanner) is keyed separately for `"ensures"` vs
// `"requires"` clauses.
#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::amount_positive_holds",
        "creusot",
        "ensures",
        || AMOUNT_POSITIVE_HOLDS_SRC,
    )
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::amount_positive_holds",
        "creusot",
        "requires",
        || AMOUNT_POSITIVE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_AMOUNT_POSITIVE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_amount_positive`'s
        /// own claim, isolated the same way that function is isolated on
        /// the Kani side.
        #[requires(true)]
        #[ensures(amount_positive_holds(amount, result))]
        pub(crate) fn check_amount_positive(amount: i64) -> bool {
            amount > 0
        }
    }
}
