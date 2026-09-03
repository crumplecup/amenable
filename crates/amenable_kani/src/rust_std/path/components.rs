//! `KaniWitness` impls and harnesses for the path-traversal iterators:
//! `Ancestors`, `Component`, `Components`, and `Iter`. Every harness uses
//! forward-slash paths, which parse identically on Unix and Windows.

#[cfg(kani)]
use std::path::Path;
use std::path::{Ancestors, Component, Components};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
#[cfg(kani)]
use crate::{
    CollectedSequenceMatchesExpected, IndexRecoversTheStoredElement,
    IteratorYieldsNoneWhenExhausted,
};

impl KaniWitness for RustStdStandard<Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ancestors_yields_self_then_each_parent_up_to_root".to_owned(),
            VERIFY_ANCESTORS_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Ancestors<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ancestors<'static>>",
        "kani",
        || <RustStdStandard<Ancestors<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ANCESTORS_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC, {
        /// `.ancestors()` yields the path itself, then each parent in turn,
        /// stopping once the root is reached.
        #[kani::proof]
        fn verify_ancestors_yields_self_then_each_parent_up_to_root() {
            let path = Path::new("/a/b/c");
            let ancestors: Vec<&Path> = path.ancestors().collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    ancestors,
                    vec![
                        Path::new("/a/b/c"),
                        Path::new("/a/b"),
                        Path::new("/a"),
                        Path::new("/"),
                    ]
                )),
                "ancestors runs from the path itself up to the root"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Component<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_component_distinguishes_root_from_normal_segments".to_owned(),
            VERIFY_COMPONENT_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Component<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Component<'static>>",
        "kani",
        || <RustStdStandard<Component<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_COMPONENT_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC, {
        /// A parsed root component is `Component::RootDir`, and a named
        /// segment is `Component::Normal` wrapping that segment's text.
        #[kani::proof]
        fn verify_component_distinguishes_root_from_normal_segments() {
            let mut components = Path::new("/a").components();
            assert!(RustStdStandard::<Components<'static>>::ensures((
                components.next(),
                Some(Component::RootDir)
            )));
            assert!(RustStdStandard::<Components<'static>>::ensures((
                components.next(),
                Some(Component::Normal(std::ffi::OsStr::new("a")))
            )));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(components.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<Components<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_components_yields_root_then_named_segments_in_order".to_owned(),
            VERIFY_COMPONENTS_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Components<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Components<'static>>",
        "kani",
        || <RustStdStandard<Components<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Components<'static>>,
    "amenable_std::rust_std::RustStdStandard<Components<'static>>",
    (Option<Component<'static>>, Option<Component<'static>>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_COMPONENTS_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC, {
        /// `.components()` yields a root component followed by each named
        /// segment, in path order.
        /// Observed incrementally via `.next()` rather than `.collect()`
        /// into a `Vec`: confirmed the eager-collection form times out even
        /// for this fully concrete literal path, matching the
        /// materialization-cost pattern already documented in
        /// `gallery::iter_materialization`.
        #[kani::proof]
        fn verify_components_yields_root_then_named_segments_in_order() {
            let mut components = Path::new("/a/b").components();
            assert!(RustStdStandard::<Components<'static>>::ensures((
                components.next(),
                Some(Component::RootDir)
            )));
            assert!(RustStdStandard::<Components<'static>>::ensures((
                components.next(),
                Some(Component::Normal(std::ffi::OsStr::new("a")))
            )));
            assert!(RustStdStandard::<Components<'static>>::ensures((
                components.next(),
                Some(Component::Normal(std::ffi::OsStr::new("b")))
            )));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(components.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_iter_yields_the_named_segments".to_owned(),
            VERIFY_ITER_YIELDS_THE_NAMED_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::path::Iter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
        "kani",
        || <RustStdStandard<std::path::Iter<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::path::Iter<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_THE_NAMED_SEGMENTS_SRC, {
        /// `.iter()` yields the path's raw `OsStr` segments in order,
        /// including the root but named segments are checked exactly
        /// (the root's own spelling is platform-dependent).
        #[kani::proof]
        fn verify_iter_yields_the_named_segments() {
            let segments: Vec<&std::ffi::OsStr> = Path::new("/a/b").iter().collect();
            assert!(
                RustStdStandard::<std::path::Iter<'static>>::ensures((segments.len(), 3)),
                "root, then two named segments"
            );
            assert!(IndexRecoversTheStoredElement::ensures((
                segments[1],
                std::ffi::OsStr::new("a")
            )));
            assert!(IndexRecoversTheStoredElement::ensures((
                segments[2],
                std::ffi::OsStr::new("b")
            )));
        }
    }
}
