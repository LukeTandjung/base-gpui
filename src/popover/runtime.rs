use std::time::Duration;

use gpui::{px, Bounds, ElementId, FocusHandle, Pixels, Point, SharedString, Size};

use crate::primitives::safe_polygon::{
    SafePolygon, SafePolygonConfig, SafePolygonSide, SafePolygonVerdict,
};

use crate::popover::{
    PopoverAlign, PopoverArrowStyleState, PopoverBackdropStyleState, PopoverCloseStyleState,
    PopoverDescriptionStyleState, PopoverPopupStyleState, PopoverPortalStyleState,
    PopoverPositionerStyleState, PopoverProps, PopoverRootStyleState, PopoverSide,
    PopoverTitleStyleState, PopoverTriggerStyleState, PopoverViewportStyleState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverOpenChangeReason {
    TriggerHover,
    TriggerFocus,
    TriggerPress,
    OutsidePress,
    EscapeKey,
    ClosePress,
    FocusOut,
    ImperativeAction,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverOpenChangeSource {
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
pub enum PopoverActivationDirection {
    Forward,
    Backward,
    #[default]
    None,
}

#[derive(Clone)]
pub struct PopoverOpenChangeDetails<P: Clone + 'static> {
    reason: PopoverOpenChangeReason,
    source: PopoverOpenChangeSource,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
    cancelable: bool,
    canceled: bool,
    prevent_unmount_on_close: bool,
}

impl<P: Clone + 'static> PopoverOpenChangeDetails<P> {
    pub fn new(
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
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

    pub fn reason(&self) -> PopoverOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> PopoverOpenChangeSource {
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
pub struct PopoverTriggerMetadata<P: Clone + 'static> {
    id: ElementId,
    source_id: ElementId,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
    disabled: bool,
    open_on_hover: bool,
    delay: Duration,
    close_delay: Duration,
    payload: Option<P>,
    order: usize,
    detached: bool,
}

impl<P: Clone + 'static> PopoverTriggerMetadata<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
        disabled: bool,
        open_on_hover: bool,
        delay: Duration,
        close_delay: Duration,
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
            open_on_hover,
            delay,
            close_delay,
            payload,
            order,
            detached,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_without_focus(
        id: ElementId,
        source_id: ElementId,
        disabled: bool,
        open_on_hover: bool,
        delay: Duration,
        close_delay: Duration,
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
            open_on_hover,
            delay,
            close_delay,
            payload,
            order,
            detached,
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

    pub fn open_on_hover(&self) -> bool {
        self.open_on_hover
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn close_delay(&self) -> Duration {
        self.close_delay
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

    pub fn with_bounds(mut self, bounds: Option<Bounds<Pixels>>) -> Self {
        self.bounds = bounds;
        self
    }
}

pub struct PopoverOpenChangeOutcome<P: Clone + 'static> {
    changed: bool,
    open: bool,
    trigger_id: Option<ElementId>,
    source_trigger_id: Option<ElementId>,
    payload: Option<P>,
}

impl<P: Clone + 'static> PopoverOpenChangeOutcome<P> {
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
pub enum PopoverHoverTarget {
    Open,
    Close,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PopoverBoundsKind {
    Trigger(ElementId),
    Popup,
    Arrow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PopoverPendingHover {
    generation: u64,
    target: PopoverHoverTarget,
    trigger_id: Option<ElementId>,
}

#[derive(Clone)]
pub struct PopoverRuntime<P: Clone + 'static> {
    open: bool,
    last_open_source: PopoverOpenChangeSource,
    last_open_reason: PopoverOpenChangeReason,
    prevent_unmount_on_close: bool,
    active_trigger_id: Option<ElementId>,
    previous_trigger_id: Option<ElementId>,
    triggers: Vec<PopoverTriggerMetadata<P>>,
    title_ids: Vec<ElementId>,
    description_ids: Vec<ElementId>,
    popup_focus_handles: Vec<FocusHandle>,
    focus_popup_on_open: bool,
    popup_bounds: Option<Bounds<Pixels>>,
    arrow_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    effective_side: Option<PopoverSide>,
    effective_align: Option<PopoverAlign>,
    modal: bool,
    was_focused: bool,
    focus_out_close_requested: bool,
    hover_generation: u64,
    pending_hover: Option<PopoverPendingHover>,
    popup_hovered: bool,
    safe_polygon: SafePolygon,
}

impl<P: Clone + 'static> PopoverRuntime<P> {
    /// Creates runtime state with the observed initial open and active-trigger values.
    pub fn new(open: bool, active_trigger_id: Option<ElementId>, modal: bool) -> Self {
        Self {
            open,
            last_open_source: PopoverOpenChangeSource::None,
            last_open_reason: PopoverOpenChangeReason::None,
            prevent_unmount_on_close: false,
            active_trigger_id,
            previous_trigger_id: None,
            triggers: Vec::new(),
            title_ids: Vec::new(),
            description_ids: Vec::new(),
            popup_focus_handles: Vec::new(),
            focus_popup_on_open: false,
            popup_bounds: None,
            arrow_bounds: None,
            available_size: None,
            effective_side: None,
            effective_align: None,
            modal,
            was_focused: false,
            focus_out_close_requested: false,
            hover_generation: 0,
            pending_hover: None,
            popup_hovered: false,
            safe_polygon: SafePolygon::new(SafePolygonConfig::default()),
        }
    }

    /// Returns the currently observed open state.
    pub fn open_value(&self) -> bool {
        self.open
    }

    /// Returns whether the popup subtree should currently be mounted.
    pub fn mounted_value(&self, keep_mounted: bool) -> bool {
        self.open || keep_mounted || self.prevent_unmount_on_close
    }

    /// Returns the currently active trigger id in the root-scoped id space.
    pub fn active_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger_id.clone()
    }

    /// Returns the currently active trigger's public source id.
    pub fn active_source_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger()
            .map(|trigger| trigger.source_id().clone())
            .or_else(|| self.active_trigger_id.clone())
    }

    /// Returns the active trigger payload, when one is present.
    pub fn active_payload(&self) -> Option<P> {
        self.active_trigger()
            .and_then(|trigger| trigger.payload.clone())
    }

    /// Reconciles the runtime with an externally controlled open value.
    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open;
        if open {
            self.prevent_unmount_on_close = false;
        }
    }

    /// Reconciles the runtime with an externally controlled active trigger id.
    pub fn sync_trigger_id_from_context(&mut self, trigger_id: Option<ElementId>) {
        if self.active_trigger_id != trigger_id {
            self.previous_trigger_id = self.active_trigger_id.clone();
            self.active_trigger_id = trigger_id;
        }
    }

    /// Replaces the root-level modal flag for this render pass.
    pub fn sync_modal(&mut self, modal: bool) {
        self.modal = modal;
    }

    /// Replaces walked child trigger metadata while retaining detached handle triggers.
    pub fn sync_triggers(&mut self, mut triggers: Vec<PopoverTriggerMetadata<P>>) {
        let mut detached = self
            .triggers
            .iter()
            .filter(|trigger| trigger.detached())
            .cloned()
            .collect::<Vec<_>>();
        triggers.append(&mut detached);
        self.triggers = triggers;
    }

    /// Clears walked title/description metadata before registering current render labels.
    pub fn clear_label_metadata(&mut self) {
        self.title_ids.clear();
        self.description_ids.clear();
    }

    /// Registers walked title metadata for future accessibility integration.
    pub fn register_title(&mut self, id: ElementId) {
        self.title_ids.push(id);
        self.title_ids.sort_by_key(|id| id.to_string());
    }

    /// Registers walked description metadata for future accessibility integration.
    pub fn register_description(&mut self, id: ElementId) {
        self.description_ids.push(id);
        self.description_ids.sort_by_key(|id| id.to_string());
    }

    /// Replaces walked title metadata for future accessibility integration.
    pub fn sync_titles(&mut self, title_ids: Vec<ElementId>) {
        self.clear_label_metadata();
        for id in title_ids {
            self.register_title(id);
        }
    }

    /// Replaces walked description metadata for future accessibility integration.
    pub fn sync_descriptions(&mut self, mut description_ids: Vec<ElementId>) {
        self.description_ids.clear();
        for id in description_ids.drain(..) {
            self.register_description(id);
        }
    }

    pub fn sync_popup_focus_handles(&mut self, focus_handles: Vec<FocusHandle>) {
        self.popup_focus_handles = focus_handles;
    }

    pub fn request_popup_focus_on_open(
        &mut self,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
    ) {
        self.focus_popup_on_open = matches!(reason, PopoverOpenChangeReason::TriggerPress)
            && matches!(
                source,
                PopoverOpenChangeSource::Pointer | PopoverOpenChangeSource::Keyboard
            );
    }

    pub fn take_popup_focus_on_open(&mut self) -> Option<FocusHandle> {
        if !self.focus_popup_on_open || !self.open {
            return None;
        }

        self.focus_popup_on_open = false;
        self.popup_focus_handles.first().cloned()
    }

    /// Registers or refreshes a detached trigger driven through a PopoverHandle.
    pub fn register_detached_trigger(&mut self, trigger: PopoverTriggerMetadata<P>) {
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

    /// Records bounds measured during GPUI prepaint.
    pub fn set_bounds(&mut self, kind: PopoverBoundsKind, bounds: Bounds<Pixels>) -> bool {
        match kind {
            PopoverBoundsKind::Trigger(trigger_id) => self.set_trigger_bounds(&trigger_id, bounds),
            PopoverBoundsKind::Popup => self.set_popup_bounds(bounds),
            PopoverBoundsKind::Arrow => self.set_arrow_bounds(bounds),
        }
    }

    /// Records trigger bounds measured during GPUI prepaint.
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

    /// Records popup bounds measured during GPUI prepaint.
    pub fn set_popup_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.popup_bounds == Some(bounds) {
            return false;
        }

        self.popup_bounds = Some(bounds);
        true
    }

    /// Synchronizes focus containment for non-modal focus-out dismissal.
    pub fn sync_focus(&mut self, focused: bool) {
        if self.was_focused && !focused && self.open && !self.modal {
            self.focus_out_close_requested = true;
        }
        self.was_focused = focused;
    }

    pub fn take_focus_out_close_request(&mut self) -> bool {
        let requested = self.focus_out_close_requested;
        self.focus_out_close_requested = false;
        requested
    }

    /// Schedules a delayed hover-open or hover-close transition.
    pub fn schedule_hover(
        &mut self,
        target: PopoverHoverTarget,
        trigger_id: Option<ElementId>,
    ) -> u64 {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = Some(PopoverPendingHover {
            generation: self.hover_generation,
            target,
            trigger_id,
        });
        self.hover_generation
    }

    /// Clears any delayed hover transition and invalidates pending timer tasks.
    pub fn cancel_hover(&mut self) {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = None;
    }

    /// Consumes a pending delayed hover transition if it is still current.
    pub fn take_scheduled_hover(
        &mut self,
        generation: u64,
        target: PopoverHoverTarget,
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

    /// Records decorative arrow bounds measured during GPUI prepaint.
    pub fn set_arrow_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.arrow_bounds == Some(bounds) {
            return false;
        }

        self.arrow_bounds = Some(bounds);
        true
    }

    /// Records effective popup placement after collision handling.
    pub fn set_effective_placement(&mut self, side: PopoverSide, align: PopoverAlign) -> bool {
        if self.effective_side == Some(side) && self.effective_align == Some(align) {
            return false;
        }

        self.effective_side = Some(side);
        self.effective_align = Some(align);
        true
    }

    /// Records the current viewport size used for positioning diagnostics.
    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }

        self.available_size = Some(size);
        true
    }

    /// Requests an open-state transition for an optional trigger.
    pub fn request_open(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> PopoverOpenChangeOutcome<P> {
        self.request_open_change(current, true, trigger_id)
    }

    /// Requests a close-state transition, preserving trigger metadata when available.
    pub fn request_close(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> PopoverOpenChangeOutcome<P> {
        self.request_open_change(current, false, trigger_id)
    }

    pub fn toggle_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> PopoverOpenChangeOutcome<P> {
        let should_close = current && self.active_trigger_id.as_ref() == Some(&trigger_id);
        self.request_open_change(current, !should_close, Some(trigger_id))
    }

    pub fn activate_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> PopoverOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn dismiss_outside(&self, current: bool) -> PopoverOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn close_from_escape(&self, current: bool) -> PopoverOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    /// Requests an open-state transition for an optional trigger.
    pub fn request_open_change(
        &self,
        current: bool,
        next: bool,
        trigger_id: Option<ElementId>,
    ) -> PopoverOpenChangeOutcome<P> {
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
                    .map(PopoverTriggerMetadata::disabled)
                    .unwrap_or(false))
        {
            return PopoverOpenChangeOutcome::new(false, current, None, None, None);
        }

        let mounted_only_close = !next && !current && self.prevent_unmount_on_close;
        let trigger_changed_while_open = next && self.active_trigger_id != resolved_trigger_id;
        if current == next && !mounted_only_close && !trigger_changed_while_open {
            return PopoverOpenChangeOutcome::new(
                false,
                current,
                resolved_trigger_id,
                trigger.map(|trigger| trigger.source_id().clone()),
                trigger.and_then(|trigger| trigger.payload.clone()),
            );
        }

        PopoverOpenChangeOutcome::new(
            true,
            next,
            resolved_trigger_id,
            trigger.map(|trigger| trigger.source_id().clone()),
            trigger.and_then(|trigger| trigger.payload.clone()),
        )
    }

    /// Commits an accepted open-state transition and active-trigger transition.
    pub fn commit_open(
        &mut self,
        open: bool,
        trigger_id: Option<ElementId>,
        source: PopoverOpenChangeSource,
        prevent_unmount_on_close: bool,
        commit_open_state: bool,
        commit_trigger_state: bool,
    ) {
        if commit_trigger_state && self.active_trigger_id != trigger_id {
            self.previous_trigger_id = self.active_trigger_id.clone();
            self.active_trigger_id = trigger_id;
        }

        if commit_open_state {
            self.open = open;
        }
        if source != PopoverOpenChangeSource::None {
            self.last_open_source = source;
        }

        if open {
            self.prevent_unmount_on_close = false;
        } else {
            self.prevent_unmount_on_close = prevent_unmount_on_close;
            self.focus_popup_on_open = false;
            self.popup_hovered = false;
            self.disarm_safe_polygon();
        }
    }

    // ------------------------------------------------------------------
    // Hover intent: hoverable popup + safe polygon
    // ------------------------------------------------------------------

    pub fn set_popup_hovered(&mut self, hovered: bool) {
        self.popup_hovered = hovered;
    }

    /// Hovering the popup keeps a hover-opened popover open.
    pub fn should_keep_open_for_popup_hover(&self) -> bool {
        self.popup_hovered
    }

    /// Whether the current open state was produced by trigger hover; only
    /// then does leaving the trigger/popup close the popover.
    pub fn opened_by_hover(&self) -> bool {
        self.open && self.last_open_reason == PopoverOpenChangeReason::TriggerHover
    }

    /// Arms the safe-polygon tracker with the pointer's exit point, the
    /// active trigger's bounds, the measured popup bounds, and the effective
    /// side. Returns whether the tracker armed (both bounds were available).
    pub fn arm_safe_polygon(&mut self, exit_point: Point<Pixels>) -> bool {
        let Some(trigger_bounds) = self.active_trigger_bounds() else {
            return false;
        };
        let Some(popup_bounds) = self.popup_bounds else {
            return false;
        };
        let side = match self.effective_side {
            Some(PopoverSide::Top) => SafePolygonSide::Top,
            Some(PopoverSide::Left | PopoverSide::InlineStart) => SafePolygonSide::Left,
            Some(PopoverSide::Right | PopoverSide::InlineEnd) => SafePolygonSide::Right,
            _ => SafePolygonSide::Bottom,
        };
        self.safe_polygon
            .arm(exit_point, trigger_bounds, popup_bounds, side);
        true
    }

    /// Evaluates the pointer against the armed safe polygon; `None` when the
    /// tracker is not armed.
    pub fn evaluate_safe_polygon(
        &mut self,
        pointer: Point<Pixels>,
        now: Duration,
    ) -> Option<SafePolygonVerdict> {
        if !self.safe_polygon.is_armed() {
            return None;
        }
        Some(self.safe_polygon.evaluate(pointer, now))
    }

    pub fn disarm_safe_polygon(&mut self) {
        self.safe_polygon.disarm();
    }

    pub fn safe_polygon_armed(&self) -> bool {
        self.safe_polygon.is_armed()
    }

    pub fn active_close_delay(&self) -> Duration {
        self.active_trigger()
            .map(PopoverTriggerMetadata::close_delay)
            .unwrap_or(Duration::ZERO)
    }

    /// Records the reason/source of an accepted transition without changing controlled open state.
    pub fn record_open_change(
        &mut self,
        reason: PopoverOpenChangeReason,
        source: PopoverOpenChangeSource,
    ) {
        if source != PopoverOpenChangeSource::None {
            self.last_open_source = source;
        }
        if reason != PopoverOpenChangeReason::None {
            self.last_open_reason = reason;
        }
    }

    /// Returns whether a trigger id exists and is enabled.
    pub fn can_open_trigger(&self, trigger_id: &ElementId) -> bool {
        self.trigger_by_id(trigger_id)
            .map(|trigger| !trigger.disabled())
            .unwrap_or(false)
    }

    /// Returns the focus handle for the active trigger.
    pub fn active_trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.active_trigger()
            .and_then(PopoverTriggerMetadata::focus_handle)
    }

    pub fn root_state(&self, props: &PopoverProps<P>) -> PopoverRootStyleState<P> {
        PopoverRootStyleState::new(
            self.open,
            self.mounted_value(false),
            self.modal || props.modal(),
            self.last_open_source,
            self.active_source_trigger_id(),
            self.active_payload(),
            !self.triggers.is_empty(),
        )
    }

    pub fn trigger_state(
        &self,
        trigger_id: &ElementId,
        disabled: bool,
        payload_present: bool,
    ) -> PopoverTriggerStyleState<P> {
        let active = self.active_trigger_id.as_ref() == Some(trigger_id);
        let trigger_payload = self
            .trigger_by_id(trigger_id)
            .and_then(|trigger| trigger.payload.clone());
        PopoverTriggerStyleState::new(
            disabled,
            self.open,
            active,
            false,
            false,
            payload_present || trigger_payload.is_some(),
            trigger_payload,
        )
    }

    pub fn portal_state(&self, keep_mounted: bool) -> PopoverPortalStyleState {
        PopoverPortalStyleState::new(self.open, self.mounted_value(keep_mounted))
    }

    pub fn backdrop_state(&self, keep_mounted: bool) -> PopoverBackdropStyleState {
        PopoverBackdropStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.last_open_source != PopoverOpenChangeSource::Pointer
                || self.last_open_reason != PopoverOpenChangeReason::TriggerHover,
        )
    }

    pub fn positioner_state(
        &self,
        side: PopoverSide,
        align: PopoverAlign,
        keep_mounted: bool,
    ) -> PopoverPositionerStyleState {
        PopoverPositionerStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            side,
            align,
            self.active_trigger_bounds(),
            self.popup_bounds,
            self.available_size,
            false,
        )
    }

    pub fn popup_state(
        &self,
        side: PopoverSide,
        align: PopoverAlign,
        keep_mounted: bool,
    ) -> PopoverPopupStyleState {
        PopoverPopupStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            false,
        )
    }

    pub fn arrow_state(&self, side: PopoverSide, align: PopoverAlign) -> PopoverArrowStyleState {
        let side = self.effective_side.unwrap_or(side);
        let align = self.effective_align.unwrap_or(align);
        let padding = px(4.0);
        let (offset_x, offset_y, uncentered) = self.arrow_offsets(side, align, padding);

        PopoverArrowStyleState::new(
            self.open, side, align, offset_x, offset_y, padding, uncentered,
        )
    }

    pub fn title_state(&self) -> PopoverTitleStyleState {
        PopoverTitleStyleState
    }

    pub fn description_state(&self) -> PopoverDescriptionStyleState {
        PopoverDescriptionStyleState
    }

    pub fn close_state(&self, disabled: bool) -> PopoverCloseStyleState {
        PopoverCloseStyleState::new(disabled, self.open)
    }

    pub fn viewport_state(&self) -> PopoverViewportStyleState {
        PopoverViewportStyleState::new(
            self.activation_direction(),
            false,
            false,
            self.popup_bounds.map(|bounds| bounds.size),
        )
    }

    pub fn popup_bounds(&self) -> Option<Bounds<Pixels>> {
        self.popup_bounds
    }

    pub fn arrow_bounds(&self) -> Option<Bounds<Pixels>> {
        self.arrow_bounds
    }

    pub fn pending_hover(&self) -> Option<(u64, PopoverHoverTarget, Option<ElementId>)> {
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

    pub fn title_ids(&self) -> &[ElementId] {
        &self.title_ids
    }

    pub fn description_ids(&self) -> &[ElementId] {
        &self.description_ids
    }

    fn arrow_offsets(
        &self,
        side: PopoverSide,
        align: PopoverAlign,
        padding: Pixels,
    ) -> (Option<Pixels>, Option<Pixels>, bool) {
        let Some(popup_bounds) = self.popup_bounds else {
            return (None, None, false);
        };
        let Some(arrow_bounds) = self.arrow_bounds else {
            return (None, None, false);
        };

        let popup_width = popup_bounds.size.width;
        let popup_height = popup_bounds.size.height;
        let arrow_width = arrow_bounds.size.width;
        let arrow_height = arrow_bounds.size.height;
        let arrow_half_width = arrow_width * 0.5;
        let arrow_half_height = arrow_height * 0.5;

        match side {
            PopoverSide::Top => {
                let (center, uncentered) = self.clamped_arrow_center_x(
                    align,
                    padding,
                    popup_bounds,
                    popup_width,
                    arrow_half_width,
                );
                (
                    Some(center - arrow_half_width),
                    Some(popup_height - arrow_height),
                    uncentered,
                )
            }
            PopoverSide::Bottom => {
                let (center, uncentered) = self.clamped_arrow_center_x(
                    align,
                    padding,
                    popup_bounds,
                    popup_width,
                    arrow_half_width,
                );
                (Some(center - arrow_half_width), Some(px(0.0)), uncentered)
            }
            PopoverSide::Left | PopoverSide::InlineStart => {
                let (center, uncentered) = self.clamped_arrow_center_y(
                    align,
                    padding,
                    popup_bounds,
                    popup_height,
                    arrow_half_height,
                );
                (
                    Some(popup_width - arrow_width),
                    Some(center - arrow_half_height),
                    uncentered,
                )
            }
            PopoverSide::Right | PopoverSide::InlineEnd => {
                let (center, uncentered) = self.clamped_arrow_center_y(
                    align,
                    padding,
                    popup_bounds,
                    popup_height,
                    arrow_half_height,
                );
                (Some(px(0.0)), Some(center - arrow_half_height), uncentered)
            }
        }
    }

    fn clamped_arrow_center_x(
        &self,
        align: PopoverAlign,
        padding: Pixels,
        popup_bounds: Bounds<Pixels>,
        popup_width: Pixels,
        arrow_half_width: Pixels,
    ) -> (Pixels, bool) {
        let target = self
            .active_trigger_bounds()
            .map(|bounds| bounds.center().x - popup_bounds.left())
            .unwrap_or_else(|| {
                fallback_arrow_center(align, padding, popup_width, arrow_half_width)
            });
        let clamped = clamp_arrow_center(target, padding, popup_width, arrow_half_width);
        (clamped, clamped != target)
    }

    fn clamped_arrow_center_y(
        &self,
        align: PopoverAlign,
        padding: Pixels,
        popup_bounds: Bounds<Pixels>,
        popup_height: Pixels,
        arrow_half_height: Pixels,
    ) -> (Pixels, bool) {
        let target = self
            .active_trigger_bounds()
            .map(|bounds| bounds.center().y - popup_bounds.top())
            .unwrap_or_else(|| {
                fallback_arrow_center(align, padding, popup_height, arrow_half_height)
            });
        let clamped = clamp_arrow_center(target, padding, popup_height, arrow_half_height);
        (clamped, clamped != target)
    }

    fn active_trigger(&self) -> Option<&PopoverTriggerMetadata<P>> {
        self.active_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id))
    }

    fn active_trigger_bounds(&self) -> Option<Bounds<Pixels>> {
        self.active_trigger()
            .and_then(PopoverTriggerMetadata::bounds)
    }

    fn trigger_by_id(&self, trigger_id: &ElementId) -> Option<&PopoverTriggerMetadata<P>> {
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

    fn activation_direction(&self) -> PopoverActivationDirection {
        let Some(previous_id) = self.previous_trigger_id.as_ref() else {
            return PopoverActivationDirection::None;
        };
        let Some(active_id) = self.active_trigger_id.as_ref() else {
            return PopoverActivationDirection::None;
        };
        if previous_id == active_id {
            return PopoverActivationDirection::None;
        }

        let previous_order = self
            .trigger_by_id(previous_id)
            .map(PopoverTriggerMetadata::order);
        let active_order = self
            .trigger_by_id(active_id)
            .map(PopoverTriggerMetadata::order);
        match (previous_order, active_order) {
            (Some(previous), Some(active)) if active > previous => {
                PopoverActivationDirection::Forward
            }
            (Some(previous), Some(active)) if active < previous => {
                PopoverActivationDirection::Backward
            }
            _ => PopoverActivationDirection::None,
        }
    }
}

fn fallback_arrow_center(
    align: PopoverAlign,
    padding: Pixels,
    axis_length: Pixels,
    arrow_half_length: Pixels,
) -> Pixels {
    match align {
        PopoverAlign::Start => padding + arrow_half_length,
        PopoverAlign::Center => axis_length * 0.5,
        PopoverAlign::End => axis_length - padding - arrow_half_length,
    }
}

fn clamp_arrow_center(
    target: Pixels,
    padding: Pixels,
    axis_length: Pixels,
    arrow_half_length: Pixels,
) -> Pixels {
    let min = padding + arrow_half_length;
    let max = axis_length - padding - arrow_half_length;
    if max < min {
        return min;
    }
    if target < min {
        min
    } else if target > max {
        max
    } else {
        target
    }
}

pub fn scoped_trigger_id(root_id: &ElementId, trigger_id: &ElementId) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(trigger_id.to_string())))
}
