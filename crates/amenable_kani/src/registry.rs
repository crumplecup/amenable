//! Inventory records for executable Kani proof harnesses.

/// One executable Kani proof harness compiled into `amenable_kani`.
///
/// This record is a catalog entry, not a verification result. It says which
/// proof Kani can run; the CLI's CSV ledger records whether a particular run
/// later passed, failed, or timed out.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniProof {
    /// Stable, fully-qualified proof identifier.
    pub id: String,
    /// Exact Kani harness selector.
    pub harness: String,
    /// Cargo package containing the library target that defines this harness.
    pub package: String,
}

/// Static registration that constructs an owned [`KaniProof`] on discovery.
///
/// `inventory` entries must be static, so the registration holds only a
/// function pointer. The CLI calls it to obtain owned strings; no proof data
/// is represented by `&'static str` fields.
pub struct KaniProofRegistration {
    /// Construct this registration's executable Kani proof descriptor.
    pub proof: fn() -> KaniProof,
}

inventory::collect!(KaniProofRegistration);

/// How a proof-gallery case should be interpreted operationally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaniGalleryDisposition {
    /// An open question we are testing in reduced form.
    Hypothesis,
    /// A pattern that looks promising but is known to mislead or fail.
    FalseTrail,
    /// A pattern we want to reuse as a verifier-friendly practice.
    BestPractice,
}

impl KaniGalleryDisposition {
    /// Stable snake-case rendering used in CLI output and ledgers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hypothesis => "hypothesis",
            Self::FalseTrail => "false_trail",
            Self::BestPractice => "best_practice",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "hypothesis" => Some(Self::Hypothesis),
            "false_trail" => Some(Self::FalseTrail),
            "best_practice" => Some(Self::BestPractice),
            _ => None,
        }
    }
}

/// Expected or observed verifier outcome for a proof-gallery case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaniGalleryExpectation {
    /// Kani should verify the harness successfully.
    Passed,
    /// Kani should report a verification failure.
    Failed,
    /// Kani should time out while analyzing the harness.
    Timeout,
}

impl KaniGalleryExpectation {
    /// Stable snake-case rendering used in CLI output and ledgers.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "passed" => Some(Self::Passed),
            "failed" => Some(Self::Failed),
            "timeout" => Some(Self::Timeout),
            _ => None,
        }
    }
}

/// One non-production Kani experiment in the proof gallery.
///
/// Gallery cases are executable and self-registering like production proofs,
/// but they are explicitly diagnostic. Their expected verifier outcome may be
/// `failed` or `timeout`, because the point is to document what Kani does with
/// a particular modeling pattern.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    pub id: String,
    /// Exact Kani harness selector.
    pub harness: String,
    /// Cargo package containing the library target that defines this harness.
    pub package: String,
    /// Short human-facing summary of what the case demonstrates.
    pub title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    pub disposition: KaniGalleryDisposition,
    /// The verifier outcome this diagnostic case is expected to produce.
    pub expected: KaniGalleryExpectation,
}

/// Static registration that constructs an owned [`KaniGalleryCase`] on demand.
pub struct KaniGalleryRegistration {
    /// Construct this registration's proof-gallery descriptor.
    pub case: fn() -> KaniGalleryCase,
}

inventory::collect!(KaniGalleryRegistration);
