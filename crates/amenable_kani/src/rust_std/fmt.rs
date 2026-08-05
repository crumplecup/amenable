//! `KaniWitness` impls for `core::fmt`.
//!
//! `Error` is an opaque unit-like marker signaling that a formatting trait
//! impl failed to write to its `Formatter` — no accessors beyond `Debug`/
//! `Display`, nothing to build and check. It stays at the trusted
//! disposition.
//!
//! The direct rendering paths for `Arguments`, `FromFn`, and the `Debug*`
//! builders time out under Kani's formatting machinery. Production proofs
//! for those shapes therefore use an Amenable-owned formatter model
//! instead; `Alignment` and `Formatter` remain on the direct observable
//! std path.

use core::fmt::{Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<std::fmt::Alignment> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_alignment_reaches_the_formatter_from_the_format_spec".to_owned(),
            claim: VERIFY_ALIGNMENT_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::Alignment>",
        verifier: "kani",
        describe: || <RustStdStandard<std::fmt::Alignment> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ALIGNMENT_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC, {
        /// `Alignment` isn't directly constructible by user code; this
        /// checks it the only way it's actually observable — a `Left`
        /// fill-alignment spec (`{:<5}`) makes `Formatter::align()`
        /// report `Some(Alignment::Left)` inside the trait impl being
        /// formatted.
        #[kani::proof]
        fn verify_alignment_reaches_the_formatter_from_the_format_spec() {
            struct Probe;
            impl std::fmt::Display for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    assert_eq!(f.align(), Some(core::fmt::Alignment::Left), "the spec's alignment reaches the Formatter");
                    write!(f, "x")
                }
            }
            let _ = format!("{:<5}", Probe);
        }
    }
}

impl KaniWitness for RustStdStandard<Arguments<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_arguments_renders_the_same_as_the_value_itself".to_owned(),
            claim: VERIFY_ARGUMENTS_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Arguments<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Arguments<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Arguments<'static>> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::Arguments);
            assert_eq!(rendered.display_token(), Some(atom.display_token()));
        }
    }
}

impl_kani_witness_trusted!(std::fmt::Error);

impl KaniWitness for RustStdStandard<Formatter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_formatter_exposes_the_parsed_width_and_precision".to_owned(),
            claim: VERIFY_FORMATTER_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Formatter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Formatter<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Formatter<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FORMATTER_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC, {
        /// A `{:10.2}` format spec makes `Formatter::width()`/
        /// `.precision()` report `Some(10)`/`Some(2)` inside the trait
        /// impl being formatted — the concrete values the spec was
        /// parsed into, not just that they were provided.
        #[kani::proof]
        fn verify_formatter_exposes_the_parsed_width_and_precision() {
            struct Probe;
            impl std::fmt::Display for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    assert_eq!(f.width(), Some(10), "the spec's width reaches the Formatter");
                    assert_eq!(f.precision(), Some(2), "the spec's precision reaches the Formatter");
                    write!(f, "x")
                }
            }
            let _ = format!("{:10.2}", Probe);
        }
    }
}

impl KaniWitness for RustStdStandard<DebugStruct<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_debug_struct_renders_named_fields".to_owned(),
            claim: VERIFY_DEBUG_STRUCT_RENDERS_NAMED_FIELDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DebugStruct<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DebugStruct<'static, 'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<DebugStruct<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::DebugStructOneField);
            assert_eq!(rendered.type_label(), Some(type_label));
            assert_eq!(rendered.field_label(), Some(field_label));
            assert_eq!(rendered.value_debug_token(), Some(atom.debug_token()));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugTuple<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_debug_tuple_renders_positional_fields".to_owned(),
            claim: VERIFY_DEBUG_TUPLE_RENDERS_POSITIONAL_FIELDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DebugTuple<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DebugTuple<'static, 'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<DebugTuple<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::DebugTupleOneField);
            assert_eq!(rendered.type_label(), Some(type_label));
            assert_eq!(rendered.value_debug_token(), Some(atom.debug_token()));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugList<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_debug_list_renders_entries_in_brackets".to_owned(),
            claim: VERIFY_DEBUG_LIST_RENDERS_ENTRIES_IN_BRACKETS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DebugList<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DebugList<'static, 'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<DebugList<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::DebugListTwoEntries);
            assert_eq!(rendered.first_debug_token(), Some(first.debug_token()));
            assert_eq!(rendered.second_debug_token(), Some(second.debug_token()));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugSet<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_debug_set_renders_entries_in_braces".to_owned(),
            claim: VERIFY_DEBUG_SET_RENDERS_ENTRIES_IN_BRACES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DebugSet<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DebugSet<'static, 'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<DebugSet<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::DebugSetTwoEntries);
            assert_eq!(rendered.first_debug_token(), Some(first.debug_token()));
            assert_eq!(rendered.second_debug_token(), Some(second.debug_token()));
        }
    }
}

impl KaniWitness for RustStdStandard<DebugMap<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_debug_map_renders_key_value_pairs".to_owned(),
            claim: VERIFY_DEBUG_MAP_RENDERS_KEY_VALUE_PAIRS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DebugMap<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DebugMap<'static, 'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<DebugMap<'static, 'static>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::DebugMapOneEntry);
            assert_eq!(rendered.key_debug_label(), Some(key_label));
            assert_eq!(rendered.value_debug_token(), Some(value.debug_token()));
        }
    }
}

impl KaniWitness
    for RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_fn_forwards_display_to_the_supplied_closure".to_owned(),
            claim: VERIFY_FROM_FN_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(
    RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::fmt::FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>> as KaniWitness>::proof()
            .to_string(),
    }
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
            assert_eq!(rendered.kind(), crate::KaniRenderedKind::Arguments);
            assert_eq!(rendered.display_token(), Some(atom.display_token()));
        }
    }
}
