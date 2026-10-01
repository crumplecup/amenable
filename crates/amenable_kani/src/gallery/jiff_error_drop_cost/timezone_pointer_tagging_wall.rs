::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::jiff_error_drop_cost::timezone_to_offset_on_concrete_utc_times_out".to_owned(),
            "gallery::jiff_error_drop_cost::timezone_pointer_tagging_wall::timezone_to_offset_on_concrete_utc_times_out".to_owned(),
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
            "gallery::jiff_error_drop_cost::timezone_pointer_tagging_wall::timezone_utc_construction_alone_passes".to_owned(),
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
