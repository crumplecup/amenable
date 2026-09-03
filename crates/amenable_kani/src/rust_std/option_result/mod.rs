//! `KaniWitness` impls for `core::option` and `core::result`, split by the
//! source abstraction instead of keeping both families in one file.

mod option;
mod result;

pub use option::{OptionIsNoneReportsTrue, OptionIsSomeReportsTrue};
pub use result::{FallibleOperationReportsFailure, FallibleOperationReportsSuccess};
