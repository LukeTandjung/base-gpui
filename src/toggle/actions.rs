use gpui::{actions, App, KeyBinding};

pub const TOGGLE_KEY_CONTEXT: &str = "Toggle";

actions!(base_gpui_toggle, [ToggleActivate]);

pub fn init(cx: &mut App) {
    let context = Some(TOGGLE_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("space", ToggleActivate, context),
        KeyBinding::new("enter", ToggleActivate, context),
    ]);
}
