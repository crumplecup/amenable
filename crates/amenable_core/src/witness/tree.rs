//! The structural shape of a witness artifact tree.

use super::support::{WitnessSupportKind, WitnessSupportSummary};
use crate::MetadataEntry;

/// Shape-reporting surface for a witness proof artifact.
pub trait WitnessArtifact {
    /// Reify this proof artifact as a structured tree for audit and
    /// backend-specific scaffold generation.
    fn witness_artifact(&self) -> WitnessArtifactNode;
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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(kind, support, detail))
    )]
    pub fn leaf(
        kind: WitnessSupportKind,
        support: WitnessSupportSummary,
        detail: impl Into<String>,
    ) -> Self {
        Self::leaf_with_metadata(kind, support, detail, Vec::new())
    }

    /// Construct one leaf node with structured metadata facts.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(kind, support, detail, metadata))
    )]
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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(shape, support, members))
    )]
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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(support, tag, variants))
    )]
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
