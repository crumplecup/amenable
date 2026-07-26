use amenable_kani::KaniLinkedListExtractIf;

fn is_even(x: &mut i32) -> bool {
    *x % 2 == 0
}

#[test]
fn full_traversal_yields_matches_and_leaves_non_matches_in_order() {
    let mut extractor = KaniLinkedListExtractIf::new(vec![1, 2, 3, 4]);

    assert_eq!(extractor.next(is_even as fn(&mut i32) -> bool), Some(2));
    assert_eq!(extractor.next(is_even as fn(&mut i32) -> bool), Some(4));
    assert_eq!(extractor.next(is_even as fn(&mut i32) -> bool), None);
    assert_eq!(extractor.into_remaining(), vec![1, 3]);
}

#[test]
fn early_drop_retains_the_unvisited_suffix() {
    let mut extractor = KaniLinkedListExtractIf::new(vec![1, 2, 3, 4]);

    assert_eq!(extractor.next(is_even as fn(&mut i32) -> bool), Some(2));
    assert_eq!(extractor.into_remaining(), vec![1, 3, 4]);
}
