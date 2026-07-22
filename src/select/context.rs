use std::{rc::Rc, time::Instant};

use gpui::{App, Bounds, ElementId, Entity, Pixels, Size, Window};

use crate::select::{
    SelectOpenChangeDetails, SelectOpenChangeReason, SelectOpenChangeSource, SelectProps,
    SelectRuntime, SelectScrollArrowDirection, SelectSelectionChange, SelectSelectionMode,
    SelectTypeaheadOutcome, SelectValueChangeDetails, SelectValueChangeReason,
    SelectValueChangeSource,
};

pub struct SelectContext<T: Clone + Eq + 'static> {
    id: ElementId,
    runtime: Entity<SelectRuntime<T>>,
    props: Rc<SelectProps<T>>,
    controlled_single: Rc<Option<Option<T>>>,
    controlled_multiple: Rc<Option<Vec<T>>>,
    controlled_open: Rc<Option<bool>>,
}

impl<T: Clone + Eq + 'static> Clone for SelectContext<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled_single: Rc::clone(&self.controlled_single),
            controlled_multiple: Rc::clone(&self.controlled_multiple),
            controlled_open: Rc::clone(&self.controlled_open),
        }
    }
}

impl<T: Clone + Eq + 'static> SelectContext<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        selection_mode: SelectSelectionMode,
        controlled_single: Option<Option<T>>,
        default_single: Option<T>,
        controlled_multiple: Option<Vec<T>>,
        default_multiple: Vec<T>,
        controlled_open: Option<bool>,
        default_open: bool,
        props: SelectProps<T>,
    ) -> Self {
        let id = id.into();
        let selected = controlled_single.clone().unwrap_or(default_single);
        let selected_values = controlled_multiple.clone().unwrap_or(default_multiple);
        let open = controlled_open.unwrap_or(default_open);
        let runtime = window.use_keyed_state(id.clone(), cx, |_, _| {
            SelectRuntime::new(selection_mode, selected, selected_values, open)
        });

        Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled_single: Rc::new(controlled_single),
            controlled_multiple: Rc::new(controlled_multiple),
            controlled_open: Rc::new(controlled_open),
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&SelectRuntime<T>, &SelectProps<T>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut SelectRuntime<T>) -> Output,
    ) -> Output {
        let controlled_single = self.controlled_single.as_ref().clone();
        let controlled_multiple = self.controlled_multiple.as_ref().clone();
        let controlled_open = *self.controlled_open.as_ref();
        let selection_mode = self.props.selection_mode();
        let value_comparator = self.props.value_comparator();

        self.runtime.update(cx, |runtime, cx| {
            runtime.sync_selection_mode(selection_mode);
            runtime.sync_value_comparator(value_comparator.clone());
            match selection_mode {
                SelectSelectionMode::Single => {
                    if let Some(selected) = controlled_single.clone() {
                        runtime.sync_selected_from_context(selected);
                    }
                }
                SelectSelectionMode::Multiple => {
                    if let Some(selected_values) = controlled_multiple.clone() {
                        runtime.sync_selected_values_from_context(selected_values);
                    }
                }
            }
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }

            let output = update(runtime);

            runtime.sync_value_comparator(value_comparator);
            match selection_mode {
                SelectSelectionMode::Single => {
                    if let Some(selected) = controlled_single {
                        runtime.sync_selected_from_context(selected);
                    }
                }
                SelectSelectionMode::Multiple => {
                    if let Some(selected_values) = controlled_multiple {
                        runtime.sync_selected_values_from_context(selected_values);
                    }
                }
            }
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }

            cx.notify();
            output
        })
    }

    pub fn record_available_size(&self, size: Size<Pixels>, cx: &mut App) {
        self.runtime.update(cx, |runtime, cx| {
            if runtime.set_available_size(size) {
                cx.notify();
            }
        });
    }

    pub fn record_value_bounds(&self, bounds: Bounds<Pixels>, cx: &mut App) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.set_value_bounds(bounds);
            if changed {
                cx.notify();
            }
            changed
        })
    }

    pub fn record_item_bounds(&self, index: usize, bounds: Bounds<Pixels>, cx: &mut App) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.set_item_bounds(index, bounds);
            if changed {
                cx.notify();
            }
            changed
        })
    }

    pub fn record_item_text_bounds(
        &self,
        index: usize,
        bounds: Bounds<Pixels>,
        cx: &mut App,
    ) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.set_item_text_bounds(index, bounds);
            if changed {
                cx.notify();
            }
            changed
        })
    }

    pub fn refresh_scroll_arrow_visibility(&self, cx: &mut App) -> bool {
        self.runtime.update(cx, |runtime, cx| {
            let changed = runtime.refresh_scroll_arrow_visibility();
            if changed {
                cx.notify();
            }
            changed
        })
    }

    pub fn scroll_toward(
        &self,
        direction: SelectScrollArrowDirection,
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.runtime.update(cx, |runtime, cx| {
            runtime.scroll_toward(direction);
            runtime.refresh_scroll_arrow_visibility();
            cx.notify();
        });
    }

    pub fn move_highlight(
        &self,
        direction: crate::select::SelectMove,
        loop_focus: bool,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.runtime.update(cx, |runtime, cx| {
            runtime.move_highlight(direction, loop_focus);
            runtime.scroll_highlighted_into_view();
            let focus_handle = runtime.highlighted_focus_handle();
            cx.notify();
            focus_handle
        });

        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }
    }

    pub fn notify_value_fallback(&self, value: Option<T>, window: &mut Window, cx: &mut App) {
        let Some(on_value_change) = self.props.on_value_change() else {
            return;
        };
        let mut details = SelectValueChangeDetails::new(
            SelectValueChangeReason::Fallback,
            SelectValueChangeSource::Programmatic,
            false,
        );
        on_value_change(value.as_ref(), &mut details, window, cx);
    }

    pub fn set_open(
        &self,
        next_open: bool,
        reason: SelectOpenChangeReason,
        source: SelectOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled_open = *self.controlled_open.as_ref();
        let props = Rc::clone(&self.props);
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            let current = controlled_open.unwrap_or_else(|| runtime.open_value());
            let outcome = runtime.request_open_change(
                current,
                next_open,
                props.disabled(),
                props.read_only(),
            );
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            outcome
        });

        if !outcome.changed() {
            return;
        }

        let mut details = SelectOpenChangeDetails::new(reason, source, true);
        if let Some(on_open_change) = self.props.on_open_change() {
            on_open_change(outcome.open(), &mut details, window, cx);
        }

        if details.is_canceled() {
            return;
        }

        self.runtime.update(cx, |runtime, cx| {
            if controlled_open.is_none() {
                runtime.commit_open(outcome.open(), source);
            } else {
                runtime.record_open_source(source);
            }
            cx.notify();
        });

        if outcome.open() {
            if let Some(focus_handle) =
                self.read(cx, |runtime, _| runtime.highlighted_focus_handle())
            {
                focus_handle.focus(window, cx);
            }
        } else if reason != SelectOpenChangeReason::FocusOut {
            if let Some(focus_handle) = self.read(cx, |runtime, _| runtime.trigger_focus_handle()) {
                focus_handle.focus(window, cx);
            }
        }
    }

    pub fn toggle_open(
        &self,
        reason: SelectOpenChangeReason,
        source: SelectOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let next_open = !self.read(cx, |runtime, _| runtime.open_value());
        self.set_open(next_open, reason, source, window, cx);
    }

    pub fn select_value(
        &self,
        value: T,
        item_disabled: bool,
        reason: SelectValueChangeReason,
        source: SelectValueChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let controlled_single = self.controlled_single.as_ref().clone();
        let controlled_multiple = self.controlled_multiple.as_ref().clone();
        let props = Rc::clone(&self.props);
        let selection_mode = props.selection_mode();
        let value_comparator = props.value_comparator();
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            runtime.sync_selection_mode(selection_mode);
            runtime.sync_value_comparator(value_comparator);
            let current_single = match controlled_single.as_ref() {
                Some(selected) => selected.clone(),
                None => runtime.selected_value(),
            };
            let current_multiple = match controlled_multiple.as_ref() {
                Some(values) => values.clone(),
                None => runtime.selected_values(),
            };

            match selection_mode {
                SelectSelectionMode::Single => {
                    if let Some(selected) = controlled_single.clone() {
                        runtime.sync_selected_from_context(selected);
                    }
                }
                SelectSelectionMode::Multiple => {
                    if let Some(values) = controlled_multiple.clone() {
                        runtime.sync_selected_values_from_context(values);
                    }
                }
            }

            let outcome = runtime.request_select_value(
                current_single,
                current_multiple,
                value,
                item_disabled,
                props.disabled(),
                props.read_only(),
            );

            match selection_mode {
                SelectSelectionMode::Single => {
                    if let Some(selected) = controlled_single.clone() {
                        runtime.sync_selected_from_context(selected);
                    }
                }
                SelectSelectionMode::Multiple => {
                    if let Some(values) = controlled_multiple.clone() {
                        runtime.sync_selected_values_from_context(values);
                    }
                }
            }

            outcome
        });

        if !outcome.changed() {
            return;
        }

        match outcome {
            SelectSelectionChange::None => {}
            SelectSelectionChange::Single(outcome) => {
                let mut details = SelectValueChangeDetails::new(reason, source, true);
                if let Some(on_value_change) = self.props.on_value_change() {
                    on_value_change(outcome.value(), &mut details, window, cx);
                }

                if details.is_canceled() {
                    return;
                }

                let next_value = outcome.into_value();
                if controlled_single.is_none() {
                    self.runtime.update(cx, |runtime, cx| {
                        runtime.commit_selected(next_value);
                        cx.notify();
                    });
                }

                self.set_open(
                    false,
                    SelectOpenChangeReason::ItemPress,
                    open_source_from_value_source(source),
                    window,
                    cx,
                );
            }
            SelectSelectionChange::Multiple(outcome) => {
                let mut details = SelectValueChangeDetails::new(reason, source, true);
                if let Some(on_values_change) = self.props.on_values_change() {
                    on_values_change(outcome.values(), &mut details, window, cx);
                }

                if details.is_canceled() {
                    return;
                }

                let next_values = outcome.into_values();
                if controlled_multiple.is_none() {
                    self.runtime.update(cx, |runtime, cx| {
                        runtime.commit_selected_values(next_values);
                        cx.notify();
                    });
                }
            }
        }
    }

    pub fn activate_highlighted(
        &self,
        source: SelectValueChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some((value, disabled)) =
            self.read(cx, |runtime, _| runtime.highlighted_value_with_disabled())
        else {
            return;
        };

        self.select_value(
            value,
            disabled,
            SelectValueChangeReason::ItemPress,
            source,
            window,
            cx,
        );
    }

    pub fn typeahead(&self, text: &str, window: &mut Window, cx: &mut App) -> bool {
        let outcome = self.update(cx, |runtime| runtime.apply_typeahead(text, Instant::now()));

        match outcome {
            SelectTypeaheadOutcome::Commit(value) => {
                self.select_value(
                    value,
                    false,
                    SelectValueChangeReason::Typeahead,
                    SelectValueChangeSource::Typeahead,
                    window,
                    cx,
                );
                true
            }
            SelectTypeaheadOutcome::Highlighted(_) => true,
            SelectTypeaheadOutcome::NoMatch | SelectTypeaheadOutcome::Disabled => false,
        }
    }

    pub fn focus_trigger(&self, window: &mut Window, cx: &mut App) {
        if let Some(focus_handle) = self.read(cx, |runtime, _| runtime.trigger_focus_handle()) {
            focus_handle.focus(window, cx);
        }
    }
}

fn open_source_from_value_source(source: SelectValueChangeSource) -> SelectOpenChangeSource {
    match source {
        SelectValueChangeSource::Pointer => SelectOpenChangeSource::Pointer,
        SelectValueChangeSource::Touch => SelectOpenChangeSource::Touch,
        SelectValueChangeSource::Keyboard => SelectOpenChangeSource::Keyboard,
        SelectValueChangeSource::Typeahead
        | SelectValueChangeSource::Programmatic
        | SelectValueChangeSource::None => SelectOpenChangeSource::Programmatic,
    }
}
