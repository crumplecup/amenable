use amenable_kani::{KaniSplitNObservation, KaniSplitObservation};

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

    observation.set_before(10);
    observation.set_after(20);

    assert_eq!(observation.data(), [10, 0, 20]);
}

#[test]
fn split_n_observation_preserves_the_capped_second_piece() {
    let observation = KaniSplitNObservation::new(1, 0, 2, 0, 3);

    assert_eq!(observation.splitn_two(), ([1], [2, 0, 3]));
    assert_eq!(observation.rsplitn_two(), ([3], [1, 0, 2]));
}
