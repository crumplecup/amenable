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

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::layout_new_size_and_align_are_opaque_even_for_primitives".to_owned(),
            title: "Layout::new::<i32>()'s size/align values are unprovable: vstd deliberately treats size_of/align_of as fully opaque, even for i32".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Unproved,
            claim: r#"
// Attempt: the same claim amenable_kani's verify_layout_new_reports_
// the_types_size_and_alignment harness checks — Layout::new::<i32>()
// reports size 4, align 4.
pub assume_specification [Layout::new::<i32>] () -> (result: Layout)
    ensures
        result.size() == 4,
        result.align() == 4,
;

// Observed under `verus --crate-type=lib`: signature mismatch first —
// the real Layout::new is generic over T, so a concrete i32
// instantiation is rejected outright (same shape as the Reverse::cmp
// and TryFromIntError findings above):
//   error: assume_specification requires function type signature to
//   match ... exactly ... expected: `for<T> () -> Layout`

// Root cause, found by reading vstd's own source (vstd/layout.rs), not
// guessed: vstd already gives real, working specs for
// core::mem::size_of::<V>()/align_of::<V>() — but as `uninterp spec fn
// size_of<V>() -> nat` / `align_of<V>() -> nat`, deliberately left
// UNCONSTRAINED for every V, primitives included. The file's own
// comment explains why: "we are NOT creating an axiom that size_of fits
// in usize" (soundness concern about reasoning over arbitrarily large,
// possibly-unmonomorphized generic types in ghost code). So even
// switching to the correct generic form
// (`pub assume_specification<T> [Layout::new::<T>] () -> (result:
// Layout) ensures result.size() == size_of::<T>() as usize, ...`) only
// relates the result to size_of::<T>()'s ABSTRACT value — never to a
// concrete number like 4, for ANY T, not just i32. There is no path
// from this crate to the concrete fact "size_of::<i32>() == 4": the
// opacity is deliberate upstream design in vstd itself, not a gap we
// could close with our own assume_specification (declaring one that
// pins size_of::<i32>() to 4 would itself be a second, conflicting
// assume_specification for a function vstd already specifies — see the
// duplicate-assume_specification ICE finding above; the same crash
// would very likely recur here too).

// Real coverage lands on the independent half of the claim instead
// (amenable_verus::rust_std::layout_carrier's actual, live proof):
// Layout::from_size_align rejects a non-power-of-two alignment — a pure
// fact about the constructor's own validation logic, provable without
// ever touching size_of/align_of's opacity.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::cell_hidden_state_unreachable_via_plain_assume_specification".to_owned(),
            title: "Cell<T>'s get/set/replace/take can't be chained: assume_specification only relates one call's own inputs/outputs, never a prior call's effect".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::Unproved,
            claim: r#"
// Attempt: the same claim amenable_kani's verify_cell_get_set_replace_
// take_round_trip harness checks — new stores the initial value, set
// overwrites it, replace overwrites it and hands back the old value,
// take does the same against T::default().
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCell<T: core::marker::MetaSized>(std::cell::Cell<T>);

pub assume_specification<T> [std::cell::Cell::<T>::new] (value: T) -> (result: std::cell::Cell<T>);
pub assume_specification<T: Copy> [std::cell::Cell::<T>::get] (cell: &std::cell::Cell<T>) -> (result: T);
pub assume_specification<T> [std::cell::Cell::<T>::set] (cell: &std::cell::Cell<T>, value: T);
pub assume_specification<T> [std::cell::Cell::<T>::replace] (cell: &std::cell::Cell<T>, value: T) -> (result: T);
pub assume_specification<T: Default + Default> [std::cell::Cell::<T>::take] (cell: &std::cell::Cell<T>) -> (result: T);

pub fn verify_cell_round_trip(initial: i32) -> (result: i32)
    ensures
        result == initial,
{
    let cell = std::cell::Cell::new(initial);
    cell.get()
}

// Getting the bounds to even reach the signature-match stage needed two
// real fixes along the way, not guesses: ExCell's real bound is
// `T: core::marker::MetaSized` (a newer nightly supertrait of `Sized`
// this toolchain's std uses — Verus compares bound lists structurally,
// not by trait implication, so `T: Sized` alone doesn't satisfy it even
// though Sized: MetaSized); Cell::take's real where-clause lists
// `T: Default` TWICE (an upstream quirk), which the proxy has to match
// literally.

// Observed under `verus --crate-type=lib`, once past both bound issues:
//   error: postcondition not satisfied
//     result == initial
// Root cause: none of the assume_specification declarations above have
// an ensures clause connecting them to each other — and none CAN,
// because assume_specification only states a fact about ONE function's
// own arguments and return value. Cell's whole contract is inherently
// relational across calls (what get() returns depends on what a PRIOR
// set()/new() call did through the SAME shared reference) — the same
// class of "hidden state behind a shared reference" problem vstd's own
// answer for Cell-like types (pcell::PCell) solves with an entirely
// different API shape: explicit Tracked<PermissionToken> objects
// threaded through every call, not std::cell::Cell's plain &self
// methods. There is no way to retrofit that onto the REAL, unmodified
// std::cell::Cell from outside vstd — assume_specification has no
// mechanism for "this call's postcondition may reference a previous
// call's effect."

// Not attempted: no known workaround from a downstream crate. Would
// need vstd itself to ship a real spec module for std::cell::Cell
// (as it does, differently, for Cell-like PCell) before this becomes
// provable.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::try_from_slice_phantom_lifetime_binder_and_match_ergonomics".to_owned(),
            title: "<[T; N]>::try_from(&[T]) needed a phantom outer lifetime binder to match, and a match expression on the result doesn't see the call's own postcondition -- both solved, full claim proved".to_owned(),
            disposition: VerusGalleryDisposition::BestPractice,
            expected: VerusGalleryExpectation::Proved,
            claim: r#"
// Real claim, now proved in full in amenable_verus::rust_std::
// try_from_slice_carrier: <[T; N]>::try_from(&[T]) succeeds exactly
// when the slice's length matches N, round-tripping the elements
// otherwise fails -- the same claim amenable_kani's own harness checks,
// with no case dropped or weakened.

// Lesson 1 (signature matching): verus prints this kind of generic
// TryFrom impl as TWO separate binder groups (for<'_0, T, N> for<'_>),
// and only the SECOND governs the argument's actual lifetime. Declaring
// the lifetime tied to the argument (&'a [T]) as a single combined
// for<'a, T, N> group does NOT match. What matches: put the lifetime in
// the TRAIT REFERENCE only (TryFrom<&'a [T]>) and leave the argument
// itself elided (bare &[T], not &'a [T]):
pub assume_specification<'a, T: Copy, const N: usize> [<[T; N] as core::convert::TryFrom<&'a [T]>>::try_from] (slice: &[T]) -> (result: Result<[T; N], TryFromSliceError>)
    ensures
        slice@.len() == N ==> (result is Ok && result->Ok_0@ == slice@),
        slice@.len() != N ==> result is Err,
;

// Lesson 2 (match ergonomics): once the axiom above compiled, a plain
// match on the call's result didn't let its postcondition reach the Ok
// arm at all -- both facts about the returned array read as completely
// unknown:
match <[i32; 2]>::try_from(matching) {
    Ok(arr) => arr[0] == matching[0] && arr[1] == matching[1],  // "postcondition not satisfied"
    Err(_) => false,
}
// Fix: bind the call's result to a `let` first, assert its shape, then
// `.unwrap()` it -- the SAME real call, but broken into steps verus's
// own reasoning can follow:
let converted = <[i32; 2]>::try_from(matching);
assert(converted is Ok);
let arr = converted.unwrap();
arr[0] == matching[0] && arr[1] == matching[1]  // verifies cleanly

// Both lessons generalize beyond this one type: any future const-
// generic-array TryFrom axiom needs the same phantom-lifetime shape,
// and any proof consuming an assume_specification'd Result should
// prefer let+assert+unwrap over a bare match when the postcondition
// needs to be visible inside the branch.
"#,
        },
    }
}

::inventory::submit! {
    VerusGalleryRegistration {
        case: || VerusGalleryCase {
            id: "amenable_std::verus_gallery::cow_deref_lifetime_elision_ambiguity".to_owned(),
            title: "Cow<'a, B>::deref can't be axiomatized: spelling the receiver out concretely creates a lifetime ambiguity plain Rust elision can't resolve".to_owned(),
            disposition: VerusGalleryDisposition::FalseTrail,
            expected: VerusGalleryExpectation::CompileError,
            claim: r#"
// Attempt: the deref half of the claim amenable_kani's
// verify_cow_borrowed_and_owned_agree_on_their_value harness checks --
// Cow::Borrowed and Cow::Owned both deref to the wrapped value.
pub assume_specification<'a, B: ToOwned + ?Sized> [<Cow<'a, B> as core::ops::Deref>::deref] (cow: &Cow<'a, B>) -> (result: &B);

// Observed under `verus --crate-type=lib`:
//   error[E0106]: missing lifetime specifier
//   this function's return type contains a borrowed value, but the
//   signature does not say which one of `cow`'s 2 lifetimes it is
//   borrowed from
// Real std::ops::Deref::deref is `fn deref(&self) -> &Self::Target` --
// its return elides to `&self`'s own lifetime with no ambiguity,
// because `Self` stays abstract in the trait definition. Spelling the
// receiver out concretely as `&Cow<'a, B>` (required to name the
// function for assume_specification at all) introduces a SECOND,
// competing candidate lifetime -- Cow's own `'a` -- that Rust's plain
// elision rules cannot disambiguate between.

// Tried every combination of naming/eliding both lifetimes:
//   (result: &'a B)                          -- typechecks, but then
//     doesn't match assume_specification's required generic-binder
//     shape (`for<'_0, B> for<'_> (&Cow<'_0, B>) -> &B`, a BARE `&B`
//     return with no name)
//   cow: &Cow<'_, B> ... -> &B                -- same "missing lifetime
//     specifier" error as the fully-named version; anonymizing Cow's
//     own lifetime with `_` doesn't remove the ambiguity, since there
//     are still two candidate sources
//   cow: &'b Cow<'a, B> ... -> &'b B           -- typechecks (like
//     TryFromSliceError's phantom-lifetime fix), but produces a single
//     combined `for<'a, 'b>` binder group, not the required TWO
//     separate groups
// Every variant either fails to typecheck as ordinary Rust at all, or
// typechecks into a shape assume_specification's exact-match
// requirement rejects. This is a different KIND of blocker than
// TryFromSliceError's (that one was about binder ORDER once the
// underlying signature was unambiguous; this one is a genuine
// unresolvable ambiguity in the concrete spelling itself).

// Real, narrower coverage lands instead (amenable_verus::rust_std::
// cow_carrier's actual, live proof): the variant-construction facts
// (Cow::Borrowed(_)/Cow::Owned(_) pattern matching needs no axiom,
// vstd's own ExCow registration keeps Cow's variants transparent) plus
// the full into_owned claim (no receiver reference, so no elision
// ambiguity at all) -- covering two of the claim's three original
// facts in full, with only the deref half left uncovered.
"#,
        },
    }
}
