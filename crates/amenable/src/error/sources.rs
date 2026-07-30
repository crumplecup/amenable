//! Foreign error wrappers preserving the underlying `source` chain.
//!
//! Each wrapper holds exactly one foreign error in a field named `source`,
//! which `derive_more::Error` recognizes as the chain-preserving source
//! without any extra attribute.

/// Preserved `std::io::Error` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct IoSource {
    source: std::io::Error,
}

/// Preserved `serde_json::Error` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct SerdeSource {
    source: serde_json::Error,
}

/// Preserved `std::time::SystemTimeError` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct SystemTimeSource {
    source: std::time::SystemTimeError,
}

/// Preserved `time::error::InvalidFormatDescription` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct TimeFormatDescriptionSource {
    source: time::error::InvalidFormatDescription,
}

/// Preserved `time::error::Parse` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct TimeParseSource {
    source: time::error::Parse,
}

/// Preserved `time::error::ComponentRange` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct TimeComponentRangeSource {
    source: time::error::ComponentRange,
}

/// Preserved `time::error::Format` source.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
#[display("{source}")]
pub struct TimeFormatSource {
    source: time::error::Format,
}
