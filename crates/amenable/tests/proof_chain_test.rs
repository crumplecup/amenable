use amenable::ChainError;

// These three verifier-completeness assertions only hold when
// `amenable_creusot`'s registrations are linked in, which only happens
// under the `creusot` feature (nightly-only, see justfile's
// `test-creusot`) — `amenable_creusot` depends on `creusot-std`, which
// can't compile on stable, so it's not in `amenable`'s default features.
#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn bool_proof_chain_is_a_single_root_node_with_all_three_verifiers() {
    let report =
        amenable::proof_chain("RustStdStandard<bool>").expect("bool's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<bool>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 3);
    assert_eq!(report.verifiers.len(), 3);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    assert!(verifiers.contains(&"verus"));
}

#[test]
fn char_proof_chain_carries_the_checked_harness_name_per_verifier() {
    let report =
        amenable::proof_chain("RustStdStandard<char>").expect("char's evidence link is registered");

    let root = &report.root;
    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for char");

    assert!(kani_description.contains("verify_char_unicode_scalar"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<Cell<i32>>")
        .expect("Cell's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Cell<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Cell");
    assert!(kani_description.contains("verify_cell_get_set_replace_take_round_trip"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<RefCell<i32>>")
        .expect("RefCell's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<RefCell<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for RefCell");
    assert!(kani_description.contains("verify_ref_cell_dynamic_borrow_rules"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Ref<'static, i32>>").expect("Ref's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Ref<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Ref");
    assert!(kani_description.contains("verify_ref_derefs_to_the_borrowed_value"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_mut_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<RefMut<'static, i32>>").expect("RefMut's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<RefMut<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for RefMut");
    assert!(kani_description.contains("verify_ref_mut_derefs_and_writes_through_to_the_cell"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn once_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<OnceCell<i32>>")
        .expect("OnceCell's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<OnceCell<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for OnceCell");
    assert!(kani_description.contains("verify_once_cell_initializes_exactly_once"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn unsafe_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<UnsafeCell<i32>>")
        .expect("UnsafeCell's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<UnsafeCell<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for UnsafeCell");
    assert!(kani_description.contains("verify_unsafe_cell_get_mut_and_into_inner_round_trip"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn lazy_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<LazyCell<i32, fn() -> i32>>").expect("LazyCell's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<LazyCell<i32, fn() -> i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for LazyCell");
    assert!(kani_description.contains("verify_lazy_cell_caches_its_initializer_result"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn borrow_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<BorrowError>")
        .expect("BorrowError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<BorrowError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn borrow_mut_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<BorrowMutError>")
        .expect("BorrowMutError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<BorrowMutError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_try_from_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<CharTryFromError>")
        .expect("CharTryFromError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<CharTryFromError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for CharTryFromError");
    assert!(
        kani_description
            .contains("verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn decode_utf16_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>").expect("DecodeUtf16's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DecodeUtf16");
    assert!(kani_description.contains("verify_decode_utf16_round_trips_a_bmp_code_unit"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn decode_utf16_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<DecodeUtf16Error>")
        .expect("DecodeUtf16Error's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<DecodeUtf16Error>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DecodeUtf16Error");
    assert!(kani_description.contains("verify_decode_utf16_error_reports_the_unpaired_surrogate"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_debug_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::char::EscapeDebug>").expect("core::char::EscapeDebug's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::char::EscapeDebug>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::char::EscapeDebug");
    assert!(kani_description.contains("verify_char_escape_debug_escapes_a_newline"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_default_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::char::EscapeDefault>").expect("core::char::EscapeDefault's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::char::EscapeDefault>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::char::EscapeDefault");
    assert!(kani_description.contains("verify_char_escape_default_escapes_a_newline"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_unicode_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::char::EscapeUnicode>").expect("core::char::EscapeUnicode's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::char::EscapeUnicode>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::char::EscapeUnicode");
    assert!(kani_description.contains("verify_char_escape_unicode_renders_the_codepoint_escape"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_char_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<ParseCharError>")
        .expect("ParseCharError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<ParseCharError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for ParseCharError");
    assert!(
        kani_description
            .contains("verify_parse_char_error_occurs_for_empty_or_multi_character_strings")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn to_lowercase_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<ToLowercase>")
        .expect("ToLowercase's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<ToLowercase>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for ToLowercase");
    assert!(kani_description.contains("verify_to_lowercase_maps_an_uppercase_ascii_letter"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn to_uppercase_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<ToUppercase>")
        .expect("ToUppercase's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<ToUppercase>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for ToUppercase");
    assert!(kani_description.contains("verify_to_uppercase_maps_a_lowercase_ascii_letter"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_char_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<TryFromCharError>")
        .expect("TryFromCharError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<TryFromCharError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for TryFromCharError");
    assert!(
        kani_description.contains("verify_try_from_char_error_occurs_exactly_when_out_of_range")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn btree_map_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<BTreeMap<i32, i32>>")
        .expect("BTreeMap's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<BTreeMap<i32, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BTreeMap");
    assert!(kani_description.contains("verify_btree_map_iterates_in_key_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for BTreeMap");
    assert!(creusot_description.contains("verify_btree_map_iterates_in_key_order"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn btree_set_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<BTreeSet<i32>>")
        .expect("BTreeSet's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<BTreeSet<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BTreeSet");
    assert!(kani_description.contains("verify_btree_set_iterates_in_sorted_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for BTreeSet");
    assert!(creusot_description.contains("verify_btree_set_iterates_in_sorted_order"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<BinaryHeap<i32>>")
        .expect("BinaryHeap's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<BinaryHeap<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BinaryHeap");
    assert!(kani_description.contains("verify_binary_heap_pop_yields_the_maximum_first"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for BinaryHeap");
    assert!(creusot_description.contains("verify_binary_heap_pop_yields_the_maximum_first"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_drain_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>").expect("BinaryHeap drain's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BinaryHeap drain");
    assert!(kani_description.contains("verify_binary_heap_drain_yields_every_pushed_element_once"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for BinaryHeap drain");
    assert!(
        creusot_description.contains("verify_binary_heap_drain_yields_every_pushed_element_once")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::binary_heap::IntoIter<i32>>").expect("BinaryHeap into_iter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::binary_heap::IntoIter<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BinaryHeap into_iter");
    assert!(
        kani_description.contains("verify_binary_heap_into_iter_yields_every_pushed_element_once")
    );

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for BinaryHeap into_iter");
    assert!(
        creusot_description
            .contains("verify_binary_heap_into_iter_yields_every_pushed_element_once")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<LinkedList<i32>>")
        .expect("LinkedList's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<LinkedList<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for LinkedList");
    assert!(kani_description.contains("verify_linked_list_is_fifo_through_back_and_front"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for LinkedList");
    assert!(creusot_description.contains("verify_linked_list_is_fifo_through_back_and_front"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::linked_list::IntoIter<i32>>").expect("LinkedList into_iter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::linked_list::IntoIter<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for LinkedList into_iter");
    assert!(kani_description.contains("verify_linked_list_into_iter_yields_owned_values_in_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for LinkedList into_iter");
    assert!(
        creusot_description.contains("verify_linked_list_into_iter_yields_owned_values_in_order")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_reserve_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<TryReserveError>")
        .expect("TryReserveError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<TryReserveError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for TryReserveError");
    assert!(kani_description.contains("verify_try_reserve_rejects_an_impossible_capacity"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for TryReserveError");
    assert!(creusot_description.contains("verify_try_reserve_rejects_an_impossible_capacity"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<VecDeque<i32>>")
        .expect("VecDeque's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<VecDeque<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for VecDeque");
    assert!(kani_description.contains("verify_vec_deque_pushes_and_pops_from_both_ends"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for VecDeque");
    assert!(creusot_description.contains("verify_vec_deque_pushes_and_pops_from_both_ends"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::vec_deque::IntoIter<i32>>").expect("VecDeque into_iter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::vec_deque::IntoIter<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for VecDeque into_iter");
    assert!(kani_description.contains("verify_vec_deque_into_iter_yields_owned_values_in_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for VecDeque into_iter");
    assert!(
        creusot_description.contains("verify_vec_deque_into_iter_yields_owned_values_in_order")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_drain_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>").expect("VecDeque drain's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for VecDeque drain");
    assert!(kani_description.contains("verify_vec_deque_drain_removes_and_yields_in_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for VecDeque drain");
    assert!(creusot_description.contains("verify_vec_deque_drain_removes_and_yields_in_order"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>").expect("VecDeque iter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for VecDeque iter");
    assert!(kani_description.contains("verify_vec_deque_iter_yields_references_in_order"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for VecDeque iter");
    assert!(creusot_description.contains("verify_vec_deque_iter_yields_references_in_order"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>").expect("VecDeque iter_mut's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for VecDeque iter_mut");
    assert!(kani_description.contains("verify_vec_deque_iter_mut_writes_through"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for VecDeque iter_mut");
    assert!(creusot_description.contains("verify_vec_deque_iter_mut_writes_through"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<Vec<i32>>")
        .expect("Vec's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Vec<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Vec");
    assert!(kani_description.contains("verify_vec_push_pop_round_trips"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_drain_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::vec::Drain<'static, i32>>").expect("Vec drain's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::vec::Drain<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Vec drain");
    assert!(kani_description.contains("verify_vec_drain_removes_and_yields_in_order"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_into_iter_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::vec::IntoIter<i32>>").expect("Vec into_iter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::vec::IntoIter<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Vec into_iter");
    assert!(kani_description.contains("verify_vec_into_iter_yields_owned_values_in_order"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_extract_if_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>").expect("Vec extract_if's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Vec extract_if");
    assert!(kani_description.contains("verify_vec_extract_if_partitions_by_the_predicate"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_splice_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>").expect("Vec splice's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Vec splice");
    assert!(kani_description.contains("verify_splice_replaces_a_range_and_yields_what_it_removed"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn type_id_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<TypeId>")
        .expect("TypeId's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<TypeId>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for TypeId");
    assert!(
        kani_description.contains("verify_type_id_is_reflexive_and_distinguishes_distinct_types")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn layout_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<Layout>")
        .expect("Layout's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Layout>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Layout");
    assert!(kani_description.contains("verify_layout_new_reports_the_types_size_and_alignment"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_slice_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<TryFromSliceError>")
        .expect("TryFromSliceError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<TryFromSliceError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for TryFromSliceError");
    assert!(kani_description.contains("verify_try_from_slice_rejects_a_length_mismatch"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn array_into_iter_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<IntoIter<i32, 3>>")
        .expect("array IntoIter's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<IntoIter<i32, 3>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for array IntoIter");
    assert!(kani_description.contains("verify_array_into_iter_yields_elements_in_order"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ascii_escape_default_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<core::ascii::EscapeDefault>")
        .expect("core::ascii::EscapeDefault's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::ascii::EscapeDefault>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::ascii::EscapeDefault");
    assert!(kani_description.contains("verify_escape_default_escapes_a_control_byte"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ordering_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<std::cmp::Ordering>")
        .expect("Ordering's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::cmp::Ordering>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Ordering");
    assert!(kani_description.contains("verify_ordering_reverse_involution"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Ordering");
    assert!(creusot_description.contains("verify_ordering_reverse_swaps_less_and_greater"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn reverse_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<Reverse<i32>>")
        .expect("Reverse<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Reverse<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Reverse<i32>");
    assert!(kani_description.contains("verify_reverse_inverts_comparison"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Reverse<i32>");
    assert!(creusot_description.contains("verify_reverse_inverts_comparison"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rc_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<Rc<i32>>")
        .expect("Rc's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Rc<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rc_weak_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::rc::Weak<i32>>").expect("Rc weak's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::rc::Weak<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn string_drain_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::string::Drain<'static>>").expect("String drain's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::string::Drain<'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for String drain");
    assert!(kani_description.contains("verify_string_drain_removes_and_yields_the_content"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_utf16_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<FromUtf16Error>")
        .expect("FromUtf16Error's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<FromUtf16Error>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromUtf16Error");
    assert!(kani_description.contains("verify_from_utf16_rejects_a_lone_surrogate"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_utf8_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<FromUtf8Error>")
        .expect("FromUtf8Error's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<FromUtf8Error>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromUtf8Error");
    assert!(kani_description.contains("verify_from_utf8_error_recovers_the_original_bytes"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn arc_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<Arc<i32>>")
        .expect("Arc's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Arc<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn arc_weak_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::sync::Weak<i32>>").expect("Arc weak's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::sync::Weak<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn infallible_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<Infallible>")
        .expect("Infallible's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Infallible>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn layout_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<LayoutError>")
        .expect("LayoutError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<LayoutError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for LayoutError");
    assert!(
        kani_description
            .contains("verify_layout_from_size_align_rejects_a_non_power_of_two_alignment")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn addr_parse_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    let report = amenable::proof_chain("RustStdStandard<AddrParseError>")
        .expect("AddrParseError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<AddrParseError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);
    assert!(root.proofs.iter().any(|(verifier, _)| *verifier == "kani"));
    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ip_addr_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<IpAddr>").expect("IpAddr's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<IpAddr>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for IpAddr");
    assert!(kani_description.contains("verify_ip_addr_variant_matches_its_kind"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ipv4_addr_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Ipv4Addr>").expect("Ipv4Addr's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Ipv4Addr>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Ipv4Addr");
    assert!(kani_description.contains("verify_ipv4_addr_octets_round_trip"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ipv6_addr_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Ipv6Addr>").expect("Ipv6Addr's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Ipv6Addr>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Ipv6Addr");
    assert!(kani_description.contains("verify_ipv6_addr_segments_round_trip"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<SocketAddr>").expect("SocketAddr's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<SocketAddr>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for SocketAddr");
    assert!(kani_description.contains("verify_socket_addr_variant_matches_its_kind"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_v4_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<SocketAddrV4>").expect("SocketAddrV4's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<SocketAddrV4>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for SocketAddrV4");
    assert!(kani_description.contains("verify_socket_addr_v4_round_trips_ip_and_port"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_v6_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<SocketAddrV6>").expect("SocketAddrV6's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<SocketAddrV6>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for SocketAddrV6");
    assert!(kani_description.contains("verify_socket_addr_v6_round_trips_all_fields"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
fn nonzero_i8_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<i8>>").expect("NonZero<i8>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<i8>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<i8>");
    assert!(kani_description.contains("verify_nonzero_i8"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn nonzero_i16_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<i16>>").expect("NonZero<i16>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<i16>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<i16>");
    assert!(kani_description.contains("verify_nonzero_i16"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for NonZero<i16>");
    assert!(creusot_description.contains("verify_nonzero_i16_roundtrips"));
}

#[test]
fn nonzero_i32_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<i32>>").expect("NonZero<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<i32>");
    assert!(kani_description.contains("verify_nonzero_i32"));
}

#[test]
fn nonzero_i64_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<i64>>").expect("NonZero<i64>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<i64>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<i64>");
    assert!(kani_description.contains("verify_nonzero_i64"));
}

#[test]
fn nonzero_i128_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<i128>>").expect("NonZero<i128>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<i128>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<i128>");
    assert!(kani_description.contains("verify_nonzero_i128"));
}

#[test]
fn nonzero_isize_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<isize>>").expect("NonZero<isize>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<isize>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<isize>");
    assert!(kani_description.contains("verify_nonzero_isize"));
}

#[test]
fn nonzero_u8_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<u8>>").expect("NonZero<u8>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<u8>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<u8>");
    assert!(kani_description.contains("verify_nonzero_u8"));
}

#[test]
fn nonzero_u16_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<u16>>").expect("NonZero<u16>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<u16>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<u16>");
    assert!(kani_description.contains("verify_nonzero_u16"));
}

#[test]
fn nonzero_u32_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<u32>>").expect("NonZero<u32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<u32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<u32>");
    assert!(kani_description.contains("verify_nonzero_u32"));
}

#[test]
fn nonzero_u64_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<u64>>").expect("NonZero<u64>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<u64>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<u64>");
    assert!(kani_description.contains("verify_nonzero_u64"));
}

#[test]
fn nonzero_u128_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<u128>>").expect("NonZero<u128>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<u128>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<u128>");
    assert!(kani_description.contains("verify_nonzero_u128"));
}

#[test]
fn nonzero_usize_proof_chain_registers_the_kani_proof() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<NonZero<usize>>").expect("NonZero<usize>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NonZero<usize>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 1);
    assert_eq!(report.verifiers.len(), 1);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NonZero<usize>");
    assert!(kani_description.contains("verify_nonzero_usize"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn wrapping_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Wrapping<i32>>").expect("Wrapping<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Wrapping<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Wrapping<i32>");
    assert!(kani_description.contains("verify_wrapping_add_matches_the_inner_wrapping_add"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Wrapping<i32>");
    assert!(creusot_description.contains("verify_wrapping_i32_add_wraps"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn saturating_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Saturating<i32>>").expect("Saturating<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Saturating<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Saturating<i32>");
    assert!(kani_description.contains("verify_saturating_add_matches_the_inner_saturating_add"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Saturating<i32>");
    assert!(creusot_description.contains("verify_saturating_i32_add_clamps"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn int_error_kind_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::num::IntErrorKind>").expect("core::num::IntErrorKind's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::num::IntErrorKind>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::num::IntErrorKind");
    assert!(kani_description.contains("verify_int_error_kind_classifies_parse_failures"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::num::IntErrorKind");
    assert!(creusot_description.contains("verify_int_error_kind_classifies_parse_failures"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_int_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::num::TryFromIntError>").expect("core::num::TryFromIntError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::num::TryFromIntError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::num::TryFromIntError");
    assert!(
        kani_description.contains("verify_try_from_int_error_occurs_exactly_when_out_of_range")
    );

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::num::TryFromIntError");
    assert!(
        creusot_description.contains("verify_try_from_int_error_occurs_exactly_when_out_of_range")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_int_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::num::ParseIntError>").expect("core::num::ParseIntError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::num::ParseIntError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::num::ParseIntError");
    assert!(kani_description.contains("verify_parse_int_error_reports_the_kind_of_the_failure"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::num::ParseIntError");
    assert!(creusot_description.contains("verify_parse_int_error_reports_the_kind_of_the_failure"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fp_category_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::num::FpCategory>").expect("core::num::FpCategory's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::num::FpCategory>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::num::FpCategory");
    assert!(kani_description.contains("verify_fp_category_matches_the_value_it_classifies"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::num::FpCategory");
    assert!(creusot_description.contains("verify_fp_category_matches_the_value_it_classifies"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_float_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::num::ParseFloatError>").expect("core::num::ParseFloatError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::num::ParseFloatError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::num::ParseFloatError");
    assert!(
        kani_description.contains("verify_parse_float_error_occurs_only_for_unparseable_input")
    );

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::num::ParseFloatError");
    assert!(
        creusot_description.contains("verify_parse_float_error_occurs_only_for_unparseable_input")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cstring_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<CString>")
        .expect("CString's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<CString>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for CString");
    assert!(
        kani_description
            .contains("verify_cstring_excludes_the_terminator_and_rejects_interior_nul")
    );

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for CString");
    assert!(
        creusot_description
            .contains("verify_cstring_excludes_the_terminator_and_rejects_interior_nul")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_vec_with_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<FromVecWithNulError>")
        .expect("FromVecWithNulError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<FromVecWithNulError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromVecWithNulError");
    assert!(kani_description.contains("verify_from_vec_with_nul_requires_the_nul_only_at_the_end"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for FromVecWithNulError");
    assert!(
        creusot_description.contains("verify_from_vec_with_nul_requires_the_nul_only_at_the_end")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn into_string_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<IntoStringError>")
        .expect("IntoStringError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<IntoStringError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for IntoStringError");
    assert!(kani_description.contains("verify_into_string_error_recovers_the_original_cstring"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for IntoStringError");
    assert!(creusot_description.contains("verify_into_string_error_recovers_the_original_cstring"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<NulError>")
        .expect("NulError's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<NulError>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for NulError");
    assert!(kani_description.contains("verify_nul_error_reports_the_interior_nuls_position"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for NulError");
    assert!(creusot_description.contains("verify_nul_error_reports_the_interior_nuls_position"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cstr_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report =
        amenable::proof_chain("RustStdStandard<CStr>").expect("CStr's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<CStr>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for CStr");
    assert!(kani_description.contains("verify_cstr_excludes_the_terminating_nul_from_to_bytes"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for CStr");
    assert!(creusot_description.contains("verify_cstr_excludes_the_terminating_nul_from_to_bytes"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_bytes_until_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<FromBytesUntilNulError>")
        .expect("FromBytesUntilNulError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<FromBytesUntilNulError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromBytesUntilNulError");
    assert!(kani_description.contains("verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for FromBytesUntilNulError");
    assert!(
        creusot_description.contains("verify_from_bytes_until_nul_requires_a_nul_byte_somewhere")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_bytes_with_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() {
    let report = amenable::proof_chain("RustStdStandard<FromBytesWithNulError>")
        .expect("FromBytesWithNulError's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<FromBytesWithNulError>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromBytesWithNulError");
    assert!(
        kani_description.contains("verify_from_bytes_with_nul_requires_the_nul_only_at_the_end")
    );

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for FromBytesWithNulError");
    assert!(
        creusot_description.contains("verify_from_bytes_with_nul_requires_the_nul_only_at_the_end")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn c_void_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::ffi::c_void>").expect("core::ffi::c_void's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::ffi::c_void>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_alignment_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::fmt::Alignment>").expect("std::fmt::Alignment's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::fmt::Alignment>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for std::fmt::Alignment");
    assert!(
        kani_description.contains("verify_alignment_reaches_the_formatter_from_the_format_spec")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_arguments_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Arguments<'static>>").expect("Arguments' evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Arguments<'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Arguments");
    assert!(kani_description.contains("verify_arguments_renders_the_same_as_the_value_itself"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_error_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::fmt::Error>").expect("std::fmt::Error's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<std::fmt::Error>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_formatter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Formatter<'static>>").expect("Formatter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Formatter<'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Formatter");
    assert!(kani_description.contains("verify_formatter_exposes_the_parsed_width_and_precision"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_list_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DebugList<'static, 'static>>").expect("DebugList's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DebugList<'static, 'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DebugList");
    assert!(kani_description.contains("verify_debug_list_renders_entries_in_brackets"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_map_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DebugMap<'static, 'static>>").expect("DebugMap's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DebugMap<'static, 'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DebugMap");
    assert!(kani_description.contains("verify_debug_map_renders_key_value_pairs"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_set_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DebugSet<'static, 'static>>").expect("DebugSet's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DebugSet<'static, 'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DebugSet");
    assert!(kani_description.contains("verify_debug_set_renders_entries_in_braces"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_struct_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DebugStruct<'static, 'static>>").expect("DebugStruct's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DebugStruct<'static, 'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DebugStruct");
    assert!(kani_description.contains("verify_debug_struct_renders_named_fields"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_tuple_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<DebugTuple<'static, 'static>>").expect("DebugTuple's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<DebugTuple<'static, 'static>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for DebugTuple");
    assert!(kani_description.contains("verify_debug_tuple_renders_positional_fields"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_from_fn_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>").expect("FromFn's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FromFn");
    assert!(kani_description.contains("verify_from_fn_forwards_display_to_the_supplied_closure"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Result<i32, i32>>").expect("Result<i32, i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Result<i32, i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Result<i32, i32>");
    assert!(kani_description.contains("verify_result_ok_and_err_are_disjoint"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Result<i32, i32>");
    assert!(creusot_description.contains("verify_result_ok_and_err_are_disjoint"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn option_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::option::Iter<'static, i32>>").expect("core::option::Iter<'static, i32>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::option::Iter<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::option::Iter<'static, i32>");
    assert!(kani_description.contains("verify_option_iter_yields_zero_or_one_reference"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::option::Iter<'static, i32>");
    assert!(creusot_description.contains("verify_option_iter_yields_zero_or_one_reference"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn option_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::option::IterMut<'static, i32>>").expect("core::option::IterMut<'static, i32>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::option::IterMut<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::option::IterMut<'static, i32>");
    assert!(kani_description.contains("verify_option_iter_mut_writes_through_to_the_option"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::option::IterMut<'static, i32>");
    assert!(creusot_description.contains("verify_option_iter_mut_writes_through_to_the_option"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_iter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::result::Iter<'static, i32>>").expect("core::result::Iter<'static, i32>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::result::Iter<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::result::Iter<'static, i32>");
    assert!(kani_description.contains("verify_result_iter_yields_a_reference_to_the_ok_value"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::result::Iter<'static, i32>");
    assert!(creusot_description.contains("verify_result_iter_yields_a_reference_to_the_ok_value"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<core::result::IterMut<'static, i32>>").expect("core::result::IterMut<'static, i32>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<core::result::IterMut<'static, i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for core::result::IterMut<'static, i32>");
    assert!(kani_description.contains("verify_result_iter_mut_writes_through_to_the_result"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for core::result::IterMut<'static, i32>");
    assert!(creusot_description.contains("verify_result_iter_mut_writes_through_to_the_result"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn pending_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Pending<i32>>").expect("Pending<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Pending<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Pending<i32>");
    assert!(kani_description.contains("verify_pending_never_resolves"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Pending<i32>");
    assert!(creusot_description.contains("verify_pending_never_resolves"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn poll_fn_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>").expect("PollFn<fn(&mut Context<'_>) -> Poll<i32>>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for PollFn<fn(&mut Context<'_>) -> Poll<i32>>");
    assert!(kani_description.contains("verify_poll_fn_dispatches_through_to_its_closure"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for PollFn<fn(&mut Context<'_>) -> Poll<i32>>");
    assert!(creusot_description.contains("verify_poll_fn_dispatches_through_to_its_closure"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ready_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Ready<i32>>").expect("Ready<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Ready<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Ready<i32>");
    assert!(kani_description.contains("verify_ready_resolves_immediately_with_its_value"));

    let (_, creusot_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .expect("creusot proof registered for Ready<i32>");
    assert!(creusot_description.contains("verify_ready_resolves_immediately_with_its_value"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_chain_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>").expect("Chain's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Chain");
    assert!(kani_description.contains("verify_chain_sequences_two_iterators_end_to_end"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_cloned_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Cloned<Iter<'static, i32>>>").expect("Cloned's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Cloned<Iter<'static, i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Cloned");
    assert!(kani_description.contains("verify_cloned_clones_each_referenced_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_copied_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Copied<Iter<'static, i32>>>").expect("Copied's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Copied<Iter<'static, i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Copied");
    assert!(kani_description.contains("verify_copied_copies_each_referenced_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_cycle_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Cycle<Range<i32>>>").expect("Cycle's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Cycle<Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Cycle");
    assert!(kani_description.contains("verify_cycle_repeats_its_sequence_forever"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_empty_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::iter::Empty<i32>>").expect("Empty's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::iter::Empty<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Empty");
    assert!(kani_description.contains("verify_empty_yields_nothing"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_enumerate_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Enumerate<Range<i32>>>").expect("Enumerate's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Enumerate<Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Enumerate");
    assert!(kani_description.contains("verify_enumerate_pairs_each_item_with_its_index"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_filter_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>").expect("Filter's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Filter");
    assert!(kani_description.contains("verify_filter_yields_only_items_matching_the_predicate"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_filter_map_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>").expect("FilterMap's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with(
        "RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>"
    ));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FilterMap");
    assert!(kani_description.contains("verify_filter_map_applies_and_filters_in_one_step"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_flat_map_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>").expect("FlatMap's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with(
        "RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>"
    ));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for FlatMap");
    assert!(kani_description.contains("verify_flat_map_flattens_each_generated_iterator"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_flatten_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Flatten<IntoIter<Range<i32>>>>").expect("Flatten's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Flatten<IntoIter<Range<i32>>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Flatten");
    assert!(kani_description.contains("verify_flatten_concatenates_the_inner_iterators"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_map_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>").expect("Map's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Map");
    assert!(kani_description.contains("verify_map_applies_its_closure_to_each_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_zip_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Zip<Range<i32>, Range<i32>>>").expect("Zip's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Zip<Range<i32>, Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Zip");
    assert!(kani_description.contains("verify_zip_pairs_items_from_two_iterators"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_rev_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Rev<Range<i32>>>").expect("Rev's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Rev<Range<i32>>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Rev");
    assert!(kani_description.contains("verify_rev_reverses_iteration_order"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_fuse_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Fuse<Range<i32>>>").expect("Fuse's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Fuse<Range<i32>>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Fuse");
    assert!(kani_description.contains("verify_fuse_keeps_returning_none_once_exhausted"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_inspect_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Inspect<Range<i32>, fn(&i32)>>").expect("Inspect's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Inspect<Range<i32>, fn(&i32)>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Inspect");
    assert!(
        kani_description.contains("verify_inspect_calls_once_per_item_without_changing_values")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_peekable_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Peekable<Range<i32>>>").expect("Peekable's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Peekable<Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Peekable");
    assert!(kani_description.contains("verify_peekable_peek_does_not_consume"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_scan_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>").expect("Scan's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Scan");
    assert!(kani_description.contains("verify_scan_threads_state_through_its_closure"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_skip_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Skip<Range<i32>>>").expect("Skip's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<Skip<Range<i32>>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Skip");
    assert!(kani_description.contains("verify_skip_discards_the_first_n_items"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_skip_while_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>").expect("SkipWhile's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for SkipWhile");
    assert!(
        kani_description.contains("verify_skip_while_discards_items_while_the_predicate_holds")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_step_by_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<StepBy<Range<i32>>>").expect("StepBy's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<StepBy<Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for StepBy");
    assert!(kani_description.contains("verify_step_by_yields_every_nth_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_take_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::iter::Take<Range<i32>>>").expect("Take's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::iter::Take<Range<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Take");
    assert!(kani_description.contains("verify_take_yields_at_most_n_items"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_take_while_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>").expect("TakeWhile's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for TakeWhile");
    assert!(kani_description.contains("verify_take_while_yields_items_while_the_predicate_holds"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_map_while_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>").expect("MapWhile's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for MapWhile");
    assert!(
        kani_description.contains("verify_map_while_maps_items_while_the_closure_returns_some")
    );

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_once_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::iter::Once<i32>>").expect("Once's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::iter::Once<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Once");
    assert!(kani_description.contains("verify_once_yields_exactly_one_value"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_once_with_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<OnceWith<fn() -> i32>>").expect("OnceWith's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<OnceWith<fn() -> i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for OnceWith");
    assert!(kani_description.contains("verify_once_with_calls_its_closure_exactly_once"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<std::iter::Repeat<i32>>").expect("Repeat's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<std::iter::Repeat<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Repeat");
    assert!(kani_description.contains("verify_repeat_yields_the_same_value_forever"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_with_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<RepeatWith<fn() -> i32>>").expect("RepeatWith's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<RepeatWith<fn() -> i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for RepeatWith");
    assert!(kani_description.contains("verify_repeat_with_calls_its_closure_once_per_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_n_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<RepeatN<i32>>").expect("RepeatN's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<RepeatN<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for RepeatN");
    assert!(kani_description.contains("verify_repeat_n_yields_the_value_exactly_n_times"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_successors_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>").expect("Successors' evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Successors");
    assert!(kani_description.contains("verify_successors_generates_from_the_previous_item"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_build_hasher_default_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<BuildHasherDefault<DefaultHasher>>").expect("BuildHasherDefault's evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<BuildHasherDefault<DefaultHasher>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for BuildHasherDefault");
    assert!(kani_description.contains("verify_build_hasher_default_produces_consistent_hashers"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_sip_hasher_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<SipHasher>").expect("SipHasher's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<SipHasher>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for SipHasher");
    assert!(kani_description.contains("verify_sip_hasher_produces_consistent_hashes"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn marker_phantom_data_i32_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<PhantomData<i32>>").expect("PhantomData<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<PhantomData<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn marker_phantom_pinned_proof_chain_registers_the_kani_and_creusot_proofs() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<PhantomPinned>").expect("PhantomPinned's evidence link is registered");

    let root = &report.root;
    assert!(root.evidence.ends_with("RustStdStandard<PhantomPinned>"));
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let verifiers: Vec<&str> = root.proofs.iter().map(|(verifier, _)| *verifier).collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mem_manually_drop_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<ManuallyDrop<i32>>").expect("ManuallyDrop<i32>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<ManuallyDrop<i32>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for ManuallyDrop<i32>");
    assert!(kani_description.contains("verify_manually_drop_derefs_and_into_inner_round_trip"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mem_discriminant_option_i32_proof_chain_reports_the_kani_and_creusot_harnesses() {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = amenable::proof_chain("RustStdStandard<Discriminant<Option<i32>>>").expect("Discriminant<Option<i32>>'s evidence link is registered");

    let root = &report.root;
    assert!(
        root.evidence
            .ends_with("RustStdStandard<Discriminant<Option<i32>>>")
    );
    assert!(root.is_root());
    assert_eq!(root.proofs.len(), 2);
    assert_eq!(report.verifiers.len(), 2);

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for Discriminant<Option<i32>>");
    assert!(kani_description.contains("verify_discriminant_identifies_variant_not_payload"));

    assert!(
        root.proofs
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
}

#[test]
fn unregistered_subject_yields_a_not_found_error() {
    match amenable::proof_chain("NoSuchEvidenceType") {
        Err(ChainError::NotFound { subject }) => assert_eq!(subject, "NoSuchEvidenceType"),
        other => panic!("expected ChainError::NotFound, got {other:?}"),
    }
}

#[test]
fn proof_chain_report_renders_as_human_readable_text() {
    let report =
        amenable::proof_chain("RustStdStandard<bool>").expect("bool's evidence link is registered");
    let rendered = report.to_string();

    assert!(rendered.starts_with("Proof chain for"));
    assert!(rendered.contains("complete for: "));
    assert!(rendered.contains("(root)"));
    assert!(rendered.contains("proof [kani]:"));
}

#[test]
fn calculation_over_two_arguments_fans_out_into_a_tree_that_bottoms_out_in_std() {
    // AddEvidence/Debit/Credit only have kani proofs — the default (auto
    // discover every verifier seen anywhere in the tree) would find
    // creusot/verus too (present on the RustStdStandard<i64> leaf) and
    // correctly refuse to return a report, so scope this to kani only.
    let report = amenable::proof_chain_for_verifiers("AddEvidence", Some(&["kani"]))
        .expect("AddEvidence's evidence link is registered and complete for kani");

    let root = &report.root;
    assert!(root.evidence.ends_with("AddEvidence"));
    assert!(!root.is_root());
    assert_eq!(
        root.bases.len(),
        2,
        "Debit and Credit are separate branches"
    );

    // Argument order is preserved (a: Debit, b: Credit), not whatever
    // order `inventory` happened to iterate registrations in.
    assert!(root.bases[0].evidence.ends_with("Debit"));
    assert!(root.bases[1].evidence.ends_with("Credit"));

    // Neither Debit nor Credit is itself a root: both are thin domain
    // wrappers around i64, so both bottom out in the same std standard.
    for branch in &root.bases {
        assert!(!branch.is_root());
        assert_eq!(branch.bases.len(), 1);
        assert_eq!(branch.proofs.len(), 1, "access proof, kani-only");

        let std_node = &branch.bases[0];
        assert!(std_node.evidence.ends_with("RustStdStandard<i64>"));
        assert!(std_node.is_root());
        assert_eq!(std_node.proofs.len(), 1, "kani-only trusted proof");
    }

    let (_, kani_description) = root
        .proofs
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .expect("kani proof registered for AddEvidence");
    assert!(kani_description.contains("add_impl_computes_exact_sum"));
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn calculation_chain_is_incomplete_for_creusot_and_verus() {
    for verifier in ["creusot", "verus"] {
        match amenable::proof_chain_for_verifiers("AddEvidence", Some(&[verifier])) {
            Err(ChainError::Incomplete {
                subject,
                required,
                gaps,
            }) => {
                assert!(subject.ends_with("AddEvidence"));
                assert_eq!(required, vec![verifier.to_string()]);

                // AddEvidence, Debit, and Credit all lack a proof for
                // this verifier; only the RustStdStandard<i64> leaf has
                // one, so exactly three gaps are expected.
                assert_eq!(gaps.len(), 3, "gaps for {verifier}: {gaps:?}");
                assert!(gaps.iter().any(|gap| gap.evidence.ends_with("AddEvidence")));
                assert!(gaps.iter().any(|gap| gap.evidence.ends_with("Debit")));
                assert!(gaps.iter().any(|gap| gap.evidence.ends_with("Credit")));
                assert!(gaps.iter().all(|gap| gap.verifier == verifier));
            }
            other => panic!("expected ChainError::Incomplete for {verifier}, got {other:?}"),
        }
    }
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn calculation_chain_with_no_verifier_filter_is_also_incomplete() {
    // Auto-discovery finds kani, creusot, and verus (all present on the
    // std leaf); AddEvidence/Debit/Credit only ever proved kani, so the
    // unscoped lookup must refuse to return a report rather than quietly
    // showing a chain that looks uniformly proven.
    match amenable::proof_chain("AddEvidence") {
        Err(ChainError::Incomplete { required, .. }) => {
            assert_eq!(required.len(), 3);
            assert!(required.iter().any(|v| v == "kani"));
            assert!(required.iter().any(|v| v == "creusot"));
            assert!(required.iter().any(|v| v == "verus"));
        }
        other => panic!("expected ChainError::Incomplete, got {other:?}"),
    }
}
