use gpui::{actions, App, KeyBinding};

pub const TOGGLE_GROUP_KEY_CONTEXT: &str = "ToggleGroup";

actions!(
    base_gpui_toggle_group,
    [
        ToggleGroupFocusLeft,
        ToggleGroupFocusRight,
        ToggleGroupFocusUp,
        ToggleGroupFocusDown,
        ToggleGroupFocusFirst,
        ToggleGroupFocusLast,
        ToggleGroupActivateFocused,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(TOGGLE_GROUP_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", ToggleGroupFocusLeft, context),
        KeyBinding::new("right", ToggleGroupFocusRight, context),
        KeyBinding::new("up", ToggleGroupFocusUp, context),
        KeyBinding::new("down", ToggleGroupFocusDown, context),
        KeyBinding::new("home", ToggleGroupFocusFirst, context),
        KeyBinding::new("end", ToggleGroupFocusLast, context),
        KeyBinding::new("space", ToggleGroupActivateFocused, context),
        KeyBinding::new("enter", ToggleGroupActivateFocused, context),
    ]);
}
