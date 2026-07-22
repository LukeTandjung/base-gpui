use gpui::TestAppContext;

use crate::utils::direction::TextDirection;

use super::support::{
    focus_next, open_radio_group, read_observations, simulate_keys, RadioGroupTestConfig, EXPRESS,
    OVERNIGHT, STANDARD,
};

#[gpui::test]
fn arrow_down_and_up_navigation_moves_highlight_and_selects(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    simulate_keys(cx, window, "down");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.selected_value(), Some(EXPRESS));

    simulate_keys(cx, window, "up");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(STANDARD));
    assert_eq!(observations.selected_value(), Some(STANDARD));
}

#[gpui::test]
fn direction_provider_ltr_right_and_left_navigation_moves_focus_and_selects(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            direction: Some(TextDirection::Ltr),
            ..Default::default()
        },
    );

    simulate_keys(cx, window, "right");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.selected_value(), Some(EXPRESS));

    simulate_keys(cx, window, "left");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(STANDARD));
    assert_eq!(observations.selected_value(), Some(STANDARD));
}

#[gpui::test]
fn direction_provider_rtl_left_and_right_navigation_moves_focus_and_selects(
    cx: &mut TestAppContext,
) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            direction: Some(TextDirection::Rtl),
            ..Default::default()
        },
    );

    simulate_keys(cx, window, "right");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(OVERNIGHT));
    assert_eq!(observations.selected_value(), Some(OVERNIGHT));

    simulate_keys(cx, window, "left");
    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(STANDARD));
    assert_eq!(observations.selected_value(), Some(STANDARD));
}

#[gpui::test]
fn arrow_navigation_wraps_at_the_ends(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    simulate_keys(cx, window, "left");

    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(OVERNIGHT));
    assert_eq!(observations.selected_value(), Some(OVERNIGHT));
}

#[gpui::test]
fn shift_modified_arrow_navigation_still_moves_focus(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    simulate_keys(cx, window, "shift-right");

    let observations = read_observations(cx, window);
    assert_eq!(observations.highlighted_value(), Some(EXPRESS));
    assert_eq!(observations.selected_value(), Some(EXPRESS));
}

#[gpui::test]
fn home_end_and_enter_do_not_select_radios(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    simulate_keys(cx, window, "end home enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), None);
    assert_eq!(observations.highlighted_value(), Some(STANDARD));
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn tabbing_away_and_back_returns_to_current_roving_tab_stop(cx: &mut TestAppContext) {
    let window = open_radio_group(
        cx,
        RadioGroupTestConfig {
            default_value: Some(EXPRESS),
            include_trailing_focusable: true,
            ..Default::default()
        },
    );

    assert_eq!(read_observations(cx, window).focused_value(), Some(EXPRESS));

    focus_next(cx, window);
    assert_eq!(read_observations(cx, window).focused_value(), None);

    focus_next(cx, window);
    assert_eq!(read_observations(cx, window).focused_value(), Some(EXPRESS));
}

#[gpui::test]
fn space_selects_focused_radio_once(cx: &mut TestAppContext) {
    let window = open_radio_group(cx, RadioGroupTestConfig::default());

    simulate_keys(cx, window, "space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.selected_value(), Some(STANDARD));
    assert_eq!(observations.value_changes, vec![Some(STANDARD)]);
}
