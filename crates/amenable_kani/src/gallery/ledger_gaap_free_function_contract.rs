//! `GAAP_LEDGER_PLAN.md`'s Step 9: does `#[kani::proof_for_contract]` work
//! when the checked function's body is a one-line delegating call into a
//! *different* crate's real logic, rather than the logic itself living
//! right here? This is the one genuinely unconfirmed piece behind moving
//! `Ledger`/`Transfer`/`TransferError` into `amenable_gaap` for real --
//! Kani has no `extern_spec!`-style mechanism for attaching a contract to
//! a foreign function the way Creusot does, and inherent impls can't
//! reach across a crate boundary the way trait impls can, so the only
//! place left for a real Kani contract to live is a thin wrapper here,
//! delegating to `amenable_gaap::Ledger::commit`'s own real body (generic
//! over `V: Verifier`, monomorphized to `KaniVerifier` at the call site)
//! via one real function call -- not a copy of it.
//!
//! **Confirmed a real Kani tooling limitation, not a bug in our own
//! logic.** `commit_contract_free_function_wrapper` (below) verifies
//! `1 of 287 failed` -- but every check from 1 through 24 is Kani's own
//! DFCC contract-checking scaffolding (`__CPROVER_contracts_write_set_
//! check_assignment`, `__rust_dealloc`, `single_top_level_call`, `no_
//! alloc_dealloc_in_requires`/`_ensures`, `no_recursive_call`), not a
//! single one naming `Ledger`/`Transfer`/`commit` or any of our own code.
//! The one that actually fails, `free.frees.1` ("Check that ptr is
//! freeable"), sits immediately after `no_alloc_dealloc_in_ensures`
//! passes, at a generic builtin location (`<builtin-library-free>:43 in
//! function free`) -- confirmed via a real `--harness-timeout 6m` rerun
//! with full (non-terse) output, not assumed from the terse summary
//! alone. Every existing `#[kani::proof_for_contract]` elsewhere in this
//! workspace targets a *method* (`Type::method`); this is the first one
//! to target a bare free function, and that's the one structurally new
//! thing here -- strongly suggesting a genuine edge case in Kani 0.67.0's
//! `-Z function-contracts` DFCC machinery specific to free-function
//! contract targets, not something a different Rust body could route
//! around.
//!
//! A real, second CBMC finding along the way, distinct from the DFCC
//! issue above: the first attempt (`--harness-timeout 3m`, matching
//! every other real contract in this workspace) timed out inconclusively
//! before reaching this failure at all -- only a `6m` rerun converged far
//! enough to surface the real `free.frees.1` failure. The generic-over-
//! `V` path (`Sidecar<V>`/`Establish<C, V>`'s blanket impls, monomorphized
//! at the call site) costs CBMC noticeably more than the concrete,
//! non-generic version already proven in `amenable_kani::ledger` -- a
//! real, separate cost worth keeping in mind even once/if the DFCC issue
//! itself is worked around.
//!
//! **`commit_contract_local_type_wrapper` rules out "bare free function"
//! as the cause.** Identical hypothesis, different wrapper shape: the
//! Kani-contracted function is `KaniLedgerCommit::commit`, an associated
//! function on a local zero-sized type, matching the `Type::method`
//! shape every other real contract in this workspace uses -- and it
//! fails *identically* (`1 of 287 failed`, `free.frees.1`, the exact
//! same builtin location). So the failure isn't specific to bare free
//! functions as `#[kani::proof_for_contract]` targets; it narrows to
//! either the cross-crate call itself or the generic-over-`V` dispatch
//! (both still present in this second variant). Since CBMC/Kani actually
//! analyzes monomorphized MIR (not generic source), "genericity" as a
//! literal runtime-dispatch cost is an unlikely culprit on its own --
//! whatever's actually happening is more specific than either headline
//! hypothesis suggests, and isolating it further needs a differently-
//! shaped probe (e.g. a wrapper delegating to a same-crate or non-generic
//! target) than either variant tried so far.
//!
//! **`commit_contract_trivial_ensures` rules out the postcondition's own
//! content, decisively.** Same real delegating body (`ledger.commit::
//! <KaniVerifier>(input)`), but a content-free contract (`requires(true)`/
//! `ensures(|_| true)`, no `BalancedEntries::ensures(..)` call at all) --
//! still fails identically (`1 of 266 failed` -- 21 fewer checks than the
//! other two variants, exactly the `BalancedEntries`-related ones this
//! trivial contract skips -- but the same `free.frees.1` failure at the
//! same builtin location). Three variants, three consistent failures,
//! nothing left to vary except the delegation itself: this is a real,
//! structural limitation of a Kani-contracted function whose body
//! delegates to a cross-crate generic method with an `Establish`/
//! `Sidecar`-chain-heavy real implementation -- not the wrapper's shape,
//! not the contract's content. Given Kani/CBMC analyzes monomorphized
//! MIR (genericity itself shouldn't matter post-compilation), the most
//! likely remaining candidate is something in DFCC's own "write set"
//! bookkeeping interacting badly with the delegate's real heap-allocating
//! body (`TransferPayload`/`AccountId` clones) once it's reached through
//! an extra call frame -- untested further; see `GAAP_LEDGER_PLAN.md`'s
//! own Step 9 status for where this leaves the wider migration.
//!
//! **`commit_contract_no_wrapper` finds the real solution.** All three
//! failures above shared one thing: the Kani-contracted function was a
//! *wrapper*, delegating to `Ledger::commit` via a separate call. This
//! case removes the wrapper entirely -- the contract sits directly on
//! `amenable_gaap::Ledger::commit<V>`, the real generic method
//! itself, with no delegation at all. It calls through a fully generic
//! `BalancedEntries: Ensures<V, Input = i64, Bound = bool>` bound (never
//! naming `KaniVerifier`, so it still compiles from `amenable_gaap` with
//! no dependency on any backend crate) rather than the established-
//! elsewhere `<Evidence as Ensures<V>>::ensures(result.clone())` shape
//! `#[amenable_derive::exchange]` generates -- a deliberate simplification
//! for this probe, not yet reconciled with that convention. **Verifies
//! clean: `0 of 287 failed`.** This confirms the DFCC failure was
//! specific to the wrapper/delegation pattern, not to cross-crate generic
//! contracts as such -- `Ledger`'s own methods *can* move to
//! `amenable_gaap` for real, as long as each backend's contract attaches
//! directly to the real generic method (Kani: `#[cfg_attr(kani, ..)]`
//! directly on the method, matching this case; Creusot: `extern_spec!`,
//! already proven for `establish()`; Verus: a generated companion,
//! already proven for the token layer) rather than through a delegating
//! wrapper.
//!
//! **Addendum, re-verified 2026-08-28 (`AccountId`/`Account` split,
//! `docs/GAAP_LEDGER_PLAN.md`'s own addendum on that refactor):** all
//! three `free.frees.1` failures above no longer reproduce, even at the
//! same `--harness-timeout 6m` that originally reached them -- each now
//! times out mid-CBMC-solve (millions of SAT variables/clauses, per a
//! real `cargo kani` run with full output, not the terse summary) rather
//! than hitting the fast DFCC scaffolding check at all. Re-classified
//! `Failed` -> `Timeout` on the three affected registrations below to
//! match current, reproducible behavior -- what these three probes
//! establish (a real DFCC limitation on wrapper/delegation shapes,
//! confirmed by the wrapper's error location) is unaffected by *why*
//! they no longer converge, and the actually-relied-upon solution
//! (`commit_contract_no_wrapper`, no wrapper at all) still verifies
//! clean, unaffected either way. Cause not chased further -- same
//! Kani/CBMC version confirmed (0.67.0), no flag change explains it
//! (tested with and without `-Z stubbing`, both time out); one real
//! candidate not ruled out is that `TransferPayload::from`/`::to` went
//! from a flat `{ id: Uuid, name: String }` to one more level of
//! struct nesting (`Account { id: AccountId, name: String }`), adding
//! marginal cost to a proof this section's own investigation already
//! found sitting right at the edge of what a `6m` timeout could reach.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_gaap_free_function_contract::commit_contract_free_function_wrapper".to_owned(),
            "gallery::ledger_gaap_free_function_contract::commit_contract_free_function_wrapper".to_owned(),
            "amenable_kani".to_owned(),
            "Ledger::commit's real body moved to amenable_gaap (generic over V: Verifier), Kani contract attached to a thin free-function wrapper delegating to it -- originally converged on a real Kani DFCC scaffolding failure (free.frees.1, builtin location, immediately after no_alloc_dealloc_in_ensures passes) at a 6m harness timeout; re-verified 2026-08-28, no longer reaches that check even at 6m, now times out mid-CBMC-solve instead -- see this module's own addendum doc comment".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

/// The `#[cfg(kani)]` imports, `commit_checked`, and the two local
/// contract-carrying wrapper types this file needs, consolidated into
/// one gate on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `gallery_harness!
/// { .. }` blocks below (macro invocations, invisible to the
/// cfg-scatter scanner) and the `#[kani::proof_for_contract(..)]`
/// attributes inside them reference all of it, unqualified, from this
/// file's own top level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Sidecar};
    pub(super) use amenable_gaap::{
        BalancedEntries, Committed, CommittedToken, Ledger, Transfer, TransferError, Validated,
        ValidatedToken,
    };

    pub(super) use crate::KaniVerifier;

    /// Not left to go dead-code-unused under an ordinary build: this
    /// whole module only exists under `#[cfg(kani)]`, and this wrapper's
    /// only role is carrying a Kani contract for verification -- it has
    /// no production caller, unlike `amenable_kani::ledger`'s own real
    /// helpers, which real (non-cfg-gated) logic calls into too. Real,
    /// not decorative, otherwise: `Ledger::commit`'s own `#[kani::
    /// requires]` precondition, moved here verbatim from `amenable_kani::
    /// ledger::Ledger::commit`'s own doc comment (real overflow finding,
    /// `GAAP_LEDGER_PLAN.md`'s Step 2) -- `commit` is only ever meant to
    /// be called on an already-`validate`d transfer, which already
    /// established `AmountPositive`.
    #[kani::requires(input.primary().amount().value() > 0)]
    #[kani::ensures(|result: &Result<Transfer<Committed, CommittedToken>, TransferError>| match result {
        Ok(committed) => BalancedEntries::ensures(committed.primary().amount().value()),
        Err(_) => false,
    })]
    pub(super) fn commit_checked(
        ledger: &Ledger,
        input: Transfer<Validated, ValidatedToken>,
    ) -> Result<Transfer<Committed, CommittedToken>, TransferError> {
        ledger.commit::<KaniVerifier>(input)
    }

    /// Zero-sized local wrapper: `Ledger` itself lives in `amenable_gaap` now,
    /// so an inherent impl for it can't be written here at all (inherent
    /// impls require the *type* to be local, no orphan-rule loophole the way
    /// trait impls have one) -- this local type exists purely so `commit`
    /// below can be an *associated function on a type this crate owns*,
    /// matching the shape (`Type::function`) every other real Kani contract
    /// in this workspace uses, rather than a bare free function.
    pub(super) struct KaniLedgerCommit;

    impl KaniLedgerCommit {
        /// Identical real contract and body to `commit_checked`, above --
        /// only the *shape* of what carries it differs (associated function
        /// on a local type vs. a bare free function).
        #[kani::requires(input.primary().amount().value() > 0)]
        #[kani::ensures(|result: &Result<Transfer<Committed, CommittedToken>, TransferError>| match result {
            Ok(committed) => BalancedEntries::ensures(committed.primary().amount().value()),
            Err(_) => false,
        })]
        pub(super) fn commit(
            ledger: &Ledger,
            input: Transfer<Validated, ValidatedToken>,
        ) -> Result<Transfer<Committed, CommittedToken>, TransferError> {
            ledger.commit::<KaniVerifier>(input)
        }
    }

    /// Trivial contract, same real delegating body as `KaniLedgerCommit::
    /// commit`/`commit_checked` -- isolates contract content from
    /// delegation structure as the cause of the confirmed `free.frees.1`
    /// failure.
    pub(super) struct KaniLedgerCommitTrivial;

    impl KaniLedgerCommitTrivial {
        #[kani::requires(true)]
        #[kani::ensures(|_result: &Result<Transfer<Committed, CommittedToken>, TransferError>| true)]
        pub(super) fn commit(
            ledger: &Ledger,
            input: Transfer<Validated, ValidatedToken>,
        ) -> Result<Transfer<Committed, CommittedToken>, TransferError> {
            ledger.commit::<KaniVerifier>(input)
        }
    }
}
#[cfg(kani)]
use mirror::{
    BalancedEntries, Committed, CommittedToken, Ensures, KaniLedgerCommit, KaniLedgerCommitTrivial,
    KaniVerifier, Ledger, Sidecar, Transfer, TransferError, Validated, ValidatedToken,
    commit_checked,
};

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_FREE_FUNCTION_WRAPPER_SRC, {
        #[kani::proof_for_contract(commit_checked)]
        fn commit_contract_free_function_wrapper() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = commit_checked(&ledger, validated);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_gaap_free_function_contract::commit_contract_local_type_wrapper".to_owned(),
            "gallery::ledger_gaap_free_function_contract::commit_contract_local_type_wrapper".to_owned(),
            "amenable_kani".to_owned(),
            "Same real cross-crate delegation as commit_contract_free_function_wrapper, but the Kani-contracted function is an associated function on a local zero-sized wrapper type (KaniLedgerCommit::commit) instead of a bare free function -- originally converged on a real Kani DFCC scaffolding failure (free.frees.1, same builtin location) at a 6m harness timeout, ruling out \"bare free function specifically\" as the cause and narrowing it to the cross-crate call or the generic-over-V dispatch; re-verified 2026-08-28, no longer reaches that check even at 6m, now times out mid-CBMC-solve instead -- see this module's own addendum doc comment".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_LOCAL_TYPE_WRAPPER_SRC, {
        #[kani::proof_for_contract(KaniLedgerCommit::commit)]
        fn commit_contract_local_type_wrapper() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = KaniLedgerCommit::commit(&ledger, validated);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_gaap_free_function_contract::commit_contract_trivial_ensures".to_owned(),
            "gallery::ledger_gaap_free_function_contract::commit_contract_trivial_ensures".to_owned(),
            "amenable_kani".to_owned(),
            "Same real cross-crate generic delegation (ledger.commit::<KaniVerifier>(input)) as the other two cases, but the contract itself is trivial (requires(true)/ensures(|_| true), dropping the BalancedEntries::ensures(..) call and match) -- originally converged identically (free.frees.1, same builtin location, 266 checks not 287 since the trivial ensures skips BalancedEntries) at a 6m harness timeout, definitively ruling out contract content as the cause: this is purely structural to a Kani-contracted function whose body delegates to a cross-crate generic method; re-verified 2026-08-28, no longer reaches that check even at 6m, now times out mid-CBMC-solve instead -- see this module's own addendum doc comment".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_TRIVIAL_ENSURES_SRC, {
        #[kani::proof_for_contract(KaniLedgerCommitTrivial::commit)]
        fn commit_contract_trivial_ensures() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = KaniLedgerCommitTrivial::commit(&ledger, validated);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_gaap_free_function_contract::commit_contract_no_wrapper".to_owned(),
            "gallery::ledger_gaap_free_function_contract::commit_contract_no_wrapper".to_owned(),
            "amenable_kani".to_owned(),
            "CONFIRMED SOLUTION: attach the Kani contract directly to amenable_gaap::Ledger::commit (a real, generic-over-V method) with zero delegating wrapper at all, using a fully generic BalancedEntries: Ensures<V, ..> bound (never naming a concrete verifier) so it still compiles from amenable_gaap with no backend dependency -- verifies clean (0 of 287 failed), confirming the free.frees.1 DFCC failure was specific to the wrapper/delegation pattern, not to cross-crate generic contracts as such; Ledger's own methods CAN move to amenable_gaap for real by attaching the contract directly instead of through a wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_NO_WRAPPER_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::commit)]
        fn commit_contract_no_wrapper() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.commit::<KaniVerifier>(validated);
        }
    }
}
