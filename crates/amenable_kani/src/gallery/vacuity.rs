//! Gallery cases that calibrate vacuity and basic satisfiability.
//!
//! These are the first proof-gallery cases because they establish a sharp
//! boundary for later assessments:
//!
//! - `assume(false)` can make a harness "pass" without checking anything
//! - a genuine contradiction should fail under Kani
//! - a satisfiable bounded assumption with a matching assertion is the minimal
//!   shape of a real, non-vacuous pass

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::vacuity::assume_false_is_vacuous_pass".to_owned(),
            harness: "gallery::vacuity::assume_false_is_vacuous_pass".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "an unsatisfiable assumption can produce a vacuous pass".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, ASSUME_FALSE_IS_VACUOUS_PASS_SRC, {
        /// Demonstrates the core vacuity risk: once `assume(false)` removes all
        /// paths, the later assertion is unreachable and Kani reports success.
        #[kani::proof]
        fn assume_false_is_vacuous_pass() {
            let value: u8 = kani::any();
            kani::assume(value == 0 && value == 1);
            assert_eq!(
                value,
                0,
                "this assertion is unreachable because the assumption kills all paths"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::vacuity::explicit_contradiction_fails".to_owned(),
            harness: "gallery::vacuity::explicit_contradiction_fails".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a reachable contradiction produces a genuine failure".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, EXPLICIT_CONTRADICTION_FAILS_SRC, {
        /// Provides the non-vacuous control case: the contradiction is
        /// reachable, so Kani should report a real failing assertion.
        #[kani::proof]
        fn explicit_contradiction_fails() {
            let value: u8 = kani::any();
            assert!(
                value == 0 && value == 1,
                "a reachable contradiction must fail rather than passing vacuously"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::vacuity::bounded_assumption_passes_nonvacuously".to_owned(),
            harness: "gallery::vacuity::bounded_assumption_passes_nonvacuously".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a satisfiable bounded assumption yields a genuine pass".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, BOUNDED_ASSUMPTION_PASSES_NONVACUOUSLY_SRC, {
        /// This is the minimal "real pass" shape for later experiments: the
        /// assumption is satisfiable and the assertion restates the bounded
        /// property the solver must actually preserve.
        #[kani::proof]
        fn bounded_assumption_passes_nonvacuously() {
            let value: u8 = kani::any();
            kani::assume(value < 4);
            assert!(
                value < 4,
                "the pass is genuine because the bounded assumption is satisfiable"
            );
        }
    }
}
