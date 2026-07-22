#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToolbarOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Stable root configuration for a toolbar.
#[derive(Clone, Copy, Debug)]
pub struct ToolbarProps {
    orientation: ToolbarOrientation,
    loop_focus: bool,
    disabled: bool,
}

impl Default for ToolbarProps {
    fn default() -> Self {
        Self {
            orientation: ToolbarOrientation::Horizontal,
            loop_focus: true,
            disabled: false,
        }
    }
}

impl ToolbarProps {
    pub fn new(orientation: ToolbarOrientation, loop_focus: bool, disabled: bool) -> Self {
        Self {
            orientation,
            loop_focus,
            disabled,
        }
    }

    pub fn orientation(&self) -> ToolbarOrientation {
        self.orientation
    }

    pub fn loop_focus(&self) -> bool {
        self.loop_focus
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }
}
