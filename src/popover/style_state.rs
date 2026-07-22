use gpui::{Bounds, ElementId, Pixels, Size};

use crate::{
    popover::{PopoverActivationDirection, PopoverAlign, PopoverOpenChangeSource, PopoverSide},
    utils::PresenceState,
};

#[derive(Clone)]
pub struct PopoverRootStyleState<P: Clone + 'static> {
    pub open: bool,
    pub mounted: bool,
    pub modal: bool,
    pub open_source: PopoverOpenChangeSource,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
    pub trigger_available: bool,
}

impl<P: Clone + 'static> PopoverRootStyleState<P> {
    pub fn new(
        open: bool,
        mounted: bool,
        modal: bool,
        open_source: PopoverOpenChangeSource,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
        trigger_available: bool,
    ) -> Self {
        let payload_present = active_payload.is_some();
        Self {
            open,
            mounted,
            modal,
            open_source,
            active_trigger_id,
            active_payload,
            payload_present,
            trigger_available,
        }
    }
}

#[derive(Clone)]
pub struct PopoverTriggerStyleState<P: Clone + 'static> {
    pub disabled: bool,
    pub open: bool,
    pub active_trigger: bool,
    pub pressed: bool,
    pub focused: bool,
    pub payload_present: bool,
    pub payload: Option<P>,
}

impl<P: Clone + 'static> PopoverTriggerStyleState<P> {
    pub fn new(
        disabled: bool,
        open: bool,
        active_trigger: bool,
        pressed: bool,
        focused: bool,
        payload_present: bool,
        payload: Option<P>,
    ) -> Self {
        Self {
            disabled,
            open,
            active_trigger,
            pressed,
            focused,
            payload_present,
            payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl PopoverPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
    pub interactive: bool,
}

impl PopoverBackdropStyleState {
    pub fn new(open: bool, mounted: bool, interactive: bool) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            transitioning: presence.transitioning,
            interactive,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PopoverPositionerStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub anchor_hidden: bool,
    pub anchor_available: bool,
    pub anchor_bounds: Option<Bounds<Pixels>>,
    pub popup_bounds: Option<Bounds<Pixels>>,
    pub available_size: Option<Size<Pixels>>,
    pub anchor_width: Option<Pixels>,
    pub anchor_height: Option<Pixels>,
    pub popup_width: Option<Pixels>,
    pub popup_height: Option<Pixels>,
    pub available_width: Option<Pixels>,
    pub available_height: Option<Pixels>,
    pub transform_origin_x_percent: f32,
    pub transform_origin_y_percent: f32,
    pub instant: bool,
}

impl PopoverPositionerStyleState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        side: PopoverSide,
        align: PopoverAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
        instant: bool,
    ) -> Self {
        let (transform_origin_x_percent, transform_origin_y_percent) =
            transform_origin_percent(side, align);
        Self {
            open,
            mounted,
            side,
            align,
            anchor_hidden: anchor_bounds.is_none(),
            anchor_available: anchor_bounds.is_some(),
            anchor_width: anchor_bounds.map(|bounds| bounds.size.width),
            anchor_height: anchor_bounds.map(|bounds| bounds.size.height),
            popup_width: popup_bounds.map(|bounds| bounds.size.width),
            popup_height: popup_bounds.map(|bounds| bounds.size.height),
            available_width: available_size.map(|size| size.width),
            available_height: available_size.map(|size| size.height),
            anchor_bounds,
            popup_bounds,
            available_size,
            transform_origin_x_percent,
            transform_origin_y_percent,
            instant,
        }
    }
}

fn transform_origin_percent(side: PopoverSide, align: PopoverAlign) -> (f32, f32) {
    let cross_axis = match align {
        PopoverAlign::Start => 0.0,
        PopoverAlign::Center => 50.0,
        PopoverAlign::End => 100.0,
    };

    match side {
        PopoverSide::Top => (cross_axis, 100.0),
        PopoverSide::Bottom => (cross_axis, 0.0),
        PopoverSide::Left | PopoverSide::InlineStart => (100.0, cross_axis),
        PopoverSide::Right | PopoverSide::InlineEnd => (0.0, cross_axis),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub transitioning: bool,
    pub instant: bool,
}

impl PopoverPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: PopoverSide,
        align: PopoverAlign,
        instant: bool,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);
        Self {
            open,
            mounted: presence.present,
            side,
            align,
            transitioning: presence.transitioning,
            instant,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverArrowStyleState {
    pub open: bool,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub offset_x: Option<Pixels>,
    pub offset_y: Option<Pixels>,
    pub padding: Pixels,
    pub uncentered: bool,
}

impl PopoverArrowStyleState {
    pub fn new(
        open: bool,
        side: PopoverSide,
        align: PopoverAlign,
        offset_x: Option<Pixels>,
        offset_y: Option<Pixels>,
        padding: Pixels,
        uncentered: bool,
    ) -> Self {
        Self {
            open,
            side,
            align,
            offset_x,
            offset_y,
            padding,
            uncentered,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverTitleStyleState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverDescriptionStyleState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PopoverCloseStyleState {
    pub disabled: bool,
    pub open: bool,
}

impl PopoverCloseStyleState {
    pub fn new(disabled: bool, open: bool) -> Self {
        Self { disabled, open }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverViewportStyleState {
    pub activation_direction: PopoverActivationDirection,
    pub transitioning: bool,
    pub instant: bool,
    pub popup_size: Option<Size<Pixels>>,
}

impl PopoverViewportStyleState {
    pub fn new(
        activation_direction: PopoverActivationDirection,
        transitioning: bool,
        instant: bool,
        popup_size: Option<Size<Pixels>>,
    ) -> Self {
        Self {
            activation_direction,
            transitioning,
            instant,
            popup_size,
        }
    }
}
