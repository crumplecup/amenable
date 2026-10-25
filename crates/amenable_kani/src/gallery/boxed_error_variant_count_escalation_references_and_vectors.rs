//! Escalation ladder, rungs 7 and 9-11 (reference/`Vec` rungs), for
//! the CBMC out-of-memory investigation -- see
//! `boxed_error_variant_count.rs`'s own module doc comment for the full
//! narrative this file is one chapter of. Rung 7: a bare `&'static
//! str` field (passes -- a fat pointer, no allocation). Rung 9:
//! `Box<&'static str>` (also passes -- confirms a reference, boxed or
//! not, is never the problem). Rungs 10-11: `Box<Vec<u32>>` and
//! `Box<Vec<u8>>` (both pass, ruling out "any `RawVec`-owning type
//! behind a `Box`" -- the true boundary is specific to `String`, not
//! shared with `Vec`). Rung 6 (`WrappedBoxedStringError`, the one that
//! OOMs) lives in the sibling
//! `boxed_error_variant_count_escalation_string_boxing.rs`, alongside
//! its two "is the `String` itself bounded?" follow-ups (rungs 12-13),
//! since both reuse that file's own `BoxedStringOnlyError` type.

mod kani_only {
    #![cfg(kani)]

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

    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct WrappedBoxedStringError {
        kind: BoxedStringKind,
    }

    pub(super) fn wrapped_boxed_string_fallible(value: u32) -> Result<u8, WrappedBoxedStringError> {
        u8::try_from(value).map_err(|_err| {
            WrappedBoxedStringError::new(BoxedStringKind::Heavy(Box::new("placeholder".to_owned())))
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

    /// Escalation ladder, rung 8: a bare `&'static str` field -- a fat
    /// pointer (ptr + length) into the binary's static data. No
    /// allocation, no `Drop` impl, nothing for `RawVec` to touch.
    /// Isolates whether Kani handles `&'static str` at all, and
    /// whether a plain reference field (not owning any heap memory)
    /// behaves like the passing `u32` cases above.
    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct StaticStrError {
        value: &'static str,
    }

    pub(super) fn static_str_fallible(value: u32) -> Result<u8, StaticStrError> {
        u8::try_from(value).map_err(|_err| StaticStrError::new("placeholder"))
    }

    /// The same, calling `static_str_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_static_str_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = static_str_fallible(u32::from(v1));
        let _ = static_str_fallible(u32::from(v2));
        let _ = static_str_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 9: `Box<&'static str>` -- a `Box`
    /// wrapping a pointer, but that pointer owns no heap allocation of
    /// its own (unlike `Box<String>`, confirmed to OOM). Sharpens the
    /// refined hypothesis from rung 7: is the trigger "`Box` wrapping
    /// *anything containing a pointer*", or specifically "`Box`
    /// wrapping a type that *owns* a heap allocation (has `RawVec`/
    /// `Drop`-driven deallocation)"? `&'static str` has neither.
    pub(super) struct BoxedStaticStrError {
        value: Box<&'static str>,
    }

    impl BoxedStaticStrError {
        fn new(value: &'static str) -> Self {
            Self {
                value: Box::new(value),
            }
        }
    }

    pub(super) fn boxed_static_str_fallible(value: u32) -> Result<u8, BoxedStaticStrError> {
        u8::try_from(value).map_err(|_err| BoxedStaticStrError::new("placeholder"))
    }

    /// The same, calling `boxed_static_str_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_static_str_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_static_str_fallible(u32::from(v1));
        let _ = boxed_static_str_fallible(u32::from(v2));
        let _ = boxed_static_str_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 10: `Box<Vec<u32>>` -- a direct
    /// prediction from rung 9's refined rule ("`Box` wraps a type
    /// that *owns* a heap allocation"). `Vec<u32>` (rung 2) is
    /// `RawVec`-backed exactly like `String`, just without `String`'s
    /// UTF-8/`Display` machinery. If the rule holds, this should OOM
    /// the same way `Box<String>` did.
    pub(super) struct BoxedVecError {
        value: Box<Vec<u32>>,
    }

    impl BoxedVecError {
        fn new(value: u32) -> Self {
            Self {
                value: Box::new(vec![value]),
            }
        }
    }

    pub(super) fn boxed_vec_fallible(value: u32) -> Result<u8, BoxedVecError> {
        u8::try_from(value).map_err(|_err| BoxedVecError::new(value))
    }

    /// The same, calling `boxed_vec_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_vec_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_vec_fallible(u32::from(v1));
        let _ = boxed_vec_fallible(u32::from(v2));
        let _ = boxed_vec_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 11: `Box<Vec<u8>>` -- `Box<Vec<u32>>`
    /// (rung 10) unexpectedly passed, refuting "`Box` wraps a type
    /// that owns a heap allocation" as stated: `Vec<u32>` is
    /// `RawVec`-backed exactly like `String`, yet only `Box<String>`
    /// OOMs. This isolates the one remaining difference: element
    /// type. `String` is internally `Vec<u8>`; this tests `Vec<u8>`
    /// directly, same backing byte type as `String`, still no
    /// `String`-specific UTF-8/`Display` machinery.
    pub(super) struct BoxedVecU8Error {
        value: Box<Vec<u8>>,
    }

    impl BoxedVecU8Error {
        fn new(value: u8) -> Self {
            Self {
                value: Box::new(vec![value]),
            }
        }
    }

    pub(super) fn boxed_vec_u8_fallible(value: u32) -> Result<u8, BoxedVecU8Error> {
        u8::try_from(value).map_err(|_err| BoxedVecU8Error::new(0))
    }

    /// The same, calling `boxed_vec_u8_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_vec_u8_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_vec_u8_fallible(u32::from(v1));
        let _ = boxed_vec_u8_fallible(u32::from(v2));
        let _ = boxed_vec_u8_fallible(u32::from(v3));
    }

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let boxed_string_tiny = BoxedStringKind::Tiny(3);
        assert!(matches!(boxed_string_tiny, BoxedStringKind::Tiny(3)));
        let BoxedStringKind::Heavy(ref boxed) =
            BoxedStringKind::Heavy(Box::new("placeholder".to_owned()))
        else {
            unreachable!()
        };
        assert_eq!(**boxed, "placeholder");
        let wrapped_boxed_string = WrappedBoxedStringError::new(BoxedStringKind::Heavy(Box::new(
            "placeholder".to_owned(),
        )));
        let BoxedStringKind::Heavy(ref boxed) = wrapped_boxed_string.kind else {
            unreachable!()
        };
        assert_eq!(**boxed, "placeholder");

        let static_str = StaticStrError::new("placeholder");
        assert_eq!(static_str.value, "placeholder");

        let boxed_static_str = BoxedStaticStrError::new("placeholder");
        assert_eq!(*boxed_static_str.value, "placeholder");

        let boxed_vec = BoxedVecError::new(11);
        assert_eq!(*boxed_vec.value, vec![11]);

        let boxed_vec_u8 = BoxedVecU8Error::new(5);
        assert_eq!(*boxed_vec_u8.value, vec![5]);
    }
}

#[cfg(kani)]
use kani_only::{
    call_boxed_static_str_thrice_contracted, call_boxed_vec_thrice_contracted,
    call_boxed_vec_u8_thrice_contracted, call_static_str_thrice_contracted,
    call_wrapped_boxed_string_thrice_contracted,
};

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
            "amenable_kani::gallery::boxed_error_variant_count::static_str_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_static_str_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 8: a bare &'static str field -- a fat pointer into static data, no allocation, no Drop impl -- confirmed PASSES, 9.5s, 32/32 checks: Kani handles references with no trouble at all, the fastest rung on this whole ladder".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_STATIC_STR_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_static_str_thrice_contracted)]
        fn check_call_static_str_thrice_contracted() {
            call_static_str_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_static_str_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_static_str_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 9: Box<&'static str> -- a Box wrapping a pointer that owns no heap allocation of its own -- confirmed PASSES, 29.8s, 95/95 checks: resolves rung 7/8's open question decisively -- the trigger is specifically Box wrapping a type that OWNS a heap allocation (RawVec/Drop-driven deallocation), not Box wrapping anything that merely contains a pointer. A reference, boxed or not, is never the problem.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_STATIC_STR_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_static_str_thrice_contracted)]
        fn check_call_boxed_static_str_thrice_contracted() {
            call_boxed_static_str_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_vec_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_vec_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 10: Box<Vec<u32>> -- a direct prediction from rung 9's rule, since Vec is RawVec-backed exactly like String -- confirmed PASSES, 76.5s, 159/159 checks: REFUTES the rung-9 rule as stated. Vec<u32> and String are both RawVec-backed, yet Box<Vec<u32>> passes where Box<String> OOMs. 'Box wraps a type that owns a heap allocation' is not sufficient on its own -- something more specific to String (or its u8 element type) is required too.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_VEC_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_vec_thrice_contracted)]
        fn check_call_boxed_vec_thrice_contracted() {
            call_boxed_vec_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_vec_u8_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_vec_u8_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 11: Box<Vec<u8>> -- same backing byte type as String's internal Vec<u8>, isolated from rung 10's u32 element and from String's own UTF-8/Display machinery -- confirmed PASSES, 67.0s, 152/152 checks: element type (u8 vs u32) is not the differentiator either. Box<Vec<T>> passes for any T tried; only Box<String> OOMs. The trigger is specific to String itself, not to RawVec/Vec ownership in general.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_VEC_U8_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_vec_u8_thrice_contracted)]
        fn check_call_boxed_vec_u8_thrice_contracted() {
            call_boxed_vec_u8_thrice_contracted();
        }
    }
}
