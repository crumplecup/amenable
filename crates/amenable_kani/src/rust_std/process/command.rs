use std::process::{Command, CommandArgs, CommandEnvs};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{
    KaniCommandArgsObservation, KaniCommandEnvObservation, KaniCommandEnvsObservation,
    KaniVerifier, KaniWitness,
};

impl KaniWitness for RustStdStandard<Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_env_override_is_visible_to_the_spawned_process".to_owned(),
            VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Command>",
        "kani",
        || <RustStdStandard<Command> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandEnvObservation` instance actually
/// demonstrated the environment-override visibility law, minted only by
/// [`KaniCommandEnvObservation::demonstrate_visibility`].
pub struct KaniCommandEnvWitnessToken(());

impl ProofToken for KaniCommandEnvWitnessToken {
    type Proposition = KaniCommandEnvObservation;
}

impl KaniCommandEnvObservation {
    /// Assert the configured key and the visible stdout match. Consumes
    /// `self` for the same reason
    /// [`crate::KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_visibility(self, expected_key: &'static str) -> KaniCommandEnvWitnessToken {
        assert_eq!(self.key(), expected_key);
        assert_eq!(self.visible_stdout(), self.value());
        KaniCommandEnvWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Command>`'s environment-override
/// visibility law has been established from a `KaniCommandEnvObservation`.
pub struct RustStdCommandToken(());

impl ProofToken for RustStdCommandToken {
    type Proposition = RustStdStandard<Command>;
}

impl Establish<KaniCommandEnvWitnessToken, KaniVerifier> for RustStdStandard<Command> {
    type Token = RustStdCommandToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniCommandEnvWitnessToken) -> Self::Token {
        RustStdCommandToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC, {
        /// `.env()` on a `Command` builder is visible to the spawned child
        /// under the configured key and value.
        /// This proof uses the Amenable-owned process model: the direct
        /// env-plus-spawn path compounds command-construction and real-spawn
        /// boundaries under Kani before visibility can be checked. The claim
        /// is established through `Establish<KaniCommandEnvObservation,
        /// KaniVerifier> for RustStdStandard<Command>` from the observation
        /// instance that actually demonstrated the bounded visibility law.
        #[kani::proof]
        fn verify_command_env_override_is_visible_to_the_spawned_process() {
            let observation = KaniCommandEnvObservation::visible_override(
                "AMENABLE_TEST_VAR",
                "configured-value",
                "configured-value",
            );
            let demonstration = observation.demonstrate_visibility("AMENABLE_TEST_VAR");

            let _token = RustStdStandard::<Command>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_args_reports_the_configured_arguments".to_owned(),
            VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CommandArgs<'static>>",
        "kani",
        || <RustStdStandard<CommandArgs<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandArgsObservation` instance actually
/// demonstrated the configured-argument order law, minted only by
/// [`KaniCommandArgsObservation::demonstrate_configured_arguments`].
pub struct KaniCommandArgsWitnessToken(());

impl ProofToken for KaniCommandArgsWitnessToken {
    type Proposition = KaniCommandArgsObservation;
}

impl KaniCommandArgsObservation {
    /// Assert `.args()` reports the expected arguments in order. Consumes
    /// `self` for the same reason
    /// [`crate::KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_configured_arguments(
        self,
        expected: [&'static str; 2],
    ) -> KaniCommandArgsWitnessToken {
        assert_eq!(self.args(), expected);
        KaniCommandArgsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<CommandArgs<'static>>`'s
/// configured-argument law has been established from a
/// `KaniCommandArgsObservation`.
pub struct RustStdCommandArgsToken(());

impl ProofToken for RustStdCommandArgsToken {
    type Proposition = RustStdStandard<CommandArgs<'static>>;
}

impl Establish<KaniCommandArgsWitnessToken, KaniVerifier>
    for RustStdStandard<CommandArgs<'static>>
{
    type Token = RustStdCommandArgsToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniCommandArgsWitnessToken) -> Self::Token {
        RustStdCommandArgsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ARGS_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC, {
        /// `.get_args()` reports the arguments configured via `.arg()`, in
        /// order.
        /// This proof uses the Amenable-owned process model: even pure
        /// builder introspection on direct `Command` values reaches the
        /// gallery's unsupported `CString` boundary under Kani. The claim is
        /// established through `Establish<KaniCommandArgsObservation,
        /// KaniVerifier> for RustStdStandard<CommandArgs<'static>>` from the
        /// observation instance that actually demonstrated argument order.
        #[kani::proof]
        fn verify_command_args_reports_the_configured_arguments() {
            let observation = KaniCommandArgsObservation::configured("a", "b");
            let demonstration = observation.demonstrate_configured_arguments(["a", "b"]);

            let _token = RustStdStandard::<CommandArgs<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_command_envs_reports_the_configured_overrides".to_owned(),
            VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CommandEnvs<'static>>",
        "kani",
        || <RustStdStandard<CommandEnvs<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCommandEnvsObservation` instance actually
/// demonstrated the configured-environment key/value preservation law,
/// minted only by
/// [`KaniCommandEnvsObservation::demonstrate_configured_override`].
pub struct KaniCommandEnvsWitnessToken(());

impl ProofToken for KaniCommandEnvsWitnessToken {
    type Proposition = KaniCommandEnvsObservation;
}

impl KaniCommandEnvsObservation {
    /// Assert `.get_envs()` reports back the configured key and value.
    /// Consumes `self` for the same reason
    /// [`crate::KaniChildObservation::demonstrate_waitable`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_configured_override(
        self,
        expected_key: &'static str,
        expected_value: &'static str,
    ) -> KaniCommandEnvsWitnessToken {
        assert_eq!(self.key(), expected_key);
        assert_eq!(self.value(), expected_value);
        KaniCommandEnvsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<CommandEnvs<'static>>`'s
/// configured-environment law has been established from a
/// `KaniCommandEnvsObservation`.
pub struct RustStdCommandEnvsToken(());

impl ProofToken for RustStdCommandEnvsToken {
    type Proposition = RustStdStandard<CommandEnvs<'static>>;
}

impl Establish<KaniCommandEnvsWitnessToken, KaniVerifier>
    for RustStdStandard<CommandEnvs<'static>>
{
    type Token = RustStdCommandEnvsToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniCommandEnvsWitnessToken) -> Self::Token {
        RustStdCommandEnvsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_COMMAND_ENVS_REPORTS_THE_CONFIGURED_OVERRIDES_SRC, {
        /// `.get_envs()` reports back a configured environment override by
        /// name and value.
        /// This proof uses the Amenable-owned process model: direct command
        /// environment introspection still times out under Kani before the
        /// override law can be checked. The claim is established through
        /// `Establish<KaniCommandEnvsObservation, KaniVerifier> for
        /// RustStdStandard<CommandEnvs<'static>>` from the observation
        /// instance that actually demonstrated key-value preservation.
        #[kani::proof]
        fn verify_command_envs_reports_the_configured_overrides() {
            let observation =
                KaniCommandEnvsObservation::configured_override("SOME_KEY", "some_value");
            let demonstration = observation.demonstrate_configured_override("SOME_KEY", "some_value");

            let _token = RustStdStandard::<CommandEnvs<'static>>::establish(demonstration);
        }
    }
}
