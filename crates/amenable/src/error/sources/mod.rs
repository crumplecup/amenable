//! Foreign error wrappers preserving the underlying `source` chain.
//!
//! Each wrapper holds exactly one foreign error in a field named `source`,
//! which `derive_more::Error` recognizes as the chain-preserving source
//! without any extra attribute. Every wrapper also carries its own owned
//! `file`/`line`, captured from `Location::caller()` by a `#[track_caller]
//! fn new` -- not `&'static Location`, so the wrapper stays `'static`-free
//! and comparable/clonable the same way the rest of this module's types
//! are, and not passed as constructor arguments, so the location can never
//! silently drift from the real call site the way a hand-typed `file!()`/
//! `line!()` pair at the wrong call frame could.
//!
//! Grouped by the foreign domain each wrapper preserves: [`io`] (IO and
//! JSON parsing), [`time`] (the `time` crate and the system clock),
//! [`numeric`] (integer-conversion failures), [`internal`] (business-rule
//! invariants and this workspace's own cross-crate umbrella errors).

mod internal;
mod io;
mod numeric;
mod time;

pub use internal::{ChainSource, InvariantSource, StdSource};
pub use io::{IoSource, JsonLineSource, SerdeSource};
pub use numeric::{
    AssessmentCountSource, InvalidScoreSource, PreEpochDateSource, TimestampTooLargeSource,
};
pub use time::{
    InvalidUtcDateSource, SystemTimeSource, TimeComponentRangeSource, TimeFormatDescriptionSource,
    TimeFormatSource, TimeParseSource,
};
