//! The `inventory`-backed registry of explicitly exported witness
//! targets, and the owned snapshots downstream tooling consumes.

use super::support::WitnessSupportSummary;
use super::tree::WitnessArtifactNode;

/// A statically registered export target for an explicitly instantiated
/// witness surface.
///
/// Unlike [`crate::ProofRecord`], which tracks every proof bridge a crate
/// already implements, this record is opt-in and concrete: callers
/// register the exact instantiated witness types they want a separate
/// backend pipeline to materialize.
///
/// Hand-written `const fn new`/getters, not derived: this record is
/// itself passed to `inventory::submit!`, which requires a
/// `const`-evaluable value, and `derive_new::new` cannot generate a
/// `const fn`.
pub struct WitnessExportRecord {
    verifier: fn() -> &'static str,
    evidence: fn() -> &'static str,
    destination_module: fn() -> &'static str,
    describe: fn() -> String,
    support: fn() -> WitnessSupportSummary,
    artifact: fn() -> WitnessArtifactNode,
}

impl WitnessExportRecord {
    /// Register an explicit witness export target.
    #[must_use]
    pub const fn new(
        verifier: fn() -> &'static str,
        evidence: fn() -> &'static str,
        destination_module: fn() -> &'static str,
        describe: fn() -> String,
        support: fn() -> WitnessSupportSummary,
        artifact: fn() -> WitnessArtifactNode,
    ) -> Self {
        Self {
            verifier,
            evidence,
            destination_module,
            describe,
            support,
            artifact,
        }
    }

    /// The verifier backend this export targets.
    #[must_use]
    pub const fn verifier(&self) -> fn() -> &'static str {
        self.verifier
    }

    /// The concrete evidence type to materialize.
    #[must_use]
    pub const fn evidence(&self) -> fn() -> &'static str {
        self.evidence
    }

    /// The backend module path where the proof content belongs.
    #[must_use]
    pub const fn destination_module(&self) -> fn() -> &'static str {
        self.destination_module
    }

    /// Render the witness artifact for audit without running a verifier.
    #[must_use]
    pub const fn describe(&self) -> fn() -> String {
        self.describe
    }

    /// Summarize the support surface the witness closes over.
    #[must_use]
    pub const fn support(&self) -> fn() -> WitnessSupportSummary {
        self.support
    }

    /// Structured witness artifact tree for backend-specific generation.
    #[must_use]
    pub const fn artifact(&self) -> fn() -> WitnessArtifactNode {
        self.artifact
    }
}

inventory::collect!(WitnessExportRecord);

/// Owned snapshot of one registered witness export target.
#[derive(
    Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_getters::Dissolve, derive_new::new,
)]
pub struct WitnessExportSnapshot {
    /// The verifier backend this export targets.
    verifier: String,
    /// The concrete evidence type to materialize.
    evidence: String,
    /// The backend module path where the proof content belongs.
    destination_module: String,
    /// Structural summary of the support surface this export closes over.
    #[getter(copy)]
    support: WitnessSupportSummary,
    /// Structured witness artifact tree for backend-specific generation.
    artifact: WitnessArtifactNode,
}

/// Collect every registered witness export target into owned values.
///
/// The result is sorted by `(verifier, evidence, destination_module)` so
/// downstream tooling and tests can consume it deterministically.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub fn witness_exports() -> Vec<WitnessExportSnapshot> {
    let mut exports: Vec<_> = inventory::iter::<WitnessExportRecord>()
        .map(|record| {
            WitnessExportSnapshot::new(
                (record.verifier())().to_owned(),
                (record.evidence())().to_owned(),
                (record.destination_module())().to_owned(),
                (record.support())(),
                (record.artifact())(),
            )
        })
        .collect();

    exports.sort_by(|left, right| {
        (
            left.verifier().as_str(),
            left.evidence().as_str(),
            left.destination_module().as_str(),
        )
            .cmp(&(
                right.verifier().as_str(),
                right.evidence().as_str(),
                right.destination_module().as_str(),
            ))
    });

    exports
}
