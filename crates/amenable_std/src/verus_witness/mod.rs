//! `VerusWitness` impls for Rust standard-library carriers.
//!
//! This lives here, not alongside the proof functions in `amenable_verus`,
//! for a stronger reason than `amenable_std::creusot_witness`'s: it's not
//! just that the witness/registry machinery is awkward to compile under
//! Verus, it's that `amenable_verus` has *no* `amenable_core`/`inventory`
//! dependency to build that machinery against at all. Verus is invoked as
//! a bare compiler over a single file tree (`verus --crate-type=lib
//! path/to/lib.rs`) — it never reads `Cargo.toml`, so it cannot resolve
//! any external crate, proc-macro or otherwise. Confirmed empirically:
//! pointing `verus` at this crate's pre-split structure (which depended on
//! `amenable_core`/`amenable_derive`/`inventory`) failed immediately with
//! unresolved-crate errors, not proof errors — matching the exact failure
//! `elicitation_verus`'s own real, working proof crate structure avoids by
//! depending on nothing but `verus_builtin_macros`/`vstd`.
//!
//! So there is no `VERIFY_*_SRC` constant to import here the way
//! `creusot_witness` imports one per proof from `amenable_creusot`
//! (`amenable_derive::harness!`, the macro that generates those constants,
//! is itself a proc-macro from a crate Verus can't resolve — it wouldn't
//! compile under Verus's toolchain either). Each `claim` below is captured
//! via `include_str!` instead — a plain Rust language feature (no
//! proc-macro, no external crate), reading `amenable_verus`'s real proof
//! source file directly at compile time, so the claim text can never drift
//! from what `verus` actually checked.
//!
//! Legal under Rust's orphan rule for the same reason `creusot_witness` is:
//! `RustStdStandard<T>` (the `Self` type) is local to this crate. Unlike
//! `creusot_witness`, though, `VerusVerifier`/`VerusVerifierMetadata`/
//! `VerusWitness` are defined *here* too, not in `amenable_verus` — they
//! need `amenable_core::{Verifier, Evidence, ...}`, which `amenable_verus`
//! no longer depends on.
//!
//! Split into one file per real standard-library carrier cluster, roughly
//! following `amenable_kani::rust_std`'s own module boundaries where a
//! cluster maps cleanly onto one std module, and named for the several
//! std modules a cluster actually covers where it doesn't (this file grew
//! one carrier at a time, in std-adjacent but not perfectly module-pure
//! order, and re-sorting the content itself -- as opposed to just cutting
//! it into smaller files at its own existing boundaries -- would trade a
//! mechanical, verifiable split for a much riskier hand-reordering of
//! ~10,000 lines of proof-registration code for a purity gain with no
//! functional benefit). [`machinery`] is the one shared-infrastructure
//! file every other file here depends on: the `VerusVerifier`, the
//! `VerusWitness` trait, the `bridge_verus_witness!`/
//! `impl_verus_witness_trusted!` macros, and the `VerusCallShape` family.

mod ascii_and_drain;
mod call_shape;
mod cell;
mod char_decode_slice_chunking;
mod char_ffi_errors;
mod collections;
mod collections_iter_cell_ref;
mod fs;
mod hash_ffi_collections_tail;
mod io_and_sync_atomic;
mod iter_adapters_a;
mod iter_adapters_b;
mod iter_adapters_c_and_fmt;
mod machinery;
mod mem_slice_net_non_zero;
mod numeric_alloc;
mod panic_ops_time_future;
mod path;
mod primitives;
mod process_and_atomic_tail;
mod rc_arc_hash;
mod str_family;
mod str_more_and_io_a;
mod sync_net_task;
mod thread_env_mpsc;

pub use call_shape::{
    VerusCallKind, VerusCallShape, VerusCallShapeRecord, VerusImport, VerusParam, verus_call_shape,
};
pub use machinery::{VerusCheckedProof, VerusVerifier, VerusVerifierMetadata, VerusWitness};
