use gpui::TestAppContext;

use super::support::{open_field, read_observations, FieldTestConfig};

#[gpui::test]
fn root_disabled_state_is_exposed_on_field_parts_and_control(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_disabled: true,
            error_always: true,
            ..Default::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.root_state().disabled);
    assert!(observations.label_state().disabled);
    assert!(observations.description_state().disabled);
    assert!(
        observations
            .error_state()
            .expect("error should render")
            .root
            .disabled
    );
    assert!(observations.validity_state().root.disabled);
    assert!(observations.last_control_disabled());
    assert_eq!(observations.root_state().valid, None);
}
