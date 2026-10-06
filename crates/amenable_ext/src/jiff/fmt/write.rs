//! `ExtType` registration for [`jiff::fmt`]'s `std::fmt`/`std::io`/`defmt`
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

impl_ext_type!(
    jiff::fmt::DefmtWrite<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt",
    "https://docs.rs/jiff/latest/jiff/fmt/struct.DefmtWrite.html",
    "An adapter for writing jiff output to a defmt::Formatter, so \
     formatted values reach the defmt logger."
);

register_ext_standard_evidence!(
    jiff::fmt::StdFmtWrite<String>,
    jiff::fmt::StdIoWrite<Vec<u8>>,
    jiff::fmt::DefmtWrite<'static>,
);
