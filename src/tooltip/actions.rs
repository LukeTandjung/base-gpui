use gpui::{actions, App, KeyBinding};

pub const TOOLTIP_KEY_CONTEXT: &str = "Tooltip";

actions!(base_gpui_tooltip, [TooltipCloseAction]);

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "escape",
        TooltipCloseAction,
        Some(TOOLTIP_KEY_CONTEXT),
    )]);
}
