use std::fs::{OpenOptions, Permissions};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniCreateNewObservation, KaniPermissionsObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_open_options_create_new_rejects_an_existing_file".to_owned(),
            VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OpenOptions>",
        "kani",
        || <RustStdStandard<OpenOptions> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCreateNewObservation` instance actually demonstrated
/// a successful creation against a fresh path, having also confirmed the
/// same operation rejects an already-occupied one, minted only by
/// [`KaniCreateNewObservation::demonstrate_creation_succeeds`].
pub struct KaniCreateNewWitnessToken(());

impl ProofToken for KaniCreateNewWitnessToken {
    type Proposition = KaniCreateNewObservation;
}

impl KaniCreateNewObservation {
    /// Assert `.create_new()` fails against `existing` and
    /// `existing_directory`, then assert it succeeds against `self` (a
    /// fresh path) and leaves a file there. Consumes all three: the only
    /// way to obtain the token is to have run this check against real
    /// observation instances, not to assert it independently.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(self, existing, existing_directory))
    )]
    #[must_use]
    pub fn demonstrate_creation_succeeds(
        mut self,
        mut existing: KaniCreateNewObservation,
        mut existing_directory: KaniCreateNewObservation,
    ) -> KaniCreateNewWitnessToken {
        assert!(
            existing.create_new().is_err(),
            "create_new fails against a path that already has a file"
        );
        assert!(
            existing_directory.create_new().is_err(),
            "create_new also fails when the path already names a directory"
        );

        assert!(
            self.create_new().is_ok(),
            "create_new succeeds against a genuinely fresh path"
        );
        assert!(
            self.is_file(),
            "a successful create_new leaves a file at the created path"
        );
        KaniCreateNewWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<OpenOptions>`'s `create_new`
/// existence-check claim has been established from a `KaniCreateNewObservation`
/// that has itself demonstrated the successful-creation transition.
pub struct RustStdOpenOptionsCreateNewToken(());

impl ProofToken for RustStdOpenOptionsCreateNewToken {
    type Proposition = RustStdStandard<OpenOptions>;
}

impl Establish<KaniCreateNewWitnessToken, KaniVerifier> for RustStdStandard<OpenOptions> {
    type Token = RustStdOpenOptionsCreateNewToken;

    fn establish(_credential: KaniCreateNewWitnessToken) -> Self::Token {
        RustStdOpenOptionsCreateNewToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC, {
        /// `.create_new(true)` fails with `AlreadyExists` on a path that
        /// already has a file, and succeeds on a genuinely fresh one.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `OpenOptions::create_new` path preserves this existence check,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniCreateNewObservation, KaniVerifier> for
        /// RustStdStandard<OpenOptions>` from the observation instance that
        /// actually demonstrated a successful creation, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_open_options_create_new_rejects_an_existing_file() {
            let existing = crate::KaniCreateNewObservation::existing_file();
            let existing_directory = crate::KaniCreateNewObservation::existing_directory();
            let fresh = crate::KaniCreateNewObservation::missing();
            let demonstration = fresh.demonstrate_creation_succeeds(existing, existing_directory);

            let _token = RustStdStandard::<OpenOptions>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_permissions_readonly_round_trips_through_set_permissions".to_owned(),
            VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Permissions>",
        "kani",
        || <RustStdStandard<Permissions> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniPermissionsObservation` instance actually
/// demonstrated the readonly round trip in both directions, minted only by
/// [`KaniPermissionsObservation::demonstrate_readonly_round_trip`].
pub struct KaniPermissionsWitnessToken(());

impl ProofToken for KaniPermissionsWitnessToken {
    type Proposition = KaniPermissionsObservation;
}

impl KaniPermissionsObservation {
    /// Assert a fresh file isn't readonly, then assert setting and
    /// clearing readonly are each reflected the next time permissions are
    /// read. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_readonly_round_trip(mut self) -> KaniPermissionsWitnessToken {
        assert!(!self.readonly(), "a freshly created file is not readonly");

        self = self.with_readonly(true);
        assert!(
            self.readonly(),
            "setting readonly is reflected the next time permissions are read"
        );

        self = self.with_readonly(false);
        assert!(!self.readonly(), "clearing readonly is reflected as well");
        KaniPermissionsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Permissions>`'s readonly
/// round-trip claim has been established from a `KaniPermissionsObservation`
/// that has itself demonstrated the round trip in both directions.
pub struct RustStdPermissionsToken(());

impl ProofToken for RustStdPermissionsToken {
    type Proposition = RustStdStandard<Permissions>;
}

impl Establish<KaniPermissionsWitnessToken, KaniVerifier> for RustStdStandard<Permissions> {
    type Token = RustStdPermissionsToken;

    fn establish(_credential: KaniPermissionsWitnessToken) -> Self::Token {
        RustStdPermissionsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC, {
        /// Flipping `.set_readonly(true)` and applying it via
        /// `fs::set_permissions` is reflected the next time the file's
        /// permissions are read.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_permissions` / `metadata().permissions()` path preserves
        /// the readonly bit this way, the Rust-facing claim follows. The
        /// claim is established through `Establish<KaniPermissionsObservation,
        /// KaniVerifier> for RustStdStandard<Permissions>` from the
        /// observation instance that actually demonstrated the round trip
        /// in both directions, rather than asserted independently of it.
        #[kani::proof]
        fn verify_permissions_readonly_round_trips_through_set_permissions() {
            let observation = crate::KaniPermissionsObservation::new();
            let demonstration = observation.demonstrate_readonly_round_trip();

            let _token = RustStdStandard::<Permissions>::establish(demonstration);
        }
    }
}
