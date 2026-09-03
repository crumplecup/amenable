use core::fmt::{Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// A rendered value's `.kind()` known to match the formatting operation
/// that actually built it.
///
/// Independently hand-written as `assert_eq!(rendered.kind(),
/// crate::KaniRenderedKind::Variant)` at 7 real sites, one per
/// `KaniFmt` builder (`arguments`, the five `Debug*` builders, and
/// `from_fn`) -- the identical equality check regardless of which
/// variant the real site expects, since `rendered.kind()` and every
/// expected value share the one local `KaniRenderedKind` enum. Needs no
/// type parameter: unlike the generic contract types in
/// `rust_std::iter`/`rust_std::primitives`, every real site already
/// compares the same fixed type, so there's nothing left to be generic
/// over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct RenderedKindMatchesTheBuildingOperation;

impl KaniWitness for RenderedKindMatchesTheBuildingOperation {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_arguments_renders_the_same_as_the_value_itself".to_owned(),
            VERIFY_ARGUMENTS_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RenderedKindMatchesTheBuildingOperation);

kani_ensures!(
    RenderedKindMatchesTheBuildingOperation,
    "amenable_kani::RenderedKindMatchesTheBuildingOperation",
    (crate::KaniRenderedKind, crate::KaniRenderedKind),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::RenderedKindMatchesTheBuildingOperation",
        "kani",
        || <RenderedKindMatchesTheBuildingOperation as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Arguments<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_arguments_renders_the_same_as_the_value_itself".to_owned(),
            VERIFY_ARGUMENTS_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Arguments<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Arguments<'static>>",
        "kani",
        || <RustStdStandard<Arguments<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ARGUMENTS_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC, {
        /// `format_args!("{}", value)` renders identically to
        /// `value.to_string()` — `Arguments` is precompiled formatting
        /// instructions, not a copy or transformation of the value.
        /// This proof uses the Amenable-owned formatter accommodation model:
        /// if the real formatting path refines these modeled laws, the
        /// Rust-facing claim follows.
        #[kani::proof]
        fn verify_arguments_renders_the_same_as_the_value_itself() {
            let atom = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let rendered = crate::KaniFmt::arguments(&atom);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::Arguments
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.display_token(),
                Some(atom.display_token())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugStruct<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_debug_struct_renders_named_fields".to_owned(),
            VERIFY_DEBUG_STRUCT_RENDERS_NAMED_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DebugStruct<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DebugStruct<'static, 'static>>",
        "kani",
        || <RustStdStandard<DebugStruct<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBUG_STRUCT_RENDERS_NAMED_FIELDS_SRC, {
        /// `debug_struct("Name").field("x", &value).finish()` renders
        /// as `Name { x: value }`, for any symbolic value.
        /// This proof uses the Amenable-owned formatter accommodation model:
        /// if the real builder path preserves the supplied labels, named-field
        /// shape, and debug value token the same way this model does, the
        /// Rust-facing claim follows.
        #[kani::proof]
        fn verify_debug_struct_renders_named_fields() {
            let atom = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let type_label = crate::KaniFormatLabel::new('P');
            let field_label = crate::KaniFormatLabel::new('x');
            let rendered = crate::KaniFmt::debug_struct_one_field(type_label, field_label, &atom);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::DebugStructOneField
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.type_label(),
                Some(type_label)
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.field_label(),
                Some(field_label)
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.value_debug_token(),
                Some(atom.debug_token())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugTuple<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_debug_tuple_renders_positional_fields".to_owned(),
            VERIFY_DEBUG_TUPLE_RENDERS_POSITIONAL_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DebugTuple<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DebugTuple<'static, 'static>>",
        "kani",
        || <RustStdStandard<DebugTuple<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBUG_TUPLE_RENDERS_POSITIONAL_FIELDS_SRC, {
        /// `debug_tuple("Name").field(&value).finish()` renders as
        /// `Name(value)`, unlike `DebugStruct`'s named-field form.
        #[kani::proof]
        fn verify_debug_tuple_renders_positional_fields() {
            let atom = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let type_label = crate::KaniFormatLabel::new('P');
            let rendered = crate::KaniFmt::debug_tuple_one_field(type_label, &atom);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::DebugTupleOneField
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.type_label(),
                Some(type_label)
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.value_debug_token(),
                Some(atom.debug_token())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugList<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_debug_list_renders_entries_in_brackets".to_owned(),
            VERIFY_DEBUG_LIST_RENDERS_ENTRIES_IN_BRACKETS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DebugList<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DebugList<'static, 'static>>",
        "kani",
        || <RustStdStandard<DebugList<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBUG_LIST_RENDERS_ENTRIES_IN_BRACKETS_SRC, {
        /// `debug_list().entries([a, b]).finish()` renders as `[a, b]`,
        /// unlike `DebugSet`'s brace form for the same two entries.
        #[kani::proof]
        fn verify_debug_list_renders_entries_in_brackets() {
            let first = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let second = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let rendered = crate::KaniFmt::debug_list_two_entries(&first, &second);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::DebugListTwoEntries
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.first_debug_token(),
                Some(first.debug_token())
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.second_debug_token(),
                Some(second.debug_token())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugSet<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_debug_set_renders_entries_in_braces".to_owned(),
            VERIFY_DEBUG_SET_RENDERS_ENTRIES_IN_BRACES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DebugSet<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DebugSet<'static, 'static>>",
        "kani",
        || <RustStdStandard<DebugSet<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBUG_SET_RENDERS_ENTRIES_IN_BRACES_SRC, {
        /// `debug_set().entries([a, b]).finish()` renders as `{a, b}`
        /// — braces, not `DebugList`'s brackets, for the same two
        /// entries.
        #[kani::proof]
        fn verify_debug_set_renders_entries_in_braces() {
            let first = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let second = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let rendered = crate::KaniFmt::debug_set_two_entries(&first, &second);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::DebugSetTwoEntries
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.first_debug_token(),
                Some(first.debug_token())
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.second_debug_token(),
                Some(second.debug_token())
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugMap<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_debug_map_renders_key_value_pairs".to_owned(),
            VERIFY_DEBUG_MAP_RENDERS_KEY_VALUE_PAIRS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DebugMap<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DebugMap<'static, 'static>>",
        "kani",
        || <RustStdStandard<DebugMap<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBUG_MAP_RENDERS_KEY_VALUE_PAIRS_SRC, {
        /// `debug_map().entry(&key, &value).finish()` renders each
        /// entry as `key: value` (both `Debug`-formatted, so a string
        /// key is quoted) inside braces.
        #[kani::proof]
        fn verify_debug_map_renders_key_value_pairs() {
            let value = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let key_label = crate::KaniFormatLabel::new('k');
            let rendered = crate::KaniFmt::debug_map_one_entry(key_label, &value);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::DebugMapOneEntry
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.key_debug_label(),
                Some(key_label)
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.value_debug_token(),
                Some(value.debug_token())
            )));
        }
    }
}

impl KaniWitness
    for RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_fn_forwards_display_to_the_supplied_closure".to_owned(),
            VERIFY_FROM_FN_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(
    RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>",
        "kani",
        || <RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FROM_FN_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC, {
        /// `fmt::from_fn(closure)` renders via `Display` exactly as the
        /// closure itself writes — the same "Display pass-through for
        /// one leaf" law `Arguments`'s proof already models (calling
        /// `.to_string()` directly times out under Kani's formatting
        /// machinery here too, confirmed empirically). This proof uses
        /// the same Amenable-owned formatter accommodation model: if the
        /// real `from_fn` path preserves the written token the same way
        /// `KaniFmt::arguments` does, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_from_fn_forwards_display_to_the_supplied_closure() {
            let atom = <crate::KaniFormatAtom as crate::KaniCompose>::kani_any();
            let rendered = crate::KaniFmt::arguments(&atom);
            assert!(RenderedKindMatchesTheBuildingOperation::ensures((
                rendered.kind(),
                crate::KaniRenderedKind::Arguments
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                rendered.display_token(),
                Some(atom.display_token())
            )));
        }
    }
}
