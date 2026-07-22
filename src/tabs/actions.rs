use gpui::{actions, App, KeyBinding};

pub const TABS_LIST_KEY_CONTEXT: &str = "TabsList";

actions!(
    base_gpui_tabs,
    [
        TabsSelectLeft,
        TabsSelectRight,
        TabsSelectUp,
        TabsSelectDown,
        TabsSelectFirst,
        TabsSelectLast,
        TabsActivateHighlighted,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(TABS_LIST_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("left", TabsSelectLeft, context),
        KeyBinding::new("right", TabsSelectRight, context),
        KeyBinding::new("up", TabsSelectUp, context),
        KeyBinding::new("down", TabsSelectDown, context),
        KeyBinding::new("home", TabsSelectFirst, context),
        KeyBinding::new("end", TabsSelectLast, context),
        KeyBinding::new("enter", TabsActivateHighlighted, context),
        KeyBinding::new("space", TabsActivateHighlighted, context),
    ]);
}
