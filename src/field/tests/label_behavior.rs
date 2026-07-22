use gpui::TestAppContext;

use super::support::{debug_bounds, open_field, read_observations, FieldTestConfig};

#[gpui::test]
fn label_style_state_receives_current_field_state(cx: &mut TestAppContext) {
    let window = open_field(
        cx,
        FieldTestConfig {
            root_invalid: Some(true),
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).label_state();
    assert!(state.root.invalid);
}

#[gpui::test]
fn label_click_focuses_registered_control(cx: &mut TestAppContext) {
    let window = open_field(cx, FieldTestConfig::default());
    let label_bounds = debug_bounds(cx, window, "field-label").expect("label should render");
    let mut visual = gpui::VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(label_bounds.center(), gpui::Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    assert!(read_observations(cx, window).last_control_focused());
}
