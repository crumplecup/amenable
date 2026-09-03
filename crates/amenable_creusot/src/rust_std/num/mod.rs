//! `KaniWitness`-mirroring Creusot postconditions for `core::num`, split by
//! numeric topic: `nonzero` (`NonZero<i16>`), `wrapping_saturating`
//! (`Wrapping`/`Saturating` arithmetic), `parse` (integer parse/conversion
//! failures), and `float` (`FpCategory` / float parsing).
//!
//! Each `NonZero<T>` / `Wrapping` / `Saturating` instantiation and each
//! parse claim is written out literally rather than generated through a
//! wrapping `macro_rules!`: `amenable_derive::harness!` captures a harness's
//! verbatim source via the group's span, and a span produced by a
//! `macro_rules!` expansion resolves back to the *defining* macro's on-disk
//! text.

mod float;
mod nonzero;
mod parse;
mod wrapping_saturating;

pub use float::{
    FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_HOLDS_SRC,
    PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_HOLDS_SRC,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
    VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
};
pub use nonzero::{
    NONZERO_I16_GET_ROUND_TRIPS_SRC, NONZERO_I16_NEW_SUCCEEDS_EXACTLY_WHEN_NONZERO_SRC,
    VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
};
pub use parse::{
    INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_HOLDS_SRC,
    PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_HOLDS_SRC,
    TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_HOLDS_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
    VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
    VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
};
pub use wrapping_saturating::{
    SATURATING_I32_ADD_CLAMPS_HOLDS_SRC,
    VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, WRAPPING_I32_ADD_WRAPS_HOLDS_SRC,
};
