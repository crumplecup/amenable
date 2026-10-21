//! Gallery cases from the investigation into why `amenable_ext::
//! narrow_to_u8` (a trivially cheap, bounded `u32 -> u8` narrowing
//! helper, confirmed cheap alone and confirmed cheap called three times
//! in a row under a plain `#[kani::proof]`) blows CBMC's memory the
//! moment the exact same three calls are wrapped in a function that is
//! *itself* under `#[kani::proof_for_contract]` analysis -- even with a
//! trivial `|_| true` ensures on both sides. Confirmed directly from a
//! real `cargo kani` run, capped at 8 GiB of virtual memory to fail fast
//! instead of taking the whole machine down (no swap configured; an
//! uncapped run before this one did exactly that):
//!
//! ```text
//! converting SSA
//! Out of memory
//!
//! CBMC failed with status 6
//! VERIFICATION:- FAILED
//! ```
//!
//! The `aborting path on assume(false)` lines immediately before the
//! crash are almost all `alloc::raw_vec::RawVecInner::deallocate` --
//! heap-deallocation paths tied to `TemporalError`'s own
//! `Box<TemporalErrorKind>` field. The real function's own error path
//! only ever constructs exactly one of `TemporalErrorKind`'s nine
//! variants (`InvalidDescriptor`, via `invalid(...)`) -- never the
//! other eight. Hypothesis: a `Box`-wrapped sum type with many variants
//! is expensive for CBMC/DFCC to model across a contract boundary
//! regardless of which variant a given call site can actually produce,
//! and restricting to the variant(s) actually reachable should shrink
//! that cost. These two standalone toy types -- no connection to the
//! real error machinery, so this isolates exactly one variable -- test
//! that directly: a nine-variant `Box`ed enum mirroring
//! `TemporalErrorKind`'s rough shape, against a one-variant version with
//! the same payload, under the identical call pattern that crashed.
//! Error handling built this way (an umbrella `Kind` enum boxed behind
//! one error struct, CLAUDE.md pattern 2/3) is central to this whole
//! codebase, so whatever this finds generalizes well past this one
//! chrono bridge function.
//!
//! **What was actually confirmed, in order:**
//!
//! - Nine variants, boxed (mirroring `TemporalErrorKind` exactly):
//!   out-of-memory crash, reproducing the real failure exactly.
//! - One variant, boxed: **still** out-of-memory, even though the
//!   program shrank to 17,027 steps and 246 remaining VCCs -- a tiny
//!   problem by verification standards. This disproves the variant-count
//!   hypothesis outright: restricting to the variant(s) actually
//!   producible does not fix it.
//! - Nine variants, *not* boxed under `cfg(kani)` (same payload,
//!   `detail: String` kept): **still** out-of-memory. This disproves
//!   "`Box` alone is the cause": `Payload`'s own `String` field is
//!   backed by a heap-allocating `Vec<u8>` internally -- the exact same
//!   `RawVecInner::deallocate` machinery `Box` uses. Removing the outer
//!   `Box` without removing the inner `String` changes nothing.
//! - Nine variants, not boxed, payload holds a plain `u32` instead of a
//!   `String` -- **no heap allocation anywhere in the error at all**:
//!   passes cleanly in 9.6s (32/32 checks, `VERIFICATION:- SUCCESSFUL`).
//! - A fifth case tests a specific inversion a reviewer proposed: instead
//!   of boxing the whole `*Kind` enum (as above), keep the enum's own
//!   discriminant tiny and unboxed -- most variants carry no heap
//!   allocation -- and box only the one heavy variant's own payload
//!   (`enum SmallDiscriminantKind { Tiny(u32), Heavy(Box<Payload>) }`,
//!   never itself wrapped in an outer `Box`). The reasoning behind the
//!   proposal: CBMC's cost should come from modeling the max-sized
//!   variant inline within a boxed union, so moving the `Box` one level
//!   inward -- so it wraps only the field that needs it -- should let
//!   the union itself (and the solver's reasoning about the enum type)
//!   stay small. Confirmed **still out-of-memory**, same
//!   `RawVecInner::deallocate`-driven failure during SSA-to-SAT
//!   conversion, at 18,213 steps / 253 remaining VCCs. This directly
//!   rules out "the whole enum is boxed" as the driver too: moving the
//!   `Box` from around the enum to around just the heavy field, while
//!   *keeping the exact same reachable heap allocation* on the error
//!   path, changes nothing.
//!
//! **First-pass conclusion, later revised below.** The driver is heap
//! allocation itself -- `Box`, `String`, `Vec`, anything backed by
//! `RawVec` -- reachable anywhere in a function's error path, when
//! that function is called (even unstubbed, even unboxed, even with a
//! single trivial variant) from inside another function under
//! `#[kani::proof_for_contract]` analysis. It is not about how many
//! variants the error enum has, not about string *content* (no proof
//! here ever inspected a string's bytes), not specifically about
//! `Box`, and not about *where* the `Box` sits relative to the enum
//! (whole-enum-boxed and only-the-heavy-field-boxed both OOM
//! identically, case five above).
//!
//! **This turned out to be too broad.** A later escalation ladder
//! (further down this file), built one primitive at a time
//! specifically to pull apart what this first pass conflated, found
//! that `Box<u32>`, `Vec<u32>`, a bare `String`, and even `String`
//! living directly inside an enum variant (with or without an extra
//! outer struct layer, boxed or not) **all pass cleanly** on their
//! own. The actual trigger, isolated precisely, is narrower: `Box`
//! wrapping a type that is *itself* already heap-allocating
//! (`Box<String>`, confirmed sufficient entirely on its own, no enum
//! or wrapper struct required) -- a doubled heap indirection, not
//! heap allocation per se. This still fully explains the real crash:
//! `TemporalError`'s `Box<TemporalErrorKind>`, with
//! `InvalidDescriptorSource`'s `String` fields nested inside the
//! boxed enum, is exactly this shape. It's that DFCC's own
//! contract-checking instrumentation, combined with CBMC's heap-object
//! modeling, blows up the SSA-to-SAT encoding when a boxed
//! allocation's own contents require further heap-object modeling --
//! not simply because heap allocation exists somewhere in the
//! picture. The real fix for `amenable_ext::narrow_to_u8` (and any
//! other function reachable this way) remains to avoid *all* heap
//! allocation on its error path specifically under `cfg(kani)`
//! (confirmed sufficient by `no_box_no_heap_fallible` above), while
//! leaving the real, non-Kani behavior (string details, `miette`
//! rendering) completely unchanged -- since nothing in this codebase
//! ever pattern-matches on `TemporalErrorKind`'s variants for its own
//! control flow (confirmed directly: every `TemporalErrorKind::`
//! reference anywhere in `amenable_ext`/`amenable_kani` is a
//! constructor call, never a `match`), the error's internal
//! representation is free to differ under Kani as long as the public
//! shape stays identical.
//!
//! **Open thread, not yet resolved.** `no_box_fallible` above (nine
//! variants, each carrying a two-field `Payload { detail: String,
//! line: u32 }`, with *no* `Box` anywhere under `cfg(kani)`) still
//! OOMs -- which looks like a second, independent route to the same
//! failure, since the later ladder's unboxed `WrappedMixedError`/
//! `MixedKind`/bare-`String` cases all pass. The two variables
//! conflated between them -- variant *count* (9 vs. 2) and payload
//! shape (two-field `Payload` struct vs. a bare `String`) -- have not
//! been pulled apart from each other. Whichever it turns out to be,
//! it doesn't change the recommended fix (remove all heap allocation
//! under `cfg(kani)`, already confirmed sufficient), so this is left
//! as a genuinely open question rather than chased further here.

/// Every item here exists only for the Kani harnesses below, so the whole
/// module is gated once, rather than scattering `#[cfg(kani)]` across each
/// item individually (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    /// Mirrors a real error source's rough shape: a detail string plus
    /// a line number. Identical across both toy types below, so the
    /// only variable that differs between them is the enclosing enum's
    /// variant count.
    #[derive(Debug)]
    pub(super) struct Payload {
        detail: String,
        line: u32,
    }

    impl Payload {
        fn new(detail: &str) -> Self {
            Self {
                detail: detail.to_owned(),
                line: 0,
            }
        }
    }

    /// Mirrors `TemporalErrorKind`'s rough shape: nine variants, each a
    /// clean 1-tuple wrapping a payload struct.
    #[derive(Debug)]
    pub(super) enum NineVariantKind {
        V1(Payload),
        V2(Payload),
        V3(Payload),
        V4(Payload),
        V5(Payload),
        V6(Payload),
        V7(Payload),
        V8(Payload),
        V9(Payload),
    }

    /// Mirrors `TemporalError`'s rough shape: the umbrella kind, boxed.
    pub(super) struct NineVariantError {
        kind: Box<NineVariantKind>,
    }

    impl NineVariantError {
        fn new(kind: NineVariantKind) -> Self {
            Self {
                kind: Box::new(kind),
            }
        }
    }

    /// Exactly `narrow_to_u8`'s own shape: a `u32 -> u8` narrowing that
    /// never actually fails for this harness's bounded domain (`value`
    /// is always `u32::from(some u8)`, so `u8::try_from` always
    /// succeeds), but the `Err` arm's `Box` construction is still
    /// present in the compiled body, same as the real function.
    pub(super) fn nine_variant_fallible(value: u32) -> Result<u8, NineVariantError> {
        u8::try_from(value)
            .map_err(|_err| NineVariantError::new(NineVariantKind::V1(Payload::new("placeholder"))))
    }

    /// The real library never pattern-matches on `TemporalErrorKind`
    /// (or this toy's equivalent) for its own control flow -- confirmed
    /// directly: every `TemporalErrorKind::` reference anywhere in
    /// `amenable_ext`/`amenable_kani` is a *constructor* call, never a
    /// `match`. It's a write-only payload consumed downstream (`miette`,
    /// `Display`, tests, binaries), never inspected internally. So
    /// nothing stops the *representation* from differing under Kani,
    /// as long as the public shape (constructor signature, accessor)
    /// stays identical. Same nine-variant enum as `NineVariantError`
    /// -- same full complexity -- but no `Box`, no heap indirection,
    /// under `cfg(kani)`. Isolates `Box` itself as the variable,
    /// independent of variant count or content.
    pub(super) struct NoBoxError {
        #[cfg(not(kani))]
        kind: Box<NineVariantKind>,
        #[cfg(kani)]
        kind: NineVariantKind,
    }

    impl NoBoxError {
        fn new(kind: NineVariantKind) -> Self {
            #[cfg(not(kani))]
            let kind = Box::new(kind);
            Self { kind }
        }
    }

    /// Identical shape to `nine_variant_fallible`, differing only in
    /// whether the error is boxed.
    pub(super) fn no_box_fallible(value: u32) -> Result<u8, NoBoxError> {
        u8::try_from(value)
            .map_err(|_err| NoBoxError::new(NineVariantKind::V1(Payload::new("placeholder"))))
    }

    /// The same, calling `no_box_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_no_box_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = no_box_fallible(u32::from(v1));
        let _ = no_box_fallible(u32::from(v2));
        let _ = no_box_fallible(u32::from(v3));
    }

    /// Removing `Box` alone didn't help: `Payload`'s own `detail:
    /// String` field is backed by a heap-allocating `Vec<u8>`
    /// internally -- the exact same `RawVecInner::deallocate` machinery
    /// as `Box`. This isolates heap allocation itself as the variable:
    /// same nine-variant enum, no `Box`, but the payload carries a
    /// plain `u32` tag instead of a `String` -- no heap allocation
    /// anywhere in the error at all.
    #[derive(Debug)]
    pub(super) struct NoHeapPayload {
        tag: u32,
    }

    impl NoHeapPayload {
        fn new(tag: u32) -> Self {
            Self { tag }
        }
    }

    #[derive(Debug)]
    pub(super) enum NineVariantKindNoHeap {
        V1(NoHeapPayload),
        V2(NoHeapPayload),
        V3(NoHeapPayload),
        V4(NoHeapPayload),
        V5(NoHeapPayload),
        V6(NoHeapPayload),
        V7(NoHeapPayload),
        V8(NoHeapPayload),
        V9(NoHeapPayload),
    }

    pub(super) struct NoBoxNoHeapError {
        #[cfg(not(kani))]
        kind: Box<NineVariantKindNoHeap>,
        #[cfg(kani)]
        kind: NineVariantKindNoHeap,
    }

    impl NoBoxNoHeapError {
        fn new(kind: NineVariantKindNoHeap) -> Self {
            #[cfg(not(kani))]
            let kind = Box::new(kind);
            Self { kind }
        }
    }

    pub(super) fn no_box_no_heap_fallible(value: u32) -> Result<u8, NoBoxNoHeapError> {
        u8::try_from(value)
            .map_err(|_err| NoBoxNoHeapError::new(NineVariantKindNoHeap::V1(NoHeapPayload::new(0))))
    }

    /// The same, calling `no_box_no_heap_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_no_box_no_heap_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = no_box_no_heap_fallible(u32::from(v1));
        let _ = no_box_no_heap_fallible(u32::from(v2));
        let _ = no_box_no_heap_fallible(u32::from(v3));
    }

    /// The same umbrella shape, restricted to the one variant that's
    /// actually ever producible -- the comparison case.
    #[derive(Debug)]
    pub(super) enum OneVariantKind {
        V1(Payload),
    }

    pub(super) struct OneVariantError {
        kind: Box<OneVariantKind>,
    }

    impl OneVariantError {
        fn new(kind: OneVariantKind) -> Self {
            Self {
                kind: Box::new(kind),
            }
        }
    }

    /// Identical shape to `nine_variant_fallible`, differing only in how
    /// many variants the boxed enum has.
    pub(super) fn one_variant_fallible(value: u32) -> Result<u8, OneVariantError> {
        u8::try_from(value)
            .map_err(|_err| OneVariantError::new(OneVariantKind::V1(Payload::new("placeholder"))))
    }

    /// Tests a specific inversion of the whole-enum-boxed pattern above:
    /// keep the enum's own discriminant tiny -- most variants carry no
    /// heap allocation at all -- and box only the one variant's heavy
    /// payload, rather than boxing the entire enum. The enum itself is
    /// *not* boxed here (contrast `NineVariantError`/`OneVariantError`,
    /// which box the whole `*Kind` enum). If CBMC's cost comes from
    /// having to model the max-sized variant inline within a boxed
    /// union, this should be cheap; if it comes from heap allocation
    /// being reachable at all on the error path (our standing
    /// conclusion above), this should still blow up.
    #[derive(Debug)]
    pub(super) enum SmallDiscriminantKind {
        Tiny(u32),
        Heavy(Box<Payload>),
    }

    pub(super) struct SmallDiscriminantError {
        kind: SmallDiscriminantKind,
    }

    impl SmallDiscriminantError {
        fn new(kind: SmallDiscriminantKind) -> Self {
            Self { kind }
        }
    }

    /// Identical shape to `nine_variant_fallible`'s error path -- always
    /// constructs the one variant that actually carries a payload --
    /// but that variant boxes only its own `Payload`, not the enclosing
    /// enum.
    pub(super) fn small_discriminant_fallible(
        value: u32,
    ) -> Result<u8, SmallDiscriminantError> {
        u8::try_from(value).map_err(|_err| {
            SmallDiscriminantError::new(SmallDiscriminantKind::Heavy(Box::new(Payload::new(
                "placeholder",
            ))))
        })
    }

    /// The same, calling `small_discriminant_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_small_discriminant_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = small_discriminant_fallible(u32::from(v1));
        let _ = small_discriminant_fallible(u32::from(v2));
        let _ = small_discriminant_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 1: the smallest possible heap allocation
    /// reachable on the error path -- a `Box` around a single `u32`,
    /// no struct, no `String`, nothing `RawVec`-backed inside it.
    /// Every boxed case above also had a `String` buried somewhere
    /// inside the box; this isolates whether `Box`'s own allocator call
    /// -- independent of what it points to -- is already sufficient to
    /// trigger the blowup, or whether it's specifically growable/
    /// `RawVec`-backed allocations (`String`, `Vec`) that do.
    pub(super) struct BoxedU32Error {
        value: Box<u32>,
    }

    impl BoxedU32Error {
        fn new(value: u32) -> Self {
            Self {
                value: Box::new(value),
            }
        }
    }

    pub(super) fn boxed_u32_fallible(value: u32) -> Result<u8, BoxedU32Error> {
        u8::try_from(value).map_err(|_err| BoxedU32Error::new(value))
    }

    /// The same, calling `boxed_u32_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_u32_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_u32_fallible(u32::from(v1));
        let _ = boxed_u32_fallible(u32::from(v2));
        let _ = boxed_u32_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 2: `Box<u32>` (rung 1) passed cleanly --
    /// `Box`'s own single fixed-size allocator call is not, by itself,
    /// sufficient to trigger the blowup. This isolates the next
    /// suspect: a growable, `RawVec`-backed allocation, but with none
    /// of `String`'s extra machinery (UTF-8 validity, `Display`
    /// formatting, `Formatter::padding`). A plain `Vec<u32>`, unboxed,
    /// holding exactly one pushed element.
    pub(super) struct VecU32Error {
        value: Vec<u32>,
    }

    impl VecU32Error {
        fn new(value: u32) -> Self {
            Self { value: vec![value] }
        }
    }

    pub(super) fn vec_u32_fallible(value: u32) -> Result<u8, VecU32Error> {
        u8::try_from(value).map_err(|_err| VecU32Error::new(value))
    }

    /// The same, calling `vec_u32_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_vec_u32_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = vec_u32_fallible(u32::from(v1));
        let _ = vec_u32_fallible(u32::from(v2));
        let _ = vec_u32_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 3: `Vec<u32>` (rung 2) also passed
    /// cleanly -- plain `RawVec` growable-allocation machinery, with no
    /// `String` involved, is not sufficient either. This isolates the
    /// last remaining candidate: a bare `String` field, same pattern,
    /// stripped of any enum/variant context (unlike every earlier
    /// failing case, where `String` always sat inside a `Payload`
    /// struct inside an enum variant). A literal, never-displayed
    /// `.to_owned()` string, identical every call.
    pub(super) struct StringError {
        value: String,
    }

    impl StringError {
        fn new(value: &str) -> Self {
            Self {
                value: value.to_owned(),
            }
        }
    }

    pub(super) fn string_fallible(value: u32) -> Result<u8, StringError> {
        u8::try_from(value).map_err(|_err| StringError::new("placeholder"))
    }

    /// The same, calling `string_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_string_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = string_fallible(u32::from(v1));
        let _ = string_fallible(u32::from(v2));
        let _ = string_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 4: `String` on its own (rung 3) also
    /// passed cleanly -- bare heap text, with no enum/variant context
    /// at all, is not sufficient either. `Box<u32>`, `Vec<u32>`, and
    /// bare `String` all pass individually, yet `small_discriminant`
    /// above (an enum with a tiny arm and a `Box<Payload>` arm) still
    /// OOMs. This isolates what's actually different between them: a
    /// heap-backed payload reachable through *one arm of a sum type*
    /// sitting next to a heap-free arm -- no extra `Box`, no separate
    /// `Payload` struct, just the plain enum itself holding `String`
    /// directly in one variant.
    pub(super) enum MixedKind {
        Tiny(u32),
        Heavy(String),
    }

    pub(super) fn mixed_fallible(value: u32) -> Result<u8, MixedKind> {
        u8::try_from(value).map_err(|_err| MixedKind::Heavy("placeholder".to_owned()))
    }

    /// The same, calling `mixed_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_mixed_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = mixed_fallible(u32::from(v1));
        let _ = mixed_fallible(u32::from(v2));
        let _ = mixed_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 5: `MixedKind` (rung 4) passed cleanly
    /// too -- a plain enum, `Tiny(u32)` next to `Heavy(String)`,
    /// returned directly as the error type, is not sufficient. But
    /// `small_discriminant` above (same tiny-arm/heap-arm shape) still
    /// OOMs, and the one structural difference is that it always sat
    /// behind an *extra outer struct* wrapping the enum
    /// (`SmallDiscriminantError { kind: SmallDiscriminantKind }`,
    /// mirroring the real `TemporalError { kind: ... }` shape) --
    /// `MixedKind` above was returned directly, with no wrapper. This
    /// isolates nesting depth itself, independent of `Box`: the exact
    /// same `MixedKind` enum, now wrapped in one extra struct layer,
    /// still with no `Box` anywhere.
    pub(super) struct WrappedMixedError {
        kind: MixedKind,
    }

    impl WrappedMixedError {
        fn new(kind: MixedKind) -> Self {
            Self { kind }
        }
    }

    pub(super) fn wrapped_mixed_fallible(value: u32) -> Result<u8, WrappedMixedError> {
        u8::try_from(value)
            .map_err(|_err| WrappedMixedError::new(MixedKind::Heavy("placeholder".to_owned())))
    }

    /// The same, calling `wrapped_mixed_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_wrapped_mixed_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = wrapped_mixed_fallible(u32::from(v1));
        let _ = wrapped_mixed_fallible(u32::from(v2));
        let _ = wrapped_mixed_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 6: `WrappedMixedError` (rung 5) also
    /// passed cleanly -- adding the outer struct layer alone, still
    /// with no `Box`, is not sufficient. Only two conflated
    /// differences remain between this passing shape and the OOMing
    /// `small_discriminant` case: (1) `Box` wrapping the heap payload,
    /// and (2) the payload being a separate two-field `Payload` struct
    /// (`detail: String, line: u32`) rather than a bare `String`.
    /// Pulling those apart: change only the `Box`, keep the payload a
    /// single bare `String` (now `Box<String>` -- a `Box` pointing to
    /// an already heap-allocating type, double indirection), still
    /// wrapped in the same outer struct.
    pub(super) enum BoxedStringKind {
        Tiny(u32),
        Heavy(Box<String>),
    }

    pub(super) struct WrappedBoxedStringError {
        kind: BoxedStringKind,
    }

    impl WrappedBoxedStringError {
        fn new(kind: BoxedStringKind) -> Self {
            Self { kind }
        }
    }

    pub(super) fn wrapped_boxed_string_fallible(
        value: u32,
    ) -> Result<u8, WrappedBoxedStringError> {
        u8::try_from(value).map_err(|_err| {
            WrappedBoxedStringError::new(BoxedStringKind::Heavy(Box::new(
                "placeholder".to_owned(),
            )))
        })
    }

    /// The same, calling `wrapped_boxed_string_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_wrapped_boxed_string_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = wrapped_boxed_string_fallible(u32::from(v1));
        let _ = wrapped_boxed_string_fallible(u32::from(v2));
        let _ = wrapped_boxed_string_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 7: `WrappedBoxedStringError` (rung 6)
    /// -- the same outer-struct-wrapped enum shape as the passing
    /// `WrappedMixedError`, but `Heavy` now holds `Box<String>`
    /// (double heap indirection) instead of a bare `String` -- is
    /// **confirmed out-of-memory** (`RawVecInner::deallocate`-driven,
    /// 17,971 steps / 253 VCCs). Found it: `Box<String>` is the first
    /// configuration on this ladder that reproduces the blowup, where
    /// bare `String` in the identical position passed. One more pull
    /// to separate the last conflated variable: does `Box<String>`
    /// need the enum/sum-type context to trigger it, or is it
    /// sufficient entirely on its own -- no enum, no outer struct
    /// beyond the error type itself?
    pub(super) struct BoxedStringOnlyError {
        value: Box<String>,
    }

    impl BoxedStringOnlyError {
        fn new(value: &str) -> Self {
            Self {
                value: Box::new(value.to_owned()),
            }
        }
    }

    pub(super) fn boxed_string_only_fallible(value: u32) -> Result<u8, BoxedStringOnlyError> {
        u8::try_from(value).map_err(|_err| BoxedStringOnlyError::new("placeholder"))
    }

    /// The same, calling `boxed_string_only_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_string_only_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_string_only_fallible(u32::from(v1));
        let _ = boxed_string_only_fallible(u32::from(v2));
        let _ = boxed_string_only_fallible(u32::from(v3));
    }

    /// Three real, inlined calls to `nine_variant_fallible`, wrapped in
    /// a trivially-contracted function -- the exact shape that crashed
    /// for the real `narrow_to_u8`.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_nine_variant_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = nine_variant_fallible(u32::from(v1));
        let _ = nine_variant_fallible(u32::from(v2));
        let _ = nine_variant_fallible(u32::from(v3));
    }

    /// The same, calling `one_variant_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_one_variant_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = one_variant_fallible(u32::from(v1));
        let _ = one_variant_fallible(u32::from(v2));
        let _ = one_variant_fallible(u32::from(v3));
    }
}

#[cfg(kani)]
use kani_only::{
    call_boxed_string_only_thrice_contracted, call_boxed_u32_thrice_contracted,
    call_mixed_thrice_contracted, call_nine_variant_thrice_contracted,
    call_no_box_no_heap_thrice_contracted, call_no_box_thrice_contracted,
    call_one_variant_thrice_contracted, call_small_discriminant_thrice_contracted,
    call_string_thrice_contracted, call_vec_u32_thrice_contracted,
    call_wrapped_boxed_string_thrice_contracted, call_wrapped_mixed_thrice_contracted,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::nine_variant_boxed_error_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_nine_variant_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "three calls to a narrow_to_u8-shaped function returning Result<u8, Box<NineVariantKind>>, wrapped in a trivially-contracted caller -- confirmed reproduces the real out-of-memory CBMC failure exactly (Out of memory during SSA conversion)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_NINE_VARIANT_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_nine_variant_thrice_contracted)]
        fn check_call_nine_variant_thrice_contracted() {
            call_nine_variant_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::one_variant_boxed_error_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_one_variant_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the identical shape, but the boxed enum has only one variant -- confirmed STILL runs out of memory during SSA conversion even at a tiny 17,027-step program size, ruling out variant count/content as the driver".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_ONE_VARIANT_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_one_variant_thrice_contracted)]
        fn check_call_one_variant_thrice_contracted() {
            call_one_variant_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::no_box_error_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_no_box_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the full nine-variant enum, never boxed under cfg(kani) -- confirmed STILL runs out of memory: Payload's own String field is backed by heap-allocating Vec<u8> internally, the same RawVecInner::deallocate machinery as Box -- removing the outer Box alone didn't remove the heap allocation".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_NO_BOX_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_no_box_thrice_contracted)]
        fn check_call_no_box_thrice_contracted() {
            call_no_box_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::no_box_no_heap_error_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_no_box_no_heap_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the full nine-variant enum, never boxed, payload carries a plain u32 tag instead of a String -- no heap allocation anywhere in the error at all -- confirmed passes in 9.6s: removing ALL heap allocation (not just Box) resolves the blowup completely".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_NO_BOX_NO_HEAP_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_no_box_no_heap_thrice_contracted)]
        fn check_call_no_box_no_heap_thrice_contracted() {
            call_no_box_no_heap_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::small_discriminant_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_small_discriminant_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the reviewer-proposed inversion: enum discriminant kept tiny and unboxed, only the one heavy variant boxes its own Payload -- confirmed STILL out-of-memory (RawVecInner::deallocate, 18,213 steps / 253 VCCs), ruling out 'the whole enum is boxed' as the driver: moving the Box one level inward while keeping the same reachable heap allocation changes nothing".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_SMALL_DISCRIMINANT_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_small_discriminant_thrice_contracted)]
        fn check_call_small_discriminant_thrice_contracted() {
            call_small_discriminant_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_u32_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_u32_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 1: a Box around a single u32, no struct, no String, nothing RawVec-backed inside -- confirmed PASSES, 36.7s, 101/101 checks: Box's own fixed-size allocator call alone is not sufficient to trigger the blowup".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_U32_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_u32_thrice_contracted)]
        fn check_call_boxed_u32_thrice_contracted() {
            call_boxed_u32_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::vec_u32_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_vec_u32_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 2: a plain, unboxed Vec<u32> holding one pushed element -- no String, no UTF-8/Display machinery, just RawVec's own growable allocation -- confirmed PASSES, 55.3s, 134/134 checks: RawVec's growable-allocation machinery alone, without String, is also not sufficient".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_VEC_U32_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_vec_u32_thrice_contracted)]
        fn check_call_vec_u32_thrice_contracted() {
            call_vec_u32_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::string_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_string_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 3: a bare String field, stripped of any enum/variant context, one fixed never-displayed literal -- confirmed PASSES, 84.8s, 246/246 checks: String on its own, outside a sum type, is also not sufficient".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_STRING_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_string_thrice_contracted)]
        fn check_call_string_thrice_contracted() {
            call_string_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::mixed_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_mixed_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 4: a plain enum, no extra Box, no separate Payload struct -- Tiny(u32) next to Heavy(String) -- confirmed PASSES, 83.5s, 252/252 checks: a mixed-size enum arm with a heap payload, returned directly as the error type (no outer wrapper struct), is also not sufficient".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_MIXED_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_mixed_thrice_contracted)]
        fn check_call_mixed_thrice_contracted() {
            call_mixed_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::wrapped_mixed_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_wrapped_mixed_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 5: the same MixedKind enum, now wrapped in one extra outer struct layer (mirroring TemporalError's shape), still with no Box anywhere -- confirmed PASSES, 83.4s, 252/252 checks: nesting depth alone, without Box, is also not sufficient".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_WRAPPED_MIXED_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_wrapped_mixed_thrice_contracted)]
        fn check_call_wrapped_mixed_thrice_contracted() {
            call_wrapped_mixed_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::wrapped_boxed_string_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_wrapped_boxed_string_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 6: same outer-struct-wrapped enum shape, but the heavy variant now holds Box<String> (double heap indirection) instead of a bare String -- confirmed OUT OF MEMORY (RawVecInner::deallocate, 17,971 steps / 253 VCCs): the first configuration on this ladder to reproduce the blowup".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_WRAPPED_BOXED_STRING_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_wrapped_boxed_string_thrice_contracted)]
        fn check_call_wrapped_boxed_string_thrice_contracted() {
            call_wrapped_boxed_string_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_string_only_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_string_only_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 7: Box<String> entirely on its own -- no enum, no sum-type context, no extra outer struct layer -- confirmed OUT OF MEMORY (RawVecInner::deallocate, 16,685 steps / 246 VCCs): the enum/sum-type context from rung 6 was never required -- Box wrapping an already heap-allocating type is sufficient by itself".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_STRING_ONLY_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_string_only_thrice_contracted)]
        fn check_call_boxed_string_only_thrice_contracted() {
            call_boxed_string_only_thrice_contracted();
        }
    }
}
