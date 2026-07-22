use crate::menubar::MenubarOrientation;

/// Typed state for `Menubar::style_with_state`. Base UI's `data-orientation`,
/// `data-modal`, and `data-has-submenu-open` attributes map to these fields.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MenubarStyleState {
    pub orientation: MenubarOrientation,
    pub modal: bool,
    pub has_submenu_open: bool,
    pub disabled: bool,
}

impl MenubarStyleState {
    pub fn new(
        orientation: MenubarOrientation,
        modal: bool,
        has_submenu_open: bool,
        disabled: bool,
    ) -> Self {
        Self {
            orientation,
            modal,
            has_submenu_open,
            disabled,
        }
    }
}
