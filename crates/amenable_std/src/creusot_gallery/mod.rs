//! Creusot proof gallery: documented findings about `creusot-rustc`'s own
//! translation behavior, discovered while building the real char/String/
//! Duration/`NonZero<i16>`/`Ordering`/`Wrapping<i32>`/`Saturating<i32>`/
//! `IntErrorKind` proof pipeline in `amenable_creusot`.
//!
//! Mirrors `amenable_kani`'s gallery in spirit — production proofs answer
//! "does this harness establish the intended claim?", the gallery answers
//! "what does the verifier do with this pattern?" — but not in mechanism.
//! Kani gallery cases are live, independently runnable `#[kani::proof]`
//! harnesses: one failing or timing out doesn't stop `cargo kani` from
//! running the others. Creusot has no equivalent isolation — `cargo
//! creusot`/`creusot-rustc` translates a whole crate as one compilation
//! unit, and a single ICE or translation error anywhere aborts the entire
//! build, including the real char/String proofs this crate exists to
//! protect. So a gallery case here is *not* live Pearlite content: `claim`
//! is a plain string constant holding the reduced repro, hand-verified
//! once against the real toolchain (`just verify-creusot-translate`) and
//! recorded as a fact, not re-checked automatically on every build. Cases
//! whose `expected` is [`CreusotGalleryExpectation::Proved`] are the
//! exception — safe to keep live, since a real one already does (see
//! `amenable_creusot::rust_std`'s `verify_char_roundtrip`, which uses the
//! `c@` idiom this gallery's `char_as_u32_cast_is_unsupported` case failed
//! without).
//!
//! [`model`] is the gallery's own data model. Every other file registers a
//! themed cluster of cases: [`macro_and_type_translation`], [`extern_spec_and_logic_context`],
//! [`float_and_misc`], [`slice_iterator_contracts`],
//! [`str_and_slice_iterator_contracts`], and [`collection_and_macro_tail`].

mod collection_and_macro_tail;
mod extern_spec_and_logic_context;
mod float_and_misc;
mod macro_and_type_translation;
mod model;
mod slice_iterator_contracts;
mod str_and_slice_iterator_contracts;

pub use model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};
