//! Grouped-mode seam: pressed state derives from group membership and the
//! disabled fact resolves own OR group disabled. Full grouped interaction is
//! verified with `issues/port-baseui-toggle-group.md`; this stubs the group
//! context at the runtime seam.

use gpui::SharedString;

use crate::toggle::ToggleRuntime;

fn membership(group_value: &[SharedString], own_value: &SharedString) -> Option<bool> {
    Some(group_value.contains(own_value))
}

#[test]
fn pressed_derives_from_group_membership() {
    let mut runtime = ToggleRuntime::new(Some(false));
    let own_value = SharedString::from("bold");

    runtime.sync_group(membership(&[SharedString::from("bold")], &own_value), false);
    assert!(runtime.pressed());

    runtime.sync_group(membership(&[], &own_value), false);
    assert!(!runtime.pressed());
}

#[test]
fn group_membership_overrides_local_pressed_state() {
    let mut runtime = ToggleRuntime::new(Some(true));
    let own_value = SharedString::from("italic");

    runtime.sync_group(membership(&[], &own_value), false);
    assert!(!runtime.pressed());
}

#[test]
fn disabled_fact_resolves_own_or_group_disabled() {
    let mut runtime = ToggleRuntime::new(Some(false));
    assert!(!runtime.disabled());

    runtime.sync_group(Some(false), true);
    assert!(runtime.disabled());

    runtime.sync_group(Some(false), false);
    runtime.sync_own_disabled(true);
    assert!(runtime.disabled());
}
