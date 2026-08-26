//! Kani-only accommodation model for focused formatting-builder semantics.
//!
//! This module is where Amenable stops asking Kani to execute std formatting
//! machinery directly and instead proves against a small package of explicit
//! punctuation, ordering, and pass-through laws that the real implementation
//! is expected to refine.
//!
//! The direct `Arguments` / `Debug*` rendering paths remain preserved in the
//! proof gallery as formatting-timeout false trails. Production proofs that
//! use this model are therefore conditional:
//!
//! - if the real builder/rendering path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

#[cfg(kani)]
use crate::KaniCompose;

/// Modeled leaf value with opaque display and debug tokens.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFormatAtom(char, char);

/// Opaque label token for modeled type, field, or key names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFormatLabel(char);

/// Which focused formatting law package a rendered value satisfies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaniRenderedKind {
    /// Display-style pass-through for one leaf.
    Arguments,
    /// Named-field debug rendering with one field.
    DebugStructOneField,
    /// Positional debug rendering with one field.
    DebugTupleOneField,
    /// Bracketed debug rendering with two entries.
    DebugListTwoEntries,
    /// Braced debug rendering with two entries.
    DebugSetTwoEntries,
    /// Braced key/value debug rendering with one entry.
    DebugMapOneEntry,
}

/// Modeled rendered formatting output as a structural law package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaniRendered {
    /// Display pass-through for one leaf value.
    Arguments {
        /// Observed display token.
        display_token: char,
    },
    /// Named-field debug shape.
    DebugStructOneField {
        /// Opaque type-label token preserved by the builder.
        type_label: KaniFormatLabel,
        /// Opaque field-label token preserved by the builder.
        field_label: KaniFormatLabel,
        /// Debug token preserved for the field value.
        field_debug_token: char,
    },
    /// Positional debug tuple shape.
    DebugTupleOneField {
        /// Opaque type-label token preserved by the builder.
        type_label: KaniFormatLabel,
        /// Debug token preserved for the field value.
        field_debug_token: char,
    },
    /// Ordered debug list shape.
    DebugListTwoEntries {
        /// First entry's debug token.
        first_debug_token: char,
        /// Second entry's debug token.
        second_debug_token: char,
    },
    /// Ordered debug set shape for the two-entry case used here.
    DebugSetTwoEntries {
        /// First entry's debug token.
        first_debug_token: char,
        /// Second entry's debug token.
        second_debug_token: char,
    },
    /// One-entry debug map shape.
    DebugMapOneEntry {
        /// Opaque key-label token for the already-debug-rendered key.
        key_debug_label: KaniFormatLabel,
        /// Value debug token.
        value_debug_token: char,
    },
}

/// Namespace for focused formatting-builder laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFmt;

impl KaniFormatAtom {
    /// Construct a modeled leaf value from display and debug tokens.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(display_token: char, debug_token: char) -> Self {
        Self(display_token, debug_token)
    }

    /// Report the modeled `Display` token.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn display_token(&self) -> char {
        self.0
    }

    /// Report the modeled `Debug` token.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn debug_token(&self) -> char {
        self.1
    }
}

impl KaniFormatLabel {
    /// Construct an opaque modeled label token.
    pub fn new(token: char) -> Self {
        Self(token)
    }

    /// Report the opaque modeled label token.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn token(&self) -> char {
        self.0
    }
}

impl KaniRendered {
    /// Report which focused formatting law package this output satisfies.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn kind(&self) -> KaniRenderedKind {
        match self {
            Self::Arguments { .. } => KaniRenderedKind::Arguments,
            Self::DebugStructOneField { .. } => KaniRenderedKind::DebugStructOneField,
            Self::DebugTupleOneField { .. } => KaniRenderedKind::DebugTupleOneField,
            Self::DebugListTwoEntries { .. } => KaniRenderedKind::DebugListTwoEntries,
            Self::DebugSetTwoEntries { .. } => KaniRenderedKind::DebugSetTwoEntries,
            Self::DebugMapOneEntry { .. } => KaniRenderedKind::DebugMapOneEntry,
        }
    }

    /// Report the modeled display token when this is an `Arguments` rendering.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn display_token(&self) -> Option<char> {
        match self {
            Self::Arguments { display_token } => Some(*display_token),
            _ => None,
        }
    }

    /// Report the modeled type-label token when this rendering carries one.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn type_label(&self) -> Option<KaniFormatLabel> {
        match self {
            Self::DebugStructOneField { type_label, .. }
            | Self::DebugTupleOneField { type_label, .. } => Some(*type_label),
            _ => None,
        }
    }

    /// Report the modeled field-label token when this rendering carries one.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn field_label(&self) -> Option<KaniFormatLabel> {
        match self {
            Self::DebugStructOneField { field_label, .. } => Some(*field_label),
            _ => None,
        }
    }

    /// Report the first modeled debug token when this rendering carries one.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn first_debug_token(&self) -> Option<char> {
        match self {
            Self::DebugListTwoEntries {
                first_debug_token, ..
            }
            | Self::DebugSetTwoEntries {
                first_debug_token, ..
            } => Some(*first_debug_token),
            _ => None,
        }
    }

    /// Report the second modeled debug token when this rendering carries one.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn second_debug_token(&self) -> Option<char> {
        match self {
            Self::DebugListTwoEntries {
                second_debug_token, ..
            }
            | Self::DebugSetTwoEntries {
                second_debug_token, ..
            } => Some(*second_debug_token),
            _ => None,
        }
    }

    /// Report the modeled debug token for the field or value position.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn value_debug_token(&self) -> Option<char> {
        match self {
            Self::DebugStructOneField {
                field_debug_token, ..
            }
            | Self::DebugTupleOneField {
                field_debug_token, ..
            } => Some(*field_debug_token),
            Self::DebugMapOneEntry {
                value_debug_token, ..
            } => Some(*value_debug_token),
            _ => None,
        }
    }

    /// Report the modeled map-key label token when this rendering carries one.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn key_debug_label(&self) -> Option<KaniFormatLabel> {
        match self {
            Self::DebugMapOneEntry {
                key_debug_label, ..
            } => Some(*key_debug_label),
            _ => None,
        }
    }
}

impl KaniFmt {
    /// Model `Arguments` as display-token pass-through for the leaf value.
    pub fn arguments(atom: &KaniFormatAtom) -> KaniRendered {
        KaniRendered::Arguments {
            display_token: atom.display_token(),
        }
    }

    /// Model `DebugStruct` for the one-field shape used in this proof queue.
    pub fn debug_struct_one_field(
        type_label: KaniFormatLabel,
        field_label: KaniFormatLabel,
        field_value: &KaniFormatAtom,
    ) -> KaniRendered {
        KaniRendered::DebugStructOneField {
            type_label,
            field_label,
            field_debug_token: field_value.debug_token(),
        }
    }

    /// Model `DebugTuple` for the one-field shape used in this proof queue.
    pub fn debug_tuple_one_field(
        type_label: KaniFormatLabel,
        field_value: &KaniFormatAtom,
    ) -> KaniRendered {
        KaniRendered::DebugTupleOneField {
            type_label,
            field_debug_token: field_value.debug_token(),
        }
    }

    /// Model `DebugList` for the two-entry shape used in this proof queue.
    pub fn debug_list_two_entries(first: &KaniFormatAtom, second: &KaniFormatAtom) -> KaniRendered {
        KaniRendered::DebugListTwoEntries {
            first_debug_token: first.debug_token(),
            second_debug_token: second.debug_token(),
        }
    }

    /// Model `DebugSet` for the two-entry shape used in this proof queue.
    pub fn debug_set_two_entries(first: &KaniFormatAtom, second: &KaniFormatAtom) -> KaniRendered {
        KaniRendered::DebugSetTwoEntries {
            first_debug_token: first.debug_token(),
            second_debug_token: second.debug_token(),
        }
    }

    /// Model `DebugMap` for the one-entry shape used in this proof queue.
    pub fn debug_map_one_entry(
        key_debug_label: KaniFormatLabel,
        value: &KaniFormatAtom,
    ) -> KaniRendered {
        KaniRendered::DebugMapOneEntry {
            key_debug_label,
            value_debug_token: value.debug_token(),
        }
    }
}

#[cfg(kani)]
impl KaniCompose for KaniFormatAtom {
    fn kani_depth0() -> Self {
        Self::new('0', '0')
    }

    fn kani_depth1() -> Self {
        Self::new('1', '1')
    }

    fn kani_depth2() -> Self {
        Self::new('a', 'A')
    }

    fn kani_any() -> Self {
        Self::new(char::kani_any(), char::kani_any())
    }
}

#[cfg(kani)]
impl KaniCompose for KaniFormatLabel {
    fn kani_depth0() -> Self {
        Self::new('n')
    }

    fn kani_depth1() -> Self {
        Self::new('t')
    }

    fn kani_depth2() -> Self {
        Self::new('k')
    }

    fn kani_any() -> Self {
        Self::new(char::kani_any())
    }
}

// Self-test of KaniCompose's own contract for KaniFormatAtom, not a
// production proof -- same reasoning as compose.rs's own `mod proofs`:
// KaniCompose is Kani-only modeling infrastructure (see
// docs/KANI_COMPOSE_PLAN.md), so a claim about what its depth
// constructors return belongs here, not in an ordinary #[test].
#[cfg(kani)]
mod proofs {
    use super::{KaniCompose, KaniFormatAtom};

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_FORMAT_ATOM_DEPTH2_SRC, {
            #[kani::proof]
            fn verify_kani_compose_format_atom_depth2() {
                let atom = KaniFormatAtom::kani_depth2();
                assert_eq!(atom.display_token(), 'a');
                assert_eq!(atom.debug_token(), 'A');
            }
        }
    }
}
