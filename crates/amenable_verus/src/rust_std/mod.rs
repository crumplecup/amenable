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
