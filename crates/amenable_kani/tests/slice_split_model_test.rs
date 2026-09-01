use amenable_kani::{KaniChunkByObservation, KaniSplitNObservationBuilder, KaniSplitObservation};
use miette::IntoDiagnostic;

#[test]
fn split_observation_recovers_split_inclusive_and_reverse_views() {
    let observation = KaniSplitObservation::new(1, 0, 2);

    assert_eq!(observation.split(), ([1], [2]));
    assert_eq!(observation.split_inclusive(), ([1, 0], [2]));
    assert_eq!(observation.rsplit(), ([2], [1]));
}

#[test]
fn split_observation_write_through_updates_the_underlying_data() {
    let mut observation = KaniSplitObservation::new(1, 0, 2);

    observation = observation.with_before(10);
    observation = observation.with_after(20);

    assert_eq!(observation.data(), [10, 0, 20]);
}

#[test]
fn split_n_observation_preserves_the_capped_second_piece() -> miette::Result<()> {
    let observation = KaniSplitNObservationBuilder::default()
        .first(1)
        .first_delimiter(0)
        .middle(2)
        .second_delimiter(0)
        .last(3)
        .build()
        .into_diagnostic()?;

    assert_eq!(observation.splitn_two(), ([1], [2, 0, 3]));
    assert_eq!(observation.rsplitn_two(), ([3], [1, 0, 2]));

    Ok(())
}

#[test]
fn chunk_by_observation_distinguishes_grouped_from_split_pairs() {
    let grouped = KaniChunkByObservation::new(1, 3, true);
    let split = KaniChunkByObservation::new(1, 2, false);

    assert!(grouped.grouped_together());
    assert_eq!(grouped.first_chunk_len(), 2);
    assert_eq!(grouped.trailing_chunk_len(), None);
    assert_eq!(grouped.first(), 1);
    assert_eq!(grouped.second(), 3);

    assert!(!split.grouped_together());
    assert_eq!(split.first_chunk_len(), 1);
    assert_eq!(split.trailing_chunk_len(), Some(1));
    assert_eq!(split.first(), 1);
    assert_eq!(split.second(), 2);
}
