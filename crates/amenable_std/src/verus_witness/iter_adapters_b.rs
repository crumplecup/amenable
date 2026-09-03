//! `VerusWitness` impls for the first batch of stateless/count-limited
//! iterator adapters: `Rev`/`Skip`/`SkipWhile`/`StepBy`/`Take`/`TakeWhile`/
//! `Once`/`OnceWith`/`Repeat`/`RepeatWith`/`RepeatN`/`Empty`. The stateful
//! adapters (`Cycle`/`Fuse`/`Inspect`/`Peekable`/`Scan`/`FlatMap`/`Flatten`)
//! are in `iter_adapters_d`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

const VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_rev_model_reverses_iteration_order".to_owned(),
            VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_model_discards_the_first_n_items".to_owned(),
            VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_while_model_discards_items_while_the_predicate_holds".to_owned(),
            VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_step_by_model_yields_every_nth_item".to_owned(),
            VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Take<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_model_yields_at_most_n_items".to_owned(),
            VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Take<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Take<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Take<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_while_model_yields_items_while_the_predicate_holds".to_owned(),
            VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

// `pub(super)`: `iter_markers`'s `ObservedOptionMatchesInput` witness reuses
// this harness's source as its own `claim`.
pub(super) const VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Once<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::OnceWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_with_model_calls_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::OnceWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::OnceWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_model_yields_the_same_value_forever".to_owned(),
            VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Repeat<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_with_model_calls_its_closure_once_per_item".to_owned(),
            VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_n_model_yields_the_value_exactly_n_times".to_owned(),
            VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatN<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatN<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_empty_model_yields_nothing".to_owned(),
            VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Empty<i32>> as VerusWitness>::proof().to_string()
        },
    )
}
