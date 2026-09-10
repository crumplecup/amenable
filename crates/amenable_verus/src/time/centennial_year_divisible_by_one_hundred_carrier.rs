//! Verus spec for `amenable_time::CentennialYearDivisibleByOneHundred`.
//!
//! ISO 8601-1:2019, 3.1.1.22 — a centennial year is one whose year number is an exact multiple of 100. The exec `y % 100 == 0` check matches the spec, and the dated
//! anchors (0/1900/2000 centennial, 2024 not) hold.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A centennial year: the year number is an exact multiple of 100.
pub open spec fn centennial_year_divisible_by_one_hundred_holds(year: i32) -> bool {
    year % 100 == 0
}

/// The exec check matches the spec, and the dated anchors hold.
pub fn verify_centennial_year_divisible_by_one_hundred(year: i32) -> (result: bool)
    ensures
        result == centennial_year_divisible_by_one_hundred_holds(year),
        centennial_year_divisible_by_one_hundred_holds(0),
        centennial_year_divisible_by_one_hundred_holds(1900),
        centennial_year_divisible_by_one_hundred_holds(2000),
        !centennial_year_divisible_by_one_hundred_holds(2024),
{
    year % 100 == 0
}

} // verus!
