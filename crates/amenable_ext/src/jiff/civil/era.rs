//! `ExtType` registration for [`jiff::civil::Era`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::Era,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/enum.Era.html",
    "The era corresponding to a particular year, either the Common Era \
     or Before the Common Era."
);

register_ext_standard_evidence!(jiff::civil::Era);
