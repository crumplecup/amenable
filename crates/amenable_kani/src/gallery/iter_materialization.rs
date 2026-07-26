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
//!
//! A second, distinct failure mode lives below: `Filter` (and `FilterMap`,
//! `FlatMap`'s outer source) time out even for a single-item observation
//! with no collection at all, because their `next()` routes through
//! `Iterator::find`/`try_fold`, and `Range<i32>::try_fold`'s loop bound is a
//! runtime comparison Kani's unwinder cannot bound. Swapping only the
//! source's *type* -- from `Range<i32>` to `std::array::IntoIter<i32, 1>`,
//! same symbolic element, same predicate -- resolves it, since the array's
//! `try_fold` loop bound is a compile-time constant.

#[cfg(kani)]
use std::{ops::Range, vec::IntoIter};

#[cfg(kani)]
use core::iter::Flatten;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::iter_materialization::find_routed_filter_over_symbolic_range_times_out".to_owned(),
            harness: "gallery::iter_materialization::find_routed_filter_over_symbolic_range_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Filter over a single-item symbolic Range<i32> still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, FIND_ROUTED_FILTER_OVER_SYMBOLIC_RANGE_TIMES_OUT_SRC, {
        /// `Filter::next` is implemented in std via `Iterator::find`, which
        /// routes through the source's `try_fold`. `Range<i32>::try_fold`'s
        /// loop bound is a runtime comparison between two symbolic `i32`
        /// endpoints, so Kani's unwinder cannot conclude the loop is bounded
        /// even though the range is logically a single item -- confirmed
        /// this still times out past 500 unwind iterations. This is a
        /// distinct failure mode from the eager-materialization case above:
        /// this harness never collects anything, it calls `.next()` once.
        #[kani::proof]
        fn find_routed_filter_over_symbolic_range_times_out() {
            fn is_even(x: &i32) -> bool {
                *x != 0
            }
            let x: i32 = kani::any();
            let expected = if is_even(&x) { Some(x) } else { None };
            assert_eq!(
                (x..x + 1).filter(is_even).next(),
                expected,
                "filter yields the item only when the predicate holds"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::iter_materialization::find_routed_filter_over_array_source_passes".to_owned(),
            harness: "gallery::iter_materialization::find_routed_filter_over_array_source_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Filter over a one-element array source resolves the same find/try_fold timeout".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, FIND_ROUTED_FILTER_OVER_ARRAY_SOURCE_PASSES_SRC, {
        /// Same claim, same predicate, same symbolic element value as the
        /// timeout control above -- the only change is the source type.
        /// `std::array::IntoIter<i32, 1>` has the same `find`-routed
        /// `next()` as `Range<i32>`, but its `try_fold` loop bound is a
        /// compile-time array length rather than a runtime comparison, so
        /// Kani resolves it immediately. This is the fix applied to the
        /// production `Filter`/`FilterMap`/`FlatMap` proofs in
        /// `rust_std::iter`.
        #[kani::proof]
        fn find_routed_filter_over_array_source_passes() {
            fn is_even(x: &i32) -> bool {
                *x != 0
            }
            let x: i32 = kani::any();
            let expected = if is_even(&x) { Some(x) } else { None };
            assert_eq!(
                [x].into_iter().filter(is_even).next(),
                expected,
                "filter yields the item only when the predicate holds"
            );
        }
    }
}

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
