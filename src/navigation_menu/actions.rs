use gpui::{actions, App, KeyBinding};

/// Key context set on the navigation menu list (and popup, for Escape).
pub const NAVIGATION_MENU_KEY_CONTEXT: &str = "NavigationMenu";

actions!(
    base_gpui_navigation_menu,
    [
        NavigationMenuFocusLeft,
        NavigationMenuFocusRight,
        NavigationMenuFocusUp,
        NavigationMenuFocusDown,
        NavigationMenuFocusFirst,
        NavigationMenuFocusLast,
        NavigationMenuCloseAction,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(NAVIGATION_MENU_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", NavigationMenuFocusLeft, context),
        KeyBinding::new("right", NavigationMenuFocusRight, context),
        KeyBinding::new("up", NavigationMenuFocusUp, context),
        KeyBinding::new("down", NavigationMenuFocusDown, context),
        KeyBinding::new("home", NavigationMenuFocusFirst, context),
        KeyBinding::new("end", NavigationMenuFocusLast, context),
        KeyBinding::new("escape", NavigationMenuCloseAction, context),
    ]);
}
