use gpui::{Bounds, Pixels, Size};

use crate::menu::{MenuAlign, MenuInstantKind, MenuParentKind, MenuSide};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuRootStyleState {
    pub open: bool,
    pub disabled: bool,
    pub parent_kind: MenuParentKind,
    pub instant: MenuInstantKind,
}

impl MenuRootStyleState {
    pub fn new(
        open: bool,
        disabled: bool,
        parent_kind: MenuParentKind,
        instant: MenuInstantKind,
    ) -> Self {
        Self {
            open,
            disabled,
            parent_kind,
            instant,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MenuTriggerStyleState<P: Clone + 'static> {
    pub open: bool,
    pub disabled: bool,
    pub active_trigger: bool,
    pub payload_present: bool,
    pub payload: Option<P>,
    pub focused: bool,
}

impl<P: Clone + 'static> MenuTriggerStyleState<P> {
    pub fn new(
        open: bool,
        disabled: bool,
        active_trigger: bool,
        payload_present: bool,
        payload: Option<P>,
    ) -> Self {
        Self {
            open,
            disabled,
            active_trigger,
            payload_present,
            payload,
            focused: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl MenuPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    /// False when the menu was opened by hover: Base UI makes a hover-opened
    /// menu's backdrop pointer-inert.
    pub interactive: bool,
}

impl MenuBackdropStyleState {
    pub fn new(open: bool, mounted: bool, interactive: bool) -> Self {
        Self {
            open,
            mounted,
            interactive,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuPositionerStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: MenuSide,
    pub align: MenuAlign,
    pub anchor_hidden: bool,
    pub nested: bool,
    pub instant: MenuInstantKind,
    pub anchor_bounds: Option<Bounds<Pixels>>,
    pub popup_bounds: Option<Bounds<Pixels>>,
    pub available_size: Option<Size<Pixels>>,
}

impl MenuPositionerStyleState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        side: MenuSide,
        align: MenuAlign,
        anchor_hidden: bool,
        nested: bool,
        instant: MenuInstantKind,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
    ) -> Self {
        Self {
            open,
            mounted,
            side,
            align,
            anchor_hidden,
            nested,
            instant,
            anchor_bounds,
            popup_bounds,
            available_size,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: MenuSide,
    pub align: MenuAlign,
    pub nested: bool,
    pub instant: MenuInstantKind,
}

impl MenuPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: MenuSide,
        align: MenuAlign,
        nested: bool,
        instant: MenuInstantKind,
    ) -> Self {
        Self {
            open,
            mounted,
            side,
            align,
            nested,
            instant,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MenuArrowStyleState {
    pub open: bool,
    pub side: MenuSide,
    pub align: MenuAlign,
    pub offset_x: Option<Pixels>,
    pub offset_y: Option<Pixels>,
    pub uncentered: bool,
}

impl MenuArrowStyleState {
    pub fn new(
        open: bool,
        side: MenuSide,
        align: MenuAlign,
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
pub struct MenuItemStyleState {
    pub highlighted: bool,
    pub disabled: bool,
}

impl MenuItemStyleState {
    pub fn new(highlighted: bool, disabled: bool) -> Self {
        Self {
            highlighted,
            disabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuLinkItemStyleState {
    pub highlighted: bool,
}

impl MenuLinkItemStyleState {
    pub fn new(highlighted: bool) -> Self {
        Self { highlighted }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuCheckboxItemStyleState {
    pub checked: bool,
    pub highlighted: bool,
    pub disabled: bool,
}

impl MenuCheckboxItemStyleState {
    pub fn new(checked: bool, highlighted: bool, disabled: bool) -> Self {
        Self {
            checked,
            highlighted,
            disabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuCheckboxItemIndicatorStyleState {
    pub checked: bool,
    pub highlighted: bool,
    pub disabled: bool,
    pub present: bool,
}

impl MenuCheckboxItemIndicatorStyleState {
    pub fn new(checked: bool, highlighted: bool, disabled: bool, present: bool) -> Self {
        Self {
            checked,
            highlighted,
            disabled,
            present,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuRadioGroupStyleState {
    pub disabled: bool,
}

impl MenuRadioGroupStyleState {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuRadioItemStyleState {
    pub checked: bool,
    pub highlighted: bool,
    pub disabled: bool,
}

impl MenuRadioItemStyleState {
    pub fn new(checked: bool, highlighted: bool, disabled: bool) -> Self {
        Self {
            checked,
            highlighted,
            disabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuRadioItemIndicatorStyleState {
    pub checked: bool,
    pub highlighted: bool,
    pub disabled: bool,
    pub present: bool,
}

impl MenuRadioItemIndicatorStyleState {
    pub fn new(checked: bool, highlighted: bool, disabled: bool, present: bool) -> Self {
        Self {
            checked,
            highlighted,
            disabled,
            present,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuSubmenuTriggerStyleState {
    pub open: bool,
    pub highlighted: bool,
    pub disabled: bool,
}

impl MenuSubmenuTriggerStyleState {
    pub fn new(open: bool, highlighted: bool, disabled: bool) -> Self {
        Self {
            open,
            highlighted,
            disabled,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuGroupStyleState {
    pub disabled: bool,
}

impl MenuGroupStyleState {
    pub fn new(disabled: bool) -> Self {
        Self { disabled }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MenuGroupLabelStyleState;
