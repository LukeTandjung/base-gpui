use gpui::{App, FocusHandle, Window};

use crate::{
    toggle::toggle_focus_handle,
    toggle_group::{ToggleGroupChild, ToggleGroupContext, ToggleGroupToggleMetadata},
};

pub struct WiredToggleGroupChildren<T: Clone + Eq + 'static> {
    pub toggles: Vec<ToggleGroupToggleMetadata<T>>,
    pub focus_handles: Vec<(usize, FocusHandle)>,
    pub focused_index: Option<usize>,
    pub children: Vec<ToggleGroupChild<T>>,
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<ToggleGroupChild<T>>,
    context: ToggleGroupContext<T>,
    group_disabled: bool,
    value_initialized: bool,
    window: &mut Window,
    cx: &mut App,
) -> WiredToggleGroupChildren<T> {
    let mut toggles = Vec::new();
    let mut focus_handles = Vec::new();
    let mut focused_index = None;
    let mut wired = Vec::new();

    for (index, child) in children.into_iter().enumerate() {
        match child {
            ToggleGroupChild::Toggle(toggle) => {
                let focus_handle = toggle_focus_handle(toggle.toggle_id(), window, cx);
                let value = toggle.group_value().cloned();

                if value.is_none() && value_initialized {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "base_gpui: a Toggle inside a ToggleGroup with an initialized value \
                         should set an explicit `value`; it will not join the group value."
                    );
                }

                let disabled = toggle.own_disabled() || group_disabled;
                toggles.push(ToggleGroupToggleMetadata::new(value, disabled, index));

                if focus_handle.is_focused(window) {
                    focused_index = Some(index);
                }
                focus_handles.push((index, focus_handle.clone()));

                wired.push(ToggleGroupChild::Toggle(toggle.with_toggle_group(
                    context.clone(),
                    index,
                    focus_handle,
                )));
            }
        }
    }

    WiredToggleGroupChildren {
        toggles,
        focus_handles,
        focused_index,
        children: wired,
    }
}
