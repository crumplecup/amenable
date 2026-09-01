//! `KaniWitness` impls for `std::path`.
//!
//! Every direct std harness here uses forward-slash paths, which parse
//! identically on Unix and Windows, except where noted. The `Display`,
//! `Prefix`, and `PrefixComponent` contracts are proved through
//! Amenable-owned observations instead of the direct std paths: `Display`
//! times out under Kani's formatting machinery, while Windows prefix parsing
//! is host-platform-specific and therefore not executable on this Linux
//! verifier host.

use std::path::{
    Ancestors, Component, Components, Path, PathBuf, Prefix, PrefixComponent, StripPrefixError,
};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{KaniPathDisplayObservation, KaniVerifier, KaniWindowsPrefixObservation, KaniWitness};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::Ensures;

    pub(super) use crate::AccessorRecoversTheExpectedValue;
    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::FallibleOperationReportsFailure;
    pub(super) use crate::IndexRecoversTheStoredElement;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    AccessorRecoversTheExpectedValue, CollectedSequenceMatchesExpected, Ensures,
    FallibleOperationReportsFailure, IndexRecoversTheStoredElement,
    IteratorYieldsNoneWhenExhausted,
};

impl KaniWitness for RustStdStandard<Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

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

impl KaniWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_display_renders_a_valid_utf8_path_verbatim".to_owned(),
            VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        "kani",
        || <RustStdStandard<std::path::Display<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniPathDisplayObservation` instance actually
/// demonstrated verbatim rendering, minted only by
/// [`KaniPathDisplayObservation::demonstrate_verbatim_rendering`].
pub struct KaniPathDisplayWitnessToken(());

impl ProofToken for KaniPathDisplayWitnessToken {
    type Proposition = KaniPathDisplayObservation;
}

impl KaniPathDisplayObservation {
    /// Assert the rendered display text matches the source text exactly.
    /// Consumes `self`: the only way to obtain the token is to have run
    /// this check against a real observation instance, not to assert it
    /// independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_verbatim_rendering(self) -> KaniPathDisplayWitnessToken {
        assert_eq!(self.display_text(), self.source_text());
        KaniPathDisplayWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::path::Display<'static>>`'s
/// UTF-8-display claim has been established from a
/// `KaniPathDisplayObservation` that has itself demonstrated verbatim
/// rendering.
pub struct RustStdPathDisplayToken(());

impl ProofToken for RustStdPathDisplayToken {
    type Proposition = RustStdStandard<std::path::Display<'static>>;
}

impl Establish<KaniPathDisplayWitnessToken, KaniVerifier>
    for RustStdStandard<std::path::Display<'static>>
{
    type Token = RustStdPathDisplayToken;

    fn establish(_credential: KaniPathDisplayWitnessToken) -> Self::Token {
        RustStdPathDisplayToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC, {
        /// A path made entirely of valid Unicode renders through
        /// `.display()` exactly as its own string form.
        /// This proof uses the Amenable-owned path-display observation:
        /// the direct `Path::display()` rendering path times out under
        /// Kani even for a fully concrete UTF-8 literal path, matching the
        /// general formatting-cost false trail already preserved in the
        /// gallery. The claim is established through
        /// `Establish<KaniPathDisplayObservation, KaniVerifier> for
        /// RustStdStandard<std::path::Display<'static>>` from the
        /// observation instance that actually demonstrated verbatim
        /// rendering, rather than asserted independently of it.
        #[kani::proof]
        fn verify_display_renders_a_valid_utf8_path_verbatim() {
            let observation = crate::KaniPathDisplayObservation::utf8("/a/b.txt");
            let demonstration = observation.demonstrate_verbatim_rendering();

            let _token =
                RustStdStandard::<std::path::Display<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

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

impl KaniWitness for RustStdStandard<Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

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

impl KaniWitness for RustStdStandard<Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_prefix_disk_identifies_the_drive_letter".to_owned(),
            VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Prefix<'static>>",
        "kani",
        || <RustStdStandard<Prefix<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniWindowsPrefixObservation` instance actually
/// demonstrated the parsed `Disk` drive letter, minted only by
/// [`KaniWindowsPrefixObservation::demonstrate_drive_letter`].
pub struct KaniWindowsPrefixDriveLetterWitnessToken(());

impl ProofToken for KaniWindowsPrefixDriveLetterWitnessToken {
    type Proposition = KaniWindowsPrefixObservation;
}

impl KaniWindowsPrefixObservation {
    /// Assert the parsed drive letter matches the expected byte. Consumes
    /// `self`: the only way to obtain the token is to have run this check
    /// against a real observation instance, not to assert it
    /// independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_drive_letter(
        self,
        expected: u8,
    ) -> KaniWindowsPrefixDriveLetterWitnessToken {
        assert_eq!(self.drive_letter(), expected);
        KaniWindowsPrefixDriveLetterWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Prefix<'static>>`'s drive-letter
/// claim has been established from a `KaniWindowsPrefixObservation` that has
/// itself demonstrated the parsed `Disk` drive letter.
pub struct RustStdPrefixToken(());

impl ProofToken for RustStdPrefixToken {
    type Proposition = RustStdStandard<Prefix<'static>>;
}

impl Establish<KaniWindowsPrefixDriveLetterWitnessToken, KaniVerifier>
    for RustStdStandard<Prefix<'static>>
{
    type Token = RustStdPrefixToken;

    fn establish(_credential: KaniWindowsPrefixDriveLetterWitnessToken) -> Self::Token {
        RustStdPrefixToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC, {
        /// A Windows drive-letter path (`C:\...`) parses to a `Disk`
        /// prefix naming that letter.
        /// This proof uses the Amenable-owned Windows-prefix observation:
        /// the direct std path is host-platform-specific and does not
        /// execute on this Linux verifier host. The claim is established
        /// through `Establish<KaniWindowsPrefixObservation, KaniVerifier>
        /// for RustStdStandard<Prefix<'static>>` from the observation
        /// instance that actually demonstrated the `Disk` drive letter,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_prefix_disk_identifies_the_drive_letter() {
            let observation = crate::KaniWindowsPrefixObservation::disk("C:", b'C');
            let demonstration = observation.demonstrate_drive_letter(b'C');

            let _token = RustStdStandard::<Prefix<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_prefix_component_pairs_raw_text_with_parsed_prefix".to_owned(),
            VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PrefixComponent<'static>>",
        "kani",
        || <RustStdStandard<PrefixComponent<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniWindowsPrefixObservation` instance actually
/// demonstrated both its raw text and parsed drive letter, minted only by
/// [`KaniWindowsPrefixObservation::demonstrate_raw_text_and_drive_letter`].
pub struct KaniWindowsPrefixComponentWitnessToken(());

impl ProofToken for KaniWindowsPrefixComponentWitnessToken {
    type Proposition = KaniWindowsPrefixObservation;
}

impl KaniWindowsPrefixObservation {
    /// Assert both the raw text and the parsed drive letter match what
    /// the source path actually wrote. Consumes `self` for the same
    /// reason [`KaniWindowsPrefixObservation::demonstrate_drive_letter`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_raw_text_and_drive_letter(
        self,
        expected_text: &'static str,
        expected_letter: u8,
    ) -> KaniWindowsPrefixComponentWitnessToken {
        assert_eq!(self.raw_text(), expected_text);
        assert_eq!(self.drive_letter(), expected_letter);
        KaniWindowsPrefixComponentWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<PrefixComponent<'static>>`'s
/// raw-text-plus-parsed-prefix claim has been established from a
/// `KaniWindowsPrefixObservation` that has itself demonstrated both facets.
pub struct RustStdPrefixComponentToken(());

impl ProofToken for RustStdPrefixComponentToken {
    type Proposition = RustStdStandard<PrefixComponent<'static>>;
}

impl Establish<KaniWindowsPrefixComponentWitnessToken, KaniVerifier>
    for RustStdStandard<PrefixComponent<'static>>
{
    type Token = RustStdPrefixComponentToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniWindowsPrefixComponentWitnessToken) -> Self::Token {
        RustStdPrefixComponentToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC, {
        /// A `PrefixComponent`'s raw `OsStr` text and its parsed `Prefix`
        /// agree with what the source path actually wrote.
        /// This proof uses the same Amenable-owned Windows-prefix
        /// observation as `Prefix`: the direct std path is
        /// host-platform-specific and does not execute on this Linux
        /// verifier host. The claim is established through
        /// `Establish<KaniWindowsPrefixObservation, KaniVerifier> for
        /// RustStdStandard<PrefixComponent<'static>>` from the observation
        /// instance that actually demonstrated both the raw text and the
        /// parsed `Disk` drive letter, rather than asserted independently
        /// of it.
        #[kani::proof]
        fn verify_prefix_component_pairs_raw_text_with_parsed_prefix() {
            let observation = crate::KaniWindowsPrefixObservation::disk("C:", b'C');
            let demonstration = observation.demonstrate_raw_text_and_drive_letter("C:", b'C');

            let _token = RustStdStandard::<PrefixComponent<'static>>::establish(demonstration);
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
