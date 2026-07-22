use std::time::Duration;

use gpui::{px, Bounds, FocusHandle, Pixels, Point, Size};

use crate::navigation_menu::{
    NavigationMenuAlign, NavigationMenuArrowStyleState, NavigationMenuBackdropStyleState,
    NavigationMenuContentStyleState, NavigationMenuIconStyleState, NavigationMenuInstant,
    NavigationMenuItemStyleState, NavigationMenuLinkStyleState, NavigationMenuListStyleState,
    NavigationMenuOrientation, NavigationMenuPopupStyleState, NavigationMenuPortalStyleState,
    NavigationMenuPositionerStyleState, NavigationMenuRootStyleState, NavigationMenuSide,
    NavigationMenuTriggerStyleState, NavigationMenuViewportStyleState,
    NAVIGATION_MENU_PATIENT_CLICK_THRESHOLD,
};
use crate::primitives::safe_polygon::{
    SafePolygon, SafePolygonConfig, SafePolygonSide, SafePolygonVerdict,
};

/// Base UI Navigation Menu close/open reasons from `NavigationMenuRoot.tsx`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuValueChangeReason {
    TriggerPress,
    TriggerHover,
    OutsidePress,
    ListNavigation,
    FocusOut,
    EscapeKey,
    LinkPress,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuValueChangeSource {
    Pointer,
    Keyboard,
    Focus,
    Unknown,
    #[default]
    None,
}

/// Activation direction derived from previous vs next trigger bounds:
/// horizontal orientation compares x, vertical compares y.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NavigationMenuActivationDirection {
    Left,
    Right,
    Up,
    Down,
    #[default]
    None,
}

/// GPUI-native change details: no DOM event objects.
#[derive(Clone, Debug)]
pub struct NavigationMenuValueChangeDetails {
    reason: NavigationMenuValueChangeReason,
    source: NavigationMenuValueChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl NavigationMenuValueChangeDetails {
    pub fn new(
        reason: NavigationMenuValueChangeReason,
        source: NavigationMenuValueChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> NavigationMenuValueChangeReason {
        self.reason
    }

    pub fn source(&self) -> NavigationMenuValueChangeSource {
        self.source
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
}

/// One registered item (trigger + content) keyed by its required value.
#[derive(Clone)]
pub struct NavigationMenuItemMetadata<T: Clone + Eq + 'static> {
    value: T,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    bounds: Option<Bounds<Pixels>>,
    order: usize,
}

impl<T: Clone + Eq + 'static> NavigationMenuItemMetadata<T> {
    pub fn new(value: T, disabled: bool, focus_handle: Option<FocusHandle>, order: usize) -> Self {
        Self {
            value,
            disabled,
            focus_handle,
            bounds: None,
            order,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds
    }

    pub fn order(&self) -> usize {
        self.order
    }
}

/// One roving-focus stop in the list (a trigger or a top-level link), in
/// source order.
#[derive(Clone)]
pub struct NavigationMenuListEntry<T: Clone + Eq + 'static> {
    focus_handle: Option<FocusHandle>,
    trigger_value: Option<T>,
    disabled: bool,
}

impl<T: Clone + Eq + 'static> NavigationMenuListEntry<T> {
    pub fn new(
        focus_handle: Option<FocusHandle>,
        trigger_value: Option<T>,
        disabled: bool,
    ) -> Self {
        Self {
            focus_handle,
            trigger_value,
            disabled,
        }
    }

    pub fn focus_handle(&self) -> Option<FocusHandle> {
        self.focus_handle.clone()
    }

    pub fn trigger_value(&self) -> Option<&T> {
        self.trigger_value.as_ref()
    }
}

pub struct NavigationMenuValueChangeOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    value: Option<T>,
}

impl<T: Clone + Eq + 'static> NavigationMenuValueChangeOutcome<T> {
    fn new(changed: bool, value: Option<T>) -> Self {
        Self { changed, value }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<T> {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NavigationMenuHoverTarget<T: Clone + Eq + 'static> {
    Open(T),
    Close,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NavigationMenuBoundsKind<T: Clone + Eq + 'static> {
    Trigger(T),
    Popup,
    Viewport,
    Arrow,
    List,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavigationMenuMove {
    Next,
    Previous,
    First,
    Last,
}

struct PendingHover<T: Clone + Eq + 'static> {
    generation: u64,
    target: NavigationMenuHoverTarget<T>,
}

/// Deep runtime for the Navigation Menu family. Owns the current value
/// (uncontrolled), registered item/list metadata, active + previous trigger
/// identity and measured bounds, activation direction, measured
/// popup/viewport/arrow/list bounds, size morph facts, hover timer
/// generations, safe-polygon arming, patient-click state, and instant
/// classification. It never calls user callbacks; commands return outcomes
/// for the context to act on.
pub struct NavigationMenuRuntime<T: Clone + Eq + 'static> {
    value: Option<T>,
    previous_value: Option<T>,
    orientation: NavigationMenuOrientation,
    items: Vec<NavigationMenuItemMetadata<T>>,
    list_entries: Vec<NavigationMenuListEntry<T>>,
    highlighted_index: Option<usize>,
    activation_direction: NavigationMenuActivationDirection,
    last_change_reason: NavigationMenuValueChangeReason,
    instant: NavigationMenuInstant,
    last_anchor_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    previous_popup_size: Option<Size<Pixels>>,
    viewport_bounds: Option<Bounds<Pixels>>,
    arrow_bounds: Option<Bounds<Pixels>>,
    arrow_padding: Pixels,
    list_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    effective_side: Option<NavigationMenuSide>,
    effective_align: Option<NavigationMenuAlign>,
    hovered_trigger: Option<T>,
    popup_hovered: bool,
    hover_open_at: Option<Duration>,
    hover_generation: u64,
    pending_hover: Option<PendingHover<T>>,
    safe_polygon: SafePolygon,
}

impl<T: Clone + Eq + 'static> NavigationMenuRuntime<T> {
    pub fn new(value: Option<T>, orientation: NavigationMenuOrientation) -> Self {
        let instant = match value.is_some() {
            true => NavigationMenuInstant::Initial,
            false => NavigationMenuInstant::None,
        };
        Self {
            value,
            previous_value: None,
            orientation,
            items: Vec::new(),
            list_entries: Vec::new(),
            highlighted_index: None,
            activation_direction: NavigationMenuActivationDirection::None,
            last_change_reason: NavigationMenuValueChangeReason::None,
            instant,
            last_anchor_bounds: None,
            popup_bounds: None,
            previous_popup_size: None,
            viewport_bounds: None,
            arrow_bounds: None,
            arrow_padding: px(5.0),
            list_bounds: None,
            available_size: None,
            effective_side: None,
            effective_align: None,
            hovered_trigger: None,
            popup_hovered: false,
            hover_open_at: None,
            hover_generation: 0,
            pending_hover: None,
            safe_polygon: SafePolygon::new(SafePolygonConfig::default()),
        }
    }

    /// Open is derived: the popup is open exactly when the value is non-`None`.
    pub fn open_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn current_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn is_active_value(&self, value: &T) -> bool {
        self.value.as_ref() == Some(value)
    }

    pub fn orientation(&self) -> NavigationMenuOrientation {
        self.orientation
    }

    /// Controlled reconciliation: the controlled value is the source of truth
    /// and overwrites the uncontrolled value without transition bookkeeping
    /// for identical values.
    pub fn reconcile(&mut self, controlled: Option<Option<T>>) {
        let Some(controlled) = controlled else {
            return;
        };
        if self.value == controlled {
            return;
        }
        self.apply_value_transition(controlled);
    }

    /// Replaces the registered items and roving entries, preserving measured
    /// trigger bounds by value.
    pub fn sync_children(
        &mut self,
        mut items: Vec<NavigationMenuItemMetadata<T>>,
        list_entries: Vec<NavigationMenuListEntry<T>>,
    ) {
        let previous_items = std::mem::take(&mut self.items);
        for item in &mut items {
            if let Some(bounds) = previous_items
                .iter()
                .find(|candidate| candidate.value == item.value)
                .and_then(NavigationMenuItemMetadata::bounds)
            {
                item.bounds = Some(bounds);
            }
        }
        self.items = items;
        self.list_entries = list_entries;

        let valid_highlight = self
            .highlighted_index
            .map(|index| index < self.list_entries.len())
            .unwrap_or(false);
        if !valid_highlight {
            self.highlighted_index = match self.list_entries.is_empty() {
                true => None,
                false => Some(0),
            };
        }
    }

    /// Resolves a value-change request: deduplicated (`changed` is false when
    /// the next value equals the current one), so switching items never
    /// double-fires callbacks.
    pub fn request_value(
        &self,
        current: Option<T>,
        next: Option<T>,
    ) -> NavigationMenuValueChangeOutcome<T> {
        if current == next {
            return NavigationMenuValueChangeOutcome::new(false, next);
        }
        if let Some(next_value) = next.as_ref() {
            if self
                .item_by_value(next_value)
                .map(NavigationMenuItemMetadata::disabled)
                .unwrap_or(false)
            {
                return NavigationMenuValueChangeOutcome::new(false, current);
            }
        }
        NavigationMenuValueChangeOutcome::new(true, next)
    }

    /// Commits a value change: records previous value, derives activation
    /// direction on trigger switch (from previous vs next trigger bounds per
    /// orientation), preserves last-known popup size facts on close for the
    /// exit transition, and resets activation direction on close.
    pub fn commit_value(&mut self, next: Option<T>, commit_state: bool) {
        if !commit_state {
            self.apply_transition_facts(&next);
            return;
        }
        self.apply_value_transition(next);
    }

    fn apply_value_transition(&mut self, next: Option<T>) {
        self.apply_transition_facts(&next);
        self.previous_value = self.value.take();
        self.value = next;
    }

    fn apply_transition_facts(&mut self, next: &Option<T>) {
        match (self.value.as_ref(), next.as_ref()) {
            (Some(previous), Some(active)) if previous != active => {
                let previous_bounds = self.trigger_bounds(previous);
                let next_bounds = self.trigger_bounds(active);
                if let (Some(previous_bounds), Some(next_bounds)) = (previous_bounds, next_bounds) {
                    if let Some(direction) = activation_direction_from_bounds(
                        self.orientation,
                        previous_bounds,
                        next_bounds,
                    ) {
                        self.activation_direction = direction;
                    }
                }
                self.previous_popup_size = self.popup_bounds.map(|bounds| bounds.size);
                if let Some(bounds) = self.trigger_bounds(active) {
                    self.last_anchor_bounds = Some(bounds);
                }
            }
            (_, Some(active)) => {
                if let Some(bounds) = self.trigger_bounds(active) {
                    self.last_anchor_bounds = Some(bounds);
                }
            }
            (Some(_), None) => {
                // Closing resets activation direction; popup size facts are
                // preserved for the exit transition.
                self.activation_direction = NavigationMenuActivationDirection::None;
                self.hover_open_at = None;
                self.safe_polygon.disarm();
            }
            (None, None) => {}
        }
    }

    /// Records reason facts after a committed change. Any non-initial change
    /// clears the `Initial`/`Resize` instant classification.
    pub fn record_change(&mut self, reason: NavigationMenuValueChangeReason) {
        if reason != NavigationMenuValueChangeReason::None {
            self.last_change_reason = reason;
        }
        self.instant = NavigationMenuInstant::None;
    }

    /// Focus return on close is blocked for Base UI's blocked-return set.
    pub fn focus_return_blocked(&self) -> bool {
        matches!(
            self.last_change_reason,
            NavigationMenuValueChangeReason::TriggerHover
                | NavigationMenuValueChangeReason::OutsidePress
                | NavigationMenuValueChangeReason::FocusOut
        )
    }

    pub fn active_trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.previous_value
            .as_ref()
            .or(self.value.as_ref())
            .and_then(|value| self.item_by_value(value))
            .and_then(NavigationMenuItemMetadata::focus_handle)
    }

    /// Classifies a window resize as instant for the resize re-position.
    pub fn note_window_resize(&mut self) {
        self.instant = NavigationMenuInstant::Resize;
    }

    pub fn instant(&self) -> NavigationMenuInstant {
        self.instant
    }

    // --- measurement -----------------------------------------------------

    pub fn set_bounds(
        &mut self,
        kind: NavigationMenuBoundsKind<T>,
        bounds: Bounds<Pixels>,
    ) -> bool {
        match kind {
            NavigationMenuBoundsKind::Trigger(value) => {
                let Some(item) = self.items.iter_mut().find(|item| item.value == value) else {
                    return false;
                };
                if item.bounds == Some(bounds) {
                    return false;
                }
                item.bounds = Some(bounds);
                if self.value.as_ref() == Some(&value) {
                    self.last_anchor_bounds = Some(bounds);
                }
                true
            }
            NavigationMenuBoundsKind::Popup => {
                if self.popup_bounds == Some(bounds) {
                    return false;
                }
                self.previous_popup_size = self.popup_bounds.map(|bounds| bounds.size);
                self.popup_bounds = Some(bounds);
                true
            }
            NavigationMenuBoundsKind::Viewport => {
                if self.viewport_bounds == Some(bounds) {
                    return false;
                }
                self.viewport_bounds = Some(bounds);
                true
            }
            NavigationMenuBoundsKind::Arrow => {
                if self.arrow_bounds == Some(bounds) {
                    return false;
                }
                self.arrow_bounds = Some(bounds);
                true
            }
            NavigationMenuBoundsKind::List => {
                if self.list_bounds == Some(bounds) {
                    return false;
                }
                self.list_bounds = Some(bounds);
                true
            }
        }
    }

    /// Records the viewport size; a change while already measured and open
    /// classifies as a resize re-position (`instant`, no animation).
    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }
        let resized = self.available_size.is_some() && self.open_value();
        self.available_size = Some(size);
        if resized {
            self.instant = NavigationMenuInstant::Resize;
        }
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
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
    ) -> bool {
        if self.effective_side == Some(side) && self.effective_align == Some(align) {
            return false;
        }
        self.effective_side = Some(side);
        self.effective_align = Some(align);
        true
    }

    pub fn effective_side(&self) -> Option<NavigationMenuSide> {
        self.effective_side
    }

    pub fn effective_align(&self) -> Option<NavigationMenuAlign> {
        self.effective_align
    }

    /// The positioner's anchor: the active trigger's measured bounds, falling
    /// back to the last-known anchor bounds when the active trigger unmounted
    /// (prev-trigger fallback for the close transition).
    pub fn anchor_bounds(&self) -> Option<Bounds<Pixels>> {
        self.value
            .as_ref()
            .and_then(|value| self.trigger_bounds(value))
            .or(self.last_anchor_bounds)
    }

    pub fn popup_bounds(&self) -> Option<Bounds<Pixels>> {
        self.popup_bounds
    }

    /// Outside-press hit test: presses inside the popup or on any trigger of
    /// this menu tree never dismiss.
    pub fn outside_press_should_dismiss(&self, point: Point<Pixels>) -> bool {
        if self
            .popup_bounds
            .map(|bounds| bounds.contains(&point))
            .unwrap_or(false)
        {
            return false;
        }
        if self
            .items
            .iter()
            .filter_map(NavigationMenuItemMetadata::bounds)
            .any(|bounds| bounds.contains(&point))
        {
            return false;
        }
        true
    }

    // --- hover intent ----------------------------------------------------

    pub fn set_trigger_hovered(&mut self, value: Option<T>) {
        self.hovered_trigger = value;
    }

    pub fn another_trigger_hovered(&self, value: &T) -> bool {
        self.hovered_trigger
            .as_ref()
            .map(|hovered| hovered != value)
            .unwrap_or(false)
    }

    pub fn set_popup_hovered(&mut self, hovered: bool) {
        self.popup_hovered = hovered;
    }

    pub fn popup_hovered(&self) -> bool {
        self.popup_hovered
    }

    pub fn schedule_hover(&mut self, target: NavigationMenuHoverTarget<T>) -> u64 {
        self.hover_generation = self.hover_generation.wrapping_add(1);
        self.pending_hover = Some(PendingHover {
            generation: self.hover_generation,
            target,
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
        target: &NavigationMenuHoverTarget<T>,
    ) -> bool {
        let matches = self
            .pending_hover
            .as_ref()
            .map(|pending| pending.generation == generation && &pending.target == target)
            .unwrap_or(false);
        if matches {
            self.pending_hover = None;
        }
        matches
    }

    /// Records the moment a hover open committed (patient-click anchor).
    pub fn note_hover_open(&mut self, now: Duration) {
        self.hover_open_at = Some(now);
    }

    /// Base UI `stickIfOpen`: clicking the active trigger within 500ms of a
    /// hover open keeps the menu open; afterwards clicks toggle-close.
    pub fn patient_click_blocks_close(&self, now: Duration) -> bool {
        self.hover_open_at
            .map(|opened_at| {
                now.checked_sub(opened_at)
                    .map(|elapsed| elapsed < NAVIGATION_MENU_PATIENT_CLICK_THRESHOLD)
                    .unwrap_or(true)
            })
            .unwrap_or(false)
    }

    pub fn trigger_disabled(&self, value: &T) -> bool {
        self.item_by_value(value)
            .map(NavigationMenuItemMetadata::disabled)
            .unwrap_or(false)
    }

    // --- safe polygon ----------------------------------------------------

    /// Arms the safe-polygon tracker with the pointer's exit point, the
    /// active trigger's bounds, the measured popup bounds, and the effective
    /// side. Returns whether the tracker armed (both bounds were available).
    pub fn arm_safe_polygon(&mut self, exit_point: Point<Pixels>) -> bool {
        let Some(trigger_bounds) = self.anchor_bounds() else {
            return false;
        };
        let Some(popup_bounds) = self.popup_bounds else {
            return false;
        };
        let side = match self.effective_side {
            Some(NavigationMenuSide::Top) => SafePolygonSide::Top,
            Some(NavigationMenuSide::Left | NavigationMenuSide::InlineStart) => {
                SafePolygonSide::Left
            }
            Some(NavigationMenuSide::Right | NavigationMenuSide::InlineEnd) => {
                SafePolygonSide::Right
            }
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

    // --- roving list navigation -------------------------------------------

    /// Moves the roving highlight across list entries without wrapping
    /// (Base UI `loopFocus=false`): navigation clamps at the ends.
    pub fn move_highlight(&mut self, direction: NavigationMenuMove) -> Option<FocusHandle> {
        if self.list_entries.is_empty() {
            return None;
        }
        let last = self.list_entries.len() - 1;
        self.highlighted_index = match direction {
            NavigationMenuMove::First => Some(0),
            NavigationMenuMove::Last => Some(last),
            NavigationMenuMove::Next => match self.highlighted_index {
                None => Some(0),
                Some(index) => Some((index + 1).min(last)),
            },
            NavigationMenuMove::Previous => match self.highlighted_index {
                None => Some(0),
                Some(index) => Some(index.saturating_sub(1)),
            },
        };
        self.highlighted_focus_handle()
    }

    pub fn set_highlight(&mut self, index: usize) {
        if index < self.list_entries.len() {
            self.highlighted_index = Some(index);
        }
    }

    /// Focus handles of the roving entries in order, for the list layer to
    /// sync the highlight with actual GPUI focus.
    pub fn entry_focus_handles(&self) -> Vec<Option<FocusHandle>> {
        self.list_entries
            .iter()
            .map(NavigationMenuListEntry::focus_handle)
            .collect()
    }

    pub fn set_highlight_for_value(&mut self, value: &T) {
        if let Some(index) = self
            .list_entries
            .iter()
            .position(|entry| entry.trigger_value.as_ref() == Some(value))
        {
            self.highlighted_index = Some(index);
        }
    }

    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted_index
    }

    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        self.highlighted_index
            .and_then(|index| self.list_entries.get(index))
            .and_then(NavigationMenuListEntry::focus_handle)
    }

    /// The trigger value whose content the "open" arrow key should open: the
    /// highlighted entry's trigger value, when it is an enabled trigger.
    pub fn highlighted_trigger_value(&self) -> Option<T> {
        self.highlighted_index
            .and_then(|index| self.list_entries.get(index))
            .filter(|entry| !entry.disabled)
            .and_then(|entry| entry.trigger_value.clone())
    }

    pub fn is_tab_stop(&self, index: usize) -> bool {
        self.highlighted_index == Some(index)
    }

    // --- queries -----------------------------------------------------------

    pub fn root_state(&self, nested: bool) -> NavigationMenuRootStyleState {
        NavigationMenuRootStyleState::new(self.open_value(), nested)
    }

    pub fn list_state(&self) -> NavigationMenuListStyleState {
        NavigationMenuListStyleState::new(self.open_value())
    }

    /// Number of registered items; the item layer's `aria_size_of_set`.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn item_state(&self) -> NavigationMenuItemStyleState {
        NavigationMenuItemStyleState::new()
    }

    pub fn trigger_state(&self, value: &T, disabled: bool) -> NavigationMenuTriggerStyleState {
        NavigationMenuTriggerStyleState::new(self.is_active_value(value), disabled)
    }

    pub fn content_state(&self, value: &T, keep_mounted: bool) -> NavigationMenuContentStyleState {
        let open = self.is_active_value(value);
        NavigationMenuContentStyleState::new(open, open || keep_mounted, self.activation_direction)
    }

    pub fn portal_state(&self, keep_mounted: bool) -> NavigationMenuPortalStyleState {
        NavigationMenuPortalStyleState::new(self.open_value(), self.open_value() || keep_mounted)
    }

    pub fn positioner_state(
        &self,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
        keep_mounted: bool,
    ) -> NavigationMenuPositionerStyleState {
        NavigationMenuPositionerStyleState::new(
            self.open_value(),
            self.open_value() || keep_mounted,
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            self.anchor_bounds(),
            self.popup_bounds,
            self.available_size,
            self.instant,
        )
    }

    pub fn popup_state(
        &self,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
        keep_mounted: bool,
    ) -> NavigationMenuPopupStyleState {
        NavigationMenuPopupStyleState::new(
            self.open_value(),
            self.open_value() || keep_mounted,
            self.effective_side.unwrap_or(side),
            self.effective_align.unwrap_or(align),
            self.anchor_bounds().is_none(),
            self.popup_bounds.map(|bounds| bounds.size),
        )
    }

    pub fn viewport_state(&self) -> NavigationMenuViewportStyleState {
        NavigationMenuViewportStyleState::new(
            self.activation_direction,
            self.previous_popup_size.is_some()
                && self.previous_popup_size != self.popup_bounds.map(|bounds| bounds.size),
            self.previous_popup_size,
            self.viewport_bounds.map(|bounds| bounds.size),
        )
    }

    pub fn backdrop_state(&self, keep_mounted: bool) -> NavigationMenuBackdropStyleState {
        NavigationMenuBackdropStyleState::new(self.open_value(), self.open_value() || keep_mounted)
    }

    pub fn arrow_state(
        &self,
        side: NavigationMenuSide,
        align: NavigationMenuAlign,
    ) -> NavigationMenuArrowStyleState {
        let side = self.effective_side.unwrap_or(side);
        let align = self.effective_align.unwrap_or(align);
        let (offset_x, offset_y, uncentered) = self.arrow_offsets(side, align, self.arrow_padding);
        NavigationMenuArrowStyleState::new(
            self.open_value(),
            side,
            align,
            offset_x,
            offset_y,
            uncentered,
        )
    }

    pub fn link_state(&self, active: bool) -> NavigationMenuLinkStyleState {
        NavigationMenuLinkStyleState::new(active)
    }

    pub fn icon_state(&self, value: Option<&T>) -> NavigationMenuIconStyleState {
        let open = value
            .map(|value| self.is_active_value(value))
            .unwrap_or_else(|| self.open_value());
        NavigationMenuIconStyleState::new(open)
    }

    // --- internal -----------------------------------------------------------

    fn item_by_value(&self, value: &T) -> Option<&NavigationMenuItemMetadata<T>> {
        self.items.iter().find(|item| &item.value == value)
    }

    fn trigger_bounds(&self, value: &T) -> Option<Bounds<Pixels>> {
        self.item_by_value(value)
            .and_then(NavigationMenuItemMetadata::bounds)
    }

    fn arrow_offsets(
        &self,
        side: NavigationMenuSide,
        _align: NavigationMenuAlign,
        padding: Pixels,
    ) -> (Option<Pixels>, Option<Pixels>, bool) {
        let Some(popup_bounds) = self.popup_bounds else {
            return (None, None, false);
        };
        let Some(arrow_bounds) = self.arrow_bounds else {
            return (None, None, false);
        };
        let Some(anchor_bounds) = self.anchor_bounds() else {
            return (None, None, false);
        };

        let arrow_half_width = arrow_bounds.size.width * 0.5;
        let arrow_half_height = arrow_bounds.size.height * 0.5;
        match side {
            NavigationMenuSide::Top | NavigationMenuSide::Bottom => {
                let target = anchor_bounds.center().x - popup_bounds.left();
                let min = padding + arrow_half_width;
                let max = popup_bounds.size.width - padding - arrow_half_width;
                let clamped = clamp_pixels(target, min, max);
                let offset_y = match side {
                    NavigationMenuSide::Top => popup_bounds.size.height - arrow_bounds.size.height,
                    _ => px(0.0),
                };
                (
                    Some(clamped - arrow_half_width),
                    Some(offset_y),
                    clamped != target,
                )
            }
            _ => {
                let target = anchor_bounds.center().y - popup_bounds.top();
                let min = padding + arrow_half_height;
                let max = popup_bounds.size.height - padding - arrow_half_height;
                let clamped = clamp_pixels(target, min, max);
                let offset_x = match side {
                    NavigationMenuSide::Left | NavigationMenuSide::InlineStart => {
                        popup_bounds.size.width - arrow_bounds.size.width
                    }
                    _ => px(0.0),
                };
                (
                    Some(offset_x),
                    Some(clamped - arrow_half_height),
                    clamped != target,
                )
            }
        }
    }
}

fn clamp_pixels(target: Pixels, min: Pixels, max: Pixels) -> Pixels {
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

/// Direction of a trigger switch per orientation; equal positions (within
/// tolerance) answer `None` so the previous direction is left unchanged.
fn activation_direction_from_bounds(
    orientation: NavigationMenuOrientation,
    previous_bounds: Bounds<Pixels>,
    next_bounds: Bounds<Pixels>,
) -> Option<NavigationMenuActivationDirection> {
    let tolerance = px(1.0);
    match orientation {
        NavigationMenuOrientation::Horizontal => {
            let offset = next_bounds.center().x - previous_bounds.center().x;
            if offset > tolerance {
                Some(NavigationMenuActivationDirection::Right)
            } else if offset < -tolerance {
                Some(NavigationMenuActivationDirection::Left)
            } else {
                None
            }
        }
        NavigationMenuOrientation::Vertical => {
            let offset = next_bounds.center().y - previous_bounds.center().y;
            if offset > tolerance {
                Some(NavigationMenuActivationDirection::Down)
            } else if offset < -tolerance {
                Some(NavigationMenuActivationDirection::Up)
            } else {
                None
            }
        }
    }
}
