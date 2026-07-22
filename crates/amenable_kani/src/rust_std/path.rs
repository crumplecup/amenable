//! `KaniWitness` impls for `std::path`.
//!
//! Every harness uses forward-slash paths, which parse identically on Unix
//! and Windows, except where noted — `Prefix`/`PrefixComponent` are only
//! meaningfully exercised on Windows, since Unix path parsing never
//! produces a prefix component, so those two harnesses are additionally
//! gated on `#[cfg(windows)]`.

use std::path::{
    Ancestors, Component, Components, Path, PathBuf, Prefix, PrefixComponent, StripPrefixError,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ancestors_yields_self_then_each_parent_up_to_root".to_owned(),
            claim: VERIFY_ANCESTORS_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Ancestors<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ancestors<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Ancestors<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ANCESTORS_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC, {
        /// `.ancestors()` yields the path itself, then each parent in turn,
        /// stopping once the root is reached.
        #[kani::proof]
        fn verify_ancestors_yields_self_then_each_parent_up_to_root() {
            let path = Path::new("/a/b/c");
            let ancestors: Vec<&Path> = path.ancestors().collect();
            assert_eq!(
                ancestors,
                vec![
                    Path::new("/a/b/c"),
                    Path::new("/a/b"),
                    Path::new("/a"),
                    Path::new("/"),
                ],
                "ancestors runs from the path itself up to the root"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Component<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_component_distinguishes_root_from_normal_segments".to_owned(),
            claim: VERIFY_COMPONENT_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Component<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Component<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Component<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMPONENT_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC, {
        /// A parsed root component is `Component::RootDir`, and a named
        /// segment is `Component::Normal` wrapping that segment's text.
        #[kani::proof]
        fn verify_component_distinguishes_root_from_normal_segments() {
            let components: Vec<Component> = Path::new("/a").components().collect();
            assert_eq!(components[0], Component::RootDir);
            assert_eq!(
                components[1],
                Component::Normal(std::ffi::OsStr::new("a"))
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Components<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_components_yields_root_then_named_segments_in_order".to_owned(),
            claim: VERIFY_COMPONENTS_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Components<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Components<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Components<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMPONENTS_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC, {
        /// `.components()` yields a root component followed by each named
        /// segment, in path order.
        #[kani::proof]
        fn verify_components_yields_root_then_named_segments_in_order() {
            let components: Vec<Component> = Path::new("/a/b").components().collect();
            assert_eq!(
                components,
                vec![
                    Component::RootDir,
                    Component::Normal(std::ffi::OsStr::new("a")),
                    Component::Normal(std::ffi::OsStr::new("b")),
                ]
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_display_renders_a_valid_utf8_path_verbatim".to_owned(),
            claim: VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::path::Display<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC, {
        /// A path made entirely of valid Unicode renders through
        /// `.display()` exactly as its own string form.
        #[kani::proof]
        fn verify_display_renders_a_valid_utf8_path_verbatim() {
            let path = Path::new("/a/b.txt");
            assert_eq!(format!("{}", path.display()), "/a/b.txt");
        }
    }
}

impl KaniWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_iter_yields_the_named_segments".to_owned(),
            claim: VERIFY_ITER_YIELDS_THE_NAMED_SEGMENTS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::path::Iter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::path::Iter<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_THE_NAMED_SEGMENTS_SRC, {
        /// `.iter()` yields the path's raw `OsStr` segments in order,
        /// including the root but named segments are checked exactly
        /// (the root's own spelling is platform-dependent).
        #[kani::proof]
        fn verify_iter_yields_the_named_segments() {
            let segments: Vec<&std::ffi::OsStr> = Path::new("/a/b").iter().collect();
            assert_eq!(segments.len(), 3, "root, then two named segments");
            assert_eq!(segments[1], std::ffi::OsStr::new("a"));
            assert_eq!(segments[2], std::ffi::OsStr::new("b"));
        }
    }
}

impl KaniWitness for RustStdStandard<Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_path_derives_extension_file_name_and_parent".to_owned(),
            claim: VERIFY_PATH_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Path>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Path>",
        verifier: "kani",
        describe: || <RustStdStandard<Path> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_PATH_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC, {
        /// `.extension()`, `.file_name()`, `.parent()`, and `.has_root()`
        /// all report what the path's own text says.
        #[kani::proof]
        fn verify_path_derives_extension_file_name_and_parent() {
            let path = Path::new("/a/b/c.txt");
            assert_eq!(path.extension(), Some(std::ffi::OsStr::new("txt")));
            assert_eq!(path.file_name(), Some(std::ffi::OsStr::new("c.txt")));
            assert_eq!(path.parent(), Some(Path::new("/a/b")));
            assert!(path.has_root(), "a leading separator gives the path a root");
        }
    }
}

impl KaniWitness for RustStdStandard<PathBuf> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_path_buf_push_pop_and_join_build_the_expected_path".to_owned(),
            claim: VERIFY_PATH_BUF_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PathBuf>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PathBuf>",
        verifier: "kani",
        describe: || <RustStdStandard<PathBuf> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_PATH_BUF_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC, {
        /// `.push()` appends a segment, `.pop()` removes the last one, and
        /// `Path::join` composes a new path the same way.
        #[kani::proof]
        fn verify_path_buf_push_pop_and_join_build_the_expected_path() {
            let mut built = PathBuf::from("/a");
            built.push("b");
            built.push("c.txt");
            assert_eq!(built.as_path(), Path::new("/a/b/c.txt"));

            assert!(built.pop(), "pop removes the last pushed segment");
            assert_eq!(built.as_path(), Path::new("/a/b"));

            let joined = Path::new("/a").join("b").join("c.txt");
            assert_eq!(joined, PathBuf::from("/a/b/c.txt"));
        }
    }
}

impl KaniWitness for RustStdStandard<Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_prefix_disk_identifies_the_drive_letter".to_owned(),
            claim: VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Prefix<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Prefix<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC, {
        /// A Windows drive-letter path (`C:\...`) parses to a `Disk`
        /// prefix naming that letter. Windows-only: Unix path parsing
        /// never produces a prefix component at all.
        #[cfg(windows)]
        #[kani::proof]
        fn verify_prefix_disk_identifies_the_drive_letter() {
            let path = Path::new(r"C:\foo");
            match path.components().next() {
                Some(Component::Prefix(prefix_component)) => {
                    match prefix_component.kind() {
                        Prefix::Disk(letter) => assert_eq!(letter, b'C'),
                        other => panic!("expected a Disk prefix, got {other:?}"),
                    }
                }
                other => panic!("expected a Prefix component, got {other:?}"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_prefix_component_pairs_raw_text_with_parsed_prefix".to_owned(),
            claim: VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PrefixComponent<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<PrefixComponent<'static>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC, {
        /// A `PrefixComponent`'s raw `OsStr` text and its parsed `Prefix`
        /// agree with what the source path actually wrote. Windows-only,
        /// for the same reason as the `Prefix` harness above.
        #[cfg(windows)]
        #[kani::proof]
        fn verify_prefix_component_pairs_raw_text_with_parsed_prefix() {
            let path = Path::new(r"C:\foo");
            match path.components().next() {
                Some(Component::Prefix(prefix_component)) => {
                    assert_eq!(prefix_component.as_os_str(), std::ffi::OsStr::new("C:"));
                    assert_eq!(prefix_component.kind(), Prefix::Disk(b'C'));
                }
                other => panic!("expected a Prefix component, got {other:?}"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<StripPrefixError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_strip_prefix_error_reports_a_non_matching_prefix".to_owned(),
            claim: VERIFY_STRIP_PREFIX_ERROR_REPORTS_A_NON_MATCHING_PREFIX_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<StripPrefixError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<StripPrefixError>",
        verifier: "kani",
        describe: || <RustStdStandard<StripPrefixError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRIP_PREFIX_ERROR_REPORTS_A_NON_MATCHING_PREFIX_SRC, {
        /// `.strip_prefix()` fails when the path doesn't actually start
        /// with the given prefix, and succeeds (producing no error) when
        /// it does.
        #[kani::proof]
        fn verify_strip_prefix_error_reports_a_non_matching_prefix() {
            assert!(
                Path::new("/a/b").strip_prefix("/x").is_err(),
                "strip_prefix fails on a non-matching prefix"
            );
            assert_eq!(
                Path::new("/a/b").strip_prefix("/a").unwrap(),
                Path::new("b"),
                "strip_prefix succeeds and removes a matching prefix"
            );
        }
    }
}
