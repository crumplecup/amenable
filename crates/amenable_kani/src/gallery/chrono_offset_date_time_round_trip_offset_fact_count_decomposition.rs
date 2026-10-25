//! Offset-only round-trip decomposition, `reflect_offset`'s own
//! contract decomposed by fact count, for the CBMC round-trip timeout
//! investigation -- see `chrono_offset_date_time_round_trip.rs`'s own
//! module doc comment for the full narrative this file is one chapter
//! of. `reflect_offset`'s own contract -- a single real call, no
//! chaining -- timed out once its ensures stated the real relation
//! (sign matches *and* the three-field magnitude reconstruction
//! matches), despite the identical body passing easily under a
//! trivial ensures (see the sibling
//! `chrono_offset_date_time_round_trip_offset_fact_count_baseline.rs`).
//! Decomposed by fact count instead: each of the four individual facts
//! alone passes cheaply; two facts together still pass; three facts
//! pass but right at the edge of the project's 3-minute budget; all
//! four together times out, confirmed twice under two different
//! equality shapes. The threshold is specifically about how many
//! correlated facts one proof obligation states at once, not which
//! facts or their arithmetic representation. The staged fix built from
//! this decomposition lives in the sibling
//! `chrono_offset_date_time_round_trip_offset_staged_fix.rs`.

mod kani_only {
    #![cfg(kani)]

    // ── Decomposing the single-call reflect-only predicate ──────────
    //
    // `check_reflect_offset`'s real, meaningful ensures (sign matches
    // *and* the three-field magnitude reconstruction matches) timed
    // out on `reflect_offset` ALONE -- one real call, no chaining --
    // even though the identical body with a trivial `true` ensures
    // passed in 126.7s. These isolate which single piece of that
    // conjunction, if any, is the actual cost, versus the conjunction
    // itself being the cost regardless of its parts.

    /// Same body, ensures checks *only* the sign relation.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let seconds = offset.local_minus_utc();
                match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => seconds >= 0,
                    amenable_time::UtcOffsetSign::Negative => seconds < 0,
                }
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_sign_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// Same body, ensures checks *only* `hours()` -- a single required,
    /// non-`Option` field, no `unwrap_or` branching.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let magnitude = offset.local_minus_utc().unsigned_abs();
                i64::from(descriptor.hours()) == i64::from(magnitude / 3600)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_hours_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// Same body, ensures checks *only* `minutes()` -- the first
    /// `Option<u8>`/`unwrap_or(0)` field, set conditionally by the real
    /// builder (`if remainder_minutes != 0 { builder.minutes(...) }`).
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let magnitude = offset.local_minus_utc().unsigned_abs();
                let remainder_minutes = (magnitude % 3600) / 60;
                i64::from(descriptor.minutes().unwrap_or(0)) == i64::from(remainder_minutes)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_minutes_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// Same body, ensures checks *only* `seconds()` -- the second
    /// conditionally-set `Option<u8>` field.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let magnitude = offset.local_minus_utc().unsigned_abs();
                let remainder_seconds = magnitude % 60;
                i64::from(descriptor.seconds().unwrap_or(0)) == i64::from(remainder_seconds)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_seconds_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    // ── Pairwise combinations ────────────────────────────────────────
    //
    // Every single fact above passed. These narrow in on exactly which
    // *pairing* of facts starts compounding: two independent branch
    // dimensions (sign, hours -- neither conditionally set) versus two
    // jointly conditional ones (minutes, seconds -- each independently
    // `Some`/`None`, four branch combinations together).

    /// `sign` and `hours` together -- neither field is conditionally
    /// set by the real builder.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let seconds = offset.local_minus_utc();
                let sign_matches = match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => seconds >= 0,
                    amenable_time::UtcOffsetSign::Negative => seconds < 0,
                };
                let magnitude = seconds.unsigned_abs();
                sign_matches && i64::from(descriptor.hours()) == i64::from(magnitude / 3600)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_sign_and_hours(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// `minutes` and `seconds` together -- both conditionally set by
    /// the real builder (`Some`/`None` independently), four branch
    /// combinations jointly instead of two.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let magnitude = offset.local_minus_utc().unsigned_abs();
                let remainder_minutes = (magnitude % 3600) / 60;
                let remainder_seconds = magnitude % 60;
                i64::from(descriptor.minutes().unwrap_or(0)) == i64::from(remainder_minutes)
                    && i64::from(descriptor.seconds().unwrap_or(0)) == i64::from(remainder_seconds)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_minutes_and_seconds(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// Three facts: `hours`, `minutes`, `seconds` -- the full magnitude
    /// reconstruction, `sign` deliberately left out. Brackets between
    /// the passing pair (`minutes_and_seconds`, borderline at ~180s)
    /// and the timing-out full four-fact conjunction.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let magnitude = offset.local_minus_utc().unsigned_abs();
                let remainder_minutes = (magnitude % 3600) / 60;
                let remainder_seconds = magnitude % 60;
                i64::from(descriptor.hours()) == i64::from(magnitude / 3600)
                    && i64::from(descriptor.minutes().unwrap_or(0)) == i64::from(remainder_minutes)
                    && i64::from(descriptor.seconds().unwrap_or(0)) == i64::from(remainder_seconds)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_magnitude_only(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }

    /// All four facts (`sign`, `hours`, `minutes`, `seconds`), but
    /// stated as four *separate* per-field equalities ANDed together --
    /// never summed into one combined `magnitude` value first, unlike
    /// the original `reflect_offset` contract (`sign_matches &&
    /// magnitude == ...`, a single equality over a three-term sum).
    /// Isolates "the sum-based equality's own cost" from "four facts is
    /// simply too many," now that `reflect_offset_magnitude_only`
    /// showed three per-field equalities (no sum) pass barely.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError>| {
        match result {
            Ok(descriptor) => {
                let seconds = offset.local_minus_utc();
                let sign_matches = match descriptor.sign() {
                    amenable_time::UtcOffsetSign::Positive => seconds >= 0,
                    amenable_time::UtcOffsetSign::Negative => seconds < 0,
                };
                let magnitude = seconds.unsigned_abs();
                let remainder_minutes = (magnitude % 3600) / 60;
                let remainder_seconds = magnitude % 60;
                sign_matches
                    && i64::from(descriptor.hours()) == i64::from(magnitude / 3600)
                    && i64::from(descriptor.minutes().unwrap_or(0)) == i64::from(remainder_minutes)
                    && i64::from(descriptor.seconds().unwrap_or(0)) == i64::from(remainder_seconds)
            }
            Err(_) => true,
        }
    })]
    pub(super) fn reflect_offset_per_field(
        offset: chrono::FixedOffset,
    ) -> Result<amenable_time::UtcOffsetDescriptor, amenable_time::TemporalError> {
        amenable_ext::fixed_offset_to_utc_offset_descriptor(offset)
    }
}

#[cfg(kani)]
use kani_only::{
    reflect_offset_hours_only, reflect_offset_magnitude_only, reflect_offset_minutes_and_seconds,
    reflect_offset_minutes_only, reflect_offset_per_field, reflect_offset_seconds_only,
    reflect_offset_sign_and_hours, reflect_offset_sign_only,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_sign_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_sign_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks only the sign relation -- confirmed passes (126.9s): the sign check alone is not the cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_SIGN_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_sign_only`'s own contract.
        #[kani::proof_for_contract(reflect_offset_sign_only)]
        fn reflect_offset_sign_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_sign_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_hours_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_hours_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks only hours() -- confirmed passes (147.9s): the required non-Option field alone is not the cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_HOURS_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_hours_only`'s own contract.
        #[kani::proof_for_contract(reflect_offset_hours_only)]
        fn reflect_offset_hours_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_hours_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_minutes_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_minutes_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks only minutes() -- confirmed passes (160.1s): the conditionally-set Option<u8> field alone is not the cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_MINUTES_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_minutes_only`'s own contract.
        #[kani::proof_for_contract(reflect_offset_minutes_only)]
        fn reflect_offset_minutes_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_minutes_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_seconds_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_seconds_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks only seconds() -- confirmed passes (142.5s): the second conditionally-set Option<u8> field alone is not the cost either".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_SECONDS_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_seconds_only`'s own contract.
        #[kani::proof_for_contract(reflect_offset_seconds_only)]
        fn reflect_offset_seconds_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_seconds_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_sign_and_hours_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_sign_and_hours_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks sign and hours together -- confirmed passes (154.4s)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_SIGN_AND_HOURS_ISOLATION_SRC, {
        /// `reflect_offset_sign_and_hours`'s own contract.
        #[kani::proof_for_contract(reflect_offset_sign_and_hours)]
        fn reflect_offset_sign_and_hours_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_sign_and_hours(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_minutes_and_seconds_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_minutes_and_seconds_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks minutes and seconds together -- confirmed passes, but borderline (~180s wall against the 3-minute budget)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_MINUTES_AND_SECONDS_ISOLATION_SRC, {
        /// `reflect_offset_minutes_and_seconds`'s own contract.
        #[kani::proof_for_contract(reflect_offset_minutes_and_seconds)]
        fn reflect_offset_minutes_and_seconds_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_minutes_and_seconds(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_magnitude_only_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_magnitude_only_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "reflect_offset alone, ensures checks hours+minutes+seconds (full magnitude, sign left out) -- confirmed passes, right at the edge (~187s wall)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_MAGNITUDE_ONLY_ISOLATION_SRC, {
        /// `reflect_offset_magnitude_only`'s own contract.
        #[kani::proof_for_contract(reflect_offset_magnitude_only)]
        fn reflect_offset_magnitude_only_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_magnitude_only(offset);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::reflect_offset_per_field_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::reflect_offset_per_field_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "all four facts, but as four separate per-field equalities, never summed into one combined magnitude -- confirmed still times out: the fourth fact (sign) tips it over regardless of the sum-vs-per-field representation".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REFLECT_OFFSET_PER_FIELD_ISOLATION_SRC, {
        /// `reflect_offset_per_field`'s own contract.
        #[kani::proof_for_contract(reflect_offset_per_field)]
        fn reflect_offset_per_field_isolation() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = reflect_offset_per_field(offset);
        }
    }
}
