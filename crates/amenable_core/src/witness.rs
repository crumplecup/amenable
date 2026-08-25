//! Verifier-facing proof-emission roles.

use crate::{Evidence, MetadataEntry, Verifier};

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

/// Structural shape classification for one witness artifact node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WitnessArtifactShape {
    /// A named-field struct proof artifact.
    NamedStruct,
    /// A tuple-struct proof artifact.
    TupleStruct,
    /// A unit-struct proof artifact.
    UnitStruct,
    /// An enum proof artifact carrying per-variant sub-artifacts.
    Enum,
    /// A named-field enum variant proof artifact.
    NamedVariant,
    /// A tuple enum variant proof artifact.
    TupleVariant,
    /// A unit enum variant proof artifact.
    UnitVariant,
    /// A leaf artifact with no child members or variants.
    Leaf,
}

impl WitnessArtifactShape {
    /// Stable label for audit and generated-code scaffolding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamedStruct => "named_struct",
            Self::TupleStruct => "tuple_struct",
            Self::UnitStruct => "unit_struct",
            Self::Enum => "enum",
            Self::NamedVariant => "named_variant",
            Self::TupleVariant => "tuple_variant",
            Self::UnitVariant => "unit_variant",
            Self::Leaf => "leaf",
        }
    }
}

/// One named child member inside a witness artifact tree.
#[derive(
    Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_getters::Dissolve, derive_new::new,
)]
pub struct WitnessArtifactMember {
    /// Stable field/member label.
    label: String,
    /// Nested proof artifact for that member.
    artifact: Box<WitnessArtifactNode>,
}

/// One named enum variant inside a witness artifact tree.
#[derive(
    Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_getters::Dissolve, derive_new::new,
)]
pub struct WitnessArtifactVariant {
    /// Stable variant label after any derive-side rename.
    name: String,
    /// Nested proof artifact for that variant.
    artifact: Box<WitnessArtifactNode>,
}

/// One node in a verifier-facing witness artifact tree.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_getters::Dissolve)]
pub struct WitnessArtifactNode {
    /// Structural shape for this node.
    #[getter(copy)]
    shape: WitnessArtifactShape,
    /// Support summary this node closes over.
    #[getter(copy)]
    support: WitnessSupportSummary,
    /// Leaf classification or the collapsed class of this composite node.
    #[getter(copy)]
    kind: WitnessSupportKind,
    /// Optional tag name for enum roots.
    tag: Option<String>,
    /// Optional source-language variant name for enum-variant nodes.
    variant: Option<String>,
    /// Optional backend detail for leaves, such as a harness name or
    /// provenance report.
    detail: Option<String>,
    /// Structured backend metadata for leaves, such as a harness name,
    /// captured claim source, or provenance facts.
    metadata: Vec<MetadataEntry>,
    /// Named child members for struct- and variant-like nodes.
    members: Vec<WitnessArtifactMember>,
    /// Named child variants for enum roots.
    variants: Vec<WitnessArtifactVariant>,
}

impl WitnessArtifactNode {
    /// Construct one leaf node.
    pub fn leaf(
        kind: WitnessSupportKind,
        support: WitnessSupportSummary,
        detail: impl Into<String>,
    ) -> Self {
        Self::leaf_with_metadata(kind, support, detail, Vec::new())
    }

    /// Construct one leaf node with structured metadata facts.
    pub fn leaf_with_metadata(
        kind: WitnessSupportKind,
        support: WitnessSupportSummary,
        detail: impl Into<String>,
        metadata: impl IntoIterator<Item = MetadataEntry>,
    ) -> Self {
        Self {
            shape: WitnessArtifactShape::Leaf,
            support,
            kind,
            tag: None,
            variant: None,
            detail: Some(detail.into()),
            metadata: metadata.into_iter().collect(),
            members: Vec::new(),
            variants: Vec::new(),
        }
    }

    /// Construct one composite node with named child members.
    pub fn with_members(
        shape: WitnessArtifactShape,
        support: WitnessSupportSummary,
        variant: Option<String>,
        members: Vec<WitnessArtifactMember>,
    ) -> Self {
        Self {
            shape,
            support,
            kind: support.kind(),
            tag: None,
            variant,
            detail: None,
            metadata: Vec::new(),
            members,
            variants: Vec::new(),
        }
    }

    /// Construct one enum root node with named variants.
    pub fn enum_variants(
        support: WitnessSupportSummary,
        tag: impl Into<String>,
        variants: Vec<WitnessArtifactVariant>,
    ) -> Self {
        Self {
            shape: WitnessArtifactShape::Enum,
            support,
            kind: support.kind(),
            tag: Some(tag.into()),
            variant: None,
            detail: None,
            metadata: Vec::new(),
            members: Vec::new(),
            variants,
        }
    }
}

/// Shape-reporting surface for a witness proof artifact.
pub trait WitnessArtifact {
    /// Reify this proof artifact as a structured tree for audit and
    /// backend-specific scaffold generation.
    fn witness_artifact(&self) -> WitnessArtifactNode;
}

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

/// Marks a [`Witness`] whose support surface has actually been
/// classified — never blanket-implemented, and never implemented by
/// [`Witness`]'s own default `support()` (which stays `Opaque`).
///
/// Compositional structural closure (`#[derive(Witness)]`) propagates
/// this bound onto every field/variant it composes, the same way it
/// already propagates the base `Witness<V>` bound. A leaf that never
/// overrode `support()` — still `Opaque` — never implements this trait,
/// so a composite containing one fails to implement it too. Exporting a
/// witness to a backend (`register_witness_exports!`) requires this
/// bound, so an `Opaque` leaf anywhere in the composed tree turns into a
/// real `cargo check`-time trait-resolution error naming the exact
/// unclassified leaf — not a runtime failure, and not a `const`-eval
/// panic (confirmed empirically during design: `assert!` in a `const`
/// initializer does fail the build, but only via that one panic-shaped
/// channel, and doesn't generalize to code still generic over field
/// types — this trait-bound approach has neither limitation).
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no registered classification for this verifier (its Witness::support() is still Opaque)",
    note = "implement `ClassifiedWitness<{V}>` for it (e.g. via `bridge_verus_witness!`, or by overriding `support()` directly) before it can be composed into an exported witness"
)]
pub trait ClassifiedWitness<V: Verifier>: Witness<V> {}

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
            const _: fn() = || {
                fn assert_classified<T: $crate::ClassifiedWitness<$verifier>>() {}
                assert_classified::<$ty>();
            };

            $crate::__inventory::submit! {
                $crate::WitnessExportRecord::new(
                    || <$verifier as $crate::Verifier>::name(),
                    || ::std::any::type_name::<$ty>(),
                    || <<$ty as $crate::Witness<$verifier>>::ProofArtifact as $crate::WitnessModulePath>::MODULE_PATH,
                    || <$ty as $crate::Witness<$verifier>>::proof().to_string(),
                    || <$ty as $crate::Witness<$verifier>>::support(),
                    || {
                        let proof = <$ty as $crate::Witness<$verifier>>::proof();
                        $crate::WitnessArtifact::witness_artifact(&proof)
                    },
                )
            }
        )*
    };
}
