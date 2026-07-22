use gpui::{actions, App, KeyBinding};

pub const POPOVER_KEY_CONTEXT: &str = "Popover";

actions!(
    base_gpui_popover,
    [PopoverOpenAction, PopoverCloseAction, PopoverToggleAction]
);

pub fn init(cx: &mut App) {
    let context = Some(POPOVER_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("enter", PopoverToggleAction, context),
        KeyBinding::new("space", PopoverToggleAction, context),
        KeyBinding::new("escape", PopoverCloseAction, context),
    ]);
}
