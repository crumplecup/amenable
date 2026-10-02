use crate::JiffTimeBackend;
use amenable_time::{SerializationProfile, TemporalReporter};

// ── Reporter (Phase 6) ───────────────────────────────────────────────
//
// `TemporalReporter`'s real capability declaration. Every flag below
// is checked against jiff's own real source/docs, not assumed from the
// canary's own values:
//
// - `max_fractional_second_digits`: `Some(9)` -- jiff's civil time and
//   `Span` are both nanosecond-precision throughout (confirmed in
//   Phases 1-5's own real conversions).
// - `supports_leap_seconds`: `false` -- jiff's own docs state outright
//   "Jiff does not support leap seconds. Jiff behaves as if they don't
//   exist" (verbatim, `civil::DateTime`/`Timestamp`/`civil::Time`/
//   `Zoned`'s own doc comments).
// - `supports_unknown_local_offset`: `false` -- Phase 2's own
//   `realize_offset_date_time_rejects_the_unknown_local_offset_case`
//   test already confirmed `jiff::tz::Offset` has no representation
//   for RFC 9557's `-00:00` convention at all.
// - `supports_named_zone_round_trip`: `true` -- real IANA tzdb lookups
//   via `TimeZone::get`, exercised for real since Phase 4.
// - `supports_end_of_day_twenty_four`: `false` -- `jiff::civil::Time::MAX`
//   is `23:59:59.999999999`; jiff has no `24:00:00` representation.
// - `current_tzdb_revision`: `None` -- jiff exposes no public API to
//   query the linked tzdb's own revision string (checked its real
//   `tz::db` module directly in Phase 4); re-confirmed here, not
//   reassumed.
// - `supported_serialization_profiles`: empty, matching the canary --
//   an honest declaration, not a pessimistic one: no `TemporalParser`/
//   `TemporalFormatter` edge exists on this backend yet (Phases 9-10),
//   so no serialization profile is actually reachable through this
//   backend's own `Exchange` surface today, even though jiff's real
//   `fmt::temporal` module could back several of them once those
//   phases land.
impl TemporalReporter for JiffTimeBackend {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile> {
        Vec::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn max_fractional_second_digits(&self) -> Option<u8> {
        Some(9)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_leap_seconds(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_unknown_local_offset(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_named_zone_round_trip(&self) -> bool {
        true
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_end_of_day_twenty_four(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn current_tzdb_revision(&self) -> Option<String> {
        None
    }
}
