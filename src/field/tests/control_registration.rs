use gpui::{SharedString, TestAppContext};

use super::support::{open_field, read_observations, update_config, FieldTestConfig};

#[gpui::test]
fn registration_replacement_updates_filled_state_without_stale_values(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            value: SharedString::from("filled"),
            ..Default::default()
        },
    );
    assert!(read_observations(cx, window).root_state().filled);

    update_config(cx, window, |config| {
        config.value = SharedString::default();
    });

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().filled);
}
