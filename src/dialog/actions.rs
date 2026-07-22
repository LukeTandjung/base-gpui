use gpui::{actions, App, KeyBinding};

pub const DIALOG_TRIGGER_KEY_CONTEXT: &str = "DialogTrigger";
pub const DIALOG_POPUP_KEY_CONTEXT: &str = "DialogPopup";

actions!(
    base_gpui_dialog,
    [
        DialogOpenAction,
        DialogCloseAction,
        DialogFocusNextAction,
        DialogFocusPreviousAction
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("space", DialogOpenAction, Some(DIALOG_TRIGGER_KEY_CONTEXT)),
        KeyBinding::new("enter", DialogOpenAction, Some(DIALOG_TRIGGER_KEY_CONTEXT)),
        KeyBinding::new(
            "escape",
            DialogCloseAction,
            Some(DIALOG_TRIGGER_KEY_CONTEXT),
        ),
        KeyBinding::new("space", DialogOpenAction, Some(DIALOG_POPUP_KEY_CONTEXT)),
        KeyBinding::new("enter", DialogOpenAction, Some(DIALOG_POPUP_KEY_CONTEXT)),
        KeyBinding::new("escape", DialogCloseAction, Some(DIALOG_POPUP_KEY_CONTEXT)),
        KeyBinding::new("tab", DialogFocusNextAction, Some(DIALOG_POPUP_KEY_CONTEXT)),
        KeyBinding::new(
            "shift-tab",
            DialogFocusPreviousAction,
            Some(DIALOG_POPUP_KEY_CONTEXT),
        ),
    ]);
}
