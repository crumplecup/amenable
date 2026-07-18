//! `RustStdType` registrations for `std::path`.

use std::path::{
    Ancestors, Component, Components, Display, Iter, Path, PathBuf, Prefix, PrefixComponent,
    StripPrefixError,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type_lifetime0!(
    Ancestors,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.Ancestors.html",
    "The Ancestors carrier lazily yields a Path and each of its parent directories in turn."
);

impl_rust_std_type_lifetime0!(
    Component,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/enum.Component.html",
    "The Component carrier names one segment of a Path: a root, prefix, or named component."
);

impl_rust_std_type_lifetime0!(
    Components,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.Components.html",
    "The Components carrier lazily yields the individual components of a Path."
);

impl_rust_std_type_lifetime0!(
    Display,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.Display.html",
    "The Display carrier renders a Path for user-facing output, lossily substituting invalid Unicode where necessary."
);

impl_rust_std_type_lifetime0!(
    Iter,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.Iter.html",
    "The Iter carrier lazily yields the components of a Path as OsStr segments."
);

impl_rust_std_type!(
    Path,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.Path.html",
    "The Path carrier borrows a slice of a filesystem path, without taking ownership of it."
);

impl_rust_std_type!(
    PathBuf,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.PathBuf.html",
    "The PathBuf carrier owns a mutable, growable filesystem path."
);

impl_rust_std_type_lifetime0!(
    Prefix,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/enum.Prefix.html",
    "The Prefix carrier names a Windows path prefix, such as a drive letter or UNC share."
);

impl_rust_std_type_lifetime0!(
    PrefixComponent,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.PrefixComponent.html",
    "The PrefixComponent carrier pairs a parsed Windows Prefix with its raw OsStr representation."
);

impl_rust_std_type!(
    StripPrefixError,
    "std",
    "std::path",
    "https://doc.rust-lang.org/std/path/struct.StripPrefixError.html",
    "The StripPrefixError carrier reports that a Path did not start with the given prefix."
);

// `'static` is the representative lifetime this module's proof batch
// covers. `Prefix`/`PrefixComponent` are only meaningfully exercised on
// Windows targets, since Unix path parsing never produces a prefix
// component.
register_rust_std_standard_evidence!(
    Ancestors<'static>,
    Component<'static>,
    Components<'static>,
    Display<'static>,
    Iter<'static>,
    Path,
    PathBuf,
    Prefix<'static>,
    PrefixComponent<'static>,
    StripPrefixError,
);
