use std::rc::Rc;

use gpui::{App, Bounds, ElementId, Entity, Pixels, SharedString, Size, Window};

use crate::combobox::{
    ComboboxChangeDetails, ComboboxChangeReason, ComboboxChangeSource, ComboboxChipMoveOutcome,
    ComboboxHighlightReason, ComboboxItemHighlightDetails, ComboboxMove, ComboboxProps,
    ComboboxRuntime, ComboboxSelectionChange, ComboboxSelectionMode,
};

/// Thin mediation layer between layers and the runtime: resolves the three
/// controlled axes (selected value(s), input value, open) and fires props
/// callbacks from runtime outcomes. The controlled/uncontrolled rule lives
/// only here (and in what `ComboboxRoot` passes to `reconcile`).
pub struct ComboboxContext<T: Clone + Eq + 'static> {
    id: ElementId,
    runtime: Entity<ComboboxRuntime<T>>,
    props: Rc<ComboboxProps<T>>,
    controlled_single: Rc<Option<Option<T>>>,
    controlled_multiple: Rc<Option<Vec<T>>>,
    controlled_input: Rc<Option<SharedString>>,
    controlled_open: Rc<Option<bool>>,
}

impl<T: Clone + Eq + 'static> Clone for ComboboxContext<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled_single: Rc::clone(&self.controlled_single),
            controlled_multiple: Rc::clone(&self.controlled_multiple),
            controlled_input: Rc::clone(&self.controlled_input),
            controlled_open: Rc::clone(&self.controlled_open),
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxContext<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        selection_mode: ComboboxSelectionMode,
        controlled_single: Option<Option<T>>,
        default_single: Option<T>,
        controlled_multiple: Option<Vec<T>>,
        default_multiple: Vec<T>,
        controlled_input: Option<SharedString>,
        default_input: Option<SharedString>,
        controlled_open: Option<bool>,
        default_open: bool,
        props: ComboboxProps<T>,
    ) -> Self {
        let id = id.into();
        let selected = controlled_single.clone().unwrap_or(default_single);
        let selected_values = controlled_multiple.clone().unwrap_or(default_multiple);
        let input_value = controlled_input
            .clone()
            .or(default_input)
            .unwrap_or_default();
        let open = controlled_open.unwrap_or(default_open);
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            ComboboxRuntime::new(selection_mode, selected, selected_values, input_value, open)
        });

        Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled_single: Rc::new(controlled_single),
            controlled_multiple: Rc::new(controlled_multiple),
            controlled_input: Rc::new(controlled_input),
            controlled_open: Rc::new(controlled_open),
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn props(&self) -> &ComboboxProps<T> {
        self.props.as_ref()
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&ComboboxRuntime<T>, &ComboboxProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut ComboboxRuntime<T>) -> Output,
    ) -> Output {
        let props = Rc::clone(&self.props);
        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_selection_mode(props.selection_mode);
            runtime.sync_filtering(
                props.filter.clone(),
                props.filter_disabled,
                props.label_resolver.clone(),
                props.limit,
                props.auto_highlight,
            );
            let output = update(runtime);
            cx.notify();
            output
        })
    }

    // ------------------------------------------------------------------
    // Measurement recorders
    // ------------------------------------------------------------------

    pub fn record_available_size(&self, size: Size<Pixels>, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_available_size(size) {
                cx.notify();
            }
        });
    }

    pub fn record_input_group_bounds(&self, bounds: Bounds<Pixels>, cx: &mut App) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.set_input_group_bounds(bounds);
            if changed {
                cx.notify();
            }
            changed
        })
    }

    pub fn record_input_bounds(&self, bounds: Bounds<Pixels>, cx: &mut App) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.set_input_bounds(bounds);
            if changed {
                cx.notify();
            }
            changed
        })
    }

    // ------------------------------------------------------------------
    // Open
    // ------------------------------------------------------------------

    pub fn set_open(
        &self,
        next_open: bool,
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled_open = *self.controlled_open.as_ref();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            let current = controlled_open.unwrap_or_else(|| runtime.open_value());
            runtime.request_open_change(current, next_open, props.disabled, props.read_only)
        });

        if !outcome.changed() {
            return;
        }

        let mut details = ComboboxChangeDetails::new(reason, source, true);
        if let Some(on_open_change) = self.props.on_open_change.as_ref() {
            on_open_change(outcome.open(), &mut details, window, cx);
        }
        if details.is_canceled() {
            return;
        }

        if controlled_open.is_none() {
            self.runtime.update(cx, |runtime, cx| {
                runtime.commit_open(outcome.open());
                if !outcome.open() {
                    runtime.complete_close();
                }
                cx.notify();
            });
        }

        if !outcome.open() {
            // Input cleanup after close: single syncs to the selected label,
            // multiple clears the typed filter.
            let cleanup = self.runtime.read(cx).input_cleanup_after_close();
            if let Some(cleanup) = cleanup {
                self.set_input_value(cleanup, ComboboxChangeReason::None, source, window, cx);
            }
        }

        self.notify_highlight(ComboboxHighlightReason::None, window, cx);
    }

    pub fn toggle_open(
        &self,
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let next_open = !self.read(cx, |runtime, _| runtime.open_value());
        self.set_open(next_open, reason, source, window, cx);
    }

    // ------------------------------------------------------------------
    // Input value
    // ------------------------------------------------------------------

    pub fn set_input_value(
        &self,
        value: SharedString,
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled_input = self.controlled_input.as_ref().clone();
        let current = controlled_input
            .clone()
            .unwrap_or_else(|| self.runtime.read(cx).input_value());
        if current == value {
            return;
        }

        let mut details = ComboboxChangeDetails::new(reason, source, true);
        if let Some(on_input_value_change) = self.props.on_input_value_change.as_ref() {
            on_input_value_change(&value, &mut details, window, cx);
        }
        if details.is_canceled() {
            return;
        }

        if controlled_input.is_none() {
            self.update(cx, |runtime| runtime.commit_input_value(value.clone()));
        }

        // Clearing the input to empty in single mode clears the selection.
        if reason == ComboboxChangeReason::InputChange
            && value.trim().is_empty()
            && self.props.selection_mode == ComboboxSelectionMode::Single
        {
            let current_single = self
                .controlled_single
                .as_ref()
                .clone()
                .unwrap_or_else(|| self.runtime.read(cx).selected_value());
            if current_single.is_some() {
                let mut clear_details =
                    ComboboxChangeDetails::new(ComboboxChangeReason::InputClear, source, true);
                if let Some(on_value_change) = self.props.on_value_change.as_ref() {
                    on_value_change(None, &mut clear_details, window, cx);
                }
                if !clear_details.is_canceled() && self.controlled_single.is_none() {
                    self.update(cx, |runtime| runtime.commit_selected(None));
                }
            }
        }

        self.notify_highlight(ComboboxHighlightReason::None, window, cx);
    }

    /// Typing path from the input primitive: updates the input value and
    /// opens the popup on a non-empty typed query.
    pub fn input_typed(&self, value: SharedString, window: &mut Window, cx: &mut App) {
        if self.props.disabled || self.props.read_only {
            return;
        }
        self.update(cx, |runtime| runtime.highlight_chip(None));
        self.set_input_value(
            value.clone(),
            ComboboxChangeReason::InputChange,
            ComboboxChangeSource::Keyboard,
            window,
            cx,
        );
        if !value.trim().is_empty() && !self.read(cx, |runtime, _| runtime.open_value()) {
            self.set_open(
                true,
                ComboboxChangeReason::InputChange,
                ComboboxChangeSource::Keyboard,
                window,
                cx,
            );
        }
    }

    // ------------------------------------------------------------------
    // Selection
    // ------------------------------------------------------------------

    /// Shared press-selection path for pointer and Enter so callbacks and
    /// reasons stay consistent.
    pub fn select_item(
        &self,
        value: T,
        item_disabled: bool,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled_single = self.controlled_single.as_ref().clone();
        let controlled_multiple = self.controlled_multiple.as_ref().clone();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            runtime.sync_selection_mode(props.selection_mode);
            let current_single = controlled_single
                .clone()
                .unwrap_or_else(|| runtime.selected_value());
            let current_multiple = controlled_multiple
                .clone()
                .unwrap_or_else(|| runtime.selected_values());
            runtime.request_select_value(
                current_single,
                current_multiple,
                value.clone(),
                item_disabled,
                props.disabled,
                props.read_only,
            )
        });

        match outcome {
            ComboboxSelectionChange::None => {
                // None mode: pressing an item optionally fills the input.
                if self.props.selection_mode == ComboboxSelectionMode::None {
                    if item_disabled || self.props.disabled || self.props.read_only {
                        return;
                    }
                    if self.props.fill_input_on_item_press {
                        let fill = self.read(cx, |runtime, _| runtime.fill_input_value_for(&value));
                        self.set_input_value(
                            fill,
                            ComboboxChangeReason::ItemPress,
                            source,
                            window,
                            cx,
                        );
                    }
                    self.set_open(false, ComboboxChangeReason::ItemPress, source, window, cx);
                }
            }
            ComboboxSelectionChange::Single {
                changed,
                value: next_value,
            } => {
                if changed {
                    let mut details =
                        ComboboxChangeDetails::new(ComboboxChangeReason::ItemPress, source, true);
                    if let Some(on_value_change) = self.props.on_value_change.as_ref() {
                        on_value_change(next_value.as_ref(), &mut details, window, cx);
                    }
                    if details.is_canceled() {
                        return;
                    }
                    if controlled_single.is_none() {
                        self.update(cx, |runtime| runtime.commit_selected(next_value.clone()));
                    }
                }

                // Fill the input with the selected label (input outside popup).
                let fill = self.read(cx, |runtime, _| runtime.fill_input_value_for(&value));
                self.set_input_value(fill, ComboboxChangeReason::None, source, window, cx);
                self.set_open(false, ComboboxChangeReason::ItemPress, source, window, cx);
            }
            ComboboxSelectionChange::Multiple { changed, values } => {
                if !changed {
                    return;
                }
                let was_filtering = !self
                    .read(cx, |runtime, _| runtime.input_value())
                    .trim()
                    .is_empty();
                let mut details =
                    ComboboxChangeDetails::new(ComboboxChangeReason::ItemPress, source, true);
                if let Some(on_values_change) = self.props.on_values_change.as_ref() {
                    on_values_change(&values, &mut details, window, cx);
                }
                if details.is_canceled() {
                    return;
                }
                if controlled_multiple.is_none() {
                    self.update(cx, |runtime| {
                        runtime.commit_selected_values(values);
                    });
                }
                // Selecting while a typed filter was active clears the filter
                // so the full list is visible again. The popup stays open.
                if was_filtering {
                    self.set_input_value(
                        SharedString::default(),
                        ComboboxChangeReason::None,
                        source,
                        window,
                        cx,
                    );
                }
                self.notify_highlight(ComboboxHighlightReason::None, window, cx);
            }
        }
    }

    /// Enter with a highlighted item selects it; Enter with no highlight
    /// closes the popup.
    pub fn activate_highlighted(
        &self,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        match self.read(cx, |runtime, _| runtime.highlighted_value_with_disabled()) {
            Some((value, disabled)) => self.select_item(value, disabled, source, window, cx),
            None => {
                if self.read(cx, |runtime, _| runtime.open_value()) {
                    self.set_open(false, ComboboxChangeReason::None, source, window, cx);
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Highlight
    // ------------------------------------------------------------------

    pub fn move_highlight(&self, direction: ComboboxMove, window: &mut Window, cx: &mut App) {
        let loop_focus = self.props.loop_focus;
        self.update(cx, |runtime| {
            runtime.move_highlight(direction, loop_focus);
            runtime.scroll_highlighted_into_view();
        });
        self.notify_highlight(ComboboxHighlightReason::Keyboard, window, cx);
    }

    /// ArrowDown/ArrowUp on the input: opens a closed popup with the
    /// list-navigation reason, moves the highlight when open.
    pub fn navigate_list(&self, direction: ComboboxMove, window: &mut Window, cx: &mut App) {
        if self.props.disabled || self.props.read_only {
            return;
        }
        if self.read(cx, |runtime, _| runtime.open_value()) {
            self.move_highlight(direction, window, cx);
        } else {
            self.set_open(
                true,
                ComboboxChangeReason::ListNavigation,
                ComboboxChangeSource::Keyboard,
                window,
                cx,
            );
            self.notify_highlight(ComboboxHighlightReason::Keyboard, window, cx);
        }
    }

    pub fn highlight_item_from_pointer(
        &self,
        index: Option<usize>,
        disabled: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        if !self.props.highlight_item_on_hover {
            return;
        }
        self.update(cx, |runtime| runtime.highlight_item(index, disabled));
        self.notify_highlight(ComboboxHighlightReason::Pointer, window, cx);
    }

    pub fn pointer_left_list(&self, window: &mut Window, cx: &mut App) {
        let keep_highlight = self.props.keep_highlight;
        self.update(cx, |runtime| {
            runtime.clear_highlight_unless_kept(keep_highlight)
        });
        self.notify_highlight(ComboboxHighlightReason::None, window, cx);
    }

    // ------------------------------------------------------------------
    // Chips
    // ------------------------------------------------------------------

    pub fn move_chip_highlight(
        &self,
        direction: ComboboxMove,
        cx: &mut App,
    ) -> ComboboxChipMoveOutcome {
        self.update(cx, |runtime| runtime.move_chip_highlight(direction))
    }

    pub fn remove_chip(
        &self,
        position: usize,
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some((next_values, next_chip_highlight, removed)) = self.read(cx, |runtime, _| {
            runtime
                .request_remove_chip(position)
                .map(|(values, highlight)| {
                    (
                        values,
                        highlight,
                        runtime.selected_values().get(position).cloned(),
                    )
                })
        }) else {
            return;
        };

        let mut details = ComboboxChangeDetails::new(reason, source, true);
        if let Some(on_values_change) = self.props.on_values_change.as_ref() {
            on_values_change(&next_values, &mut details, window, cx);
        }
        if details.is_canceled() {
            return;
        }

        self.update(cx, |runtime| {
            if let Some(removed) = removed.as_ref() {
                runtime.clear_highlight_for_removed(removed);
            }
            runtime.highlight_chip(next_chip_highlight);
        });
        if self.controlled_multiple.is_none() {
            self.update(cx, |runtime| {
                runtime.commit_selected_values(next_values);
            });
        }
        self.focus_input(window, cx);
        self.notify_highlight(ComboboxHighlightReason::None, window, cx);
    }

    /// Backspace in an empty input with no chip highlighted removes the last
    /// selected value.
    pub fn remove_last_value(&self, window: &mut Window, cx: &mut App) {
        if self.props.disabled || self.props.read_only {
            return;
        }
        let chip_count = self.read(cx, |runtime, _| runtime.selected_values().len());
        if chip_count == 0 {
            return;
        }
        self.remove_chip(
            chip_count - 1,
            ComboboxChangeReason::ChipRemovePress,
            ComboboxChangeSource::Keyboard,
            window,
            cx,
        );
    }

    /// Backspace/Delete on a highlighted chip removes that value.
    pub fn remove_highlighted_chip(&self, window: &mut Window, cx: &mut App) -> bool {
        let Some(position) = self.read(cx, |runtime, _| runtime.highlighted_chip_index()) else {
            return false;
        };
        self.remove_chip(
            position,
            ComboboxChangeReason::ChipRemovePress,
            ComboboxChangeSource::Keyboard,
            window,
            cx,
        );
        true
    }

    // ------------------------------------------------------------------
    // Clear / Escape
    // ------------------------------------------------------------------

    /// Clear-button press: clears input + selection + highlight and refocuses
    /// the input without opening the popup.
    pub fn clear_all(
        &self,
        reason: ComboboxChangeReason,
        source: ComboboxChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.props.disabled || self.props.read_only {
            return;
        }

        match self.props.selection_mode {
            ComboboxSelectionMode::Single => {
                let current = self
                    .controlled_single
                    .as_ref()
                    .clone()
                    .unwrap_or_else(|| self.runtime.read(cx).selected_value());
                if current.is_some() {
                    let mut details = ComboboxChangeDetails::new(reason, source, true);
                    if let Some(on_value_change) = self.props.on_value_change.as_ref() {
                        on_value_change(None, &mut details, window, cx);
                    }
                    if details.is_canceled() {
                        return;
                    }
                }
            }
            ComboboxSelectionMode::Multiple => {
                let current = self
                    .controlled_multiple
                    .as_ref()
                    .clone()
                    .unwrap_or_else(|| self.runtime.read(cx).selected_values());
                if !current.is_empty() {
                    let mut details = ComboboxChangeDetails::new(reason, source, true);
                    if let Some(on_values_change) = self.props.on_values_change.as_ref() {
                        on_values_change(&[], &mut details, window, cx);
                    }
                    if details.is_canceled() {
                        return;
                    }
                }
            }
            ComboboxSelectionMode::None => {}
        }

        self.set_input_value(SharedString::default(), reason, source, window, cx);
        if self.controlled_single.is_none() && self.controlled_multiple.is_none() {
            self.update(cx, |runtime| runtime.commit_clear());
        }
        self.focus_input(window, cx);
        self.notify_highlight(ComboboxHighlightReason::None, window, cx);
    }

    /// Escape while open closes; Escape while closed with a value or a
    /// non-empty input clears both.
    pub fn escape_pressed(&self, window: &mut Window, cx: &mut App) {
        if self.read(cx, |runtime, _| runtime.open_value()) {
            self.set_open(
                false,
                ComboboxChangeReason::EscapeKey,
                ComboboxChangeSource::Keyboard,
                window,
                cx,
            );
            self.focus_input(window, cx);
        } else if self.read(cx, |runtime, _| runtime.clear_visible()) {
            self.clear_all(
                ComboboxChangeReason::EscapeKey,
                ComboboxChangeSource::Keyboard,
                window,
                cx,
            );
        }
    }

    // ------------------------------------------------------------------
    // Focus / callbacks
    // ------------------------------------------------------------------

    pub fn focus_input(&self, window: &mut Window, cx: &mut App) {
        if let Some(focus_handle) = self.read(cx, |runtime, _| runtime.input_focus_handle()) {
            focus_handle.focus(window, cx);
        }
    }

    /// Fires `on_item_highlighted` exactly once per highlight transition.
    /// With `inline_autocomplete`, keyboard/programmatic highlight changes
    /// also sync the inline overlay; pointer highlights never touch the
    /// displayed input text.
    pub fn notify_highlight(
        &self,
        reason: ComboboxHighlightReason,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.props.inline_autocomplete && reason != ComboboxHighlightReason::Pointer {
            self.runtime.update(cx, |runtime, cx| {
                runtime.sync_inline_overlay();
                cx.notify();
            });
        }
        let Some(transition) = self
            .runtime
            .update(cx, |runtime, _| runtime.take_highlight_transition())
        else {
            return;
        };
        let Some(on_item_highlighted) = self.props.on_item_highlighted.as_ref() else {
            return;
        };
        let value = self.runtime.read(cx).highlighted_value();
        let details = ComboboxItemHighlightDetails {
            reason,
            index: transition,
        };
        on_item_highlighted(value.as_ref(), &details, window, cx);
    }
}
