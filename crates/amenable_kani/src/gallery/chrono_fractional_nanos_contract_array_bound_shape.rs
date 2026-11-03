//! The array-typed twin of `chrono_fractional_nanos_contract_bound_
//! shape`: the same no-contract/raw-bounds/trait-bounds comparison,
//! but over `[i32; 9]` chains rather than scalars. Split out of
//! `chrono_fractional_nanos_contract_cost` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]




    /// Hypothesis 14, step 1: isolate the array-type variable alone.
    /// Same shape as `trivial_no_contract` (identity, no contract),
    /// but the parameter/return type is a fixed-size `[u8; 9]` array
    /// instead of a scalar `i64` -- the one variable this step
    /// changes relative to the hypothesis 13 baseline.
    pub(super) fn trivial_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    pub(super) fn chain2_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(trivial_array_no_contract(arr))
    }



    pub(super) fn chain3_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain2_array_no_contract(arr))
    }



    pub(super) fn chain4_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain3_array_no_contract(arr))
    }



    pub(super) fn chain5_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain4_array_no_contract(arr))
    }



    pub(super) fn chain6_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain5_array_no_contract(arr))
    }



    pub(super) fn chain7_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain6_array_no_contract(arr))
    }



    pub(super) fn chain8_array_no_contract(arr: [u8; 9]) -> [u8; 9] {
        trivial_array_no_contract(chain7_array_no_contract(arr))
    }



    /// Hypothesis 14, step 2: add a trivial requires/ensures bound on
    /// the array, matching the byte-range + identity shape that
    /// combine_digit_K's own contracts use. Still an identity body;
    /// the only new variable vs. trivial_array_no_contract is the
    /// contract.
    #[kani::requires(arr.iter().all(|&b| b <= 9))]
    #[kani::ensures(|result: &[u8; 9]| *result == arr)]
    pub(super) fn trivial_array_raw_bounds(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    /// Hypothesis 14, step 3: same array type, same identity body,
    /// but the requires/ensures touch only a single fixed index
    /// instead of an `.iter().all()` quantifier over all 9 elements.
    /// Isolates whether the quantifier form itself is the cost, vs.
    /// "any contract on an array type at all".
    #[kani::requires(arr[0] <= 9)]
    #[kani::ensures(|result: &[u8; 9]| result[0] == arr[0])]
    pub(super) fn trivial_array_single_index_bounds(arr: [u8; 9]) -> [u8; 9] {
        arr
    }
}

#[cfg(kani)]
use kani_only::{
    chain2_array_no_contract, chain3_array_no_contract, chain4_array_no_contract,
    chain5_array_no_contract, chain6_array_no_contract, chain7_array_no_contract,
    chain8_array_no_contract, trivial_array_no_contract, trivial_array_raw_bounds,
    trivial_array_single_index_bounds,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::trivial_array_no_contract_alone".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_trivial_array_no_contract".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 14, step 1: a single call to an identity function over [u8; 9] instead of i64, no contract. Isolates the array-type variable alone.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_trivial_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = trivial_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain2_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain2_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 2, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN2_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain2_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain2_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain3_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain3_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 3, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN3_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain3_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain3_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain4_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain4_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 4, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN4_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain4_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain4_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain5_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain5_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 5, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN5_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain5_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain5_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain6_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain6_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 6, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN6_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain6_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain6_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain7_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain7_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 7, array type, no contract.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN7_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain7_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain7_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::chain8_array_no_contract_passes".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_chain8_array_no_contract".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 14: chain of 8, array type, no contract -- matches the chain length of the original full_encode_param_passing comparison.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_CHAIN8_ARRAY_NO_CONTRACT_SRC, {
        #[kani::proof]
        fn check_chain8_array_no_contract() {
            let arr: [u8; 9] = kani::any();
            let _ = chain8_array_no_contract(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::trivial_array_raw_bounds_alone".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_trivial_array_raw_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 14, step 2: a single call with a trivial byte-range requires/ensures on [u8; 9]. Isolates the array-type + contract interaction alone.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_ARRAY_RAW_BOUNDS_SRC, {
        #[kani::proof]
        fn check_trivial_array_raw_bounds() {
            let arr: [u8; 9] = kani::any();
            kani::assume(arr.iter().all(|&b| b <= 9));
            let _ = trivial_array_raw_bounds(arr);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_contract_array_bound_shape::trivial_array_single_index_bounds_alone".to_owned(),
            "gallery::chrono_fractional_nanos_contract_array_bound_shape::check_trivial_array_single_index_bounds".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 14, step 3: a single call with a requires/ensures touching only arr[0], no .iter().all() quantifier. Isolates the quantifier form from array-with-contract in general.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_TRIVIAL_ARRAY_SINGLE_INDEX_BOUNDS_SRC, {
        #[kani::proof]
        fn check_trivial_array_single_index_bounds() {
            let arr: [u8; 9] = kani::any();
            kani::assume(arr[0] <= 9);
            let _ = trivial_array_single_index_bounds(arr);
        }
    }
}
