//! `ExtType` registration for [`jiff::fmt`]'s `std::fmt`/`std::io`
//! `Write` adapters.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::fmt::StdFmtWrite<String>,
    "jiff contributors",
    "jiff",
    "jiff::fmt",
    "https://docs.rs/jiff/latest/jiff/fmt/struct.StdFmtWrite.html",
    "An adapter for using a std::fmt::Write implementation (here, \
     String) with jiff::fmt::Write."
);

impl_ext_type!(
    jiff::fmt::StdIoWrite<Vec<u8>>,
    "jiff contributors",
    "jiff",
    "jiff::fmt",
    "https://docs.rs/jiff/latest/jiff/fmt/struct.StdIoWrite.html",
    "An adapter for using a std::io::Write implementation (here, \
     Vec<u8>) with jiff::fmt::Write."
);

register_ext_standard_evidence!(
    jiff::fmt::StdFmtWrite<String>,
    jiff::fmt::StdIoWrite<Vec<u8>>,
);
