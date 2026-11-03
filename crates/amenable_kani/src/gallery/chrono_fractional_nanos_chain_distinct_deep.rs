//! The deep half (depth 6 through 9, both the plain and
//! peeled-and-stubbed `_contract` variants) of
//! `chrono_fractional_nanos_chain_distinct`, split out once that file
//! grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::gallery::chrono_fractional_nanos_growing_ensures::{growing_ensures_6, growing_ensures_7, growing_ensures_8, growing_ensures_9};
    use crate::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_5;





    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
    })]
    pub(super) fn chain_distinct_through_6(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_6(chain_distinct_through_5(arr))
    }




    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
    })]
    pub(super) fn chain_distinct_through_7(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_7(chain_distinct_through_6(arr))
    }




    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
            && result[7] == arr[7]
    })]
    pub(super) fn chain_distinct_through_8(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_8(chain_distinct_through_7(arr))
    }




    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
            && result[7] == arr[7]
            && result[8] == arr[8]
    })]
    pub(super) fn chain_distinct_through_9(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_9(chain_distinct_through_8(arr))
    }
}

#[cfg(kani)]
use kani_only::{chain_distinct_through_6, chain_distinct_through_7, chain_distinct_through_8, chain_distinct_through_9};






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_6".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 6 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_6_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_6() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_6(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_7".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 7 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_7_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_7() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_7(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_8".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 8 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_8_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_8() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_8(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_9".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 9 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_9_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_9() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_9(arr);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_6_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_6_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_6's own contract, stubbing chain_distinct_through_5's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_6_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_6)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_5)]
        fn check_chain_distinct_through_6_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_6(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_7_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_7_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_7's own contract, stubbing chain_distinct_through_6's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_7_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_7)]
        #[kani::stub_verified(chain_distinct_through_6)]
        fn check_chain_distinct_through_7_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_7(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_8_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_8_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_8's own contract, stubbing chain_distinct_through_7's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_8_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_8)]
        #[kani::stub_verified(chain_distinct_through_7)]
        fn check_chain_distinct_through_8_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_8(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct_deep::chain_distinct_through_9_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct_deep::check_chain_distinct_through_9_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_9's own contract, stubbing chain_distinct_through_8's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_9_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_9)]
        #[kani::stub_verified(chain_distinct_through_8)]
        fn check_chain_distinct_through_9_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_9(arr);
        }
    }
}
