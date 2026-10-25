//! Escalation ladder, rungs 1-5, for the CBMC out-of-memory
//! investigation -- see `boxed_error_variant_count.rs`'s own module doc
//! comment for the full narrative this file is one chapter of. The
//! first-pass conclusion in `boxed_error_variant_count_first_pass.rs`
//! ("heap allocation itself is the driver") turned out to be too
//! broad; this ladder was built one primitive at a time to pull that
//! apart. This rung range: `Box<u32>` (passes), `Vec<u32>` (passes), a
//! bare `String` (passes), `String` inside an enum variant (passes),
//! and the same wrapped in an extra struct layer (passes) -- every
//! rung here is a real pass, narrowing the trigger away from plain
//! heap allocation. Continues in
//! `boxed_error_variant_count_escalation_string_boxing.rs` (rung 6,
//! where it finally OOMs again) and
//! `boxed_error_variant_count_escalation_references_and_vectors.rs`
//! (rungs 7-11).

mod kani_only {
    #![cfg(kani)]

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
    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct StringError {
        #[new(into)]
        value: String,
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
    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct WrappedMixedError {
        kind: MixedKind,
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

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let boxed_u32 = BoxedU32Error::new(42);
        assert_eq!(*boxed_u32.value, 42);

        let vec_u32 = VecU32Error::new(9);
        assert_eq!(*vec_u32.value, vec![9]);

        let string_err = StringError::new("placeholder");
        assert_eq!(string_err.value, "placeholder");

        let mixed_tiny = MixedKind::Tiny(3);
        assert!(matches!(mixed_tiny, MixedKind::Tiny(3)));
        let MixedKind::Heavy(ref s) = MixedKind::Heavy("placeholder".to_owned()) else {
            unreachable!()
        };
        assert_eq!(s, "placeholder");
        let wrapped_mixed = WrappedMixedError::new(MixedKind::Heavy("placeholder".to_owned()));
        let MixedKind::Heavy(ref s) = wrapped_mixed.kind else {
            unreachable!()
        };
        assert_eq!(s, "placeholder");
    }
}

#[cfg(kani)]
use kani_only::{
    call_boxed_u32_thrice_contracted, call_mixed_thrice_contracted, call_string_thrice_contracted,
    call_vec_u32_thrice_contracted, call_wrapped_mixed_thrice_contracted,
};

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
