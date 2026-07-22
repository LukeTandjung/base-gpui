use gpui::TestAppContext;

use super::support::{open_field, read_observations, FieldTestConfig};

#[gpui::test]
fn field_item_merges_disabled_state_and_disables_wrapped_control(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            wrap_control_in_item: true,
            item_disabled: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.item_state().disabled);
    assert!(observations.last_control_disabled());
}

#[gpui::test]
fn root_disabled_takes_precedence_over_item_and_control_state(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_disabled: true,
            wrap_control_in_item: true,
            item_disabled: false,
            control_disabled: false,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.item_state().disabled);
    assert!(observations.last_control_disabled());
}
