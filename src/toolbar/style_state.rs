use crate::{input::InputStyleState, toolbar::ToolbarOrientation};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ToolbarRootStyleState {
    pub disabled: bool,
    pub orientation: ToolbarOrientation,
}

impl ToolbarRootStyleState {
    pub fn new(disabled: bool, orientation: ToolbarOrientation) -> Self {
        Self {
            disabled,
            orientation,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ToolbarButtonStyleState {
    pub disabled: bool,
    pub orientation: ToolbarOrientation,
    /// Whether the item stays focusable while disabled
    /// (`focusable_when_disabled`); `true` for enabled items.
    pub focusable: bool,
    pub focused: bool,
    /// Whether this item currently owns the toolbar's single roving tab stop.
    pub tab_stop: bool,
}

impl ToolbarButtonStyleState {
    pub fn new(
        disabled: bool,
        orientation: ToolbarOrientation,
        focusable: bool,
        focused: bool,
        tab_stop: bool,
    ) -> Self {
        Self {
            disabled,
            orientation,
            focusable,
            focused,
            tab_stop,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ToolbarLinkStyleState {
    pub orientation: ToolbarOrientation,
    pub focused: bool,
    pub tab_stop: bool,
}

impl ToolbarLinkStyleState {
    pub fn new(orientation: ToolbarOrientation, focused: bool, tab_stop: bool) -> Self {
        Self {
            orientation,
            focused,
            tab_stop,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ToolbarInputStyleState {
    pub disabled: bool,
    pub orientation: ToolbarOrientation,
    /// Whether the item stays focusable while disabled
    /// (`focusable_when_disabled`); `true` for enabled items.
    pub focusable: bool,
    /// The reused input component's own style state.
    pub input: InputStyleState,
}

impl ToolbarInputStyleState {
    pub fn new(
        disabled: bool,
        orientation: ToolbarOrientation,
        focusable: bool,
        input: InputStyleState,
    ) -> Self {
        Self {
            disabled,
            orientation,
            focusable,
            input,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ToolbarGroupStyleState {
    pub disabled: bool,
    pub orientation: ToolbarOrientation,
}

impl ToolbarGroupStyleState {
    pub fn new(disabled: bool, orientation: ToolbarOrientation) -> Self {
        Self {
            disabled,
            orientation,
        }
    }
}
