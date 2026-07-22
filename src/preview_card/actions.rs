use gpui::{actions, App, KeyBinding};

pub const PREVIEW_CARD_KEY_CONTEXT: &str = "PreviewCard";

actions!(base_gpui_preview_card, [PreviewCardCloseAction]);

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "escape",
        PreviewCardCloseAction,
        Some(PREVIEW_CARD_KEY_CONTEXT),
    )]);
}
