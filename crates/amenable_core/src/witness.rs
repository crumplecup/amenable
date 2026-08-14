//! Verifier-facing proof-emission roles.

use crate::{Evidence, Verifier};

/// Coarse support class for a witness surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WitnessSupportKind {
    /// The witness closes by definitional identity or shape alone.
    Trivial,
    /// The witness is backed by machine-checked proof content.
    Checked,
    /// The witness rests on an explicit trusted or provenance-backed root.
    Trusted,
    /// The witness combines checked and trusted support.
    Mixed,
    /// The witness has not classified its support surface yet.
    #[default]
    Opaque,
}

impl WitnessSupportKind {
    /// Stable label for audit reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trivial => "trivial",
            Self::Checked => "checked",
            Self::Trusted => "trusted",
            Self::Mixed => "mixed",
            Self::Opaque => "opaque",
        }
    }
}

/// Structural summary of the support a witness artifact closes over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WitnessSupportSummary {
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
}

impl WitnessSupportSummary {
    /// One trivial leaf.
    pub const fn trivial_leaf() -> Self {
        Self {
            trivial: 1,
            checked: 0,
            trusted: 0,
            opaque: 0,
        }
    }

    /// One machine-checked leaf.
    pub const fn checked_leaf() -> Self {
        Self {
            trivial: 0,
            checked: 1,
            trusted: 0,
            opaque: 0,
        }
    }

    /// One trusted leaf.
    pub const fn trusted_leaf() -> Self {
        Self {
            trivial: 0,
            checked: 0,
            trusted: 1,
            opaque: 0,
        }
    }

    /// One unclassified leaf.
    pub const fn opaque_leaf() -> Self {
        Self {
            trivial: 0,
            checked: 0,
            trusted: 0,
            opaque: 1,
        }
    }

    /// Combine the support surface from child witnesses.
    ///
    /// An empty product or unit variant is itself trivial.
    pub fn compose(parts: &[Self]) -> Self {
        if parts.is_empty() {
            return Self::trivial_leaf();
        }

        parts
            .iter()
            .copied()
            .fold(Self::default(), |acc, part| Self {
                trivial: acc.trivial + part.trivial,
                checked: acc.checked + part.checked,
                trusted: acc.trusted + part.trusted,
                opaque: acc.opaque + part.opaque,
            })
    }

    /// Overall support kind after collapsing the child counts.
    pub const fn kind(self) -> WitnessSupportKind {
        if self.opaque > 0 {
            return WitnessSupportKind::Opaque;
        }

        if self.checked > 0 && self.trusted > 0 {
            return WitnessSupportKind::Mixed;
        }

        if self.checked > 0 {
            return WitnessSupportKind::Checked;
        }

        if self.trusted > 0 {
            return WitnessSupportKind::Trusted;
        }

        WitnessSupportKind::Trivial
    }

    /// Number of trivial leaves in this support summary.
    pub const fn trivial(self) -> usize {
        self.trivial
    }

    /// Number of checked leaves in this support summary.
    pub const fn checked(self) -> usize {
        self.checked
    }

    /// Number of trusted leaves in this support summary.
    pub const fn trusted(self) -> usize {
        self.trusted
    }

    /// Number of opaque leaves in this support summary.
    pub const fn opaque(self) -> usize {
        self.opaque
    }
}

impl std::fmt::Display for WitnessSupportSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (trivial={}, checked={}, trusted={}, opaque={})",
            self.kind().as_str(),
            self.trivial,
            self.checked,
            self.trusted,
            self.opaque
        )
    }
}

/// Compile-time destination contract for witness artifacts whose proof
/// content lives in a separately compiled backend module.
pub trait WitnessModulePath {
    /// Backend module path where the generated proof content belongs.
    const MODULE_PATH: &'static str;
}

/// A statically registered export target for an explicitly instantiated
/// witness surface.
///
/// Unlike [`crate::ProofRecord`], which tracks every proof bridge a crate
/// already implements, this record is opt-in and concrete: callers
/// register the exact instantiated witness types they want a separate
/// backend pipeline to materialize.
pub struct WitnessExportRecord {
    /// The verifier backend this export targets.
    pub verifier: fn() -> &'static str,
    /// The concrete evidence type to materialize.
    pub evidence: fn() -> &'static str,
    /// The backend module path where the proof content belongs.
    pub destination_module: fn() -> &'static str,
    /// Render the witness artifact for audit without running a verifier.
    pub describe: fn() -> String,
    /// Summarize the support surface the witness closes over.
    pub support: fn() -> WitnessSupportSummary,
}

inventory::collect!(WitnessExportRecord);

/// Owned snapshot of one registered witness export target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessExportSnapshot {
    /// The verifier backend this export targets.
    pub verifier: String,
    /// The concrete evidence type to materialize.
    pub evidence: String,
    /// The backend module path where the proof content belongs.
    pub destination_module: String,
    /// Structural summary of the support surface this export closes over.
    pub support: WitnessSupportSummary,
}

/// Collect every registered witness export target into owned values.
///
/// The result is sorted by `(verifier, evidence, destination_module)` so
/// downstream tooling and tests can consume it deterministically.
pub fn witness_exports() -> Vec<WitnessExportSnapshot> {
    let mut exports: Vec<_> = inventory::iter::<WitnessExportRecord>()
        .map(|record| WitnessExportSnapshot {
            verifier: (record.verifier)().to_owned(),
            evidence: (record.evidence)().to_owned(),
            destination_module: (record.destination_module)().to_owned(),
            support: (record.support)(),
        })
        .collect();

    exports.sort_by(|left, right| {
        (
            left.verifier.as_str(),
            left.evidence.as_str(),
            left.destination_module.as_str(),
        )
            .cmp(&(
                right.verifier.as_str(),
                right.evidence.as_str(),
                right.destination_module.as_str(),
            ))
    });

    exports
}

/// Constitutional extraction of verifier-facing proof emission.
///
/// A witness names which proof (if any) backs a piece of evidence for a
/// given verifier — a descriptor, discoverable without running anything.
/// Proving is a separate mode from doing: `proof` never executes a
/// verifier, it identifies the harness/contract that a separate tool
/// invocation (`cargo kani`, etc.) would check. Like `Evidence::basis`,
/// this is a static fact about the type, true for every instance.
pub trait Witness<V: Verifier> {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the backend-facing proof for this verifier.
    type ProofArtifact;

    /// Identify the proof artifact relevant to this evidence, for this
    /// verifier.
    fn proof() -> Self::ProofArtifact;

    /// Describe what kind of support backs this witness.
    ///
    /// Backends should override this when they can distinguish checked,
    /// trusted, or trivial closure. The default stays explicit: the
    /// support surface is not classified yet.
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::opaque_leaf()
    }

    /// Produce the basis behind this proof's supporting evidence.
    fn basis() -> <Self::SupportingEvidence as Evidence>::Basis {
        <Self::SupportingEvidence as Evidence>::basis()
    }
}

/// Register explicit witness exports for a verifier backend.
///
/// This is for backends such as Verus that compile proof content in a
/// separate source unit and therefore cannot discover every derived type
/// automatically. Callers provide the concrete instantiations they want to
/// export; the macro records their evidence type, destination module, and
/// rendered witness artifact for later tooling.
#[macro_export]
macro_rules! register_witness_exports {
    (verifier = $verifier:ty; $($ty:ty),* $(,)?) => {
        $(
            $crate::__inventory::submit! {
                $crate::WitnessExportRecord {
                    verifier: || <$verifier as $crate::Verifier>::name(),
                    evidence: || ::std::any::type_name::<$ty>(),
                    destination_module: || <<$ty as $crate::Witness<$verifier>>::ProofArtifact as $crate::WitnessModulePath>::MODULE_PATH,
                    describe: || <$ty as $crate::Witness<$verifier>>::proof().to_string(),
                    support: || <$ty as $crate::Witness<$verifier>>::support(),
                }
            }
        )*
    };
}
