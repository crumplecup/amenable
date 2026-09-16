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
//!
//! **A fourth, genuinely different wall (found assessing
//! `jiff::ZonedSeries` as a witness candidate): not Drop glue at all,
//! but `TimeZone`'s own pointer-tagged internal representation.**
//! First hypothesized (wrongly) as the same `Error`-drop mechanism
//! reached through `ZonedSeries::next()`'s `checked_mul`/`checked_add`
//! calls — checked directly instead of assumed, and the real cause is
//! earlier and more fundamental: merely *constructing* a `Zoned`,
//! before any series/iteration logic runs at all, already times out.
//! Isolated by direct substitution, narrowing steadily: `TimeZone::UTC`
//! alone (a bare `const`, `mem::forget`-ed) verifies instantly, but
//! `TimeZone::UTC.to_offset(timestamp)` — even for a single *concrete*
//! `Timestamp::from_second(0)`, no symbolic input anywhere — times out.
//! Root-caused by reading jiff's real source
//! (`tz::timezone::repr`): `TimeZone`'s `Repr` is a hand-rolled
//! pointer-tagging union (`ptr: *const u8`, encoding UTC/fixed/tzif
//! variants in the low bits of what is never a real allocation),
//! built via `without_provenance(addr) -> *const u8` — a bare `unsafe {
//! core::mem::transmute(addr) }` from a `usize` straight into a
//! pointer, with the reverse transmute used again to read the tag back
//! out in `Repr::tag()`. CBMC's pointer/memory object model has to
//! treat that transmuted value as a genuine pointer despite it never
//! addressing real memory, and pays a wall-clock cost for it even on
//! the single cheapest, fully concrete `UTC` case — nothing to do with
//! `jiff::Error`'s recursive Drop glue, an entirely distinct root cause
//! that happens to land in the same file because it was found while
//! chasing the same family of witness. `ext::jiff::zoned_series`'s
//! witness is trusted for Kani specifically for this reason (checked
//! on Creusot and Verus instead, neither of which shares CBMC's
//! pointer/memory model) — and this same wall applies to *any* future
//! Kani witness needing a real, non-trivial `Zoned` construction, not
//! just this one.
//!
//! **A fifth case (found assessing `jiff::civil::DateSeries` as a
//! witness candidate): back to the SAME wall as `TimestampSeries`,
//! not `ZonedSeries`'s — confirmed, not assumed from either
//! resemblance.** `Date::series(period)`'s own implementation is
//! cheap (`DateSeries { start: self, period, step: 0 }`, no fallible
//! call), and `Date` has no `TimeZone` at all, so the fourth case's
//! `Repr` pointer-tagging wall cannot apply here. But `DateSeries::
//! next()` calls `self.period.checked_mul(self.step).ok()?` and
//! `self.start.checked_add(span).ok()?`, both `Result<_,
//! jiff::Error>`-returning — the identical recursive-Arc Drop-glue
//! wall as `TimestampSeries`. Isolated the same way: a bare `Date::
//! new(year, month, day)` call passes instantly; adding `.series(
//! period).next()` times out even with the result immediately
//! `mem::forget`-ed.

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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::timezone_to_offset_on_concrete_utc_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::timezone_to_offset_on_concrete_utc_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "TimeZone::UTC.to_offset(Timestamp::from_second(0)) times out even though every input is concrete (no symbolic values at all) -- the wall is in TimeZone's own hand-rolled pointer-tagged Repr, not in symbolic-input complexity or Drop glue".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TIMEZONE_TO_OFFSET_ON_CONCRETE_UTC_TIMES_OUT_SRC, {
        /// A single, fully concrete `TimeZone::UTC.to_offset(ts)` call,
        /// the result `mem::forget`-ed immediately — still times out.
        /// No symbolic input anywhere in this harness, ruling out
        /// symbolic-domain complexity as the explanation; the wall is
        /// in `to_offset` itself, reached via `Repr`'s pointer-tagging
        /// dispatch (`without_provenance`'s `usize`-to-pointer
        /// transmute and its reverse in `Repr::tag()`).
        #[kani::proof]
        fn timezone_to_offset_on_concrete_utc_times_out() {
            let tz = jiff::tz::TimeZone::UTC;
            let ts = jiff::Timestamp::from_second(0)
                .expect("0 is within Timestamp's valid range");
            let offset = tz.to_offset(ts);
            std::mem::forget(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::timezone_utc_construction_alone_passes".to_owned(),
            "gallery::jiff_error_drop_cost::timezone_utc_construction_alone_passes".to_owned(),
            "amenable_kani".to_owned(),
            "TimeZone::UTC alone, mem::forget-ed with no to_offset/to_datetime call at all, verifies instantly -- confirming the bare pointer-tagged constant itself is not the source of the timeout, isolating it specifically to to_offset's Repr-dispatch machinery".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TIMEZONE_UTC_CONSTRUCTION_ALONE_PASSES_SRC, {
        /// `timezone_to_offset_on_concrete_utc_times_out`, with the
        /// `.to_offset()` call removed — confirms the bare constant
        /// alone is fast, isolating the wall to `to_offset`'s `Repr`
        /// dispatch specifically.
        #[kani::proof]
        fn timezone_utc_construction_alone_passes() {
            let tz = jiff::tz::TimeZone::UTC;
            std::mem::forget(tz);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::date_series_next_call_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::date_series_next_call_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "A single .next() call on a Date::series(period) iterator times out even for a tiny assumed range and even when the returned Option<Date> is immediately mem::forget-ed -- DateSeries::next's own checked_mul/checked_add calls drop a transient jiff::Error via .ok()? on every step, the same Drop-glue wall TimestampSeries hits, unrelated to TimeZone (Date has none)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DATE_SERIES_NEXT_CALL_TIMES_OUT_SRC, {
        /// A bare `Date::new(year, month, day).series(period).next()`
        /// call, forgotten immediately, still times out — isolating
        /// the cost to `DateSeries::next`'s own internal fallible
        /// conversions, the same `jiff::Error` Drop-glue wall
        /// `TimestampSeries` hits (confirmed distinct from
        /// `ZonedSeries`'s `TimeZone::Repr` wall, since `Date` has no
        /// time zone at all).
        #[kani::proof]
        fn date_series_next_call_times_out() {
            let year: i16 = kani::any();
            let month: i8 = kani::any();
            let day: i8 = kani::any();
            let period_days: i64 = kani::any();
            kani::assume(year >= -9999 && year <= 9999);
            kani::assume(month >= 1 && month <= 12);
            kani::assume(day >= 1 && day <= 28);
            kani::assume(period_days >= -1_000_000_000 && period_days <= 1_000_000_000);
            let d = jiff::civil::Date::new(year, month, day)
                .expect("year/month/day are already checked to always be a valid civil::Date");
            let mut series = d.series(period_days.days());
            let next = series.next();
            std::mem::forget(series);
            std::mem::forget(next);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::civil_date_new_alone_passes".to_owned(),
            "gallery::jiff_error_drop_cost::civil_date_new_alone_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical symbolic Date::new(year, month, day) call, with no .series()/.next() call at all, verifies instantly -- confirming Date::new itself is not the source of the timeout, isolating it specifically to DateSeries::next's internal conversions".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CIVIL_DATE_NEW_ALONE_PASSES_SRC, {
        /// `date_series_next_call_times_out`, with the
        /// `.series()`/`.next()` calls removed — confirms `Date::new`
        /// alone is fast, isolating the wall to `DateSeries::next`
        /// specifically.
        #[kani::proof]
        fn civil_date_new_alone_passes() {
            let year: i16 = kani::any();
            let month: i8 = kani::any();
            let day: i8 = kani::any();
            kani::assume(year >= -9999 && year <= 9999);
            kani::assume(month >= 1 && month <= 12);
            kani::assume(day >= 1 && day <= 28);
            let d = jiff::civil::Date::new(year, month, day)
                .expect("year/month/day are already checked to always be a valid civil::Date");
            assert!(d.year() == year && d.month() == month && d.day() == day);
        }
    }
}
