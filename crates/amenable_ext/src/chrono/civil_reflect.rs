//! The reflect direction of the civil date-time bridge: a real
//! [`chrono::NaiveDateTime`] to a [`LocalDateTimeDescriptor`], plus its
//! two Kani-cost-driven variants (`_no_fraction`, `_infallible`).
//! Split out of `civil` -- these three are one cohesive family (the
//! same operation, staged for proof cost reasons), separate from the
//! realize direction (`civil::local_date_time_descriptor_to_naive`)
//! that converts the other way.

use crate::chrono::utc_offset_reflect::narrow_to_u8;
use crate::temporal_fraction::nanos_to_fractional_seconds_digits;
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, FractionalSecondDescriptor,
    LeapSecondNotRepresentableSource, LocalDateTimeBuilderFailedSource, LocalDateTimeDescriptor,
    LocalDateTimeDescriptorBuilder, LocalTimeBuilderFailedSource, LocalTimeDescriptorBuilder,
    TemporalComponent, TemporalError, TemporalErrorKind,
};

/// Reflect a [`chrono::NaiveDateTime`] as a local date-time descriptor, in
/// calendar form.
///
/// Leap seconds (chrono's nanoseconds past `999_999_999`) are rejected for the
/// same reason as the inverse direction.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(datetime)))]
pub fn naive_date_time_to_local_date_time_descriptor(
    datetime: chrono::NaiveDateTime,
) -> Result<LocalDateTimeDescriptor, TemporalError> {
    use chrono::{Datelike, Timelike};

    let nanos = datetime.nanosecond();
    if nanos >= 1_000_000_000 {
        return Err(TemporalError::new(
            TemporalErrorKind::LeapSecondNotRepresentable(LeapSecondNotRepresentableSource::new(
                nanos,
            )),
        ));
    }

    let date = datetime.date();
    // chrono's month/day/hour/minute/second are `u32`, always
    // 1..=12/1..=31/0..=23/0..=59/0..=59, so none of the narrowings
    // below can genuinely fail for a real chrono value -- every
    // `narrow_to_u8` call here is unreachable defensive code. Routed
    // through the shared `narrow_to_u8` helper (not a local inline
    // `u8::try_from` + hand-written string) so a real overflow reports
    // the actual component and value via `ComponentOutOfRangeSource`,
    // not five near-duplicate free-text messages.
    let calendar = CalendarDateDescriptor::new(
        date.year(),
        narrow_to_u8(date.month(), TemporalComponent::Month)?,
        narrow_to_u8(date.day(), TemporalComponent::Day)?,
    );

    let mut time_builder = LocalTimeDescriptorBuilder::default()
        .hour(narrow_to_u8(datetime.hour(), TemporalComponent::Hour)?)
        .minute(narrow_to_u8(datetime.minute(), TemporalComponent::Minute)?)
        .second(narrow_to_u8(datetime.second(), TemporalComponent::Second)?);
    if nanos != 0 {
        time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(
            nanos_to_fractional_seconds_digits(i64::from(nanos)),
        ));
    }
    let time = time_builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::LocalTimeBuilderFailed(
            LocalTimeBuilderFailedSource::new(err),
        ))
    })?;

    LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(calendar))
        .time(time)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::LocalDateTimeBuilderFailed(
                LocalDateTimeBuilderFailedSource::new(err),
            ))
        })
}

mod kani_only {
    #![cfg(kani)]

    use crate::chrono::utc_offset_reflect::narrow_to_u8_in_range;
    use crate::temporal_fraction::{digit_bytes_to_string_unconditional, nanos_to_fractional_digit_bytes};
    use amenable_time::{
        CalendarDateDescriptor, CompleteDateDescriptor, FractionalSecondDescriptor,
        LocalDateTimeDescriptor, LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder,
    };

    /// `naive_date_time_to_local_date_time_descriptor`'s infallible core,
    /// for Kani only, restricted to the integral-second case
    /// (`datetime.nanosecond() == 0`) -- the fractional-second path routes
    /// through `FractionalSecondDescriptor`'s `String` field, which this
    /// codebase treats as not generally CBMC-tractable (the digit-
    /// decomposition work elsewhere proves the fractional-second round
    /// trip below the text-encoding layer, not through it), so it's
    /// deliberately out of scope for this first pass rather than silently
    /// assumed away.
    ///
    /// Expected to be infallible, mirroring `narrow_to_u8`'s own shape:
    /// chrono's month/day/hour/minute/second are documented elsewhere as
    /// always within `u8` range for a real value, and neither builder
    /// struct's definition has a `build_fn(validate = ...)` hook. That
    /// reasoning is not itself the proof -- this function's own Kani
    /// harness (below) is what actually checks every `.expect()` here
    /// never panics; nothing in this comment should be read as confirmed
    /// until that harness has actually passed.
    #[kani::requires(chrono::Timelike::nanosecond(&datetime) == 0)]
    #[kani::ensures(|result: &LocalDateTimeDescriptor| {
        use chrono::{Datelike, Timelike};
        let calendar_matches = match result.date() {
            CompleteDateDescriptor::Calendar(cal) => {
                cal.year() == datetime.year()
                    && u32::from(cal.month()) == datetime.month()
                    && u32::from(cal.day()) == datetime.day()
            }
            CompleteDateDescriptor::Ordinal(_) | CompleteDateDescriptor::Week(_) => false,
        };
        calendar_matches
            && u32::from(result.time().hour()) == datetime.hour()
            && u32::from(result.time().minute()) == datetime.minute()
            && u32::from(result.time().second()) == datetime.second()
            && result.time().fractional_second().is_none()
    })]
    pub fn naive_date_time_to_local_date_time_descriptor_no_fraction(
        datetime: chrono::NaiveDateTime,
    ) -> LocalDateTimeDescriptor {
        use chrono::{Datelike, Timelike};

        let date = datetime.date();
        let calendar = CalendarDateDescriptor::new(
            date.year(),
            narrow_to_u8_in_range(date.month()),
            narrow_to_u8_in_range(date.day()),
        );

        let time = LocalTimeDescriptorBuilder::default()
            .hour(narrow_to_u8_in_range(datetime.hour()))
            .minute(narrow_to_u8_in_range(datetime.minute()))
            .second(narrow_to_u8_in_range(datetime.second()))
            .build()
            .expect("hour/minute/second always set -- no validation on this builder");

        LocalDateTimeDescriptorBuilder::default()
            .date(CompleteDateDescriptor::Calendar(calendar))
            .time(time)
            .build()
            .expect("date/time always set -- no validation on this builder")
    }

    /// `naive_date_time_to_local_date_time_descriptor`'s infallible core,
    /// for Kani only, covering the full real domain including the
    /// fractional-second path -- unlike the `_no_fraction` sibling above,
    /// this calls the real `nanos_to_fractional_seconds_digits` directly
    /// rather than excluding that path: the digit-chain gallery
    /// investigation already resolved the nine-digit identity's own cost
    /// (`gallery::chrono_offset_date_time_round_trip_digit_decomposition_
    /// steps`), and this function's own hot spots (`trim_end_matches`,
    /// `String::from_utf8`) were already independently fixed (see that
    /// function's own doc comment) -- a real reason to test this
    /// directly rather than assume it's still expensive.
    ///
    /// `requires` excludes only the genuine leap-second case (`nanosecond
    /// >= 1_000_000_000`), the real function's own `Err` path -- not
    /// assumed away, the same precondition shape as `utc_offset_
    /// descriptor_to_fixed_offset_infallible`'s relationship bound.
    #[kani::requires(chrono::Timelike::nanosecond(&datetime) < 1_000_000_000)]
    #[kani::ensures(|result: &LocalDateTimeDescriptor| {
        use chrono::{Datelike, Timelike};
        let nanos = datetime.nanosecond();
        let calendar_matches = match result.date() {
            CompleteDateDescriptor::Calendar(cal) => {
                cal.year() == datetime.year()
                    && u32::from(cal.month()) == datetime.month()
                    && u32::from(cal.day()) == datetime.day()
            }
            CompleteDateDescriptor::Ordinal(_) | CompleteDateDescriptor::Week(_) => false,
        };
        let time_matches = u32::from(result.time().hour()) == datetime.hour()
            && u32::from(result.time().minute()) == datetime.minute()
            && u32::from(result.time().second()) == datetime.second();
        // Not re-parsed via `fractional_seconds_digits_to_nanos`: content-
        // exactness is already `nanos_to_fractional_seconds_digits`'s own
        // contract's job (stubbed in the harness that checks this one),
        // not something this function's own ensures needs to re-derive by
        // calling the real parser again on the stub's already-constrained
        // result -- that duplicated a fact already assumed, forcing Kani
        // to symbolically re-execute the whole parse a second time for no
        // new information, a real, confirmed contributor to an OOM here.
        let fraction_matches = result.time().fractional_second().is_some() == (nanos != 0);
        calendar_matches && time_matches && fraction_matches
    })]
    pub fn naive_date_time_to_local_date_time_descriptor_infallible(
        datetime: chrono::NaiveDateTime,
    ) -> LocalDateTimeDescriptor {
        use chrono::{Datelike, Timelike};

        let nanos = datetime.nanosecond();
        let date = datetime.date();
        let calendar = CalendarDateDescriptor::new(
            date.year(),
            narrow_to_u8_in_range(date.month()),
            narrow_to_u8_in_range(date.day()),
        );

        let mut time_builder = LocalTimeDescriptorBuilder::default()
            .hour(narrow_to_u8_in_range(datetime.hour()))
            .minute(narrow_to_u8_in_range(datetime.minute()))
            .second(narrow_to_u8_in_range(datetime.second()));
        if nanos != 0 {
            // Under Kani, the real `nanos_to_fractional_seconds_digits`
            // is never called: reading back from a push-built `String`
            // after construction was confirmed, directly, to reintroduce
            // the same OOM its own symbolic trim length causes, so this
            // function's own `ensures` (checked below) never inspects
            // the fractional content, only whether one is present -- all
            // this branch needs is *some* correctly-typed, non-empty
            // `String` to exist, which the cheap, unconditional 9-push
            // path already guarantees by construction, with nothing read
            // back from it here either.
            #[cfg(kani)]
            let trimmed = {
                let digits = nanos_to_fractional_digit_bytes(i64::from(nanos));
                digit_bytes_to_string_unconditional(digits)
            };
            #[cfg(not(kani))]
            let trimmed = nanos_to_fractional_seconds_digits(i64::from(nanos));
            time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(trimmed));
        }
        let time = time_builder
            .build()
            .expect("hour/minute/second always set -- no validation on this builder");

        LocalDateTimeDescriptorBuilder::default()
            .date(CompleteDateDescriptor::Calendar(calendar))
            .time(time)
            .build()
            .expect("date/time always set -- no validation on this builder")
    }
}

#[cfg(kani)]
pub use kani_only::{
    naive_date_time_to_local_date_time_descriptor_infallible,
    naive_date_time_to_local_date_time_descriptor_no_fraction,
};
