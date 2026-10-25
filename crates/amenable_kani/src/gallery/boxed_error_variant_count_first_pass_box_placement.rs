//! First-pass hypothesis test, box-placement inversion, for the CBMC
//! out-of-memory investigation -- see `boxed_error_variant_count.rs`'s
//! own module doc comment for the full narrative this file is one
//! chapter of, and the sibling
//! `boxed_error_variant_count_first_pass_variant_count.rs` for the
//! other four first-pass rungs. A reviewer-proposed inversion: instead
//! of boxing the whole `*Kind` enum, keep the enum's own discriminant
//! tiny and unboxed, and box only the one heavy variant's own payload.
//! Confirmed still out-of-memory, ruling out "the whole enum is
//! boxed" as the driver too -- moving the `Box` from around the enum
//! to around just the heavy field, while keeping the exact same
//! reachable heap allocation, changes nothing.

mod kani_only {
    #![cfg(kani)]

    /// A detail string, standing in for a real error source's own
    /// `detail` field -- the one piece this file's own box-placement
    /// question ever touches (unlike the sibling
    /// `boxed_error_variant_count_first_pass_variant_count.rs`'s own,
    /// separately-defined `Payload`, this trimmed copy carries no
    /// `line` field, since nothing here ever reads one). Duplicated
    /// rather than shared across files -- `pub(super)` visibility on
    /// that sibling's own private `kani_only` module doesn't reach
    /// this file, and this shape is small enough that a one-off local
    /// copy is cheaper than widening visibility crate-wide for a
    /// single use site.
    #[derive(Debug, derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct Payload {
        #[new(into)]
        detail: String,
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

    #[derive(derive_new::new)]
    #[new(visibility = "")]
    pub(super) struct SmallDiscriminantError {
        kind: SmallDiscriminantKind,
    }

    /// Identical shape to `nine_variant_fallible`'s error path -- always
    /// constructs the one variant that actually carries a payload --
    /// but that variant boxes only its own `Payload`, not the enclosing
    /// enum.
    pub(super) fn small_discriminant_fallible(value: u32) -> Result<u8, SmallDiscriminantError> {
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

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let small_tiny = SmallDiscriminantKind::Tiny(5);
        assert!(matches!(small_tiny, SmallDiscriminantKind::Tiny(5)));
        let SmallDiscriminantKind::Heavy(ref boxed) =
            SmallDiscriminantKind::Heavy(Box::new(Payload::new("heavy")))
        else {
            unreachable!()
        };
        assert!(!boxed.detail.is_empty());
        let small = SmallDiscriminantError::new(SmallDiscriminantKind::Heavy(Box::new(
            Payload::new("heavy"),
        )));
        let SmallDiscriminantKind::Heavy(ref boxed) = small.kind else {
            unreachable!()
        };
        assert!(!boxed.detail.is_empty());
    }
}

#[cfg(kani)]
use kani_only::call_small_discriminant_thrice_contracted;

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
