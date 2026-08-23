use gpui::{actions, App, KeyBinding};

pub const FILE_TREE_KEY_CONTEXT: &str = "FileTree";

actions!(
    base_gpui_file_tree,
    [
        FileTreeMoveNext,
        FileTreeMovePrevious,
        FileTreeMoveFirst,
        FileTreeMoveLast,
        FileTreePageUp,
        FileTreePageDown,
        FileTreeExpandOrNext,
        FileTreeCollapseOrParent,
        FileTreeExtendNext,
        FileTreeExtendPrevious,
        FileTreeActivateFocused,
        FileTreeToggleFocusedSelection,
        FileTreeSelectAllVisible,
    ]
);

pub fn init(cx: &mut App) {
    let context = Some(FILE_TREE_KEY_CONTEXT);

    cx.bind_keys([
        KeyBinding::new("down", FileTreeMoveNext, context),
        KeyBinding::new("up", FileTreeMovePrevious, context),
        KeyBinding::new("home", FileTreeMoveFirst, context),
        KeyBinding::new("end", FileTreeMoveLast, context),
        KeyBinding::new("pageup", FileTreePageUp, context),
        KeyBinding::new("pagedown", FileTreePageDown, context),
        KeyBinding::new("right", FileTreeExpandOrNext, context),
        KeyBinding::new("left", FileTreeCollapseOrParent, context),
        KeyBinding::new("shift-down", FileTreeExtendNext, context),
        KeyBinding::new("shift-up", FileTreeExtendPrevious, context),
        KeyBinding::new("enter", FileTreeActivateFocused, context),
        KeyBinding::new("space", FileTreeActivateFocused, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-space", FileTreeToggleFocusedSelection, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-space", FileTreeToggleFocusedSelection, context),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", FileTreeSelectAllVisible, context),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", FileTreeSelectAllVisible, context),
    ]);
}
