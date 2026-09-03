//! `KaniWitness` impls for `std::path`, split by path abstraction:
//! `components` (the `Ancestors`/`Component`/`Components`/`Iter`
//! traversal iterators), `path_buf` (`Path`/`PathBuf`/`StripPrefixError`),
//! `display` (`Path::display()`, via an Amenable-owned observation), and
//! `windows_prefix` (`Prefix`/`PrefixComponent`, likewise).
//!
//! Every direct std harness uses forward-slash paths, which parse
//! identically on Unix and Windows. `display` and `windows_prefix` are
//! proved through Amenable-owned observations instead of the direct std
//! paths: `Display` times out under Kani's formatting machinery, and
//! Windows prefix parsing is host-platform-specific and not executable on
//! this Linux verifier host.

mod components;
mod display;
mod path_buf;
mod windows_prefix;
