//! Isolates a real CBMC timeout hit while building the first checked
//! `amenable_ext` jiff witness (`ext::jiff::offset`, `jiff::tz::Offset`):
//! any harness that lets a symbolic `Result<T, jiff::Error>` drop
//! *normally* on its `Err` arm times out at 3 minutes, even for a tiny
//! assumed input range — despite the reachable bounds-check logic being
//! a single comparison.
//!
//! **Root cause, isolated by direct substitution:** `jiff::Error` is
//! `Option<Arc<ErrorInner>>`-backed, and `ErrorInner` holds `cause:
//! Option<Error>` — a recursive, heap-backed error chain. Narrowing the
//! harness down to a bare `Offset::from_seconds(secs)` call with the
//! result immediately discarded (no `assert!`, no `ensures`, no macro
//! indirection) still times out; wrapping that exact call in
//! `std::mem::forget` instead of a normal drop passes instantly. That
//! isolates the cost to Drop glue specifically, not general call-graph
//! complexity — Kani compiles with `panic = "abort"`, so `.unwrap()`'s
//! `Err` arm (a panic) never runs Drop either, and passes just as fast
//! as `mem::forget` does.
//!
//! **The fix**: never call `Offset::from_seconds` on a value that might
//! be out of range in the first place. An independent, Drop-free `i32`
//! bounds check (jiff's own documented `-93,599..=93,599`) gates the
//! call, so `.unwrap()` is a safe assertion under a proven precondition,
//! not a workaround — see `ext::jiff::offset`'s real, shipped witness,
//! which verifies for every `i32` via this exact shape, not an assumed
//! slice of one.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::normal_drop_of_a_symbolic_jiff_result_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::normal_drop_of_a_symbolic_jiff_result_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "A symbolic i32 fed into jiff::tz::Offset::from_seconds, with the Result discarded via `let _ = ..;` (a normal, non-panicking drop), times out even within a tiny assumed valid range".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, NORMAL_DROP_OF_A_SYMBOLIC_JIFF_RESULT_TIMES_OUT_SRC, {
        /// The minimal reproduction: no assertion, no ensures, just a
        /// symbolic `Offset::from_seconds` call whose `Result` is
        /// discarded normally. Times out even constrained to a tiny
        /// five-value slop around the documented valid boundary.
        #[kani::proof]
        fn normal_drop_of_a_symbolic_jiff_result_times_out() {
            let secs: i32 = kani::any();
            kani::assume(secs >= -93_604 && secs <= 93_604);
            let _ = jiff::tz::Offset::from_seconds(secs);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::mem_forget_of_the_identical_symbolic_result_passes".to_owned(),
            "gallery::jiff_error_drop_cost::mem_forget_of_the_identical_symbolic_result_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical symbolic from_seconds call, with the Result skipped via std::mem::forget instead of a normal drop, verifies instantly -- isolating the cost to Drop glue on jiff::Error's recursive Arc chain, not the bounds-check logic itself".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, MEM_FORGET_OF_THE_IDENTICAL_SYMBOLIC_RESULT_PASSES_SRC, {
        /// `normal_drop_of_a_symbolic_jiff_result_times_out`, with the
        /// one change that isolated the cause: `std::mem::forget`
        /// instead of an implicit drop. Never shipped as a real
        /// witness — forgetting a `Result` on purpose proves nothing —
        /// this case exists only to confirm the diagnosis.
        #[kani::proof]
        fn mem_forget_of_the_identical_symbolic_result_passes() {
            let secs: i32 = kani::any();
            kani::assume(secs >= -93_604 && secs <= 93_604);
            std::mem::forget(jiff::tz::Offset::from_seconds(secs));
        }
    }
}
