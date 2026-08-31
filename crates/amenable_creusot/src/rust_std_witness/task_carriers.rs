use super::CheckedProof;

use std::future::{Pending, PollFn, Ready};
use std::task::{Context, Poll, Waker};

use crate::{
    CreusotVerifier, CreusotWitness, PENDING_NEVER_RESOLVES_HOLDS_SRC,
    POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_HOLDS_SRC,
    POLL_READY_AND_PENDING_ARE_DISJOINT_HOLDS_SRC,
    READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_HOLDS_SRC,
    VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC, VERIFY_PENDING_NEVER_RESOLVES_SRC,
    VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC,
    VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC,
    VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC,
    VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC,
    WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_HOLDS_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
// Bare `Pending<i32>`, matching `amenable_std::rust_std::future`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Pending<i32>>`).
impl CreusotWitness for RustStdStandard<Pending<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_pending_never_resolves".to_string(),
            VERIFY_PENDING_NEVER_RESOLVES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        "creusot",
        || <RustStdStandard<Pending<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::PENDING_NEVER_RESOLVES_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn pending_never_resolves_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Pending<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        PENDING_NEVER_RESOLVES_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Pending<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Bare `PollFn<fn(&mut Context<'_>) -> Poll<i32>>`, matching
// `amenable_std::rust_std::future`'s own registration exactly.
impl CreusotWitness for RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_fn_dispatches_through_to_its_closure".to_string(),
            VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        "creusot",
        || <RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> as CreusotWitness>::proof()
            .to_string(),
    )
}

/// Returns
/// `amenable_creusot::POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn poll_fn_dispatches_through_to_its_closure_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Bare `Ready<i32>`, matching `amenable_std::rust_std::future`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Ready<i32>>`).
impl CreusotWitness for RustStdStandard<Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ready_resolves_immediately_with_its_value".to_string(),
            VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        "creusot",
        || <RustStdStandard<Ready<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn ready_resolves_immediately_with_its_value_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Ready<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Ready<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Bare `Context<'static>`, matching `amenable_std::rust_std::task`'s own
// representative lifetime choice exactly.
impl CreusotWitness for RustStdStandard<Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_context_from_waker_exposes_the_same_waker".to_string(),
            VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Context<'static>>",
        "creusot",
        || <RustStdStandard<Context<'static>> as CreusotWitness>::proof().to_string(),
    )
}

// Bare `Poll<i32>`, matching `amenable_std::rust_std::task`'s own
// representative instantiation exactly.
impl CreusotWitness for RustStdStandard<Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_ready_and_pending_are_disjoint".to_string(),
            VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Poll<i32>>);

/// Returns `amenable_creusot::POLL_READY_AND_PENDING_ARE_DISJOINT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn poll_ready_and_pending_are_disjoint_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Poll<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        POLL_READY_AND_PENDING_ARE_DISJOINT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Poll<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
        "creusot",
        || <RustStdStandard<Poll<i32>> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_waker_wake_by_ref_invokes_the_wake_impl".to_string(),
            VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Waker>",
        "creusot",
        || <RustStdStandard<Waker> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn waker_wake_by_ref_invokes_the_wake_impl_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Waker> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Waker>",
        "creusot",
        "ensures",
        || <RustStdStandard<Waker> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
