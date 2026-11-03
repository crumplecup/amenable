//! `derive_builder`-generated builder-failure error sources: each
//! wraps the real generated `<Descriptor>BuilderError` as `source`
//! rather than stringifying it -- the builder's own error *is* the
//! genuine cause, so wrapping it natively is normalizing, not the
//! stringifying antipattern `InvalidDescriptorSource` used to commit.

use derive_getters::Getters;
use derive_more::{Display, Error};

use crate::{
    DurationDescriptorBuilderError, IxdtfTimestampDescriptorBuilderError,
    LocalDateTimeDescriptorBuilderError, LocalTimeDescriptorBuilderError,
    NamedTimeZoneDescriptorBuilderError, OffsetDateTimeDescriptorBuilderError,
    RecurringIntervalDescriptorBuilderError, UtcOffsetDescriptorBuilderError,
    ZonedDateTimeDescriptorBuilderError,
};

/// A `UtcOffsetDescriptorBuilder::build` validation failure, preserving
/// the real `derive_builder`-generated error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a UTC offset descriptor: {source}")]
pub struct UtcOffsetBuilderFailedSource {
    source: UtcOffsetDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl UtcOffsetBuilderFailedSource {
    /// Preserve a `UtcOffsetDescriptorBuilder` failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: UtcOffsetDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `LocalTimeDescriptorBuilder::build` validation failure, preserving
/// the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a local time descriptor: {source}")]
pub struct LocalTimeBuilderFailedSource {
    source: LocalTimeDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl LocalTimeBuilderFailedSource {
    /// Preserve a `LocalTimeDescriptorBuilder` failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: LocalTimeDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `LocalDateTimeDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a local date-time descriptor: {source}")]
pub struct LocalDateTimeBuilderFailedSource {
    source: LocalDateTimeDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl LocalDateTimeBuilderFailedSource {
    /// Preserve a `LocalDateTimeDescriptorBuilder` failure, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: LocalDateTimeDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// An `OffsetDateTimeDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build an offset date-time descriptor: {source}")]
pub struct OffsetDateTimeBuilderFailedSource {
    source: OffsetDateTimeDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl OffsetDateTimeBuilderFailedSource {
    /// Preserve an `OffsetDateTimeDescriptorBuilder` failure, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: OffsetDateTimeDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `NamedTimeZoneDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a named time-zone descriptor: {source}")]
pub struct NamedTimeZoneBuilderFailedSource {
    source: NamedTimeZoneDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl NamedTimeZoneBuilderFailedSource {
    /// Preserve a `NamedTimeZoneDescriptorBuilder` failure, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: NamedTimeZoneDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `ZonedDateTimeDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a zoned date-time descriptor: {source}")]
pub struct ZonedDateTimeBuilderFailedSource {
    source: ZonedDateTimeDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl ZonedDateTimeBuilderFailedSource {
    /// Preserve a `ZonedDateTimeDescriptorBuilder` failure, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: ZonedDateTimeDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `DurationDescriptorBuilder::build` validation failure, preserving
/// the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a duration descriptor: {source}")]
pub struct DurationBuilderFailedSource {
    source: DurationDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl DurationBuilderFailedSource {
    /// Preserve a `DurationDescriptorBuilder` failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: DurationDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// An `IxdtfTimestampDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build an IXDTF timestamp descriptor: {source}")]
pub struct IxdtfTimestampBuilderFailedSource {
    source: IxdtfTimestampDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl IxdtfTimestampBuilderFailedSource {
    /// Preserve an `IxdtfTimestampDescriptorBuilder` failure, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: IxdtfTimestampDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `RecurringIntervalDescriptorBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not build a recurring interval descriptor: {source}")]
pub struct RecurringIntervalBuilderFailedSource {
    source: RecurringIntervalDescriptorBuilderError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl RecurringIntervalBuilderFailedSource {
    /// Preserve a `RecurringIntervalDescriptorBuilder` failure,
    /// capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: RecurringIntervalDescriptorBuilderError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}
