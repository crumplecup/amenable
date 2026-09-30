//! A real jiff-backed `amenable_time` temporal backend.
//!
//! Lives here, not in `amenable_time` itself, for the same reason
//! `amenable_std::std_time_backend` does — see that module's own doc
//! comment: `amenable_time` is the trait/contract interface crate, kept
//! dependency-light; backend implementations live alongside the type
//! registrations they're built from. The `Exchange` impls are
//! `impl ForeignTrait for LocalType`, which the orphan rule allows here.
//!
//! Unlike `StdTimeBackend` (a deliberate **canary**, honestly scoped to
//! the slice `std::time` can back), `JiffTimeBackend` is a real, if
//! partial, backend: its `Exchange` bodies call jiff's actual calendar
//! arithmetic and parser code, not a hand-rolled stand-in. See
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md` for the full ~129-edge
//! surface map and the phased checklist this module works through.
//!
//! Split one file per phase (plus `types`, holding every real local
//! carrier type the phases below build on, and `trusted_witness`, the
//! one machine-checkable structural contract):
//!
//! **Phase 1** (`duration`): `TemporalDurationProps` +
//! `TemporalDurationNativeBridge` over `jiff::Span`. **Phase 2**
//! (`instant`): `TemporalInstantProps` + `TemporalInstantNativeBridge`
//! over `jiff::Timestamp`/`jiff::tz::Offset`/a small `JiffOffsetDateTime`
//! composite — also defines those three types directly, alongside its
//! own bridge. **Phase 3** (`types`): `TemporalCivilProps` +
//! `TemporalCivilNativeBridge` over
//! `jiff::civil::{Date,Time,DateTime,ISOWeekDate}` — widens Phase 2's
//! own calendar-date-only `LocalDateTime` realize/reflect to real
//! ordinal- and week-date support too. **Phase 4** (`zone_conversions`):
//! `TemporalZoneProps` plus `TemporalZoneNativeBridge` over
//! `jiff::tz::TimeZone`/`jiff::Zoned`, real IANA tzdb lookups and
//! zoned-instant construction, the biggest genuine capability jump over
//! the `std::time` canary, which can't touch named zones at all.
//! **Phase 4b** (`zone_factory`): `TemporalZoneFactory` plus
//! `TemporalNativeZoneFactory`, the higher-order zone-resolution
//! factory, including real ambiguity (fold) and gap disambiguation and
//! a genuine offset/named-zone consistency check Phase 4's own native
//! bridge never performed. **Phase 5** (`conversion`):
//! `TemporalConversionFactory` plus `TemporalNativeConversionFactory` —
//! real UTC normalization, real named-zone stripping (reusing Phase
//! 4b's own now-consistency-checked zoned conversion), and real, honest
//! lossless-vs-lossy sub-second precision adjustment. **Phase 6**
//! (`reporter`): `TemporalReporter`'s real capability declaration, every
//! flag checked against jiff's own real source or docs rather than
//! assumed from the canary's own values. **Phase 7** (`interval`):
//! `TemporalIntervalFactory` — real duration parsing and real
//! `order_offset_endpoints` arithmetic; the two full interval-text-parse
//! edges are an honest `Unsupported` for now, since they need
//! `TemporalParser` (Phase 9), not yet built. **Phase 8**
//! (`time_interval`, with its own carrier types in `types`):
//! `TemporalTimeIntervalProps` + `TemporalRecurringIntervalProps` plus
//! their `NativeBridge`s, and `TemporalNativeIntervalFactory`'s own
//! `order_offset_endpoints_native` edge — real for every
//! jiff-representable endpoint form, honestly `Unsupported` for the
//! CalConnect/ISO 8601-2 extension family. **Phase 9**
//! (`parser_helpers`/`parser_exchanges`): `TemporalParser` — 13 of its
//! 24 edges are real (jiff `FromStr` where it exists, hand-rolled digit
//! splitting for the ordinal/week/reduced-precision/UTC-offset forms
//! jiff's own parser doesn't accept as text, and jiff's real `Pieces`
//! decomposition for offset/RFC 3339/IXDTF timestamps); the other 11
//! (the CalConnect/ISO 8601-2 extension family, plus `TimeInterval` per
//! Phase 7's own finding) are a real, honest `Unsupported`. **Phase 10**
//! (`formatter_helpers`/`formatter_exchanges`): `TemporalFormatter` — 24
//! of its 36 edges are real (every `ParsedX` input already carries
//! validated fields, so formatting is pure text construction, not fresh
//! jiff calls, except where jiff's own `Span: Display` backs
//! `Duration`); the other 12 (the same CalConnect/ISO 8601-2 extension
//! family) are a real, honest `Unsupported`. Every other
//! `Temporal*Props`/`NativeBridge`/`Factory` family named in the plan
//! doc's checklist lands in later commits, each widening this same
//! `JiffTimeBackend` struct with its own real `Exchange` impls.

mod conversion;
mod duration;
mod formatter_exchanges;
mod formatter_helpers;
mod instant;
mod interval;
mod parser_exchanges;
mod parser_helpers;
mod reporter;
mod time_interval;
mod trusted_witness;
mod types;
mod zone_conversions;
mod zone_factory;

pub use instant::{JiffOffset, JiffOffsetDateTime, JiffTimestamp};
pub use types::{
    JiffDate, JiffDateTime, JiffISOWeekDate, JiffRecurringInterval, JiffReducedCalendarDate,
    JiffReducedLocalTime, JiffSpan, JiffTime, JiffTimeBackend, JiffTimeInterval,
    JiffTimeIntervalEndpoint, JiffTimeIntervalRepresentation, JiffTimeZone, JiffVerifier,
    JiffVerifierMetadata, JiffZoned,
};
