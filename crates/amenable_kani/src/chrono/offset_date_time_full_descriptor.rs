//! Verifies the full-descriptor/time-builder contracts built on top
//! of the fractional-digit core (`amenable_time::FractionalSecond
//! Descriptor` construction, the `TimeDescriptorBuilder`'s fractional-
//! second field, and the whole `OffsetDateTimeDescriptor` assembled
//! from symbolic year/month/day/hour/minute/second/nanos/offset).
//! Split out of `offset_date_time.rs` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    pub(super) struct NanosAreWithinFractionalSecondRange;
    crate::rust_std::kani_local_claim!(NanosAreWithinFractionalSecondRange);

    impl amenable_core::Requires<crate::KaniVerifier> for NanosAreWithinFractionalSecondRange {
        type Input = i64;
        type Bound = bool;

        fn requires(nanos: i64) -> bool {
            nanos > 0 && nanos <= 999_999_999
        }
    }

    /// `narrow_to_u8_in_range`'s own precondition (`value <= 255`), applied
    /// to the three clock fields together at once -- named for the same
    /// reason as `NanosAreWithinFractionalSecondRange` just above.
    pub(super) struct HourMinuteSecondAreEachValidForNarrowing;
    crate::rust_std::kani_local_claim!(HourMinuteSecondAreEachValidForNarrowing);

    impl amenable_core::Requires<crate::KaniVerifier> for HourMinuteSecondAreEachValidForNarrowing {
        type Input = (u32, u32, u32);
        type Bound = bool;

        fn requires((hour, minute, second): Self::Input) -> bool {
            hour <= 255 && minute <= 255 && second <= 255
        }
    }

    /// The same `narrow_to_u8_in_range` precondition, applied to the two
    /// calendar fields `narrow_to_u8_in_range` is called on together.
    pub(super) struct MonthAndDayAreEachValidForNarrowing;
    crate::rust_std::kani_local_claim!(MonthAndDayAreEachValidForNarrowing);

    impl amenable_core::Requires<crate::KaniVerifier> for MonthAndDayAreEachValidForNarrowing {
        type Input = (u32, u32);
        type Bound = bool;

        fn requires((month, day): Self::Input) -> bool {
            month <= 255 && day <= 255
        }
    }
}

#[cfg(kani)]
use amenable_core::Requires;
#[cfg(kani)]
use kani_only::{
    HourMinuteSecondAreEachValidForNarrowing, MonthAndDayAreEachValidForNarrowing,
    NanosAreWithinFractionalSecondRange,
};

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time_full_descriptor::NanosAreWithinFractionalSecondRange",
        "kani",
        "requires",
        || "nanos > 0 && nanos <= 999_999_999",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time_full_descriptor::HourMinuteSecondAreEachValidForNarrowing",
        "kani",
        "requires",
        || "hour <= 255 && minute <= 255 && second <= 255",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time_full_descriptor::MonthAndDayAreEachValidForNarrowing",
        "kani",
        "requires",
        || "month <= 255 && day <= 255",
    )
}


amenable_derive::harness! {
    kani, CHECK_NANOS_TO_FRACTIONAL_SECONDS_DIGITS_PLAIN_SRC, {
        /// FLAGGED, NOT PASSING (2026-11-01): a real re-run via the
        /// protected `amenable verify kani` CLI OOMs (CBMC status 6,
        /// clean exit, not a crash). Calls the real
        /// `nanos_to_fractional_seconds_digits`, which builds a real
        /// `String` with a symbolic trim length (`end`, computed from
        /// a trailing-zero scan over symbolic digit content) -- a
        /// third instance of the same class this investigation has
        /// already flagged twice elsewhere (`fractional_digit_round_
        /// trip`, `digit_bytes_to_kani_fractional_second_descriptor_
        /// refines`). Deliberately not fixed now -- flagged and
        /// deferred until the open cordial worklist clears, so that
        /// doesn't turn into open-ended proof-architecture work.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        fn check_nanos_to_fractional_seconds_digits_plain() {
            use crate::ValueIsWithinInclusiveRange;

            let nanos: i64 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((nanos, 0, 999_999_999)));
            let _ = amenable_ext::nanos_to_fractional_seconds_digits(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_DIGIT_BYTES_TO_STRING_UNCONDITIONAL_PLAIN_SRC, {
        /// Diagnostic: the one new piece -- `digit_bytes_to_string_
        /// unconditional`, called plainly, result discarded, never
        /// read back. Isolates whether this specific function, in
        /// amenable_ext, is cheap on its own before it's used inside
        /// the much larger reflect harness.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        fn check_digit_bytes_to_string_unconditional_plain() {
            use crate::ValueIsWithinInclusiveRange;

            let nanos: i64 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((nanos, 0, 999_999_999)));
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let _ = amenable_ext::digit_bytes_to_string_unconditional(digits);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_TIME_BUILDER_WITH_FRACTIONAL_SECOND_SRC, {
        /// Diagnostic, one layer up: attaches the (now confirmed
        /// cheap) fractional-second `String` to a real `LocalTime
        /// DescriptorBuilder` via `FractionalSecondDescriptor::new`
        /// and calls `.build()` -- hour/minute/second held concrete,
        /// no `chrono` construction, no calendar logic at all.
        /// Isolates whether the builder/descriptor attachment itself
        /// adds cost on top of the already-cheap string construction,
        /// before combining with the rest of the reflect function.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        fn check_time_builder_with_fractional_second() {
            let nanos: i64 = kani::any();
            kani::assume(NanosAreWithinFractionalSecondRange::requires(nanos));
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let trimmed = amenable_ext::digit_bytes_to_string_unconditional(digits);
            let fractional = amenable_time::FractionalSecondDescriptor::new(trimmed);
            let time_builder = amenable_time::LocalTimeDescriptorBuilder::default()
                .hour(0)
                .minute(0)
                .second(0)
                .fractional_second(fractional);
            let _ = time_builder.build();
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_TIME_BUILDER_SYMBOLIC_HMS_WITH_FRACTIONAL_SECOND_SRC, {
        /// Diagnostic, one more layer up: hour/minute/second are now
        /// symbolic too, each narrowed via the stubbed `narrow_to_u8_
        /// in_range` -- still no `chrono` date/time construction at
        /// all. Isolates whether three more stubbed calls, combined
        /// with the fractional-second path, add cost.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        #[kani::stub_verified(amenable_ext::narrow_to_u8_in_range)]
        fn check_time_builder_symbolic_hms_with_fractional_second() {
            let nanos: i64 = kani::any();
            kani::assume(NanosAreWithinFractionalSecondRange::requires(nanos));
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            kani::assume(HourMinuteSecondAreEachValidForNarrowing::requires((
                hour, minute, second,
            )));
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let trimmed = amenable_ext::digit_bytes_to_string_unconditional(digits);
            let fractional = amenable_time::FractionalSecondDescriptor::new(trimmed);
            let time_builder = amenable_time::LocalTimeDescriptorBuilder::default()
                .hour(amenable_ext::narrow_to_u8_in_range(hour))
                .minute(amenable_ext::narrow_to_u8_in_range(minute))
                .second(amenable_ext::narrow_to_u8_in_range(second))
                .fractional_second(fractional);
            let _ = time_builder.build();
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FULL_DESCRIPTOR_WITH_FRACTIONAL_SECOND_SRC, {
        /// Diagnostic, the last layer before the real target: adds
        /// the calendar/date side (symbolic year/month/day, via the
        /// same stubbed `narrow_to_u8_in_range`) and wraps both
        /// halves in `LocalDateTimeDescriptorBuilder` -- still no real
        /// `chrono::NaiveDate`/`NaiveDateTime` construction anywhere.
        /// If this is cheap, the only remaining variable before the
        /// real target is the real `chrono` construction itself.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        #[kani::stub_verified(amenable_ext::narrow_to_u8_in_range)]
        fn check_full_descriptor_with_fractional_second() {
            let nanos: i64 = kani::any();
            kani::assume(NanosAreWithinFractionalSecondRange::requires(nanos));
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            kani::assume(MonthAndDayAreEachValidForNarrowing::requires((month, day)));
            kani::assume(HourMinuteSecondAreEachValidForNarrowing::requires((
                hour, minute, second,
            )));
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let trimmed = amenable_ext::digit_bytes_to_string_unconditional(digits);
            let fractional = amenable_time::FractionalSecondDescriptor::new(trimmed);
            let calendar = amenable_time::CalendarDateDescriptor::new(
                year,
                amenable_ext::narrow_to_u8_in_range(month),
                amenable_ext::narrow_to_u8_in_range(day),
            );
            let time = amenable_time::LocalTimeDescriptorBuilder::default()
                .hour(amenable_ext::narrow_to_u8_in_range(hour))
                .minute(amenable_ext::narrow_to_u8_in_range(minute))
                .second(amenable_ext::narrow_to_u8_in_range(second))
                .fractional_second(fractional)
                .build()
                .expect("hour/minute/second always set");
            let _ = amenable_time::LocalDateTimeDescriptorBuilder::default()
                .date(amenable_time::CompleteDateDescriptor::Calendar(calendar))
                .time(time)
                .build();
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FULL_DESCRIPTOR_WITH_REAL_CHRONO_AND_FRACTIONAL_SECOND_SRC, {
        /// Diagnostic: the one missing piece from the previous layer
        /// -- a real `chrono::NaiveDate`/`NaiveTime`/`NaiveDateTime`,
        /// built from symbolic fields via `from_ymd_opt`/`from_hms_
        /// nano_opt`, with the fractional-second path exercised using
        /// the real datetime's own `.nanosecond()` (always > 0, via
        /// the `nano` field's own range). Isolates whether combining
        /// real `chrono` construction with the fractional-second path
        /// specifically -- never tested together before, since the
        /// top-level `check_offset_date_time_round_trips_through_
        /// descriptor` uses `and_hms_opt` with no nanosecond field at
        /// all -- is the piece still missing.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        #[kani::stub_verified(amenable_ext::narrow_to_u8_in_range)]
        fn check_full_descriptor_with_real_chrono_and_fractional_second() {
            use chrono::{Datelike, Timelike};
            use crate::{ValueIsAbove, ValueIsBelow};

            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return;
            };
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let nano: u32 = kani::any();
            kani::assume(ValueIsAbove::requires((nano, 0)));
            let Some(time_of_day) = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano)
            else {
                return;
            };
            let datetime = chrono::NaiveDateTime::new(date, time_of_day);
            kani::assume(ValueIsBelow::requires((datetime.nanosecond(), 1_000_000_000)));

            let nanos = i64::from(datetime.nanosecond());
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let trimmed = amenable_ext::digit_bytes_to_string_unconditional(digits);
            let fractional = amenable_time::FractionalSecondDescriptor::new(trimmed);
            let calendar = amenable_time::CalendarDateDescriptor::new(
                datetime.year(),
                amenable_ext::narrow_to_u8_in_range(datetime.month()),
                amenable_ext::narrow_to_u8_in_range(datetime.day()),
            );
            let time = amenable_time::LocalTimeDescriptorBuilder::default()
                .hour(amenable_ext::narrow_to_u8_in_range(datetime.hour()))
                .minute(amenable_ext::narrow_to_u8_in_range(datetime.minute()))
                .second(amenable_ext::narrow_to_u8_in_range(datetime.second()))
                .fractional_second(fractional)
                .build()
                .expect("hour/minute/second always set");
            let _ = amenable_time::LocalDateTimeDescriptorBuilder::default()
                .date(amenable_time::CompleteDateDescriptor::Calendar(calendar))
                .time(time)
                .build();
        }
    }
}
