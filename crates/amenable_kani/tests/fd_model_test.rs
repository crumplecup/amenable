use amenable_kani::{KaniFd, KaniFile};

#[test]
fn borrow_preserves_raw_fd_and_resource_identity() {
    let owned = KaniFd::live(7, 19);
    let borrowed = owned.borrow();

    assert_eq!(borrowed.raw_fd(), owned.raw_fd());
    assert_eq!(borrowed.resource_id(), owned.resource_id());
    assert!(borrowed.is_live());
}

#[test]
fn file_into_owned_preserves_raw_fd_and_resource_identity() {
    let owned = KaniFd::live(11, 23);
    let file = KaniFile::from_owned_fd(owned);
    let raw_before = file.raw_fd();
    let resource_before = file.resource_id();

    let transferred = file.into_owned_fd();

    assert_eq!(transferred.raw_fd(), raw_before);
    assert_eq!(transferred.resource_id(), resource_before);
    assert!(transferred.is_live());
}
