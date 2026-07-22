use gpui::SharedString;

use crate::autocomplete::tests::support::none_mode_runtime;
use crate::combobox::ComboboxSelectionChange;

#[test]
fn item_press_produces_no_selection_in_none_mode() {
    let runtime = none_mode_runtime("", true);
    let outcome = runtime.request_select_value(None, Vec::new(), "banana", false, false, false);
    assert!(matches!(outcome, ComboboxSelectionChange::None));
    assert!(!runtime.is_value_selected(&"banana"));
}

#[test]
fn fill_input_value_resolves_the_item_label() {
    let runtime = none_mode_runtime("", true);
    assert_eq!(
        runtime.fill_input_value_for(&"banana"),
        SharedString::from("Banana")
    );
}

#[test]
fn clear_is_visible_only_with_a_non_empty_input() {
    assert!(!none_mode_runtime("", false).clear_visible());
    assert!(none_mode_runtime("b", false).clear_visible());
}
