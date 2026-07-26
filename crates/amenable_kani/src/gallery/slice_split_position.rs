//! Gallery cases isolating `std::slice::Split`'s internal position search.
//!
//! The `rust_std::slice` split family (`Split`, `SplitMut`, `SplitN`,
//! `SplitNMut`, `RSplit`, `RSplitMut`, `RSplitN`, `RSplitNMut`,
//! `SplitInclusive`, `SplitInclusiveMut`, and `ChunkBy`) time out even
//! though every harness in that module already uses a small, fixed-length
//! array source (never a symbolic-length `Vec` or `Range`) -- so this is a
//! distinct failure mode from `iter_materialization`'s `Range<i32>::try_fold`
//! issue, not a repeat of it.
//!
//! Isolated here: calling `[T]::iter().position(predicate)` directly on the
//! same fixed 3-element array with the same predicate passes in well under a
//! second. But `std::slice::Split::next()` calls that exact
//! `Iter::position` internally (confirmed via the demangled CBMC unwind
//! trace: `<slice::Iter<'_, i32> as Iterator>::position::<{closure#0
//! from Split::next}>`), and *that* call times out past 200+ unwind
//! iterations on the identical array and predicate. The only difference is
//! that `Split::next()` wraps the predicate through its own generic `P`
//! field access (`(self.pred)(x)` inside a capturing closure) before handing
//! it to `position`, rather than calling a bare `fn` item directly. This
//! extra indirection is internal to `std::slice::Split`'s implementation --
//! not something a proof-side rewrite can bypass — so the split family
//! remains a documented blocker pending a dedicated accommodation model
//! (see `PLANNING_INDEX.md`).

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::slice_split_position::bare_iter_position_over_fixed_array_passes".to_owned(),
            harness: "gallery::slice_split_position::bare_iter_position_over_fixed_array_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Iter::position over a fixed-length array resolves immediately".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, BARE_ITER_POSITION_OVER_FIXED_ARRAY_PASSES_SRC, {
        /// Control: the same fixed 3-element array, same predicate, same
        /// symbolic `a`/`b`, called through `Iter::position` directly
        /// rather than through `Split::next()`.
        #[kani::proof]
        fn bare_iter_position_over_fixed_array_passes() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let data = [a, 0, b];
            assert_eq!(data.iter().position(is_zero), Some(1));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::slice_split_position::split_next_over_the_same_fixed_array_times_out".to_owned(),
            harness: "gallery::slice_split_position::split_next_over_the_same_fixed_array_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Split::next() over the identical fixed-length array still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, SPLIT_NEXT_OVER_THE_SAME_FIXED_ARRAY_TIMES_OUT_SRC, {
        /// Same array, same predicate, same symbolic `a`/`b` as the control
        /// above -- only the call path changes, from a bare `Iter::position`
        /// call to `[T]::split(predicate)`, which calls `Iter::position`
        /// internally through its own generic predicate field. That
        /// indirection is enough to make CBMC's unwinder lose the fixed
        /// length bound the control resolves trivially.
        #[kani::proof]
        fn split_next_over_the_same_fixed_array_times_out() {
            fn is_zero(x: &i32) -> bool {
                *x == 0
            }
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a != 0 && b != 0);
            let data = [a, 0, b];
            let mut it = data.split(is_zero as fn(&i32) -> bool);
            assert_eq!(it.next(), Some(&[a][..]));
            assert_eq!(it.next(), Some(&[b][..]));
            assert_eq!(it.next(), None);
        }
    }
}
