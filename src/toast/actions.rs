use gpui::{actions, App, KeyBinding};

pub const TOAST_VIEWPORT_KEY_CONTEXT: &str = "ToastViewport";
pub const TOAST_ROOT_KEY_CONTEXT: &str = "ToastRoot";

actions!(
    base_gpui_toast,
    [
        ToastCloseAction,
        ToastFocusViewportAction,
        ToastExitViewportAction
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("escape", ToastCloseAction, Some(TOAST_ROOT_KEY_CONTEXT)),
        KeyBinding::new("f6", ToastFocusViewportAction, None),
        KeyBinding::new(
            "shift-tab",
            ToastExitViewportAction,
            Some(TOAST_VIEWPORT_KEY_CONTEXT),
        ),
    ]);
}
