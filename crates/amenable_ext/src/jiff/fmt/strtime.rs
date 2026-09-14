//! `ExtType` registration for [`jiff::fmt::strtime`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::fmt::strtime::BrokenDownTime,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.BrokenDownTime.html",
    "The \"broken down time\" used by strptime parsing and strftime \
     formatting: parsing writes individually parsed fields here before \
     converting them to datetime types, and formatting converts a \
     datetime type here before reading fields back out."
);

impl_ext_type!(
    jiff::fmt::strtime::Config<jiff::fmt::strtime::DefaultCustom>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.Config.html",
    "Configuration for customizing the behavior of strftime formatting \
     or strptime parsing, generic over a Custom locale implementation \
     (registered here at the default, DefaultCustom)."
);

impl_ext_type!(
    jiff::fmt::strtime::DefaultCustom,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.DefaultCustom.html",
    "The default Custom trait implementation used by strftime/strptime, \
     matching Jiff's own built-in (non-POSIX) conventions."
);

impl_ext_type!(
    jiff::fmt::strtime::Display<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.Display.html",
    "A lazy implementation of std::fmt::Display for strftime, returned \
     by methods like Zoned::strftime, that performs the actual \
     formatting work only when written to a formatter."
);

impl_ext_type!(
    jiff::fmt::strtime::Extension,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.Extension.html",
    "Represents which flags and/or padding were provided with a \
     strftime/strptime conversion specifier, e.g. the 3 in %_3d."
);

impl_ext_type!(
    jiff::fmt::strtime::Meridiem,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/enum.Meridiem.html",
    "A label to disambiguate hours on a 12-hour clock (AM or PM), used \
     by strftime/strptime's %p-family conversion specifiers."
);

impl_ext_type!(
    jiff::fmt::strtime::PosixCustom,
    "jiff contributors",
    "jiff",
    "jiff::fmt::strtime",
    "https://docs.rs/jiff/latest/jiff/fmt/strtime/struct.PosixCustom.html",
    "A POSIX locale Custom trait implementation used by strftime/strptime, \
     for output matching the C locale rather than Jiff's own defaults."
);

register_ext_standard_evidence!(
    jiff::fmt::strtime::BrokenDownTime,
    jiff::fmt::strtime::Config<jiff::fmt::strtime::DefaultCustom>,
    jiff::fmt::strtime::DefaultCustom,
    jiff::fmt::strtime::Display<'static>,
    jiff::fmt::strtime::Extension,
    jiff::fmt::strtime::Meridiem,
    jiff::fmt::strtime::PosixCustom,
);
