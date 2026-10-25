//! First-pass hypothesis test, variant-count rungs, for the CBMC
//! out-of-memory investigation -- see `boxed_error_variant_count.rs`'s
//! own module doc comment for the full narrative this file is one
//! chapter of. Tests variant count and the presence of `Box`: a
//! nine-variant boxed enum (reproduces the real crash), a one-variant
//! boxed enum (still OOMs, disproving the variant-count hypothesis),
//! the same nine-variant enum left unboxed with its `String` payload
//! kept (still OOMs, disproving "`Box` alone is the cause"), and the
//! same unboxed with a plain `u32` payload instead (passes cleanly, no
//! heap allocation anywhere). The box-placement inversion (is it the
//! whole enum being boxed, or just the heavy variant's own payload?)
//! lives in the sibling
//! `boxed_error_variant_count_first_pass_box_placement.rs`.

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
    #[derive(Debug, derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct NoHeapPayload {
        tag: u32,
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

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let payload = Payload::new("placeholder");
        assert_eq!(payload.detail, "placeholder");
        assert_eq!(payload.line, 0);

        for kind in [
            NineVariantKind::V1(Payload::new("v1")),
            NineVariantKind::V2(Payload::new("v2")),
            NineVariantKind::V3(Payload::new("v3")),
            NineVariantKind::V4(Payload::new("v4")),
            NineVariantKind::V5(Payload::new("v5")),
            NineVariantKind::V6(Payload::new("v6")),
            NineVariantKind::V7(Payload::new("v7")),
            NineVariantKind::V8(Payload::new("v8")),
            NineVariantKind::V9(Payload::new("v9")),
        ] {
            let (NineVariantKind::V1(ref p)
            | NineVariantKind::V2(ref p)
            | NineVariantKind::V3(ref p)
            | NineVariantKind::V4(ref p)
            | NineVariantKind::V5(ref p)
            | NineVariantKind::V6(ref p)
            | NineVariantKind::V7(ref p)
            | NineVariantKind::V8(ref p)
            | NineVariantKind::V9(ref p)) = kind;
            assert!(!p.detail.is_empty());
            let err = NineVariantError::new(kind);
            let (NineVariantKind::V1(ref p)
            | NineVariantKind::V2(ref p)
            | NineVariantKind::V3(ref p)
            | NineVariantKind::V4(ref p)
            | NineVariantKind::V5(ref p)
            | NineVariantKind::V6(ref p)
            | NineVariantKind::V7(ref p)
            | NineVariantKind::V8(ref p)
            | NineVariantKind::V9(ref p)) = *err.kind;
            assert!(!p.detail.is_empty());
        }

        let no_box = NoBoxError::new(NineVariantKind::V1(Payload::new("no_box")));
        assert!(matches!(no_box.kind, NineVariantKind::V1(_)));

        let no_heap = NoHeapPayload::new(7);
        assert_eq!(no_heap.tag, 7);

        for kind in [
            NineVariantKindNoHeap::V1(NoHeapPayload::new(1)),
            NineVariantKindNoHeap::V2(NoHeapPayload::new(2)),
            NineVariantKindNoHeap::V3(NoHeapPayload::new(3)),
            NineVariantKindNoHeap::V4(NoHeapPayload::new(4)),
            NineVariantKindNoHeap::V5(NoHeapPayload::new(5)),
            NineVariantKindNoHeap::V6(NoHeapPayload::new(6)),
            NineVariantKindNoHeap::V7(NoHeapPayload::new(7)),
            NineVariantKindNoHeap::V8(NoHeapPayload::new(8)),
            NineVariantKindNoHeap::V9(NoHeapPayload::new(9)),
        ] {
            let (NineVariantKindNoHeap::V1(ref p)
            | NineVariantKindNoHeap::V2(ref p)
            | NineVariantKindNoHeap::V3(ref p)
            | NineVariantKindNoHeap::V4(ref p)
            | NineVariantKindNoHeap::V5(ref p)
            | NineVariantKindNoHeap::V6(ref p)
            | NineVariantKindNoHeap::V7(ref p)
            | NineVariantKindNoHeap::V8(ref p)
            | NineVariantKindNoHeap::V9(ref p)) = kind;
            let tag = p.tag;
            let no_box_no_heap = NoBoxNoHeapError::new(kind);
            let (NineVariantKindNoHeap::V1(ref p)
            | NineVariantKindNoHeap::V2(ref p)
            | NineVariantKindNoHeap::V3(ref p)
            | NineVariantKindNoHeap::V4(ref p)
            | NineVariantKindNoHeap::V5(ref p)
            | NineVariantKindNoHeap::V6(ref p)
            | NineVariantKindNoHeap::V7(ref p)
            | NineVariantKindNoHeap::V8(ref p)
            | NineVariantKindNoHeap::V9(ref p)) = no_box_no_heap.kind;
            assert_eq!(p.tag, tag);
        }

        let OneVariantKind::V1(ref p) = OneVariantKind::V1(Payload::new("one_variant"));
        assert!(!p.detail.is_empty());
        let one = OneVariantError::new(OneVariantKind::V1(Payload::new("one_variant")));
        let OneVariantKind::V1(ref p) = *one.kind;
        assert!(!p.detail.is_empty());
    }
}

#[cfg(kani)]
use kani_only::{
    call_nine_variant_thrice_contracted, call_no_box_no_heap_thrice_contracted,
    call_no_box_thrice_contracted, call_one_variant_thrice_contracted,
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
