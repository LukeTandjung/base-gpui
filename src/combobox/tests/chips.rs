use crate::combobox::tests::support::multiple_runtime;
use crate::combobox::{ComboboxChipMoveOutcome, ComboboxMove};

#[test]
fn chip_highlight_moves_between_chips_and_returns_to_input_past_ends() {
    let mut runtime = multiple_runtime(vec!["apple", "banana", "blueberry"], "", false);

    // From the input, Previous highlights the last chip.
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Previous),
        ComboboxChipMoveOutcome::Chip(2)
    );
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Previous),
        ComboboxChipMoveOutcome::Chip(1)
    );
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Next),
        ComboboxChipMoveOutcome::Chip(2)
    );
    // Past the last chip returns to the input.
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Next),
        ComboboxChipMoveOutcome::Input
    );
    assert_eq!(runtime.highlighted_chip_index(), None);
}

#[test]
fn chip_highlight_past_first_returns_to_input() {
    let mut runtime = multiple_runtime(vec!["apple", "banana"], "", false);
    runtime.highlight_chip(Some(0));
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Previous),
        ComboboxChipMoveOutcome::Input
    );
}

#[test]
fn chip_move_with_no_chips_reports_no_chips() {
    let mut runtime = multiple_runtime(Vec::new(), "", false);
    assert_eq!(
        runtime.move_chip_highlight(ComboboxMove::Previous),
        ComboboxChipMoveOutcome::NoChips
    );
}

#[test]
fn remove_chip_reports_next_values_and_adjacent_highlight() {
    let runtime = multiple_runtime(vec!["apple", "banana", "blueberry"], "", false);

    let (values, highlight) = runtime.request_remove_chip(1).expect("chip exists");
    assert_eq!(values, vec!["apple", "blueberry"]);
    assert_eq!(highlight, Some(1));

    let (values, highlight) = runtime.request_remove_chip(2).expect("chip exists");
    assert_eq!(values, vec!["apple", "banana"]);
    assert_eq!(highlight, Some(1));

    assert!(runtime.request_remove_chip(5).is_none());
}

#[test]
fn remove_last_pops_the_most_recent_value() {
    let runtime = multiple_runtime(vec!["apple", "banana"], "", false);
    assert_eq!(runtime.request_remove_last(), Some(vec!["apple"]));

    let runtime = multiple_runtime(Vec::new(), "", false);
    assert_eq!(runtime.request_remove_last(), None);
}

#[test]
fn removing_a_highlighted_list_item_clears_the_list_highlight() {
    let mut runtime = multiple_runtime(vec!["apple", "banana"], "", true);
    runtime.highlight_item(Some(1), false);
    runtime.clear_highlight_for_removed(&"banana");
    assert_eq!(runtime.highlighted_index(), None);

    runtime.highlight_item(Some(0), false);
    runtime.clear_highlight_for_removed(&"banana");
    assert_eq!(runtime.highlighted_index(), Some(0));
}

#[test]
fn chip_highlight_reclamps_when_values_shrink() {
    let mut runtime = multiple_runtime(vec!["apple", "banana", "blueberry"], "", false);
    runtime.highlight_chip(Some(2));
    runtime.reconcile_multiple(vec!["apple"], false);
    assert_eq!(runtime.highlighted_chip_index(), Some(0));

    runtime.reconcile_multiple(Vec::new(), false);
    assert_eq!(runtime.highlighted_chip_index(), None);
}

#[test]
fn chip_labels_follow_selection_order() {
    let runtime = multiple_runtime(vec!["banana", "apple"], "", false);
    assert_eq!(runtime.chip_label(0), gpui::SharedString::from("Banana"));
    assert_eq!(runtime.chip_label(1), gpui::SharedString::from("Apple"));
}
