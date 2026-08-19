//! Compile-only confirmation that `amenable_core::State<V>`'s blanket
//! impl covers every real state type in the tree for free -- per
//! `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 0, no new impl work
//! anywhere. `assert_state`'s body is empty on purpose: the check *is*
//! the trait-bound resolution at each call site below, the same
//! "compiler squawks, not the macro" discipline the rest of that plan
//! uses -- if any of these types stopped satisfying `Evidence +
//! Witness<KaniVerifier>`, this file would fail to compile, not fail at
//! runtime.

use amenable_core::{Evidence, Green, Red, State, Verifier, Yellow};
use amenable_gaap::{
    AccountsDistinct, AmountPositive, BalancedEntries, Committed, Pending, Rejected,
    SufficientFunds, Validated,
};
use amenable_kani::KaniVerifier;

fn assert_state<V: Verifier, T: State<V>>() {}

#[test]
fn every_real_stoplight_and_ledger_state_satisfies_state_for_free() {
    assert_state::<KaniVerifier, Green>();
    assert_state::<KaniVerifier, Yellow>();
    assert_state::<KaniVerifier, Red>();

    assert_state::<KaniVerifier, Pending>();
    assert_state::<KaniVerifier, Validated>();
    assert_state::<KaniVerifier, Committed>();
    assert_state::<KaniVerifier, Rejected<Pending>>();
    assert_state::<KaniVerifier, Rejected<Validated>>();

    assert_state::<KaniVerifier, AmountPositive>();
    assert_state::<KaniVerifier, SufficientFunds>();
    assert_state::<KaniVerifier, AccountsDistinct>();
    assert_state::<KaniVerifier, BalancedEntries>();
}

#[test]
fn state_facade_projects_real_evidence_data() {
    let green = Green;

    assert_eq!(
        <Green as State<KaniVerifier>>::type_name(&green),
        std::any::type_name::<Green>()
    );
    assert_eq!(
        <Green as State<KaniVerifier>>::is_root(&green),
        <Green as Evidence>::is_root()
    );
}
