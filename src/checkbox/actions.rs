use gpui::{actions, App, KeyBinding};

pub const CHECKBOX_ROOT_KEY_CONTEXT: &str = "CheckboxRoot";

actions!(base_gpui_checkbox, [CheckboxToggle]);

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "space",
        CheckboxToggle,
        Some(CHECKBOX_ROOT_KEY_CONTEXT),
    )]);
}
