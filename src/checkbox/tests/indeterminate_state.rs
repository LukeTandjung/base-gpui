use gpui::TestAppContext;

use super::support::{click_checkbox, open_checkbox, read_observations, CheckboxTestConfig};

#[gpui::test]
fn indeterminate_state_takes_precedence_over_unchecked_style_state(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            indeterminate: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.indeterminate);
    assert!(!state.checked);
    assert!(!state.unchecked);
}

#[gpui::test]
fn checked_indeterminate_state_is_not_unchecked(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            default_checked: true,
            indeterminate: true,
            ..Default::default()
        },
    );

    let state = read_observations(cx, window).last_root_state();
    assert!(state.checked);
    assert!(state.indeterminate);
    assert!(!state.unchecked);
}

#[gpui::test]
fn activating_indeterminate_does_not_clear_indeterminate(cx: &mut TestAppContext) {
    let window = open_checkbox(
        cx,
        CheckboxTestConfig {
            indeterminate: true,
            ..Default::default()
        },
    );

    click_checkbox(cx, window);

    let observations = read_observations(cx, window);
    let state = observations.last_root_state();
    assert!(state.checked);
    assert!(state.indeterminate);
    assert_eq!(observations.value_changes, vec![true]);
}
