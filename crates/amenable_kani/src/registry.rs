//! Inventory records for executable Kani proof harnesses.

/// One executable Kani proof harness compiled into `amenable_kani`.
///
/// This record is a catalog entry, not a verification result. It says which
/// proof Kani can run; the CLI's CSV ledger records whether a particular run
/// later passed, failed, or timed out.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_getters::Getters,
    derive_getters::Dissolve,
    derive_new::new,
)]
pub struct KaniProof {
    /// Stable, fully-qualified proof identifier.
    id: String,
    /// Exact Kani harness selector.
    harness: String,
    /// Cargo package containing the library target that defines this harness.
    package: String,
}

/// Static registration that constructs an owned [`KaniProof`] on discovery.
///
/// `inventory` entries must be static, so the registration holds only a
/// function pointer. The CLI calls it to obtain owned strings; no proof data
/// is represented by `&'static str` fields.
///
/// Hand-written `const fn new`/getter, not derived: this record is itself
/// passed to `inventory::submit!`, which requires a `const`-evaluable
/// value, and `derive_new::new` cannot generate a `const fn`. `KaniProof`
/// itself has no such requirement -- it's built at call time inside the
/// stored closure, not at registration time -- so it uses the ordinary
/// derives above.
pub struct KaniProofRegistration {
    proof: fn() -> KaniProof,
}

impl KaniProofRegistration {
    /// Register an executable Kani proof harness constructor.
    #[must_use]
    pub const fn new(proof: fn() -> KaniProof) -> Self {
        Self { proof }
    }

    /// Construct this registration's executable Kani proof descriptor.
    #[must_use]
    pub const fn proof(&self) -> fn() -> KaniProof {
        self.proof
    }
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hypothesis => "hypothesis",
            Self::FalseTrail => "false_trail",
            Self::BestPractice => "best_practice",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
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
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, derive_new::new,
)]
pub struct KaniGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    id: String,
    /// Exact Kani harness selector.
    harness: String,
    /// Cargo package containing the library target that defines this harness.
    package: String,
    /// Short human-facing summary of what the case demonstrates.
    title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    #[getter(copy)]
    disposition: KaniGalleryDisposition,
    /// The verifier outcome this diagnostic case is expected to produce.
    #[getter(copy)]
    expected: KaniGalleryExpectation,
}

/// Static registration that constructs an owned [`KaniGalleryCase`] on demand.
///
/// Hand-written `const fn new`/getter, not derived: this record is itself
/// passed to `inventory::submit!`, which requires a `const`-evaluable value,
/// and `derive_new::new` cannot generate a `const fn`. `KaniGalleryCase`
/// itself has no such requirement -- it's built at call time inside the
/// stored closure, not at registration time -- so it uses the ordinary
/// derives above.
pub struct KaniGalleryRegistration {
    case: fn() -> KaniGalleryCase,
}

impl KaniGalleryRegistration {
    /// Register a proof-gallery case constructor.
    #[must_use]
    pub const fn new(case: fn() -> KaniGalleryCase) -> Self {
        Self { case }
    }

    /// Construct this registration's proof-gallery descriptor.
    #[must_use]
    pub const fn case(&self) -> fn() -> KaniGalleryCase {
        self.case
    }
}

inventory::collect!(KaniGalleryRegistration);
