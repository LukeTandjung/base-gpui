use gpui::ElementId;

use crate::drawer::{DrawerRuntime, DrawerSwipeDirection};

fn parent_runtime() -> DrawerRuntime {
    let mut runtime = DrawerRuntime::new(DrawerSwipeDirection::Down);
    runtime.set_viewport_height(800.0);
    runtime.set_popup_size(300.0, 400.0);
    runtime
}

#[test]
fn nested_presence_marks_count_and_unmount_decrements() {
    let mut runtime = parent_runtime();
    let id = ElementId::from("nested");

    runtime.report_nested(id.clone(), true, None, 0.0, 0.0);
    assert_eq!(runtime.nested_open_drawer_count(), 1);

    runtime.report_nested(id, false, None, 0.0, 0.0);
    assert_eq!(runtime.nested_open_drawer_count(), 0);
}

#[test]
fn frontmost_height_follows_nested_and_falls_back_to_own() {
    let mut runtime = parent_runtime();
    let id = ElementId::from("nested");

    assert_eq!(runtime.frontmost_height(), Some(400.0));

    runtime.report_nested(id.clone(), true, Some(250.0), 0.0, 0.0);
    assert_eq!(runtime.frontmost_height(), Some(250.0));

    runtime.report_nested(id, false, None, 0.0, 0.0);
    assert_eq!(runtime.frontmost_height(), Some(400.0));
}

#[test]
fn nested_swiping_requires_ten_pixels_and_resets_on_finish() {
    let mut runtime = parent_runtime();
    let id = ElementId::from("nested");

    runtime.report_nested(id.clone(), true, None, 5.0, 0.1);
    assert!(!runtime.nested_drawer_swiping());

    runtime.report_nested(id.clone(), true, None, 15.0, 0.3);
    assert!(runtime.nested_drawer_swiping());
    assert_eq!(runtime.nested_swipe_progress(), 0.3);

    runtime.finish_nested_swipe(&id);
    assert!(!runtime.nested_drawer_swiping());
    assert_eq!(runtime.nested_swipe_progress(), 0.0);
}

#[test]
fn popup_height_is_held_while_a_nested_drawer_is_present() {
    let mut runtime = parent_runtime();
    runtime.report_nested(ElementId::from("nested"), true, None, 0.0, 0.0);

    assert!(!runtime.set_popup_size(300.0, 999.0));
    assert_eq!(runtime.frontmost_height(), Some(400.0));
}

#[test]
fn nested_facts_surface_in_popup_facts() {
    let mut runtime = parent_runtime();
    runtime.mark_nested(true);
    runtime.report_nested(ElementId::from("nested"), true, Some(250.0), 15.0, 0.4);

    let facts = runtime.popup_facts();

    assert!(facts.nested);
    assert_eq!(facts.nested_drawer_count, 1);
    assert!(facts.nested_drawer_swiping);
    assert_eq!(facts.nested_swipe_progress, 0.4);
}
