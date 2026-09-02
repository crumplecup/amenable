//! Findings about which Rust constructs `creusot-rustc` can translate at all:
//! return-position `impl Trait`, `inventory::submit!`'s generated `static`,
//! `#[cfg(not(creusot))]` gating, casting `char` to `u32`, calling
//! `String::len` from a logic context, and boxed `dyn Iterator`.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::rpitit_panics_intrinsics_gathering".to_owned(),
            "a local return-position impl Trait method panics creusot-rustc's intrinsics-gathering pass".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::Ice,
            r#"
// Reduced repro (this exact shape lived in amenable_creusot::witness before
// the fix — see amenable_core::Provenance's own doc comment):
trait Provenance {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry>;
}
impl Provenance for CreusotVerifierMetadata {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   thread 'rustc' panicked at .../rustc_middle/src/hir/mod.rs:409:84:
//   index out of bounds: the len is 354 but the index is 355
//   query stack during panic:
//   #0 [local_def_id_to_hir_id] getting HIR ID of
//      `witness::<impl ...>::metadata::{opaque#0}::'_`
// RPITIT desugars to a compiler-synthesized opaque type at every impl
// site; creusot-rustc's `gather_intrinsics` pass enumerates every local
// def-id (including synthetic opaque ones) and can't map this one back to
// a HIR ID. Not gated by #[cfg(creusot)] mattering: this impl was ordinary,
// always-compiled code, swept up because it's local to a crate that
// depends on creusot-std at all.
//
// Fix: replace with an associated type (`type MetadataIter: Iterator<Item
// = MetadataEntry>; fn metadata(&self) -> Self::MetadataIter;`) — an
// ordinary named item, not an opaque one. See amenable_core::Provenance.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::inventory_submit_static_is_unsupported".to_owned(),
            "::inventory::submit!'s generated static item can't be translated".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Reduced repro (this exact shape lived in amenable_creusot::rust_std
// before the fix — the ProofRecord registrations that now live in
// amenable_std::creusot_witness instead):
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "creusot",
        || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    )
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: unsupported definition kind DefId(0:121 ~ amenable_creusot[..]
//   ::rust_std::_::__INVENTORY) Static { safety: Safe, mutability: Not,
//   nested: false }
// `inventory::submit!` expands to a `static` item (its linker-section-based
// distributed-registration mechanism); creusot-rustc's translator has no
// support for arbitrary `static` items at all, so ANY use of `inventory`
// inside a creusot-translated crate fails this way — not specific to
// ProofRecord.
//
// Fix: move every inventory::submit! (and the Witness bridge/CheckedProof
// machinery around it) out of amenable_creusot entirely, into
// amenable_std::creusot_witness — legal under the orphan rule because
// RustStdStandard<T>, not the verifier marker, is the local type there.
// amenable_creusot now contains zero inventory calls.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::cfg_not_creusot_gating_avoids_the_inventory_and_dyn_iterator_errors".to_owned(),
            "#[cfg(not(creusot))]-gating inventory::collect!/submit! and Box<dyn Iterator<..>> avoids both translator errors entirely -- confirmed in an isolated probe crate, not assumed from the earlier crate-split fix".to_owned(),
            CreusotGalleryDisposition::BestPractice,
            CreusotGalleryExpectation::Proved,
            r#"
// The `inventory_submit_static_is_unsupported` case above (and this
// crate's own historical split of witness-bridge code out of
// amenable_creusot into amenable_std) concluded "any use of inventory
// inside a creusot-translated crate fails this way" and moved the whole
// mechanism to a different crate, without separately testing whether
// #[cfg(not(creusot))]-gating the offending items *in place* would have
// been enough on its own. Tested directly in an isolated, throwaway
// probe crate (creusot_ice_probe, deleted after confirming the finding
// -- not kept live in the real workspace, matching this gallery's own
// "reduced repro as a string, not live risky code" discipline) rather
// than risking `amenable_creusot`'s real 110-file proof suite:
//
// Gating only `submit!` and leaving `collect!` ungated still fails --
// `collect!` independently trips its own translator error, a different
// message from the Static one above:
inventory::collect!(Entry);           // ungated: fails on its own
#[cfg(not(creusot))]
inventory::submit! { Entry { name: "probe" } }
//   error: Unsupported constant value: Scalar(alloc1) of type
//   &'?2 inventory::Registry
//     --> src/lib.rs:22:1
//      |
//   22 | inventory::collect!(Entry);
//      | ^^^^^^^^^^^^^^^^^^^^^^^^^^
//
// Gating BOTH `collect!` and `submit!` — real fix, confirmed clean
// (`cargo creusot -- -p creusot_ice_probe` succeeds, only ordinary
// unused-import/dead-code warnings):
#[cfg(not(creusot))]
inventory::collect!(Entry);
#[cfg(not(creusot))]
inventory::submit! { Entry { name: "probe" } }
//   Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
//
// A second, independent pattern was also checked in the same probe:
// `Box<dyn Iterator<Item = i32>>` as an associated-type value (the
// pattern `amenable_kani::stoplight`'s real `Green`/`Yellow`/`Red`
// `Provenance` impls use) — NOT the same thing as the already-fixed
// `rpitit_panics_intrinsics_gathering` case above (return-position impl
// Trait; `Box<dyn Iterator<..>>` is a concrete, ordinary trait-object
// type, no RPITIT desugaring involved at all). Left deliberately
// ungated first, confirming it is a real, independent, separate
// translator error, not folklore carried over from the RPITIT case:
impl HasIter for Boxed {
    type Iter = Box<dyn Iterator<Item = i32>>;    // ungated: fails on its own
    fn iter(&self) -> Self::Iter { Box::new(std::iter::once(1)) }
}
//   error: forbidden dyn type: dyn std::iter::Iterator<Item = i32>
//   (dyn support is currently minimal, please open an issue to improve
//   this feature)
//
// Gated the same way, it also translates clean.
//
// Implication, not yet applied anywhere real: the whole-crate ICE risk
// that justified `amenable_creusot::stoplight`'s accommodation-model
// mirror (a separately-authored, hand-kept-in-sync copy of `amenable_
// kani::stoplight`'s concrete types, needing `stoplight_mirror_
// consistency_test.rs` to guard drift) and the `amenable_kani ->
// amenable_creusot`/`amenable_std -> amenable_creusot` Cargo dependency
// edges built to work around it may not have been the only available
// fix. `#[cfg(not(creusot))]`-gating the specific offending items *in
// place*, precisely (both `collect!` and `submit!`, and any `Box<dyn
// ..>` associated type), is a real, confirmed, working alternative --
// worth real consideration next time this class of tradeoff comes up,
// not assumed away by this gallery's own earlier, narrower test.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::char_as_u32_cast_is_unsupported".to_owned(),
            "`c as u32` isn't a supported cast in Pearlite logic context; use the `@` View operator".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact clause was in amenable_creusot::rust_std's char
// contract before the fix):
#[ensures((c as u32) <= 0xD7FFu32 || ((c as u32) >= 0xE000u32 && (c as u32) <= 0x10FFFFu32))]
fn verify_char_roundtrip(c: char) -> char { c }

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: unsupported cast from char to u32 (allowed: bool as integer,
//   integer as integer, or pointer as pointer)
//
// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today):
#[ensures(c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF))]
fn verify_char_roundtrip(c: char) -> char { c }
// `char`'s `View` impl in creusot-std maps to Pearlite's arbitrary-precision
// `Int` via a builtin (`creusot.prelude.Char.to_int`) — `@` is the intended
// way to reach a char's ordinal value in logic context, not a program-level
// cast.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::string_len_in_logic_context_is_unsupported".to_owned(),
            "`s.len()` can't be called directly inside #[ensures]; wrap it in a #[trusted] #[logic(opaque)] accessor".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form:
#[ensures(result.len() == s.len())]
fn verify_string_roundtrip(s: String) -> String { s }

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: called program function `std::string::String::len` in logic
//   context

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today):
#[trusted]
#[logic(opaque)]
fn string_len(_s: &String) -> usize { dead }

#[ensures(string_len(&result) == string_len(&s))]
fn verify_string_roundtrip(s: String) -> String { s }
// `String::len` is a program function; Pearlite logic context (#[requires]/
// #[ensures]) can only call #[logic] functions. `elicitation`'s own
// logic_fns.rs solves this the same way for every stdlib method it needs
// inside a contract: a #[trusted] #[logic(opaque)] wrapper axiomatizing the
// relationship (never proven, just asserted) so the real method becomes
// referenceable from logic context at all.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::boxed_dyn_iterator_is_unsupported".to_owned(),
            "Box<dyn Iterator<...>> has \"currently minimal\" dyn support in creusot-rustc".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact shape was amenable_creusot::witness's own
// Provenance impl before the fix — and is still the shape every OTHER
// verifier backend's own equivalent impl uses, in amenable_kani/
// amenable_verus, since neither of those crates is ever creusot-translated):
type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;
fn metadata(&self) -> Self::MetadataIter {
    Box::new(FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v)))
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: forbidden dyn type: dyn std::iter::Iterator<Item =
//   amenable_core::MetadataEntry> (dyn support is currently minimal,
//   please open an issue to improve this feature)

// Working form (this is the real, proven impl, in
// amenable_creusot::witness today):
type MetadataIter = std::vec::IntoIter<MetadataEntry>;
fn metadata(&self) -> Self::MetadataIter {
    FACTS
        .iter()
        .map(|&(k, v)| MetadataEntry::new(k, v))
        .collect::<Vec<_>>()
        .into_iter()
}
// Only matters for the one Provenance impl actually local to a
// creusot-translated crate — everywhere else in this workspace, `Box<dyn
// Iterator<...>>` is the right, general answer (see
// amenable_core::provenance's impl_scalar_provenance! macro).
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::partial_eq_derive_requires_deep_model".to_owned(),
            "deriving PartialEq (and so Eq/PartialOrd/Ord) requires a DeepModel impl under real translation".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact derive list was amenable_creusot::witness's
// CreusotVerifierMetadata before the fix — CLAUDE.md's own standard
// derive policy for data structures, applied uniformly to a marker type
// that's actually creusot-translated):
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreusotVerifierMetadata;

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0277]: the trait bound `witness::CreusotVerifierMetadata:
//   creusot_std::model::DeepModel` is not satisfied
//   help: the trait `creusot_std::model::DeepModel` is not implemented for
//   `witness::CreusotVerifierMetadata`
// (`Debug`/`Clone`/`Copy`/`Hash`/`Default` alone don't trigger this —
// confirmed by dropping only PartialEq/Eq/PartialOrd/Ord and re-running;
// only the comparison-generating derives need a DeepModel.)

// Working form (this is the real derive list, in
// amenable_creusot::witness today):
#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct CreusotVerifierMetadata;
// Applies only where nothing actually needs the comparison derives —
// confirmed here specifically (Verifier::Metadata only requires Provenance
// + Default; nothing compares two CreusotVerifierMetadata values anywhere
// in this workspace) before dropping them, not as a blanket rule.
"#.to_owned(),
        ),
    )
}
