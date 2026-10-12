//! The committed, discovered gap-transition table for chrono-tz's
//! real zone data -- generated once via a real, black-box scan
//! (hourly grid, 1970-01-01..2100-01-01, each detected 1-hour window
//! refined to a one-second-precise UTC boundary via binary search) of
//! every real `Tz` variant's public API, finding every distinct
//! forward-jump (base, dst) -> (base, dst) transition -- a real gap
//! -- any real zone produces. 27,693 real occurrences collapse to
//! exactly 94 distinct transitions, the same kind of collapse
//! `DISCOVERED_OFFSET_TABLE` already found for plain offset values.
//! Each witness was confirmed directly against the real
//! `TimeZone::from_local_datetime` (returns `LocalResult::None` for
//! all 94, not assumed).
//!
//! Each row is `(TZ_VARIANTS index, year, month, day, hour, minute,
//! second, prev_base_utc_offset seconds, prev_dst_offset seconds)` --
//! the witness local date-time sits inside the real gap, and the
//! transition's `next` offset pair is recorded in the sibling
//! comment for each row, not a separate field (it is not needed by
//! the claim itself, only the local witness and the pre-transition
//! offset the real `GapInfo::begin` field is expected to report).

/// One discovered gap-transition witness row.
pub type DiscoveredGapTableRow = (usize, i32, u32, u32, u32, u32, u32, i64, i64);

/// The full discovered table, 94 rows, one per distinct real gap
/// transition any real chrono-tz zone produces.
pub const DISCOVERED_GAP_TABLE: &[DiscoveredGapTableRow] = &[
    (536, 1979, 10, 1, 0, 30, 0, -43200, 0), // base -43200 dst 0 -> base -39600 dst 0
    (516, 1993, 8, 21, 12, 0, 0, -43200, 0), // base -43200 dst 0 -> base 43200 dst 0
    (54, 1970, 4, 26, 2, 30, 0, -39600, 0),  // base -39600 dst 0 -> base -39600 dst 3600
    (536, 1994, 12, 31, 12, 0, 0, -39600, 0), // base -39600 dst 0 -> base 46800 dst 0
    (172, 1983, 10, 30, 2, 30, 0, -39600, 3600), // base -39600 dst 3600 -> base -32400 dst 0
    (529, 2011, 12, 30, 12, 0, 0, -39600, 3600), // base -39600 dst 3600 -> base 46800 dst 3600
    (547, 1979, 10, 1, 0, 20, 0, -38400, 0), // base -38400 dst 0 -> base -36000 dst 0
    (563, 1978, 11, 12, 0, 30, 0, -37800, 0), // base -37800 dst 0 -> base -36000 dst 1800
    (563, 1979, 10, 28, 0, 15, 0, -36000, 0), // base -36000 dst 0 -> base -36000 dst 1800
    (54, 1984, 4, 29, 2, 30, 0, -36000, 0),  // base -36000 dst 0 -> base -36000 dst 3600
    (547, 1994, 12, 31, 12, 0, 0, -36000, 0), // base -36000 dst 0 -> base 50400 dst 0
    (55, 1984, 4, 29, 2, 30, 0, -32400, 0),  // base -32400 dst 0 -> base -32400 dst 3600
    (104, 1973, 10, 28, 0, 30, 0, -32400, 0), // base -32400 dst 0 -> base -28800 dst 0
    (559, 1998, 4, 27, 0, 15, 0, -30600, 0), // base -30600 dst 0 -> base -28800 dst 0
    (104, 1980, 4, 27, 2, 30, 0, -28800, 0), // base -28800 dst 0 -> base -28800 dst 3600
    (113, 2015, 3, 8, 2, 30, 0, -28800, 0),  // base -28800 dst 0 -> base -25200 dst 0
    (137, 1979, 4, 29, 3, 0, 0, -28800, 0),  // base -28800 dst 0 -> base -25200 dst 3600
    (77, 1996, 4, 7, 2, 30, 0, -25200, 0),   // base -25200 dst 0 -> base -25200 dst 3600
    (210, 1972, 4, 30, 2, 30, 0, -25200, 0), // base -25200 dst 0 -> base -21600 dst 0
    (77, 2010, 4, 4, 3, 0, 0, -25200, 0),    // base -25200 dst 0 -> base -21600 dst 3600
    (77, 2011, 4, 3, 2, 30, 0, -21600, 0),   // base -21600 dst 0 -> base -21600 dst 3600
    (88, 1981, 12, 26, 2, 30, 0, -21600, 0), // base -21600 dst 0 -> base -18000 dst 0
    (135, 2007, 3, 11, 3, 0, 0, -21600, 0),  // base -21600 dst 0 -> base -18000 dst 3600
    (199, 1970, 10, 25, 0, 15, 0, -18000, 0), // base -18000 dst 0 -> base -18000 dst 1800
    (83, 1992, 5, 3, 0, 30, 0, -18000, 0),   // base -18000 dst 0 -> base -18000 dst 3600
    (110, 2008, 6, 24, 0, 30, 0, -18000, 0), // base -18000 dst 0 -> base -14400 dst 0
    (89, 2016, 5, 1, 2, 45, 0, -16200, 0),   // base -16200 dst 0 -> base -14400 dst 0
    (63, 1990, 10, 28, 0, 30, 0, -14400, 0), // base -14400 dst 0 -> base -14400 dst 3600
    (118, 1988, 4, 3, 1, 1, 0, -14400, 0),   // base -14400 dst 0 -> base -14400 dst 7200
    (60, 2004, 6, 20, 0, 30, 0, -14400, 0),  // base -14400 dst 0 -> base -10800 dst 0
    (60, 1991, 10, 20, 1, 0, 0, -14400, 0),  // base -14400 dst 0 -> base -10800 dst 3600
    (124, 1975, 8, 1, 0, 22, 30, -13500, 0), // base -13500 dst 0 -> base -10800 dst 0
    (205, 1970, 4, 26, 2, 30, 0, -12600, 0), // base -12600 dst 0 -> base -12600 dst 3600
    (205, 1988, 4, 3, 1, 1, 0, -12600, 0),   // base -12600 dst 0 -> base -12600 dst 7200
    (181, 1984, 10, 1, 0, 15, 0, -12600, 0), // base -12600 dst 0 -> base -10800 dst 0
    (58, 1985, 11, 2, 0, 30, 0, -10800, 0),  // base -10800 dst 0 -> base -10800 dst 3600
    (166, 1974, 1, 13, 0, 45, 0, -10800, 0), // base -10800 dst 0 -> base -10800 dst 5400
    (117, 2023, 3, 25, 22, 30, 0, -10800, 0), // base -10800 dst 0 -> base -7200 dst 0
    (103, 1996, 1, 1, 1, 30, 0, -10800, 0),  // base -10800 dst 0 -> base 0 dst 0
    (117, 2024, 3, 30, 23, 30, 0, -7200, 0), // base -7200 dst 0 -> base -7200 dst 3600
    (338, 1975, 11, 25, 2, 30, 0, -7200, 0), // base -7200 dst 0 -> base -3600 dst 0
    (201, 1981, 3, 29, 1, 0, 0, -7200, 0),   // base -7200 dst 0 -> base -3600 dst 3600
    (201, 1982, 3, 28, 0, 30, 0, -3600, 0),  // base -3600 dst 0 -> base -3600 dst 3600
    (9, 1975, 1, 1, 0, 30, 0, -3600, 0),     // base -3600 dst 0 -> base 0 dst 0
    (42, 1972, 1, 7, 0, 22, 15, -2670, 0),   // base -2670 dst 0 -> base 0 dst 0
    (3, 1971, 4, 25, 23, 30, 0, 0, 0),       // base 0 dst 0 -> base 0 dst 3600
    (233, 2005, 3, 27, 2, 0, 0, 0, 0),       // base 0 dst 0 -> base 0 dst 7200
    (3, 1981, 5, 1, 0, 30, 0, 0, 0),         // base 0 dst 0 -> base 3600 dst 0
    (234, 1994, 11, 1, 3, 30, 0, 0, 0),      // base 0 dst 0 -> base 25200 dst 0
    (14, 2019, 6, 9, 2, 30, 0, 3600, -3600), // base 3600 dst -3600 -> base 3600 dst 0
    (3, 1978, 3, 24, 1, 30, 0, 3600, 0),     // base 3600 dst 0 -> base 3600 dst 3600
    (51, 1990, 5, 4, 0, 30, 0, 3600, 0),     // base 3600 dst 0 -> base 7200 dst 0
    (53, 1994, 9, 4, 2, 30, 0, 7200, -3600), // base 7200 dst -3600 -> base 7200 dst 0
    (13, 1970, 5, 1, 1, 30, 0, 7200, 0),     // base 7200 dst 0 -> base 7200 dst 3600
    (26, 2000, 1, 15, 12, 30, 0, 7200, 0),   // base 7200 dst 0 -> base 10800 dst 0
    (473, 2014, 3, 30, 3, 0, 0, 7200, 0),    // base 7200 dst 0 -> base 14400 dst 0
    (473, 1994, 5, 1, 0, 30, 0, 7200, 3600), // base 7200 dst 3600 -> base 10800 dst 3600
    (245, 1982, 5, 1, 0, 30, 0, 10800, 0),   // base 10800 dst 0 -> base 10800 dst 3600
    (317, 2005, 3, 27, 2, 30, 0, 10800, 0),  // base 10800 dst 0 -> base 14400 dst 0
    (318, 1977, 3, 21, 23, 30, 0, 12600, 0), // base 12600 dst 0 -> base 12600 dst 3600
    (240, 1995, 3, 26, 2, 30, 0, 14400, 0),  // base 14400 dst 0 -> base 14400 dst 3600
    (240, 1992, 1, 19, 2, 30, 0, 14400, 0),  // base 14400 dst 0 -> base 18000 dst 0
    (240, 1983, 4, 1, 0, 30, 0, 18000, 0),   // base 18000 dst 0 -> base 18000 dst 3600
    (237, 1992, 1, 19, 2, 30, 0, 18000, 0),  // base 18000 dst 0 -> base 21600 dst 0
    (224, 2010, 3, 11, 2, 0, 0, 18000, 0),   // base 18000 dst 0 -> base 25200 dst 0
    (281, 1986, 1, 1, 0, 7, 30, 19800, 0),   // base 19800 dst 0 -> base 20700 dst 0
    (320, 1987, 10, 1, 0, 15, 0, 19800, 0),  // base 19800 dst 0 -> base 21600 dst 0
    (258, 1996, 5, 25, 0, 30, 0, 19800, 0),  // base 19800 dst 0 -> base 23400 dst 0
    (237, 1981, 4, 1, 0, 30, 0, 21600, 0),   // base 21600 dst 0 -> base 21600 dst 3600
    (249, 1992, 1, 19, 2, 30, 0, 21600, 0),  // base 21600 dst 0 -> base 25200 dst 0
    (249, 1981, 4, 1, 0, 30, 0, 25200, 0),   // base 25200 dst 0 -> base 25200 dst 3600
    (255, 1978, 1, 1, 0, 30, 0, 25200, 0),   // base 25200 dst 0 -> base 28800 dst 0
    (286, 1981, 12, 31, 23, 45, 0, 27000, 0), // base 27000 dst 0 -> base 28800 dst 0
    (255, 1983, 4, 1, 0, 30, 0, 28800, 0),   // base 28800 dst 0 -> base 28800 dst 3600
    (254, 1992, 1, 19, 2, 30, 0, 28800, 0),  // base 28800 dst 0 -> base 32400 dst 0
    (223, 2009, 10, 18, 3, 30, 0, 28800, 0), // base 28800 dst 0 -> base 39600 dst 0
    (302, 2018, 5, 4, 23, 45, 0, 30600, 0),  // base 30600 dst 0 -> base 32400 dst 0
    (354, 1974, 10, 27, 2, 30, 0, 31500, 0), // base 31500 dst 0 -> base 31500 dst 3600
    (254, 1981, 4, 1, 0, 30, 0, 32400, 0),   // base 32400 dst 0 -> base 32400 dst 3600
    (254, 2011, 3, 27, 2, 30, 0, 32400, 0),  // base 32400 dst 0 -> base 36000 dst 0
    (328, 1981, 4, 1, 1, 30, 0, 32400, 0),   // base 32400 dst 0 -> base 39600 dst 3600
    (348, 1971, 10, 31, 2, 30, 0, 34200, 0), // base 34200 dst 0 -> base 34200 dst 3600
    (226, 1970, 10, 25, 2, 30, 0, 36000, 0), // base 36000 dst 0 -> base 36000 dst 3600
    (356, 1981, 3, 1, 0, 15, 0, 36000, 0),   // base 36000 dst 0 -> base 37800 dst 0
    (283, 2011, 3, 27, 2, 30, 0, 36000, 0),  // base 36000 dst 0 -> base 39600 dst 0
    (356, 1985, 10, 27, 2, 15, 0, 37800, 0), // base 37800 dst 0 -> base 37800 dst 1800
    (356, 1981, 10, 25, 2, 30, 0, 37800, 0), // base 37800 dst 0 -> base 37800 dst 3600
    (291, 1981, 4, 1, 0, 30, 0, 39600, 0),   // base 39600 dst 0 -> base 39600 dst 3600
    (239, 1992, 1, 19, 2, 30, 0, 39600, 0),  // base 39600 dst 0 -> base 43200 dst 0
    (555, 1974, 10, 27, 2, 30, 0, 41400, 0), // base 41400 dst 0 -> base 41400 dst 3600
    (553, 1979, 2, 10, 2, 15, 0, 41400, 0),  // base 41400 dst 0 -> base 43200 dst 0
    (228, 1974, 11, 3, 2, 30, 0, 43200, 0),  // base 43200 dst 0 -> base 43200 dst 3600
    (525, 1974, 11, 3, 3, 15, 0, 45900, 0),  // base 45900 dst 0 -> base 45900 dst 3600
    (239, 1981, 4, 1, 0, 30, 0, 46800, 0),   // base 46800 dst 0 -> base 46800 dst 3600
];
