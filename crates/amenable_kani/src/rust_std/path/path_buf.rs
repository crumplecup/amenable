//! `KaniWitness` impls and harnesses for the owned/borrowed path types
//! themselves: `Path`, `PathBuf`, and `StripPrefixError`, plus the two
//! raw-boolean claim types (`PathHasRootReportsTrue`, `PopRemovedASegment`)
//! their harnesses reuse.

use std::path::{Path, PathBuf, StripPrefixError};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};
#[cfg(kani)]
use crate::{AccessorRecoversTheExpectedValue, FallibleOperationReportsFailure};

impl KaniWitness for RustStdStandard<Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_path_derives_extension_file_name_and_parent".to_owned(),
            VERIFY_PATH_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Path>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Path>",
        "kani",
        || <RustStdStandard<Path> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `Path::has_root()` reports when the
/// path's text actually starts with a root separator -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct PathHasRootReportsTrue;

impl KaniWitness for PathHasRootReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_path_derives_extension_file_name_and_parent".to_owned(),
            VERIFY_PATH_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(PathHasRootReportsTrue);

kani_ensures!(
    PathHasRootReportsTrue,
    "amenable_kani::PathHasRootReportsTrue",
    bool,
    |has_root| has_root
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PathHasRootReportsTrue",
        "kani",
        || <PathHasRootReportsTrue as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PATH_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC, {
        /// `.extension()`, `.file_name()`, `.parent()`, and `.has_root()`
        /// all report what the path's own text says.
        #[kani::proof]
        fn verify_path_derives_extension_file_name_and_parent() {
            let path = Path::new("/a/b/c.txt");
            assert!(AccessorRecoversTheExpectedValue::ensures((
                path.extension(),
                Some(std::ffi::OsStr::new("txt"))
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                path.file_name(),
                Some(std::ffi::OsStr::new("c.txt"))
            )));
            assert!(AccessorRecoversTheExpectedValue::ensures((
                path.parent(),
                Some(Path::new("/a/b"))
            )));
            assert!(
                PathHasRootReportsTrue::ensures(path.has_root()),
                "a leading separator gives the path a root"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<PathBuf> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_path_buf_push_pop_and_join_build_the_expected_path".to_owned(),
            VERIFY_PATH_BUF_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PathBuf>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PathBuf>",
        "kani",
        || <RustStdStandard<PathBuf> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<PathBuf>,
    "amenable_std::rust_std::RustStdStandard<PathBuf>",
    (PathBuf, PathBuf),
    |(actual, expected)| actual == expected
);

/// A `bool` known to be the `true` `PathBuf::pop()` reports when it
/// actually removed the last component -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim, but a distinct claim from it: this is about the
/// pop *operation's own outcome*, not the container's emptiness
/// afterward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct PopRemovedASegment;

impl KaniWitness for PopRemovedASegment {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_path_buf_push_pop_and_join_build_the_expected_path".to_owned(),
            VERIFY_PATH_BUF_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(PopRemovedASegment);

kani_ensures!(
    PopRemovedASegment,
    "amenable_kani::PopRemovedASegment",
    bool,
    |removed| removed
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PopRemovedASegment",
        "kani",
        || <PopRemovedASegment as KaniWitness>::proof().to_string(),
    )
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
            assert!(RustStdStandard::<PathBuf>::ensures((
                built.clone(),
                PathBuf::from("/a/b/c.txt")
            )));

            assert!(
                PopRemovedASegment::ensures(built.pop()),
                "pop removes the last pushed segment"
            );
            assert!(RustStdStandard::<PathBuf>::ensures((
                built.clone(),
                PathBuf::from("/a/b")
            )));

            let joined = Path::new("/a").join("b").join("c.txt");
            assert!(RustStdStandard::<PathBuf>::ensures((
                joined,
                PathBuf::from("/a/b/c.txt")
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<StripPrefixError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_strip_prefix_error_reports_a_non_matching_prefix".to_owned(),
            VERIFY_STRIP_PREFIX_ERROR_REPORTS_A_NON_MATCHING_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<StripPrefixError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<StripPrefixError>",
        "kani",
        || <RustStdStandard<StripPrefixError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_STRIP_PREFIX_ERROR_REPORTS_A_NON_MATCHING_PREFIX_SRC, {
        /// `.strip_prefix()` fails when the path doesn't actually start
        /// with the given prefix, and succeeds (producing no error) when
        /// it does.
        #[kani::proof]
        fn verify_strip_prefix_error_reports_a_non_matching_prefix() {
            assert!(
                FallibleOperationReportsFailure::ensures(
                    Path::new("/a/b").strip_prefix("/x").is_err()
                ),
                "strip_prefix fails on a non-matching prefix"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((
                    Path::new("/a/b").strip_prefix("/a").unwrap(),
                    Path::new("b")
                )),
                "strip_prefix succeeds and removes a matching prefix"
            );
        }
    }
}
