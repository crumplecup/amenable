//! ISO 8601-2 temporal set descriptors.

use derive_builder::Builder;
use derive_getters::Getters;
use strum::EnumIter;

use crate::{QualifiedOrBareTemporalValueDescriptor, QualifiedTemporalExpressionDescriptor};

/// The membership semantics for an ISO 8601-2 temporal set expression.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum TemporalSetSemanticsDescriptor {
    /// Square-bracket one-of semantics.
    #[display("alternatives")]
    #[default]
    Alternatives,
    /// Curly-brace inclusive all-members semantics.
    #[display("inclusive")]
    Inclusive,
}

/// One member of an ISO 8601-2 temporal set expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TemporalSetMemberDescriptor {
    /// A single member expression.
    Value(QualifiedOrBareTemporalValueDescriptor),
    /// A bounded or open-ended range member.
    Range {
        /// Inclusive range start, when present.
        start: Option<QualifiedOrBareTemporalValueDescriptor>,
        /// Inclusive range end, when present.
        end: Option<QualifiedOrBareTemporalValueDescriptor>,
    },
}

/// A neutral ISO 8601-2 temporal set descriptor.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    Builder,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct TemporalSetDescriptor {
    /// Whether the set denotes alternatives or inclusive membership.
    #[getter(copy)]
    semantics: TemporalSetSemanticsDescriptor,
    /// Member expressions and range members in source order.
    #[builder(default)]
    members: Vec<TemporalSetMemberDescriptor>,
    /// Uncertainty or approximation qualification metadata, when present.
    #[builder(default)]
    #[getter(copy)]
    qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

impl core::default::Default for TemporalSetMemberDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Value(core::default::Default::default())
    }
}
