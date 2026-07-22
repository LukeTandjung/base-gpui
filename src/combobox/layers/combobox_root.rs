use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

type ComboboxRootStyle<T> = Rc<dyn Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static>;

use crate::{
    combobox::{
        child_wiring::wire_children, ComboboxAutoHighlight, ComboboxChangeDetails,
        ComboboxChangeReason, ComboboxChangeSource, ComboboxChild, ComboboxContext,
        ComboboxItemHighlightDetails, ComboboxProps, ComboboxRootStyleState, ComboboxSelectionMode,
    },
    field::{
        current_field_context, current_field_item_disabled, FieldControlRegistration, FieldValue,
    },
    fieldset::current_fieldset_disabled,
};

/// The Combobox root: the single non-event mutation site. Its render wires
/// children, calls `sync_children`, reconciles all three controlled axes
/// (selected value(s), input value, open), and registers with Field.
///
/// Builder parity with Base UI `ComboboxRoot`: the Autocomplete-only knobs
/// (`selection_mode = None`, `fill_input_on_item_press`, `keep_highlight`,
/// `submit_on_item_click`) are hidden here but stay public on
/// `ComboboxProps` / `ComboboxRuntime` for the Autocomplete port.
///
/// When both single and multiple value props are supplied, the active
/// `.multiple(bool)` mode decides deterministically which axis is used; the
/// other is ignored.
#[derive(IntoElement)]
pub struct ComboboxRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<ComboboxChild<T>>,
    props: ComboboxProps<T>,
    multiple: bool,
    default_value: Option<T>,
    value: Option<Option<T>>,
    default_values: Vec<T>,
    values: Option<Vec<T>>,
    default_input_value: Option<SharedString>,
    input_value: Option<SharedString>,
    default_open: bool,
    open: Option<bool>,
    style_with_state: Option<ComboboxRootStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox"),
            base: div(),
            children: Vec::new(),
            props: ComboboxProps::default(),
            multiple: false,
            default_value: None,
            value: None,
            default_values: Vec::new(),
            values: None,
            default_input_value: None,
            input_value: None,
            default_open: false,
            open: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let field_valid = field_state.and_then(|state| state.valid);

        let mut props = self.props;
        props.disabled = props.disabled
            || field_disabled
            || current_field_item_disabled()
            || current_fieldset_disabled();
        props.selection_mode = match self.multiple {
            true => ComboboxSelectionMode::Multiple,
            false => props.selection_mode,
        };
        let selection_mode = props.selection_mode;
        let id = self.id.clone();
        let name = props.name.clone();
        let value_serializer = props.value_serializer.clone();
        let required = props.required;

        let controlled_single = self.value.clone();
        let controlled_multiple = self.values.clone();
        let controlled_input = self.input_value.clone();
        let controlled_open = self.open;
        let single_controlled = controlled_single.is_some();
        let multiple_controlled = controlled_multiple.is_some();

        let context = ComboboxContext::new(
            self.id.clone(),
            cx,
            window,
            selection_mode,
            controlled_single.clone(),
            self.default_value,
            controlled_multiple.clone(),
            self.default_values,
            controlled_input.clone(),
            self.default_input_value,
            controlled_open,
            self.default_open,
            props,
        );

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let items = wired_children.items;
        let groups = wired_children.groups;
        let input_focus_handle = wired_children.input_focus_handle;
        let input_focused = wired_children.input_focused;
        let children = wired_children.children;

        let (close_for_focus_out, input_sync) = context.update(cx, |runtime| {
            runtime.sync_groups(groups);
            runtime.sync_children(items, input_focus_handle.clone(), input_focused);

            match selection_mode {
                ComboboxSelectionMode::Single | ComboboxSelectionMode::None => {
                    let observed_selected = controlled_single
                        .clone()
                        .unwrap_or_else(|| runtime.selected_value());
                    runtime.reconcile_single(observed_selected, !single_controlled);
                }
                ComboboxSelectionMode::Multiple => {
                    let observed_values = controlled_multiple
                        .clone()
                        .unwrap_or_else(|| runtime.selected_values());
                    runtime.reconcile_multiple(observed_values, !multiple_controlled);
                }
            }
            runtime.reconcile_input_value(controlled_input.clone());
            if controlled_input.is_none() {
                runtime.derive_initial_input_from_selection();
            }
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            runtime.refilter();

            // Controlled/programmatic selected-value changes sync the input
            // text to the new label in single mode (input outside the popup).
            let input_sync = match single_controlled && !runtime.open_value() {
                true => runtime.input_sync_for_selected(),
                false => None,
            };

            (runtime.take_focus_out_close_request(), input_sync)
        });
        if let Some(input_sync) = input_sync {
            context.set_input_value(
                input_sync,
                ComboboxChangeReason::None,
                ComboboxChangeSource::Programmatic,
                window,
                cx,
            );
        }
        if close_for_focus_out {
            context.set_open(
                false,
                ComboboxChangeReason::FocusOut,
                ComboboxChangeSource::None,
                window,
                cx,
            );
        }

        let mut style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        if let Some(field_valid) = field_valid {
            style_state.valid = Some(field_valid);
            style_state.invalid = !field_valid;
        }

        if let Some(field_context) = field_context.as_ref() {
            let field_value = combobox_field_value(
                selection_mode,
                style_state.selected_value.as_ref(),
                &style_state.selected_values,
                &style_state.input_value,
                value_serializer.as_ref(),
            );
            let value_missing = required && !field_value.filled();
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(field_value)
                .disabled(style_state.disabled)
                .focused(style_state.focused)
                .required(required)
                .value_missing(value_missing);
            if let Some(focus_handle) = context.read(cx, |runtime, _| runtime.input_focus_handle())
            {
                registration = registration.focus_handle(focus_handle);
            }
            if let Some(name) = name {
                registration = registration.name(name);
            }
            field_context.register_control(registration, cx);
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(children)
    }
}

impl<T: Clone + Eq + 'static> ComboboxRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<ComboboxChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.props.name = Some(name.into());
        self
    }

    pub fn default_value(mut self, default_value: Option<T>) -> Self {
        self.default_value = default_value;
        self
    }

    pub fn value(mut self, value: Option<T>) -> Self {
        self.value = Some(value);
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Option<&T>, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.props.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn default_values(mut self, default_values: Vec<T>) -> Self {
        self.default_values = default_values;
        self
    }

    pub fn values(mut self, values: Vec<T>) -> Self {
        self.values = Some(values);
        self
    }

    pub fn on_values_change(
        mut self,
        on_values_change: impl Fn(&[T], &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.props.on_values_change = Some(Rc::new(on_values_change));
        self
    }

    pub fn default_input_value(mut self, default_input_value: impl Into<SharedString>) -> Self {
        self.default_input_value = Some(default_input_value.into());
        self
    }

    pub fn input_value(mut self, input_value: impl Into<SharedString>) -> Self {
        self.input_value = Some(input_value.into());
        self
    }

    pub fn on_input_value_change(
        mut self,
        on_input_value_change: impl Fn(&SharedString, &mut ComboboxChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.props.on_input_value_change = Some(Rc::new(on_input_value_change));
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut ComboboxChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.props.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_item_highlighted(
        mut self,
        on_item_highlighted: impl Fn(Option<&T>, &ComboboxItemHighlightDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.props.on_item_highlighted = Some(Rc::new(on_item_highlighted));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.props.read_only = read_only;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    pub fn open_on_input_click(mut self, open_on_input_click: bool) -> Self {
        self.props.open_on_input_click = open_on_input_click;
        self
    }

    pub fn auto_highlight(mut self, auto_highlight: ComboboxAutoHighlight) -> Self {
        self.props.auto_highlight = auto_highlight;
        self
    }

    pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self {
        self.props.highlight_item_on_hover = highlight_item_on_hover;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.props.loop_focus = loop_focus;
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.props.limit = Some(limit);
        self
    }

    /// Custom filter replacing the default case-insensitive contains match.
    pub fn filter(
        mut self,
        filter: impl Fn(&T, Option<&SharedString>, &str) -> bool + 'static,
    ) -> Self {
        self.props.filter = Some(Rc::new(filter));
        self
    }

    /// Disables internal filtering for externally filtered lists.
    pub fn filter_none(mut self) -> Self {
        self.props.filter_disabled = true;
        self
    }

    pub fn item_to_string_label(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self {
        self.props.label_resolver = Some(Rc::new(resolver));
        self
    }

    pub fn item_to_string_value(
        mut self,
        serializer: impl Fn(&T) -> SharedString + 'static,
    ) -> Self {
        self.props.value_serializer = Some(Rc::new(serializer));
        self
    }

    pub fn multiple_value_formatter(
        mut self,
        formatter: impl Fn(&[SharedString], &[T]) -> SharedString + 'static,
    ) -> Self {
        self.props.multiple_value_formatter = Some(Rc::new(formatter));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxRootStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn combobox_field_value<T: Clone + Eq + 'static>(
    selection_mode: ComboboxSelectionMode,
    selected_value: Option<&T>,
    selected_values: &[T],
    input_value: &SharedString,
    serializer: Option<&crate::combobox::ComboboxValueSerializer<T>>,
) -> FieldValue {
    match selection_mode {
        ComboboxSelectionMode::Single => selected_value
            .map(|value| {
                serializer
                    .map(|serializer| FieldValue::Text(serializer(value)))
                    .unwrap_or(FieldValue::Present)
            })
            .unwrap_or(FieldValue::Empty),
        ComboboxSelectionMode::Multiple => {
            if selected_values.is_empty() {
                FieldValue::List(Vec::new())
            } else if let Some(serializer) = serializer {
                FieldValue::List(
                    selected_values
                        .iter()
                        .map(|value| serializer(value))
                        .collect(),
                )
            } else {
                FieldValue::Present
            }
        }
        // None mode (Autocomplete port): the input value is the field value.
        ComboboxSelectionMode::None => match input_value.is_empty() {
            true => FieldValue::Empty,
            false => FieldValue::Text(input_value.clone()),
        },
    }
}
