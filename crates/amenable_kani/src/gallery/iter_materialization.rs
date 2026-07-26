//! Gallery cases for iterator observation strategy under Kani.
//!
//! The production `rust_std::iter::verify_flatten_concatenates_the_inner_iterators`
//! review exposed a process problem: it was too easy to swap observation styles
//! inside the proof queue without preserving why. These cases isolate the
//! question directly.
//!
//! Both cases use the same bounded semantic claim:
//!
//! - build `Vec<Range<i32>>` from two small symbolic lengths
//! - flatten it
//! - compare the result with `(0..a).chain(0..b)`
//!
//! The only difference is observation strategy:
//!
//! - eager `Vec<i32>` materialization
//! - incremental `next()` comparison
//!
//! The first result was weaker than hoped: eager collection timed out, and
//! incremental observation over the same symbolic state space also timed out.
//! The follow-up control keeps the incremental observation but removes the
//! symbolic lengths so we can isolate whether the blow-up is in the symbolic
//! iterator shape rather than the observation style alone.

#[cfg(kani)]
use std::{ops::Range, vec::IntoIter};

#[cfg(kani)]
use core::iter::Flatten;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::iter_materialization::flatten_collect_times_out".to_owned(),
            harness: "gallery::iter_materialization::flatten_collect_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "eagerly collecting a flattened iterator can time out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, FLATTEN_COLLECT_TIMES_OUT_SRC, {
        /// This mirrors the original production harness shape closely: both
        /// the flattened iterator and the direct concatenation are eagerly
        /// collected into `Vec<i32>` before comparison.
        #[kani::proof]
        fn flatten_collect_times_out() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume((0..=4).contains(&a));
            kani::assume((0..=4).contains(&b));

            let nested: Vec<Range<i32>> = vec![0..a, 0..b];
            let flattened: Vec<i32> = nested.into_iter().flatten().collect();
            let expected: Vec<i32> = (0..a).chain(0..b).collect();

            assert_eq!(
                flattened,
                expected,
                "eager materialization should preserve flatten's concatenation semantics"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::iter_materialization::flatten_incremental_next_passes".to_owned(),
            harness: "gallery::iter_materialization::flatten_incremental_next_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "incremental observation alone still times out for symbolic flatten".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, FLATTEN_INCREMENTAL_NEXT_PASSES_SRC, {
        /// This keeps the exact same semantic claim while avoiding eager
        /// materialization: compare one observed item at a time and then
        /// confirm joint exhaustion. This is the same proof shape the
        /// production flatten witness now uses.
        #[kani::proof]
        fn flatten_incremental_next_passes() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume((0..=4).contains(&a));
            kani::assume((0..=4).contains(&b));

            let nested: Vec<Range<i32>> = vec![0..a, 0..b];
            let mut flattened: Flatten<IntoIter<Range<i32>>> = nested.into_iter().flatten();
            let mut expected = (0..a).chain(0..b);

            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's first item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's second item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's third item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's fourth item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's fifth item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's sixth item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's seventh item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's eighth item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten and the direct concatenation exhaust together"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten remains exhausted once the concatenation is done"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::iter_materialization::flatten_incremental_fixed_lengths_passes".to_owned(),
            harness: "gallery::iter_materialization::flatten_incremental_fixed_lengths_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "incremental flatten passes once the lengths are concrete".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, FLATTEN_INCREMENTAL_FIXED_LENGTHS_PASSES_SRC, {
        /// This control keeps the same incremental observation style but fixes
        /// the iterator lengths to separate symbolic blow-up from the
        /// semantics of `flatten` itself.
        #[kani::proof]
        fn flatten_incremental_fixed_lengths_passes() {
            let nested: Vec<Range<i32>> = vec![0..1, 0..2];
            let mut flattened: Flatten<IntoIter<Range<i32>>> = nested.into_iter().flatten();
            let mut expected = (0..1).chain(0..2);

            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's first concrete item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's second concrete item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten's third concrete item matches the direct concatenation"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten and the concrete concatenation exhaust together"
            );
            assert_eq!(
                flattened.next(),
                expected.next(),
                "flatten remains exhausted after the concrete concatenation ends"
            );
        }
    }
}
