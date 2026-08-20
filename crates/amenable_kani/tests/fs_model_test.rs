use amenable_kani::{
    KaniCreateNewObservation, KaniFileContentObservation, KaniFileLenObservation, KaniFileSystem,
    KaniFileTimesObservation, KaniFileTypeObservation, KaniFsLabel, KaniFsPath,
    KaniLockObservation, KaniPermissionsObservation, KaniReadDirObservation,
};

#[test]
fn recursive_directory_creation_adds_each_missing_ancestor() {
    let mut filesystem = KaniFileSystem::new();
    let base = KaniFsPath::root();
    let a = base.join(KaniFsLabel::new('a'));
    let b = a.join(KaniFsLabel::new('b'));
    let c = b.join(KaniFsLabel::new('c'));

    filesystem.create_dir_all(&c);

    assert!(filesystem.is_dir(&a));
    assert!(filesystem.is_dir(&b));
    assert!(filesystem.is_dir(&c));
}

#[test]
fn directory_entries_report_their_name_and_full_path() -> miette::Result<()> {
    let mut filesystem = KaniFileSystem::new();
    let base = KaniFsPath::root().join(KaniFsLabel::new('b'));
    let path = base.join(KaniFsLabel::new('f'));

    filesystem.create_dir_all(&base);
    filesystem.create_file(&path);

    let entry = filesystem
        .entries(&base)
        .into_iter()
        .find(|entry| entry.file_name() == Some(KaniFsLabel::new('f')))
        .ok_or_else(|| miette::miette!("modeled directory entry should exist"))?;

    assert_eq!(entry.file_name(), Some(KaniFsLabel::new('f')));
    assert_eq!(entry.path(), path);
    Ok(())
}

#[test]
fn file_type_distinguishes_a_file_from_a_directory() {
    let observation = KaniFileTypeObservation::new();

    assert!(observation.file_is_file());
    assert!(!observation.file_is_dir());
    assert!(observation.directory_is_dir());
    assert!(!observation.directory_is_file());
}

#[test]
fn file_content_round_trips_the_written_bytes() {
    let bytes = [1, 2, 3, 4];
    let observation = KaniFileContentObservation::write(bytes);

    assert_eq!(observation.read(), bytes);
}

#[test]
fn file_len_reports_the_written_byte_count() {
    let observation = KaniFileLenObservation::write(11);

    assert_eq!(observation.len(), 11);
    assert!(!observation.is_empty());
    assert!(KaniFileLenObservation::write(0).is_empty());
}

#[test]
fn file_times_round_trips_the_set_modification_time() {
    let observation = KaniFileTimesObservation::set_modified(1_700_000_000);

    assert_eq!(observation.modified(), 1_700_000_000);
}

#[test]
fn create_new_rejects_an_existing_path_and_accepts_a_fresh_one() {
    let mut existing_file = KaniCreateNewObservation::existing_file();
    let mut existing_directory = KaniCreateNewObservation::existing_directory();
    let mut fresh = KaniCreateNewObservation::missing();

    assert!(existing_file.create_new().is_err());
    assert!(existing_directory.create_new().is_err());
    assert!(fresh.create_new().is_ok());
    assert!(fresh.is_file());
}

#[test]
fn permissions_readonly_round_trips_through_set_readonly() {
    let mut observation = KaniPermissionsObservation::new();
    assert!(!observation.readonly());

    observation.set_readonly(true);
    assert!(observation.readonly());

    observation.set_readonly(false);
    assert!(!observation.readonly());
}

#[test]
fn read_dir_observation_reports_both_created_entries_in_order() {
    let base = KaniFsPath::root().join(KaniFsLabel::new('d'));
    let one = KaniFsLabel::new('1');
    let two = KaniFsLabel::new('2');
    let observation = KaniReadDirObservation::new(base, one, two);
    let entries = observation.entries();

    assert_eq!(entries[0].file_name(), Some(one));
    assert_eq!(entries[1].file_name(), Some(two));
}

#[test]
fn lock_observation_rejects_a_second_try_lock_while_held() {
    let mut observation = KaniLockObservation::new();

    assert!(observation.try_lock().is_ok());
    assert!(observation.try_lock().is_err());
}
