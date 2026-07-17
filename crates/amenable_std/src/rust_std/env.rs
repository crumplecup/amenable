//! `RustStdType` registrations for `std::env`.

use std::env::{Args, ArgsOs, JoinPathsError, SplitPaths, VarError, Vars, VarsOs};

use crate::rust_std::macros::{impl_rust_std_type, impl_rust_std_type_lifetime0};

impl_rust_std_type!(
    Args,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.Args.html",
    "The Args carrier lazily yields the process's command-line arguments as Strings."
);

impl_rust_std_type!(
    ArgsOs,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.ArgsOs.html",
    "The ArgsOs carrier lazily yields the process's command-line arguments as OsStrings."
);

impl_rust_std_type!(
    JoinPathsError,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.JoinPathsError.html",
    "The JoinPathsError carrier reports that a set of paths could not be joined into a single PATH-style variable."
);

impl_rust_std_type_lifetime0!(
    SplitPaths,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.SplitPaths.html",
    "The SplitPaths carrier lazily yields the individual paths of a platform PATH-style variable."
);

impl_rust_std_type!(
    VarError,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/enum.VarError.html",
    "The VarError carrier reports why an environment variable could not be read: NotPresent or NotUnicode."
);

impl_rust_std_type!(
    Vars,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.Vars.html",
    "The Vars carrier lazily yields the process's environment variables as (String, String) pairs."
);

impl_rust_std_type!(
    VarsOs,
    "std",
    "std::env",
    "https://doc.rust-lang.org/std/env/struct.VarsOs.html",
    "The VarsOs carrier lazily yields the process's environment variables as (OsString, OsString) pairs."
);
