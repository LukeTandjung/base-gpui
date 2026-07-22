use gpui::{actions, App, KeyBinding};

pub const RADIO_GROUP_KEY_CONTEXT: &str = "RadioGroup";

actions!(
    base_gpui_radio_group,
    [
        RadioGroupSelectLeft,
        RadioGroupSelectRight,
        RadioGroupSelectUp,
        RadioGroupSelectDown,
        RadioGroupActivateFocused,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(RADIO_GROUP_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", RadioGroupSelectLeft, context),
        KeyBinding::new("right", RadioGroupSelectRight, context),
        KeyBinding::new("up", RadioGroupSelectUp, context),
        KeyBinding::new("down", RadioGroupSelectDown, context),
        KeyBinding::new("shift-left", RadioGroupSelectLeft, context),
        KeyBinding::new("shift-right", RadioGroupSelectRight, context),
        KeyBinding::new("shift-up", RadioGroupSelectUp, context),
        KeyBinding::new("shift-down", RadioGroupSelectDown, context),
        KeyBinding::new("space", RadioGroupActivateFocused, context),
    ]);
}
