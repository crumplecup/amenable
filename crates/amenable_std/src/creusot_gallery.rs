//! Creusot proof gallery: documented findings about `creusot-rustc`'s own
//! translation behavior, discovered while building the real char/String/
//! Duration/`NonZero<i16>`/`Ordering`/`Wrapping<i32>`/`Saturating<i32>`/
//! `IntErrorKind` proof pipeline in `amenable_creusot`.
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

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::check_ghost_extern_spec_methods_are_still_program_functions".to_owned(),
            title: "creusot-std's own #[check(ghost)] extern_spec methods can't be called inside #[ensures] either".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact shape was in amenable_creusot::rust_std's
// Duration contract before the fix — calling creusot-std's own trusted
// extern_spec methods for Duration::as_secs/subsec_nanos directly inside
// #[ensures]):
#[ensures(result.as_secs()@ == secs@ + (nanos@ / 1_000_000_000))]
#[ensures(result.subsec_nanos()@ == nanos@ % 1_000_000_000)]
fn verify_duration_new_normalizes_nanos_and_carries_into_secs(secs: u64, nanos: u32) -> Duration {
    Duration::new(secs, nanos)
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: called program function `std::time::Duration::as_secs` in
//   logic context
//   error: called program function `std::time::Duration::subsec_nanos`
//   in logic context
// `creusot_std::std::time`'s extern_spec! block marks Duration::as_secs/
// subsec_nanos/etc. with #[check(ghost)], which gives them a real
// postcondition creusot-rustc trusts — but #[check(ghost)] only makes a
// function callable from GHOST program context (ghost! blocks and the
// like), not from Pearlite LOGIC context (#[requires]/#[ensures]). Same
// underlying restriction as the plain String::len() case, just easy to
// miss here because the method genuinely does have a stated
// postcondition, unlike String::len().

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today) — expressed via `result@` (Duration's
// View, its total nanosecond count as Pearlite's Int) and creusot-std's
// own PUBLIC #[logic(open)] helper functions (nanos_to_secs et al, from
// creusot_std::std::time — plain logic functions, not extern_spec
// methods, so freely callable) instead of calling the methods at all:
#[ensures(nanos_to_secs(result@) == secs@ + (nanos@ / 1_000_000_000))]
#[ensures(result@ % 1_000_000_000 == nanos@ % 1_000_000_000)]
fn verify_duration_new_normalizes_nanos_and_carries_into_secs(secs: u64, nanos: u32) -> Duration {
    Duration::new(secs, nanos)
}
// These are the exact terms as_secs/subsec_nanos's own postconditions are
// stated in, so this proves the same underlying fact their (untouchable)
// contracts would give, without ever invoking the methods themselves.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::nonzero_new_extern_spec_needs_a_sealed_unstable_trait_bound".to_owned(),
            title: "extern_spec!-ing NonZero<T>::new isn't practical: it needs the sealed, unstable ZeroablePrimitive bound".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact shape was in amenable_creusot::rust_std before
// the fix — an extern_spec targeting the concrete NonZero<i16> alone,
// bypassing the generic std::num::NonZero<T>::new):
extern_spec! {
    impl NonZero<i16> {
        #[check(ghost)]
        #[ensures(value != 0i16 ==> match result { Some(_) => true, None => false })]
        #[ensures(value == 0i16 ==> match result { Some(_) => false, None => true })]
        fn new(value: i16) -> Option<NonZero<i16>>;
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: extern spec generics don't match
//           rust_std::extern_spec_... []
//           std::num::NonZero::<T>::new [i16]
// extern_spec! requires the declared signature to match the REAL one
// structurally, generics included — `new` is defined once, generically,
// on `impl<T: ZeroablePrimitive> NonZero<T>`, not per-instantiation, so a
// non-generic `impl NonZero<i16>` extern_spec doesn't match no matter what
// the body says. Writing the generic version faithfully would need
// `impl<T: ZeroablePrimitive> NonZero<T> { fn new(value: T) -> ... }` —
// but `core::num::ZeroablePrimitive` is `pub unsafe trait ... : ... +
// private::Sealed`, and its own doc comment calls it "currently
// permanently unstable": not nameable as a bound from outside `std` on
// stable Rust at all, confirmed by reading the real std source
// (library/core/src/num/nonzero.rs), not assumed.
//
// Unlike every other case in this gallery, there's no "working form" that
// stays a real, why3find-discharged proof — `NonZero::new` genuinely
// cannot be given a contract from outside `std` under these constraints.
// The honest fallback (this is the real content, in
// amenable_creusot::rust_std today): state the same claim Kani checks by
// symbolic execution, but mark the whole harness #[trusted] rather than
// silently dropping the postconditions or pretending they're verified:
#[trusted]
#[ensures(match result { Some(_) => value != 0i16, None => value == 0i16 })]
#[ensures(match result { Some(nz) => nonzero_i16_get(&nz) == value, None => true })]
fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
    NonZero::new(value)
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::uncontracted_calls_poison_the_whole_goal_not_just_logic_context".to_owned(),
            title: "an uncontracted external call anywhere in a harness body blocks the goal, not just calls inside #[ensures]".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::Unproved,
            claim: r#"
// Failing form (this exact shape was in amenable_creusot::rust_std before
// the fix — the postcondition matches the (o, result) PAIR structurally,
// never calling .reverse() inside #[ensures], which was expected to route
// around "called program function in logic context" the same way it did
// for char/Duration's range/decomposition claims):
#[ensures(match (o, result) {
    (Ordering::Less, Ordering::Greater) => true,
    (Ordering::Equal, Ordering::Equal) => true,
    (Ordering::Greater, Ordering::Less) => true,
    _ => false,
})]
fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
    o.reverse()
}

// Observed under `cargo creusot prove -- -p amenable_creusot` (translates
// clean, no compile error — the difference from every earlier finding
// here — but fails at the SMT stage):
//   warning: calling external function `reverse` with no contract will
//   yield an impossible precondition
//   Goal Coma.vc_verify_ordering_reverse_swaps_less_and_greater: ✘
// The earlier String::len()/Duration::as_secs() cases were about what's
// callABLE inside #[ensures] specifically. This is a different, wider
// restriction: `.reverse()` is called in the harness BODY, not inside any
// ensures clause, yet the goal still fails — an uncontracted external
// call anywhere in the function poisons the whole verification condition,
// because WP has no idea what the call actually does and assumes the
// worst (an impossible precondition) for everything downstream of it,
// logic context or not.
//
// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today) — give `Ordering::reverse` a local
// extern_spec! instead. Unlike NonZero::new, this one is actually
// practical: `reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so it's a real,
// why3find-discharged proof, not a #[trusted] fallback:
extern_spec! {
    impl Ordering {
        #[check(ghost)]
        #[ensures(match (self, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn reverse(self) -> Ordering;
    }
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::view_operator_needs_pearlite_macro_outside_attributes".to_owned(),
            title: "the @ View operator only parses inside #[requires]/#[ensures]; a #[logic] function body needs pearlite! {} to use it".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact shape was attempted while writing
// amenable_creusot::rust_std's IntErrorKind contract — a plain #[logic]
// helper function using `@` directly in its body, the same operator
// every #[ensures]/#[requires] clause in this crate already uses freely):
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    c@ >= 48 && c@ <= 57
}

// Observed under plain `cargo check -p amenable_creusot` (not even real
// translation — a hard parse error, before creusot-rustc is involved at
// all):
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an
//   operator, found `@`
// `#[requires(...)]`/`#[ensures(...)]` attribute *arguments* are consumed
// as an opaque token stream by their own proc-macro (parsed internally
// via pearlite-syn, never by rustc's ordinary expression grammar) — `@`
// is legal there regardless of context. An ordinary function BODY,
// even one annotated `#[logic]`, is parsed by rustc's normal expression
// grammar first; `@` isn't a valid operator there at all, so this fails
// to even parse, unconditionally, `#[cfg(creusot)]` gating or not (cfg
// stripping happens after parsing, not before).

// Working form (this is the real, proven helper, in
// amenable_creusot::rust_std today) — wrap the body in creusot-std's own
// `pearlite! {}` macro, which (like `requires!`/`ensures!`) receives its
// contents as an opaque token stream too, making `@` legal inside it
// even in an ordinary expression position:
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    pearlite! { c@ >= 48 && c@ <= 57 }
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::str_parse_is_a_distinct_uncontracted_wrapper_around_from_str".to_owned(),
            title: "extern_spec-ing FromStr::from_str doesn't cover calls through str::parse<F>, a separate generic wrapper method".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (this exact call shape was in amenable_creusot::rust_std's
// IntErrorKind harness before the fix — calling through the ordinary,
// idiomatic `.parse()` method, after already giving `<i32 as
// FromStr>::from_str` a real local extern_spec!):
fn verify_int_error_kind_classifies_parse_failures() -> ... {
    (
        "".parse::<i32>(),
        "not a number".parse::<i32>(),
        // ...
    )
}

// Observed under `cargo creusot -- -p amenable_creusot` (translates, but
// with a warning that predicts the same "impossible precondition" class
// of goal failure the Ordering::reverse finding hit):
//   warning: calling external function `parse` with no contract will
//   yield an impossible precondition
// `str::parse<F>(&self) -> Result<F, F::Err>` is its own distinct,
// generic method (`{ FromStr::from_str(self) }`, forwarding at runtime)
// — extern-speccing the trait method `FromStr::from_str` doesn't
// automatically cover calls made through this separate wrapper; Creusot
// reasons about exactly the function actually called, not what it
// happens to delegate to internally.

// Working form (this is the real, proven harness, in
// amenable_creusot::rust_std today) — call the contracted trait method
// directly; semantically identical at runtime, since `parse` just
// forwards to it anyway:
fn verify_int_error_kind_classifies_parse_failures() -> ... {
    (
        <i32 as std::str::FromStr>::from_str(""),
        <i32 as std::str::FromStr>::from_str("not a number"),
        // ...
    )
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::f64_has_no_view_impl_at_all".to_owned(),
            title: "f64/f32 have no View impl in creusot-std, so `self@` is unavailable for any float postcondition".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Attempted while investigating FpCategory coverage (a candidate contract
// for f64::classify(), the same idiom every other extern_spec in this
// crate uses for its own input — `self@` to reach an arbitrary-precision
// value in logic context):
extern_spec! {
    impl f64 {
        #[check(ghost)]
        #[ensures(self@ == 0.0 ==> result == FpCategory::Zero)]
        fn classify(self) -> FpCategory;
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0599]: Cannot take the view of `f64`
//     |
//     | #[ensures(self@ == 0.0 ==> result == FpCategory::Zero)]
//     |           ^^^^^ no implementation for `f64@`
//     = note: the following trait bounds were not satisfied:
//             `f64: creusot_std::model::View`
//             `&f64: creusot_std::model::View`
//             `&mut f64: creusot_std::model::View`
// Unlike char (View -> Int via a builtin) or the fixed-width integers
// (View -> Int natively), creusot-std ships no View impl for f32/f64 at
// all — confirmed by grepping the real creusot-std/creusot source trees
// (`~/repos/creusot`), not assumed. So no float postcondition can use `@`
// to reach an arbitrary-precision numeric value the way every other
// harness in this crate does for its own inputs.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::float_literals_in_pearlite_ice_the_compiler".to_owned(),
            title: "a plain float literal (e.g. 0.0) inside #[ensures]/#[requires] panics creusot-rustc outright, not just an unsupported-construct error".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::Ice,
            claim: r#"
// Attempted next, after `f64_has_no_view_impl_at_all` ruled out `@`:
// compare the plain (non-View) f64 value directly, the same way `.0` field
// projections in the Wrapping/Saturating contracts compare plain i32s:
extern_spec! {
    impl f64 {
        #[check(ghost)]
        #[ensures(self == 0.0 ==> result == FpCategory::Zero)]
        fn classify(self) -> FpCategory;
    }
}

// Observed under `cargo creusot -- -p amenable_creusot` — not a reported
// diagnostic, a real compiler panic with a backtrace:
//   error: internal compiler error: Unsupported literal
//     --> ...:580:27
//     | #[ensures(self == 0.0 ==> result == FpCategory::Zero)]
//     |                   ^^^
//   thread 'rustc' panicked at creusot/src/translation/pearlite/from_thir.rs:328:41
// Substituting a named f64 associated const for the literal (e.g.
// `self == f64::NAN`) translates and proves fine — the panic is
// specifically on Pearlite's THIR-to-term lowering hitting a raw float
// LITERAL token, not on floats in general. But that workaround doesn't
// rescue a real classify() contract: `self == f64::NAN` is vacuously
// false under IEEE-754 (NaN != NaN), so a clause built only from named
// constants proves trivially without exercising classify()'s actual
// behavior — and the Zero/Subnormal cases Kani's own harness checks need
// literal values (`0.0`, `f64::MIN_POSITIVE / 2.0`) that hit this ICE
// directly, with no const-only substitute available.
//
// Combined with the missing View impl above, this is a genuine, confirmed
// structural blocker for any real float-valued Creusot contract under the
// current toolchain — not a "looks hard" judgment call. `amenable_kani`'s
// FpCategory proof stays the honest fallback: state the same five-case
// claim Kani checks by symbolic execution, marked #[trusted] rather than
// silently dropped, the same as NonZero::new's sealed-trait case.
//
// Not float-specific, it turns out: attempting a `f64::from_str`
// extern_spec afterward (see the
// `parse_float_error_extern_spec_translates_but_wont_discharge` finding
// below) hit the identical ICE from a bare STRING literal
// (`s@ == "not a float"@`, same panic site) — so "Unsupported literal"
// applies to Pearlite literal kinds more broadly than just floats; only
// integer/bool/char literals are confirmed to translate.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::parse_float_error_extern_spec_translates_but_wont_discharge".to_owned(),
            title: "a real, char/int-literal-only extern_spec for FromStr for f64 translates cleanly but why3find's automatic strategy won't discharge the goal against it".to_owned(),
            disposition: CreusotGalleryDisposition::Hypothesis,
            expected: CreusotGalleryExpectation::Unproved,
            claim: r#"
// Attempted after `f64_has_no_view_impl_at_all`/
// `float_literals_in_pearlite_ice_the_compiler` ruled out `@` and float
// literals: ParseFloatError's own claim never needs a float VALUE (only
// Result::is_ok/is_err), so the same char/int-literal-only technique
// IntErrorKind's Pos/NegOverflow clauses use looked applicable:
extern_spec! {
    impl core::str::FromStr for f64 {
        #[check(ghost)]
        #[ensures(
            s@.len() > 0 && !is_ascii_digit(s@[0]) && s@[0] != '.' && s@[0] != '-' && s@[0] != '+'
            ==> match result { Err(_) => true, Ok(_) => false }
        )]
        #[ensures(
            s@.len() == 4
                && is_ascii_digit(s@[0]) && s@[1] == '.'
                && is_ascii_digit(s@[2]) && is_ascii_digit(s@[3])
            ==> match result { Err(_) => false, Ok(_) => true }
        )]
        fn from_str(s: &str) -> Result<f64, ParseFloatError>;
    }
}

fn verify_parse_float_error_occurs_only_for_unparseable_input()
-> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
    (
        <f64 as std::str::FromStr>::from_str("not a float"),
        <f64 as std::str::FromStr>::from_str("3.14"),
    )
}

// Observed under `cargo creusot prove -- -p amenable_creusot`: translates
// clean (no ICE, no "generics don't match" — the extern_spec's own
// well-formedness VC, `vc_from_str_f64`, proves in 0.012s), but the
// harness's own goal fails:
//   Goal Coma.vc_verify_parse_float_error_occurs_only_for_unparseable_input: ✘ (1/2)
// The emitted proof.json shows the goal split (split_vc) into two
// sub-cases, one proved (alt-ergo, 0.017s) and one left `null` —
// unattempted, not a reported counterexample. Reproduced isolating the
// Err clause alone, the Ok clause alone, and both together: all three
// isolate to the same unresolved split. The IDENTICAL technique
// (`s@.len()`/`s@[i]` via the same local `is_ascii_digit` helper) proves
// fine for `i32 as FromStr` in this same file (see
// `verify_int_error_kind_classifies_parse_failures`'s InvalidDigit
// clause) — inspecting both `.coma` files side by side shows `view_str`
// is declared identically (an uninterpreted `function view_str (self:
// string) : Seq.seq Char.t`, no visible axiom in either file) in both
// cases, so the difference isn't about string-literal reasoning itself.
// The only structural difference between the two goals is `f64`/
// `Float64.t` appearing in one `Result` and not the other.
//
// Disposition: Hypothesis, not FalseTrail — genuinely attempted (three
// independent isolation experiments, real `.coma`/`proof.json`
// inspection), but not root-caused to a specific creusot-rustc/why3find
// mechanism the way the other findings here are. A different solver or
// explicit split-vc tactic might discharge it; not explored further given
// the effort already spent relative to one checklist row. Confirmed
// reproducible, not a "looks hard" guess — the honest fallback (this is
// the real content, in amenable_creusot::rust_std today) states the same
// claim Kani checks by symbolic execution, marked #[trusted]:
#[trusted]
#[ensures(match result { (Err(_), Ok(_)) => true, _ => false })]
fn verify_parse_float_error_occurs_only_for_unparseable_input() -> (...) {
    (
        <f64 as std::str::FromStr>::from_str("not a float"),
        <f64 as std::str::FromStr>::from_str("3.14"),
    )
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::derefs_own_postcondition_cant_reference_self_via_deref".to_owned(),
            title: "*self inside deref's OWN #[ensures] yields Self, not Target -- circular, and a real type error".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (attempted while writing ManuallyDrop<T>'s extern_spec —
// ManuallyDrop's own field is private, so there's no `.0` to compare
// against the way Wrapping/Saturating/Reverse's postconditions do):
extern_spec! {
    impl<T> std::ops::Deref for ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(*result == *self)]
        fn deref(&self) -> &T;
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0308]: mismatched types
//     | #[ensures(*result == *self)]
//     |                       ^^^^^ expected type parameter `T`, found
//     |                             struct `std::mem::ManuallyDrop<T>`
// `self: &ManuallyDrop<T>` inside the contract, so `*self` yields
// `ManuallyDrop<T>` (one layer of reference removed) — reaching the
// wrapped `T` needs ANOTHER deref, i.e. calling the very method this
// postcondition is trying to specify. Not fixable by stating deref's
// contract in terms of itself, structurally, no matter the phrasing.

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today) — same shape `String::len`/
// `NonZero::get`'s private-field cases use: a `#[trusted]
// #[logic(opaque)]` accessor axiomatizing the wrapped value
// independently, generic over `T` this time (every earlier trusted
// wrapper in this file was monomorphic):
#[trusted]
#[logic(opaque)]
fn manually_drop_value<T>(_m: &ManuallyDrop<T>) -> T { dead }

extern_spec! {
    impl<T> std::ops::Deref for ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(*result == manually_drop_value(self))]
        fn deref(&self) -> &T;
    }
}
// `new`/`into_inner`'s extern_specs are stated in terms of the same
// logic function, so the harness can connect "the value passed to new"
// to "what deref/into_inner return" without ever needing `*self` inside
// deref's own contract.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::fn_pointer_calls_are_unsupported".to_owned(),
            title: "calling through a plain fn pointer is rejected as an unsupported function call type".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (attempted while adding Creusot coverage for
// amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>):
#[ensures(result == value)]
fn verify_fn_pointer_calls_the_underlying_function(value: i32) -> i32 {
    fn identity(x: i32) -> i32 { x }
    let f: fn(i32) -> i32 = identity;
    f(value)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on
// August 5, 2026:
//   error: unsupported function call type
//     | f(value)
//     | ^^^^^^^^
// A bare `fn` pointer value is accepted as a Rust type and can appear in
// Amenable's provenance/witness registry, but creusot-rustc refuses to
// translate the actual call-through expression in the proof body.

// Working fallback (this is the real content in
// amenable_creusot::rust_std today):
#[trusted]
#[ensures(result == value)]
fn verify_fn_pointer_calls_the_underlying_function(value: i32) -> i32 {
    value
}
// The trusted boundary states the dispatch law explicitly for the carrier,
// while the gallery preserves the exact translator limitation that blocked
// a real call-through proof.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::atomic_sc_empty_callbacks_leave_the_store_load_relation_unproved".to_owned(),
            title: "atomic_sc callbacks must shoot the committer permission; empty callbacks translate but leave the store/load relation unproved".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::Unproved,
            claim: r#"
// Failing form (attempted first while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<AtomicBool> and the rest of
// core::sync::atomic):
#[ensures(result.0 == initial)]
#[ensures(result.1 == next)]
fn verify_atomic_bool_load_store(initial: bool, next: bool) -> (bool, bool) {
    let (atomic, _own) = CreusotAtomicBool::new(initial);
    let observed_initial =
        atomic.load(ghost!(|_: &Committer<CreusotAtomicBool, bool, AtomicSeqCst, AtomicNone>| ()));
    atomic.store(
        next,
        ghost!(|_: &mut Committer<CreusotAtomicBool, bool, AtomicNone, AtomicSeqCst>| ()),
    );
    let observed_next =
        atomic.load(ghost!(|_: &Committer<CreusotAtomicBool, bool, AtomicSeqCst, AtomicNone>| ()));
    (observed_initial, observed_next)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on
// August 5, 2026:
//   Goal Coma.vc_verify_atomic_bool_load_store: ✘
//   Goal Coma.vc_verify_atomic_i32_load_store: ✘
// Translation succeeds, but the postconditions don't follow because the
// callbacks never connect the committer's logical `val_load`/`val_store`
// facts back to the `Perm` returned by `Atomic*_sc::new`.

// Working form (this is the real, proved pattern in
// amenable_creusot::rust_std today):
let (atomic, mut own) = CreusotAtomicBool::new(initial);
let observed_initial = atomic.load(ghost!(
    |c: &Committer<CreusotAtomicBool, bool, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
));
atomic.store(
    next,
    ghost!(
        |c: &mut Committer<CreusotAtomicBool, bool, AtomicNone, AtomicSeqCst>| c.shoot_store(&mut **own)
    ),
);
let observed_next = atomic.load(ghost!(
    |c: &Committer<CreusotAtomicBool, bool, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
));
// `atomic_sc` is usable, but only if the ghost callback actually "shoots"
// the committer against the permission token returned by `new`.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::borrowed_slice_chunk_iterators_lack_creusot_contracts".to_owned(),
            title: "borrowed slice chunk/window iterators still need trusted boundaries because creusot-std lacks the direct contracts".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<Chunks<'static, i32>> and the
// related ChunksExact/ChunksMut/ChunksExactMut/Windows carriers):
#[ensures(match result {
    (first_chunk, second_chunk, exhausted) =>
        first_chunk == Some((a, b))
            && second_chunk == Some(c)
            && exhausted,
})]
fn verify_chunks_yields_non_overlapping_groups_with_a_short_last_chunk(
    a: i32,
    b: i32,
    c: i32,
) -> (Option<(i32, i32)>, Option<i32>, bool) {
    let data = [a, b, c];
    let mut chunks = data.chunks(2);
    let first_chunk = match chunks.next() {
        Some(chunk) => match chunk {
            [first, second] => Some((*first, *second)),
            _ => None,
        },
        None => None,
    };
    let second_chunk = match chunks.next() {
        Some(chunk) => match chunk {
            [only] => Some(*only),
            _ => None,
        },
        None => None,
    };
    let exhausted = match chunks.next() {
        Some(_) => false,
        None => true,
    };
    (first_chunk, second_chunk, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `chunks` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::Chunks<'_, i32>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The direct route fails before the law can be discharged: `creusot-std`
// does not currently provide the `IteratorSpec`/`next` contract coverage
// this borrowed carrier family needs. While reducing the repro, matching
// yielded slices with patterns like `[first, second]` also triggered a
// separate creusot-rustc ICE:
//   error: internal compiler error: Unsupported projection
//   ConstantIndex { offset: 0, min_length: 2, from_end: false }
// So this is not a one-harness typo or a bad postcondition shape; the
// family is blocked on real translator/library gaps.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<Chunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for ChunksExact, ChunksMut,
// ChunksExactMut, and Windows until creusot-std grows the missing
// contracts and creusot-rustc stops ICE-ing on the slice-pattern route.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::borrowed_slice_reverse_chunk_iterators_lack_creusot_contracts".to_owned(),
            title: "borrowed reverse chunk iterators still need trusted boundaries because creusot-std lacks the direct contracts".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<RChunks<'static, i32>> and the
// related RChunksExact/RChunksExactMut/RChunksMut carriers):
#[ensures(match result {
    (first_len, first_second_last, second_len, second_first, exhausted) =>
        first_len == 2usize
            && first_second_last == Some((b, c))
            && second_len == 1usize
            && second_first == Some(a)
            && exhausted,
})]
fn verify_rchunks_groups_from_the_back(
    a: i32,
    b: i32,
    c: i32,
) -> (usize, Option<(i32, i32)>, usize, Option<i32>, bool) {
    let data = [a, b, c];
    let mut it = data.rchunks(2);
    let (first_len, first_second_last) = match it.next() {
        Some(chunk) => {
            let pair = if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            };
            (chunk.len(), pair)
        }
        None => (0usize, None),
    };
    let (second_len, second_first) = match it.next() {
        Some(chunk) => {
            let first = if chunk.len() == 1 {
                Some(chunk[0])
            } else {
                None
            };
            (chunk.len(), first)
        }
        None => (0usize, None),
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (
        first_len,
        first_second_last,
        second_len,
        second_first,
        exhausted,
    )
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `rchunks` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::RChunks<'_, i32>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same pattern repeated for `RChunksExact`, `RChunksExactMut`, and
// `RChunksMut`, with companion contractless-external warnings on
// `rchunks_exact`, `rchunks_exact_mut`, `rchunks_mut`, `remainder`, and
// `into_remainder`. This is a real `creusot-std` contract gap for the
// reverse borrowed chunk family, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<RChunks<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for RChunksExact, RChunksExactMut, and
// RChunksMut until creusot-std grows the missing contracts.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::slice_predicate_iterators_lack_creusot_iterator_contracts".to_owned(),
            title: "slice predicate carriers still need trusted boundaries because creusot-std lacks both method contracts and IteratorSpec for them".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (representative probes run while adding direct Creusot
// coverage for the remaining predicate-driven slice carriers in
// amenable_std::rust_std):
#[ensures(match result {
    (first_len, second_len, exhausted) =>
        first_len >= 1usize && second_len <= 1usize && exhausted,
})]
fn verify_slice_chunk_by_groups_adjacent_equality(a: i32, b: i32) -> (usize, usize, bool) {
    let data = [a, b];
    let mut it = data.chunk_by(|left, right| *left == *right);
    let first_len = match it.next() {
        Some(chunk) => chunk.len(),
        None => 0usize,
    };
    let second_len = match it.next() {
        Some(chunk) => chunk.len(),
        None => 0usize,
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first_len, second_len, exhausted)
}

#[ensures(match result {
    (first_len, second_len, exhausted) =>
        first_len == 1usize && second_len == 1usize && exhausted,
})]
fn verify_slice_split_separates_on_zero(a: i32, b: i32) -> (usize, usize, bool) {
    let data = [a, 0, b];
    let mut it = data.split(|value| *value == 0);
    let first_len = match it.next() {
        Some(piece) => piece.len(),
        None => 0usize,
    };
    let second_len = match it.next() {
        Some(piece) => piece.len(),
        None => 0usize,
    };
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first_len, second_len, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `chunk_by` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `ChunkBy<'_, i32, {closure@...}>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and likewise:
//   warning: calling external function `split` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `Split<'_, i32, {closure@...}>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same boundary applies to the rest of the predicate-driven slice
// family: `ChunkByMut`, `RSplit`, `RSplitMut`, `RSplitN`, `RSplitNMut`,
// `SplitInclusive`, `SplitInclusiveMut`, `SplitMut`, `SplitN`, and
// `SplitNMut`. This is a real `creusot-std` contract gap for these
// carriers, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable laws
// through direct proofs or accommodation models as appropriate.
impl CreusotWitness for RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for ChunkByMut, RSplit, RSplitMut,
// RSplitN, RSplitNMut, Split, SplitInclusive, SplitInclusiveMut, SplitMut,
// SplitN, and SplitNMut until creusot-std grows the missing contracts.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::str_iterators_lack_creusot_iterator_contracts".to_owned(),
            title: "core::str iterator carriers still need trusted boundaries because creusot-std lacks both method contracts and IteratorSpec for them".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (representative probes run while assessing whether the
// remaining `core::str` carriers in amenable_std::rust_std::str could be
// moved from trusted witnesses to direct Creusot proofs):
#[ensures(match result {
    (first, exhausted) => first == Some(byte) && exhausted,
})]
fn verify_bytes_yields_the_utf8_encoding(byte: u8) -> (Option<u8>, bool) {
    let c = if byte < 128 { byte as char } else { 'a' };
    let s = c.to_string();
    let mut it = s.bytes();
    let first = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, exhausted)
}

#[ensures(match result {
    (first, second, exhausted) =>
        first == Some("a") && second == Some("b") && exhausted,
})]
fn verify_lines_split_on_newlines() -> (Option<&'static str>, Option<&'static str>, bool) {
    let mut it = "a\nb".lines();
    let first = it.next();
    let second = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, second, exhausted)
}

#[ensures(match result {
    (first, second, exhausted) =>
        first == Some("a") && second == Some("b") && exhausted,
})]
fn verify_split_char_separates_on_the_pattern() -> (Option<&'static str>, Option<&'static str>, bool) {
    let mut it = "a,b".split(',');
    let first = it.next();
    let second = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, second, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `bytes` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Bytes<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and likewise:
//   warning: calling external function `lines` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Lines<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and:
//   warning: calling external function `split` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Split<'_, char>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same boundary applies across the rest of the `core::str` iterator
// family: `CharIndices`, `Chars`, `EncodeUtf16`, `EscapeDebug`,
// `EscapeDefault`, `EscapeUnicode`, `LinesAny`, `SplitAsciiWhitespace`,
// `SplitWhitespace`, `Utf8Chunks`, and the pattern-generic family
// monomorphized on `char` (`RSplit`, `SplitN`, `RSplitN`,
// `SplitInclusive`, `SplitTerminator`, `RSplitTerminator`, `Matches`,
// `RMatches`, `MatchIndices`, `RMatchIndices`). This is a real
// `creusot-std` contract gap for these carriers, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable laws
// through direct proofs or accommodation models as appropriate.
impl CreusotWitness for RustStdStandard<std::str::Bytes<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for the rest of the `core::str`
// iterator family until creusot-std grows the missing contracts.
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::slice_escape_ascii_lacks_creusot_iterator_contracts".to_owned(),
            title: "slice escape_ascii still needs a trusted boundary because creusot-std has neither iterator contracts nor stable byte-literal ergonomics for the direct proof".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::TranslationError,
            claim: r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<EscapeAscii<'static>>):
#[ensures(match result {
    (first_matches, second_is_backslash, third_is_n, exhausted) =>
        first_matches && second_is_backslash && third_is_n && exhausted,
})]
fn verify_escape_ascii_leaves_printable_bytes_unescaped() -> (bool, bool, bool, bool) {
    let data = [b'A', b'\n'];
    let mut escaped = data.escape_ascii();
    let first = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let second = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let third = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let exhausted = match escaped.next() {
        Some(_) => false,
        None => true,
    };
    (
        first == b'A',
        second == b'\\',
        third == b'n',
        exhausted,
    )
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `escape_ascii` with no contract
//   will yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::EscapeAscii<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// So the direct iterator route is blocked on the same `IteratorSpec` gap
// as the chunk families. While reducing the repro, an earlier postcondition
// that wrote the byte literals directly also triggered a separate
// creusot-rustc ICE:
//   error: internal compiler error: Unsupported literal
// at `second == b'\\'` inside `#[ensures]`
// The literal-free postcondition above still fails on the missing iterator
// contracts, so the carrier is blocked even without the ICE-inducing form.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep the carrier registered for
// Creusot via an explicit trusted witness whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law
// through its bounded accommodation model.
impl CreusotWitness for RustStdStandard<EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
"#,
        },
    }
}

::inventory::submit! {
    CreusotGalleryRegistration {
        case: || CreusotGalleryCase {
            id: "amenable_std::creusot_gallery::get_disjoint_mut_without_a_contract_can_appear_proved_but_is_still_unusable".to_owned(),
            title: "get_disjoint_mut can appear proved under Creusot even though the call is contractless and therefore not acceptable proof evidence".to_owned(),
            disposition: CreusotGalleryDisposition::FalseTrail,
            expected: CreusotGalleryExpectation::Proved,
            claim: r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<GetDisjointMutError>):
#[ensures(match result {
    (disjoint_ok, overlap_err, out_of_bounds_err) =>
        disjoint_ok && overlap_err && out_of_bounds_err,
})]
fn verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds(
    a: i32,
    b: i32,
) -> (bool, bool, bool) {
    let mut data = [a, b, 0, 0];
    let disjoint_ok = data.get_disjoint_mut([0, 2]).is_ok();
    let overlap_err = data.get_disjoint_mut([0, 0]).is_err();
    let out_of_bounds_err = data.get_disjoint_mut([0, 10]).is_err();
    (disjoint_ok, overlap_err, out_of_bounds_err)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `get_disjoint_mut` with no contract
//   will yield an impossible precondition
// and then the crate still finishes with:
//   Proved (34 files) ✔
// This is exactly the dangerous false trail documented elsewhere in the
// gallery: a contractless external call can let a harness "prove" for the
// wrong reason. The reported success is not acceptable evidence for
// Amenable's registry because `creusot-std` still has no contract telling
// the verifier what `get_disjoint_mut` actually does.

// Working fallback (this is the real content in
// amenable_std::creusot_witness today): keep the carrier registered for
// Creusot via an explicit trusted witness whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
"#,
        },
    }
}
