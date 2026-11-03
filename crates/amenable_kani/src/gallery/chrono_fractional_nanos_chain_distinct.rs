//! Confirms the peeling methodology (stub the previous stage, never
//! inline it) keeps verification cost flat as a chain of *distinct*
//! functions grows, where plain inlined chaining goes exponential --
//! both the unstubbed-chain and stubbed-at-each-stage (`_contract`)
//! variants, through depth 9. Split out of
//! `chrono_fractional_nanos_contract_cost` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::gallery::chrono_fractional_nanos_growing_ensures::{
        growing_ensures_1, growing_ensures_2, growing_ensures_3, growing_ensures_4,
        growing_ensures_5,
    };



    /// Hypothesis 16: chain DISTINCT functions, each with its own
    /// growing-conjunct ensures, matching full_encode_param_passing's
    /// real structure (combine_digit_2..9 called in sequence) more
    /// closely than hypothesis 13's chains, which reused a single
    /// function. Built up one step at a time.
    #[kani::ensures(|result: &[u8; 9]| result[0] == arr[0] && result[1] == arr[1])]
    pub(super) fn chain_distinct_through_2(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_2(growing_ensures_1(arr))
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0] && result[1] == arr[1] && result[2] == arr[2]
    })]
    pub(super) fn chain_distinct_through_3(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_3(chain_distinct_through_2(arr))
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0] && result[1] == arr[1] && result[2] == arr[2] && result[3] == arr[3]
    })]
    pub(super) fn chain_distinct_through_4(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_4(chain_distinct_through_3(arr))
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
    })]
    pub(crate) fn chain_distinct_through_5(arr: [u8; 9]) -> [u8; 9] {
        growing_ensures_5(chain_distinct_through_4(arr))
    }}

#[cfg(kani)]
pub(crate) use kani_only::chain_distinct_through_5;
#[cfg(kani)]
use kani_only::{chain_distinct_through_2, chain_distinct_through_3, chain_distinct_through_4};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_2".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 2 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_2_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_2() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_2(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_3".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 3 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_3_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_3() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_3(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_4".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 4 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_4_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_4() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_4(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_5".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 16: chain of 5 distinct functions, each with its own growing-conjunct ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_5_SRC, {
        #[kani::proof]
        fn check_chain_distinct_through_5() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_5(arr);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_2_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_2_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_2's own contract directly, no stubbing needed (both callees are trivial leaves).".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_2_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_2)]
        fn check_chain_distinct_through_2_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_2(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_3_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_3_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_3's own contract, stubbing chain_distinct_through_2's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_3_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_3)]
        #[kani::stub_verified(chain_distinct_through_2)]
        fn check_chain_distinct_through_3_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_3(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_4_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_4_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_4's own contract, stubbing chain_distinct_through_3's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_4_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_4)]
        #[kani::stub_verified(chain_distinct_through_3)]
        fn check_chain_distinct_through_4_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_4(arr);
        }
    }
}






::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_chain_distinct::chain_distinct_through_5_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_chain_distinct::check_chain_distinct_through_5_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 17: verify chain_distinct_through_5's own contract, stubbing chain_distinct_through_4's already-proven contract instead of inlining it. Tests whether this keeps cost flat vs. hypothesis 16's exponential plain-call chain.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN_DISTINCT_THROUGH_5_CONTRACT_SRC, {
        #[kani::proof_for_contract(chain_distinct_through_5)]
        #[kani::stub_verified(chain_distinct_through_4)]
        fn check_chain_distinct_through_5_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain_distinct_through_5(arr);
        }
    }
}

