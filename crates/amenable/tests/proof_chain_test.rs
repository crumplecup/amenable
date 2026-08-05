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
