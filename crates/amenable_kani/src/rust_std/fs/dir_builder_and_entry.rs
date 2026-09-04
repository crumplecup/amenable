use std::fs::{DirBuilder, DirEntry};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniDirEntryObservation, KaniRecursiveDirObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<DirBuilder> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_dir_builder_creates_nested_directories_recursively".to_owned(),
            VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DirBuilder>",
        "kani",
        || <RustStdStandard<DirBuilder> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniRecursiveDirObservation` instance actually
/// demonstrated the ancestor-preserving join law, minted only by
/// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`].
pub struct KaniRecursiveDirWitnessToken(());

impl ProofToken for KaniRecursiveDirWitnessToken {
    type Proposition = KaniRecursiveDirObservation;
}

impl KaniRecursiveDirObservation {
    /// Assert the observation's ancestors and leaf match repeated joins
    /// from `base`. Consumes `self`: the only way to obtain the token is
    /// to have run this check against a real observation instance, not to
    /// assert it independently.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, base, first, second, leaf))
    )]
    #[must_use]
    pub fn demonstrate_ancestor_preservation(
        self,
        base: crate::KaniFsPath,
        first: crate::KaniFsLabel,
        second: crate::KaniFsLabel,
        leaf: crate::KaniFsLabel,
    ) -> KaniRecursiveDirWitnessToken {
        assert_eq!(
            self.first_ancestor(),
            base.join(first),
            "recursive creation preserves the first ancestor"
        );
        assert_eq!(
            self.second_ancestor(),
            base.join(first).join(second),
            "recursive creation preserves the second ancestor"
        );
        assert_eq!(
            self.leaf(),
            base.join(first).join(second).join(leaf),
            "recursive creation preserves the leaf"
        );
        KaniRecursiveDirWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<DirBuilder>`'s recursive
/// directory-creation claim has been established from a
/// `KaniRecursiveDirObservation` that has itself demonstrated the
/// ancestor-preserving join law.
pub struct RustStdDirBuilderRecursiveToken(());

impl ProofToken for RustStdDirBuilderRecursiveToken {
    type Proposition = RustStdStandard<DirBuilder>;
}

impl Establish<KaniRecursiveDirWitnessToken, KaniVerifier> for RustStdStandard<DirBuilder> {
    type Token = RustStdDirBuilderRecursiveToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniRecursiveDirWitnessToken) -> Self::Token {
        RustStdDirBuilderRecursiveToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC, {
        /// `.recursive(true)` creates every missing ancestor directory,
        /// not just the leaf.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `DirBuilder` recursive path refines these directory-creation laws,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniRecursiveDirObservation, KaniVerifier> for
        /// RustStdStandard<DirBuilder>` from the observation instance that
        /// actually demonstrated the ancestor-preserving join law, rather
        /// than asserted independently of it.
        #[kani::proof]
        fn verify_dir_builder_creates_nested_directories_recursively() {
            let base = crate::KaniFsPath::root();
            let a = crate::KaniFsLabel::new('a');
            let b = crate::KaniFsLabel::new('b');
            let c = crate::KaniFsLabel::new('c');
            let observation = crate::KaniRecursiveDirObservation::new(base, a, b, c);
            let demonstration = observation.demonstrate_ancestor_preservation(base, a, b, c);

            let _token = RustStdStandard::<DirBuilder>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<DirEntry> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_dir_entry_reports_the_created_files_name_and_path".to_owned(),
            VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DirEntry>",
        "kani",
        || <RustStdStandard<DirEntry> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniDirEntryObservation` instance actually demonstrated
/// entry identity is preserved exactly, minted only by
/// [`KaniDirEntryObservation::demonstrate_identity`].
pub struct KaniDirEntryWitnessToken(());

impl ProofToken for KaniDirEntryWitnessToken {
    type Proposition = KaniDirEntryObservation;
}

impl KaniDirEntryObservation {
    /// Assert the entry reports the expected name and path. Consumes
    /// `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, expected_name, expected_path))
    )]
    #[must_use]
    pub fn demonstrate_identity(
        self,
        expected_name: crate::KaniFsLabel,
        expected_path: crate::KaniFsPath,
    ) -> KaniDirEntryWitnessToken {
        let entry = self.entry();
        assert_eq!(entry.file_name(), Some(expected_name));
        assert_eq!(entry.path(), expected_path);
        KaniDirEntryWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<DirEntry>`'s name/path
/// reporting claim has been established from a `KaniDirEntryObservation`
/// that has itself demonstrated entry identity is preserved exactly.
pub struct RustStdDirEntryToken(());

impl ProofToken for RustStdDirEntryToken {
    type Proposition = RustStdStandard<DirEntry>;
}

impl Establish<KaniDirEntryWitnessToken, KaniVerifier> for RustStdStandard<DirEntry> {
    type Token = RustStdDirEntryToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniDirEntryWitnessToken) -> Self::Token {
        RustStdDirEntryToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC, {
        /// A `DirEntry` yielded for a created file reports that file's
        /// own name and full path.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` / `DirEntry` path preserves entry identity this way, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniDirEntryObservation, KaniVerifier> for
        /// RustStdStandard<DirEntry>` from the observation instance that
        /// actually demonstrated that identity, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_dir_entry_reports_the_created_files_name_and_path() {
            let base = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('b'));
            let name = crate::KaniFsLabel::new('f');
            let path = base.join(name);
            let observation = crate::KaniDirEntryObservation::new(base, name);
            let demonstration = observation.demonstrate_identity(name, path);

            let _token = RustStdStandard::<DirEntry>::establish(demonstration);
        }
    }
}
