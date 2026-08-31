//! One file per std carrier, each holding exactly the `verus! { ... }`
//! spec function(s) that carrier's real invariant needs — matching how
//! `elicitation_verus` groups its own proof files (`chars.rs`,
//! `durations.rs`, one file per type cluster), so `amenable_std::
//! verus_witness` can `include_str!` a single file as one type's whole
//! `claim`, the same one-claim-per-carrier granularity `amenable_kani`/
//! `amenable_creusot` get from `amenable_derive::harness!` capturing one
//! function at a time.
//!
//! Grouped into thematic subdirectories by real std module family
//! (mirroring `amenable_kani::rust_std`'s own directory shape) rather
//! than left as 133 files flat under one directory: [`io`], [`str_and_char`],
//! [`ffi`], [`collections`], [`sync`], [`cell_and_ref`], [`path_and_fs`],
//! [`process_and_net`], [`iter`], [`num`], [`task_and_thread`], and
//! [`misc`] (single-carrier std modules with no sibling in this crate --
//! `Box`, `Cow`, `Discriminant`, `Layout`, `ManuallyDrop`, a handful of
//! zero-invariant singletons, `ops`, `Option`, `Result`, `panic`,
//! primitive-shape carriers, `TypeId`, `env`, `fmt`, the Windows-handle
//! cluster, `SystemTime`). The one-file-per-carrier granularity inside
//! each subdirectory is unchanged -- this only adds a middle layer, it
//! never merges or renames a carrier file.
//!
//! Every leaf carrier's own `mod` declaration (in its group's `mod.rs`)
//! is private, with its real items re-exported explicitly via `pub use`
//! -- the standard `mod core; pub use core::{Type1, Type2};` idiom, not
//! a blanket `pub mod` per file. `pub` here isn't over-promising to an
//! external consumer this crate doesn't have (confirmed: `amenable_
//! verus` is never a Cargo dependency of anything); it's the mechanism
//! that keeps `cargo build`'s `dead_code` lint honest for the many
//! carriers whose only real "caller" is `verus`'s own traversal, not
//! any Rust call site. `amenable_core::verus_carrier::find_fn`'s own
//! path derivation (`module_path_for`) understands this: given a
//! function name, it walks up from the file that declares it and stops
//! climbing at the first level whose own `mod` is genuinely `pub`,
//! re-deriving the real, shorter reachable path automatically -- so
//! `amenable_std`'s `register_verus_call_shape!`/derived-witness codegen
//! never needs manual patching after a carrier moves or a group gets
//! reorganized. Spec fns (`open spec fn`, which Verus itself requires
//! stay `pub` unconditionally) and every already-`#[cfg(verus_keep_
//! ghost)]`-gated item (the `#[verifier::external_type_specification]`
//! markers) are re-exported under that same cfg, matching how every
//! existing cross-file reference to one already gates itself.

pub mod cell_and_ref;
pub mod collections;
pub mod ffi;
pub mod io;
pub mod iter;
pub mod misc;
pub mod num;
pub mod path_and_fs;
pub mod process_and_net;
pub mod str_and_char;
pub mod sync;
pub mod task_and_thread;
