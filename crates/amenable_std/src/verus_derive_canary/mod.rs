//! Live derived-witness canaries for the Verus export pipeline.
//!
//! [`leaves`] holds the atomic building-block leaves; [`composites`]
//! builds and registers the composite structs/enums a real Verus derive-
//! witness composition run exercises.

mod composites;
mod leaves;

pub use composites::{VerusExportCanaryEnum, VerusExportMultiCheckedEnum};
pub use leaves::{
    CheckedVerusExportLeaf, RawTemplateVerusExportLeaf, RequiresVerusExportLeaf,
    TrustedVerusExportLeaf,
};
