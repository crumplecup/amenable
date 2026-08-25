use amenable_kani::{KaniArgv, KaniEnvPath, KaniEnvPathList, KaniEnvPaths};

#[test]
fn argv_always_counts_the_program_slot() {
    // Not KaniArgv::kani_depth0(): this test is about KaniArgv's own
    // accounting, not about KaniCompose (a Kani-only trait -- see
    // docs/KANI_COMPOSE_PLAN.md's "Scope Correction"), so it builds the
    // same zero-extra-args shape through KaniArgv's own public
    // constructor instead of reaching into Kani-only infrastructure.
    let argv = KaniArgv::new(String::new(), 0);

    assert_eq!(argv.args_count(), 1);
    assert_eq!(argv.args_os_count(), 1);
    assert_eq!(argv.extra_count(), 0);
}

#[test]
fn argv_keeps_the_program_slot_and_extra_count() {
    let argv = KaniArgv::new("program".to_owned(), 2);

    assert_eq!(argv.program_slot(), "program");
    assert_eq!(argv.extra_count(), 2);
    assert_eq!(argv.args_count(), 3);
    assert_eq!(argv.args_os_count(), 3);
}

#[test]
fn env_paths_join_with_the_modeled_separator() -> miette::Result<()> {
    let paths = KaniEnvPathList::from_strings(vec!["one".to_owned(), "two".to_owned()])
        .map_err(|error| miette::miette!("{error:?}"))?;

    let joined = KaniEnvPaths::join(&paths);
    let expected = if cfg!(windows) { "one;two" } else { "one:two" };
    assert_eq!(joined, expected);
    Ok(())
}

#[test]
fn env_paths_reject_an_unjoinable_string() {
    let bad_path = if cfg!(windows) { "a\"b" } else { "a:b" }.to_owned();
    let err = KaniEnvPath::new(bad_path.clone()).unwrap_err();

    assert_eq!(err.offending_path(), &bad_path);
    assert_eq!(err.into_offending_path(), bad_path);
}

#[test]
fn split_recovers_a_joined_modeled_path_list() -> miette::Result<()> {
    let paths =
        KaniEnvPathList::from_strings(vec!["one".to_owned(), "two".to_owned(), "three".to_owned()])
            .map_err(|error| miette::miette!("{error:?}"))?;

    let joined = KaniEnvPaths::join(&paths);
    let split = KaniEnvPaths::split(&joined);

    assert_eq!(split.len(), 3);
    assert!(!split.is_empty());
    assert_eq!(
        split.as_strings(),
        vec!["one".to_owned(), "two".to_owned(), "three".to_owned()]
    );
    Ok(())
}

#[test]
fn semantic_join_and_split_round_trip_the_modeled_path_list() -> miette::Result<()> {
    let paths =
        KaniEnvPathList::from_strings(vec!["one".to_owned(), "two".to_owned(), "three".to_owned()])
            .map_err(|error| miette::miette!("{error:?}"))?;

    let split = KaniEnvPaths::split_semantic(KaniEnvPaths::join_semantic(paths));

    assert_eq!(split.len(), 3);
    assert_eq!(split.paths()[0].as_str(), "one");
    assert_eq!(split.paths()[1].as_str(), "two");
    assert_eq!(split.paths()[2].as_str(), "three");
    Ok(())
}
