use gpui::{actions, App, KeyBinding};

pub const COLLAPSIBLE_TRIGGER_KEY_CONTEXT: &str = "CollapsibleTrigger";

actions!(base_gpui_collapsible, [CollapsibleToggle]);

pub fn init(cx: &mut App) {
    let context = Some(COLLAPSIBLE_TRIGGER_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("space", CollapsibleToggle, context),
        KeyBinding::new("enter", CollapsibleToggle, context),
    ]);
}
