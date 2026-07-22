use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    field::{current_field_context, FieldControlRegistration, FieldValue},
    fieldset::current_fieldset_disabled,
    radio_group::{
        child_wiring::wire_children, RadioGroupChild, RadioGroupContext, RadioGroupProps,
        RadioGroupRootStyleState, RadioGroupValueChangeDetails, RadioGroupValueChangeHandler,
    },
};

#[derive(IntoElement)]
pub struct RadioGroupRoot<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<RadioGroupChild<T>>,
    name: Option<SharedString>,
    form: Option<SharedString>,
    default_value: Option<T>,
    value: Option<Option<T>>,
    disabled: bool,
    read_only: bool,
    required: bool,
    aria_label: Option<SharedString>,
    on_value_change: Option<RadioGroupValueChangeHandler<T>>,
    style_with_state: Option<Rc<dyn Fn(RadioGroupRootStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for RadioGroupRoot<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("radio-group"),
            base: div(),
            children: Vec::new(),
            name: None,
            form: None,
            default_value: None,
            value: None,
            disabled: false,
            read_only: false,
            required: false,
            aria_label: None,
            on_value_change: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for RadioGroupRoot<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for RadioGroupRoot<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_disabled = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props).disabled))
            .unwrap_or(false);
        let disabled = self.disabled || field_disabled || current_fieldset_disabled();
        let name = self.name.clone();
        let id = self.id.clone();
        let controlled = self.value.clone();
        let context = RadioGroupContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            RadioGroupProps::new(
                self.name,
                self.form,
                disabled,
                self.read_only,
                self.required,
                self.on_value_change,
            ),
        );
        let wired_children = wire_children(self.children, context.clone(), window, cx);
        let radios = wired_children.radios;
        let radio_focus_handles = wired_children.radio_focus_handles;
        let focused_radio_index = wired_children.focused_radio_index;
        let children = wired_children.children;

        let focus_handle = context.update(cx, |runtime| {
            runtime.sync_children(radios, radio_focus_handles);
            runtime.sync_focused_index(focused_radio_index);

            let observed_selected = controlled.unwrap_or_else(|| runtime.selected_value());
            runtime.reconcile(observed_selected);
            runtime.take_initial_focus_handle()
        });
        if let Some(focus_handle) = focus_handle {
            focus_handle.focus(window, cx);
        }

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        if let Some(field_context) = field_context.as_ref() {
            let value = match style_state.filled {
                true => FieldValue::Present,
                false => FieldValue::Empty,
            };
            let mut registration = FieldControlRegistration::new(id.to_string())
                .value(value)
                .disabled(style_state.disabled)
                .focused(style_state.focused)
                .required(style_state.required);
            if let Some(focus_handle) =
                context.read(cx, |runtime, _| runtime.highlighted_focus_handle())
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

        base.id(id)
            .role(Role::RadioGroup)
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .children(children)
    }
}

impl<T: Clone + Eq + 'static> RadioGroupRoot<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<RadioGroupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<RadioGroupChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
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

    /// Sets the accessible label announced by screen readers. This is the
    /// literal-string substitute for Base UI's `aria-labelledby` id wiring,
    /// which has no gpui builder. When set, render the group's visible label
    /// text with `Text::new_inaccessible(...)` instead of `text!(...)` so the
    /// label is not announced twice.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Option<&T>, &mut RadioGroupValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(RadioGroupRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
