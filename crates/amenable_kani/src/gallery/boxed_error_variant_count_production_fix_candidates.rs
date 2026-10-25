//! Production-fix candidates for the CBMC out-of-memory investigation
//! -- see `boxed_error_variant_count.rs`'s own module doc comment for
//! the full narrative this file is one chapter of. Once the escalation
//! ladder (the sibling `boxed_error_variant_count_escalation_*.rs`
//! files) pinned the trigger down to `Box<String>` specifically, these
//! cases validate real candidate fixes against the most faithful
//! possible recreation of the actual blocker: a boxed `Kind` enum
//! wrapping a multi-field `Source` struct with a string-like field,
//! matching `TemporalError`/`TemporalErrorKind::InvalidDescriptor`
//! almost exactly. `ProxyError` swaps `detail` for a `KaniUtf8Buffer<8>`
//! (justified by the demonstrated `String`<->`KaniUtf8Buffer`
//! refinement); `StaticStrProxyError`/`StaticStrRefError` swap it for a
//! `&'static str` instead. All three pass cleanly -- real, validated
//! fix candidates, not just a theory. `BoxedKaniUtf8BufferError` (the
//! escalation ladder's own rung 14, filed here instead since it
//! belongs with the other fix candidates) confirms the same holds for
//! a bare `Box<KaniUtf8Buffer<8>>`, with no enum/wrapper layering at
//! all.

mod kani_only {
    #![cfg(kani)]

    /// The general tactic this whole gallery file's investigation
    /// points to, demonstrated on the most faithful possible
    /// recreation of the real blocker -- not just a bare
    /// `Box<KaniUtf8Buffer>` (rung 14 below), but this exact shape:
    /// a boxed `Kind` enum wrapping a multi-field `Source` struct
    /// with a string-like field, matching `TemporalError { kind:
    /// Box<TemporalErrorKind> }` / `TemporalErrorKind::
    /// InvalidDescriptor(InvalidDescriptorSource { detail: String,
    /// line: u32, .. })` almost exactly. `ProxyPayload` is `Payload`
    /// (confirmed to OOM when boxed, `one_variant_fallible` above)
    /// with its one real structural change: `detail` is a
    /// `KaniUtf8Buffer<8>` instead of a `String`, justified by the
    /// demonstrated (not assumed) `String`<->`KaniUtf8Buffer`
    /// refinement (`gallery::string_buffer_correspondence`) rather
    /// than an unjustified hack -- any property that would hold for
    /// the real `String`-backed version transfers to this one.
    #[derive(Debug)]
    pub(super) struct ProxyPayload {
        detail: crate::KaniUtf8Buffer<8>,
        line: u32,
    }

    #[derive(Debug)]
    pub(super) enum ProxyKind {
        V1(ProxyPayload),
    }

    pub(super) struct ProxyError {
        kind: Box<ProxyKind>,
    }

    impl ProxyError {
        fn new(kind: ProxyKind) -> Self {
            Self {
                kind: Box::new(kind),
            }
        }
    }

    /// Identical shape to `one_variant_fallible`, with the one
    /// structural change the proxy tactic makes: `detail` is a
    /// genuinely symbolic `KaniUtf8Buffer<8>` (not a fixed literal
    /// like `one_variant_fallible`'s "placeholder" -- a stronger
    /// claim than that comparison needs, since the proxy covers any
    /// content, not one fixed string) instead of `String`.
    pub(super) fn proxy_fallible(value: u32) -> Result<u8, ProxyError> {
        u8::try_from(value).map_err(|_err| {
            let detail: crate::KaniUtf8Buffer<8> = kani::any();
            ProxyError::new(ProxyKind::V1(ProxyPayload { detail, line: 0 }))
        })
    }

    /// The same, calling `proxy_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_proxy_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = proxy_fallible(u32::from(v1));
        let _ = proxy_fallible(u32::from(v2));
        let _ = proxy_fallible(u32::from(v3));
    }

    /// A third option, alongside the real `String` (OOM, above) and
    /// the `KaniUtf8Buffer` proxy (cheap, above): what if
    /// `TemporalError`'s `detail` fields were simply `&'static str`
    /// in production, not `String`, not a `cfg(kani)`-only swap?
    /// Every real call site observed so far (`amenable_ext::chrono`'s
    /// `narrow_to_u8`/`fixed_offset_to_utc_offset_descriptor`) passes
    /// a static string literal into a field whose only reason to be
    /// `String` is an `impl Into<String>` signature -- if that's true
    /// everywhere, `&'static str` would remove the heap allocation
    /// unconditionally, in real builds too, not just under Kani.
    /// `StaticStrProxyPayload` is `ProxyPayload` with that one change:
    /// `detail: &'static str` instead of `KaniUtf8Buffer<8>` (named
    /// to avoid colliding with the unrelated, already-existing bare
    /// `StaticStrError`/`static_str_fallible` at rung 8 below, which
    /// tests a plain `&'static str` field with no enclosing enum at
    /// all). Same umbrella shape otherwise -- boxed `Kind` enum,
    /// multi-field `Source`-like payload -- for a direct,
    /// apples-to-apples comparison against both `OneVariantError`
    /// and `ProxyError`.
    #[derive(Debug)]
    pub(super) struct StaticStrProxyPayload {
        detail: &'static str,
        line: u32,
    }

    #[derive(Debug)]
    pub(super) enum StaticStrProxyKind {
        V1(StaticStrProxyPayload),
    }

    pub(super) struct StaticStrProxyError {
        kind: Box<StaticStrProxyKind>,
    }

    impl StaticStrProxyError {
        fn new(kind: StaticStrProxyKind) -> Self {
            Self {
                kind: Box::new(kind),
            }
        }
    }

    /// Identical shape to `one_variant_fallible`/`proxy_fallible`,
    /// with `detail` a `&'static str` instead.
    pub(super) fn static_str_proxy_fallible(value: u32) -> Result<u8, StaticStrProxyError> {
        u8::try_from(value).map_err(|_err| {
            StaticStrProxyError::new(StaticStrProxyKind::V1(StaticStrProxyPayload {
                detail: "placeholder",
                line: 0,
            }))
        })
    }

    /// The same, calling `static_str_proxy_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_static_str_proxy_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = static_str_proxy_fallible(u32::from(v1));
        let _ = static_str_proxy_fallible(u32::from(v2));
        let _ = static_str_proxy_fallible(u32::from(v3));
    }

    /// A fourth option, still with `detail: &'static str`: instead of
    /// boxing the `Kind` enum (an owned allocation, even though
    /// `&'static str` itself owns nothing), the parent error holds a
    /// plain `&'static` reference to it. This only works at all
    /// because every field of `StaticStrRefKind` is itself
    /// const-constructible (`&'static str` literal, `u32` literal),
    /// so the whole value can live in a `static` -- there is no
    /// dynamically-computed content to point at here. Real call sites
    /// that always pass static literals and a small number of
    /// distinct `(detail, line)` combinations could in principle
    /// each get their own `static`, but that is a much bigger design
    /// commitment than the other three options above, which each
    /// construct the value fresh, in place, from arbitrary runtime
    /// inputs. Testing purely whether Kani handles the reference
    /// shape at all, not endorsing it as realistic yet.
    #[derive(Debug)]
    pub(super) struct StaticStrRefPayload {
        detail: &'static str,
        line: u32,
    }

    #[derive(Debug)]
    pub(super) enum StaticStrRefKind {
        V1(StaticStrRefPayload),
    }

    static STATIC_STR_REF_PLACEHOLDER: StaticStrRefKind =
        StaticStrRefKind::V1(StaticStrRefPayload {
            detail: "placeholder",
            line: 0,
        });

    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct StaticStrRefError {
        kind: &'static StaticStrRefKind,
    }

    /// Identical shape to `static_str_proxy_fallible`, with the
    /// parent error holding `&'static StaticStrRefKind` instead of
    /// `Box<StaticStrProxyKind>`.
    pub(super) fn static_str_ref_fallible(value: u32) -> Result<u8, StaticStrRefError> {
        u8::try_from(value).map_err(|_err| StaticStrRefError::new(&STATIC_STR_REF_PLACEHOLDER))
    }

    /// The same, calling `static_str_ref_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_static_str_ref_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = static_str_ref_fallible(u32::from(v1));
        let _ = static_str_ref_fallible(u32::from(v2));
        let _ = static_str_ref_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 14 (out of numeric order -- added
    /// after the `String`<->`KaniUtf8Buffer` correspondence was
    /// demonstrated, `gallery::string_buffer_correspondence`):
    /// `Box<KaniUtf8Buffer<8>>`. Does boxing the trenchcoat stay
    /// cheap the way boxing `u32`/`&'static str` (rungs 1 and 8) did,
    /// or does it secretly reproduce the real `Box<String>` OOM?
    /// `KaniUtf8Buffer` owns no heap allocation internally (`Copy`,
    /// backed by a fixed `[u8; N]` + `usize`), so this should be
    /// structurally the former -- not a retest of the real wall,
    /// which was specifically about boxing a type that OWNS its own
    /// heap allocation.
    pub(super) struct BoxedKaniUtf8BufferError {
        value: Box<crate::KaniUtf8Buffer<8>>,
    }

    impl BoxedKaniUtf8BufferError {
        fn new(value: crate::KaniUtf8Buffer<8>) -> Self {
            Self {
                value: Box::new(value),
            }
        }
    }

    pub(super) fn boxed_kani_utf8_buffer_fallible(
        value: u32,
    ) -> Result<u8, BoxedKaniUtf8BufferError> {
        u8::try_from(value).map_err(|_err| {
            let buffer: crate::KaniUtf8Buffer<8> = kani::any();
            BoxedKaniUtf8BufferError::new(buffer)
        })
    }

    /// The same, calling `boxed_kani_utf8_buffer_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_kani_utf8_buffer_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_kani_utf8_buffer_fallible(u32::from(v1));
        let _ = boxed_kani_utf8_buffer_fallible(u32::from(v2));
        let _ = boxed_kani_utf8_buffer_fallible(u32::from(v3));
    }

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let empty_buffer = crate::KaniUtf8Buffer::<8>::new([0u8; 8], 0)
            .expect("zero length is always valid for any byte content");
        let proxy_payload = ProxyPayload {
            detail: empty_buffer,
            line: 0,
        };
        assert_eq!(proxy_payload.line, 0);
        assert_eq!(proxy_payload.detail.len(), 0);
        let ProxyKind::V1(ref p) = ProxyKind::V1(proxy_payload);
        assert_eq!(p.line, 0);
        assert_eq!(p.detail.len(), 0);
        let proxy = ProxyError::new(ProxyKind::V1(ProxyPayload {
            detail: empty_buffer,
            line: 0,
        }));
        let ProxyKind::V1(ref p) = *proxy.kind;
        assert_eq!(p.line, 0);
        assert_eq!(p.detail.len(), 0);

        let static_str_payload = StaticStrProxyPayload {
            detail: "placeholder",
            line: 0,
        };
        assert_eq!(static_str_payload.detail, "placeholder");
        assert_eq!(static_str_payload.line, 0);
        let StaticStrProxyKind::V1(ref p) = StaticStrProxyKind::V1(StaticStrProxyPayload {
            detail: "placeholder",
            line: 0,
        });
        assert_eq!(p.detail, "placeholder");
        let static_str_proxy =
            StaticStrProxyError::new(StaticStrProxyKind::V1(StaticStrProxyPayload {
                detail: "placeholder",
                line: 0,
            }));
        let StaticStrProxyKind::V1(ref p) = *static_str_proxy.kind;
        assert_eq!(p.detail, "placeholder");

        let static_str_ref_payload = StaticStrRefPayload {
            detail: "placeholder",
            line: 0,
        };
        assert_eq!(static_str_ref_payload.detail, "placeholder");
        assert_eq!(static_str_ref_payload.line, 0);
        let StaticStrRefKind::V1(ref p) = StaticStrRefKind::V1(StaticStrRefPayload {
            detail: "placeholder",
            line: 0,
        });
        assert_eq!(p.detail, "placeholder");
        let static_str_ref = StaticStrRefError::new(&STATIC_STR_REF_PLACEHOLDER);
        let StaticStrRefKind::V1(ref p) = *static_str_ref.kind;
        assert_eq!(p.detail, "placeholder");

        let boxed_buffer_value: crate::KaniUtf8Buffer<8> = crate::KaniUtf8Buffer::new([0u8; 8], 0)
            .expect("zero length is always valid for any byte content");
        let boxed_buffer = BoxedKaniUtf8BufferError::new(boxed_buffer_value);
        assert_eq!(boxed_buffer.value.len(), 0);
    }
}

#[cfg(kani)]
use kani_only::{
    call_boxed_kani_utf8_buffer_thrice_contracted, call_proxy_thrice_contracted,
    call_static_str_proxy_thrice_contracted, call_static_str_ref_thrice_contracted,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_kani_utf8_buffer_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_kani_utf8_buffer_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 14, added after gallery::string_buffer_correspondence demonstrated the String<->KaniUtf8Buffer refinement: Box<KaniUtf8Buffer<8>> -- confirmed PASSES, 27.3s, 95/95 checks, matching Box<u32>/Box<&'static str>'s already-confirmed cheapness, not Box<String>'s OOM. Confirms boxing the trenchcoat stays cheap: swapping a production field's cfg(kani) representation from String to KaniUtf8Buffer, justified by the now-demonstrated correspondence, is a viable fix for the Box<TemporalErrorKind> wall that doesn't carry String's own vacuity risk (an empty-placeholder swap would have).".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_KANI_UTF8_BUFFER_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_kani_utf8_buffer_thrice_contracted)]
        fn check_call_boxed_kani_utf8_buffer_thrice_contracted() {
            call_boxed_kani_utf8_buffer_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::proxy_tactic_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_proxy_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the general proxy tactic, demonstrated on the most faithful possible recreation of the real blocker: ProxyError { kind: Box<ProxyKind> }, ProxyKind::V1(ProxyPayload { detail: KaniUtf8Buffer<8>, line: u32 }) -- OneVariantError's exact shape (confirmed OOM) with the one structural change the tactic makes, detail swapped from String to KaniUtf8Buffer, with a genuinely symbolic (not fixed-literal) detail -- confirmed PASSES, 26.0s, 95/95 checks. The exact shape that OOM'd with String now passes cleanly with the proxy swap, same enum/struct nesting, same Box wrapping, nothing simplified away. The general tactic, end to end: wherever Box<Kind>-wrapping-a-String-bearing-Source is the blocker, swap the field's cfg(kani) representation to KaniUtf8Buffer, and lean on the demonstrated (gallery::string_buffer_correspondence) refinement to carry any property proven on the proxy back to the real String.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_PROXY_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_proxy_thrice_contracted)]
        fn check_call_proxy_thrice_contracted() {
            call_proxy_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::static_str_proxy_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_static_str_proxy_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "a third option alongside String (OOM) and the KaniUtf8Buffer proxy (26.0s): StaticStrProxyError { kind: Box<StaticStrProxyKind> }, StaticStrProxyKind::V1(StaticStrProxyPayload { detail: &'static str, line: u32 }) -- OneVariantError's exact shape with detail as &'static str instead of String -- confirmed PASSES, 25.2s, 95/95 checks, matching the proxy's cost almost exactly. Unlike the KaniUtf8Buffer proxy, this isn't a cfg(kani)-only swap -- if every real call site already passes a static literal (confirmed for amenable_ext::chrono's narrow_to_u8/fixed_offset_to_utc_offset_descriptor), &'static str would remove the heap allocation unconditionally, in real builds too, not just under Kani. Whether every real call site across the whole error surface is actually always a static literal is the separate question that decides whether this is viable in production, not yet checked here.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_STATIC_STR_PROXY_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_static_str_proxy_thrice_contracted)]
        fn check_call_static_str_proxy_thrice_contracted() {
            call_static_str_proxy_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::static_str_ref_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_static_str_ref_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "a fourth option: StaticStrRefError { kind: &'static StaticStrRefKind } instead of Box<StaticStrProxyKind> -- the parent holds a plain reference to a static value rather than owning a heap allocation at all. Confirmed PASSES, 0.13s, 26/26 checks -- instant, no Box::new/Drop in the whole path, cheaper than every other rung on this ladder including the &'static-str-boxed variant (25.2s). But this only works because every field of the Kind is const-constructible, so the whole value lives in one static with no dynamically-computed content -- not yet a realistic stand-in for a case with genuinely varying detail, unlike the other three rungs above, which all construct their value fresh from arbitrary runtime input.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_STATIC_STR_REF_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_static_str_ref_thrice_contracted)]
        fn check_call_static_str_ref_thrice_contracted() {
            call_static_str_ref_thrice_contracted();
        }
    }
}
