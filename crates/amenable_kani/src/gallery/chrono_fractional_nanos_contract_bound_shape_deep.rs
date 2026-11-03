//! The deep-chain half (depth 5 through 8) of
//! `chrono_fractional_nanos_contract_bound_shape`'s no-contract/raw-
//! bounds/trait-bounds comparison, split out once that file grew
//! large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::gallery::chrono_fractional_nanos_contract_bound_shape::{
        chain4_no_contract, chain4_raw_bounds, chain4_trait_bounds, trivial_no_contract,
        trivial_raw_bounds, trivial_trait_bounds,
    };



    pub(super) fn chain5_no_contract(x: i64) -> i64 {
        trivial_no_contract(chain4_no_contract(x))
    }




    pub(super) fn chain5_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(chain4_raw_bounds(x))
    }




    pub(super) fn chain5_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(chain4_trait_bounds(x))
    }




    pub(super) fn chain6_no_contract(x: i64) -> i64 {
        trivial_no_contract(chain5_no_contract(x))
    }




    pub(super) fn chain6_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(chain5_raw_bounds(x))
    }




    pub(super) fn chain6_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(chain5_trait_bounds(x))
    }




    pub(super) fn chain7_no_contract(x: i64) -> i64 {
        trivial_no_contract(chain6_no_contract(x))
    }




    pub(super) fn chain7_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(chain6_raw_bounds(x))
    }




    pub(super) fn chain7_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(chain6_trait_bounds(x))
    }




    pub(super) fn chain8_no_contract(x: i64) -> i64 {
        trivial_no_contract(chain7_no_contract(x))
    }




    pub(super) fn chain8_raw_bounds(x: i64) -> i64 {
        trivial_raw_bounds(chain7_raw_bounds(x))
    }




    pub(super) fn chain8_trait_bounds(x: i64) -> i64 {
        trivial_trait_bounds(chain7_trait_bounds(x))
    }
}

#[cfg(kani)]
use kani_only::{
    chain5_no_contract, chain5_raw_bounds, chain5_trait_bounds, chain6_no_contract,
    chain6_raw_bounds, chain6_trait_bounds, chain7_no_contract, chain7_raw_bounds,
    chain7_trait_bounds, chain8_no_contract, chain8_raw_bounds, chain8_trait_bounds,
};







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain5_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain5_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 5, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN5_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain5_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain5_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain5_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain5_raw_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 5, raw bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN5_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain5_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain5_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain5_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain5_trait_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 5, trait bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN5_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain5_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain5_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain6_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain6_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 6, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN6_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain6_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain6_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain6_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain6_raw_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 6, raw bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN6_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain6_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain6_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain6_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain6_trait_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 6, trait bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN6_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain6_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain6_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain7_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain7_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 7, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN7_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain7_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain7_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain7_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain7_raw_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 7, raw bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN7_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain7_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain7_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain7_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain7_trait_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 7, trait bounds.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN7_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain7_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain7_trait_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain8_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain8_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 8, no contract -- matches the chain length of the original full_encode_param_passing comparison.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN8_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain8_no_contract() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain8_no_contract(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain8_raw_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain8_raw_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 8, raw bounds -- matches the chain length of the original full_encode_param_passing comparison.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN8_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain8_raw_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain8_raw_bounds(x);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_bound_shape_deep::chain8_trait_bounds_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_bound_shape_deep::check_chain8_trait_bounds".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 13: chain of 8, trait bounds -- matches the chain length of the original full_encode_param_passing comparison.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN8_TRAIT_BOUNDS_SRC, {
        #[kani::proof]
        fn check_chain8_trait_bounds() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x <= 999_999_999);
            let _ = chain8_trait_bounds(x);
        }
    }
}
