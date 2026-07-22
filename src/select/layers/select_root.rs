use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window,
};

type SelectRootStyle<T> = Rc<dyn Fn(SelectRootStyleState<T>, Div) -> Div + 'static>;

use crate::{
    field::{
        current_field_context, current_field_item_disabled, FieldControlRegistration, FieldValue,
    },
    fieldset::current_fieldset_disabled,
    select::{
        child_wiring::wire_children, SelectChild, SelectContext, SelectLabelResolver,
        SelectMultipleValueFormatter, SelectOpenChangeDetails, SelectOpenChangeHandler,
        SelectProps, SelectRootStyleState, SelectSelectionMode, SelectValueChangeDetails,
        SelectValueChangeHandler, SelectValueComparator, SelectValueSerializer,
        SelectValuesChangeHandler,
    },
};

#[derive(IntoElement)]
pub struct SelectRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<SelectChild<T>>,
    name: Option<SharedString>,
    form: Option<SharedString>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    default_values: Vec<T>,
    values: Option<Vec<T>>,
    default_open: bool,
    open: Option<bool>,
    multiple: bool,
    disabled: bool,
    read_only: bool,
    required: bool,
    modal: bool,
    highlight_item_on_hover: bool,
    label_resolver: Option<SelectLabelResolver<T>>,
    value_serializer: Option<SelectValueSerializer<T>>,
    value_comparator: Option<SelectValueComparator<T>>,
    multiple_value_formatter: Option<SelectMultipleValueFormatter<T>>,
    on_value_change: Option<SelectValueChangeHandler<T>>,
    on_values_change: Option<SelectValuesChangeHandler<T>>,
    on_open_change: Option<SelectOpenChangeHandler>,
    style_with_state: Option<SelectRootStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for SelectRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("select"),
            base: div(),
            children: Vec::new(),
            name: None,
            form: None,
            default_value: None,
            value: None,
            default_values: Vec::new(),
            values: None,
            default_open: false,
            open: None,
            multiple: false,
            disabled: false,
            read_only: false,
            required: false,
            modal: false,
            highlight_item_on_hover: true,
            label_resolver: None,
            value_serializer: None,
            value_comparator: None,
            multiple_value_formatter: None,
            on_value_change: None,
            on_values_change: None,
            on_open_change: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)));
        let field_disabled = field_state.map(|state| state.disabled).unwrap_or(false);
        let field_valid = field_state.and_then(|state| state.valid);
        let disabled = self.disabled
            || field_disabled
            || current_field_item_disabled()
            || current_fieldset_disabled();
        let id = self.id.clone();
        let name = self.name.clone();
        let selection_mode = match self.multiple {
            true => SelectSelectionMode::Multiple,
            false => SelectSelectionMode::Single,
        };
        let controlled_single = self.value.clone();
        let controlled_multiple = self.values.clone();
        let controlled_open = self.open;
        let single_controlled = controlled_single.is_some();
        let multiple_controlled = controlled_multiple.is_some();
        let value_serializer = self.value_serializer.clone();

        let context = SelectContext::new(
            self.id.clone(),
            cx,
            window,
            selection_mode,
            self.value,
            self.default_value,
            self.values,
            self.default_values,
            self.open,
            self.default_open,
            SelectProps::new(
                self.name,
                self.form,
                disabled,
                self.read_only,
                self.required,
                self.modal,
                selection_mode,
                self.highlight_item_on_hover,
                self.label_resolver,
                self.value_serializer,
                self.value_comparator,
                self.multiple_value_formatter,
                self.on_value_change,
                self.on_values_change,
                self.on_open_change,
            ),
        );

        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let items = wired_children.items;
        let groups = wired_children.groups;
        let item_focus_handles = wired_children.item_focus_handles;
        let trigger_focus_handle = wired_children.trigger_focus_handle;
        let trigger_focused = wired_children.trigger_focused;
        let focused_item_index = wired_children.focused_item_index;
        let children = wired_children.children;

        let (close_for_focus_out, controlled_fallback) = context.update(cx, |runtime| {
            runtime.sync_selection_mode(selection_mode);
            runtime.sync_groups(groups);
            runtime.sync_children(
                items,
                item_focus_handles,
                trigger_focus_handle,
                trigger_focused,
                focused_item_index,
            );

            let mut controlled_fallback = None;
            match selection_mode {
                SelectSelectionMode::Single => {
                    let observed_selected = controlled_single
                        .clone()
                        .unwrap_or_else(|| runtime.selected_value());
                    runtime.reconcile_single(observed_selected.clone(), !single_controlled);
                    if single_controlled {
                        controlled_fallback =
                            runtime.take_controlled_single_fallback(observed_selected);
                    }
                }
                SelectSelectionMode::Multiple => {
                    let observed_values = controlled_multiple
                        .clone()
                        .unwrap_or_else(|| runtime.selected_values());
                    runtime.reconcile_multiple(observed_values, !multiple_controlled);
                }
            }

            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }

            (runtime.take_focus_out_close_request(), controlled_fallback)
        });
        if let Some(fallback) = controlled_fallback {
            context.notify_value_fallback(fallback, window, cx);
        }
        if close_for_focus_out {
            context.set_open(
                false,
                crate::select::SelectOpenChangeReason::FocusOut,
                crate::select::SelectOpenChangeSource::None,
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
            let field_value = select_field_value(
                selection_mode,
                style_state.selected_value.as_ref(),
                &style_state.selected_values,
                value_serializer.as_ref(),
            );
            let value_missing = self_value_missing(&field_value, style_state.required);
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(field_value)
                .disabled(style_state.disabled)
                .focused(style_state.focused)
                .required(style_state.required)
                .value_missing(value_missing);
            if let Some(focus_handle) =
                context.read(cx, |runtime, _| runtime.trigger_focus_handle())
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

impl<T: Clone + Eq + 'static> SelectRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SelectChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<SelectChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn form(mut self, form: impl Into<SharedString>) -> Self {
        self.form = Some(form.into());
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
        on_value_change: impl Fn(Option<&T>, &mut SelectValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
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
        on_open_change: impl Fn(bool, &mut SelectOpenChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
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
        on_values_change: impl Fn(&[T], &mut SelectValueChangeDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_values_change = Some(Rc::new(on_values_change));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self {
        self.highlight_item_on_hover = highlight_item_on_hover;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn label_resolver(mut self, resolver: impl Fn(&T) -> SharedString + 'static) -> Self {
        self.label_resolver = Some(Rc::new(resolver));
        self
    }

    pub fn value_serializer(mut self, serializer: impl Fn(&T) -> SharedString + 'static) -> Self {
        self.value_serializer = Some(Rc::new(serializer));
        self
    }

    pub fn item_to_string_value(self, serializer: impl Fn(&T) -> SharedString + 'static) -> Self {
        self.value_serializer(serializer)
    }

    pub fn value_comparator(mut self, comparator: impl Fn(&T, &T) -> bool + 'static) -> Self {
        self.value_comparator = Some(Rc::new(comparator));
        self
    }

    pub fn multiple_value_formatter(
        mut self,
        formatter: impl Fn(&[SharedString], &[T]) -> SharedString + 'static,
    ) -> Self {
        self.multiple_value_formatter = Some(Rc::new(formatter));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectRootStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn select_field_value<T: Clone + Eq + 'static>(
    selection_mode: SelectSelectionMode,
    selected_value: Option<&T>,
    selected_values: &[T],
    serializer: Option<&SelectValueSerializer<T>>,
) -> FieldValue {
    match selection_mode {
        SelectSelectionMode::Single => selected_value
            .map(|value| {
                serializer
                    .map(|serializer| FieldValue::Text(serializer(value)))
                    .unwrap_or(FieldValue::Present)
            })
            .unwrap_or(FieldValue::Empty),
        SelectSelectionMode::Multiple => {
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
    }
}

fn self_value_missing(value: &FieldValue, required: bool) -> bool {
    required && !value.filled()
}
