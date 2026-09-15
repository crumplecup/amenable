//! `Witness<KaniVerifier>` registrations for `amenable_ext::ExtStandard<T>`
//! over jiff's registered carriers, split one file per real jiff area
//! that has earned a checked harness, plus the trusted carriers that
//! haven't (below).
//!
//! `Timestamp`/`Zoned`/`civil::DateTime` stay trusted here: they're
//! opaque, high-level composite types (calendar + zone + instant) with
//! no simple black-box property statable from jiff's public API alone —
//! see `docs/AMENABLE_EXT_PLAN.md`'s Phase 1 for why that set was chosen
//! first. Simpler value types get a real per-type assessment as they're
//! added (see `offset.rs` for the first checked example).
//!
//! `jiff::Error` also stays trusted, but for a different, real reason
//! confirmed empirically (not the default): every public accessor
//! (`Display`, `is_range`, `is_invalid_parameter`, `is_crate_feature`)
//! routes through `Error::chain()`'s iterator traversal, which times
//! out under Kani's unwinder even for a single, deterministic,
//! `mem::forget`-ed value — see `gallery::jiff_error_drop_cost`'s own
//! doc comment for the full isolation. There is no accessor-level
//! property left to check once every accessor hits the same wall.
//!
//! `jiff::RoundMode` stays trusted for a third, simpler real reason:
//! checked directly against jiff's real source, it has *no* public
//! methods at all beyond the standard derives (`Clone`/`Copy`/`Debug`/
//! `Eq`/`Hash`/`PartialEq`) — its actual rounding logic
//! (`round_by_duration`) is `pub(crate)`. There is nothing non-
//! tautological to state about a fieldless, behaviorless configuration
//! marker on any backend.
//!
//! `jiff::SignedDurationRound` also stays trusted, for the builder-
//! type shape of that same reason: its only public methods are
//! `new`/`smallest`/`mode`/`increment`, plain field setters returning a
//! new value via `..self` — no getters, and its actual rounding logic
//! (`round`, called from `SignedDuration::round`) is private. There is
//! no publicly-inspectable state to state a property about. The same
//! shape recurs across jiff's other `*Round`/`*Arithmetic`/`*Compare`/
//! `*Difference`/`*Total` builder-option types (confirmed for
//! `SpanRound` too, which has the identical setter-only surface) —
//! expect most of that family to land here for the same reason.
//!
//! `jiff::SpanArithmetic<'static>`/`jiff::SpanCompare<'static>` are the
//! first two of that family actually confirmed and landed: checked
//! directly against jiff's real source, each has exactly one public
//! method (`days_are_24_hours(self) -> Self`), no getters at all
//! (their fields are both private), and their own `checked_add`/
//! `compare`/`relative` are `pub(crate)`. Nothing publicly inspectable
//! to state a property about.
//!
//! `jiff::SpanFieldwise` gets a real checked property (see
//! `span_fieldwise.rs`): its own added value over `Span` is `Neg`/`Eq`/
//! `Hash`, and negating flips `Span`'s one shared `sign` field, which
//! every unit getter multiplies through — so negating a `SpanFieldwise`
//! negates every one of its ten unit getters simultaneously, checked
//! reusing `span.rs`'s own construction discipline.
//!
//! `jiff::SpanRelativeTo<'static>` stays trusted, even more opaque than
//! `SpanArithmetic`/`SpanCompare`: checked directly against jiff's real
//! source, its only public "method" is `days_are_24_hours() -> Self`, a
//! bare `pub const fn` static marker constructor (not even
//! `self`-consuming) — no getters, its one field (`kind`) is private,
//! and its own `to_relative` is private too. Nothing publicly
//! inspectable to state a property about.

mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;

use crate::ext::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(
    jiff::Timestamp,
    jiff::Zoned,
    jiff::civil::DateTime,
    jiff::Error,
    jiff::RoundMode,
    jiff::SignedDurationRound,
    jiff::SpanArithmetic<'static>,
    jiff::SpanCompare<'static>,
    jiff::SpanRelativeTo<'static>
);
