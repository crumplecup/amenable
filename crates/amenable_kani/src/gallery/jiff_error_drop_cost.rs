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
//!
//! **A second, distinct `jiff::Error` wall (found assessing `jiff::Error`
//! itself as a witness candidate, not `Offset`): every accessor times
//! out too, deterministic input, no Drop involved at all.** A single
//! fixed `jiff::Error::from_args(format_args!("something failed"))`,
//! immediately `mem::forget`-ed (so the drop-glue cost above can't be
//! the explanation), still times out the moment *any* accessor is
//! called on it — `.to_string()`, and even the cheap-looking boolean
//! `.is_range()`. Root-caused by reading jiff's real source, not
//! guessed: `Error::root()`/`is_range()`/`Display::fmt` all go through
//! `Error::chain()`, which returns `impl Iterator<Item = &Error>` over
//! the (potentially recursive) `cause` field and calls `.last()`/loops
//! over `.next()` — the exact "iterator adapter with an unwind bound
//! Kani's unwinder can't conclude is finite" shape this crate's own
//! [[project_kani_failure_patterns]] catalog already names (pattern 1),
//! just reached through `jiff::Error`'s own chain-walking accessors
//! instead of a `Range`/`Iterator` adapter directly. Since every public
//! accessor on `Error` routes through `chain()`, there is no accessor-
//! level property left to check — `ext::jiff::error`'s witness is
//! trusted for Kani specifically for this reason, not by default.
//!
//! **A third case (found assessing `jiff::TimestampSeries` as a witness
//! candidate): the exact same recursive-Arc Drop-glue wall, reached
//! through a completely different, *unavoidable* call site.**
//! `Timestamp::series(period)`'s own real implementation
//! (`TimestampSeries::new`) is `let duration =
//! SignedDuration::try_from(period).ok(); TimestampSeries { ts,
//! duration }` — the fallible `TryFrom<Span> for SignedDuration`
//! conversion (whose `Error` is the same `jiff::Error`) happens
//! *inside* `.series()` itself, with `.ok()` dropping the transient
//! `Result`'s `Err` arm immediately, before `.series()` even returns.
//! Isolated by direct substitution, same technique as the `Offset`
//! case above: a bare `Timestamp::from_second(secs)` call (no series
//! involved) passes instantly; adding `.series(period_secs.seconds())`
//! afterward times out, even with the *entire* `series` value
//! immediately `mem::forget`-ed (ruling out the series struct's own
//! Drop, since `TimestampSeries` derives no custom `Drop` at all —
//! confirming the cost is paid *during* construction, inside jiff's
//! own `.ok()` call, not on anything the caller can reach or skip).
//! Unlike the `Offset` case, there is no fix available here: `Offset`
//! let the harness call the fallible constructor *directly*, so an
//! independent bounds check upstream could keep the call from ever
//! reaching its `Err` arm; `Timestamp::series`'s fallible conversion is
//! entirely private to jiff's own implementation, with no way to gate
//! around it from outside. `ext::jiff::timestamp_series`'s witness is
//! trusted for Kani specifically for this reason (checked on Creusot
//! and Verus instead, neither of which shares this Rust-Drop-glue
//! mechanism), not by default.

#[cfg(kani)]
use jiff::ToSpan;

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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::any_accessor_on_a_deterministic_forgotten_error_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::any_accessor_on_a_deterministic_forgotten_error_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "A single fixed, mem::forget-ed jiff::Error still times out the moment any accessor (.to_string(), even the boolean .is_range()) is called on it -- isolating the cost to Error::chain()'s unbounded iterator traversal, not Drop".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ANY_ACCESSOR_ON_A_DETERMINISTIC_FORGOTTEN_ERROR_TIMES_OUT_SRC, {
        /// Deterministic input, `mem::forget` on the error itself (so
        /// drop-glue cost is ruled out), and still a timeout the moment
        /// `.is_range()` is called — confirms the wall is in
        /// `Error::chain()`'s iterator traversal, reached by every
        /// public accessor, not in Drop or in construction.
        #[kani::proof]
        fn any_accessor_on_a_deterministic_forgotten_error_times_out() {
            let err = jiff::Error::from_args(format_args!("something failed"));
            let is_range = err.is_range();
            std::mem::forget(err);
            assert!(!is_range);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::timestamp_series_construction_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::timestamp_series_construction_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "Timestamp::series(period) times out even for a tiny assumed range and even when the entire returned TimestampSeries is immediately mem::forget-ed -- TimestampSeries::new's own internal SignedDuration::try_from(period).ok() call drops a transient jiff::Error before series() even returns, unreachable from outside".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TIMESTAMP_SERIES_CONSTRUCTION_TIMES_OUT_SRC, {
        /// A bare `Timestamp::from_second(secs).series(period)` call,
        /// forgotten immediately, still times out — isolating the cost
        /// to `.series()`'s own internal fallible conversion, not
        /// anything the caller does with the result.
        #[kani::proof]
        fn timestamp_series_construction_times_out() {
            let secs: i64 = kani::any();
            let period_secs: i64 = kani::any();
            kani::assume(secs >= -300_000_000_000 && secs <= 200_000_000_000);
            kani::assume(period_secs >= -1_000_000_000 && period_secs <= 1_000_000_000);
            let ts = jiff::Timestamp::from_second(secs)
                .expect("secs is already checked to be within Timestamp's valid range");
            let series = ts.series(period_secs.seconds());
            std::mem::forget(series);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::timestamp_from_second_alone_passes".to_owned(),
            "gallery::jiff_error_drop_cost::timestamp_from_second_alone_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical symbolic Timestamp::from_second(secs) call, with no .series() call at all, verifies instantly -- confirming from_second itself is not the source of the timeout, isolating it specifically to .series()'s internal conversion".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TIMESTAMP_FROM_SECOND_ALONE_PASSES_SRC, {
        /// `timestamp_series_construction_times_out`, with the
        /// `.series()` call removed — confirms `from_second` alone is
        /// fast, isolating the wall to `.series()` specifically.
        #[kani::proof]
        fn timestamp_from_second_alone_passes() {
            let secs: i64 = kani::any();
            kani::assume(secs >= -300_000_000_000 && secs <= 200_000_000_000);
            let ts = jiff::Timestamp::from_second(secs)
                .expect("secs is already checked to be within Timestamp's valid range");
            assert!(ts.as_second() == secs);
        }
    }
}
