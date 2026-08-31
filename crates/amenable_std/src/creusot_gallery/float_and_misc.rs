//! Findings about floating point (`f64` has no `View` impl, float literals ICE
//! the compiler, `ParseFloatError`'s extern_spec translates but won't
//! discharge) and a few unrelated findings (postconditions can't `Deref`
//! `self`, function pointers, atomic SeqCst callbacks) that didn't earn their
//! own file.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::f64_has_no_view_impl_at_all".to_owned(),
            "f64/f32 have no View impl in creusot-std, so `self@` is unavailable for any float postcondition".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
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
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::float_literals_in_pearlite_ice_the_compiler".to_owned(),
            "a plain float literal (e.g. 0.0) inside #[ensures]/#[requires] panics creusot-rustc outright, not just an unsupported-construct error".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::Ice,
            r#"
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
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::parse_float_error_extern_spec_translates_but_wont_discharge".to_owned(),
            "a real, char/int-literal-only extern_spec for FromStr for f64 translates cleanly but why3find's automatic strategy won't discharge the goal against it".to_owned(),
            CreusotGalleryDisposition::Hypothesis,
            CreusotGalleryExpectation::Unproved,
            r#"
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
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::derefs_own_postcondition_cant_reference_self_via_deref".to_owned(),
            "*self inside deref's OWN #[ensures] yields Self, not Target -- circular, and a real type error".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
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
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::fn_pointer_calls_are_unsupported".to_owned(),
            "calling through a plain fn pointer is rejected as an unsupported function call type".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
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
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::atomic_sc_empty_callbacks_leave_the_store_load_relation_unproved".to_owned(),
            "atomic_sc callbacks must shoot the committer permission; empty callbacks translate but leave the store/load relation unproved".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::Unproved,
            r#"
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
        ),
    )
}
