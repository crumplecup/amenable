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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerusGalleryCase {
    /// Stable, fully-qualified gallery-case identifier.
    pub id: String,
    /// Short human-facing summary of what the case demonstrates.
    pub title: String,
    /// Whether the case is a hypothesis, a false trail, or a best practice.
    pub disposition: VerusGalleryDisposition,
    /// The real `verus` outcome this case documents.
    pub expected: VerusGalleryExpectation,
    /// The reduced repro (or, for `BestPractice` cases, the working
    /// alternative) as real Rust/Verus source — verbatim, not a
    /// paraphrase — plus a trailing note citing the actual diagnostic
    /// observed, so this can't silently drift into an unverified claim.
    pub claim: &'static str,
}

/// Static registration that constructs an owned [`VerusGalleryCase`] on
/// demand.
pub struct VerusGalleryRegistration {
    /// Construct this registration's proof-gallery descriptor.
    pub case: fn() -> VerusGalleryCase,
}

inventory::collect!(VerusGalleryRegistration);

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::wrapping_add_operator_blocked_by_coherence".to_owned(),
            title: "Wrapping<i32>'s `+` operator can't be verified from outside vstd (coherence); real, narrower field-roundtrip coverage lands instead".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Unproved,
            claim: r#"
// Attempt 1 (naive): call the real Wrapping<i32> `+` operator directly,
// the same claim amenable_kani's verify_wrapping_add_matches_the_inner_
// wrapping_add harness checks.
use std::num::Wrapping;
pub fn verify_wrapping_add_matches_the_inner_wrapping_add(a: i32, b: i32) -> (result: Wrapping<i32>)
    ensures
        result.0 == a.wrapping_add(b),
{
    Wrapping(a) + Wrapping(b)
}

// Observed under `verus --crate-type=lib`:
//   error: `core::num::wrapping::Wrapping` is not supported (note: you
//   may be able to add a Verus specification to this type with the
//   `external_type_specification` attribute)
//   error: `core::num::wrapping::impl&%393::add` is not supported (note:
//   you may be able to add a Verus specification to this function with
//   `assume_specification`)

// Attempt 2: follow both hints — register the foreign type, then supply
// a direct axiom for its `Add::add`:
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
pub struct ExWrapping<T>(Wrapping<T>);

pub assume_specification [<Wrapping<i32> as std::ops::Add>::add] (a: Wrapping<i32>, b: Wrapping<i32>) -> (result: Wrapping<i32>)
    ensures
        result.0 == a.0.wrapping_add(b.0),
;

// Observed under `verus --crate-type=lib`:
//   error: precondition not satisfied
//     --> Wrapping(a) + Wrapping(b)
//     ::: vstd/std_specs/ops.rs:68: self.$req(rhs) -- failed precondition
//   verification results:: 7 verified, 1 errors

// Root cause: registering ExWrapping makes vstd treat Wrapping<i32> as a
// known external type, which routes its `+` operator through vstd's own
// generic operator-overload machinery (`external_trait_extension`, see
// vstd/std_specs/ops.rs's ExAdd/AddSpec/AddSpecImpl trio) INSTEAD of the
// direct assume_specification above — operator syntax and any `.add()`
// call dispatch through the trait extension, not straight to the
// concrete impl. That machinery requires `self.add_req(rhs)` to hold,
// and add_req/add_spec/obeys_add_spec are uninterpreted spec functions
// vstd implements AddSpecImpl for only on the primitive integer types
// (vstd/std_specs/ops.rs's def_bop_impls_check_overflow! instantiation).
// Rust's coherence rules block this crate from providing its own
// `impl AddSpecImpl for Wrapping<i32>` (AddSpecImpl and Wrapping are both
// foreign here), so the precondition can never be discharged this way.

// Checked against vstd's own docs (external_trait_specifications.md,
// "The obeys_* pattern in vstd"), which name assume_specification as the
// escape hatch — but on `obeys_add_spec()`/`add_spec`, both SPEC-mode
// functions. assume_specification's own reference doc (reference-assume-
// specification.md) is explicit: it only applies to `exec`-mode
// functions. There's no route to axiomatize a spec fn this way, and
// AddSpecImpl (the only other route) is coherence-blocked as above — so
// this is a real dead end, not just an unconfirmed syntax question.
//
// Real, narrower coverage lands instead (amenable_verus::rust_std::
// wrapping_carrier's actual, live proof): Wrapping(value).0 == value, the
// tuple constructor/field-access roundtrip via the same ExWrapping
// external_type_specification, WITHOUT going through Add at all. Genuine
// machine-checked coverage for Wrapping<i32> exists; the specific
// operator-overload claim Kani/Creusot check remains unprovable under
// Verus from this crate.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::saturating_add_operator_blocked_by_coherence_and_missing_primitive_spec".to_owned(),
            title: "Saturating<i32>'s `+` operator hits Wrapping's same coherence block, plus i32::saturating_add itself has no vstd spec at all".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::NotSupported,
            claim: r#"
// Attempt: the same claim amenable_kani's verify_saturating_add_matches_
// the_inner_saturating_add harness checks, via the same
// external_type_specification approach that got Wrapping<i32> as far as
// a real "precondition not satisfied" (see
// wrapping_add_operator_blocked_by_coherence, above).
use std::num::Saturating;
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
pub struct ExSaturating<T>(Saturating<T>);

pub fn verify_saturating_add_matches_the_inner_saturating_add(a: i32, b: i32) -> (result: Saturating<i32>)
    ensures
        result.0 == a.saturating_add(b),
{
    Saturating(a) + Saturating(b)
}

// Observed under `verus --crate-type=lib`: TWO independent unsupported
// errors, not one:
//   error: `core::num::impl&%2::saturating_add` is not supported (note:
//   you may be able to add a Verus specification to this function with
//   `assume_specification`)
//     --> result.0 == a.saturating_add(b),
//   error: [same AddSpecImpl-routed operator failure Wrapping<i32> hits]
//     --> Saturating(a) + Saturating(b)
// Worse than Wrapping's case in one respect: vstd gives `wrapping_add`
// a real spec for the primitive integer types (referencing it in an
// `ensures` clause works fine), but `saturating_add` has no vstd spec at
// all — even stating the claim's right-hand side fails before the
// left-hand side's operator-coherence problem is reached.

// Real, narrower coverage lands instead, same shape as Wrapping:
// Saturating(value).0 == value, the tuple constructor/field-access
// roundtrip, via ExSaturating, without touching Add or saturating_add.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::reverse_cmp_blocked_by_coherence_through_ord_not_add".to_owned(),
            title: "Reverse<i32>'s comparison-inversion claim hits the same coherence wall as Wrapping's +, through Ord's OrdSpecImpl instead of Add's AddSpecImpl".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Unproved,
            claim: r#"
// Attempt 1: the same claim amenable_kani's verify_reverse_inverts_
// comparison harness checks — Reverse<T>'s .cmp() swaps T's ordering.
use std::cmp::Reverse;
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
pub struct ExReverse<T>(Reverse<T>);

pub assume_specification [<Reverse<i32> as core::cmp::Ord>::cmp] (a: &Reverse<i32>, b: &Reverse<i32>) -> (result: core::cmp::Ordering)
    ensures
        a.0 < b.0 ==> result == core::cmp::Ordering::Greater,
        a.0 == b.0 ==> result == core::cmp::Ordering::Equal,
        a.0 > b.0 ==> result == core::cmp::Ordering::Less,
;

// Observed under `verus --crate-type=lib`: signature mismatch — the real
// `cmp` is generic over T (Reverse<T>: Ord requires T: Ord), so a
// concrete i32 instantiation is rejected outright:
//   error: assume_specification requires function type signature to
//   match `core::cmp::impl&%2::cmp` exactly
//   expected: `for<T> for<'_, '_> (&Reverse<T>, &Reverse<T>) -> Ordering`

// Attempt 2: match the real generic signature exactly, dropping the
// concrete i32 ensures (a fully generic T has no comparison result to
// state one against):
pub assume_specification<T> [<Reverse<T> as core::cmp::Ord>::cmp] (a: &Reverse<T>, b: &Reverse<T>) -> (result: core::cmp::Ordering)
    where
        T: core::cmp::Ord,
;

// Observed: this compiles (past the signature-match stage this time),
// but with no ensures clause the caller's own postcondition
// (result.0 == Greater when a < b, etc.) is now genuinely unproved —
// `verus` reports real "postcondition not satisfied" errors, not a
// signature/type error. Adding an ensures clause generic over T would
// need to reference T's OWN comparison result symbolically (e.g.
// `result == b.0.cmp(&a.0)`), which regresses into exactly the same
// obeys_*_spec machinery Wrapping's Add claim is blocked by (see
// wrapping_add_operator_blocked_by_coherence, above) — Ord is under the
// identical vstd `external_trait_extension` treatment as Add
// (OrdSpecImpl instead of AddSpecImpl), so this is confirmed to be the
// same root cause, not a coincidence of two unrelated blockers.

// Real, narrower coverage lands instead, same shape as Wrapping/
// Saturating: Reverse(value).0 == value, the tuple constructor/field-
// access roundtrip, via ExReverse, without touching Ord or cmp.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::nonzero_new_blocked_by_sealed_zeroable_primitive".to_owned(),
            title: "NonZero::new can't be given a Verus spec: its real signature bounds on the sealed, unstable ZeroablePrimitive trait".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::NotSupported,
            claim: r#"
// Attempt 1: axiomatize the real, unmodified std::num::NonZero::<T>::new,
// generic over T, the same shape amenable_kani/amenable_creusot both
// check concretely (e.g. for i8).
pub assume_specification<T> [std::num::NonZero::<T>::new] (value: T) -> (result: Option<std::num::NonZero<T>>)
    where
        T: std::num::ZeroablePrimitive,
    ensures
        result.is_some() <==> value != T::ZERO,
;

// Observed under `verus --crate-type=lib`: T::ZERO doesn't exist at all
// (rustc E0599 — ZeroablePrimitive has no such associated item; wrong
// guess at its shape) and, separately, Option<NonZero<T>> as a return
// type isn't a registered external type yet either.

// Attempt 2: drop the guessed T::ZERO postcondition, register NonZero
// itself via external_type_specification + external_body (its field is
// private, unlike Wrapping/Saturating's public .0 — plain
// external_type_specification alone fails with "private fields not
// supported for transparent datatypes").
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExNonZero<T: std::num::ZeroablePrimitive>(std::num::NonZero<T>);

pub assume_specification<T> [std::num::NonZero::<T>::new] (value: T) -> (result: Option<std::num::NonZero<T>>)
    where
        T: std::num::ZeroablePrimitive,
;

// Observed: NonZero itself is now accepted, but the ZeroablePrimitive
// bound produces (for now) only a WARNING ("cannot use external trait ...
// as a bound without declaring the trait ... this is a warning for now
// but will eventually be an error") alongside a real, blocking error:
// Verus synthesizes an internal shadow trait name for the unrecognized
// bound and then can't resolve it ("cannot find trait
// `T15_ZeroablePrimitive` in this scope").

// Attempt 3: follow the warning's own advice and declare
// ZeroablePrimitive via external_trait_specification, reproducing its
// real (checked directly against std's own source,
// core::num::nonzero::ZeroablePrimitive) shape: `pub impl(self) unsafe
// trait ZeroablePrimitive: Sized + Copy { type NonZeroInner: Sized +
// Copy; }`.
#[verifier::external_trait_specification]
pub trait ExZeroablePrimitive: Sized + Copy {
    type ExternalTraitSpecificationFor: std::num::ZeroablePrimitive;
    type NonZeroInner: Sized + Copy;
}

// Observed — the real, final, confirmed dead end:
//   error: external_trait_specification trait bound mismatch
//   the external trait bounds are:
//     - ...ExternalTraitSpecificationFor: std::marker::Sized
//     - ...ExternalTraitSpecificationFor: std::marker::Copy
//     - ...ExternalTraitSpecificationFor: core::num::nonzero::private::Sealed
// ZeroablePrimitive's REAL bound set includes a third supertrait,
// `core::num::nonzero::private::Sealed` — `impl(self)` (an unstable
// "sealed impl" restriction) desugars to exactly this: a hidden
// supertrait in a `mod private` that is not `pub`, so it cannot be named
// from any downstream crate, amenable_verus included. There is no
// syntax to declare a bound on a trait we cannot name. This is not a
// missing-syntax problem to retry differently — it is std deliberately
// making ZeroablePrimitive unnameable outside `core` itself.
//
// Matches the identical wall amenable_creusot's extern_spec! hit for
// NonZero, confirming this is a genuine cross-verifier limitation, not
// a Verus-specific gap: NonZero::new cannot be given a real spec by any
// downstream crate in either verifier until std stabilizes a nameable
// bound.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::cfg_verus_is_never_actually_set".to_owned(),
            title: "#[cfg(verus)] is a declared check-cfg name, not a cfg the real verus binary ever sets — gating proof content behind it silently strips it".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Unproved,
            claim: r#"
// Hypothesis (by analogy with amenable_kani's #[cfg(kani)] and
// amenable_creusot's #[cfg(creusot)], both of which really are set by
// their respective toolchains): gate Option/Result proof content behind
// #[cfg(verus)] so it's invisible to plain, non-Verus rustc/clippy —
// amenable_verus/Cargo.toml's own `check-cfg` list even declares
// 'cfg(verus)' as a possible name, which reads as confirmation.
#![cfg(verus)]
verus! {
    pub fn verify_option_unwrap_returns_the_wrapped_value(/* ... */) { /* ... */ }
}

// Observed: this compiles with no error under `verus --crate-type=lib`
// (declaring a cfg name via check-cfg only silences the "unexpected cfg"
// lint — it never implies the cfg is ever set to true by anything).
// `just verify-verus`'s own reported proof count DROPPED (5 -> 4) with
// the gate in place versus without it: the real verus binary does not
// set cfg(verus) either, so the gated block compiles out under real
// verus compilation too, not just under plain rustc — the content was
// being silently skipped, not silently protected.

// Fix: no cfg gate at all. amenable_verus has no plain-rustc build to
// protect content from in the first place (it is never a dependency of
// anything — see amenable_verus::lib's own module doc comment) — the
// clippy-visibility problem this hypothesis was trying to solve
// (clippy::unnecessary_literal_unwrap) was fixed instead by restructuring
// the proof itself (see option_carrier.rs/result_carrier.rs: take the
// Option/Result as a `requires`-constrained parameter, not a literal
// constructed inline).
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::try_from_int_error_occurs_via_duplicate_assume_specification_ice".to_owned(),
            title: "declaring assume_specification for a trait method vstd already specifies crashes verus outright, not a diagnosed conflict".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Ice,
            claim: r#"
// Attempt: axiomatize u8::try_from(i32), the same claim amenable_kani's
// verify_try_from_int_error_occurs_exactly_when_out_of_range harness
// checks over every possible i32.
pub assume_specification [<u8 as std::convert::TryFrom<i32>>::try_from] (value: i32) -> (result: Result<u8, <u8 as std::convert::TryFrom<i32>>::Error>)
    ensures
        (0 <= value && value <= u8::MAX as i32) ==> (result is Ok && result->Ok_0 == value as u8),
        (value < 0 || value > u8::MAX as i32) ==> result is Err,
;

// Observed under `verus --crate-type=lib` — NOT a diagnosed error:
//   thread 'rustc' panicked at vir/src/traits.rs:511:13:
//   assertion failed: !method_impls.contains(&p)
// Confirmed this is a genuine internal crash, not a syntax problem: the
// exact same panic reproduces regardless of surface form (fully
// qualified `<u8 as TryFrom<i32>>::try_from` vs. the short `u8::try_from`
// path, with or without an explicit `<u8 as TryFrom<i32>>::Error`
// associated-type return — every variant that reaches signature-match
// crashes identically).

// Root cause, found by reading vstd's own source
// (vstd/std_specs/convert.rs), not guessed: vstd ALREADY declares an
// assume_specification for this exact trait-method instantiation, via
// its impl_int_try_from_spec! macro (`impl_int_try_from_spec! { i32 =>
// [u8 u16 u32 u64 u128 i8 i16 usize isize] }`), with real, matching
// semantics (`if Self::MIN <= v <= Self::MAX { Ok(v as Self) } else {
// Err(arbitrary()) }`, `obeys_try_from_spec()` unconditionally true for
// this pair). A second, local assume_specification for the identical
// (Self, T) instantiation doesn't produce a diagnosed "already declared"
// error the way redeclaring an ordinary Rust item would — verus's
// internal trait-impl bookkeeping (`vir::traits`) asserts the impl slot
// is unclaimed and panics when it finds it already is.

// Fix: don't declare a local assume_specification for a trait method
// vstd already specifies at all — just call it. amenable_verus::rust_std
// ::try_from_int_error_carrier's real, working proof relies on vstd's
// own spec directly and states the same postcondition as its own
// function-level ensures clause instead, with no local
// assume_specification for try_from whatsoever.

// General lesson: before writing a new assume_specification for any std
// trait method, check vstd's own std_specs/*.rs for an existing one
// first (as amenable_std::creusot_gallery's own findings already
// establish for Creusot's extern_spec! equivalent) — not just to avoid
// duplicate effort, but because here a duplicate isn't merely wasted
// work, it crashes the toolchain.
"#,
        },
    }
}
