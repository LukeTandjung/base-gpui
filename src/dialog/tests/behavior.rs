use gpui::TestAppContext;

use crate::dialog::{
    create_dialog_handle,
    tests::support::{
        click_backdrop, click_close, click_trigger, open_dialog, read_observations, simulate_keys,
        DialogTestConfig,
    },
    DialogOpenChangeReason,
};

#[gpui::test]
fn trigger_click_opens_dialog(cx: &mut TestAppContext) {
    let window = open_dialog(cx, DialogTestConfig::default());

    assert!(!read_observations(cx, window).root_state().unwrap().open);

    click_trigger(cx, window);
    let observations = read_observations(cx, window);

    assert!(observations.root_state().unwrap().open);
    assert!(observations.trigger_state().unwrap().open);
    assert!(observations.popup_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(true, DialogOpenChangeReason::TriggerPress, Some(7))]
    );
}

#[gpui::test]
fn close_part_closes_dialog(cx: &mut TestAppContext) {
    let window = open_dialog(
        cx,
        DialogTestConfig {
            default_open: true,
            ..DialogTestConfig::default()
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
fn escape_closes_dialog_from_popup(cx: &mut TestAppContext) {
    let window = open_dialog(
        cx,
        DialogTestConfig {
            default_open: true,
            ..DialogTestConfig::default()
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
fn backdrop_click_dismisses_dialog(cx: &mut TestAppContext) {
    let window = open_dialog(
        cx,
        DialogTestConfig {
            default_open: true,
            ..DialogTestConfig::default()
        },
    );

    click_backdrop(cx, window);
    let observations = read_observations(cx, window);

    assert!(!observations.root_state().unwrap().open);
    assert_eq!(
        observations.open_changes,
        vec![(false, DialogOpenChangeReason::OutsidePress, None)]
    );
}

#[gpui::test]
fn disabled_trigger_is_not_activatable(cx: &mut TestAppContext) {
    let window = open_dialog(
        cx,
        DialogTestConfig {
            trigger_disabled: true,
            ..DialogTestConfig::default()
        },
    );

    click_trigger(cx, window);
    let observations = read_observations(cx, window);

    assert!(!observations.root_state().unwrap().open);
    assert!(observations.open_changes.is_empty());
    assert!(observations.trigger_state().unwrap().disabled);
}

#[gpui::test]
fn controlled_root_reports_open_change_without_internal_mutation(cx: &mut TestAppContext) {
    let window = open_dialog(
        cx,
        DialogTestConfig {
            controlled_open: Some(false),
            ..DialogTestConfig::default()
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
fn detached_handle_opens_and_closes(cx: &mut TestAppContext) {
    let handle = create_dialog_handle::<usize>();
    let window = open_dialog(
        cx,
        DialogTestConfig {
            handle: Some(handle.clone()),
            ..DialogTestConfig::default()
        },
    );

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.open("dialog-trigger", window, cx));
        })
        .expect("dialog test window should be open");
    cx.run_until_parked();
    assert!(read_observations(cx, window).root_state().unwrap().open);

    window
        .update(cx, |_view, window, cx| {
            assert!(handle.close(window, cx));
        })
        .expect("dialog test window should be open");
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
