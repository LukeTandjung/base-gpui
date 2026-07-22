#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenubarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Stable root configuration for a menubar. Note the horizontal default —
/// the opposite of `MenuRoot`'s vertical orientation default.
#[derive(Clone, Copy, Debug)]
pub struct MenubarProps {
    orientation: MenubarOrientation,
    loop_focus: bool,
    modal: bool,
    disabled: bool,
}

impl Default for MenubarProps {
    fn default() -> Self {
        Self {
            orientation: MenubarOrientation::Horizontal,
            loop_focus: true,
            modal: true,
            disabled: false,
        }
    }
}

impl MenubarProps {
    pub fn new(
        orientation: MenubarOrientation,
        loop_focus: bool,
        modal: bool,
        disabled: bool,
    ) -> Self {
        Self {
            orientation,
            loop_focus,
            modal,
            disabled,
        }
    }

    pub fn orientation(&self) -> MenubarOrientation {
        self.orientation
    }

    pub fn loop_focus(&self) -> bool {
        self.loop_focus
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}
