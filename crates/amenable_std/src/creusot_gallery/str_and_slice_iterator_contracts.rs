//! Findings that `str` iterators, `slice::escape_ascii`, and
//! `slice::get_disjoint_mut` have no usable Creusot iterator contract either
//! -- the last one can even appear proved while still being unusable.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::str_iterators_lack_creusot_iterator_contracts".to_owned(),
            "core::str iterator carriers still need trusted boundaries because creusot-std lacks both method contracts and IteratorSpec for them".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (representative probes run while assessing whether the
// remaining `core::str` carriers in amenable_std::rust_std::str could be
// moved from trusted witnesses to direct Creusot proofs):
#[ensures(match result {
    (first, exhausted) => first == Some(byte) && exhausted,
})]
fn verify_bytes_yields_the_utf8_encoding(byte: u8) -> (Option<u8>, bool) {
    let c = if byte < 128 { byte as char } else { 'a' };
    let s = c.to_string();
    let mut it = s.bytes();
    let first = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, exhausted)
}

#[ensures(match result {
    (first, second, exhausted) =>
        first == Some("a") && second == Some("b") && exhausted,
})]
fn verify_lines_split_on_newlines() -> (Option<&'static str>, Option<&'static str>, bool) {
    let mut it = "a\nb".lines();
    let first = it.next();
    let second = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, second, exhausted)
}

#[ensures(match result {
    (first, second, exhausted) =>
        first == Some("a") && second == Some("b") && exhausted,
})]
fn verify_split_char_separates_on_the_pattern() -> (Option<&'static str>, Option<&'static str>, bool) {
    let mut it = "a,b".split(',');
    let first = it.next();
    let second = it.next();
    let exhausted = match it.next() {
        Some(_) => false,
        None => true,
    };
    (first, second, exhausted)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `bytes` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Bytes<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and likewise:
//   warning: calling external function `lines` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Lines<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// and:
//   warning: calling external function `split` with no contract will
//   yield an impossible precondition
//   error[E0277]: the trait bound `std::str::Split<'_, char>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// The same boundary applies across the rest of the `core::str` iterator
// family: `CharIndices`, `Chars`, `EncodeUtf16`, `EscapeDebug`,
// `EscapeDefault`, `EscapeUnicode`, `LinesAny`, `SplitAsciiWhitespace`,
// `SplitWhitespace`, `Utf8Chunks`, and the pattern-generic family
// monomorphized on `char` (`RSplit`, `SplitN`, `RSplitN`,
// `SplitInclusive`, `SplitTerminator`, `RSplitTerminator`, `Matches`,
// `RMatches`, `MatchIndices`, `RMatchIndices`). This is a real
// `creusot-std` contract gap for these carriers, not a mistaken harness.

// Working fallback (this is the real content in
// amenable_creusot::rust_std_witness today): keep these carriers registered for
// Creusot via explicit trusted witnesses whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable laws
// through direct proofs or accommodation models as appropriate.
impl CreusotWitness for RustStdStandard<std::str::Bytes<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
// The same trusted boundary is used for the rest of the `core::str`
// iterator family until creusot-std grows the missing contracts.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::slice_escape_ascii_lacks_creusot_iterator_contracts".to_owned(),
            "slice escape_ascii still needs a trusted boundary because creusot-std has neither iterator contracts nor stable byte-literal ergonomics for the direct proof".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<EscapeAscii<'static>>):
#[ensures(match result {
    (first_matches, second_is_backslash, third_is_n, exhausted) =>
        first_matches && second_is_backslash && third_is_n && exhausted,
})]
fn verify_escape_ascii_leaves_printable_bytes_unescaped() -> (bool, bool, bool, bool) {
    let data = [b'A', b'\n'];
    let mut escaped = data.escape_ascii();
    let first = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let second = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let third = match escaped.next() {
        Some(value) => value,
        None => 0,
    };
    let exhausted = match escaped.next() {
        Some(_) => false,
        None => true,
    };
    (
        first == b'A',
        second == b'\\',
        third == b'n',
        exhausted,
    )
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `escape_ascii` with no contract
//   will yield an impossible precondition
//   error[E0277]: the trait bound `std::slice::EscapeAscii<'_>:
//   creusot_std::prelude::IteratorSpec` is not satisfied
// So the direct iterator route is blocked on the same `IteratorSpec` gap
// as the chunk families. While reducing the repro, an earlier postcondition
// that wrote the byte literals directly also triggered a separate
// creusot-rustc ICE:
//   error: internal compiler error: Unsupported literal
// at `second == b'\\'` inside `#[ensures]`
// The literal-free postcondition above still fails on the missing iterator
// contracts, so the carrier is blocked even without the ICE-inducing form.

// Working fallback (this is the real content in
// amenable_creusot::rust_std_witness today): keep the carrier registered for
// Creusot via an explicit trusted witness whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law
// through its bounded accommodation model.
impl CreusotWitness for RustStdStandard<EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::get_disjoint_mut_without_a_contract_can_appear_proved_but_is_still_unusable".to_owned(),
            "get_disjoint_mut can appear proved under Creusot even though the call is contractless and therefore not acceptable proof evidence".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::Proved,
            r#"
// Failing form (attempted while adding direct Creusot coverage for
// amenable_std::rust_std::RustStdStandard<GetDisjointMutError>):
#[ensures(match result {
    (disjoint_ok, overlap_err, out_of_bounds_err) =>
        disjoint_ok && overlap_err && out_of_bounds_err,
})]
fn verify_get_disjoint_mut_rejects_overlap_and_out_of_bounds(
    a: i32,
    b: i32,
) -> (bool, bool, bool) {
    let mut data = [a, b, 0, 0];
    let disjoint_ok = data.get_disjoint_mut([0, 2]).is_ok();
    let overlap_err = data.get_disjoint_mut([0, 0]).is_err();
    let out_of_bounds_err = data.get_disjoint_mut([0, 10]).is_err();
    (disjoint_ok, overlap_err, out_of_bounds_err)
}

// Observed under `cargo creusot prove -- -p amenable_creusot` on August
// 5, 2026:
//   warning: calling external function `get_disjoint_mut` with no contract
//   will yield an impossible precondition
// and then the crate still finishes with:
//   Proved (34 files) ✔
// This is exactly the dangerous false trail documented elsewhere in the
// gallery: a contractless external call can let a harness "prove" for the
// wrong reason. The reported success is not acceptable evidence for
// Amenable's registry because `creusot-std` still has no contract telling
// the verifier what `get_disjoint_mut` actually does.

// Working fallback (this is the real content in
// amenable_creusot::rust_std_witness today): keep the carrier registered for
// Creusot via an explicit trusted witness whose provenance still comes from
// the same proof chain, while Kani continues to carry the executable law.
impl CreusotWitness for RustStdStandard<GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}
"#.to_owned(),
        ),
    )
}
