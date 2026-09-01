use amenable_kani::{
    KaniStrMatchObservationBuilder, KaniStrRSplitNObservationBuilder, KaniStrRSplitObservation,
    KaniStrSplitTerminatorObservation,
};
use miette::IntoDiagnostic;

#[test]
fn rsplit_observation_reverses_the_before_after_order() {
    let observation = KaniStrRSplitObservation::new('a', ',', 'b');

    assert_eq!(observation.rsplit(), ['b', 'a']);
    assert_eq!(observation.data(), ['a', ',', 'b']);
}

#[test]
fn rsplitn_observation_keeps_the_uncut_leading_piece() -> miette::Result<()> {
    let observation = KaniStrRSplitNObservationBuilder::default()
        .a('a')
        .pattern(',')
        .b('b')
        .c('c')
        .build()
        .into_diagnostic()?;

    assert_eq!(observation.rsplitn_two(), ('c', ['a', ',', 'b']));

    Ok(())
}

#[test]
fn split_terminator_observation_suppresses_the_trailing_empty_piece() {
    let observation = KaniStrSplitTerminatorObservation::new('a', '.', 'b');

    assert_eq!(observation.split_terminator(), ['a', 'b']);
    assert_eq!(observation.rsplit_terminator(), ['b', 'a']);
}

#[test]
fn match_observation_pairs_each_occurrence_with_its_byte_offset() -> miette::Result<()> {
    let observation = KaniStrMatchObservationBuilder::default()
        .f0('x')
        .pattern('a')
        .f1('y')
        .f2('z')
        .build()
        .into_diagnostic()?;

    assert_eq!(observation.matches(), ['a', 'a']);
    assert_eq!(observation.rmatches(), ['a', 'a']);
    assert_eq!(observation.match_indices(), [(1, 'a'), (3, 'a')]);
    assert_eq!(observation.rmatch_indices(), [(3, 'a'), (1, 'a')]);

    Ok(())
}
