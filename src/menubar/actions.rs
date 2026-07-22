use gpui::{actions, App, KeyBinding};

/// Key context set on the menubar row. Menubar-hosted triggers carry the
/// Menu key context, whose handlers route menubar roving through the typed
/// menubar link; these bindings cover dispatch on the row itself.
pub const MENUBAR_KEY_CONTEXT: &str = "Menubar";

actions!(
    base_gpui_menubar,
    [
        MenubarFocusLeft,
        MenubarFocusRight,
        MenubarFocusUp,
        MenubarFocusDown,
        MenubarFocusFirst,
        MenubarFocusLast,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(MENUBAR_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", MenubarFocusLeft, context),
        KeyBinding::new("right", MenubarFocusRight, context),
        KeyBinding::new("up", MenubarFocusUp, context),
        KeyBinding::new("down", MenubarFocusDown, context),
        KeyBinding::new("home", MenubarFocusFirst, context),
        KeyBinding::new("end", MenubarFocusLast, context),
    ]);
}
