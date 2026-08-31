//! Coarse classification of the support surface a witness closes over.

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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(parts)))]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
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
