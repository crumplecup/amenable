/// The `#[cfg(creusot)]` content of this file (imports, `extern_spec!`s,
/// `Transfer<S, Token>`, `Amount`, `TransferPayload`, the `Pending`
/// witness), consolidated into one gate on this `mod` instead of one per
/// item -- see `stoplight::mirror`'s own doc comment for the general
/// rationale. The two `harness! { .. }` invocations below stay outside,
/// at this file's own top level: each generates an always-visible
/// `..._SRC` const its own `#[cfg(not(creusot))]`-gated `ContractRecord`
/// submission needs, so (like `stoplight.rs`'s `include!`d files) they
/// can't move into a `#[cfg(creusot)]`-only module without making that
/// registration unsatisfiable in every configuration. `Transfer` is the
/// only name this module re-exports: `validated_committed_rejected.rs`
/// (a sibling file, not a descendant of this module) is the only outside
/// consumer of anything defined here.
#[cfg(creusot)]
mod mirror {
    use amenable_core::{Establish, Evidence, Sidecar, Witness};
    use amenable_gaap::{
        Committed, CommittedToken, Pending, PendingToken, Rejected, RejectedFromPendingToken,
        RejectedFromValidatedToken, Validated, ValidatedToken,
    };
    use creusot_std::macros::{ensures, extern_spec, requires};
    use creusot_std::std::ops::FnOnceExt;

    use crate::CreusotVerifier;

    use super::{amount_value_matches_field, transfer_payload_field_matches};

    // A real, confirmed gap in `creusot_std` itself, not this crate's own:
    // `Result::map_err` carries no contract anywhere in `creusot-std`'s own
    // source (confirmed by reading `creusot-std/src/std/result.rs` directly
    // -- `map`/`and`/`or`/`unwrap_or`/etc. are all specified there, `map_err`
    // is not), so calling it with no `extern_spec!` of our own produces a
    // real "calling external function `map_err` with no contract will yield
    // an impossible precondition" warning and a genuine unprovable goal --
    // confirmed against the real toolchain, not assumed: `Ledger::validate`'s
    // own real body captured below (`GAAP_LEDGER_PLAN.md`'s Step 6) is the
    // first proof in this crate to call `.map_err(..)` at all. Modeled
    // directly on `creusot-std`'s own `Option::map`'s real `extern_spec!`
    // entry (`existential postcondition_once`, the same shape every other
    // `FnOnce`-taking `Result`/`Option` method there uses).
    //
    // Plain `//` comments, not `///`: rustdoc doesn't generate documentation
    // for items produced by a macro invocation, so a doc comment here is
    // real, silently-dropped dead weight -- confirmed by `cargo creusot`'s
    // own real "unused doc comment" warning, invisible to ordinary `cargo
    // check`/clippy the same way every `#[cfg(creusot)]`-gated content is.
    extern_spec! {
        impl<T, E> Result<T, E> {
            #[requires(match self {
                Ok(_) => true,
                Err(e) => op.precondition((e,)),
            })]
            #[ensures(match self {
                Ok(t) => result == Ok(t),
                Err(e) => exists<r> result == Err(r) && op.postcondition_once((e,), r),
            })]
            fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F>;
        }
    }

    // `GAAP_LEDGER_PLAN.md`'s Step 7: the same real, confirmed gap as
    // `map_err`'s own extern_spec, above, one layer up. `Establish::
    // establish` now lives in `amenable_gaap` (one real, backend-generic
    // blanket impl, not a per-verifier one hand-written locally here) --
    // `creusot-rustc`'s translator only fully analyzes items *local* to the
    // crate it's translating, so a call through a dependency's impl is
    // "external" to it the same way a std method is, and needs its own
    // explicit contract. Confirmed against the real toolchain: without
    // this, `cargo creusot prove` reports "calling external function
    // `establish` with no contract will yield an impossible precondition"
    // and a genuine unprovable `vc_validate`/`vc_commit` goal.
    //
    // `#[ensures(true)]`, not a richer claim: `establish`'s own real body
    // (every hand-written impl in this workspace, and the blanket impl
    // alike) ignores its credential and mints a bare unit token whose only
    // field is private -- there is no observable postcondition to state
    // about the *result* from Pearlite, matching this project's own
    // "tautological model" precedent (a real, honest `true` for a
    // genuinely content-free claim, not a stand-in for a stronger one this
    // crate is avoiding).
    //
    // `-> ValidatedToken`/`-> CommittedToken`, not `-> Self::Token`:
    // `extern_spec!` real-toolchain-confirmed rejects an associated-type
    // return position ("Cannot use Self here") -- spelled out as the
    // concrete type each blanket impl instantiation actually resolves to
    // instead.
    //
    // `impl<V: Verifier> .. for Validated`, not `impl .. for Validated`
    // fixed to `CreusotVerifier`: `extern_spec!` real-toolchain-confirmed
    // ("extern spec generics don't match") requires matching the *real*
    // item's own generic shape exactly -- the real impl is the one, generic
    // blanket impl in `amenable_gaap` (`impl<V: Verifier> Establish<C, V>
    // for Y where Y: Witness<V>`), not a per-verifier concrete one, so the
    // extern_spec has to be written the same way, `V` and all.
    extern_spec! {
        impl<V: amenable_core::Verifier> Establish<PendingToken, V> for Validated
        where
            Validated: Witness<V>,
        {
            #[ensures(true)]
            fn establish(credential: PendingToken) -> ValidatedToken;
        }

        impl<V: amenable_core::Verifier> Establish<ValidatedToken, V> for Committed
        where
            Committed: Witness<V>,
        {
            #[ensures(true)]
            fn establish(credential: ValidatedToken) -> CommittedToken;
        }

        // `reject`'s/`rollback`'s own real captured bodies (`generated/
        // reject.rs`/`generated/rollback.rs`, below) call the identical
        // external `establish` -- revisited scope, `GAAP_LEDGER_PLAN.md`'s
        // Step 7: same real gap, same fix, same reasoning as `Validated`'s/
        // `Committed`'s own entries above.
        impl<V: amenable_core::Verifier> Establish<PendingToken, V> for Rejected<Pending>
        where
            Rejected<Pending>: Witness<V>,
        {
            #[ensures(true)]
            fn establish(credential: PendingToken) -> RejectedFromPendingToken;
        }

        impl<V: amenable_core::Verifier> Establish<ValidatedToken, V> for Rejected<Validated>
        where
            Rejected<Validated>: Witness<V>,
        {
            #[ensures(true)]
            fn establish(credential: ValidatedToken) -> RejectedFromValidatedToken;
        }
    }

    /// `GAAP_LEDGER_PLAN.md`'s Step 6: an accommodation-model mirror for
    /// `amenable_kani::ledger::{Ledger, Transfer<S, Token>}` and their tokens
    /// -- needed for the same real reason `amenable_creusot::stoplight`'s own
    /// mirror is, and only now, since `Ledger::validate`'s/`::commit`'s real
    /// bodies are captured verbatim below for the first time (`validate.rs`/
    /// `commit.rs`, generated by `amenable::creusot_export`): `Ledger`/
    /// `Transfer<S, Token>` live only in `amenable_kani`, a crate this one
    /// cannot depend on (verifier backends never depend on each other), and
    /// their real constructors are deliberately private (`Transfer::new` is
    /// `pub(crate)` there), so a real dependency wouldn't let the captured
    /// bodies compile against the real types even if the dependency itself
    /// were legal. Matches `amenable_gaap`'s own real evidence markers
    /// (`Pending`/`Validated`/`Committed`) directly -- no mirror needed for
    /// those, since they're not privacy-gated and this crate already depends
    /// on `amenable_gaap` for real.
    ///
    /// `Transfer<S, Token>`'s own `Sidecar<CreusotVerifier>` impl is derived,
    /// not hand-written -- `#[derive(amenable_derive::Sidecar)]`, the same
    /// derive `amenable_kani::ledger::Transfer`'s real definition and
    /// `amenable_creusot::stoplight::Established`'s own mirror both use for
    /// the identical shape (`GAAP_LEDGER_PLAN.md`'s Step 5).
    #[derive(amenable_derive::Sidecar)]
    #[sidecar(
        verifier = "CreusotVerifier",
        proposition = "S",
        constructor = "pub(crate)"
    )]
    pub struct Transfer<S, Token> {
        // Fully `pub`, not `pub(crate)`: Creusot's own proof-transparency
        // check requires an `#[ensures(..)]` clause to only ever reach items
        // *at least* as visible as the function stating it -- and `Sidecar`'s
        // own `primary()` method (`amenable_core::Sidecar` is a `pub` trait)
        // is as visible as the trait impl itself, not merely `pub(crate)`
        // like `new`'s own constructor. Confirmed the hard way in two steps:
        // a real "cannot make `.. payload` transparent in `.. new`" error
        // once `#[derive(amenable_derive::Sidecar)]`'s generated constructor
        // gained a real `#[ensures(result.payload == payload)]` clause
        // (`GAAP_LEDGER_PLAN.md`'s Step 6), fixed by widening to `pub(crate)`
        // -- then the *same* error again, this time naming `Sidecar::
        // primary()` instead of `new`, once that method *also* gained a real
        // `ensures` clause. No privacy invariant to protect here either way,
        // unlike the real `amenable_kani::ledger::Transfer`'s own private
        // fields.
        #[sidecar(primary)]
        pub payload: TransferPayload,
        #[sidecar(token)]
        pub token: Token,
        _state: std::marker::PhantomData<S>,
    }

    // `Amount` only exists under `#[cfg(creusot)]` (a sanitized mirror, not
    // the real `amenable_gaap::Amount`), so it can never carry a real
    // `impl Ensures<CreusotVerifier>` the way `AmountPositive` does --
    // `inventory::submit!` needs to run in an ordinary (non-translated)
    // build to ever reach `amenable dump-registry`, and this type doesn't
    // exist there at all. A bare `ContractRecord` submission sidesteps
    // that: the registry entry only needs a descriptive `evidence` label
    // and a real fragment, not a trait impl on a real `Self` type --
    // `cordial`'s own Creusot/Verus call-shape recognition never consults
    // `evidence` for these two verifiers anyway (see `CONTRACT_BOUND_
    // NAMING_WORKFLOW.md`'s own Gotchas).

    /// Sanitized mirror of `amenable_gaap::Amount` -- the real captured
    /// `validate`/`commit` bodies call `.amount().value()`, so this needs
    /// the same two-method chain, not just a bare `i64`. The field is `pub`,
    /// not private: `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4 gave
    /// `Ledger`'s edges a real `Exchange` trait impl, whose `exchange()`
    /// method is necessarily as visible as the (public) `Exchange` trait
    /// itself -- more visible than the private inherent method its
    /// `#[ensures(..)]` clause used to sit on alone. Creusot's
    /// proof-transparency check requires everything an `#[ensures(..)]`
    /// clause touches to be at least as visible as the function carrying it
    /// (the same real constraint `GAAP_LEDGER_PLAN.md`'s Step 6 already hit
    /// and fixed the same way, confirmed again here rather than assumed
    /// still true).
    #[derive(Clone, Copy)]
    pub struct Amount(pub i64);

    impl Amount {
        // `#[ensures(..)]`: ordinary modular verification only exposes what
        // a function's own `ensures` promises -- without this, nothing
        // downstream could learn `.value()`'s actual result from `self`'s
        // own field, the identical real reason every accessor below needs
        // one too (`GAAP_LEDGER_PLAN.md`'s Step 6, confirmed the hard way:
        // `validate`'s own success-path postcondition could not be proven
        // without these, even after `Transfer::new`'s own `#[derive(Sidecar)]`
        // -generated `ensures` was fixed).
        #[requires(true)]
        #[ensures(amount_value_matches_field(result, self.0))]
        pub(crate) fn value(&self) -> i64 {
            self.0
        }
    }

    // See `Amount`'s own bare `ContractRecord` submission, at this file's
    // own top level, for why this can't be a real `impl
    // Ensures<CreusotVerifier>` on `TransferPayload` itself.

    /// Sanitized mirror of `amenable_gaap::TransferPayload` -- `from`/`to`
    /// are bare `u64`, matching this file's own existing sanitized-mirror
    /// choice for `check_accounts_distinct`'s own isolated proof, above:
    /// the real claim only needs *some* comparable identity type, and the
    /// real captured body never names `AccountId` as a type, only ever
    /// compares what `.from()`/`.to()` return. `from`/`to`/`amount` are all
    /// `pub`, not private -- see [`Amount`]'s own doc comment for why: the real
    /// `Exchange::exchange` impl's `#[ensures(..)]` clause (`docs/
    /// STATE_MACHINE_DERIVATION_PLAN.md`'s Step 4) accesses them directly,
    /// and needs to be at least as visible as that (public trait) method.
    #[derive(Clone, Copy)]
    pub struct TransferPayload {
        pub from: u64,
        pub to: u64,
        pub amount: Amount,
    }

    impl TransferPayload {
        // `#[ensures(..)]` throughout this impl -- same real reason as
        // [`Amount::value`], above.
        #[requires(true)]
        #[ensures(transfer_payload_field_matches(result, self.from))]
        pub(crate) fn from(&self) -> u64 {
            self.from
        }

        #[requires(true)]
        #[ensures(transfer_payload_field_matches(result, self.to))]
        pub(crate) fn to(&self) -> u64 {
            self.to
        }

        #[requires(true)]
        #[ensures(amount_value_matches_field(result.0, self.amount.0))]
        pub(crate) fn amount(&self) -> Amount {
            self.amount
        }
    }

    impl Evidence for TransferPayload {
        type Basis = Self;
        type Audit = ();

        fn basis() -> Self::Basis {
            TransferPayload {
                from: 0,
                to: 0,
                amount: Amount(0),
            }
        }

        fn audit(&self) {}
    }

    /// `GAAP_LEDGER_PLAN.md`'s Step 7: `PendingToken`/`ValidatedToken`/
    /// `CommittedToken` are no longer mirrored here -- they're real,
    /// backend-neutral types imported directly from `amenable_gaap`
    /// (`amenable_creusot` already depends on it for real), each minted via
    /// one backend-generic `impl<V: Verifier> Establish<C, V> for Y where Y:
    /// Witness<V>` living in `amenable_gaap` itself. That blanket impl
    /// becomes a real, usable `Establish<PendingToken, CreusotVerifier> for
    /// Validated` for free, the moment `Validated: Witness<CreusotVerifier>`
    /// holds -- which it already does, further down this file. No token
    /// mirror, no hand-written `Establish` impl, needed here at all anymore.
    ///
    /// Trivial, matching `amenable_kani::ledger::Pending`'s own `Witness<
    /// KaniVerifier>` impl exactly (see its own doc comment): nothing in
    /// this worked example's scope ever targets `Pending`, so nothing
    /// generates one -- needed here only because `Transfer<Pending,
    /// PendingToken>: Sidecar<CreusotVerifier>` (`validate`'s own input
    /// type) requires `Pending: Evidence + Witness<CreusotVerifier>`.
    impl Witness<CreusotVerifier> for Pending {
        type SupportingEvidence = Self;
        type ProofArtifact = ();

        fn proof() -> Self::ProofArtifact {}
    }
}
#[cfg(creusot)]
pub(crate) use mirror::Transfer;

// `logic`, not folded into `mirror`'s own import list: the two
// `harness! { .. }` invocations below are pasted at this file's own top
// level (see `mirror`'s own doc comment for why), so their `#[logic(open)]`
// attribute needs to resolve from here, not from inside `mirror`.
// `pearlite!` itself needs no explicit import -- confirmed against the
// original, unmodified file: it resolves unqualified either way.
#[cfg(creusot)]
use creusot_std::macros::logic;

amenable_derive::harness! {
    creusot, AMOUNT_VALUE_MATCHES_FIELD_SRC, {
        /// `Amount::value`'s own accessor postcondition, named once
        /// instead of restated -- real, callable Pearlite content: an
        /// accessor returns exactly the field it wraps. Plain `i64`
        /// logic, no dependency on `Amount` itself, so (like
        /// `amount_positive_holds` above) it compiles the same in
        /// ordinary and Creusot-translated builds alike.
        #[logic(open)]
        pub fn amount_value_matches_field(observed: i64, field: i64) -> bool {
            pearlite! { observed == field }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::Amount",
        "creusot",
        "ensures",
        || AMOUNT_VALUE_MATCHES_FIELD_SRC,
    )
}

amenable_derive::harness! {
    creusot, TRANSFER_PAYLOAD_FIELD_MATCHES_SRC, {
        /// `TransferPayload::from`/`::to`'s shared accessor
        /// postcondition -- same real reason as `amount_value_matches_
        /// field`, above, generalized to the one other field type this
        /// struct's own accessors need. Plain `u64` logic, same
        /// unconditional-compile reasoning.
        #[logic(open)]
        pub fn transfer_payload_field_matches(observed: u64, field: u64) -> bool {
            pearlite! { observed == field }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::TransferPayload",
        "creusot",
        "ensures",
        || TRANSFER_PAYLOAD_FIELD_MATCHES_SRC,
    )
}
