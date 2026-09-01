#![cfg(feature = "verus")]

use amenable_core::{
    ContractRecord, Ensures, Evidence, MetadataEntry, Provenance, Requires, Verifier, Witness,
    WitnessSupportSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct VerusVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

struct CharRoundtrip;

impl Evidence for CharRoundtrip {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for CharRoundtrip {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

amenable_derive::verus_ensures_witness!(
    CharRoundtrip,
    "verus_contract_test::CharRoundtrip",
    "verify_char_roundtrip"
);

struct ValidUnicodeScalarLike;

impl Evidence for ValidUnicodeScalarLike {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for ValidUnicodeScalarLike {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

// Reuses verify_char_roundtrip's harness like CharRoundtrip above, but
// claims only its second real clause (the unicode-scalar one, not the
// roundtrip one) -- the subset-index feature real ValidUnicodeScalar
// needs.
amenable_derive::verus_ensures_witness!(
    ValidUnicodeScalarLike,
    "verus_contract_test::ValidUnicodeScalarLike",
    "verify_char_roundtrip",
    [1]
);

struct EscapeAsciiModel;

impl Evidence for EscapeAsciiModel {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for EscapeAsciiModel {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

amenable_derive::verus_requires_witness!(
    EscapeAsciiModel,
    "verus_contract_test::EscapeAsciiModel",
    "verify_escape_ascii_model_leaves_printable_bytes_unescaped"
);

struct WriteStoresNewValueLike;

impl Evidence for WriteStoresNewValueLike {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for WriteStoresNewValueLike {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

// write_stores_new_value is shared across Cell/RefCell/UnsafeCell/
// OrderedPair's own methods -- no single harness to derive from, so
// this derives from the predicate's own real declaration instead.
amenable_derive::verus_ensures_predicate!(
    WriteStoresNewValueLike,
    "verus_contract_test::WriteStoresNewValueLike",
    "write_stores_new_value"
);

struct MultiplePredicatesLike;

impl Evidence for MultiplePredicatesLike {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for MultiplePredicatesLike {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

// Two distinct, independently-declared real predicates -- like
// `IncrementHeadroom`'s tight/single/wide margin variants -- each
// contribute their own real clause, the same way a harness's own
// multiple clauses do.
amenable_derive::verus_ensures_predicate!(
    MultiplePredicatesLike,
    "verus_contract_test::MultiplePredicatesLike",
    [
        "char_roundtrip_preserves_value",
        "char_is_valid_unicode_scalar"
    ]
);

#[test]
fn ensures_bound_is_the_real_multi_clause_slice() {
    amenable_core::init_tracing();
    let clauses: &[&str] = CharRoundtrip::ensures(());

    assert_eq!(
        clauses,
        [
            "char_roundtrip_preserves_value(result, c)",
            "char_is_valid_unicode_scalar(c)",
        ]
    );
}

#[test]
fn requires_bound_is_the_real_clause_slice() {
    amenable_core::init_tracing();
    let clauses: &[&str] = EscapeAsciiModel::requires(());

    assert_eq!(
        clauses,
        ["escape_ascii_input_is_printable_ascii(printable)"]
    );
}

#[test]
fn indexed_subset_claims_only_the_selected_real_clause() {
    amenable_core::init_tracing();
    let clauses: &[&str] = ValidUnicodeScalarLike::ensures(());

    assert_eq!(clauses, ["char_is_valid_unicode_scalar(c)"]);
}

#[test]
fn predicate_witness_derives_from_the_predicates_own_declaration() {
    amenable_core::init_tracing();
    let clauses: &[&str] = WriteStoresNewValueLike::ensures(());

    assert_eq!(clauses, ["observed == new_value"]);
}

#[test]
fn predicate_registration_carries_a_real_fn_signature_not_the_bare_clause() -> miette::Result<()> {
    amenable_core::init_tracing();
    // `verus_ensures_witness!`'s own registration (`CharRoundtrip`, see
    // `one_contract_record_is_registered_per_real_clause` above) stays
    // the bare clause -- only `verus_ensures_predicate!`'s registration
    // differs, and only in the registered `ContractRecord.fragment()`,
    // never in the trait's own `Bound` (already covered above). A bare
    // clause with no `fn` token can never be recognized by `cordial`'s
    // real call-shape scanner (`fragment_fn_name`, `~/repos/cordial`) as
    // naming `write_stores_new_value` -- this is the literal defect that
    // left every `verus_ensures_predicate!`/`verus_requires_predicate!`
    // real site permanently unrecognized until this fix landed.
    let record = inventory::iter::<ContractRecord>()
        .find(|record| {
            record.evidence() == "verus_contract_test::WriteStoresNewValueLike"
                && record.kind() == "ensures"
        })
        .ok_or_else(|| {
            miette::miette!("WriteStoresNewValueLike registers a real ContractRecord")
        })?;
    let fragment = (record.fragment())();

    assert!(
        fragment.contains("fn write_stores_new_value"),
        "registered fragment {fragment:?} should carry the real predicate's own `fn` \
         signature, not just its bare clause"
    );
    assert!(
        fragment.contains("observed == new_value"),
        "registered fragment {fragment:?} should still carry the real clause body"
    );

    Ok(())
}

#[test]
fn bracketed_predicate_list_derives_one_clause_per_named_predicate() {
    amenable_core::init_tracing();
    let clauses: &[&str] = MultiplePredicatesLike::ensures(());

    assert_eq!(
        clauses,
        [
            "result == input",
            "(value as u32) <= 0xD7FFu32 || ((value as u32) >= 0xE000u32 && (value as u32) <= 0x10FFFFu32)"
        ]
    );
}

#[test]
fn one_contract_record_is_registered_per_real_clause() {
    amenable_core::init_tracing();
    // `inventory::iter` doesn't promise any particular order across
    // separate `submit!` blocks, so compare as sets, not sequences.
    // Both of `CharRoundtrip`'s real clauses are themselves bare calls
    // to a real, separately-shared spec fn -- `expand_verus_witness`
    // resolves that callee and wraps its own real `fn NAME(..) ->
    // ReturnType` signature around the clause (see `bare_call_name`'s
    // own doc comment), so the registered fragment carries both the
    // callee's real signature and the real call, not just the bare
    // call the trait's own `Bound` still returns unchanged (covered by
    // `ensures_bound_is_the_real_multi_clause_slice` above).
    let mut ensures_records = inventory::iter::<ContractRecord>()
        .filter(|record| {
            record.evidence() == "verus_contract_test::CharRoundtrip" && record.kind() == "ensures"
        })
        .map(|record| (record.fragment())())
        .collect::<Vec<_>>();
    ensures_records.sort_unstable();

    let mut expected_ensures = vec![
        "open spec fn char_roundtrip_preserves_value(result : char, input : char) -> bool { \
         char_roundtrip_preserves_value(result, c) }",
        "open spec fn char_is_valid_unicode_scalar(value : char) -> bool { \
         char_is_valid_unicode_scalar(c) }",
    ];
    expected_ensures.sort_unstable();
    assert_eq!(ensures_records, expected_ensures);

    let requires_records = inventory::iter::<ContractRecord>()
        .filter(|record| {
            record.evidence() == "verus_contract_test::EscapeAsciiModel"
                && record.kind() == "requires"
        })
        .map(|record| (record.fragment())())
        .collect::<Vec<_>>();

    assert_eq!(
        requires_records,
        vec![
            "open spec fn escape_ascii_input_is_printable_ascii(printable : u8) -> bool { \
             escape_ascii_input_is_printable_ascii(printable) }"
        ]
    );
}

#[test]
fn predicate_body_extracts_the_real_shared_predicate_declaration() -> miette::Result<()> {
    amenable_core::init_tracing();
    let (_, _, item_fn) =
        amenable_core::verus_find_fn("write_stores_new_value").ok_or_else(|| {
            miette::miette!("write_stores_new_value should be a real, public spec fn")
        })?;
    let body = amenable_core::verus_predicate_body(&item_fn)
        .map_err(|error| miette::miette!("write_stores_new_value {error}"))?;

    assert_eq!(body, "observed == new_value");
    Ok(())
}
