use gpui::{actions, App, KeyBinding};

/// Key context set on the toolbar root; arrow roving is handled there so
/// that focused items (including the reused input, whose own deeper key
/// context consumes caret arrows) dispatch through it.
pub const TOOLBAR_KEY_CONTEXT: &str = "Toolbar";

/// Key context set on activatable toolbar items (buttons and links) only, so
/// Space/Enter never intercept typing inside a toolbar input.
pub const TOOLBAR_ITEM_KEY_CONTEXT: &str = "ToolbarItem";

actions!(
    base_gpui_toolbar,
    [
        ToolbarFocusLeft,
        ToolbarFocusRight,
        ToolbarFocusUp,
        ToolbarFocusDown,
        ToolbarActivateFocused,
    ]
);

/// Base UI Toolbar leaves `enableHomeAndEndKeys` off, so Home/End are
/// intentionally not bound.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("left", ToolbarFocusLeft, Some(TOOLBAR_KEY_CONTEXT)),
        KeyBinding::new("right", ToolbarFocusRight, Some(TOOLBAR_KEY_CONTEXT)),
        KeyBinding::new("up", ToolbarFocusUp, Some(TOOLBAR_KEY_CONTEXT)),
        KeyBinding::new("down", ToolbarFocusDown, Some(TOOLBAR_KEY_CONTEXT)),
        KeyBinding::new(
            "space",
            ToolbarActivateFocused,
            Some(TOOLBAR_ITEM_KEY_CONTEXT),
        ),
        KeyBinding::new(
            "enter",
            ToolbarActivateFocused,
            Some(TOOLBAR_ITEM_KEY_CONTEXT),
        ),
    ]);
}
