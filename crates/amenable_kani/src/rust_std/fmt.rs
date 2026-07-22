//! `KaniWitness` impls for `core::fmt`.
//!
//! `Error` is an opaque unit-like marker signaling that a formatting trait
//! impl failed to write to its `Formatter` — no accessors beyond `Debug`/
//! `Display`, nothing to build and check. It stays at the trusted
//! disposition.

use core::fmt::{Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Formatter};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<core::fmt::Alignment> {
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

bridge_kani_witness!(RustStdStandard<core::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::fmt::Alignment>",
        verifier: "kani",
        describe: || <RustStdStandard<core::fmt::Alignment> as KaniWitness>::proof().to_string(),
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
        #[kani::proof]
        fn verify_arguments_renders_the_same_as_the_value_itself() {
            let value: i32 = kani::any();
            let args = format_args!("{}", value);
            assert_eq!(
                args.to_string(),
                value.to_string(),
                "Arguments renders the same as the value's own Display"
            );
        }
    }
}

impl_kani_witness_trusted!(core::fmt::Error);

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
        #[kani::proof]
        fn verify_debug_struct_renders_named_fields() {
            struct Probe(i32);
            impl std::fmt::Debug for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct("Probe").field("x", &self.0).finish()
                }
            }
            let value: i32 = kani::any();
            let out = format!("{:?}", Probe(value));
            assert_eq!(out, format!("Probe {{ x: {} }}", value));
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
            struct Probe(i32);
            impl std::fmt::Debug for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_tuple("Probe").field(&self.0).finish()
                }
            }
            let value: i32 = kani::any();
            let out = format!("{:?}", Probe(value));
            assert_eq!(out, format!("Probe({})", value));
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
            struct Probe(i32, i32);
            impl std::fmt::Debug for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_list().entries([self.0, self.1]).finish()
                }
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let out = format!("{:?}", Probe(a, b));
            assert_eq!(out, format!("[{}, {}]", a, b));
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
            struct Probe(i32, i32);
            impl std::fmt::Debug for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_set().entries([self.0, self.1]).finish()
                }
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let out = format!("{:?}", Probe(a, b));
            assert_eq!(out, format!("{{{}, {}}}", a, b));
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
            struct Probe(i32);
            impl std::fmt::Debug for Probe {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_map().entry(&"k", &self.0).finish()
                }
            }
            let value: i32 = kani::any();
            let out = format!("{:?}", Probe(value));
            assert_eq!(out, format!("{{\"k\": {}}}", value));
        }
    }
}
