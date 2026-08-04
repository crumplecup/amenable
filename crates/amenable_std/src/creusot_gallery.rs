//! Creusot proof gallery: documented findings about `creusot-rustc`'s own
//! translation behavior, discovered while building the real char/String
//! proof pipeline in `amenable_creusot`.
//!
//! Mirrors `amenable_kani`'s gallery in spirit — production proofs answer
//! "does this harness establish the intended claim?", the gallery answers
//! "what does the verifier do with this pattern?" — but not in mechanism.
//! Kani gallery cases are live, independently runnable `#[kani::proof]`
//! harnesses: one failing or timing out doesn't stop `cargo kani` from
//! running the others. Creusot has no equivalent isolation — `cargo
//! creusot`/`creusot-rustc` translates a whole crate as one compilation
//! unit, and a single ICE or translation error anywhere aborts the entire
//! build, including the real char/String proofs this crate exists to
//! protect. So a gallery case here is *not* live Pearlite content: `claim`
//! is a plain string constant holding the reduced repro, hand-verified
//! once against the real toolchain (`just verify-creusot-translate`) and
//! recorded as a fact, not re-checked automatically on every build. Cases
//! whose `expected` is [`CreusotGalleryExpectation::Proved`] are the
//! exception — safe to keep live, since a real one already does (see
//! `amenable_creusot::rust_std`'s `verify_char_roundtrip`, which uses the
//! `c@` idiom this gallery's `char_as_u32_cast_is_unsupported` case failed
//! without).

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
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proved => "proved",
            Self::Unproved => "unproved",
            Self::TranslationError => "translation_error",
            Self::Ice => "ice",
        }
    }

    /// Parse a stable snake-case rendering from a persisted artifact.
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreusotGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    pub id: String,
    /// Short human-facing summary of what the case demonstrates.
    pub title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    pub disposition: CreusotGalleryDisposition,
    /// The `creusot-rustc`/`why3find` outcome this case documents.
    pub expected: CreusotGalleryExpectation,
    /// The reduced repro (or, for `BestPractice` cases, the working
    /// alternative) as real Rust/Pearlite source — verbatim, not a
    /// paraphrase — plus a trailing note citing the actual diagnostic
    /// observed, so this can't silently drift into an unverified claim.
    pub claim: &'static str,
}

/// Static registration that constructs an owned [`CreusotGalleryCase`] on
/// demand.
pub struct CreusotGalleryRegistration {
    /// Construct this registration's proof-gallery descriptor.
    pub case: fn() -> CreusotGalleryCase,
}

inventory::collect!(CreusotGalleryRegistration);

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::rpitit_panics_intrinsics_gathering".to_owned(),
            title: "a local return-position impl Trait method panics creusot-rustc's intrinsics-gathering pass".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::Ice,
            claim: r#"
// Reduced repro (this exact shape lived in amenable_creusot::witness before
// the fix — see amenable_core::Provenance's own doc comment):
trait Provenance {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry>;
}
impl Provenance for CreusotVerifierMetadata {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   thread 'rustc' panicked at .../rustc_middle/src/hir/mod.rs:409:84:
//   index out of bounds: the len is 354 but the index is 355
//   query stack during panic:
//   #0 [local_def_id_to_hir_id] getting HIR ID of
//      `witness::<impl ...>::metadata::{opaque#0}::'_`
// RPITIT desugars to a compiler-synthesized opaque type at every impl
// site; creusot-rustc's `gather_intrinsics` pass enumerates every local
// def-id (including synthetic opaque ones) and can't map this one back to
// a HIR ID. Not gated by #[cfg(creusot)] mattering: this impl was ordinary,
// always-compiled code, swept up because it's local to a crate that
// depends on creusot-std at all.
//
// Fix: replace with an associated type (`type MetadataIter: Iterator<Item
// = MetadataEntry>; fn metadata(&self) -> Self::MetadataIter;`) — an
// ordinary named item, not an opaque one. See amenable_core::Provenance.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::inventory_submit_static_is_unsupported".to_owned(),
            title: "::inventory::submit!'s generated static item can't be translated".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Reduced repro (this exact shape lived in amenable_creusot::rust_std
// before the fix — the ProofRecord registrations that now live in
// amenable_std::creusot_witness instead):
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        describe: || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: unsupported definition kind DefId(0:121 ~ amenable_creusot[..]
//   ::rust_std::_::__INVENTORY) Static { safety: Safe, mutability: Not,
//   nested: false }
// `inventory::submit!` expands to a `static` item (its linker-section-based
// distributed-registration mechanism); creusot-rustc's translator has no
// support for arbitrary `static` items at all, so ANY use of `inventory`
// inside a creusot-translated crate fails this way — not specific to
// ProofRecord.
//
// Fix: move every inventory::submit! (and the Witness bridge/CheckedProof
// machinery around it) out of amenable_creusot entirely, into
// amenable_std::creusot_witness — legal under the orphan rule because
// RustStdStandard<T>, not the verifier marker, is the local type there.
// amenable_creusot now contains zero inventory calls.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::char_as_u32_cast_is_unsupported".to_owned(),
            title: "`c as u32` isn't a supported cast in Pearlite logic context; use the `@` View operator".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact clause was in amenable_creusot::rust_std's char
// contract before the fix):
#[ensures((c as u32) <= 0xD7FFu32 || ((c as u32) >= 0xE000u32 && (c as u32) <= 0x10FFFFu32))]
fn verify_char_roundtrip(c: char) -> char { c }

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: unsupported cast from char to u32 (allowed: bool as integer,
//   integer as integer, or pointer as pointer)
//
// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today):
#[ensures(c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF))]
fn verify_char_roundtrip(c: char) -> char { c }
// `char`'s `View` impl in creusot-std maps to Pearlite's arbitrary-precision
// `Int` via a builtin (`creusot.prelude.Char.to_int`) — `@` is the intended
// way to reach a char's ordinal value in logic context, not a program-level
// cast.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::string_len_in_logic_context_is_unsupported".to_owned(),
            title: "`s.len()` can't be called directly inside #[ensures]; wrap it in a #[trusted] #[logic(opaque)] accessor".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form:
#[ensures(result.len() == s.len())]
fn verify_string_roundtrip(s: String) -> String { s }

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: called program function `std::string::String::len` in logic
//   context

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today):
#[trusted]
#[logic(opaque)]
fn string_len(_s: &String) -> usize { dead }

#[ensures(string_len(&result) == string_len(&s))]
fn verify_string_roundtrip(s: String) -> String { s }
// `String::len` is a program function; Pearlite logic context (#[requires]/
// #[ensures]) can only call #[logic] functions. `elicitation`'s own
// logic_fns.rs solves this the same way for every stdlib method it needs
// inside a contract: a #[trusted] #[logic(opaque)] wrapper axiomatizing the
// relationship (never proven, just asserted) so the real method becomes
// referenceable from logic context at all.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::boxed_dyn_iterator_is_unsupported".to_owned(),
            title: "Box<dyn Iterator<...>> has \"currently minimal\" dyn support in creusot-rustc".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact shape was amenable_creusot::witness's own
// Provenance impl before the fix — and is still the shape every OTHER
// verifier backend's own equivalent impl uses, in amenable_kani/
// amenable_verus, since neither of those crates is ever creusot-translated):
type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;
fn metadata(&self) -> Self::MetadataIter {
    Box::new(FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v)))
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: forbidden dyn type: dyn std::iter::Iterator<Item =
//   amenable_core::MetadataEntry> (dyn support is currently minimal,
//   please open an issue to improve this feature)

// Working form (this is the real, proven impl, in
// amenable_creusot::witness today):
type MetadataIter = std::vec::IntoIter<MetadataEntry>;
fn metadata(&self) -> Self::MetadataIter {
    FACTS
        .iter()
        .map(|&(k, v)| MetadataEntry::new(k, v))
        .collect::<Vec<_>>()
        .into_iter()
}
// Only matters for the one Provenance impl actually local to a
// creusot-translated crate — everywhere else in this workspace, `Box<dyn
// Iterator<...>>` is the right, general answer (see
// amenable_core::provenance's impl_scalar_provenance! macro).
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::partial_eq_derive_requires_deep_model".to_owned(),
            title: "deriving PartialEq (and so Eq/PartialOrd/Ord) requires a DeepModel impl under real translation".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact derive list was amenable_creusot::witness's
// CreusotVerifierMetadata before the fix — CLAUDE.md's own standard
// derive policy for data structures, applied uniformly to a marker type
// that's actually creusot-translated):
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreusotVerifierMetadata;

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0277]: the trait bound `witness::CreusotVerifierMetadata:
//   creusot_std::model::DeepModel` is not satisfied
//   help: the trait `creusot_std::model::DeepModel` is not implemented for
//   `witness::CreusotVerifierMetadata`
// (`Debug`/`Clone`/`Copy`/`Hash`/`Default` alone don't trigger this —
// confirmed by dropping only PartialEq/Eq/PartialOrd/Ord and re-running;
// only the comparison-generating derives need a DeepModel.)

// Working form (this is the real derive list, in
// amenable_creusot::witness today):
#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct CreusotVerifierMetadata;
// Applies only where nothing actually needs the comparison derives —
// confirmed here specifically (Verifier::Metadata only requires Provenance
// + Default; nothing compares two CreusotVerifierMetadata values anywhere
// in this workspace) before dropping them, not as a blanket rule.
"#,
        },
    }
}
