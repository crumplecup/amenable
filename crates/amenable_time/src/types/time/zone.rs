//! UTC-offset descriptors.
//!
//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

use crate::LocalTimeDescriptor;

/// The sign of a numeric UTC offset.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    EnumIter,
    derive_more::Display,
)]
pub enum UtcOffsetSign {
    /// Positive or east-of-UTC offset.
    #[default]
    #[display("+")]
    Positive,
    /// Negative or west-of-UTC offset.
    #[display("-")]
    Negative,
}

// No new Cargo dependency: `kani` is available as an implicit crate name
// under the Kani compiler regardless of `Cargo.toml`, the same way this
// workspace already gates `tracing::instrument` calls behind `cfg(kani)`
// elsewhere -- needed so Kani proofs that `#[kani::stub_verified]` a
// function returning this type (or a tuple containing it) can generate a
// nondeterministic replacement value for it.
#[cfg(kani)]
impl kani::Arbitrary for UtcOffsetSign {
    fn any() -> Self {
        if kani::any::<bool>() {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

/// The semantic interpretation of a UTC-relationship payload in an
/// offset-aware timestamp.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    EnumIter,
    derive_more::Display,
)]
pub enum UtcOffsetRelationship {
    /// The local offset is known.
    #[default]
    #[display("known")]
    Known,
    /// RFC 9557-updated RFC 3339 unknown-local-offset semantics.
    #[display("unknown-local-offset")]
    UnknownLocalOffset,
}
/// A UTC-offset relationship descriptor for an offset-aware timestamp.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    Builder,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UtcOffsetDescriptor {
    /// Offset sign when the UTC relationship is carried numerically.
    #[getter(copy)]
    sign: UtcOffsetSign,
    /// Absolute hour component for numeric-offset forms.
    #[getter(copy)]
    hours: u8,
    /// Absolute minute component when the offset is represented with
    /// hour-minute precision. `None` denotes the integral-hour form
    /// permitted by ISO 8601.
    #[builder(default)]
    #[getter(copy)]
    minutes: Option<u8>,
    /// Absolute seconds-within-the-minute component, present only
    /// for real backend values with sub-minute precision -- `None`
    /// for the ordinary, spec-shaped case.
    ///
    /// ISO/WD 8601-1:2016(E) section 4.2.5.1 states a UTC offset
    /// "carries a sign, an hour component, and optionally a minute
    /// component" -- a floor on the required shape, not a ceiling on
    /// what the type may also carry. A real backend offset (e.g.
    /// `chrono::FixedOffset`) can have sub-minute precision -- real,
    /// not hypothetical: chrono-tz's own historical zone data has
    /// 1952 real `(zone, year)` combinations with a genuine non-zero
    /// seconds remainder, pre-dating timezone standardization (e.g.
    /// `Africa/Abidjan`, 1800: a real Local Mean Time offset of
    /// `-968s` = `-0:16:08`). This field exists so those real values
    /// round-trip losslessly rather than being silently truncated or
    /// rejected, the same way `minutes` already exists for
    /// hour-only vs. hour-minute precision.
    ///
    /// No ISO 8601 (or RFC 3339/9557) textual form defines how to
    /// render a seconds-bearing UTC offset -- a real, open gap in
    /// this crate's own parse/format paths, not yet addressed. A
    /// descriptor built directly (as the chrono instant bridge
    /// does) is unaffected; only serialization of a `Some` value
    /// would need to resolve this.
    #[builder(default)]
    #[getter(copy)]
    seconds: Option<u8>,
    /// Whether the relationship is a known local offset or the RFC
    /// 9557-updated unknown-offset case.
    #[builder(default)]
    #[getter(copy)]
    relationship: UtcOffsetRelationship,
}
/// A descriptor for the UTC reference time scale.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum UtcTimeScaleDescriptor {
    /// Coordinated Universal Time.
    #[display("UTC")]
    #[default]
    Utc,
}
/// A UTC-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new, Default)]
pub struct UtcOfDayDescriptor {
    /// Time-of-day payload carried on the UTC time scale.
    time: LocalTimeDescriptor,
    /// Explicit UTC time-scale identity.
    #[getter(copy)]
    scale: UtcTimeScaleDescriptor,
}
