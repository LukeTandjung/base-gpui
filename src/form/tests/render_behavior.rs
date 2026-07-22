use super::support::{debug_bounds, open_form, read_observations, FormTestConfig};

#[gpui::test]
fn renders_arbitrary_children_and_exposes_style_state(cx: &mut gpui::TestAppContext) {
    let window = open_form(cx, FormTestConfig::default());

    let observations = read_observations(cx, window);

    assert_eq!(observations.form_states.len(), 1);
    assert!(debug_bounds(cx, window, "arbitrary-child").is_some());
}
