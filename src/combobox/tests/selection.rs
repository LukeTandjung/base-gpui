use gpui::SharedString;

use crate::combobox::tests::support::{multiple_runtime, single_runtime};
use crate::combobox::ComboboxSelectionChange;

#[test]
fn single_select_reports_change_and_fill_value() {
    let runtime = single_runtime(None, "", true);
    let outcome = runtime.request_select_value(None, Vec::new(), "banana", false, false, false);

    match outcome {
        ComboboxSelectionChange::Single { changed, value } => {
            assert!(changed);
            assert_eq!(value, Some("banana"));
        }
        _ => panic!("expected single outcome"),
    }
    assert_eq!(
        runtime.fill_input_value_for(&"banana"),
        SharedString::from("Banana")
    );
}

#[test]
fn selecting_the_same_single_value_is_not_a_change() {
    let runtime = single_runtime(Some("banana"), "", true);
    let outcome =
        runtime.request_select_value(Some("banana"), Vec::new(), "banana", false, false, false);
    assert!(!outcome.changed());
}

#[test]
fn multiple_select_toggles_membership_in_order_without_duplicates() {
    let runtime = multiple_runtime(vec!["apple"], "", true);

    let outcome = runtime.request_select_value(None, vec!["apple"], "banana", false, false, false);
    match outcome {
        ComboboxSelectionChange::Multiple { changed, values } => {
            assert!(changed);
            assert_eq!(values, vec!["apple", "banana"]);
        }
        _ => panic!("expected multiple outcome"),
    }

    let outcome =
        runtime.request_select_value(None, vec!["apple", "banana"], "apple", false, false, false);
    match outcome {
        ComboboxSelectionChange::Multiple { values, .. } => {
            assert_eq!(values, vec!["banana"]);
        }
        _ => panic!("expected multiple outcome"),
    }
}

#[test]
fn disabled_item_or_root_makes_activation_a_no_op() {
    let runtime = single_runtime(None, "", true);
    assert!(!runtime
        .request_select_value(None, Vec::new(), "cherry", true, false, false)
        .changed());
    assert!(!runtime
        .request_select_value(None, Vec::new(), "banana", false, true, false)
        .changed());
    assert!(!runtime
        .request_select_value(None, Vec::new(), "banana", false, false, true)
        .changed());
}

#[test]
fn item_selected_state_follows_mode() {
    let runtime = single_runtime(Some("banana"), "", false);
    assert!(runtime.is_value_selected(&"banana"));
    assert!(!runtime.is_value_selected(&"apple"));

    let runtime = multiple_runtime(vec!["apple", "banana"], "", false);
    assert!(runtime.is_value_selected(&"apple"));
    assert!(!runtime.is_value_selected(&"cherry"));
}

#[test]
fn escape_while_closed_clear_facts_are_exposed() {
    let runtime = single_runtime(Some("banana"), "Banana", false);
    assert!(!runtime.open_value());
    assert!(runtime.clear_visible());
}
