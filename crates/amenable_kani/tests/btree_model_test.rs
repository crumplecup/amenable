use amenable_kani::{KaniBTreeMap, KaniBTreeSet};

#[test]
fn ordered_btree_map_observes_low_then_high_and_preserves_removals() {
    let mut map = KaniBTreeMap::new(9, 90, 4, 40);

    assert_eq!(map.first_entry(), Some((&4, &40)));
    assert_eq!(map.second_entry(), Some((&9, &90)));
    assert_eq!(map.remove(&4), Some(40));
    assert_eq!(map.remove(&9), Some(90));
    assert!(map.is_empty());
}

#[test]
fn ordered_btree_set_observes_low_then_high_and_preserves_removals() {
    let mut set = KaniBTreeSet::new(9, 4);

    assert_eq!(set.first_item(), Some(&4));
    assert_eq!(set.second_item(), Some(&9));
    assert!(set.remove(&4));
    assert!(set.remove(&9));
    assert!(set.is_empty());
}
