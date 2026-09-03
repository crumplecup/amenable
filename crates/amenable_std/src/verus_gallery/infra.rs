//! Verus proof gallery: documented findings about the real `verus`
//! toolchain's behavior, discovered while building the real char/String/
//! `Ordering`/`Option<i32>`/`Result<i32,i32>`/`Wrapping<i32>` proof
//! pipeline in `amenable_verus`.
//!
//! Mirrors `amenable_std::creusot_gallery` in spirit and mechanism — a
//! gallery case answers "what does the verifier do with this pattern?",
//! not "does this harness establish the intended claim?" (that's what
//! `amenable_verus`'s own live proofs are for). Like the Creusot gallery
//! (and unlike `amenable_kani`'s live, independently-runnable harnesses),
//! `claim` is a plain string constant holding the reduced repro,
//! hand-verified once against the real toolchain (`just verify-verus`)
//! and recorded as a fact, not re-checked automatically on every build —
//! `verus --crate-type=lib` translates a whole crate as one compilation
//! unit, so a genuinely unsupported pattern would abort the build,
//! including the real proofs this crate exists to protect.

/// How a proof-gallery case should be interpreted operationally.
///
/// Same shape as `amenable_std::CreusotGalleryDisposition` — no
/// verifier-specific meaning to add here, so no reason to diverge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerusGalleryDisposition {
    /// An open question we are testing in reduced form.
    Hypothesis,
    /// A pattern that looks promising but is known to mislead or fail.
    FalseTrail,
    /// A pattern we want to reuse as a verifier-friendly practice.
    BestPractice,
}

impl VerusGalleryDisposition {
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

/// Expected or observed real `verus` outcome for a gallery case.
///
/// Not exactly `amenable_std::CreusotGalleryExpectation`'s
/// translation-pipeline shape — `verus` is invoked as a bare compiler
/// over a single file tree, not a `cargo`-driven translation pass — but
/// it turns out to share one real failure mode with Creusot after all:
/// `verus` itself can genuinely crash (an internal panic in its own
/// `vir` crate, not a diagnosed error), confirmed directly, not assumed
/// from Creusot's case (see `try_from_int_error_occurs_via_duplicate_
/// assume_specification_ice`, below).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VerusGalleryExpectation {
    /// Verifies cleanly: every proof obligation discharges.
    Proved,
    /// Type-checks and is accepted by `verus`, but at least one
    /// `requires`/`ensures` obligation doesn't discharge.
    Unproved,
    /// `verus` reports the pattern itself as unsupported (an
    /// unrecognized external type, method, or trait construct) — usually
    /// with its own "you may be able to add a Verus specification ...
    /// with `assume_specification`" hint.
    NotSupported,
    /// Fails before verification even starts: an ordinary Rust
    /// name/type-resolution error (e.g. an unresolved crate), not a
    /// proof-related one.
    CompileError,
    /// `verus` itself panics (an internal compiler error in its own
    /// `vir`/`rustc` integration), not a diagnosed, reported error.
    Ice,
}

impl VerusGalleryExpectation {
    /// Stable snake-case rendering used in CLI output and ledgers.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proved => "proved",
            Self::Unproved => "unproved",
            Self::NotSupported => "not_supported",
            Self::CompileError => "compile_error",
            Self::Ice => "ice",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "proved" => Some(Self::Proved),
            "unproved" => Some(Self::Unproved),
            "not_supported" => Some(Self::NotSupported),
            "compile_error" => Some(Self::CompileError),
            "ice" => Some(Self::Ice),
            _ => None,
        }
    }
}

/// One documented finding in the Verus proof gallery.
///
/// Unlike `amenable_kani::KaniGalleryCase`, there's no `harness`/`package`
/// pair to select and run — see the module doc comment for why these
/// aren't live, independently invocable proofs.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, derive_new::new,
)]
pub struct VerusGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    id: String,
    /// Short human-facing summary of what the case demonstrates.
    title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    #[getter(copy)]
    disposition: VerusGalleryDisposition,
    /// The real `verus` outcome this case documents.
    #[getter(copy)]
    expected: VerusGalleryExpectation,
    /// The reduced repro (or, for `BestPractice` cases, the working
    /// alternative) as real Rust/Verus source — verbatim, not a
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

/// Static registration that constructs an owned [`VerusGalleryCase`] on
/// demand.
///
/// Hand-written `const fn new`/getter, not derived: this record is itself
/// passed to `inventory::submit!`, which requires a `const`-evaluable
/// value, and `derive_new::new` cannot generate a `const fn`.
/// `VerusGalleryCase` itself has no such requirement -- it's built at call
/// time inside the stored closure, not at registration time -- so it uses
/// the ordinary derives above.
pub struct VerusGalleryRegistration {
    case: fn() -> VerusGalleryCase,
}

impl VerusGalleryRegistration {
    /// Register a proof-gallery case constructor.
    #[must_use]
    pub const fn new(case: fn() -> VerusGalleryCase) -> Self {
        Self { case }
    }

    /// Construct this registration's proof-gallery descriptor.
    #[must_use]
    pub const fn case(&self) -> fn() -> VerusGalleryCase {
        self.case
    }
}

inventory::collect!(VerusGalleryRegistration);
