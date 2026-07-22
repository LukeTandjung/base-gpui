use gpui::{actions, App, KeyBinding};

/// Key context used on the wrapper around `ComboboxInput` and on the list.
///
/// Composition with `INPUT_KEY_CONTEXT`: the input primitive owns the deeper
/// key context, so text editing keys (printable characters, Backspace,
/// Delete, Home/End, Left/Right, clipboard) are claimed by the input first —
/// printable keys always type, Home/End move the caret and never jump the
/// list highlight. Only the keys the input does not bind (ArrowUp, ArrowDown,
/// Escape) bubble to the Combobox context; Enter is routed through the input
/// primitive's `on_enter` hook rather than a Combobox binding.
pub const COMBOBOX_KEY_CONTEXT: &str = "Combobox";

actions!(
    base_gpui_combobox,
    [ComboboxMoveNext, ComboboxMovePrevious, ComboboxEscape]
);

pub fn init(cx: &mut App) {
    let context = Some(COMBOBOX_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("down", ComboboxMoveNext, context),
        KeyBinding::new("up", ComboboxMovePrevious, context),
        KeyBinding::new("escape", ComboboxEscape, context),
    ]);
}
