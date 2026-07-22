use gpui::{actions, App, KeyBinding};

pub const SELECT_KEY_CONTEXT: &str = "Select";

actions!(
    base_gpui_select,
    [
        SelectOpen,
        SelectClose,
        SelectToggleOpen,
        SelectMoveNext,
        SelectMovePrevious,
        SelectMoveFirst,
        SelectMoveLast,
        SelectActivateHighlighted,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(SELECT_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("down", SelectMoveNext, context),
        KeyBinding::new("up", SelectMovePrevious, context),
        KeyBinding::new("home", SelectMoveFirst, context),
        KeyBinding::new("end", SelectMoveLast, context),
        KeyBinding::new("enter", SelectActivateHighlighted, context),
        KeyBinding::new("space", SelectActivateHighlighted, context),
        KeyBinding::new("escape", SelectClose, context),
    ]);
}
