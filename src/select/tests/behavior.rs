use gpui::TestAppContext;

use crate::select::tests::support::{
    blur, click_item, click_label, click_outside_target, click_trigger, move_over_selector,
    open_select, read_observations, scroll_over_selector, simulate_keys, SelectTestConfig, APPLE,
    BANANA,
};

#[gpui::test]
fn trigger_click_opens_popup(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    assert!(!read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.root_state().unwrap().open);
    assert_eq!(observations.open_changes, vec![true]);
}

#[gpui::test]
fn item_click_selects_and_closes_single_select(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            ..SelectTestConfig::default()
        },
    );

    click_item(cx, window, BANANA);

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(BANANA)]);
    assert_eq!(
        observations.root_state().unwrap().selected_value,
        Some(BANANA)
    );
    assert!(!observations.root_state().unwrap().open);
}

#[gpui::test]
fn controlled_missing_single_value_notifies_fallback_without_overriding_controlled_value(
    cx: &mut TestAppContext,
) {
    let window = open_select(
        cx,
        SelectTestConfig {
            controlled_value: Some(Some("missing")),
            ..SelectTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![None]);
    assert_eq!(
        observations.root_state().unwrap().selected_value,
        Some("missing")
    );
}

#[gpui::test]
fn item_click_toggles_and_stays_open_in_multiple_select(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            multiple: true,
            default_open: true,
            default_values: vec![APPLE],
            ..SelectTestConfig::default()
        },
    );

    click_item(cx, window, BANANA);

    let observations = read_observations(cx, window);
    assert_eq!(observations.values_changes, vec![vec![APPLE, BANANA]]);
    assert_eq!(
        observations.root_state().unwrap().selected_values,
        vec![APPLE, BANANA]
    );
    assert!(observations.root_state().unwrap().open);
}

#[gpui::test]
fn keyboard_arrow_down_and_enter_activate_highlighted_item(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "down enter");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(BANANA)]);
    assert_eq!(
        observations.root_state().unwrap().selected_value,
        Some(BANANA)
    );
}

#[gpui::test]
fn keyboard_arrow_up_and_space_activate_highlighted_item(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "down up space");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(APPLE)]);
    assert_eq!(
        observations.root_state().unwrap().selected_value,
        Some(APPLE)
    );
}

#[gpui::test]
fn keyboard_navigation_can_focus_disabled_item_without_selecting_it(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            disabled_values: vec![BANANA],
            ..SelectTestConfig::default()
        },
    );

    click_trigger(cx, window);
    simulate_keys(cx, window, "down enter");

    let observations = read_observations(cx, window);
    let banana_state = observations.item_state(BANANA).unwrap();
    assert!(banana_state.disabled);
    assert!(banana_state.highlighted);
    assert!(banana_state.focused);
    assert!(banana_state.tab_stop);
    assert!(observations.value_changes.is_empty());
    assert_eq!(observations.root_state().unwrap().selected_value, None);
    assert!(observations.root_state().unwrap().open);
}

#[gpui::test]
fn label_click_focuses_trigger_without_opening_popup(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_label(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.trigger_states.last().unwrap().root.focused);
    assert!(!observations.root_state().unwrap().open);
}

#[gpui::test]
fn focus_out_closes_popup(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    blur(cx, window);

    assert!(!read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn modal_backdrop_blocks_outside_clicks_and_closes(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            modal: true,
            ..SelectTestConfig::default()
        },
    );

    click_trigger(cx, window);
    let _ = read_observations(cx, window);
    scroll_over_selector(cx, window, "select-outside-target");
    click_outside_target(cx, window);

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(observations.outside_clicks, 0);
    assert_eq!(observations.outside_scrolls, 0);

    click_outside_target(cx, window);
    scroll_over_selector(cx, window, "select-outside-target");
    let observations = read_observations(cx, window);
    assert_eq!(observations.outside_clicks, 1);
    assert_eq!(observations.outside_scrolls, 1);
}

#[gpui::test]
fn escape_closes_popup(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "escape");

    assert!(!read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn typeahead_while_closed_commits_matching_item(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_label(cx, window);
    simulate_keys(cx, window, "b");

    let observations = read_observations(cx, window);
    assert_eq!(observations.value_changes, vec![Some(BANANA)]);
    assert_eq!(
        observations.root_state().unwrap().selected_value,
        Some(BANANA)
    );
}

#[gpui::test]
fn typeahead_while_open_highlights_matching_item(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    simulate_keys(cx, window, "b");

    let observations = read_observations(cx, window);
    assert!(observations.item_state(BANANA).unwrap().highlighted);
    assert!(observations.value_changes.is_empty());
}

#[gpui::test]
fn value_displays_placeholder_and_selected_label(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    assert!(
        read_observations(cx, window)
            .value_state()
            .unwrap()
            .placeholder
    );

    click_trigger(cx, window);
    click_item(cx, window, APPLE);

    let value_state = read_observations(cx, window).value_state().unwrap();
    assert!(!value_state.placeholder);
    assert_eq!(value_state.display_text.to_string(), "Apple");
}

#[gpui::test]
fn item_indicator_keep_mounted_tracks_selected_state(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            default_value: Some(APPLE),
            ..SelectTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.latest_indicator(APPLE).unwrap().selected);
    assert!(!observations.latest_indicator(BANANA).unwrap().selected);
    assert!(observations.latest_indicator(BANANA).unwrap().present);
}

#[gpui::test]
fn hovering_scroll_arrow_scrolls_toward_next_item(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            include_scroll_arrows: true,
            include_group_label_and_separator: false,
            ..SelectTestConfig::default()
        },
    );
    cx.run_until_parked();
    cx.run_until_parked();

    move_over_selector(cx, window, "select-scroll-down-arrow");
    cx.run_until_parked();
    cx.run_until_parked();

    assert!(
        read_observations(cx, window)
            .scroll_arrow_state(crate::select::SelectScrollArrowDirection::Up)
            .unwrap()
            .visible
    );
}

#[gpui::test]
fn keyboard_navigation_scrolls_highlighted_item_into_view(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            include_scroll_arrows: true,
            include_group_label_and_separator: false,
            ..SelectTestConfig::default()
        },
    );
    cx.run_until_parked();
    cx.run_until_parked();
    click_label(cx, window);

    simulate_keys(cx, window, "down down");
    cx.run_until_parked();
    cx.run_until_parked();

    assert!(
        read_observations(cx, window)
            .scroll_arrow_state(crate::select::SelectScrollArrowDirection::Up)
            .unwrap()
            .visible
    );
}

#[gpui::test]
fn scroll_arrows_reflect_list_scrollability(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            include_scroll_arrows: true,
            include_group_label_and_separator: false,
            ..SelectTestConfig::default()
        },
    );
    cx.run_until_parked();
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert!(
        !observations
            .scroll_arrow_state(crate::select::SelectScrollArrowDirection::Up)
            .unwrap()
            .visible
    );
    assert!(
        observations
            .scroll_arrow_state(crate::select::SelectScrollArrowDirection::Down)
            .unwrap()
            .visible
    );
}

#[gpui::test]
fn popup_positioner_records_trigger_measurement_state(cx: &mut TestAppContext) {
    let window = open_select(cx, SelectTestConfig::default());

    click_trigger(cx, window);
    cx.run_until_parked();
    cx.run_until_parked();

    let positioner_state = read_observations(cx, window).positioner_state().unwrap();
    assert!(positioner_state.anchor_available);
    assert!(positioner_state.anchor_bounds.is_some());
    assert!(positioner_state.anchor_width.is_some());
    assert!(positioner_state.anchor_height.is_some());
    assert!(positioner_state.available_width.is_some());
    assert!(positioner_state.available_height.is_some());

    let apple_state = read_observations(cx, window).item_state(APPLE).unwrap();
    assert!(apple_state.item_bounds.is_some());
    assert!(apple_state.item_text_bounds.is_some());
}

#[gpui::test]
fn group_label_registers_group_metadata(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            wrap_items_in_group: true,
            ..SelectTestConfig::default()
        },
    );

    let group_state = read_observations(cx, window)
        .group_state()
        .expect("group state should render");
    assert_eq!(group_state.group_index, Some(0));
    assert_eq!(group_state.label.as_deref(), Some("Fruit"));
    assert_eq!(group_state.item_count, 3);

    let observations = read_observations(cx, window);
    assert_eq!(observations.item_state(APPLE).unwrap().group_index, Some(0));
    assert_eq!(
        observations.item_state(BANANA).unwrap().group_index,
        Some(0)
    );
}

#[gpui::test]
fn group_labels_and_separators_do_not_affect_item_indices(cx: &mut TestAppContext) {
    let window = open_select(
        cx,
        SelectTestConfig {
            default_open: true,
            include_group_label_and_separator: true,
            ..SelectTestConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.item_state(APPLE).unwrap().index, Some(0));
    assert_eq!(observations.item_state(BANANA).unwrap().index, Some(1));
}
