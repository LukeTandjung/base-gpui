use gpui::{AnyElement, IntoElement};

use crate::toolbar::layers::{
    ToolbarButton, ToolbarGroup, ToolbarInput, ToolbarLink, ToolbarSeparator,
};

/// Typed direct children of `ToolbarRoot`. Buttons, links, and inputs occupy
/// roving slots; groups and separators do not. Designed to gain
/// `Toggle(...)` and `ToggleGroup(...)` variants later: a nested ToggleGroup
/// contributes its child toggles as individual flattened toolbar items (one
/// roving slot per toggle, none for the group container). No `AnyElement`
/// escape hatch for now; trigger-hosting wrappers are deferred until
/// menu/select trigger hosting is designed.
pub enum ToolbarChild {
    Button(ToolbarButton),
    Link(ToolbarLink),
    Input(ToolbarInput),
    Group(ToolbarGroup),
    Separator(ToolbarSeparator),
}

impl IntoElement for ToolbarChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Button(button) => button.into_any_element(),
            Self::Link(link) => link.into_any_element(),
            Self::Input(input) => input.into_any_element(),
            Self::Group(group) => group.into_any_element(),
            Self::Separator(separator) => separator.into_any_element(),
        }
    }
}

impl From<ToolbarButton> for ToolbarChild {
    fn from(value: ToolbarButton) -> Self {
        Self::Button(value)
    }
}

impl From<ToolbarLink> for ToolbarChild {
    fn from(value: ToolbarLink) -> Self {
        Self::Link(value)
    }
}

impl From<ToolbarInput> for ToolbarChild {
    fn from(value: ToolbarInput) -> Self {
        Self::Input(value)
    }
}

impl From<ToolbarGroup> for ToolbarChild {
    fn from(value: ToolbarGroup) -> Self {
        Self::Group(value)
    }
}

impl From<ToolbarSeparator> for ToolbarChild {
    fn from(value: ToolbarSeparator) -> Self {
        Self::Separator(value)
    }
}

/// Typed children of `ToolbarGroup`. Group children register directly as
/// toolbar items with flattened indices; the group container occupies no
/// roving slot. Designed to gain a `Toggle(...)` variant later.
pub enum ToolbarGroupChild {
    Button(ToolbarButton),
    Link(ToolbarLink),
    Input(ToolbarInput),
}

impl IntoElement for ToolbarGroupChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Button(button) => button.into_any_element(),
            Self::Link(link) => link.into_any_element(),
            Self::Input(input) => input.into_any_element(),
        }
    }
}

impl From<ToolbarButton> for ToolbarGroupChild {
    fn from(value: ToolbarButton) -> Self {
        Self::Button(value)
    }
}

impl From<ToolbarLink> for ToolbarGroupChild {
    fn from(value: ToolbarLink) -> Self {
        Self::Link(value)
    }
}

impl From<ToolbarInput> for ToolbarGroupChild {
    fn from(value: ToolbarInput) -> Self {
        Self::Input(value)
    }
}
