use gpui::TestAppContext;

use crate::alert_dialog::{
    create_alert_dialog_handle,
    tests::support::{
        click_backdrop, click_close, click_detached_trigger, click_trigger, open_alert_dialog,
        read_observations, simulate_keys, AlertDialogTestConfig,
    },
};
use crate::dialog::{DialogModalMode, DialogOpenChangeReason};

#[gpui::test]
fn root_state_reports_forced_alert_invariants(cx: &mut TestAppContext) {
    let window = open_alert_dialog(cx, AlertDialogTestConfig::default());

    let root_state = read_observations(cx, window).root_state().unwrap();
    assert_eq!(root_state.modal_mode, DialogModalMode::Modal);
    assert!(root_state.disable_pointer_dismissal);
}

#[gpui::test]
fn trigger_click_opens_alert_dialog(cx: &mut TestAppContext) {
    let window = open_alert_dialog(cx, AlertDialogTestConfig::default());

    assert!(!read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.root_state().unwrap().open);
    assert!(observations.trigger_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, DialogOpenChangeReason::TriggerPress, Some(7))]
    );
}

#[gpui::test]
fn backdrop_click_does_not_close_alert_dialog(cx: &mut TestAppContext) {
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            default_open: true,
            ..AlertDialogTestConfig::default()
        },
    );

    click_backdrop(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.root_state().unwrap().open);
    assert!(observations.open_changes.is_empty());
}

#[gpui::test]
fn escape_closes_alert_dialog(cx: &mut TestAppContext) {
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            default_open: true,
            ..AlertDialogTestConfig::default()
        },
    );

    simulate_keys(cx, window, "escape");
    let observations = read_observations(cx, window);

    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, DialogOpenChangeReason::EscapeKey, None)]
    );
}

#[gpui::test]
fn close_part_closes_alert_dialog(cx: &mut TestAppContext) {
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            default_open: true,
            ..AlertDialogTestConfig::default()
        },
    );

    click_close(cx, window);
    let observations = read_observations(cx, window);

    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, DialogOpenChangeReason::ClosePress, None)]
    );
}

#[gpui::test]
fn uncontrolled_default_open_opens_initially(cx: &mut TestAppContext) {
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            default_open: true,
            ..AlertDialogTestConfig::default()
        },
    );

    assert!(read_observations(cx, window).root_state().unwrap().open);
}

#[gpui::test]
fn controlled_root_reports_open_change_without_internal_mutation(cx: &mut TestAppContext) {
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            controlled_open: Some(false),
            ..AlertDialogTestConfig::default()
        },
    );

    click_trigger(cx, window);
    let observations = read_observations(cx, window);

    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, DialogOpenChangeReason::TriggerPress, Some(7))]
    );
}

#[gpui::test]
fn handle_opens_and_closes_alert_dialog(cx: &mut TestAppContext) {
    let handle = create_alert_dialog_handle::<usize>();
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            handle: Some(handle.clone()),
            ..AlertDialogTestConfig::default()
        },
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(!handle.is_open(cx));
            assert!(handle.open("alert-dialog-trigger", window, cx));
            assert!(handle.is_open(cx));
        })
        .expect("alert dialog test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    let root_state = observations.root_state().unwrap();
    assert!(root_state.open);
    assert_eq!(root_state.modal_mode, DialogModalMode::Modal);
    assert!(root_state.disable_pointer_dismissal);

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.close(window, cx));
        })
        .expect("alert dialog test window should be open");
    cx.run_until_parked();

    let observations = read_observations(cx, window);
    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![
            (true, DialogOpenChangeReason::ImperativeAction, Some(7)),
            (false, DialogOpenChangeReason::ImperativeAction, Some(7)),
        ]
    );
}

#[gpui::test]
fn detached_trigger_bound_via_handle_opens_alert_dialog(cx: &mut TestAppContext) {
    let handle = create_alert_dialog_handle::<usize>();
    let window = open_alert_dialog(
        cx,
        AlertDialogTestConfig {
            handle: Some(handle.clone()),
            detached_trigger: true,
            ..AlertDialogTestConfig::default()
        },
    );

    click_detached_trigger(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, DialogOpenChangeReason::TriggerPress, Some(9))]
    );
}
