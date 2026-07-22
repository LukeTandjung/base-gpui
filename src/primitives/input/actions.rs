use gpui::{actions, App, KeyBinding};

pub const INPUT_KEY_CONTEXT: &str = "Input";

actions!(
    base_gpui_input,
    [
        InputBackspace,
        InputDelete,
        InputLeft,
        InputRight,
        InputSelectLeft,
        InputSelectRight,
        InputSelectAll,
        InputHome,
        InputEnd,
        InputPaste,
        InputCut,
        InputCopy,
        InputEnter,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", InputBackspace, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("shift-backspace", InputBackspace, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("delete", InputDelete, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("shift-delete", InputDelete, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("left", InputLeft, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("right", InputRight, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("shift-left", InputSelectLeft, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("shift-right", InputSelectRight, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("home", InputHome, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("end", InputEnd, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-left", InputHome, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-right", InputEnd, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", InputSelectAll, Some(INPUT_KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", InputSelectAll, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", InputPaste, Some(INPUT_KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", InputPaste, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", InputCopy, Some(INPUT_KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", InputCopy, Some(INPUT_KEY_CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", InputCut, Some(INPUT_KEY_CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", InputCut, Some(INPUT_KEY_CONTEXT)),
        KeyBinding::new("enter", InputEnter, Some(INPUT_KEY_CONTEXT)),
    ]);
}
