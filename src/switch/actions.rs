use gpui::{actions, App, KeyBinding};

pub const SWITCH_ROOT_KEY_CONTEXT: &str = "SwitchRoot";

actions!(base_gpui_switch, [SwitchToggle]);

pub fn init(cx: &mut App) {
    let context = Some(SWITCH_ROOT_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("space", SwitchToggle, context),
        KeyBinding::new("enter", SwitchToggle, context),
    ]);
}
