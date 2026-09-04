use core::fmt::Formatter;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<std::fmt::Alignment> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_alignment_reaches_the_formatter_from_the_format_spec".to_owned(),
            VERIFY_ALIGNMENT_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Alignment>",
        "kani",
        || <RustStdStandard<std::fmt::Alignment> as KaniWitness>::proof().to_string(),
    )
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
                    assert!(
                        AccessorRecoversTheExpectedValue::ensures((
                            f.align(),
                            Some(core::fmt::Alignment::Left)
                        )),
                        "the spec's alignment reaches the Formatter"
                    );
                    write!(f, "x")
                }
            }
            let _ = format!("{:<5}", Probe);
        }
    }
}

impl_kani_witness_trusted!(std::fmt::Error);

impl KaniWitness for RustStdStandard<Formatter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_formatter_exposes_the_parsed_width_and_precision".to_owned(),
            VERIFY_FORMATTER_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Formatter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Formatter<'static>>",
        "kani",
        || <RustStdStandard<Formatter<'static>> as KaniWitness>::proof().to_string(),
    )
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
                    assert!(
                        AccessorRecoversTheExpectedValue::ensures((f.width(), Some(10))),
                        "the spec's width reaches the Formatter"
                    );
                    assert!(
                        AccessorRecoversTheExpectedValue::ensures((f.precision(), Some(2))),
                        "the spec's precision reaches the Formatter"
                    );
                    write!(f, "x")
                }
            }
            let _ = format!("{:10.2}", Probe);
        }
    }
}
