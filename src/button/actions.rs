use gpui::{actions, App, KeyBinding};

pub const BUTTON_ROOT_KEY_CONTEXT: &str = "ButtonRoot";

actions!(base_gpui_button, [ButtonActivate]);

pub fn init(cx: &mut App) {
    let context = Some(BUTTON_ROOT_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("space", ButtonActivate, context),
        KeyBinding::new("enter", ButtonActivate, context),
    ]);
}
