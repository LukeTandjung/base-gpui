use gpui::App;

/// Alert Dialog reuses the Dialog actions and keybindings verbatim
/// (`DialogOpenAction`, `DialogCloseAction`, focus-cycling actions), which are
/// registered by `crate::dialog::init`. `base_gpui::init` calls both, so this
/// init has nothing extra to register; it exists to keep the module's
/// registration shape consistent with other components.
pub fn init(_cx: &mut App) {}
