use amenable_kani::{KaniFileSystem, KaniFsLabel, KaniFsPath};

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
fn directory_entries_report_their_name_and_full_path() {
    let mut filesystem = KaniFileSystem::new();
    let base = KaniFsPath::root().join(KaniFsLabel::new('b'));
    let path = base.join(KaniFsLabel::new('f'));

    filesystem.create_dir_all(&base);
    filesystem.create_file(&path);

    let entry = filesystem
        .entries(&base)
        .into_iter()
        .find(|entry| entry.file_name() == Some(KaniFsLabel::new('f')))
        .expect("modeled directory entry should exist");

    assert_eq!(entry.file_name(), Some(KaniFsLabel::new('f')));
    assert_eq!(entry.path(), path);
}
