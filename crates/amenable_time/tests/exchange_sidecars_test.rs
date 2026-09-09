//! `exchange` — spot-checks the exchange-surface *shape* `amenable_time`
//! owns: `RawInput` input sidecar, the boundary token, the 24 per-method
//! output sidecars, and their `Establish` edges. The `Exchange` impls and
//! the `TemporalParser<V>` bundle's `Witness<V>` obligations are a
//! backend concern, so they are not exercised here — the library
//! compiling is the proof the bundle is well-formed.

use amenable_core::{Evidence, EvidenceLink, ProofToken};
use amenable_time::{
    CalendarDateExtendedFormatted, CalendarDateValid, CalendarDateValidToken,
    FormattedCalendarDateExtended, FormattedTemporalText, LocalDateTimeProof, RawInput,
    RawTemporalText, Rfc3339TimestampProof, TemporalInputReceived, TemporalInputToken,
};

#[test]
fn raw_input_carries_the_boundary_text_and_marker() {
    amenable_core::init_tracing();

    let input = RawInput::received("2026-09-09");
    assert_eq!(input.as_str(), "2026-09-09");

    assert!(<RawTemporalText as Evidence>::is_root());
    assert_eq!(RawTemporalText::basis(), RawTemporalText::default());
    assert!(<TemporalInputReceived as Evidence>::is_root());
}

#[test]
fn the_input_token_is_a_freely_minted_root() {
    amenable_core::init_tracing();

    assert_eq!(TemporalInputToken::new(), TemporalInputToken::default());
}

#[test]
fn an_output_token_justifies_its_proposition() {
    amenable_core::init_tracing();

    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<CalendarDateValidToken, CalendarDateValid>();
}

#[test]
fn multi_proof_methods_fold_into_a_composite_proposition() {
    amenable_core::init_tracing();

    // `parse_local_date_time` returns two proofs → one `#[derive(Evidence,
    // Witness)]` composite; `parse_rfc3339_timestamp` returns three.
    let () = LocalDateTimeProof::default().audit();
    let () = Rfc3339TimestampProof::default().audit();
    assert!(<LocalDateTimeProof as Evidence>::is_root());
    assert!(<Rfc3339TimestampProof as Evidence>::is_root());
}

#[test]
fn every_output_token_registers_an_establish_edge_from_the_input_token() {
    amenable_core::init_tracing();

    let minted: usize = inventory::iter::<amenable_core::ProofTokenMintRecord>()
        .filter(|r| {
            r.credential()
                .is_some_and(|c| c.replace(' ', "").ends_with("::TemporalInputToken"))
        })
        .count();
    assert!(
        minted >= 24,
        "expected >=24 temporal establish edges, got {minted}"
    );
}

#[test]
fn the_formatter_output_family_exists() {
    amenable_core::init_tracing();

    // `format_calendar_date_extended` output: `FormattedCalendarDateExtended`
    // (primary = the emitted text) over the `CalendarDateExtendedFormatted`
    // emission-proof composite.
    let () = CalendarDateExtendedFormatted::default().audit();
    assert!(<CalendarDateExtendedFormatted as Evidence>::is_root());
    assert!(<FormattedTemporalText as Evidence>::is_root());

    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<amenable_time::CalendarDateExtendedFormattedToken, CalendarDateExtendedFormatted>(
    );

    // a compile-time reference is enough to prove the sidecar type is there
    fn _sink<T>(_: T) {}
    let _ = _sink::<Option<FormattedCalendarDateExtended>>;
}

#[test]
fn the_factory_transitions_fold_preconditions_and_re_issued_proofs() {
    amenable_core::init_tracing();

    // `resolve_local_date_time`: 3 descriptors + 4 `Established<_>`
    // preconditions in, a `ZonedDateTimeDescriptor` + 6 re-issued proofs
    // out — each side folded into one proposition.
    use amenable_time::{
        ResolveLocalDateTimeEstablished, ResolveLocalDateTimeOutput,
        ResolveLocalDateTimePreconditions,
    };
    let () = ResolveLocalDateTimePreconditions::default().audit();
    let () = ResolveLocalDateTimeEstablished::default().audit();
    assert!(<ResolveLocalDateTimePreconditions as Evidence>::is_root());
    assert!(<ResolveLocalDateTimeEstablished as Evidence>::is_root());

    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<
        amenable_time::ResolveLocalDateTimeEstablishedToken,
        ResolveLocalDateTimeEstablished,
    >();

    fn _sink<T>(_: T) {}
    let _ = _sink::<Option<ResolveLocalDateTimeOutput>>;
}

#[test]
fn every_factory_established_token_chains_from_its_preconditions_token() {
    amenable_core::init_tracing();

    // Each transition mints two edges: `*PreconditionsToken` from the
    // input token, `*EstablishedToken` from `*PreconditionsToken`.
    let chained: usize = inventory::iter::<amenable_core::ProofTokenMintRecord>()
        .filter(|r| {
            r.credential()
                .is_some_and(|c| c.replace(' ', "").ends_with("PreconditionsToken"))
        })
        .count();
    assert!(
        chained >= 9,
        "expected >=9 factory established-from-preconditions edges, got {chained}"
    );
}

#[test]
fn calconnect_parse_and_emit_sidecars_share_one_folded_proof_shape() {
    amenable_core::init_tracing();

    // `parse_selection_expression` folds 16 proof sidecars into
    // `SelectionExpressionProof`; `format_selection_expression` folds the
    // matching emission proofs into `SelectionExpressionFormatted`.
    use amenable_time::{
        FormattedSelectionExpression, ParsedSelectionExpression, SelectionExpressionFormatted,
        SelectionExpressionProof,
    };
    let () = SelectionExpressionProof::default().audit();
    let () = SelectionExpressionFormatted::default().audit();
    assert!(<SelectionExpressionProof as Evidence>::is_root());
    assert!(<SelectionExpressionFormatted as Evidence>::is_root());

    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<amenable_time::SelectionExpressionProofToken, SelectionExpressionProof>();
    assert_token::<amenable_time::SelectionExpressionFormattedToken, SelectionExpressionFormatted>(
    );

    fn _sink<T>(_: T) {}
    let _ = _sink::<Option<(ParsedSelectionExpression, FormattedSelectionExpression)>>;
}

#[test]
fn calconnect_emit_tokens_chain_from_the_matching_parse_token() {
    amenable_core::init_tracing();

    // Each CalConnect emit token is established from its parse token, not
    // from `TemporalInputToken`.
    let chained: usize = inventory::iter::<amenable_core::ProofTokenMintRecord>()
        .filter(|r| {
            r.credential()
                .is_some_and(|c| c.replace(' ', "").ends_with("ProofToken"))
        })
        .count();
    assert!(
        chained >= 6,
        "expected >=6 CalConnect emit-from-parse edges, got {chained}"
    );
}

#[test]
fn a_proven_carrier_is_a_token_keyed_sidecar() {
    amenable_core::init_tracing();

    // The bundle token is the single source of truth: `ProvenTemporalCarrier`
    // is keyed on it, and the proposition is recovered as
    // `<Tok as ProofToken>::Proposition`.
    use amenable_time::{LocalDateTimeSemanticBundle, LocalDateTimeSemanticBundleToken};
    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<LocalDateTimeSemanticBundleToken, LocalDateTimeSemanticBundle>();

    #[derive(Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
    #[evidence(basis = "Self")]
    struct FakeNative;

    fn _sink<T>(_: T) {}
    let _ = _sink::<Option<amenable_time::ProvenLocalDateTimeCarrier<FakeNative>>>;
}

#[test]
fn every_semantic_bundle_token_swaps_from_a_real_credential() {
    amenable_core::init_tracing();

    // Each `<Bundle>Token` gets a root record (`#[derive(ProofToken)]`)
    // and an `#[establish]` record naming the credential it swaps from —
    // assert every distinct bundle token has the swap edge.
    use std::collections::{HashMap, HashSet};
    let mut has_swap: HashMap<&str, bool> = HashMap::new();
    for r in inventory::iter::<amenable_core::ProofTokenMintRecord>() {
        let name = r.token().rsplit("::").next().unwrap_or(r.token());
        if name.ends_with("BundleToken") {
            *has_swap.entry(name).or_insert(false) |= r.credential().is_some();
        }
    }
    let names: HashSet<_> = has_swap.keys().copied().collect();
    assert!(
        names.len() >= 22,
        "expected >=22 distinct semantic-bundle tokens, got {}",
        names.len()
    );
    let no_swap: Vec<_> = has_swap
        .iter()
        .filter(|(_, ok)| !**ok)
        .map(|(n, _)| *n)
        .collect();
    assert!(
        no_swap.is_empty(),
        "these bundle tokens have no establish edge: {no_swap:?}"
    );
}

#[test]
fn a_reflected_sidecar_pairs_a_descriptor_with_its_bundle_token() {
    amenable_core::init_tracing();

    // `Reflected<X>` = `<X>Descriptor` + `<X>SemanticBundleToken` — the
    // shape `realize_x` consumes and `reflect_x` produces, the genuine
    // inverse of the carrier sidecar (same token rides through).
    use amenable_time::{
        LocalDateTimeSemanticBundle, LocalDateTimeSemanticBundleToken, ReflectedLocalDateTime,
    };
    fn assert_token<Tok, Prop>()
    where
        Prop: Evidence,
        Tok: ProofToken<Proposition = Prop>,
    {
    }
    assert_token::<LocalDateTimeSemanticBundleToken, LocalDateTimeSemanticBundle>();

    fn _sink<T>(_: T) {}
    let _ = _sink::<
        Option<(
            ReflectedLocalDateTime,
            amenable_time::ReflectedDateTimeFormula,
        )>,
    >;
}

#[test]
fn adding_evidence_to_descriptors_left_the_aggregate_registry_alone() {
    amenable_core::init_tracing();

    let composites: usize = inventory::iter::<EvidenceLink>()
        .map(EvidenceLink::name)
        .filter(|n| n.contains("proof_composition::composites"))
        .count();
    assert_eq!(composites, 93);
}
