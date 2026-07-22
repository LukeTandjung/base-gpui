use gpui::{Bounds, ElementId, Pixels, Size};

use crate::{
    preview_card::{
        PreviewCardActivationDirection, PreviewCardAlign, PreviewCardInstant, PreviewCardSide,
    },
    utils::PresenceState,
};

#[derive(Clone)]
pub struct PreviewCardTriggerStyleState<P: Clone + 'static> {
    /// Base UI `data-popup-open`: the card is open and this trigger is active.
    pub open: bool,
    pub active_trigger: bool,
    pub focused: bool,
    pub hovered: bool,
    pub trigger_id: ElementId,
    pub payload_present: bool,
    pub payload: Option<P>,
}

impl<P: Clone + 'static> PreviewCardTriggerStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        active_trigger: bool,
        focused: bool,
        hovered: bool,
        trigger_id: ElementId,
        payload_present: bool,
        payload: Option<P>,
    ) -> Self {
        Self {
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
pub struct PreviewCardPortalStyleState {
    pub open: bool,
    pub mounted: bool,
}

impl PreviewCardPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self { open, mounted }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreviewCardBackdropStyleState {
    pub open: bool,
    pub mounted: bool,
    pub transitioning: bool,
}

impl PreviewCardBackdropStyleState {
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
pub struct PreviewCardPositionerStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: PreviewCardSide,
    pub align: PreviewCardAlign,
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
    pub instant: PreviewCardInstant,
}

impl PreviewCardPositionerStyleState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        side: PreviewCardSide,
        align: PreviewCardAlign,
        anchor_bounds: Option<Bounds<Pixels>>,
        popup_bounds: Option<Bounds<Pixels>>,
        available_size: Option<Size<Pixels>>,
        instant: PreviewCardInstant,
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

fn transform_origin_percent(side: PreviewCardSide, align: PreviewCardAlign) -> (f32, f32) {
    let cross_axis = match align {
        PreviewCardAlign::Start => 0.0,
        PreviewCardAlign::Center => 50.0,
        PreviewCardAlign::End => 100.0,
    };

    match side {
        PreviewCardSide::Top => (cross_axis, 100.0),
        PreviewCardSide::Bottom => (cross_axis, 0.0),
        PreviewCardSide::Left | PreviewCardSide::InlineStart => (100.0, cross_axis),
        PreviewCardSide::Right | PreviewCardSide::InlineEnd => (0.0, cross_axis),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreviewCardPopupStyleState {
    pub open: bool,
    pub mounted: bool,
    pub side: PreviewCardSide,
    pub align: PreviewCardAlign,
    pub transitioning: bool,
    pub instant: PreviewCardInstant,
}

impl PreviewCardPopupStyleState {
    pub fn new(
        open: bool,
        mounted: bool,
        side: PreviewCardSide,
        align: PreviewCardAlign,
        instant: PreviewCardInstant,
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
pub struct PreviewCardArrowStyleState {
    pub open: bool,
    pub side: PreviewCardSide,
    pub align: PreviewCardAlign,
    pub offset_x: Option<Pixels>,
    pub offset_y: Option<Pixels>,
    pub padding: Pixels,
    pub uncentered: bool,
}

impl PreviewCardArrowStyleState {
    pub fn new(
        open: bool,
        side: PreviewCardSide,
        align: PreviewCardAlign,
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

#[derive(Clone, Debug, PartialEq)]
pub struct PreviewCardViewportStyleState {
    pub activation_direction: PreviewCardActivationDirection,
    pub transitioning: bool,
    pub instant: PreviewCardInstant,
    pub previous_trigger_id: Option<ElementId>,
    pub current_trigger_id: Option<ElementId>,
    pub previous_popup_size: Option<Size<Pixels>>,
    pub current_popup_size: Option<Size<Pixels>>,
}

impl PreviewCardViewportStyleState {
    pub fn new(
        activation_direction: PreviewCardActivationDirection,
        transitioning: bool,
        instant: PreviewCardInstant,
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
