use amenable_kani::{KaniCompose, KaniFmt, KaniFormatAtom, KaniFormatLabel, KaniRenderedKind};

#[test]
fn arguments_render_the_display_atom_verbatim() {
    let atom = KaniFormatAtom::new('7', '7');
    let rendered = KaniFmt::arguments(&atom);

    assert_eq!(rendered.kind(), KaniRenderedKind::Arguments);
    assert_eq!(rendered.display_token(), Some('7'));
}

#[test]
fn debug_helpers_preserve_expected_punctuation_and_order() {
    let first = KaniFormatAtom::new('1', '1');
    let second = KaniFormatAtom::new('2', '2');
    let type_label = KaniFormatLabel::new('P');
    let field_label = KaniFormatLabel::new('x');
    let key_label = KaniFormatLabel::new('k');

    let debug_struct = KaniFmt::debug_struct_one_field(type_label, field_label, &first);
    assert_eq!(debug_struct.kind(), KaniRenderedKind::DebugStructOneField);
    assert_eq!(debug_struct.type_label(), Some(type_label));
    assert_eq!(debug_struct.field_label(), Some(field_label));
    assert_eq!(debug_struct.value_debug_token(), Some('1'));

    let debug_tuple = KaniFmt::debug_tuple_one_field(type_label, &first);
    assert_eq!(debug_tuple.kind(), KaniRenderedKind::DebugTupleOneField);
    assert_eq!(debug_tuple.type_label(), Some(type_label));
    assert_eq!(debug_tuple.value_debug_token(), Some('1'));

    let debug_list = KaniFmt::debug_list_two_entries(&first, &second);
    assert_eq!(debug_list.kind(), KaniRenderedKind::DebugListTwoEntries);
    assert_eq!(debug_list.first_debug_token(), Some('1'));
    assert_eq!(debug_list.second_debug_token(), Some('2'));

    let debug_set = KaniFmt::debug_set_two_entries(&first, &second);
    assert_eq!(debug_set.kind(), KaniRenderedKind::DebugSetTwoEntries);
    assert_eq!(debug_set.first_debug_token(), Some('1'));
    assert_eq!(debug_set.second_debug_token(), Some('2'));

    let debug_map = KaniFmt::debug_map_one_entry(key_label, &first);
    assert_eq!(debug_map.kind(), KaniRenderedKind::DebugMapOneEntry);
    assert_eq!(debug_map.key_debug_label(), Some(key_label));
    assert_eq!(debug_map.value_debug_token(), Some('1'));
}

#[test]
fn composed_format_atoms_keep_display_and_debug_views() {
    let atom = KaniFormatAtom::kani_depth2();

    assert_eq!(atom.display_token(), 'a');
    assert_eq!(atom.debug_token(), 'A');
}
