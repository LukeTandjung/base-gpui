use crate::tabs::{TabsActivationDirection, TabsOrientation, TabsTabPosition, TabsTabSize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsRootStyleState {
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsRootStyleState {
    pub fn new(
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            orientation,
            activation_direction,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsListStyleState {
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsListStyleState {
    pub fn new(
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            orientation,
            activation_direction,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsTabStyleState {
    pub active: bool,
    pub disabled: bool,
    pub highlighted: bool,
    pub orientation: TabsOrientation,
}

impl TabsTabStyleState {
    pub fn new(
        active: bool,
        disabled: bool,
        highlighted: bool,
        orientation: TabsOrientation,
    ) -> Self {
        Self {
            active,
            disabled,
            highlighted,
            orientation,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabsPanelStyleState {
    pub hidden: bool,
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsPanelStyleState {
    pub fn new(
        hidden: bool,
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            hidden,
            orientation,
            activation_direction,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabsIndicatorStyleState {
    pub selected: bool,
    pub active_tab_position: Option<TabsTabPosition>,
    pub active_tab_size: Option<TabsTabSize>,
    pub orientation: TabsOrientation,
    pub activation_direction: TabsActivationDirection,
}

impl TabsIndicatorStyleState {
    pub fn new(
        selected: bool,
        active_tab_position: Option<TabsTabPosition>,
        active_tab_size: Option<TabsTabSize>,
        orientation: TabsOrientation,
        activation_direction: TabsActivationDirection,
    ) -> Self {
        Self {
            selected,
            active_tab_position,
            active_tab_size,
            orientation,
            activation_direction,
        }
    }
}
