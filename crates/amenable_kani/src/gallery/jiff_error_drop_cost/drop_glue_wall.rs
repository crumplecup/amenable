#[cfg(kani)]
use jiff::ToSpan;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::normal_drop_of_a_symbolic_jiff_result_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::normal_drop_of_a_symbolic_jiff_result_times_out".to_owned(),
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
            "gallery::jiff_error_drop_cost::drop_glue_wall::mem_forget_of_the_identical_symbolic_result_passes".to_owned(),
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
            "gallery::jiff_error_drop_cost::drop_glue_wall::any_accessor_on_a_deterministic_forgotten_error_times_out".to_owned(),
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
            "gallery::jiff_error_drop_cost::drop_glue_wall::timestamp_series_construction_times_out".to_owned(),
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
            "gallery::jiff_error_drop_cost::drop_glue_wall::timestamp_from_second_alone_passes".to_owned(),
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
            "amenable_kani::gallery::jiff_error_drop_cost::date_series_next_call_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::date_series_next_call_times_out".to_owned(),
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
            "gallery::jiff_error_drop_cost::drop_glue_wall::civil_date_new_alone_passes".to_owned(),
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
::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::date_time_series_next_call_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::date_time_series_next_call_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "A single .next() call on a DateTime::series(period) iterator times out even for a tiny assumed range and even when the returned Option<DateTime> is immediately mem::forget-ed -- DateTimeSeries::next's own checked_mul/checked_add calls drop a transient jiff::Error via .ok()? on every step, the same Drop-glue wall DateSeries/TimestampSeries hit, unrelated to TimeZone (DateTime has none)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DATE_TIME_SERIES_NEXT_CALL_TIMES_OUT_SRC, {
        /// A bare `Date::new(year, month, day).at(0, 0, 0, 0).series(
        /// period).next()` call, forgotten immediately, still times
        /// out — isolating the cost to `DateTimeSeries::next`'s own
        /// internal fallible conversions, the identical `jiff::Error`
        /// Drop-glue wall `DateSeries` hits.
        #[kani::proof]
        fn date_time_series_next_call_times_out() {
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
            let dt = d.at(0, 0, 0, 0);
            let mut series = dt.series(period_days.days());
            let next = series.next();
            std::mem::forget(series);
            std::mem::forget(next);
        }
    }
}
::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::civil_date_at_midnight_alone_passes".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::civil_date_at_midnight_alone_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical symbolic Date::new(year, month, day).at(0, 0, 0, 0) call, with no .series()/.next() call at all, verifies instantly -- confirming the cheap DateTime construction itself is not the source of the timeout, isolating it specifically to DateTimeSeries::next's internal conversions".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CIVIL_DATE_AT_MIDNIGHT_ALONE_PASSES_SRC, {
        /// `date_time_series_next_call_times_out`, with the
        /// `.series()`/`.next()` calls removed — confirms the cheap
        /// `DateTime` construction alone is fast, isolating the wall
        /// to `DateTimeSeries::next` specifically.
        #[kani::proof]
        fn civil_date_at_midnight_alone_passes() {
            let year: i16 = kani::any();
            let month: i8 = kani::any();
            let day: i8 = kani::any();
            kani::assume(year >= -9999 && year <= 9999);
            kani::assume(month >= 1 && month <= 12);
            kani::assume(day >= 1 && day <= 28);
            let d = jiff::civil::Date::new(year, month, day)
                .expect("year/month/day are already checked to always be a valid civil::Date");
            let dt = d.at(0, 0, 0, 0);
            assert!(dt.date() == d);
        }
    }
}
::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::civil_time_series_next_call_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::civil_time_series_next_call_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "A single .next() call on a Time::series(period) iterator times out even for a tiny assumed range and even when the returned Option<Time> is immediately mem::forget-ed -- TimeSeries::next's own checked_mul/checked_add calls drop a transient jiff::Error via .ok()? on every step, the same Drop-glue wall DateSeries/DateTimeSeries/TimestampSeries hit, unrelated to TimeZone (Time has none)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CIVIL_TIME_SERIES_NEXT_CALL_TIMES_OUT_SRC, {
        /// A bare `Time::new(hour, minute, second,
        /// subsec_nanosecond).series(period).next()` call, forgotten
        /// immediately, still times out — isolating the cost to
        /// `TimeSeries::next`'s own internal fallible conversions,
        /// the identical `jiff::Error` Drop-glue wall `DateSeries`/
        /// `DateTimeSeries` hit.
        #[kani::proof]
        fn civil_time_series_next_call_times_out() {
            let hour: i8 = kani::any();
            let minute: i8 = kani::any();
            let second: i8 = kani::any();
            let subsec_nanosecond: i32 = kani::any();
            let period_hours: i64 = kani::any();
            kani::assume(hour >= 0 && hour <= 23);
            kani::assume(minute >= 0 && minute <= 59);
            kani::assume(second >= 0 && second <= 59);
            kani::assume(subsec_nanosecond >= 0 && subsec_nanosecond <= 999_999_999);
            kani::assume(period_hours >= -1_000_000 && period_hours <= 1_000_000);
            let t = jiff::civil::Time::new(hour, minute, second, subsec_nanosecond).expect(
                "hour/minute/second/subsec_nanosecond are already checked to always be a valid civil::Time",
            );
            let mut series = t.series(period_hours.hours());
            let next = series.next();
            std::mem::forget(series);
            std::mem::forget(next);
        }
    }
}
::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::civil_time_new_alone_passes".to_owned(),
            "gallery::jiff_error_drop_cost::drop_glue_wall::civil_time_new_alone_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical symbolic Time::new(hour, minute, second, subsec_nanosecond) call, with no .series()/.next() call at all, verifies instantly -- confirming Time::new itself is not the source of the timeout, isolating it specifically to TimeSeries::next's internal conversions".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CIVIL_TIME_NEW_ALONE_PASSES_SRC, {
        /// `civil_time_series_next_call_times_out`, with the
        /// `.series()`/`.next()` calls removed — confirms `Time::new`
        /// alone is fast, isolating the wall to `TimeSeries::next`
        /// specifically.
        #[kani::proof]
        fn civil_time_new_alone_passes() {
            let hour: i8 = kani::any();
            let minute: i8 = kani::any();
            let second: i8 = kani::any();
            let subsec_nanosecond: i32 = kani::any();
            kani::assume(hour >= 0 && hour <= 23);
            kani::assume(minute >= 0 && minute <= 59);
            kani::assume(second >= 0 && second <= 59);
            kani::assume(subsec_nanosecond >= 0 && subsec_nanosecond <= 999_999_999);
            let t = jiff::civil::Time::new(hour, minute, second, subsec_nanosecond).expect(
                "hour/minute/second/subsec_nanosecond are already checked to always be a valid civil::Time",
            );
            assert!(t.hour() == hour);
        }
    }
}
