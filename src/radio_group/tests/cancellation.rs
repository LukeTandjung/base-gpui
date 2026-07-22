use gpui::TestAppContext;

use super::support::{
    click_radio, open_radio_group, read_observations, simulate_keys, RadioGroupTestConfig, EXPRESS,
    STANDARD,
};

#[gpui::test]
fn canceled_uncontrolled_pointer_activation_does_not_mutate_selected_value(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    click_radio(cx, window, EXPRESS);

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_uncontrolled_keyboard_activation_does_not_mutate_selected_value(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert_eq!(observations.value_changes, vec![Some(STANDARD)]);
    assert_eq!(observations.change_canceled, vec![true]);
}

#[gpui::test]
fn canceled_arrow_key_selection_moves_highlight_without_checking_radio(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            cancel_changes: true,
            ..Default::default()
        },
    );

    simulate_keys(cx, window, "right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.selected_value(), None);
    assert_eq!(observations.value_changes, vec![Some(EXPRESS)]);
}
