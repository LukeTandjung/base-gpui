use std::time::Duration;

use gpui::{px, size, Bounds, ElementId, FocusHandle, Pixels, Point, SharedString, Size};

use crate::tooltip::{
    TooltipAlign, TooltipInstant, TooltipPopupStyleState, TooltipPortalStyleState,
    TooltipPositionerStyleState, TooltipProps, TooltipRootStyleState, TooltipSide,
    TooltipTrackCursorAxis, TooltipTriggerStyleState, TooltipViewportStyleState,
};

const TOOLTIP_SAFE_GAP_CLOSE_DELAY: Duration = Duration::from_millis(120);
const TOOLTIP_SAFE_GAP_PADDING: Pixels = px(8.0);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipOpenChangeReason {
    TriggerHover,
    TriggerFocus,
    TriggerPress,
    OutsidePress,
    EscapeKey,
    Disabled,
    ImperativeAction,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipOpenChangeSource {
    Pointer,
    Touch,
    Keyboard,
    Focus,
    Imperative,
    Unknown,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TooltipActivationDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
    #[default]
    None,
}

#[derive(Clone)]
pub struct TooltipOpenChangeDetails<P: Clone + 'static> {
    reason: TooltipOpenChangeReason,
    source: TooltipOpenChangeSource,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
    cancelable: bool,
    canceled: bool,
    prevent_unmount_on_close: bool,
}

impl<P: Clone + 'static> TooltipOpenChangeDetails<P> {
    pub fn new(
        reason: TooltipOpenChangeReason,
        source: TooltipOpenChangeSource,
        trigger_id: Option<ElementId>,
        payload: Option<P>,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            trigger_id,
            payload,
            cancelable,
            canceled: false,
            prevent_unmount_on_close: false,
        }
    }

    pub fn reason(&self) -> TooltipOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> TooltipOpenChangeSource {
        self.source
    }

    pub fn trigger_id(&self) -> Option<&ElementId> {
        self.trigger_id.as_ref()
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn cancelable(&self) -> bool {
        self.cancelable
    }

    pub fn cancel(&mut self) {
        if self.cancelable {
            self.canceled = true;
        }
    }

    pub fn is_canceled(&self) -> bool {
        self.canceled
    }

    pub fn prevent_unmount_on_close(&mut self) {
        self.prevent_unmount_on_close = true;
    }

    pub fn prevents_unmount_on_close(&self) -> bool {
        self.prevent_unmount_on_close
    }
}

#[derive(Clone)]
pub struct TooltipTriggerMetadata<P: Clone + 'static> {
    id: ElementId,
    source_id: ElementId,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
    disabled: bool,
    delay: Duration,
    close_delay: Duration,
    close_on_click: bool,
    payload: Option<P>,
    order: usize,
    detached: bool,
    seen_generation: u64,
}

impl<P: Clone + 'static> TooltipTriggerMetadata<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        delay: Duration,
        close_delay: Duration,
        close_on_click: bool,
        payload: Option<P>,
        order: usize,
        detached: bool,
    ) -> Self {
        Self {
            id,
            source_id,
            focus_handle: Some(focus_handle),
            bounds: None,
            disabled,
            delay,
            close_delay,
            close_on_click,
            payload,
            order,
            detached,
            seen_generation: 0,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_without_focus(
        id: ElementId,
        source_id: ElementId,
        disabled: bool,
        delay: Duration,
        close_delay: Duration,
        close_on_click: bool,
        payload: Option<P>,
        order: usize,
        detached: bool,
    ) -> Self {
        Self {
            id,
            source_id,
            focus_handle: None,
            bounds: None,
            disabled,
            delay,
            close_delay,
            close_on_click,
            payload,
            order,
            detached,
            seen_generation: 0,
        }
    }

    pub fn id(&self) -> &ElementId {
        &self.id
    }

    pub fn source_id(&self) -> &ElementId {
        &self.source_id
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
    }

    pub fn close_on_click(&self) -> bool {
        self.close_on_click
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn detached(&self) -> bool {
        self.detached
    }

    pub fn seen_generation(&self) -> u64 {
        self.seen_generation
    }

    pub fn with_seen_generation(mut self, seen_generation: u64) -> Self {
        self.seen_generation = seen_generation;
        self
    }

    pub fn with_bounds(mut self, bounds: Option<Bounds<Pixels>>) -> Self {
        self.bounds = bounds;
        self
    }
}

pub struct TooltipOpenChangeOutcome<P: Clone + 'static> {
    changed: bool,
    open: bool,
    trigger_id: Option<ElementId>,
    source_trigger_id: Option<ElementId>,
    payload: Option<P>,
}

impl<P: Clone + 'static> TooltipOpenChangeOutcome<P> {
    fn new(
        changed: bool,
        open: bool,
        trigger_id: Option<ElementId>,
        source_trigger_id: Option<ElementId>,
        payload: Option<P>,
    ) -> Self {
        Self {
            changed,
            open,
            trigger_id,
            source_trigger_id,
            payload,
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn open(&self) -> bool {
        self.open
    }

    pub fn trigger_id(&self) -> Option<&ElementId> {
        self.trigger_id.as_ref()
    }

    pub fn source_trigger_id(&self) -> Option<&ElementId> {
        self.source_trigger_id.as_ref()
    }

    pub fn payload(&self) -> Option<&P> {
        self.payload.as_ref()
    }

    pub fn into_parts(self) -> (bool, Option<ElementId>, Option<ElementId>, Option<P>) {
        (
            self.open,
            self.trigger_id,
            self.source_trigger_id,
            self.payload,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TooltipHoverTarget {
    Open,
    Close,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TooltipBoundsKind {
    Trigger(ElementId),
    Popup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TooltipPendingHover {
    generation: u64,
    target: TooltipHoverTarget,
    trigger_id: Option<ElementId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TooltipFocusChange {
    Open(ElementId),
    Close,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TooltipTriggerPressChange {
    close_active: bool,
    open_detached_focus: bool,
}

impl TooltipTriggerPressChange {
    fn new(close_active: bool, open_detached_focus: bool) -> Self {
        Self {
            close_active,
            open_detached_focus,
        }
    }

    pub fn close_active(&self) -> bool {
        self.close_active
    }

    pub fn open_detached_focus(&self) -> bool {
        self.open_detached_focus
    }
}

#[derive(Clone)]
pub struct TooltipRuntime<P: Clone + 'static> {
    open: bool,
    disabled: bool,
    disable_hoverable_popup: bool,
    track_cursor_axis: TooltipTrackCursorAxis,
    last_open_source: TooltipOpenChangeSource,
    last_open_reason: TooltipOpenChangeReason,
    instant: TooltipInstant,
    provider_instant: TooltipInstant,
    provider_active_root_id: Option<ElementId>,
    prevent_unmount_on_close: bool,
    active_trigger_id: Option<ElementId>,
    previous_trigger_id: Option<ElementId>,
    triggers: Vec<TooltipTriggerMetadata<P>>,
    detached_trigger_generation: u64,
    focused_trigger_id: Option<ElementId>,
    hovered_trigger_id: Option<ElementId>,
    popup_hovered: bool,
    press_suppressed_trigger_id: Option<ElementId>,
    pointer_focused_trigger_id: Option<ElementId>,
    active_trigger_missing_close_requested: bool,
    popup_bounds: Option<Bounds<Pixels>>,
    previous_popup_size: Option<Size<Pixels>>,
    available_size: Option<Size<Pixels>>,
    effective_side: Option<TooltipSide>,
    effective_align: Option<TooltipAlign>,
    cursor_position: Option<Point<Pixels>>,
    hover_generation: u64,
    pending_hover: Option<TooltipPendingHover>,
}

impl<P: Clone + 'static> TooltipRuntime<P> {
    pub fn new(
        open: bool,
        active_trigger_id: Option<ElementId>,
        disabled: bool,
        disable_hoverable_popup: bool,
        track_cursor_axis: TooltipTrackCursorAxis,
    ) -> Self {
        let open = open && !disabled;
        Self {
            open,
            disabled,
            disable_hoverable_popup,
            track_cursor_axis,
            last_open_source: TooltipOpenChangeSource::None,
            last_open_reason: TooltipOpenChangeReason::None,
            instant: TooltipInstant::Delay,
            provider_instant: TooltipInstant::Delay,
            provider_active_root_id: None,
            prevent_unmount_on_close: false,
            active_trigger_id,
            previous_trigger_id: None,
            triggers: Vec::new(),
            detached_trigger_generation: 0,
            focused_trigger_id: None,
            hovered_trigger_id: None,
            popup_hovered: false,
            press_suppressed_trigger_id: None,
            pointer_focused_trigger_id: None,
            active_trigger_missing_close_requested: false,
            popup_bounds: None,
            previous_popup_size: None,
            available_size: None,
            effective_side: None,
            effective_align: None,
            cursor_position: None,
            hover_generation: 0,
            pending_hover: None,
        }
    }

    pub fn open_value(&self) -> bool {
        self.open && !self.disabled
    }

    pub fn raw_open_value(&self) -> bool {
        self.open
    }

    pub fn mounted_value(&self, keep_mounted: bool) -> bool {
        self.open_value() || keep_mounted || self.prevent_unmount_on_close
    }

    pub fn active_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger_id.clone()
    }

    pub fn active_source_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger()
            .map(|trigger| trigger.source_id().clone())
            .or_else(|| self.active_trigger_id.clone())
    }

    pub fn active_payload(&self) -> Option<P> {
        self.active_trigger()
            .and_then(|trigger| trigger.payload.clone())
    }

    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open && !self.disabled;
        if self.open {
            self.prevent_unmount_on_close = false;
        }
    }

    pub fn sync_trigger_id_from_context(&mut self, trigger_id: Option<ElementId>) {
        if self.active_trigger_id != trigger_id {
            self.previous_trigger_id = self.active_trigger_id.clone();
            self.active_trigger_id = trigger_id;
        }
    }

    pub fn reconcile(
        &mut self,
        disabled: bool,
        disable_hoverable_popup: bool,
        track_cursor_axis: TooltipTrackCursorAxis,
    ) -> bool {
        self.sync_root_options(disabled, disable_hoverable_popup, track_cursor_axis)
    }

    pub fn sync_root_options(
        &mut self,
        disabled: bool,
        disable_hoverable_popup: bool,
        track_cursor_axis: TooltipTrackCursorAxis,
    ) -> bool {
        let requested_close = !self.disabled && disabled && self.open;
        self.disabled = disabled;
        self.disable_hoverable_popup = disable_hoverable_popup;
        self.track_cursor_axis = track_cursor_axis;
        if disabled {
            self.cancel_hover();
        }
        requested_close
    }

    pub fn sync_children(&mut self, triggers: Vec<TooltipTriggerMetadata<P>>) {
        self.sync_triggers(triggers);
    }

    pub fn sync_provider_delay_group(
        &mut self,
        instant: TooltipInstant,
        active_root_id: Option<ElementId>,
    ) -> bool {
        let changed =
            self.provider_instant != instant || self.provider_active_root_id != active_root_id;
        self.provider_instant = instant;
        self.provider_active_root_id = active_root_id;
        changed
    }

    pub fn provider_active_root_id(&self) -> Option<ElementId> {
        self.provider_active_root_id.clone()
    }

    pub fn begin_detached_trigger_collection(&mut self) {
        self.detached_trigger_generation = self.detached_trigger_generation.wrapping_add(1);
    }

    pub fn sync_triggers(&mut self, mut triggers: Vec<TooltipTriggerMetadata<P>>) {
        let previous_triggers = self.triggers.clone();
        for trigger in &mut triggers {
            if let Some(bounds) = previous_triggers
                .iter()
                .rev()
                .find(|candidate| candidate.id() == trigger.id())
                .and_then(TooltipTriggerMetadata::bounds)
            {
                trigger.bounds = Some(bounds);
            }
        }

        let mut detached = previous_triggers
            .iter()
            .filter(|trigger| trigger.detached())
            .cloned()
            .collect::<Vec<_>>();
        triggers.append(&mut detached);
        self.triggers = triggers;

        if self.open_value() && self.active_trigger_id.is_none() {
            self.active_trigger_id = self.first_enabled_trigger_id();
        }

        if self.open_value()
            && self
                .active_trigger_id
                .as_ref()
                .map(|id| self.trigger_by_id(id).is_none())
                .unwrap_or(false)
        {
            self.active_trigger_missing_close_requested = true;
        }

        if self.open_value()
            && self
                .active_trigger_id
                .as_ref()
                .and_then(|id| self.trigger_by_id(id))
                .map(|trigger| {
                    trigger.detached()
                        && trigger.seen_generation().saturating_add(1)
                            < self.detached_trigger_generation
                })
                .unwrap_or(false)
        {
            self.active_trigger_missing_close_requested = true;
        }
    }

    pub fn sync_detached_trigger_focus(
        &mut self,
        trigger_id: ElementId,
        focused: bool,
    ) -> TooltipFocusChange {
        if focused {
            self.sync_focused_trigger(Some(trigger_id))
        } else if self.focused_trigger_id.as_ref() == Some(&trigger_id) {
            self.sync_focused_trigger(None)
        } else {
            TooltipFocusChange::None
        }
    }

    pub fn sync_focused_trigger(
        &mut self,
        focused_trigger_id: Option<ElementId>,
    ) -> TooltipFocusChange {
        let previous = self.focused_trigger_id.clone();
        self.focused_trigger_id = focused_trigger_id.clone();
        if self.pointer_focused_trigger_id.is_some()
            && self.pointer_focused_trigger_id != focused_trigger_id
        {
            self.pointer_focused_trigger_id = None;
        }
        if focused_trigger_id.is_none() {
            self.press_suppressed_trigger_id = None;
            if self.last_open_reason == TooltipOpenChangeReason::TriggerPress {
                self.last_open_reason = TooltipOpenChangeReason::None;
            }
        }
        match (previous, focused_trigger_id) {
            (None, Some(id))
                if !self.focus_open_suppressed(&id)
                    && !self.disabled
                    && self.can_open_trigger(&id) =>
            {
                TooltipFocusChange::Open(id)
            }
            (Some(previous), None)
                if self.open_value() && self.active_trigger_id.as_ref() == Some(&previous) =>
            {
                TooltipFocusChange::Close
            }
            (Some(previous), Some(next))
                if previous != next
                    && !self.focus_open_suppressed(&next)
                    && !self.disabled
                    && self.can_open_trigger(&next) =>
            {
                TooltipFocusChange::Open(next)
            }
            _ => TooltipFocusChange::None,
        }
    }

    fn focus_open_suppressed(&self, trigger_id: &ElementId) -> bool {
        self.pointer_focused_trigger_id.as_ref() == Some(trigger_id)
            || self.press_suppressed_trigger_id.as_ref() == Some(trigger_id)
            || (self.last_open_reason == TooltipOpenChangeReason::TriggerPress
                && self.active_trigger_id.as_ref() == Some(trigger_id))
    }

    pub fn take_active_trigger_missing_close_request(&mut self) -> bool {
        let requested = self.active_trigger_missing_close_requested;
        self.active_trigger_missing_close_requested = false;
        requested
    }

    pub fn register_detached_trigger(&mut self, trigger: TooltipTriggerMetadata<P>) {
        let trigger = trigger.with_seen_generation(self.detached_trigger_generation);
        if let Some(existing) = self
            .triggers
            .iter_mut()
            .find(|candidate| candidate.id() == trigger.id())
        {
            let bounds = existing.bounds();
            *existing = trigger.with_bounds(bounds);
            return;
        }

        self.triggers.push(trigger);
    }

    pub fn set_bounds(&mut self, kind: TooltipBoundsKind, bounds: Bounds<Pixels>) -> bool {
        match kind {
            TooltipBoundsKind::Trigger(trigger_id) => self.set_trigger_bounds(&trigger_id, bounds),
            TooltipBoundsKind::Popup => self.set_popup_bounds(bounds),
        }
    }

    pub fn set_trigger_bounds(&mut self, trigger_id: &ElementId, bounds: Bounds<Pixels>) -> bool {
        let Some(trigger) = self
            .triggers
            .iter_mut()
            .rev()
            .find(|trigger| trigger.id() == trigger_id)
        else {
            return false;
        };
        if trigger.bounds == Some(bounds) {
            return false;
        }

        trigger.bounds = Some(bounds);
        true
    }

    pub fn set_popup_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.popup_bounds == Some(bounds) {
            return false;
        }
        self.previous_popup_size = self.popup_bounds.map(|bounds| bounds.size);
        self.popup_bounds = Some(bounds);
        true
    }

    pub fn set_effective_placement(&mut self, side: TooltipSide, align: TooltipAlign) -> bool {
        if self.effective_side == Some(side) && self.effective_align == Some(align) {
            return false;
        }
        self.effective_side = Some(side);
        self.effective_align = Some(align);
        true
    }

    pub fn effective_side(&self) -> Option<TooltipSide> {
        self.effective_side
    }

    pub fn effective_align(&self) -> Option<TooltipAlign> {
        self.effective_align
    }

    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }
        self.available_size = Some(size);
        true
    }

    pub fn set_cursor_position(&mut self, position: Point<Pixels>) -> bool {
        if self.cursor_position == Some(position) {
            return false;
        }
        self.cursor_position = Some(position);
        true
    }

    pub fn set_trigger_hovered(&mut self, trigger_id: Option<ElementId>) {
        if trigger_id.is_none() || self.press_suppressed_trigger_id != trigger_id {
            self.press_suppressed_trigger_id = None;
        }
        self.hovered_trigger_id = trigger_id;
    }

    pub fn suppress_trigger_press(&mut self, trigger_id: ElementId) {
        self.press_suppressed_trigger_id = Some(trigger_id);
    }

    pub fn is_trigger_press_suppressed(&self, trigger_id: &ElementId) -> bool {
        self.press_suppressed_trigger_id.as_ref() == Some(trigger_id)
    }

    pub fn sync_trigger_press(
        &mut self,
        trigger_id: ElementId,
        close_on_click: bool,
        detached_trigger: bool,
    ) -> TooltipTriggerPressChange {
        // GPUI focuses any focusable element on mouse down and nothing blurs
        // it when empty space is later clicked; pointer-sourced focus must
        // not open the tooltip or hold it open after the pointer leaves.
        self.pointer_focused_trigger_id = Some(trigger_id.clone());
        if close_on_click || detached_trigger {
            self.cancel_hover();
        }

        let close_active = close_on_click
            && self.open_value()
            && self.active_trigger_id.as_ref() == Some(&trigger_id);
        if close_active {
            self.suppress_trigger_press(trigger_id);
        }

        TooltipTriggerPressChange::new(close_active, !close_active && detached_trigger)
    }

    pub fn sync_disabled_trigger_hover(&mut self, trigger_id: &ElementId) -> bool {
        self.cancel_hover();
        if self.open_value() && self.active_trigger_id.as_ref() != Some(trigger_id) {
            self.hovered_trigger_id = None;
            return true;
        }
        false
    }

    pub fn set_popup_hovered(&mut self, hovered: bool) {
        self.popup_hovered = hovered;
    }

    pub fn hoverable_popup_enabled(&self) -> bool {
        !self.disable_hoverable_popup
    }

    pub fn should_keep_open_for_popup_hover(&self) -> bool {
        self.hoverable_popup_enabled() && self.popup_hovered
    }

    pub fn should_keep_open_for_trigger_unhover(&self, trigger_id: &ElementId) -> bool {
        self.open_value()
            && self.active_trigger_id.as_ref() == Some(trigger_id)
            && ((self.focused_trigger_id.as_ref() == Some(trigger_id)
                && self.pointer_focused_trigger_id.as_ref() != Some(trigger_id))
                || self.last_open_source == TooltipOpenChangeSource::Focus)
    }

    pub fn close_delay_for_trigger_unhover(
        &self,
        trigger_id: &ElementId,
        pointer: Point<Pixels>,
        configured_close_delay: Duration,
    ) -> Duration {
        if !configured_close_delay.is_zero()
            || !self.open_value()
            || self.active_trigger_id.as_ref() != Some(trigger_id)
            || !self.hoverable_popup_enabled()
        {
            return configured_close_delay;
        }

        let Some(trigger_bounds) = self.active_trigger_bounds() else {
            return configured_close_delay;
        };
        let Some(popup_bounds) = self.popup_bounds else {
            return configured_close_delay;
        };

        if point_in_safe_gap(
            trigger_bounds,
            popup_bounds,
            pointer,
            TOOLTIP_SAFE_GAP_PADDING,
        ) {
            TOOLTIP_SAFE_GAP_CLOSE_DELAY
        } else {
            configured_close_delay
        }
    }

    pub fn should_close_for_provider_handoff(&self, root_id: &ElementId) -> bool {
        self.open_value()
            && self
                .provider_active_root_id
                .as_ref()
                .map(|active_root_id| active_root_id != root_id)
                .unwrap_or(false)
            && self.last_open_source != TooltipOpenChangeSource::Focus
    }

    pub fn schedule_hover(
        &mut self,
        target: TooltipHoverTarget,
        trigger_id: Option<ElementId>,
    ) -> u64 {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = Some(TooltipPendingHover {
            generation: self.hover_generation,
            target,
            trigger_id,
        });
        self.hover_generation
    }

    pub fn cancel_hover(&mut self) {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = None;
    }

    pub fn take_scheduled_hover(
        &mut self,
        generation: u64,
        target: TooltipHoverTarget,
        trigger_id: Option<&ElementId>,
    ) -> bool {
        let matches = self
            .pending_hover
            .as_ref()
            .map(|pending| {
                pending.generation == generation
                    && pending.target == target
                    && pending.trigger_id.as_ref() == trigger_id
            })
            .unwrap_or(false);
        if matches {
            self.pending_hover = None;
        }
        matches
    }

    pub fn request_open(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> TooltipOpenChangeOutcome<P> {
        self.request_open_change(current, true, trigger_id)
    }

    pub fn request_close(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> TooltipOpenChangeOutcome<P> {
        self.request_open_change(current, false, trigger_id)
    }

    pub fn hover_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> TooltipOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn unhover_trigger(&self, current: bool) -> TooltipOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn focus_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> TooltipOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn blur_trigger(&self, current: bool) -> TooltipOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn press_trigger(&self, current: bool) -> TooltipOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn dismiss_outside(&self, current: bool) -> TooltipOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn close_from_escape(&self, current: bool) -> TooltipOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn activate_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> TooltipOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn request_open_change(
        &self,
        current: bool,
        next: bool,
        trigger_id: Option<ElementId>,
    ) -> TooltipOpenChangeOutcome<P> {
        if self.disabled && next {
            return TooltipOpenChangeOutcome::new(false, current, None, None, None);
        }

        let explicit_trigger = trigger_id.is_some();
        let resolved_trigger_id = trigger_id
            .or_else(|| self.active_trigger_id.clone())
            .or_else(|| self.first_enabled_trigger_id());
        let trigger = resolved_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id));

        if next
            && ((explicit_trigger && trigger.is_none())
                || trigger
                    .map(TooltipTriggerMetadata::disabled)
                    .unwrap_or(false))
        {
            return TooltipOpenChangeOutcome::new(false, current, None, None, None);
        }

        let mounted_only_close = !next && !current && self.prevent_unmount_on_close;
        let trigger_changed_while_open = next && self.active_trigger_id != resolved_trigger_id;
        if current == next && !mounted_only_close && !trigger_changed_while_open {
            return TooltipOpenChangeOutcome::new(
                false,
                current,
                resolved_trigger_id,
                trigger.map(|trigger| trigger.source_id().clone()),
                trigger.and_then(|trigger| trigger.payload.clone()),
            );
        }

        TooltipOpenChangeOutcome::new(
            true,
            next,
            resolved_trigger_id,
            trigger.map(|trigger| trigger.source_id().clone()),
            trigger.and_then(|trigger| trigger.payload.clone()),
        )
    }

    pub fn commit_open(
        &mut self,
        open: bool,
        trigger_id: Option<ElementId>,
        source: TooltipOpenChangeSource,
        prevent_unmount_on_close: bool,
        commit_open_state: bool,
        commit_trigger_state: bool,
    ) {
        if commit_trigger_state && self.active_trigger_id != trigger_id {
            self.previous_trigger_id = self.active_trigger_id.clone();
            self.active_trigger_id = trigger_id;
        }

        if commit_open_state {
            self.open = open && !self.disabled;
        }
        if source != TooltipOpenChangeSource::None {
            self.last_open_source = source;
        }

        if open {
            self.prevent_unmount_on_close = false;
        } else {
            self.prevent_unmount_on_close = prevent_unmount_on_close;
        }
    }

    pub fn record_open_change(
        &mut self,
        reason: TooltipOpenChangeReason,
        source: TooltipOpenChangeSource,
    ) {
        if source != TooltipOpenChangeSource::None {
            self.last_open_source = source;
        }
        if reason != TooltipOpenChangeReason::None {
            self.last_open_reason = reason;
        }
        self.instant = match reason {
            TooltipOpenChangeReason::None => TooltipInstant::Instant,
            _ => TooltipInstant::Delay,
        };
    }

    pub fn can_open_trigger(&self, trigger_id: &ElementId) -> bool {
        !self.disabled
            && self
                .trigger_by_id(trigger_id)
                .map(|trigger| !trigger.disabled())
                .unwrap_or(false)
    }

    pub fn active_trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.active_trigger()
            .and_then(TooltipTriggerMetadata::focus_handle)
    }

    pub fn active_close_delay(&self) -> Duration {
        self.active_trigger()
            .map(TooltipTriggerMetadata::close_delay)
            .unwrap_or(Duration::ZERO)
    }

    pub fn root_state(&self, props: &TooltipProps<P>) -> TooltipRootStyleState<P> {
        TooltipRootStyleState::new(
            self.open_value(),
            self.mounted_value(false),
            self.disabled || props.disabled(),
            self.last_open_source,
            self.active_source_trigger_id(),
            self.active_payload(),
            !self.triggers.is_empty(),
            match (self.instant, self.provider_instant) {
                (TooltipInstant::Instant, _) | (_, TooltipInstant::Instant) => {
                    TooltipInstant::Instant
                }
                _ => TooltipInstant::Delay,
            },
        )
    }

    pub fn trigger_state(
        &self,
        trigger_id: &ElementId,
        disabled: bool,
        payload_present: bool,
    ) -> TooltipTriggerStyleState<P> {
        let active = self.active_trigger_id.as_ref() == Some(trigger_id);
        let trigger_payload = self
            .trigger_by_id(trigger_id)
            .and_then(|trigger| trigger.payload.clone());
        TooltipTriggerStyleState::new(
            disabled || self.disabled,
            self.open_value(),
            active,
            self.focused_trigger_id.as_ref() == Some(trigger_id),
            self.hovered_trigger_id.as_ref() == Some(trigger_id),
            trigger_id.clone(),
            payload_present || trigger_payload.is_some(),
            trigger_payload,
        )
    }

    pub fn portal_state(&self, keep_mounted: bool) -> TooltipPortalStyleState {
        TooltipPortalStyleState::new(self.open_value(), self.mounted_value(keep_mounted))
    }

    pub fn positioner_state(
        &self,
        side: TooltipSide,
        align: TooltipAlign,
        keep_mounted: bool,
    ) -> TooltipPositionerStyleState {
        TooltipPositionerStyleState::new(
            self.open_value(),
            self.mounted_value(keep_mounted),
            side,
            align,
            self.active_anchor_bounds(),
            self.popup_bounds,
            self.available_size,
            self.instant,
        )
    }

    pub fn popup_state(
        &self,
        side: TooltipSide,
        align: TooltipAlign,
        keep_mounted: bool,
    ) -> TooltipPopupStyleState {
        TooltipPopupStyleState::new(
            self.open_value(),
            self.mounted_value(keep_mounted),
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            self.instant,
        )
    }

    pub fn viewport_state(&self) -> TooltipViewportStyleState {
        TooltipViewportStyleState::new(
            self.activation_direction(),
            false,
            self.instant,
            self.previous_trigger_id.clone(),
            self.active_trigger_id.clone(),
            self.previous_popup_size,
            self.popup_bounds.map(|bounds| bounds.size),
        )
    }

    pub fn popup_bounds(&self) -> Option<Bounds<Pixels>> {
        self.popup_bounds
    }

    pub fn pending_hover(&self) -> Option<(u64, TooltipHoverTarget, Option<ElementId>)> {
        self.pending_hover.as_ref().map(|pending| {
            (
                pending.generation,
                pending.target,
                pending.trigger_id.clone(),
            )
        })
    }

    pub fn active_trigger_contains(&self, point: Point<Pixels>) -> bool {
        self.active_trigger_bounds()
            .map(|bounds| bounds.contains(&point))
            .unwrap_or(false)
    }

    fn active_trigger(&self) -> Option<&TooltipTriggerMetadata<P>> {
        self.active_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id))
    }

    fn active_trigger_bounds(&self) -> Option<Bounds<Pixels>> {
        self.active_trigger()
            .and_then(TooltipTriggerMetadata::bounds)
    }

    fn active_anchor_bounds(&self) -> Option<Bounds<Pixels>> {
        let cursor = self.cursor_position;
        if self.track_cursor_axis == TooltipTrackCursorAxis::Both {
            return cursor.map(|cursor| Bounds::new(cursor, size(px(0.0), px(0.0))));
        }

        let bounds = self.active_trigger_bounds()?;
        match (self.track_cursor_axis, cursor) {
            (TooltipTrackCursorAxis::None, _) | (_, None) => Some(bounds),
            (TooltipTrackCursorAxis::X, Some(cursor)) => Some(Bounds::new(
                gpui::point(cursor.x, bounds.top()),
                size(px(0.0), bounds.size.height),
            )),
            (TooltipTrackCursorAxis::Y, Some(cursor)) => Some(Bounds::new(
                gpui::point(bounds.left(), cursor.y),
                size(bounds.size.width, px(0.0)),
            )),
            (TooltipTrackCursorAxis::Both, Some(_)) => unreachable!(),
        }
    }

    fn trigger_by_id(&self, trigger_id: &ElementId) -> Option<&TooltipTriggerMetadata<P>> {
        self.triggers
            .iter()
            .rev()
            .find(|trigger| trigger.id() == trigger_id)
    }

    fn first_enabled_trigger_id(&self) -> Option<ElementId> {
        self.triggers
            .iter()
            .find(|trigger| !trigger.disabled())
            .map(|trigger| trigger.id().clone())
    }

    fn activation_direction(&self) -> TooltipActivationDirection {
        let Some(previous_id) = self.previous_trigger_id.as_ref() else {
            return TooltipActivationDirection::None;
        };
        let Some(active_id) = self.active_trigger_id.as_ref() else {
            return TooltipActivationDirection::None;
        };
        if previous_id == active_id {
            return TooltipActivationDirection::None;
        }

        let previous_trigger = self.trigger_by_id(previous_id);
        let active_trigger = self.trigger_by_id(active_id);
        if let (Some(previous_bounds), Some(active_bounds)) = (
            previous_trigger.and_then(TooltipTriggerMetadata::bounds),
            active_trigger.and_then(TooltipTriggerMetadata::bounds),
        ) {
            return activation_direction_from_bounds(previous_bounds, active_bounds);
        }

        let previous_order = previous_trigger.map(TooltipTriggerMetadata::order);
        let active_order = active_trigger.map(TooltipTriggerMetadata::order);
        match (previous_order, active_order) {
            (Some(previous), Some(active)) if active > previous => {
                TooltipActivationDirection::Forward
            }
            (Some(previous), Some(active)) if active < previous => {
                TooltipActivationDirection::Backward
            }
            _ => TooltipActivationDirection::None,
        }
    }
}

fn activation_direction_from_bounds(
    previous_bounds: Bounds<Pixels>,
    active_bounds: Bounds<Pixels>,
) -> TooltipActivationDirection {
    let horizontal = axis_direction(
        active_bounds.center().x - previous_bounds.center().x,
        TooltipActivationDirection::Right,
        TooltipActivationDirection::Left,
    );
    let vertical = axis_direction(
        active_bounds.center().y - previous_bounds.center().y,
        TooltipActivationDirection::Down,
        TooltipActivationDirection::Up,
    );

    match (horizontal, vertical) {
        (TooltipActivationDirection::Left, TooltipActivationDirection::Up) => {
            TooltipActivationDirection::LeftUp
        }
        (TooltipActivationDirection::Left, TooltipActivationDirection::Down) => {
            TooltipActivationDirection::LeftDown
        }
        (TooltipActivationDirection::Right, TooltipActivationDirection::Up) => {
            TooltipActivationDirection::RightUp
        }
        (TooltipActivationDirection::Right, TooltipActivationDirection::Down) => {
            TooltipActivationDirection::RightDown
        }
        (TooltipActivationDirection::None, direction) => direction,
        (direction, TooltipActivationDirection::None) => direction,
        _ => TooltipActivationDirection::None,
    }
}

fn point_in_safe_gap(
    trigger_bounds: Bounds<Pixels>,
    popup_bounds: Bounds<Pixels>,
    point: Point<Pixels>,
    padding: Pixels,
) -> bool {
    let left = trigger_bounds.left().min(popup_bounds.left()) - padding;
    let right = trigger_bounds.right().max(popup_bounds.right()) + padding;
    let top = trigger_bounds.top().min(popup_bounds.top()) - padding;
    let bottom = trigger_bounds.bottom().max(popup_bounds.bottom()) + padding;

    let between_horizontal = point.x >= left && point.x <= right;
    let between_vertical = point.y >= top && point.y <= bottom;
    if popup_bounds.bottom() <= trigger_bounds.top() {
        return between_horizontal
            && point.y >= popup_bounds.bottom() - padding
            && point.y <= trigger_bounds.top() + padding;
    }
    if popup_bounds.top() >= trigger_bounds.bottom() {
        return between_horizontal
            && point.y >= trigger_bounds.bottom() - padding
            && point.y <= popup_bounds.top() + padding;
    }
    if popup_bounds.right() <= trigger_bounds.left() {
        return between_vertical
            && point.x >= popup_bounds.right() - padding
            && point.x <= trigger_bounds.left() + padding;
    }
    if popup_bounds.left() >= trigger_bounds.right() {
        return between_vertical
            && point.x >= trigger_bounds.right() - padding
            && point.x <= popup_bounds.left() + padding;
    }

    false
}

fn axis_direction(
    offset: Pixels,
    positive: TooltipActivationDirection,
    negative: TooltipActivationDirection,
) -> TooltipActivationDirection {
    let tolerance = px(5.0);
    if offset > tolerance {
        positive
    } else if offset < -tolerance {
        negative
    } else {
        TooltipActivationDirection::None
    }
}

pub fn scoped_trigger_id(root_id: &ElementId, trigger_id: &ElementId) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(trigger_id.to_string())))
}
