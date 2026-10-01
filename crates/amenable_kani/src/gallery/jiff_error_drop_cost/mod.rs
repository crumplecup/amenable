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
//!
//! **A sixth case (found assessing `jiff::civil::DateTimeSeries` as a
//! witness candidate): the identical shape to `DateSeries`'s, checked
//! again rather than assumed.** `civil::DateTime` is, like `civil::
//! Date`, a pure calendar+clock value with no `TimeZone` at all — so
//! the fourth case's `Repr` wall cannot apply here either.
//! `DateTimeSeries::next()` has the exact same `checked_mul`/
//! `checked_add` shape as `DateSeries::next()`, hitting the identical
//! `jiff::Error` Drop-glue wall. Isolated the same way: a bare
//! `Date::new(year, month, day).at(0, 0, 0, 0)` call (cheap: `Date::
//! at` is a `const fn`, no fallible step) passes instantly; adding
//! `.series(period).next()` times out even with the result
//! immediately `mem::forget`-ed.
//!
//! **A seventh case (found assessing `jiff::civil::TimeSeries` as a
//! witness candidate): the identical shape to `DateSeries`'s/
//! `DateTimeSeries`'s, confirmed again rather than assumed.**
//! `civil::Time` is, like `civil::Date`/`civil::DateTime`, a pure
//! clock value with no `TimeZone` at all — so the fourth case's
//! `Repr` wall cannot apply here either. `TimeSeries::next()` has the
//! exact same `checked_mul`/`checked_add` shape as `DateSeries::
//! next()`/`DateTimeSeries::next()`, hitting the identical
//! `jiff::Error` Drop-glue wall. Isolated the same way: a bare
//! `Time::new(hour, minute, second, subsec_nanosecond)` call passes
//! instantly; adding `.series(period).next()` times out even with the
//! result immediately `mem::forget`-ed.
//!
//! Split along the investigation's own two distinct root causes, not
//! case-discovery order: [`drop_glue_wall`] (cases one, two, three,
//! five, six, and seven — all the same recursive-`Arc` `jiff::Error`
//! Drop-glue wall, reached through six different, unrelated call
//! sites) and [`timezone_pointer_tagging_wall`] (case four alone — a
//! genuinely unrelated root cause, `TimeZone`'s pointer-tagged `Repr`,
//! that only happens to have been found while chasing the same family
//! of witness).

mod drop_glue_wall;
mod timezone_pointer_tagging_wall;
