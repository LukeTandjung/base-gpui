use crate::autocomplete::tests::support::none_mode_runtime;
use crate::combobox::ComboboxMove;

#[test]
fn both_mode_filters_on_typed_query_while_overlay_is_active() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.commit_input_value("b".into());
    let filtered_before = runtime.filtered_indices();
    assert_eq!(filtered_before, vec![1, 3]); // Banana, Blueberry

    // Arrow through items with the overlay active: the list must not
    // collapse to the highlighted item's label.
    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();
    runtime.refilter();
    assert_eq!(runtime.filtered_indices(), filtered_before);

    runtime.move_highlight(ComboboxMove::Next, true);
    runtime.sync_inline_overlay();
    runtime.refilter();
    assert_eq!(runtime.filtered_indices(), filtered_before);
}

#[test]
fn static_modes_disable_internal_filtering() {
    let mut runtime = none_mode_runtime("b", true);
    runtime.sync_filtering(None, true, None, None, Default::default());
    runtime.refilter();
    assert_eq!(runtime.filtered_indices(), vec![0, 1, 2, 3]);
}
