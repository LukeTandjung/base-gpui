use gpui::{Bounds, FocusHandle, Pixels, ScrollHandle, SharedString, Size};

use crate::combobox::{
    ComboboxAlign, ComboboxArrowStyleState, ComboboxAutoHighlight, ComboboxBackdropStyleState,
    ComboboxChipRemoveStyleState, ComboboxChipStyleState, ComboboxChipsStyleState,
    ComboboxClearStyleState, ComboboxEmptyStyleState, ComboboxFilter, ComboboxGroupStyleState,
    ComboboxIconStyleState, ComboboxInputGroupStyleState, ComboboxInputStyleState,
    ComboboxItemIndicatorStyleState, ComboboxItemStyleState, ComboboxLabelResolver,
    ComboboxListStyleState, ComboboxPopupStyleState, ComboboxPortalStyleState,
    ComboboxPositionerStyleState, ComboboxProps, ComboboxRootStyleState, ComboboxSelectionMode,
    ComboboxSide, ComboboxTriggerStyleState, ComboboxValueStyleState,
};

/// Reason attached to Combobox value/input-value/open change details.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxChangeReason {
    #[default]
    None,
    InputChange,
    InputClear,
    ItemPress,
    TriggerPress,
    OutsidePress,
    EscapeKey,
    ListNavigation,
    FocusOut,
    ClearPress,
    ChipRemovePress,
    CancelOpen,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ComboboxChangeSource {
    Pointer,
    Keyboard,
    Programmatic,
    #[default]
    None,
}

/// Rust-native change details (reason, source, cancel state) shared by the
/// three controlled axes: selected value(s), input value, open.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComboboxChangeDetails {
    reason: ComboboxChangeReason,
    source: ComboboxChangeSource,
    cancelable: bool,
    canceled: bool,
}

impl ComboboxChangeDetails {
    pub fn new(
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        cancelable: bool,
    ) -> Self {
        Self {
            reason,
            source,
            cancelable,
            canceled: false,
        }
    }

    pub fn reason(&self) -> ComboboxChangeReason {
        self.reason
    }

    pub fn source(&self) -> ComboboxChangeSource {
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
pub enum ComboboxHighlightReason {
    Keyboard,
    Pointer,
    #[default]
    None,
}

/// Details passed to `on_item_highlighted`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComboboxItemHighlightDetails {
    pub reason: ComboboxHighlightReason,
    pub index: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComboboxMove {
    First,
    Last,
    Next,
    Previous,
}

#[derive(Clone)]
pub struct ComboboxItemMetadata<T: Clone + Eq + 'static> {
    value: T,
    label: Option<SharedString>,
    disabled: bool,
    index: usize,
    group_index: Option<usize>,
    list_child_index: Option<usize>,
}

impl<T: Clone + Eq + 'static> ComboboxItemMetadata<T> {
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
pub struct ComboboxGroupMetadata {
    index: usize,
    label: Option<SharedString>,
}

impl ComboboxGroupMetadata {
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

pub struct ComboboxOpenChangeOutcome {
    changed: bool,
    open: bool,
}

impl ComboboxOpenChangeOutcome {
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

pub enum ComboboxSelectionChange<T: Clone + Eq + 'static> {
    None,
    Single { changed: bool, value: Option<T> },
    Multiple { changed: bool, values: Vec<T> },
}

impl<T: Clone + Eq + 'static> ComboboxSelectionChange<T> {
    pub fn changed(&self) -> bool {
        match self {
            Self::None => false,
            Self::Single { changed, .. } | Self::Multiple { changed, .. } => *changed,
        }
    }
}

/// Where a chip-highlight move ended up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComboboxChipMoveOutcome {
    Chip(usize),
    Input,
    NoChips,
}

/// Deep, unit-testable Combobox core: open state, filtered options, highlight,
/// input value, selected value(s), and chip highlight. Commands never call
/// user callbacks; `ComboboxContext` mediates controlled state and callbacks.
#[derive(Clone)]
pub struct ComboboxRuntime<T: Clone + Eq + 'static> {
    selection_mode: ComboboxSelectionMode,
    auto_highlight: ComboboxAutoHighlight,
    limit: Option<usize>,
    filter: Option<ComboboxFilter<T>>,
    filter_disabled: bool,
    label_resolver: Option<ComboboxLabelResolver<T>>,
    selected: Option<T>,
    initial_selected: Option<T>,
    selected_values: Vec<T>,
    initial_selected_values: Vec<T>,
    input_value: SharedString,
    initial_input_value: SharedString,
    inline_overlay: Option<SharedString>,
    open: bool,
    query_changed_after_open: bool,
    close_query: Option<SharedString>,
    items: Vec<ComboboxItemMetadata<T>>,
    groups: Vec<ComboboxGroupMetadata>,
    filtered_indices: Vec<usize>,
    highlighted_index: Option<usize>,
    last_notified_highlight: Option<usize>,
    highlighted_chip_index: Option<usize>,
    list_scroll_handle: ScrollHandle,
    input_focus_handle: Option<FocusHandle>,
    input_focused: bool,
    was_focused: bool,
    touched: bool,
    focus_out_close_requested: bool,
    anchor_bounds: Option<Bounds<Pixels>>,
    input_group_bounds: Option<Bounds<Pixels>>,
    input_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
}

impl<T: Clone + Eq + 'static> ComboboxRuntime<T> {
    pub fn new(
        selection_mode: ComboboxSelectionMode,
        selected: Option<T>,
        selected_values: Vec<T>,
        input_value: SharedString,
        open: bool,
    ) -> Self {
        Self {
            selection_mode,
            auto_highlight: ComboboxAutoHighlight::Off,
            limit: None,
            filter: None,
            filter_disabled: false,
            label_resolver: None,
            initial_selected: selected.clone(),
            selected,
            initial_selected_values: selected_values.clone(),
            selected_values,
            initial_input_value: input_value.clone(),
            input_value,
            inline_overlay: None,
            open,
            query_changed_after_open: false,
            close_query: None,
            items: Vec::new(),
            groups: Vec::new(),
            filtered_indices: Vec::new(),
            highlighted_index: None,
            last_notified_highlight: None,
            highlighted_chip_index: None,
            list_scroll_handle: ScrollHandle::new(),
            input_focus_handle: None,
            input_focused: false,
            was_focused: false,
            touched: false,
            focus_out_close_requested: false,
            anchor_bounds: None,
            input_group_bounds: None,
            input_bounds: None,
            popup_bounds: None,
            available_size: None,
        }
    }

    // ------------------------------------------------------------------
    // Observed values
    // ------------------------------------------------------------------

    pub fn selected_value(&self) -> Option<T> {
        self.selected.clone()
    }

    pub fn selected_values(&self) -> Vec<T> {
        self.selected_values.clone()
    }

    /// The current input value (the real value, ignoring the inline overlay).
    /// This is the public input-value query consumed by e.g. an
    /// `AutocompleteValue`-style part.
    pub fn input_value(&self) -> SharedString {
        self.input_value.clone()
    }

    /// The string the input layer paints: the inline overlay when present,
    /// else the real input value. The overlay is a whole-value replacement,
    /// applied through the input primitive's controlled-value path, and is
    /// never treated as user typing.
    pub fn display_value(&self) -> SharedString {
        self.inline_overlay
            .clone()
            .unwrap_or_else(|| self.input_value.clone())
    }

    pub fn open_value(&self) -> bool {
        self.open
    }

    /// The active filter query: the frozen close-query while closing, else
    /// the trimmed input value.
    pub fn query(&self) -> String {
        self.close_query
            .as_ref()
            .map(|query| query.trim().to_string())
            .unwrap_or_else(|| self.input_value.trim().to_string())
    }

    pub fn query_changed_after_open(&self) -> bool {
        self.query_changed_after_open
    }

    pub fn filtered_indices(&self) -> Vec<usize> {
        self.filtered_indices.clone()
    }

    pub fn list_empty(&self) -> bool {
        self.filtered_indices.is_empty()
    }

    pub fn item_visible(&self, index: usize) -> bool {
        self.filtered_indices.contains(&index)
    }

    pub fn is_index_highlighted(&self, index: usize) -> bool {
        self.highlighted_index == Some(index)
    }

    pub fn highlighted_index(&self) -> Option<usize> {
        self.highlighted_index
    }

    pub fn highlighted_value(&self) -> Option<T> {
        self.highlighted_index
            .and_then(|index| self.item_at(index))
            .map(|item| item.value.clone())
    }

    pub fn highlighted_value_with_disabled(&self) -> Option<(T, bool)> {
        self.highlighted_index
            .and_then(|index| self.item_at(index))
            .map(|item| (item.value.clone(), item.disabled))
    }

    pub fn highlighted_chip_index(&self) -> Option<usize> {
        self.highlighted_chip_index
    }

    pub fn list_scroll_handle(&self) -> ScrollHandle {
        self.list_scroll_handle.clone()
    }

    pub fn input_focus_handle(&self) -> Option<FocusHandle> {
        self.input_focus_handle.clone()
    }

    pub fn group_metadata(&self) -> &[ComboboxGroupMetadata] {
        &self.groups
    }

    pub fn take_focus_out_close_request(&mut self) -> bool {
        let requested = self.focus_out_close_requested;
        self.focus_out_close_requested = false;
        requested
    }

    /// Returns the new highlighted index when it changed since the last take,
    /// so `on_item_highlighted` fires exactly once per transition.
    pub fn take_highlight_transition(&mut self) -> Option<Option<usize>> {
        if self.highlighted_index == self.last_notified_highlight {
            return None;
        }
        self.last_notified_highlight = self.highlighted_index;
        Some(self.highlighted_index)
    }

    // ------------------------------------------------------------------
    // Sync / reconcile
    // ------------------------------------------------------------------

    pub fn sync_selection_mode(&mut self, selection_mode: ComboboxSelectionMode) {
        self.selection_mode = selection_mode;
    }

    /// Replaces filtering configuration before reconciling this render pass.
    pub fn sync_filtering(
        &mut self,
        filter: Option<ComboboxFilter<T>>,
        filter_disabled: bool,
        label_resolver: Option<ComboboxLabelResolver<T>>,
        limit: Option<usize>,
        auto_highlight: ComboboxAutoHighlight,
    ) {
        self.filter = filter;
        self.filter_disabled = filter_disabled;
        self.label_resolver = label_resolver;
        self.limit = limit;
        self.auto_highlight = auto_highlight;
    }

    pub fn sync_selected_from_context(&mut self, selected: Option<T>) {
        self.selected = selected;
    }

    pub fn sync_selected_values_from_context(&mut self, selected_values: Vec<T>) {
        self.selected_values = unique_ordered_values(selected_values);
    }

    pub fn sync_input_value_from_context(&mut self, input_value: SharedString) {
        if self.input_value != input_value {
            self.input_value = input_value;
            self.inline_overlay = None;
            self.refilter();
        }
    }

    pub fn sync_open_from_context(&mut self, open: bool) {
        if self.open != open {
            self.commit_open(open);
        }
    }

    pub fn sync_groups(&mut self, mut groups: Vec<ComboboxGroupMetadata>) {
        groups.sort_by_key(ComboboxGroupMetadata::index);
        self.groups = groups;
    }

    /// Replaces walked item metadata, recomputes the filtered set, and
    /// preserves the highlighted value when it survives.
    pub fn sync_children(
        &mut self,
        mut items: Vec<ComboboxItemMetadata<T>>,
        input_focus_handle: Option<FocusHandle>,
        input_focused: bool,
    ) {
        let highlighted_value = self.highlighted_value();

        items.sort_by_key(ComboboxItemMetadata::index);
        self.items = items;
        self.input_focus_handle = input_focus_handle;
        self.refilter();
        self.highlighted_index = highlighted_value
            .as_ref()
            .and_then(|value| self.filtered_index_of_value(value));
        self.last_notified_highlight = self.highlighted_index;
        self.sync_focus(input_focused);
    }

    /// Reconciles selection for single mode; drops uncontrolled values whose
    /// item disappeared (falling back to the initial value when it survives).
    pub fn reconcile_single(&mut self, observed_selected: Option<T>, allow_fallback: bool) {
        self.selected = match allow_fallback {
            true => self.fallback_single_value(observed_selected),
            false => observed_selected,
        };
    }

    pub fn reconcile_multiple(&mut self, observed_values: Vec<T>, allow_prune: bool) {
        let mut values = unique_ordered_values(observed_values);
        if allow_prune && !self.items.is_empty() {
            values.retain(|value| self.items.iter().any(|item| &item.value == value));
        }
        self.selected_values = values;
        if let Some(chip) = self.highlighted_chip_index {
            if chip >= self.selected_values.len() {
                self.highlighted_chip_index = match self.selected_values.is_empty() {
                    true => None,
                    false => Some(self.selected_values.len() - 1),
                };
            }
        }
    }

    /// Reconciles the input value axis. In single mode with neither a default
    /// nor a controlled input value, the initial input value derives from the
    /// selected value's label.
    pub fn reconcile_input_value(&mut self, observed_input: Option<SharedString>) {
        if let Some(observed_input) = observed_input {
            if self.input_value != observed_input {
                self.input_value = observed_input;
                self.inline_overlay = None;
                self.refilter();
            }
        }
    }

    /// Derives the initial input value from the selected label in single mode
    /// (Base UI `initialDefaultInputValue`).
    pub fn derive_initial_input_from_selection(&mut self) {
        if self.selection_mode != ComboboxSelectionMode::Single {
            return;
        }
        if !self.input_value.is_empty() {
            return;
        }
        if let Some(label) = self
            .selected
            .clone()
            .and_then(|value| self.label_for_value(&value))
        {
            self.input_value = label.clone();
            self.initial_input_value = label;
            self.refilter();
        }
    }

    /// Syncs the input text to the selected label after a programmatic /
    /// controlled selected-value change (Base UI `useValueChanged` sync).
    /// Returns the new input value when a sync is needed.
    pub fn input_sync_for_selected(&self) -> Option<SharedString> {
        if self.selection_mode != ComboboxSelectionMode::Single {
            return None;
        }
        let label = self
            .selected
            .as_ref()
            .and_then(|value| self.label_for_value(value))
            .unwrap_or_default();
        (self.input_value != label).then_some(label)
    }

    // ------------------------------------------------------------------
    // Measurement
    // ------------------------------------------------------------------

    pub fn set_input_group_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.input_group_bounds == Some(bounds) {
            return false;
        }
        self.input_group_bounds = Some(bounds);
        true
    }

    pub fn set_input_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.input_bounds == Some(bounds) {
            return false;
        }
        self.input_bounds = Some(bounds);
        true
    }

    pub fn set_anchor_override_bounds(&mut self, bounds: Option<Bounds<Pixels>>) -> bool {
        if self.anchor_bounds == bounds {
            return false;
        }
        self.anchor_bounds = bounds;
        true
    }

    pub fn set_popup_bounds(&mut self, bounds: Bounds<Pixels>) -> bool {
        if self.popup_bounds == Some(bounds) {
            return false;
        }
        self.popup_bounds = Some(bounds);
        true
    }

    pub fn set_available_size(&mut self, size: Size<Pixels>) -> bool {
        if self.available_size == Some(size) {
            return false;
        }
        self.available_size = Some(size);
        true
    }

    /// The resolved anchor: an explicit override, else the input group when
    /// present, else the input (Base UI `resolvedAnchor`).
    pub fn resolved_anchor_bounds(&self) -> Option<Bounds<Pixels>> {
        self.anchor_bounds
            .or(self.input_group_bounds)
            .or(self.input_bounds)
    }

    // ------------------------------------------------------------------
    // Open
    // ------------------------------------------------------------------

    pub fn request_open_change(
        &self,
        current: bool,
        next: bool,
        disabled: bool,
        read_only: bool,
    ) -> ComboboxOpenChangeOutcome {
        if current == next {
            return ComboboxOpenChangeOutcome::new(false, current);
        }
        if next && (disabled || read_only) {
            return ComboboxOpenChangeOutcome::new(false, current);
        }
        ComboboxOpenChangeOutcome::new(true, next)
    }

    /// Commits an accepted open transition. Opening resets
    /// `query_changed_after_open` and refilters (enabling the single-mode
    /// browse bypass); closing freezes the close-query so the list does not
    /// flash unfiltered while closing.
    pub fn commit_open(&mut self, open: bool) {
        self.open = open;
        if open {
            self.query_changed_after_open = false;
            self.close_query = None;
            self.refilter();
            if self.auto_highlight == ComboboxAutoHighlight::Always {
                self.highlighted_index = self.first_filtered_enabled();
            }
        } else {
            self.close_query = Some(self.input_value.clone());
            self.highlighted_index = None;
            self.inline_overlay = None;
        }
    }

    /// Completes a close: resets the frozen query and browse flags. Mount /
    /// unmount is immediate in GPUI, so this runs right after `commit_open(false)`.
    pub fn complete_close(&mut self) {
        self.close_query = None;
        self.query_changed_after_open = false;
        self.refilter();
    }

    /// Input cleanup after close: single syncs the input to the selected
    /// label (or clears it), multiple/None clears the typed filter.
    pub fn input_cleanup_after_close(&self) -> Option<SharedString> {
        match self.selection_mode {
            ComboboxSelectionMode::Single => {
                let label = self
                    .selected
                    .as_ref()
                    .and_then(|value| self.label_for_value(value))
                    .unwrap_or_default();
                (self.input_value != label).then_some(label)
            }
            ComboboxSelectionMode::Multiple => {
                (!self.input_value.is_empty()).then(SharedString::default)
            }
            ComboboxSelectionMode::None => None,
        }
    }

    // ------------------------------------------------------------------
    // Input value / filtering
    // ------------------------------------------------------------------

    /// Commits a typed input value: refilters, marks the query as changed
    /// after open, applies auto-highlight, and clears the inline overlay.
    pub fn commit_input_value(&mut self, value: SharedString) {
        let changed = self.input_value != value;
        self.input_value = value;
        self.inline_overlay = None;
        if changed && self.open {
            self.query_changed_after_open = true;
        }
        self.refilter();
        if changed {
            match self.auto_highlight {
                ComboboxAutoHighlight::Off => self.highlighted_index = None,
                ComboboxAutoHighlight::OnInputChange | ComboboxAutoHighlight::Always => {
                    self.highlighted_index = self.first_filtered_enabled();
                }
            }
        }
    }

    /// Recomputes the filtered index set from registered metadata + query.
    /// Preserves the highlight when the highlighted item survives, clears it
    /// otherwise, and re-applies `Always` auto-highlight.
    pub fn refilter(&mut self) {
        let query = self.query();
        let bypass = self.should_bypass_filtering(&query);
        let mut indices = Vec::new();
        for item in &self.items {
            let keep = if self.filter_disabled || bypass || query.is_empty() {
                true
            } else if let Some(filter) = self.filter.as_ref() {
                filter(&item.value, self.resolved_label(item).as_ref(), &query)
            } else {
                default_filter(self.resolved_label(item).as_ref(), &query)
            };
            if keep {
                indices.push(item.index);
            }
        }
        if let Some(limit) = self.limit {
            indices.truncate(limit);
        }
        self.filtered_indices = indices;

        if let Some(highlighted) = self.highlighted_index {
            if !self.filtered_indices.contains(&highlighted) {
                self.highlighted_index = None;
            }
        }
        if self.highlighted_index.is_none()
            && self.open
            && self.auto_highlight == ComboboxAutoHighlight::Always
        {
            self.highlighted_index = self.first_filtered_enabled();
        }
    }

    /// Single-selection browse semantics: when the popup opened with a
    /// selected value and the query still exactly matches the selected label,
    /// show all items so the user can browse.
    fn should_bypass_filtering(&self, query: &str) -> bool {
        if self.selection_mode != ComboboxSelectionMode::Single {
            return false;
        }
        if self.query_changed_after_open {
            return false;
        }
        let Some(selected_label) = self
            .selected
            .as_ref()
            .and_then(|value| self.label_for_value(value))
        else {
            return false;
        };
        selected_label.trim().eq_ignore_ascii_case(query)
    }

    // ------------------------------------------------------------------
    // Inline overlay (Autocomplete seam)
    // ------------------------------------------------------------------

    /// Previews the highlighted item's label in the input as a whole-value
    /// overlay without touching the real input value.
    pub fn set_inline_overlay(&mut self, overlay: Option<SharedString>) {
        self.inline_overlay = overlay;
    }

    pub fn inline_overlay(&self) -> Option<SharedString> {
        self.inline_overlay.clone()
    }

    /// Syncs the inline overlay to the current highlight: the highlighted
    /// item's label when one is highlighted, else cleared (restoring the
    /// typed input text). Never touches the real input value or the query,
    /// so overlay changes are never treated as typing.
    pub fn sync_inline_overlay(&mut self) {
        self.inline_overlay = self
            .highlighted_value()
            .and_then(|value| self.label_for_value(&value));
    }

    // ------------------------------------------------------------------
    // Selection
    // ------------------------------------------------------------------

    /// One shared press-selection path for pointer and Enter.
    pub fn request_select_value(
        &self,
        current_single: Option<T>,
        current_multiple: Vec<T>,
        value: T,
        item_disabled: bool,
        root_disabled: bool,
        root_read_only: bool,
    ) -> ComboboxSelectionChange<T> {
        if item_disabled || root_disabled || root_read_only {
            return ComboboxSelectionChange::None;
        }

        match self.selection_mode {
            ComboboxSelectionMode::Single => {
                let changed = current_single.as_ref() != Some(&value);
                ComboboxSelectionChange::Single {
                    changed,
                    value: Some(value),
                }
            }
            ComboboxSelectionMode::Multiple => {
                let mut next_values = unique_ordered_values(current_multiple);
                if let Some(position) = next_values.iter().position(|selected| selected == &value) {
                    next_values.remove(position);
                } else {
                    next_values.push(value);
                }
                let changed = next_values != self.selected_values;
                ComboboxSelectionChange::Multiple {
                    changed,
                    values: next_values,
                }
            }
            ComboboxSelectionMode::None => ComboboxSelectionChange::None,
        }
    }

    pub fn commit_selected(&mut self, selected: Option<T>) {
        self.selected = selected;
    }

    /// Commits multiple selection. When a typed filter was active, clears it
    /// so the full list is visible again (Base UI `wasFiltering`). Returns the
    /// cleared input value when the filter was reset.
    pub fn commit_selected_values(&mut self, selected_values: Vec<T>) -> Option<SharedString> {
        self.selected_values = unique_ordered_values(selected_values);
        if self
            .highlighted_chip_index
            .is_some_and(|chip| chip >= self.selected_values.len())
        {
            self.highlighted_chip_index = None;
        }
        let was_filtering = !self.input_value.trim().is_empty();
        if was_filtering {
            self.input_value = SharedString::default();
            self.refilter();
            return Some(SharedString::default());
        }
        None
    }

    /// The label the input should show after selecting `value` in single mode
    /// with the input outside the popup.
    pub fn fill_input_value_for(&self, value: &T) -> SharedString {
        self.label_for_value(value).unwrap_or_default()
    }

    pub fn is_value_selected(&self, value: &T) -> bool {
        match self.selection_mode {
            ComboboxSelectionMode::Single => self.selected.as_ref() == Some(value),
            ComboboxSelectionMode::Multiple => self.selected_values.contains(value),
            ComboboxSelectionMode::None => false,
        }
    }

    // ------------------------------------------------------------------
    // Highlight / keyboard navigation
    // ------------------------------------------------------------------

    /// Moves the virtual highlight through the filtered items. With
    /// `loop_focus`, navigation loops through an "input position"
    /// (`highlight == None`) between last and first.
    pub fn move_highlight(&mut self, direction: ComboboxMove, loop_focus: bool) {
        let filtered = &self.filtered_indices;
        if filtered.is_empty() {
            self.highlighted_index = None;
            return;
        }

        let current_position = self
            .highlighted_index
            .and_then(|index| filtered.iter().position(|candidate| *candidate == index));

        self.highlighted_index = match direction {
            ComboboxMove::First => filtered.first().copied(),
            ComboboxMove::Last => filtered.last().copied(),
            ComboboxMove::Next => match current_position {
                None => filtered.first().copied(),
                Some(position) if position + 1 < filtered.len() => {
                    filtered.get(position + 1).copied()
                }
                Some(position) => match loop_focus {
                    true => None,
                    false => filtered.get(position).copied(),
                },
            },
            ComboboxMove::Previous => match current_position {
                None => filtered.last().copied(),
                Some(0) => match loop_focus {
                    true => None,
                    false => filtered.first().copied(),
                },
                Some(position) => filtered.get(position - 1).copied(),
            },
        };
    }

    pub fn highlight_item(&mut self, index: Option<usize>, disabled: bool) {
        if !disabled {
            self.highlighted_index = index;
        }
    }

    /// Clears the highlight when the pointer leaves the list, unless
    /// `keep_highlight`.
    pub fn clear_highlight_unless_kept(&mut self, keep_highlight: bool) {
        if !keep_highlight {
            self.highlighted_index = None;
        }
    }

    pub fn scroll_highlighted_into_view(&mut self) {
        let Some(highlighted) = self.highlighted_index else {
            return;
        };
        let Some(list_child_index) = self
            .item_at(highlighted)
            .and_then(ComboboxItemMetadata::list_child_index)
        else {
            return;
        };
        self.list_scroll_handle.scroll_to_item(list_child_index);
    }

    // ------------------------------------------------------------------
    // Chips
    // ------------------------------------------------------------------

    pub fn highlight_chip(&mut self, index: Option<usize>) {
        self.highlighted_chip_index = index.filter(|index| *index < self.selected_values.len());
    }

    /// Moves the chip highlight. From the input, Previous highlights the last
    /// chip; moving past either end returns to the input.
    pub fn move_chip_highlight(&mut self, direction: ComboboxMove) -> ComboboxChipMoveOutcome {
        if self.selected_values.is_empty() {
            self.highlighted_chip_index = None;
            return ComboboxChipMoveOutcome::NoChips;
        }
        let last = self.selected_values.len() - 1;
        let next = match (direction, self.highlighted_chip_index) {
            (ComboboxMove::Previous, None) => Some(last),
            (ComboboxMove::Previous, Some(0)) => None,
            (ComboboxMove::Previous, Some(index)) => Some(index - 1),
            (ComboboxMove::Next, None) => None,
            (ComboboxMove::Next, Some(index)) if index >= last => None,
            (ComboboxMove::Next, Some(index)) => Some(index + 1),
            (ComboboxMove::First, _) => Some(0),
            (ComboboxMove::Last, _) => Some(last),
        };
        self.highlighted_chip_index = next;
        match next {
            Some(index) => ComboboxChipMoveOutcome::Chip(index),
            None => ComboboxChipMoveOutcome::Input,
        }
    }

    /// Values after removing the chip at `position`; also reports where the
    /// chip highlight should land.
    pub fn request_remove_chip(&self, position: usize) -> Option<(Vec<T>, Option<usize>)> {
        if position >= self.selected_values.len() {
            return None;
        }
        let mut next_values = self.selected_values.clone();
        let removed = next_values.remove(position);
        let next_highlight = match next_values.is_empty() {
            true => None,
            false => Some(position.min(next_values.len() - 1)),
        };
        let _ = removed;
        Some((next_values, next_highlight))
    }

    /// Values after removing the last selected value (Backspace in an empty
    /// input with no chip highlighted).
    pub fn request_remove_last(&self) -> Option<Vec<T>> {
        if self.selected_values.is_empty() {
            return None;
        }
        let mut next_values = self.selected_values.clone();
        next_values.pop();
        Some(next_values)
    }

    /// Clears the list highlight when the removed value was highlighted.
    pub fn clear_highlight_for_removed(&mut self, removed: &T) {
        if self.highlighted_value().as_ref() == Some(removed) {
            self.highlighted_index = None;
        }
    }

    // ------------------------------------------------------------------
    // Clear
    // ------------------------------------------------------------------

    /// Whether the clear button has something to clear.
    pub fn clear_visible(&self) -> bool {
        match self.selection_mode {
            ComboboxSelectionMode::Single => {
                self.selected.is_some() || !self.input_value.is_empty()
            }
            ComboboxSelectionMode::Multiple => {
                !self.selected_values.is_empty() || !self.input_value.is_empty()
            }
            ComboboxSelectionMode::None => !self.input_value.is_empty(),
        }
    }

    /// Commits a full clear of input + selection + highlight.
    pub fn commit_clear(&mut self) {
        self.selected = None;
        self.selected_values = Vec::new();
        self.input_value = SharedString::default();
        self.inline_overlay = None;
        self.highlighted_index = None;
        self.highlighted_chip_index = None;
        self.refilter();
    }

    // ------------------------------------------------------------------
    // Style-state queries
    // ------------------------------------------------------------------

    pub fn root_state(&self, props: &ComboboxProps<T>) -> ComboboxRootStyleState<T> {
        ComboboxRootStyleState::new(
            props.disabled,
            props.read_only,
            props.required,
            self.open,
            self.focused(),
            self.dirty(),
            self.touched,
            None,
            self.selection_mode,
            self.selected.clone(),
            self.selected_values.clone(),
            self.input_value.clone(),
            self.list_empty(),
        )
    }

    pub fn input_state(
        &self,
        props: &ComboboxProps<T>,
        side: ComboboxSide,
    ) -> ComboboxInputStyleState<T> {
        ComboboxInputStyleState::new(self.root_state(props), side)
    }

    pub fn input_group_state(
        &self,
        props: &ComboboxProps<T>,
        side: ComboboxSide,
    ) -> ComboboxInputGroupStyleState<T> {
        ComboboxInputGroupStyleState::new(self.root_state(props), side)
    }

    pub fn trigger_state(
        &self,
        props: &ComboboxProps<T>,
        side: ComboboxSide,
    ) -> ComboboxTriggerStyleState<T> {
        ComboboxTriggerStyleState::new(self.root_state(props), side)
    }

    pub fn value_state(
        &self,
        props: &ComboboxProps<T>,
        placeholder: Option<SharedString>,
    ) -> ComboboxValueStyleState<T> {
        let selected_labels = self.selected_labels();
        let value_present = match self.selection_mode {
            ComboboxSelectionMode::Single => self.selected.is_some(),
            ComboboxSelectionMode::Multiple => !self.selected_values.is_empty(),
            ComboboxSelectionMode::None => !self.input_value.is_empty(),
        };
        let display_text = if !value_present {
            placeholder.unwrap_or_default()
        } else if self.selection_mode == ComboboxSelectionMode::Multiple {
            props
                .format_multiple_value(&selected_labels, &self.selected_values)
                .unwrap_or_else(|| join_labels(&selected_labels))
        } else if self.selection_mode == ComboboxSelectionMode::None {
            self.input_value.clone()
        } else {
            selected_labels.first().cloned().unwrap_or_default()
        };

        ComboboxValueStyleState::new(
            self.selection_mode,
            self.selected.clone(),
            self.selected_values.clone(),
            selected_labels,
            !value_present,
            display_text,
        )
    }

    pub fn icon_state(&self) -> ComboboxIconStyleState {
        ComboboxIconStyleState::new(self.open)
    }

    pub fn portal_state(&self, force_mounted: bool) -> ComboboxPortalStyleState {
        ComboboxPortalStyleState::new(self.open, self.open || force_mounted)
    }

    pub fn positioner_state(
        &self,
        side: ComboboxSide,
        align: ComboboxAlign,
    ) -> ComboboxPositionerStyleState {
        ComboboxPositionerStyleState::new(
            self.open,
            side,
            align,
            self.resolved_anchor_bounds(),
            self.popup_bounds,
            self.available_size,
            self.list_empty(),
        )
    }

    pub fn popup_state(
        &self,
        side: ComboboxSide,
        align: ComboboxAlign,
        force_mounted: bool,
    ) -> ComboboxPopupStyleState {
        ComboboxPopupStyleState::new(
            self.open,
            self.open || force_mounted,
            side,
            align,
            self.list_empty(),
        )
    }

    pub fn backdrop_state(&self, force_mounted: bool) -> ComboboxBackdropStyleState {
        ComboboxBackdropStyleState::new(self.open, self.open || force_mounted)
    }

    pub fn arrow_state(&self, side: ComboboxSide, align: ComboboxAlign) -> ComboboxArrowStyleState {
        ComboboxArrowStyleState::new(self.open, side, align, false)
    }

    pub fn list_state(&self) -> ComboboxListStyleState {
        ComboboxListStyleState::new(self.open, self.filtered_indices.len(), self.list_empty())
    }

    pub fn empty_state(&self) -> ComboboxEmptyStyleState {
        ComboboxEmptyStyleState::new(self.list_empty())
    }

    pub fn group_state(
        &self,
        group_index: Option<usize>,
        item_count: usize,
    ) -> ComboboxGroupStyleState {
        let label = group_index.and_then(|group_index| {
            self.groups
                .iter()
                .find(|group| group.index == group_index)
                .and_then(|group| group.label.clone())
        });
        ComboboxGroupStyleState::new(item_count, group_index, label)
    }

    pub fn item_state(
        &self,
        value: Option<&T>,
        disabled: bool,
        index: Option<usize>,
        props: &ComboboxProps<T>,
    ) -> ComboboxItemStyleState<T> {
        let selected = value
            .map(|value| self.is_value_selected(value))
            .unwrap_or(false);
        let highlighted = index.is_some() && index == self.highlighted_index;
        let visible = index.map(|index| self.item_visible(index)).unwrap_or(true);
        ComboboxItemStyleState::new(
            selected,
            highlighted,
            disabled,
            props.disabled,
            visible,
            index,
            value.cloned(),
        )
    }

    pub fn item_indicator_state(
        &self,
        item_state: &ComboboxItemStyleState<T>,
        keep_mounted: bool,
    ) -> ComboboxItemIndicatorStyleState {
        ComboboxItemIndicatorStyleState::new(
            item_state.selected,
            item_state.selected || keep_mounted,
        )
    }

    pub fn chips_state(&self) -> ComboboxChipsStyleState {
        ComboboxChipsStyleState::new(self.selected_values.len())
    }

    pub fn chip_state(&self, index: usize, props: &ComboboxProps<T>) -> ComboboxChipStyleState {
        ComboboxChipStyleState::new(
            self.highlighted_chip_index == Some(index),
            props.disabled,
            props.read_only,
            index,
        )
    }

    pub fn chip_remove_state(&self, props: &ComboboxProps<T>) -> ComboboxChipRemoveStyleState {
        ComboboxChipRemoveStyleState::new(props.disabled || props.read_only)
    }

    pub fn clear_state(&self, props: &ComboboxProps<T>) -> ComboboxClearStyleState {
        ComboboxClearStyleState::new(self.clear_visible(), props.disabled, self.open)
    }

    /// Label for a selected value at a chip position.
    pub fn chip_label(&self, position: usize) -> SharedString {
        self.selected_values
            .get(position)
            .and_then(|value| self.label_for_value(value))
            .unwrap_or_default()
    }

    pub fn label_for_value(&self, value: &T) -> Option<SharedString> {
        self.items
            .iter()
            .find(|item| &item.value == value)
            .and_then(|item| self.resolved_label(item))
            .or_else(|| self.label_resolver.as_ref().map(|resolver| resolver(value)))
    }

    // ------------------------------------------------------------------
    // Internals
    // ------------------------------------------------------------------

    fn sync_focus(&mut self, input_focused: bool) {
        self.input_focused = input_focused;
        let focused = self.focused();
        if self.was_focused && !focused {
            self.touched = true;
            if self.open {
                self.focus_out_close_requested = true;
            }
        }
        self.was_focused = focused;
    }

    fn focused(&self) -> bool {
        self.input_focused
    }

    fn dirty(&self) -> bool {
        match self.selection_mode {
            ComboboxSelectionMode::Single => self.selected != self.initial_selected,
            ComboboxSelectionMode::Multiple => self.selected_values != self.initial_selected_values,
            ComboboxSelectionMode::None => self.input_value != self.initial_input_value,
        }
    }

    fn selected_labels(&self) -> Vec<SharedString> {
        match self.selection_mode {
            ComboboxSelectionMode::Single | ComboboxSelectionMode::None => self
                .selected
                .as_ref()
                .and_then(|value| self.label_for_value(value))
                .into_iter()
                .collect(),
            ComboboxSelectionMode::Multiple => self
                .selected_values
                .iter()
                .filter_map(|value| self.label_for_value(value))
                .collect(),
        }
    }

    fn resolved_label(&self, item: &ComboboxItemMetadata<T>) -> Option<SharedString> {
        item.label.clone().or_else(|| {
            self.label_resolver
                .as_ref()
                .map(|resolver| resolver(&item.value))
        })
    }

    fn fallback_single_value(&self, observed_selected: Option<T>) -> Option<T> {
        if observed_selected
            .as_ref()
            .map(|value| self.items.iter().any(|item| &item.value == value))
            .unwrap_or(true)
        {
            return observed_selected;
        }
        if self
            .initial_selected
            .as_ref()
            .map(|value| self.items.iter().any(|item| &item.value == value))
            .unwrap_or(false)
        {
            return self.initial_selected.clone();
        }
        None
    }

    fn item_at(&self, index: usize) -> Option<&ComboboxItemMetadata<T>> {
        self.items.iter().find(|item| item.index == index)
    }

    fn filtered_index_of_value(&self, value: &T) -> Option<usize> {
        self.items
            .iter()
            .find(|item| &item.value == value)
            .map(|item| item.index)
            .filter(|index| self.filtered_indices.contains(index))
    }

    fn first_filtered_enabled(&self) -> Option<usize> {
        self.filtered_indices.iter().copied().find(|index| {
            self.item_at(*index)
                .map(|item| !item.disabled)
                .unwrap_or(false)
        })
    }
}

fn default_filter(label: Option<&SharedString>, query: &str) -> bool {
    let Some(label) = label else {
        return false;
    };
    label.to_lowercase().contains(&query.to_lowercase())
}

fn unique_ordered_values<T: Clone + Eq>(values: Vec<T>) -> Vec<T> {
    let mut unique = Vec::new();
    for value in values {
        if !unique.contains(&value) {
            unique.push(value);
        }
    }
    unique
}

fn join_labels(labels: &[SharedString]) -> SharedString {
    labels
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
        .into()
}
