use gpui::ElementId;

use crate::utils::PresenceState;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DialogModalMode {
    #[default]
    Modal,
    NonModal,
    TrapFocus,
}

impl DialogModalMode {
    pub fn traps_focus(self) -> bool {
        matches!(self, Self::Modal | Self::TrapFocus)
    }

    pub fn blocks_pointer(self) -> bool {
        matches!(self, Self::Modal)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogRootStyleState<P: Clone + 'static> {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub modal_mode: DialogModalMode,
    pub disable_pointer_dismissal: bool,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
    pub nested: bool,
    pub nested_dialog_count: usize,
    pub nested_dialog_open: bool,
    pub trigger_available: bool,
}

impl<P: Clone + 'static> DialogRootStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        modal_mode: DialogModalMode,
        disable_pointer_dismissal: bool,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
        nested: bool,
        nested_dialog_count: usize,
        trigger_available: bool,
    ) -> Self {
        Self {
            open,
            closed: !open,
            mounted,
            modal_mode,
            disable_pointer_dismissal,
            payload_present: active_payload.is_some(),
            active_payload,
            active_trigger_id,
            nested,
            nested_dialog_count,
            nested_dialog_open: nested_dialog_count > 0,
            trigger_available,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogTriggerStyleState<P: Clone + 'static> {
    pub disabled: bool,
    pub open: bool,
    pub active_trigger: bool,
    pub focused: bool,
    pub payload_present: bool,
    pub payload: Option<P>,
}

impl<P: Clone + 'static> DialogTriggerStyleState<P> {
    pub fn new(
        disabled: bool,
        open: bool,
        active_trigger: bool,
        focused: bool,
        payload_present: bool,
        payload: Option<P>,
    ) -> Self {
        Self {
            disabled,
            open,
            active_trigger,
            focused,
            payload_present,
            payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DialogPortalStyleState {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
}

impl DialogPortalStyleState {
    pub fn new(open: bool, mounted: bool) -> Self {
        Self {
            open,
            closed: !open,
            mounted,
            present: mounted,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DialogBackdropStyleState {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub nested: bool,
    pub force_rendered: bool,
    pub rendered: bool,
}

impl DialogBackdropStyleState {
    pub fn new(open: bool, mounted: bool, nested: bool, force_rendered: bool) -> Self {
        let presence = PresenceState::new(open, mounted);
        let rendered = presence.present && (!nested || force_rendered);

        Self {
            open,
            closed: !open,
            mounted: presence.present,
            present: presence.present,
            transitioning: presence.transitioning,
            nested,
            force_rendered,
            rendered,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogViewportStyleState<P: Clone + 'static> {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub nested: bool,
    pub nested_dialog_open: bool,
    pub nested_dialog_count: usize,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
}

impl<P: Clone + 'static> DialogViewportStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        nested: bool,
        nested_dialog_count: usize,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);

        Self {
            open,
            closed: !open,
            mounted: presence.present,
            present: presence.present,
            transitioning: presence.transitioning,
            nested,
            nested_dialog_open: nested_dialog_count > 0,
            nested_dialog_count,
            active_trigger_id,
            payload_present: active_payload.is_some(),
            active_payload,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogPopupStyleState<P: Clone + 'static> {
    pub open: bool,
    pub closed: bool,
    pub mounted: bool,
    pub present: bool,
    pub transitioning: bool,
    pub nested: bool,
    pub nested_dialog_open: bool,
    pub nested_dialog_count: usize,
    pub active_trigger_id: Option<ElementId>,
    pub active_payload: Option<P>,
    pub payload_present: bool,
    pub modal_mode: DialogModalMode,
}

impl<P: Clone + 'static> DialogPopupStyleState<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: bool,
        mounted: bool,
        nested: bool,
        nested_dialog_count: usize,
        active_trigger_id: Option<ElementId>,
        active_payload: Option<P>,
        modal_mode: DialogModalMode,
    ) -> Self {
        let presence = PresenceState::new(open, mounted);

        Self {
            open,
            closed: !open,
            mounted: presence.present,
            present: presence.present,
            transitioning: presence.transitioning,
            nested,
            nested_dialog_open: nested_dialog_count > 0,
            nested_dialog_count,
            active_trigger_id,
            payload_present: active_payload.is_some(),
            active_payload,
            modal_mode,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogTitleStyleState<P: Clone + 'static> {
    pub open: bool,
    pub active_payload: Option<P>,
    pub payload_present: bool,
}

impl<P: Clone + 'static> DialogTitleStyleState<P> {
    pub fn new(open: bool, active_payload: Option<P>) -> Self {
        Self {
            open,
            payload_present: active_payload.is_some(),
            active_payload,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogDescriptionStyleState<P: Clone + 'static> {
    pub open: bool,
    pub active_payload: Option<P>,
    pub payload_present: bool,
}

impl<P: Clone + 'static> DialogDescriptionStyleState<P> {
    pub fn new(open: bool, active_payload: Option<P>) -> Self {
        Self {
            open,
            payload_present: active_payload.is_some(),
            active_payload,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DialogCloseStyleState {
    pub disabled: bool,
    pub open: bool,
    pub focused: bool,
}

impl DialogCloseStyleState {
    pub fn new(disabled: bool, open: bool, focused: bool) -> Self {
        Self {
            disabled,
            open,
            focused,
        }
    }
}
