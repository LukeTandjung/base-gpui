use std::time::Duration;

use gpui::{Bounds, ElementId, Pixels, Size};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipProviderStyleState {
    pub delay: Duration,
    pub close_delay: Duration,
    pub timeout: Duration,
    pub instant: TooltipInstant,
}

impl TooltipProviderStyleState {
    pub fn new(
        delay: Duration,
        close_delay: Duration,
        timeout: Duration,
        instant: TooltipInstant,
    ) -> Self {
        Self {
            delay,
            close_delay,
            timeout,
            instant,
        }
    }
}

use crate::{
    tooltip::{
        TooltipActivationDirection, TooltipAlign, TooltipInstant, TooltipOpenChangeSource,
        TooltipSide,
    },
    utils::PresenceState,
};

#[derive(Clone)]
pub struct TooltipRootStyleState<P: Clone + 'static> {
    pub open: bool,
    pub mounted: bool,
    pub disabled: bool,
    pub open_source: TooltipOpenChangeSource,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
    pub trigger_available: bool,
    pub instant: TooltipInstant,
}

impl<P: Clone + 'static> TooltipRootStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        disabled: bool,
        open_source: TooltipOpenChangeSource,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
        trigger_available: bool,
        instant: TooltipInstant,
    ) -> Self {
        let payload_present = active_payload.is_some();
        Self {
            open,
            mounted,
            disabled,
            open_source,
            active_trigger_id,
            active_payload,
            payload_present,
            trigger_available,
            instant,
        }
    }
}

#[derive(Clone)]
pub struct TooltipTriggerStyleState<P: Clone + 'static> {
    pub disabled: bool,
    pub open: bool,
    pub active_trigger: bool,
    pub focused: bool,
    pub hovered: bool,
    pub trigger_id: ElementId,
    pub payload_present: bool,
    pub payload: Option<P>,
}

impl<P: Clone + 'static> TooltipTriggerStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disabled: bool,
        open: bool,
        active_trigger: bool,
        focused: bool,
        hovered: bool,
        trigger_id: ElementId,
        payload_present: bool,
        payload: Option<P>,
    ) -> Self {
        Self {
            disabled,
            open,
            active_trigger,
            focused,
            hovered,
            trigger_id,
            payload_present,
            payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl TooltipPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, PartialEq)]
pub struct TooltipPositionerStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: TooltipSide,
    pub align: TooltipAlign,
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
    pub instant: TooltipInstant,
}

impl TooltipPositionerStyleState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        side: TooltipSide,
        align: TooltipAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
        instant: TooltipInstant,
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

fn transform_origin_percent(side: TooltipSide, align: TooltipAlign) -> (f32, f32) {
    let cross_axis = match align {
        TooltipAlign::Start => 0.0,
        TooltipAlign::Center => 50.0,
        TooltipAlign::End => 100.0,
    };

    match side {
        TooltipSide::Top => (cross_axis, 100.0),
        TooltipSide::Bottom => (cross_axis, 0.0),
        TooltipSide::Left | TooltipSide::InlineStart => (100.0, cross_axis),
        TooltipSide::Right | TooltipSide::InlineEnd => (0.0, cross_axis),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TooltipPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: TooltipSide,
    pub align: TooltipAlign,
    pub transitioning: bool,
    pub instant: TooltipInstant,
}

impl TooltipPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: TooltipSide,
        align: TooltipAlign,
        instant: TooltipInstant,
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

#[derive(Clone, Debug, PartialEq)]
pub struct TooltipViewportStyleState {
    pub activation_direction: TooltipActivationDirection,
    pub transitioning: bool,
    pub instant: TooltipInstant,
    pub previous_trigger_id: Option<ElementId>,
    pub current_trigger_id: Option<ElementId>,
    pub previous_popup_size: Option<Size<Pixels>>,
    pub current_popup_size: Option<Size<Pixels>>,
}

impl TooltipViewportStyleState {
    pub fn new(
        activation_direction: TooltipActivationDirection,
        transitioning: bool,
        instant: TooltipInstant,
        previous_trigger_id: Option<ElementId>,
        current_trigger_id: Option<ElementId>,
        previous_popup_size: Option<Size<Pixels>>,
        current_popup_size: Option<Size<Pixels>>,
    ) -> Self {
        Self {
            activation_direction,
            transitioning,
            instant,
            previous_trigger_id,
            current_trigger_id,
            previous_popup_size,
            current_popup_size,
        }
    }
}
