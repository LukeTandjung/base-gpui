use gpui::{Bounds, Pixels, Size};

use crate::{
    navigation_menu::{
        NavigationMenuActivationDirection, NavigationMenuAlign, NavigationMenuInstant,
        NavigationMenuSide,
    },
    utils::PresenceState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuRootStyleState {
    pub open: bool,
    pub nested: bool,
}

impl NavigationMenuRootStyleState {
    pub fn new(open: bool, nested: bool) -> Self {
        Self { open, nested }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuListStyleState {
    pub open: bool,
}

impl NavigationMenuListStyleState {
    pub fn new(open: bool) -> Self {
        Self { open }
    }
}

/// Base UI's item state is empty; the struct exists so `style_with_state`
/// stays extensible.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NavigationMenuItemStyleState {}

impl NavigationMenuItemStyleState {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuTriggerStyleState {
    /// Base UI `data-popup-open`: this trigger's item is the active one.
    pub open: bool,
    pub disabled: bool,
}

impl NavigationMenuTriggerStyleState {
    pub fn new(open: bool, disabled: bool) -> Self {
        Self { open, disabled }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuContentStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
    pub activation_direction: NavigationMenuActivationDirection,
}

impl NavigationMenuContentStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        activation_direction: NavigationMenuActivationDirection,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
            activation_direction,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuPortalStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
}

impl NavigationMenuPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct NavigationMenuPositionerStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: NavigationMenuSide,
    pub align: NavigationMenuAlign,
    pub anchor_hidden: bool,
    pub instant: NavigationMenuInstant,
    pub anchor_bounds: Option<Bounds<Pixels>>,
    pub anchor_width: Option<Pixels>,
    pub anchor_height: Option<Pixels>,
    pub positioner_width: Option<Pixels>,
    pub positioner_height: Option<Pixels>,
    pub available_width: Option<Pixels>,
    pub available_height: Option<Pixels>,
}

impl NavigationMenuPositionerStyleState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
        instant: NavigationMenuInstant,
    ) -> Self {
        Self {
            open,
            mounted,
            side,
            align,
            anchor_hidden: anchor_bounds.is_none(),
            instant,
            anchor_width: anchor_bounds.map(|bounds| bounds.size.width),
            anchor_height: anchor_bounds.map(|bounds| bounds.size.height),
            positioner_width: popup_bounds.map(|bounds| bounds.size.width),
            positioner_height: popup_bounds.map(|bounds| bounds.size.height),
            available_width: available_size.map(|size| size.width),
            available_height: available_size.map(|size| size.height),
            anchor_bounds,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavigationMenuPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
    pub side: NavigationMenuSide,
    pub align: NavigationMenuAlign,
    pub anchor_hidden: bool,
    /// Base UI's `--popup-width`/`--popup-height` concepts as typed fields.
    pub popup_width: Option<Pixels>,
    pub popup_height: Option<Pixels>,
}

impl NavigationMenuPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
        anchor_hidden: bool,
        popup_size: Option<Size<Pixels>>,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
            side,
            align,
            anchor_hidden,
            popup_width: popup_size.map(|size| size.width),
            popup_height: popup_size.map(|size| size.height),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavigationMenuViewportStyleState {
    pub activation_direction: NavigationMenuActivationDirection,
    pub transitioning: bool,
    /// Morph facts: previous popup size (the morph source) and the measured
    /// viewport size (the morph target).
    pub previous_popup_size: Option<Size<Pixels>>,
    pub viewport_size: Option<Size<Pixels>>,
}

impl NavigationMenuViewportStyleState {
    pub fn new(
        activation_direction: NavigationMenuActivationDirection,
        transitioning: bool,
        previous_popup_size: Option<Size<Pixels>>,
        viewport_size: Option<Size<Pixels>>,
    ) -> Self {
        Self {
            activation_direction,
            transitioning,
            previous_popup_size,
            viewport_size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
}

impl NavigationMenuBackdropStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NavigationMenuArrowStyleState {
    pub open: bool,
    pub side: NavigationMenuSide,
    pub align: NavigationMenuAlign,
    pub offset_x: Option<Pixels>,
    pub offset_y: Option<Pixels>,
    pub uncentered: bool,
}

impl NavigationMenuArrowStyleState {
    pub fn new(
        open: bool,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
        offset_x: Option<Pixels>,
        offset_y: Option<Pixels>,
        uncentered: bool,
    ) -> Self {
        Self {
            open,
            side,
            align,
            offset_x,
            offset_y,
            uncentered,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuLinkStyleState {
    pub active: bool,
}

impl NavigationMenuLinkStyleState {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NavigationMenuIconStyleState {
    pub open: bool,
}

impl NavigationMenuIconStyleState {
    pub fn new(open: bool) -> Self {
        Self { open }
    }
}
