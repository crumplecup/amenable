//! The gallery's own data model: [`CreusotGalleryDisposition`],
//! [`CreusotGalleryExpectation`], [`CreusotGalleryCase`], and the
//! `inventory`-backed [`CreusotGalleryRegistration`] every case file in
//! this module registers itself through.

/// How a proof-gallery case should be interpreted operationally.
///
/// Same shape as `amenable_kani::KaniGalleryDisposition` — no
/// verifier-specific meaning to add here, so no reason to diverge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreusotGalleryDisposition {
    /// An open question we are testing in reduced form.
    Hypothesis,
    /// A pattern that looks promising but is known to mislead or fail.
    FalseTrail,
    /// A pattern we want to reuse as a verifier-friendly practice.
    BestPractice,
}

impl CreusotGalleryDisposition {
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

/// Expected or observed `creusot-rustc`/`why3find` outcome for a gallery
/// case.
///
/// Deliberately not `amenable_kani::KaniGalleryExpectation`'s
/// `Passed`/`Failed`/`Timeout` — Creusot's real failure modes are about
/// translation, not runtime verification, and don't fit a pass/fail/timeout
/// shape. `TranslationError` and `Ice` both abort the whole crate's build,
/// but are worth distinguishing: an `Ice` is a `creusot-rustc` bug (the fix
/// is "don't write this pattern, if it's a bug you can't work around"), a
/// `TranslationError` is a real, intentional limitation with a documented
/// diagnostic (the fix is usually a known alternate idiom).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CreusotGalleryExpectation {
    /// Translates and every goal discharges under `why3find prove`.
    Proved,
    /// Translates, but at least one goal doesn't discharge (e.g. an
    /// uncontracted external call yields an impossible precondition).
    Unproved,
    /// `creusot-rustc` reports a real compile error and refuses to
    /// translate the crate at all.
    TranslationError,
    /// `creusot-rustc` panics (an internal compiler error), not a
    /// diagnosed, reported error.
    Ice,
}

impl CreusotGalleryExpectation {
    /// Stable snake-case rendering used in CLI output and ledgers.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proved => "proved",
            Self::Unproved => "unproved",
            Self::TranslationError => "translation_error",
            Self::Ice => "ice",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "proved" => Some(Self::Proved),
            "unproved" => Some(Self::Unproved),
            "translation_error" => Some(Self::TranslationError),
            "ice" => Some(Self::Ice),
            _ => None,
        }
    }
}

/// One documented finding in the Creusot proof gallery.
///
/// Unlike `amenable_kani::KaniGalleryCase`, there's no `harness`/`package`
/// pair to select and run — see the module doc comment for why these
/// aren't live, independently invocable proofs.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, derive_new::new,
)]
pub struct CreusotGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    id: String,
    /// Short human-facing summary of what the case demonstrates.
    title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    #[getter(copy)]
    disposition: CreusotGalleryDisposition,
    /// The `creusot-rustc`/`why3find` outcome this case documents.
    #[getter(copy)]
    expected: CreusotGalleryExpectation,
    /// The reduced repro (or, for `BestPractice` cases, the working
    /// alternative) as real Rust/Pearlite source — verbatim, not a
    /// paraphrase — plus a trailing note citing the actual diagnostic
    /// observed, so this can't silently drift into an unverified claim.
    ///
    /// Owned, not `&'static str`: every real instance here is built owned
    /// inside a closure, not held as `&'static`, and the plain derived
    /// getter this now gets (no `#[getter(...)]` override needed) sidesteps
    /// the real `#[getter(copy)]`-on-`&'static`-field bug (confirmed via
    /// `cargo expand` elsewhere in this workspace: it generates a
    /// `&'static self` receiver, which breaks calls through a short-lived
    /// `self`) without hand-writing the getter to route around it.
    claim: String,
}

/// Static registration that constructs an owned [`CreusotGalleryCase`] on
/// demand.
///
/// Hand-written `const fn new`/getter, not derived: this record is itself
/// passed to `inventory::submit!`, which requires a `const`-evaluable
/// value, and `derive_new::new` cannot generate a `const fn`.
/// `CreusotGalleryCase` itself has no such requirement -- it's built at call
/// time inside the stored closure, not at registration time -- so it uses
/// the ordinary derives above.
pub struct CreusotGalleryRegistration {
    case: fn() -> CreusotGalleryCase,
}

impl CreusotGalleryRegistration {
    /// Register a proof-gallery case constructor.
    #[must_use]
    pub const fn new(case: fn() -> CreusotGalleryCase) -> Self {
        Self { case }
    }

    /// Construct this registration's proof-gallery descriptor.
    #[must_use]
    pub const fn case(&self) -> fn() -> CreusotGalleryCase {
        self.case
    }
}

inventory::collect!(CreusotGalleryRegistration);
