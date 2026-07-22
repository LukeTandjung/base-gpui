use std::time::{Duration, Instant};

use gpui::{Bounds, FocusHandle, Pixels, ScrollHandle, SharedString, Size};

use crate::select::{
    SelectAlign, SelectArrowStyleState, SelectBackdropStyleState, SelectGroupStyleState,
    SelectIconStyleState, SelectItemIndicatorStyleState, SelectItemStyleState,
    SelectItemTextStyleState, SelectListStyleState, SelectPopupStyleState, SelectPortalStyleState,
    SelectPositionerStyleState, SelectProps, SelectRootStyleState, SelectScrollArrowDirection,
    SelectScrollArrowStyleState, SelectSelectionMode, SelectSide, SelectTriggerStyleState,
    SelectValueComparator, SelectValueStyleState,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectValueChangeReason {
    #[default]
    None,
    ItemPress,
    Typeahead,
    Fallback,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectValueChangeSource {
    Pointer,
    Touch,
    Keyboard,
    Typeahead,
    Programmatic,
    #[default]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectValueChangeDetails {
    reason: SelectValueChangeReason,
    source: SelectValueChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl SelectValueChangeDetails {
    pub fn new(
        reason: SelectValueChangeReason,
        source: SelectValueChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> SelectValueChangeReason {
        self.reason
    }

    pub fn source(&self) -> SelectValueChangeSource {
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectOpenChangeReason {
    #[default]
    None,
    TriggerPress,
    ItemPress,
    EscapeKey,
    OutsidePress,
    FocusOut,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelectOpenChangeSource {
    Pointer,
    Touch,
    Keyboard,
    Programmatic,
    #[default]
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectOpenChangeDetails {
    reason: SelectOpenChangeReason,
    source: SelectOpenChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl SelectOpenChangeDetails {
    pub fn new(
        reason: SelectOpenChangeReason,
        source: SelectOpenChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> SelectOpenChangeReason {
        self.reason
    }

    pub fn source(&self) -> SelectOpenChangeSource {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SelectMove {
    First,
    Last,
    Next,
    Previous,
}

#[derive(Clone)]
pub struct SelectItemMetadata<T: Clone + Eq + 'static> {
    value: T,
    label: Option<SharedString>,
    disabled: bool,
    index: usize,
    group_index: Option<usize>,
    list_child_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> SelectItemMetadata<T> {
    pub fn new(value: T, label: Option<SharedString>, disabled: bool, index: usize) -> Self {
        Self {
            value,
            label,
            disabled,
            index,
            group_index: None,
            list_child_index: None,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn group_index(&self) -> Option<usize> {
        self.group_index
    }

    pub fn with_group_index(mut self, group_index: Option<usize>) -> Self {
        self.group_index = group_index;
        self
    }

    pub fn list_child_index(&self) -> Option<usize> {
        self.list_child_index
    }

    pub fn with_list_child_index(mut self, list_child_index: Option<usize>) -> Self {
        self.list_child_index = list_child_index;
        self
    }
}

#[derive(Clone)]
pub struct SelectGroupMetadata {
    index: usize,
    label: Option<SharedString>,
}

impl SelectGroupMetadata {
    pub fn new(index: usize) -> Self {
        Self { index, label: None }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn label(&self) -> Option<&SharedString> {
        self.label.as_ref()
    }

    pub fn set_label(&mut self, label: Option<SharedString>) {
        self.label = label;
    }
}

pub struct SelectOpenChangeOutcome {
    changed: bool,
    open: bool,
}

impl SelectOpenChangeOutcome {
    fn new(changed: bool, open: bool) -> Self {
        Self { changed, open }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn open(&self) -> bool {
        self.open
    }
}

pub enum SelectSelectionChange<T: Clone + Eq + 'static> {
    None,
    Single(SelectSingleValueChangeOutcome<T>),
    Multiple(SelectMultipleValuesChangeOutcome<T>),
}

impl<T: Clone + Eq + 'static> SelectSelectionChange<T> {
    pub fn changed(&self) -> bool {
        match self {
            Self::None => false,
            Self::Single(outcome) => outcome.changed(),
            Self::Multiple(outcome) => outcome.changed(),
        }
    }
}

pub struct SelectSingleValueChangeOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    value: Option<T>,
}

impl<T: Clone + Eq + 'static> SelectSingleValueChangeOutcome<T> {
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
pub enum SelectTypeaheadOutcome<T: Clone + Eq + 'static> {
    Highlighted(T),
    Commit(T),
    NoMatch,
    Disabled,
}

pub struct SelectMultipleValuesChangeOutcome<T: Clone + Eq + 'static> {
    changed: bool,
    values: Vec<T>,
}

impl<T: Clone + Eq + 'static> SelectMultipleValuesChangeOutcome<T> {
    fn new(changed: bool, values: Vec<T>) -> Self {
        Self { changed, values }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn into_values(self) -> Vec<T> {
        self.values
    }
}

#[derive(Clone)]
pub struct SelectRuntime<T: Clone + Eq + 'static> {
    selection_mode: SelectSelectionMode,
    value_comparator: Option<SelectValueComparator<T>>,
    selected: Option<T>,
    initial_selected: Option<T>,
    selected_values: Vec<T>,
    initial_selected_values: Vec<T>,
    open: bool,
    last_open_source: SelectOpenChangeSource,
    items: Vec<SelectItemMetadata<T>>,
    groups: Vec<SelectGroupMetadata>,
    highlighted_item_index: Option<usize>,
    item_focus_handles: Vec<(usize, FocusHandle)>,
    list_scroll_handle: ScrollHandle,
    scroll_up_visible: bool,
    scroll_down_visible: bool,
    trigger_focus_handle: Option<FocusHandle>,
    trigger_focused: bool,
    focused_item_index: Option<usize>,
    last_controlled_single_fallback: Option<(T, Option<T>)>,
    was_focused: bool,
    touched: bool,
    focus_out_close_requested: bool,
    trigger_bounds: Option<Bounds<Pixels>>,
    value_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    item_bounds: Vec<(usize, Bounds<Pixels>)>,
    item_text_bounds: Vec<(usize, Bounds<Pixels>)>,
    available_size: Option<Size<Pixels>>,
    typeahead: SharedString,
    last_typeahead_at: Option<Instant>,
}

impl<T: Clone + Eq + 'static> SelectRuntime<T> {
    pub fn new(
        selection_mode: SelectSelectionMode,
        selected: Option<T>,
        selected_values: Vec<T>,
        open: bool,
    ) -> Self {
        Self {
            selection_mode,
            value_comparator: None,
            initial_selected: selected.clone(),
            selected,
            initial_selected_values: selected_values.clone(),
            selected_values,
            open,
            last_open_source: SelectOpenChangeSource::None,
            items: Vec::new(),
            groups: Vec::new(),
            highlighted_item_index: None,
            item_focus_handles: Vec::new(),
            list_scroll_handle: ScrollHandle::new(),
            scroll_up_visible: false,
            scroll_down_visible: false,
            trigger_focus_handle: None,
            trigger_focused: false,
            focused_item_index: None,
            last_controlled_single_fallback: None,
            was_focused: false,
            touched: false,
            focus_out_close_requested: false,
            trigger_bounds: None,
            value_bounds: None,
            popup_bounds: None,
            item_bounds: Vec::new(),
            item_text_bounds: Vec::new(),
            available_size: None,
            typeahead: SharedString::default(),
            last_typeahead_at: None,
        }
    }

    /// Returns the currently observed single selected value.
    pub fn selected_value(&self) -> Option<T> {
        self.selected.clone()
    }

    /// Returns the currently observed ordered multiple selected values.
    pub fn selected_values(&self) -> Vec<T> {
        self.selected_values.clone()
    }

    /// Returns the currently observed popup open state.
    pub fn open_value(&self) -> bool {
        self.open
    }

    /// Replaces the runtime's mode before reconciling values for this render pass.
    pub fn sync_selection_mode(&mut self, selection_mode: SelectSelectionMode) {
        self.selection_mode = selection_mode;
    }

    /// Replaces the value comparator before reconciling values for this render pass.
    pub fn sync_value_comparator(&mut self, comparator: Option<SelectValueComparator<T>>) {
        self.value_comparator = comparator;
    }

    /// Reconciles the runtime with an externally controlled single selected value.
    pub fn sync_selected_from_context(&mut self, selected: Option<T>) {
        if !self.option_value_eq(self.selected.as_ref(), selected.as_ref()) {
            self.reset_typeahead();
        }
        self.selected = selected;
    }

    /// Reconciles the runtime with externally controlled multiple selected values.
    pub fn sync_selected_values_from_context(&mut self, selected_values: Vec<T>) {
        let selected_values = self.unique_ordered_values(selected_values);
        if !self.values_eq_ordered(&self.selected_values, &selected_values) {
            self.reset_typeahead();
        }
        self.selected_values = selected_values;
    }

    /// Reconciles the runtime with externally controlled open state.
    pub fn sync_open_from_context(&mut self, open: bool) {
        self.open = open;
        if open && self.highlighted_item_index.is_none() {
            self.sync_highlight_for_open();
        }
    }

    /// Replaces walked group metadata for this render pass.
    pub fn sync_groups(&mut self, mut groups: Vec<SelectGroupMetadata>) {
        groups.sort_by_key(SelectGroupMetadata::index);
        self.groups = groups;
    }

    /// Replaces walked item metadata and preserves highlighted value when possible.
    pub fn sync_children(
        &mut self,
        mut items: Vec<SelectItemMetadata<T>>,
        mut item_focus_handles: Vec<(usize, FocusHandle)>,
        trigger_focus_handle: Option<FocusHandle>,
        trigger_focused: bool,
        focused_item_index: Option<usize>,
    ) {
        let highlighted_value = self
            .highlighted_item_index
            .and_then(|index| self.value_at_index(index).cloned());

        items.sort_by_key(SelectItemMetadata::index);
        item_focus_handles.sort_by_key(|(index, _)| *index);

        self.items = items;
        self.item_bounds
            .retain(|(index, _)| self.items.iter().any(|item| item.index == *index));
        self.item_text_bounds
            .retain(|(index, _)| self.items.iter().any(|item| item.index == *index));
        self.item_focus_handles = item_focus_handles;
        self.trigger_focus_handle = trigger_focus_handle;
        self.highlighted_item_index = highlighted_value
            .as_ref()
            .and_then(|value| self.index_of_value(value))
            .or_else(|| self.highlight_for_selected());
        self.focused_item_index = focused_item_index;
        self.sync_focus(trigger_focused, focused_item_index);
    }

    /// Returns a controlled-mode fallback candidate when the observed selected value disappeared.
    pub fn take_controlled_single_fallback(
        &mut self,
        observed_selected: Option<T>,
    ) -> Option<Option<T>> {
        let Some(observed_value) = observed_selected else {
            self.last_controlled_single_fallback = None;
            return None;
        };
        if self
            .items
            .iter()
            .any(|item| self.value_eq(&item.value, &observed_value))
        {
            self.last_controlled_single_fallback = None;
            return None;
        }

        let fallback = self.fallback_single_value(Some(observed_value.clone()));
        if self
            .last_controlled_single_fallback
            .as_ref()
            .map(|(value, last_fallback)| {
                self.value_eq(value, &observed_value)
                    && self.option_value_eq(last_fallback.as_ref(), fallback.as_ref())
            })
            .unwrap_or(false)
        {
            return None;
        }

        self.last_controlled_single_fallback = Some((observed_value, fallback.clone()));
        Some(fallback)
    }

    /// Reconciles selected state for single selection and applies uncontrolled fallback.
    pub fn reconcile_single(&mut self, observed_selected: Option<T>, allow_fallback: bool) {
        let selected = match allow_fallback {
            true => self.fallback_single_value(observed_selected),
            false => observed_selected,
        };
        let changed = !self.option_value_eq(self.selected.as_ref(), selected.as_ref());
        self.selected = selected;

        if changed || self.highlighted_item_index.is_none() {
            self.sync_highlight_for_open();
        }
    }

    /// Reconciles selected values for multiple selection and prunes missing uncontrolled values.
    pub fn reconcile_multiple(&mut self, observed_values: Vec<T>, allow_prune: bool) {
        let mut values = self.unique_ordered_values(observed_values);
        if allow_prune && !self.items.is_empty() {
            values.retain(|value| {
                self.items
                    .iter()
                    .any(|item| self.value_eq(&item.value, value))
            });
        }
        let changed = !self.values_eq_ordered(&self.selected_values, &values);
        self.selected_values = values;

        if changed || self.highlighted_item_index.is_none() {
            self.sync_highlight_for_open();
        }
    }

    /// Records trigger bounds measured during GPUI prepaint.
    pub fn set_trigger_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.trigger_bounds == Some(bounds) {
            return false;
        }

        self.trigger_bounds = Some(bounds);
        true
    }

    /// Records value bounds measured during GPUI prepaint.
    pub fn set_value_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.value_bounds == Some(bounds) {
            return false;
        }

        self.value_bounds = Some(bounds);
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

    /// Records item bounds measured during GPUI prepaint.
    pub fn set_item_bounds(&mut self, index: usize, bounds: Bounds<Pixels>) -> bool {
        set_indexed_bounds(&mut self.item_bounds, index, bounds)
    }

    /// Records item-text bounds measured during GPUI prepaint.
    pub fn set_item_text_bounds(&mut self, index: usize, bounds: Bounds<Pixels>) -> bool {
        set_indexed_bounds(&mut self.item_text_bounds, index, bounds)
    }

    /// Records the current available viewport size for positioning diagnostics.
    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }

        self.available_size = Some(size);
        true
    }

    /// Requests an open-state transition using Select rules.
    pub fn request_open_change(
        &self,
        current: bool,
        next: bool,
        disabled: bool,
        read_only: bool,
    ) -> SelectOpenChangeOutcome {
        if current == next {
            return SelectOpenChangeOutcome::new(false, current);
        }

        if next && (disabled || read_only) {
            return SelectOpenChangeOutcome::new(false, current);
        }

        SelectOpenChangeOutcome::new(true, next)
    }

    /// Records the source of an accepted open-state transition.
    pub fn record_open_source(&mut self, source: SelectOpenChangeSource) {
        if source != SelectOpenChangeSource::None {
            self.last_open_source = source;
        }
    }

    /// Commits an accepted open-state transition.
    pub fn commit_open(&mut self, open: bool, source: SelectOpenChangeSource) {
        self.open = open;
        if source != SelectOpenChangeSource::None {
            self.last_open_source = source;
        }
        if open {
            self.sync_highlight_for_open();
        }
    }

    /// Requests selecting or toggling one item value using current mode and root rules.
    pub fn request_select_value(
        &self,
        current_single: Option<T>,
        current_multiple: Vec<T>,
        value: T,
        item_disabled: bool,
        root_disabled: bool,
        root_read_only: bool,
    ) -> SelectSelectionChange<T> {
        if item_disabled || root_disabled || root_read_only {
            return SelectSelectionChange::None;
        }

        match self.selection_mode {
            SelectSelectionMode::Single => {
                if self.option_value_eq(current_single.as_ref(), Some(&value)) {
                    SelectSelectionChange::Single(SelectSingleValueChangeOutcome::new(
                        false,
                        current_single,
                    ))
                } else {
                    SelectSelectionChange::Single(SelectSingleValueChangeOutcome::new(
                        true,
                        Some(value),
                    ))
                }
            }
            SelectSelectionMode::Multiple => {
                let mut next_values = self.unique_ordered_values(current_multiple);
                if let Some(position) = next_values
                    .iter()
                    .position(|selected| self.value_eq(selected, &value))
                {
                    next_values.remove(position);
                } else {
                    next_values.push(value);
                }

                SelectSelectionChange::Multiple(SelectMultipleValuesChangeOutcome::new(
                    !self.values_eq_ordered(&next_values, &self.selected_values),
                    next_values,
                ))
            }
        }
    }

    /// Commits an accepted single selected value.
    pub fn commit_selected(&mut self, selected: Option<T>) {
        self.selected = selected;
        self.reset_typeahead();
        self.sync_highlight_for_open();
    }

    /// Commits accepted multiple selected values.
    pub fn commit_selected_values(&mut self, selected_values: Vec<T>) {
        self.selected_values = self.unique_ordered_values(selected_values);
        self.reset_typeahead();
        self.sync_highlight_for_open();
    }

    /// Moves highlight through registered items, including disabled items.
    pub fn move_highlight(&mut self, direction: SelectMove, loop_focus: bool) {
        match direction {
            SelectMove::First => self.highlighted_item_index = self.first_index(),
            SelectMove::Last => self.highlighted_item_index = self.last_index(),
            SelectMove::Next => {
                if let Some(index) = self.next_index(self.highlighted_item_index, loop_focus) {
                    self.highlighted_item_index = Some(index);
                }
            }
            SelectMove::Previous => {
                if let Some(index) = self.previous_index(self.highlighted_item_index, loop_focus) {
                    self.highlighted_item_index = Some(index);
                }
            }
        }
    }

    /// Applies typed text to Select typeahead and either highlights or commits the next match.
    pub fn apply_typeahead(&mut self, text: &str, now: Instant) -> SelectTypeaheadOutcome<T> {
        if self.selection_mode == SelectSelectionMode::Multiple {
            return SelectTypeaheadOutcome::Disabled;
        }

        let text = text.trim();
        if text.is_empty() {
            return SelectTypeaheadOutcome::NoMatch;
        }

        if self
            .last_typeahead_at
            .map(|last| now.duration_since(last) > TYPEAHEAD_RESET_AFTER)
            .unwrap_or(true)
        {
            self.typeahead = SharedString::default();
        }
        self.last_typeahead_at = Some(now);

        let next_text = format!("{}{}", self.typeahead, text);
        let repeated = next_text
            .chars()
            .next()
            .map(|first| next_text.chars().all(|ch| ch.eq_ignore_ascii_case(&first)))
            .unwrap_or(false);
        let search = if repeated { text } else { next_text.as_str() };

        let Some(value) = self.highlight_by_typeahead(search) else {
            return SelectTypeaheadOutcome::NoMatch;
        };

        self.typeahead = next_text.into();
        match self.open {
            true => SelectTypeaheadOutcome::Highlighted(value),
            false => SelectTypeaheadOutcome::Commit(value),
        }
    }

    /// Clears the current typeahead search buffer.
    pub fn reset_typeahead(&mut self) {
        self.typeahead = SharedString::default();
        self.last_typeahead_at = None;
    }

    /// Highlights the next enabled item whose label starts with the supplied typeahead text.
    pub fn highlight_by_typeahead(&mut self, text: &str) -> Option<T> {
        let text = text.trim().to_lowercase();
        if text.is_empty() {
            return None;
        }

        let current_position = self
            .highlighted_item_index
            .and_then(|index| self.items.iter().position(|item| item.index == index));
        let len = self.items.len();
        if len == 0 {
            return None;
        }

        for offset in 1..=len {
            let position = match current_position {
                Some(position) => (position + offset) % len,
                None => offset - 1,
            };
            let item = &self.items[position];
            if item.disabled {
                continue;
            }
            let label = item.label.as_ref().map(|label| label.to_string());
            if label
                .as_ref()
                .map(|label| label.to_lowercase().starts_with(&text))
                .unwrap_or(false)
            {
                self.highlighted_item_index = Some(item.index);
                return Some(item.value.clone());
            }
        }

        None
    }

    /// Highlights one item by index when the item is selectable.
    pub fn highlight_item(&mut self, index: Option<usize>, disabled: bool) {
        if !disabled {
            self.highlighted_item_index = index;
        }
    }

    /// Returns registered group metadata in render order.
    pub fn group_metadata(&self) -> &[SelectGroupMetadata] {
        &self.groups
    }

    /// Returns the list scroll handle for GPUI-native scrolling.
    pub fn list_scroll_handle(&self) -> ScrollHandle {
        self.list_scroll_handle.clone()
    }

    /// Refreshes scroll-arrow visibility from the tracked list scroll state.
    pub fn refresh_scroll_arrow_visibility(&mut self) -> bool {
        let offset = self.list_scroll_handle.offset();
        let max_offset = self.list_scroll_handle.max_offset();
        let up_visible = offset.y < Pixels::ZERO;
        let down_visible = max_offset.y > Pixels::ZERO && -offset.y < max_offset.y;
        let changed =
            self.scroll_up_visible != up_visible || self.scroll_down_visible != down_visible;

        self.scroll_up_visible = up_visible;
        self.scroll_down_visible = down_visible;
        changed
    }

    /// Scrolls the list toward the requested edge by one visible item.
    pub fn scroll_toward(&mut self, direction: SelectScrollArrowDirection) {
        if self.items.is_empty() {
            return;
        }

        let index = match direction {
            SelectScrollArrowDirection::Up => self.list_scroll_handle.top_item().saturating_sub(1),
            SelectScrollArrowDirection::Down => {
                (self.list_scroll_handle.bottom_item() + 1).min(self.items.len().saturating_sub(1))
            }
        };
        self.list_scroll_handle.scroll_to_item(index);
    }

    /// Requests that the currently highlighted list child be scrolled into view.
    pub fn scroll_highlighted_into_view(&mut self) {
        let Some(highlighted_item_index) = self.highlighted_item_index else {
            return;
        };
        let Some(list_child_index) = self
            .items
            .iter()
            .find(|item| item.index == highlighted_item_index)
            .and_then(SelectItemMetadata::list_child_index)
        else {
            return;
        };

        self.list_scroll_handle.scroll_to_item(list_child_index);
    }

    /// Returns the currently highlighted item value.
    pub fn highlighted_value(&self) -> Option<T> {
        self.highlighted_item_index
            .and_then(|index| self.value_at_index(index).cloned())
    }

    /// Returns the currently highlighted item value and disabled state.
    pub fn highlighted_value_with_disabled(&self) -> Option<(T, bool)> {
        self.highlighted_item_index.and_then(|index| {
            self.items
                .iter()
                .find(|item| item.index == index)
                .map(|item| (item.value.clone(), item.disabled))
        })
    }

    /// Returns the focus handle for the highlighted item, when one is registered.
    pub fn highlighted_focus_handle(&self) -> Option<FocusHandle> {
        self.highlighted_item_index
            .and_then(|index| self.focus_handle_at_index(index))
    }

    /// Returns the trigger focus handle, when one is registered.
    pub fn trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.trigger_focus_handle.clone()
    }

    /// Takes a pending request to close after focus leaves the whole Select.
    pub fn take_focus_out_close_request(&mut self) -> bool {
        let requested = self.focus_out_close_requested;
        self.focus_out_close_requested = false;
        requested
    }

    pub fn root_state(&self, props: &SelectProps<T>) -> SelectRootStyleState<T> {
        SelectRootStyleState::new(
            props.disabled(),
            props.read_only(),
            props.required(),
            self.open,
            self.last_open_source,
            self.focused(),
            self.dirty(),
            self.touched,
            None,
            self.selection_mode,
            self.selected.clone(),
            self.selected_values.clone(),
            self.selected_index(),
        )
    }

    pub fn trigger_state(
        &self,
        props: &SelectProps<T>,
        side: SelectSide,
    ) -> SelectTriggerStyleState<T> {
        SelectTriggerStyleState::new(self.root_state(props), side)
    }

    pub fn value_state(
        &self,
        props: &SelectProps<T>,
        placeholder: Option<SharedString>,
    ) -> SelectValueStyleState<T> {
        let selected_labels = self.selected_labels(props);
        let value_present = match self.selection_mode {
            SelectSelectionMode::Single => self.selected.is_some(),
            SelectSelectionMode::Multiple => !self.selected_values.is_empty(),
        };
        let placeholder_active = !value_present;
        let display_text = if placeholder_active {
            placeholder.unwrap_or_default()
        } else if self.selection_mode == SelectSelectionMode::Multiple {
            props
                .format_multiple_value(&selected_labels, &self.selected_values)
                .unwrap_or_else(|| join_labels(&selected_labels))
        } else {
            selected_labels.first().cloned().unwrap_or_default()
        };

        SelectValueStyleState::new(
            self.selection_mode,
            self.selected.clone(),
            self.selected_values.clone(),
            selected_labels,
            placeholder_active,
            display_text,
        )
    }

    pub fn icon_state(&self) -> SelectIconStyleState {
        SelectIconStyleState::new(self.open)
    }

    pub fn positioner_state(
        &self,
        side: SelectSide,
        align: SelectAlign,
    ) -> SelectPositionerStyleState {
        SelectPositionerStyleState::new(
            self.open,
            self.last_open_source,
            side,
            align,
            self.trigger_bounds,
            self.popup_bounds,
            self.available_size,
        )
    }

    pub fn portal_state(&self, force_mounted: bool) -> SelectPortalStyleState {
        SelectPortalStyleState::new(self.open, self.open || force_mounted)
    }

    pub fn popup_state(
        &self,
        side: SelectSide,
        align: SelectAlign,
        force_mounted: bool,
    ) -> SelectPopupStyleState {
        SelectPopupStyleState::new(self.open, self.open || force_mounted, side, align)
    }

    pub fn backdrop_state(&self, force_mounted: bool) -> SelectBackdropStyleState {
        SelectBackdropStyleState::new(self.open, self.open || force_mounted)
    }

    pub fn arrow_state(&self, side: SelectSide, align: SelectAlign) -> SelectArrowStyleState {
        SelectArrowStyleState::new(self.open, side, align, false)
    }

    pub fn list_state(&self) -> SelectListStyleState {
        SelectListStyleState::new(self.open, self.items.len())
    }

    pub fn group_state(
        &self,
        group_index: Option<usize>,
        item_count: usize,
    ) -> SelectGroupStyleState {
        let label = group_index.and_then(|group_index| {
            self.groups
                .iter()
                .find(|group| group.index == group_index)
                .and_then(|group| group.label.clone())
        });

        SelectGroupStyleState::new(item_count, group_index, label)
    }

    pub fn item_state(
        &self,
        value: Option<&T>,
        disabled: bool,
        index: Option<usize>,
        focused: bool,
        props: &SelectProps<T>,
    ) -> SelectItemStyleState<T> {
        let selected = value
            .map(|value| match self.selection_mode {
                SelectSelectionMode::Single => {
                    self.option_value_eq(self.selected.as_ref(), Some(value))
                }
                SelectSelectionMode::Multiple => {
                    self.selected_values.iter().any(|v| self.value_eq(v, value))
                }
            })
            .unwrap_or(false);
        let highlighted = index == self.highlighted_item_index;
        let root_disabled = props.disabled();
        let tab_stop = highlighted && !root_disabled;
        let group_index = index.and_then(|index| {
            self.items
                .iter()
                .find(|item| item.index == index)
                .and_then(SelectItemMetadata::group_index)
        });
        let item_bounds = index.and_then(|index| self.item_bounds_at_index(index));
        let item_text_bounds = index.and_then(|index| self.item_text_bounds_at_index(index));

        SelectItemStyleState::new(
            selected,
            highlighted,
            disabled,
            props.read_only(),
            root_disabled,
            focused,
            tab_stop,
            index,
            group_index,
            item_bounds,
            item_text_bounds,
            value.cloned(),
        )
    }

    pub fn item_indicator_state(
        &self,
        item_state: &SelectItemStyleState<T>,
        keep_mounted: bool,
    ) -> SelectItemIndicatorStyleState {
        SelectItemIndicatorStyleState::new(item_state.selected, item_state.selected || keep_mounted)
    }

    pub fn item_text_state(
        &self,
        item_state: &SelectItemStyleState<T>,
    ) -> SelectItemTextStyleState {
        SelectItemTextStyleState::new(
            item_state.selected,
            item_state.highlighted,
            item_state.item_text_bounds,
        )
    }

    pub fn scroll_arrow_state(
        &self,
        direction: SelectScrollArrowDirection,
        side: SelectSide,
        keep_mounted: bool,
    ) -> SelectScrollArrowStyleState {
        let visible = if self.last_open_source == SelectOpenChangeSource::Touch {
            false
        } else {
            match direction {
                SelectScrollArrowDirection::Up => self.scroll_up_visible,
                SelectScrollArrowDirection::Down => self.scroll_down_visible,
            }
        };

        SelectScrollArrowStyleState::new(direction, visible, side, keep_mounted)
    }

    pub fn trigger_bounds(&self) -> Option<Bounds<Pixels>> {
        self.trigger_bounds
    }

    pub fn value_bounds(&self) -> Option<Bounds<Pixels>> {
        self.value_bounds
    }

    pub fn selected_item_text_bounds(&self) -> Option<Bounds<Pixels>> {
        self.selected_index()
            .and_then(|index| self.item_text_bounds_at_index(index))
    }

    fn value_eq(&self, left: &T, right: &T) -> bool {
        self.value_comparator
            .as_ref()
            .map(|comparator| comparator(left, right))
            .unwrap_or_else(|| left == right)
    }

    fn option_value_eq(&self, left: Option<&T>, right: Option<&T>) -> bool {
        match (left, right) {
            (Some(left), Some(right)) => self.value_eq(left, right),
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
        }
    }

    fn values_eq_ordered(&self, left: &[T], right: &[T]) -> bool {
        left.len() == right.len()
            && left
                .iter()
                .zip(right.iter())
                .all(|(left, right)| self.value_eq(left, right))
    }

    fn unique_ordered_values(&self, values: Vec<T>) -> Vec<T> {
        let mut unique = Vec::new();
        for value in values {
            if !unique
                .iter()
                .any(|existing| self.value_eq(existing, &value))
            {
                unique.push(value);
            }
        }
        unique
    }

    fn sync_focus(&mut self, trigger_focused: bool, focused_item_index: Option<usize>) {
        self.trigger_focused = trigger_focused;
        let focused = self.focused();
        if self.was_focused && !focused {
            self.touched = true;
            if self.open {
                self.focus_out_close_requested = true;
            }
        }
        self.was_focused = focused;
        self.focused_item_index = focused_item_index;
    }

    fn focused(&self) -> bool {
        self.trigger_focused || self.focused_item_index.is_some()
    }

    fn dirty(&self) -> bool {
        match self.selection_mode {
            SelectSelectionMode::Single => {
                !self.option_value_eq(self.selected.as_ref(), self.initial_selected.as_ref())
            }
            SelectSelectionMode::Multiple => {
                !self.values_eq_ordered(&self.selected_values, &self.initial_selected_values)
            }
        }
    }

    fn selected_index(&self) -> Option<usize> {
        let selected = self.selected.as_ref()?;

        self.items
            .iter()
            .find(|item| self.value_eq(&item.value, selected))
            .map(SelectItemMetadata::index)
    }

    fn selected_labels(&self, props: &SelectProps<T>) -> Vec<SharedString> {
        match self.selection_mode {
            SelectSelectionMode::Single => self
                .selected
                .as_ref()
                .and_then(|value| self.label_for_value(value, props))
                .into_iter()
                .collect(),
            SelectSelectionMode::Multiple => self
                .selected_values
                .iter()
                .filter_map(|value| self.label_for_value(value, props))
                .collect(),
        }
    }

    fn label_for_value(&self, value: &T, props: &SelectProps<T>) -> Option<SharedString> {
        self.items
            .iter()
            .find(|item| self.value_eq(&item.value, value))
            .and_then(|item| item.label.clone())
            .or_else(|| props.label_for_value(value))
    }

    fn fallback_single_value(&self, observed_selected: Option<T>) -> Option<T> {
        if observed_selected
            .as_ref()
            .map(|value| {
                self.items
                    .iter()
                    .any(|item| self.value_eq(&item.value, value))
            })
            .unwrap_or(true)
        {
            return observed_selected;
        }

        if self
            .initial_selected
            .as_ref()
            .map(|value| {
                self.items
                    .iter()
                    .any(|item| self.value_eq(&item.value, value))
            })
            .unwrap_or(false)
        {
            return self.initial_selected.clone();
        }

        None
    }

    fn sync_highlight_for_open(&mut self) {
        if !self.open {
            return;
        }

        self.highlighted_item_index = self
            .highlight_for_selected()
            .or_else(|| self.first_enabled_index());
    }

    fn highlight_for_selected(&self) -> Option<usize> {
        match self.selection_mode {
            SelectSelectionMode::Single => self
                .selected
                .as_ref()
                .and_then(|value| self.index_of_value(value)),
            SelectSelectionMode::Multiple => self
                .selected_values
                .iter()
                .find_map(|value| self.index_of_value(value)),
        }
    }

    fn value_at_index(&self, index: usize) -> Option<&T> {
        self.items
            .iter()
            .find(|item| item.index == index)
            .map(|item| &item.value)
    }

    fn index_of_value(&self, value: &T) -> Option<usize> {
        self.items
            .iter()
            .find(|item| self.value_eq(&item.value, value))
            .map(|item| item.index)
    }

    fn first_enabled_index(&self) -> Option<usize> {
        self.items
            .iter()
            .find(|item| !item.disabled)
            .map(|item| item.index)
    }

    fn first_index(&self) -> Option<usize> {
        self.items.first().map(|item| item.index)
    }

    fn last_index(&self) -> Option<usize> {
        self.items.last().map(|item| item.index)
    }

    fn next_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        let start = current
            .and_then(|index| self.items.iter().position(|item| item.index == index))
            .map(|position| position + 1)
            .unwrap_or(0);

        self.index_from_positions(start..self.items.len())
            .or_else(|| {
                loop_focus
                    .then(|| self.index_from_positions(0..start))
                    .flatten()
            })
    }

    fn previous_index(&self, current: Option<usize>, loop_focus: bool) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        let start = current
            .and_then(|index| self.items.iter().position(|item| item.index == index))
            .unwrap_or(self.items.len());

        self.index_from_positions((0..start).rev()).or_else(|| {
            loop_focus
                .then(|| self.index_from_positions((start..self.items.len()).rev()))
                .flatten()
        })
    }

    fn index_from_positions(&self, positions: impl IntoIterator<Item = usize>) -> Option<usize> {
        positions
            .into_iter()
            .filter_map(|position| self.items.get(position))
            .map(|item| item.index)
            .next()
    }

    fn focus_handle_at_index(&self, index: usize) -> Option<FocusHandle> {
        self.item_focus_handles
            .iter()
            .find(|(item_index, _)| *item_index == index)
            .map(|(_, focus_handle)| focus_handle.clone())
    }

    fn item_bounds_at_index(&self, index: usize) -> Option<Bounds<Pixels>> {
        self.item_bounds
            .iter()
            .find_map(|(item_index, bounds)| (*item_index == index).then_some(*bounds))
    }

    fn item_text_bounds_at_index(&self, index: usize) -> Option<Bounds<Pixels>> {
        self.item_text_bounds
            .iter()
            .find_map(|(item_index, bounds)| (*item_index == index).then_some(*bounds))
    }
}

const TYPEAHEAD_RESET_AFTER: Duration = Duration::from_millis(1_000);

fn set_indexed_bounds(
    bounds_by_index: &mut Vec<(usize, Bounds<Pixels>)>,
    index: usize,
    bounds: Bounds<Pixels>,
) -> bool {
    if let Some((_, existing_bounds)) = bounds_by_index
        .iter_mut()
        .find(|(item_index, _)| *item_index == index)
    {
        if *existing_bounds == bounds {
            return false;
        }
        *existing_bounds = bounds;
        return true;
    }

    bounds_by_index.push((index, bounds));
    bounds_by_index.sort_by_key(|(item_index, _)| *item_index);
    true
}

fn join_labels(labels: &[SharedString]) -> SharedString {
    labels
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
        .into()
}
