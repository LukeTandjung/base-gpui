use std::time::Duration;

use gpui::{px, Bounds, ElementId, FocusHandle, Pixels, Point, SharedString, Size};

use crate::preview_card::{
    PreviewCardAlign, PreviewCardArrowStyleState, PreviewCardBackdropStyleState,
    PreviewCardInstant, PreviewCardPopupStyleState, PreviewCardPortalStyleState,
    PreviewCardPositionerStyleState, PreviewCardSide, PreviewCardTriggerStyleState,
    PreviewCardViewportStyleState,
};
use crate::primitives::safe_polygon::{
    SafePolygon, SafePolygonConfig, SafePolygonSide, SafePolygonVerdict,
};

/// Base UI Preview Card open-change reasons; no `Disabled` variant exists
/// because the Preview Card root and trigger expose no disabled API.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PreviewCardOpenChangeReason {
    TriggerHover,
    TriggerFocus,
    TriggerPress,
    OutsidePress,
    EscapeKey,
    ImperativeAction,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PreviewCardOpenChangeSource {
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
pub enum PreviewCardActivationDirection {
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
pub struct PreviewCardOpenChangeDetails<P: Clone + 'static> {
    reason: PreviewCardOpenChangeReason,
    source: PreviewCardOpenChangeSource,
    trigger_id: Option<ElementId>,
    payload: Option<P>,
    cancelable: bool,
    canceled: bool,
    prevent_unmount_on_close: bool,
}

impl<P: Clone + 'static> PreviewCardOpenChangeDetails<P> {
    pub fn new(
        reason: PreviewCardOpenChangeReason,
        source: PreviewCardOpenChangeSource,
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

    pub fn reason(&self) -> PreviewCardOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> PreviewCardOpenChangeSource {
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
pub struct PreviewCardTriggerMetadata<P: Clone + 'static> {
    id: ElementId,
    source_id: ElementId,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
    delay: Duration,
    close_delay: Duration,
    payload: Option<P>,
    order: usize,
    detached: bool,
    seen_generation: u64,
}

impl<P: Clone + 'static> PreviewCardTriggerMetadata<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ElementId,
        source_id: ElementId,
        focus_handle: FocusHandle,
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
            delay,
            close_delay,
            payload,
            order,
            detached,
            seen_generation: 0,
        }
    }

    pub fn new_without_focus(
        id: ElementId,
        source_id: ElementId,
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
            delay,
            close_delay,
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

    pub fn with_seen_generation(mut self, seen_generation: u64) -> Self {
        self.seen_generation = seen_generation;
        self
    }

    pub fn with_bounds(mut self, bounds: Option<Bounds<Pixels>>) -> Self {
        self.bounds = bounds;
        self
    }
}

pub struct PreviewCardOpenChangeOutcome<P: Clone + 'static> {
    changed: bool,
    open: bool,
    trigger_id: Option<ElementId>,
    source_trigger_id: Option<ElementId>,
    payload: Option<P>,
}

impl<P: Clone + 'static> PreviewCardOpenChangeOutcome<P> {
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
pub enum PreviewCardHoverTarget {
    Open,
    Close,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewCardBoundsKind {
    Trigger(ElementId),
    Popup,
    Arrow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PreviewCardPendingHover {
    generation: u64,
    target: PreviewCardHoverTarget,
    trigger_id: Option<ElementId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewCardFocusChange {
    Open(ElementId),
    Close,
    None,
}

/// Deep runtime for the Preview Card family: open state, trigger metadata and
/// payloads, hover timers with generation checks, safe-polygon hover intent,
/// measured bounds, instant facts, and viewport transition facts. It never
/// calls user callbacks; commands return outcomes for the context to act on.
pub struct PreviewCardRuntime<P: Clone + 'static> {
    open: bool,
    last_open_source: PreviewCardOpenChangeSource,
    last_open_reason: PreviewCardOpenChangeReason,
    instant: PreviewCardInstant,
    prevent_unmount_on_close: bool,
    active_trigger_id: Option<ElementId>,
    previous_trigger_id: Option<ElementId>,
    triggers: Vec<PreviewCardTriggerMetadata<P>>,
    detached_trigger_generation: u64,
    focused_trigger_id: Option<ElementId>,
    hovered_trigger_id: Option<ElementId>,
    popup_hovered: bool,
    press_suppressed_trigger_id: Option<ElementId>,
    active_trigger_missing_close_requested: bool,
    popup_bounds: Option<Bounds<Pixels>>,
    previous_popup_size: Option<Size<Pixels>>,
    arrow_bounds: Option<Bounds<Pixels>>,
    arrow_padding: Pixels,
    available_size: Option<Size<Pixels>>,
    effective_side: Option<PreviewCardSide>,
    effective_align: Option<PreviewCardAlign>,
    viewport_present: bool,
    hover_generation: u64,
    pending_hover: Option<PreviewCardPendingHover>,
    safe_polygon: SafePolygon,
}

impl<P: Clone + 'static> PreviewCardRuntime<P> {
    pub fn new(open: bool, active_trigger_id: Option<ElementId>) -> Self {
        Self {
            open,
            last_open_source: PreviewCardOpenChangeSource::None,
            last_open_reason: PreviewCardOpenChangeReason::None,
            instant: PreviewCardInstant::None,
            prevent_unmount_on_close: false,
            active_trigger_id,
            previous_trigger_id: None,
            triggers: Vec::new(),
            detached_trigger_generation: 0,
            focused_trigger_id: None,
            hovered_trigger_id: None,
            popup_hovered: false,
            press_suppressed_trigger_id: None,
            active_trigger_missing_close_requested: false,
            popup_bounds: None,
            previous_popup_size: None,
            arrow_bounds: None,
            arrow_padding: px(5.0),
            available_size: None,
            effective_side: None,
            effective_align: None,
            viewport_present: false,
            hover_generation: 0,
            pending_hover: None,
            safe_polygon: SafePolygon::new(SafePolygonConfig::default()),
        }
    }

    pub fn open_value(&self) -> bool {
        self.open
    }

    pub fn mounted_value(&self, keep_mounted: bool) -> bool {
        self.open || keep_mounted || self.prevent_unmount_on_close
    }

    pub fn active_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger_id.clone()
    }

    pub fn active_source_trigger_id(&self) -> Option<ElementId> {
        self.active_trigger()
            .map(|trigger| trigger.source_id().clone())
            .or_else(|| self.active_trigger_id.clone())
    }

    /// The payload of the active trigger; `None` when the card has no active
    /// trigger so a stale payload can never leak.
    pub fn active_payload(&self) -> Option<P> {
        self.active_trigger()
            .and_then(|trigger| trigger.payload.clone())
    }

    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open;
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

    pub fn begin_detached_trigger_collection(&mut self) {
        self.detached_trigger_generation = self.detached_trigger_generation.wrapping_add(1);
    }

    /// Replaces the attached trigger set (preserving measured bounds and any
    /// detached registrations) and flags a close request when the active
    /// trigger unmounted.
    pub fn sync_children(&mut self, mut triggers: Vec<PreviewCardTriggerMetadata<P>>) {
        let previous_triggers = std::mem::take(&mut self.triggers);
        for trigger in &mut triggers {
            if let Some(bounds) = previous_triggers
                .iter()
                .rev()
                .find(|candidate| candidate.id() == trigger.id())
                .and_then(PreviewCardTriggerMetadata::bounds)
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

        if self.open && self.active_trigger_id.is_none() {
            self.active_trigger_id = self.first_trigger_id();
        }

        if self.open
            && self
                .active_trigger_id
                .as_ref()
                .map(|id| self.trigger_by_id(id).is_none())
                .unwrap_or(false)
        {
            self.active_trigger_missing_close_requested = true;
        }

        if self.open
            && self
                .active_trigger_id
                .as_ref()
                .and_then(|id| self.trigger_by_id(id))
                .map(|trigger| {
                    trigger.detached
                        && trigger.seen_generation.saturating_add(1)
                            < self.detached_trigger_generation
                })
                .unwrap_or(false)
        {
            self.active_trigger_missing_close_requested = true;
        }
    }

    pub fn take_active_trigger_missing_close_request(&mut self) -> bool {
        let requested = self.active_trigger_missing_close_requested;
        self.active_trigger_missing_close_requested = false;
        requested
    }

    pub fn register_detached_trigger(&mut self, trigger: PreviewCardTriggerMetadata<P>) {
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

    pub fn sync_detached_trigger_focus(
        &mut self,
        trigger_id: ElementId,
        focused: bool,
    ) -> PreviewCardFocusChange {
        if focused {
            self.sync_focused_trigger(Some(trigger_id))
        } else if self.focused_trigger_id.as_ref() == Some(&trigger_id) {
            self.sync_focused_trigger(None)
        } else {
            PreviewCardFocusChange::None
        }
    }

    pub fn sync_focused_trigger(
        &mut self,
        focused_trigger_id: Option<ElementId>,
    ) -> PreviewCardFocusChange {
        let previous = self.focused_trigger_id.clone();
        self.focused_trigger_id = focused_trigger_id.clone();
        if focused_trigger_id.is_none() {
            self.press_suppressed_trigger_id = None;
            if self.last_open_reason == PreviewCardOpenChangeReason::TriggerPress {
                self.last_open_reason = PreviewCardOpenChangeReason::None;
            }
        }
        match (previous, focused_trigger_id) {
            (None, Some(id)) if !self.focus_open_suppressed(&id) && self.can_open_trigger(&id) => {
                PreviewCardFocusChange::Open(id)
            }
            (Some(previous), None)
                if self.open && self.active_trigger_id.as_ref() == Some(&previous) =>
            {
                PreviewCardFocusChange::Close
            }
            (Some(previous), Some(next))
                if previous != next
                    && !self.focus_open_suppressed(&next)
                    && self.can_open_trigger(&next) =>
            {
                PreviewCardFocusChange::Open(next)
            }
            _ => PreviewCardFocusChange::None,
        }
    }

    fn focus_open_suppressed(&self, trigger_id: &ElementId) -> bool {
        self.press_suppressed_trigger_id.as_ref() == Some(trigger_id)
            || (self.last_open_reason == PreviewCardOpenChangeReason::TriggerPress
                && self.active_trigger_id.as_ref() == Some(trigger_id))
    }

    pub fn set_bounds(&mut self, kind: PreviewCardBoundsKind, bounds: Bounds<Pixels>) -> bool {
        match kind {
            PreviewCardBoundsKind::Trigger(trigger_id) => {
                self.set_trigger_bounds(&trigger_id, bounds)
            }
            PreviewCardBoundsKind::Popup => self.set_popup_bounds(bounds),
            PreviewCardBoundsKind::Arrow => self.set_arrow_bounds(bounds),
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

    pub fn set_arrow_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.arrow_bounds == Some(bounds) {
            return false;
        }
        self.arrow_bounds = Some(bounds);
        true
    }

    pub fn set_arrow_padding(&mut self, arrow_padding: Pixels) -> bool {
        if self.arrow_padding == arrow_padding {
            return false;
        }
        self.arrow_padding = arrow_padding;
        true
    }

    pub fn set_effective_placement(
        &mut self,
        side: PreviewCardSide,
        align: PreviewCardAlign,
    ) -> bool {
        if self.effective_side == Some(side) && self.effective_align == Some(align) {
            return false;
        }
        self.effective_side = Some(side);
        self.effective_align = Some(align);
        true
    }

    pub fn effective_side(&self) -> Option<PreviewCardSide> {
        self.effective_side
    }

    pub fn effective_align(&self) -> Option<PreviewCardAlign> {
        self.effective_align
    }

    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }
        self.available_size = Some(size);
        true
    }

    pub fn set_viewport_present(&mut self, present: bool) -> bool {
        if self.viewport_present == present {
            return false;
        }
        self.viewport_present = present;
        true
    }

    pub fn viewport_present(&self) -> bool {
        self.viewport_present
    }

    /// Clears the hovered fact for one trigger only; a later-arriving hover
    /// of a sibling trigger is not clobbered by this trigger's unhover.
    pub fn clear_trigger_hovered(&mut self, trigger_id: &ElementId) {
        if self.hovered_trigger_id.as_ref() == Some(trigger_id) {
            self.set_trigger_hovered(None);
        }
    }

    /// Whether a different trigger is currently hovered (a trigger-to-trigger
    /// move keeps the card open and switches instead of closing).
    pub fn another_trigger_hovered(&self, trigger_id: &ElementId) -> bool {
        self.hovered_trigger_id
            .as_ref()
            .map(|hovered| hovered != trigger_id)
            .unwrap_or(false)
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

    /// Pressing an open trigger dismisses the card (Base UI reason
    /// `trigger-press`); pressing never opens it. Returns whether the active
    /// open card should close.
    pub fn sync_trigger_press(&mut self, trigger_id: ElementId) -> bool {
        self.cancel_hover();
        let close_active = self.open && self.active_trigger_id.as_ref() == Some(&trigger_id);
        if close_active {
            self.suppress_trigger_press(trigger_id);
        }
        close_active
    }

    pub fn set_popup_hovered(&mut self, hovered: bool) {
        self.popup_hovered = hovered;
    }

    /// The Preview Card popup is always hoverable: hovering it keeps the card
    /// open (Base UI has no `disableHoverablePopup` for Preview Card).
    pub fn should_keep_open_for_popup_hover(&self) -> bool {
        self.popup_hovered
    }

    pub fn should_keep_open_for_trigger_unhover(&self, trigger_id: &ElementId) -> bool {
        self.open
            && self.active_trigger_id.as_ref() == Some(trigger_id)
            && (self.focused_trigger_id.as_ref() == Some(trigger_id)
                || self.last_open_source == PreviewCardOpenChangeSource::Focus)
    }

    pub fn schedule_hover(
        &mut self,
        target: PreviewCardHoverTarget,
        trigger_id: Option<ElementId>,
    ) -> u64 {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = Some(PreviewCardPendingHover {
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
        target: PreviewCardHoverTarget,
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
            Some(PreviewCardSide::Top) => SafePolygonSide::Top,
            Some(PreviewCardSide::Left | PreviewCardSide::InlineStart) => SafePolygonSide::Left,
            Some(PreviewCardSide::Right | PreviewCardSide::InlineEnd) => SafePolygonSide::Right,
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

    pub fn request_open(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> PreviewCardOpenChangeOutcome<P> {
        self.request_open_change(current, true, trigger_id)
    }

    pub fn request_close(
        &self,
        current: bool,
        trigger_id: Option<ElementId>,
    ) -> PreviewCardOpenChangeOutcome<P> {
        self.request_open_change(current, false, trigger_id)
    }

    pub fn hover_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> PreviewCardOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn unhover_trigger(&self, current: bool) -> PreviewCardOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn focus_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> PreviewCardOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    pub fn blur_trigger(&self, current: bool) -> PreviewCardOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn press_trigger(&self, current: bool) -> PreviewCardOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn dismiss_outside(&self, current: bool) -> PreviewCardOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn close_from_escape(&self, current: bool) -> PreviewCardOpenChangeOutcome<P> {
        self.request_close(current, None)
    }

    pub fn activate_trigger(
        &self,
        current: bool,
        trigger_id: ElementId,
    ) -> PreviewCardOpenChangeOutcome<P> {
        self.request_open(current, Some(trigger_id))
    }

    /// Resolves an open/close request against the trigger set. Opening with
    /// an explicitly named but unregistered trigger id does not change state
    /// (the handle's recoverable `false`, unlike Base UI's throw).
    pub fn request_open_change(
        &self,
        current: bool,
        next: bool,
        trigger_id: Option<ElementId>,
    ) -> PreviewCardOpenChangeOutcome<P> {
        let explicit_trigger = trigger_id.is_some();
        let resolved_trigger_id = trigger_id
            .or_else(|| self.active_trigger_id.clone())
            .or_else(|| self.first_trigger_id());
        let trigger = resolved_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id));

        if next && explicit_trigger && trigger.is_none() {
            return PreviewCardOpenChangeOutcome::new(false, current, None, None, None);
        }

        let mounted_only_close = !next && !current && self.prevent_unmount_on_close;
        let trigger_changed_while_open = next && self.active_trigger_id != resolved_trigger_id;
        if current == next && !mounted_only_close && !trigger_changed_while_open {
            return PreviewCardOpenChangeOutcome::new(
                false,
                current,
                resolved_trigger_id,
                trigger.map(|trigger| trigger.source_id().clone()),
                trigger.and_then(|trigger| trigger.payload.clone()),
            );
        }

        PreviewCardOpenChangeOutcome::new(
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
        source: PreviewCardOpenChangeSource,
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
        if source != PreviewCardOpenChangeSource::None {
            self.last_open_source = source;
        }

        if open {
            self.prevent_unmount_on_close = false;
        } else {
            self.prevent_unmount_on_close = prevent_unmount_on_close;
            self.disarm_safe_polygon();
        }
    }

    /// Clears the keep-mounted-after-close flag (the handle's `unmount()`
    /// imperative action).
    pub fn clear_prevent_unmount(&mut self) -> bool {
        let changed = self.prevent_unmount_on_close;
        self.prevent_unmount_on_close = false;
        changed
    }

    /// Records reason/source facts and classifies the instant kind: focus
    /// opens instantly as `Focus`, press/escape dismissals are `Dismiss`, and
    /// hover-driven changes clear the instant state.
    pub fn record_open_change(
        &mut self,
        reason: PreviewCardOpenChangeReason,
        source: PreviewCardOpenChangeSource,
    ) {
        if source != PreviewCardOpenChangeSource::None {
            self.last_open_source = source;
        }
        if reason != PreviewCardOpenChangeReason::None {
            self.last_open_reason = reason;
        }
        self.instant = match reason {
            PreviewCardOpenChangeReason::TriggerFocus => PreviewCardInstant::Focus,
            PreviewCardOpenChangeReason::TriggerPress | PreviewCardOpenChangeReason::EscapeKey => {
                PreviewCardInstant::Dismiss
            }
            _ => PreviewCardInstant::None,
        };
    }

    pub fn can_open_trigger(&self, trigger_id: &ElementId) -> bool {
        self.trigger_by_id(trigger_id).is_some()
    }

    pub fn active_trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.active_trigger()
            .and_then(PreviewCardTriggerMetadata::focus_handle)
    }

    pub fn active_close_delay(&self) -> Duration {
        self.active_trigger()
            .map(PreviewCardTriggerMetadata::close_delay)
            .unwrap_or(Duration::ZERO)
    }

    pub fn instant(&self) -> PreviewCardInstant {
        self.instant
    }

    pub fn trigger_state(
        &self,
        trigger_id: &ElementId,
        payload_present: bool,
    ) -> PreviewCardTriggerStyleState<P> {
        let active = self.active_trigger_id.as_ref() == Some(trigger_id);
        let trigger_payload = self
            .trigger_by_id(trigger_id)
            .and_then(|trigger| trigger.payload.clone());
        PreviewCardTriggerStyleState::new(
            self.open && active,
            active,
            self.focused_trigger_id.as_ref() == Some(trigger_id),
            self.hovered_trigger_id.as_ref() == Some(trigger_id),
            trigger_id.clone(),
            payload_present || trigger_payload.is_some(),
            trigger_payload,
        )
    }

    pub fn portal_state(&self, keep_mounted: bool) -> PreviewCardPortalStyleState {
        PreviewCardPortalStyleState::new(self.open, self.mounted_value(keep_mounted))
    }

    pub fn backdrop_state(&self, keep_mounted: bool) -> PreviewCardBackdropStyleState {
        PreviewCardBackdropStyleState::new(self.open, self.mounted_value(keep_mounted))
    }

    pub fn positioner_state(
        &self,
        side: PreviewCardSide,
        align: PreviewCardAlign,
        keep_mounted: bool,
    ) -> PreviewCardPositionerStyleState {
        PreviewCardPositionerStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            side,
            align,
            self.active_trigger_bounds(),
            self.popup_bounds,
            self.available_size,
            self.instant,
        )
    }

    pub fn popup_state(
        &self,
        side: PreviewCardSide,
        align: PreviewCardAlign,
        keep_mounted: bool,
    ) -> PreviewCardPopupStyleState {
        PreviewCardPopupStyleState::new(
            self.open,
            self.mounted_value(keep_mounted),
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            self.instant,
        )
    }

    pub fn arrow_state(
        &self,
        side: PreviewCardSide,
        align: PreviewCardAlign,
    ) -> PreviewCardArrowStyleState {
        let side = self.effective_side.unwrap_or(side);
        let align = self.effective_align.unwrap_or(align);
        let padding = self.arrow_padding;
        let (offset_x, offset_y, uncentered) = self.arrow_offsets(side, align, padding);

        PreviewCardArrowStyleState::new(
            self.open, side, align, offset_x, offset_y, padding, uncentered,
        )
    }

    pub fn viewport_state(&self) -> PreviewCardViewportStyleState {
        PreviewCardViewportStyleState::new(
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

    pub fn pending_hover(&self) -> Option<(u64, PreviewCardHoverTarget, Option<ElementId>)> {
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

    fn active_trigger(&self) -> Option<&PreviewCardTriggerMetadata<P>> {
        self.active_trigger_id
            .as_ref()
            .and_then(|id| self.trigger_by_id(id))
    }

    fn active_trigger_bounds(&self) -> Option<Bounds<Pixels>> {
        self.active_trigger()
            .and_then(PreviewCardTriggerMetadata::bounds)
    }

    fn trigger_by_id(&self, trigger_id: &ElementId) -> Option<&PreviewCardTriggerMetadata<P>> {
        self.triggers
            .iter()
            .rev()
            .find(|trigger| trigger.id() == trigger_id)
    }

    fn first_trigger_id(&self) -> Option<ElementId> {
        self.triggers.first().map(|trigger| trigger.id().clone())
    }

    fn arrow_offsets(
        &self,
        side: PreviewCardSide,
        align: PreviewCardAlign,
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
            PreviewCardSide::Top => {
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
            PreviewCardSide::Bottom => {
                let (center, uncentered) = self.clamped_arrow_center_x(
                    align,
                    padding,
                    popup_bounds,
                    popup_width,
                    arrow_half_width,
                );
                (Some(center - arrow_half_width), Some(px(0.0)), uncentered)
            }
            PreviewCardSide::Left | PreviewCardSide::InlineStart => {
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
            PreviewCardSide::Right | PreviewCardSide::InlineEnd => {
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
        align: PreviewCardAlign,
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
        align: PreviewCardAlign,
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

    fn activation_direction(&self) -> PreviewCardActivationDirection {
        let Some(previous_id) = self.previous_trigger_id.as_ref() else {
            return PreviewCardActivationDirection::None;
        };
        let Some(active_id) = self.active_trigger_id.as_ref() else {
            return PreviewCardActivationDirection::None;
        };
        if previous_id == active_id {
            return PreviewCardActivationDirection::None;
        }

        let previous_trigger = self.trigger_by_id(previous_id);
        let active_trigger = self.trigger_by_id(active_id);
        if let (Some(previous_bounds), Some(active_bounds)) = (
            previous_trigger.and_then(PreviewCardTriggerMetadata::bounds),
            active_trigger.and_then(PreviewCardTriggerMetadata::bounds),
        ) {
            return activation_direction_from_bounds(previous_bounds, active_bounds);
        }

        let previous_order = previous_trigger.map(PreviewCardTriggerMetadata::order);
        let active_order = active_trigger.map(PreviewCardTriggerMetadata::order);
        match (previous_order, active_order) {
            (Some(previous), Some(active)) if active > previous => {
                PreviewCardActivationDirection::Forward
            }
            (Some(previous), Some(active)) if active < previous => {
                PreviewCardActivationDirection::Backward
            }
            _ => PreviewCardActivationDirection::None,
        }
    }
}

fn activation_direction_from_bounds(
    previous_bounds: Bounds<Pixels>,
    active_bounds: Bounds<Pixels>,
) -> PreviewCardActivationDirection {
    let horizontal = axis_direction(
        active_bounds.center().x - previous_bounds.center().x,
        PreviewCardActivationDirection::Right,
        PreviewCardActivationDirection::Left,
    );
    let vertical = axis_direction(
        active_bounds.center().y - previous_bounds.center().y,
        PreviewCardActivationDirection::Down,
        PreviewCardActivationDirection::Up,
    );

    match (horizontal, vertical) {
        (PreviewCardActivationDirection::Left, PreviewCardActivationDirection::Up) => {
            PreviewCardActivationDirection::LeftUp
        }
        (PreviewCardActivationDirection::Left, PreviewCardActivationDirection::Down) => {
            PreviewCardActivationDirection::LeftDown
        }
        (PreviewCardActivationDirection::Right, PreviewCardActivationDirection::Up) => {
            PreviewCardActivationDirection::RightUp
        }
        (PreviewCardActivationDirection::Right, PreviewCardActivationDirection::Down) => {
            PreviewCardActivationDirection::RightDown
        }
        (PreviewCardActivationDirection::None, direction) => direction,
        (direction, PreviewCardActivationDirection::None) => direction,
        _ => PreviewCardActivationDirection::None,
    }
}

fn axis_direction(
    offset: Pixels,
    positive: PreviewCardActivationDirection,
    negative: PreviewCardActivationDirection,
) -> PreviewCardActivationDirection {
    let tolerance = px(5.0);
    if offset > tolerance {
        positive
    } else if offset < -tolerance {
        negative
    } else {
        PreviewCardActivationDirection::None
    }
}

fn fallback_arrow_center(
    align: PreviewCardAlign,
    padding: Pixels,
    axis_length: Pixels,
    arrow_half_length: Pixels,
) -> Pixels {
    match align {
        PreviewCardAlign::Start => padding + arrow_half_length,
        PreviewCardAlign::Center => axis_length * 0.5,
        PreviewCardAlign::End => axis_length - padding - arrow_half_length,
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

/// Scopes a public trigger id to its root/handle so independent preview
/// cards can reuse public trigger ids safely; a duplicate public id inside
/// one root deterministically resolves to the last registered trigger.
pub fn scoped_trigger_id(root_id: &ElementId, trigger_id: &ElementId) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(trigger_id.to_string())))
}
