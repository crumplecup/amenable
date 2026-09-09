//! `Reflected<X>` sidecars — a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaSemanticBundleToken, DurationDescriptor,
    DurationSemanticBundleToken, ExplicitDurationDescriptor, ExplicitDurationSemanticBundleToken,
    ExplicitTemporalFormDescriptor, ExplicitTemporalFormSemanticBundleToken,
    ExplicitTimeIntervalDescriptor, ExplicitTimeIntervalSemanticBundleToken,
    GroupedTimeScaleUnitDescriptor, GroupedTimeScaleUnitSemanticBundleToken,
    LocalDateTimeDescriptor, LocalDateTimeSemanticBundleToken, NamedTimeZoneDescriptor,
    NamedTimeZoneSemanticBundleToken, OffsetDateTimeDescriptor, OffsetDateTimeSemanticBundleToken,
    QualifiedTemporalValueDescriptor, QualifiedTemporalValueSemanticBundleToken,
    RecurringIntervalDescriptor, RecurringIntervalSemanticBundleToken, TemporalSetDescriptor,
    TemporalSetSemanticBundleToken, TimeIntervalDescriptor, TimeIntervalSemanticBundleToken,
    ZonedDateTimeDescriptor, ZonedDateTimeSemanticBundleToken,
};

/// A neutral [`LocalDateTimeDescriptor`](crate::LocalDateTimeDescriptor) paired with a token for the
/// folded [`LocalDateTimeSemanticBundle`](crate::LocalDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::LocalDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedLocalDateTime {
    #[sidecar(primary)]
    descriptor: LocalDateTimeDescriptor,
    #[sidecar(token)]
    token: LocalDateTimeSemanticBundleToken,
}

impl ReflectedLocalDateTime {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &LocalDateTimeDescriptor {
        &self.descriptor
    }
}

/// A neutral [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) paired with a token for the
/// folded [`OffsetDateTimeSemanticBundle`](crate::OffsetDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OffsetDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedOffsetDateTime {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: OffsetDateTimeSemanticBundleToken,
}

impl ReflectedOffsetDateTime {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// A neutral [`NamedTimeZoneDescriptor`](crate::NamedTimeZoneDescriptor) paired with a token for the
/// folded [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::NamedTimeZoneSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedNamedTimeZone {
    #[sidecar(primary)]
    descriptor: NamedTimeZoneDescriptor,
    #[sidecar(token)]
    token: NamedTimeZoneSemanticBundleToken,
}

impl ReflectedNamedTimeZone {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &NamedTimeZoneDescriptor {
        &self.descriptor
    }
}

/// A neutral [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) paired with a token for the
/// folded [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ZonedDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedZonedDateTime {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: ZonedDateTimeSemanticBundleToken,
}

impl ReflectedZonedDateTime {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ZonedDateTimeDescriptor {
        &self.descriptor
    }
}

/// A neutral [`DurationDescriptor`](crate::DurationDescriptor) paired with a token for the
/// folded [`DurationSemanticBundle`](crate::DurationSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DurationSemanticBundle", constructor = "pub")]
pub struct ReflectedDuration {
    #[sidecar(primary)]
    descriptor: DurationDescriptor,
    #[sidecar(token)]
    token: DurationSemanticBundleToken,
}

impl ReflectedDuration {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DurationDescriptor {
        &self.descriptor
    }
}

/// A neutral [`TimeIntervalDescriptor`](crate::TimeIntervalDescriptor) paired with a token for the
/// folded [`TimeIntervalSemanticBundle`](crate::TimeIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TimeIntervalSemanticBundle", constructor = "pub")]
pub struct ReflectedTimeInterval {
    #[sidecar(primary)]
    descriptor: TimeIntervalDescriptor,
    #[sidecar(token)]
    token: TimeIntervalSemanticBundleToken,
}

impl ReflectedTimeInterval {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TimeIntervalDescriptor {
        &self.descriptor
    }
}

/// A neutral [`RecurringIntervalDescriptor`](crate::RecurringIntervalDescriptor) paired with a token for the
/// folded [`RecurringIntervalSemanticBundle`](crate::RecurringIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::RecurringIntervalSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedRecurringInterval {
    #[sidecar(primary)]
    descriptor: RecurringIntervalDescriptor,
    #[sidecar(token)]
    token: RecurringIntervalSemanticBundleToken,
}

impl ReflectedRecurringInterval {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &RecurringIntervalDescriptor {
        &self.descriptor
    }
}

/// A neutral [`QualifiedTemporalValueDescriptor`](crate::QualifiedTemporalValueDescriptor) paired with a token for the
/// folded [`QualifiedTemporalValueSemanticBundle`](crate::QualifiedTemporalValueSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::QualifiedTemporalValueSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedQualifiedTemporalValue {
    #[sidecar(primary)]
    descriptor: QualifiedTemporalValueDescriptor,
    #[sidecar(token)]
    token: QualifiedTemporalValueSemanticBundleToken,
}

impl ReflectedQualifiedTemporalValue {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &QualifiedTemporalValueDescriptor {
        &self.descriptor
    }
}

/// A neutral [`ExplicitTemporalFormDescriptor`](crate::ExplicitTemporalFormDescriptor) paired with a token for the
/// folded [`ExplicitTemporalFormSemanticBundle`](crate::ExplicitTemporalFormSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTemporalFormSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitTemporalForm {
    #[sidecar(primary)]
    descriptor: ExplicitTemporalFormDescriptor,
    #[sidecar(token)]
    token: ExplicitTemporalFormSemanticBundleToken,
}

impl ReflectedExplicitTemporalForm {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitTemporalFormDescriptor {
        &self.descriptor
    }
}

/// A neutral [`ExplicitDurationDescriptor`](crate::ExplicitDurationDescriptor) paired with a token for the
/// folded [`ExplicitDurationSemanticBundle`](crate::ExplicitDurationSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitDurationSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitDuration {
    #[sidecar(primary)]
    descriptor: ExplicitDurationDescriptor,
    #[sidecar(token)]
    token: ExplicitDurationSemanticBundleToken,
}

impl ReflectedExplicitDuration {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitDurationDescriptor {
        &self.descriptor
    }
}

/// A neutral [`ExplicitTimeIntervalDescriptor`](crate::ExplicitTimeIntervalDescriptor) paired with a token for the
/// folded [`ExplicitTimeIntervalSemanticBundle`](crate::ExplicitTimeIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTimeIntervalSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitTimeInterval {
    #[sidecar(primary)]
    descriptor: ExplicitTimeIntervalDescriptor,
    #[sidecar(token)]
    token: ExplicitTimeIntervalSemanticBundleToken,
}

impl ReflectedExplicitTimeInterval {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitTimeIntervalDescriptor {
        &self.descriptor
    }
}

/// A neutral [`GroupedTimeScaleUnitDescriptor`](crate::GroupedTimeScaleUnitDescriptor) paired with a token for the
/// folded [`GroupedTimeScaleUnitSemanticBundle`](crate::GroupedTimeScaleUnitSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::GroupedTimeScaleUnitSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedGroupedTimeScaleUnit {
    #[sidecar(primary)]
    descriptor: GroupedTimeScaleUnitDescriptor,
    #[sidecar(token)]
    token: GroupedTimeScaleUnitSemanticBundleToken,
}

impl ReflectedGroupedTimeScaleUnit {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &GroupedTimeScaleUnitDescriptor {
        &self.descriptor
    }
}

/// A neutral [`TemporalSetDescriptor`](crate::TemporalSetDescriptor) paired with a token for the
/// folded [`TemporalSetSemanticBundle`](crate::TemporalSetSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalSetSemanticBundle", constructor = "pub")]
pub struct ReflectedTemporalSet {
    #[sidecar(primary)]
    descriptor: TemporalSetDescriptor,
    #[sidecar(token)]
    token: TemporalSetSemanticBundleToken,
}

impl ReflectedTemporalSet {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TemporalSetDescriptor {
        &self.descriptor
    }
}

/// A neutral [`DateTimeFormulaDescriptor`](crate::DateTimeFormulaDescriptor) paired with a token for the
/// folded [`DateTimeFormulaSemanticBundle`](crate::DateTimeFormulaSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::DateTimeFormulaSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedDateTimeFormula {
    #[sidecar(primary)]
    descriptor: DateTimeFormulaDescriptor,
    #[sidecar(token)]
    token: DateTimeFormulaSemanticBundleToken,
}

impl ReflectedDateTimeFormula {
    /// Borrow the reflected descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DateTimeFormulaDescriptor {
        &self.descriptor
    }
}
