use gpui::{actions, App, KeyBinding};

pub const MENU_KEY_CONTEXT: &str = "Menu";

actions!(
    base_gpui_menu,
    [
        MenuMoveNext,
        MenuMovePrevious,
        MenuMoveFirst,
        MenuMoveLast,
        MenuActivateHighlighted,
        MenuSpaceActivate,
        MenuCloseAction,
        MenuArrowRight,
        MenuArrowLeft,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(MENU_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("down", MenuMoveNext, context),
        KeyBinding::new("up", MenuMovePrevious, context),
        KeyBinding::new("home", MenuMoveFirst, context),
        KeyBinding::new("end", MenuMoveLast, context),
        KeyBinding::new("enter", MenuActivateHighlighted, context),
        KeyBinding::new("space", MenuSpaceActivate, context),
        KeyBinding::new("escape", MenuCloseAction, context),
        KeyBinding::new("right", MenuArrowRight, context),
        KeyBinding::new("left", MenuArrowLeft, context),
    ]);
}
