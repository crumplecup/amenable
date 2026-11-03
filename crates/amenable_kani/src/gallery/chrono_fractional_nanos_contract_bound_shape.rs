//! Isolates whether the *shape* of a contract bound
//! (no contract at all, raw inline bounds, or a named trait-object
//! bound like `ValueIsWithinInclusiveRange`) adds verification cost
//! on its own, independent of real production arithmetic, chained
//! through depth 8. Split out of `chrono_fractional_nanos_contract_cost`
//! once that file grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use amenable_core::{Ensures, Requires};
    use crate::{IndexRecoversTheStoredElement, ValueIsWithinInclusiveRange};

    // ── Hypothesis 13: a controlled, single-variable experiment.
    // Same trivial body in all three cases (identity on an i64) --
    // only the contract mechanism varies. Each gets its own isolated
    // test, one fact at a time, before any chaining.

    /// Baseline: no contract at all.
    pub(crate) fn trivial_no_contract(x: i64) -> i64 {
        x
    }



    /// Raw inline requires/ensures.
    #[kani::requires(x >= 0 && x <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == x)]
    pub(crate) fn trivial_raw_bounds(x: i64) -> i64 {
        x
    }



    /// Named-contract-trait requires/ensures -- same body, same
    /// bound, expressed via ValueIsWithinInclusiveRange::requires and
    /// IndexRecoversTheStoredElement::ensures instead of raw
    /// expressions.
    #[kani::requires(ValueIsWithinInclusiveRange::requires((x, 0, 999_999_999)))]
    #[kani::ensures(|result: &i64| IndexRecoversTheStoredElement::ensures((*result, x)))]
    pub(crate) fn trivial_trait_bounds(x: i64) -> i64 {
        x
    }



    /// Step 4: chain of 2 plain calls, no contract.
    pub(super) fn chain2_no_contract(x: i64) -> i64 {
        trivial_no_contract(trivial_no_contract(x))
    }



    /// Step 5: chain of 2 plain calls, raw bounds on each.
    pub(super) fn chain2_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(trivial_raw_bounds(x))
    }



    /// Step 6: chain of 2 plain calls, trait bounds on each.
    pub(super) fn chain2_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(trivial_trait_bounds(x))
    }



    pub(super) fn chain3_no_contract(x: i64) -> i64 {
        trivial_no_contract(trivial_no_contract(trivial_no_contract(x)))
    }



    pub(super) fn chain3_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(trivial_raw_bounds(trivial_raw_bounds(x)))
    }



    pub(super) fn chain3_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(trivial_trait_bounds(trivial_trait_bounds(x)))
    }



    pub(crate) fn chain4_no_contract(x: i64) -> i64 {
        trivial_no_contract(chain3_no_contract(x))
    }



    pub(crate) fn chain4_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(chain3_raw_bounds(x))
    }



    pub(crate) fn chain4_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(chain3_trait_bounds(x))
    }}

#[cfg(kani)]
pub(crate) use kani_only::{
    chain4_no_contract, chain4_raw_bounds, chain4_trait_bounds, trivial_no_contract,
    trivial_raw_bounds, trivial_trait_bounds,
};
#[cfg(kani)]
use kani_only::{
    chain2_no_contract, chain2_raw_bounds, chain2_trait_bounds, chain3_no_contract,
    chain3_raw_bounds, chain3_trait_bounds,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::trivial_no_contract_baseline_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_trivial_no_contract_baseline".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 1 of a controlled experiment: the trivial identity function, no contract at all, called once, plainly. Baseline for comparison against the same body with contract attributes added.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_NO_CONTRACT_BASELINE_SRC, {
        /// Step 1: baseline, no contract.
        #[kani::proof]
        fn check_trivial_no_contract_baseline() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = trivial_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::trivial_raw_bounds_alone_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_trivial_raw_bounds_alone".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 2: the same trivial identity body, now with raw #[kani::requires]/#[kani::ensures], called once, plainly (not stubbed, not via proof_for_contract). Isolates whether the contract attributes alone add cost to a single plain call.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_RAW_BOUNDS_ALONE_SRC, {
        /// Step 2: raw bounds, single plain call.
        #[kani::proof]
        fn check_trivial_raw_bounds_alone() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = trivial_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::trivial_trait_bounds_alone_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_trivial_trait_bounds_alone".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 3: the same trivial identity body, same bound, expressed via the named Requires/Ensures contract trait instead of raw expressions. Single plain call. Compares directly against step 2 (raw bounds) with every other variable held fixed.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_TRAIT_BOUNDS_ALONE_SRC, {
        /// Step 3: trait-based bounds, single plain call.
        #[kani::proof]
        fn check_trivial_trait_bounds_alone() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = trivial_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain2_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain2_no_contract".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 4: chain of 2 plain calls to the no-contract baseline. Baseline for the chaining comparison.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN2_NO_CONTRACT_SRC, {
        /// Step 4: chain of 2, no contract.
        #[kani::proof]
        fn check_chain2_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain2_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain2_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain2_raw_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 5: chain of 2 plain calls, raw #[kani::requires]/#[kani::ensures] on each. Compares directly against step 4 (no contract) with only the contract mechanism varying.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN2_RAW_BOUNDS_SRC, {
        /// Step 5: chain of 2, raw bounds.
        #[kani::proof]
        fn check_chain2_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain2_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain2_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain2_trait_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 6: chain of 2 plain calls, trait-based bounds on each. Compares directly against step 5 (raw bounds) with only the bound expression form varying.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN2_TRAIT_BOUNDS_SRC, {
        /// Step 6: chain of 2, trait bounds.
        #[kani::proof]
        fn check_chain2_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain2_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain3_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain3_no_contract".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 7: chain of 3, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN3_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain3_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain3_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain3_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain3_raw_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 8: chain of 3, raw bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN3_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain3_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain3_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain3_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain3_trait_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 13, step 9: chain of 3, trait bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN3_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain3_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain3_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain4_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain4_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 4, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN4_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain4_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain4_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain4_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain4_raw_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 4, raw bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN4_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain4_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain4_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape::chain4_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape::check_chain4_trait_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 4, trait bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN4_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain4_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain4_trait_bounds(x);
        }
    }
}

