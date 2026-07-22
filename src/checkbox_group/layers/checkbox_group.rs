use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Bounds, Div, Element, ElementId, GlobalElementId, InspectorElementId,
    InteractiveElement, IntoElement, LayoutId, ParentElement, Pixels, RenderOnce, Role,
    SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window,
};

use crate::{
    checkbox_group::{
        context::with_checkbox_group_context, CheckboxGroupContext, CheckboxGroupProps,
        CheckboxGroupStyleState, CheckboxGroupValueChangeDetails, CheckboxGroupValueChangeHandler,
    },
    field::{
        current_field_context, current_field_item_disabled, FieldContext, FieldControlRegistration,
        FieldRootStyleState,
    },
    fieldset::current_fieldset_disabled,
};

#[derive(IntoElement)]
pub struct CheckboxGroup {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    default_value: Vec<SharedString>,
    value: Option<Vec<SharedString>>,
    all_values: Vec<SharedString>,
    disabled: bool,
    aria_label: Option<SharedString>,
    on_value_change: Option<CheckboxGroupValueChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(CheckboxGroupStyleState, Div) -> Div + 'static>>,
}

impl Default for CheckboxGroup {
    fn default() -> Self {
        Self {
            id: ElementId::from("checkbox-group"),
            base: div(),
            children: Vec::new(),
            default_value: Vec::new(),
            value: None,
            all_values: Vec::new(),
            disabled: false,
            aria_label: None,
            on_value_change: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for CheckboxGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for CheckboxGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CheckboxGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let field_context = current_field_context();
        let field_state = field_context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.root_state(props)))
            .unwrap_or_default();
        let disabled = self.disabled
            || field_state.disabled
            || current_field_item_disabled()
            || current_fieldset_disabled();
        let context = CheckboxGroupContext::new(
            self.id.clone(),
            cx,
            window,
            self.value,
            self.default_value,
            CheckboxGroupProps::new(disabled, self.all_values, self.on_value_change),
        );
        let style_state = context.read(cx, |runtime, props| {
            checkbox_group_style_state(runtime, props, field_state)
        });
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };
        let mut base = base.id(self.id.clone()).role(Role::Group);

        if let Some(aria_label) = self.aria_label {
            base = base.aria_label(aria_label);
        }

        CheckboxGroupScopeElement {
            group_key: self.id.to_string().into(),
            context,
            field_context,
            inner: base.children(self.children).into_any_element(),
        }
    }
}

impl CheckboxGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn default_value(
        mut self,
        values: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.default_value = normalize_values(values.into_iter().map(Into::into).collect());
        self
    }

    pub fn value(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.value = Some(normalize_values(
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn all_values(mut self, values: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.all_values = normalize_values(values.into_iter().map(Into::into).collect());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_value_change(
        mut self,
        on_value_change: impl Fn(Vec<SharedString>, &mut CheckboxGroupValueChangeDetails, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_value_change = Some(Rc::new(on_value_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(CheckboxGroupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn checkbox_group_style_state(
    runtime: &crate::checkbox_group::CheckboxGroupRuntime,
    props: &CheckboxGroupProps,
    field_state: FieldRootStyleState,
) -> CheckboxGroupStyleState {
    CheckboxGroupStyleState::new(
        props.disabled(),
        field_state.touched,
        field_state.dirty,
        field_state.valid,
        runtime.filled() || field_state.filled,
        runtime.focused() || field_state.focused,
    )
}

fn field_registration(
    group_key: SharedString,
    context: &CheckboxGroupContext,
    cx: &App,
) -> FieldControlRegistration {
    context.read(cx, |runtime, props| {
        let mut registration = FieldControlRegistration::new(group_key)
            .value(runtime.field_value())
            .disabled(props.disabled())
            .focused(runtime.focused())
            .required(runtime.required())
            .value_missing(runtime.value_missing());

        if let Some(focus_handle) = runtime.focus_handle() {
            registration = registration.focus_handle(focus_handle);
        }

        registration
    })
}

struct CheckboxGroupScopeElement {
    group_key: SharedString,
    context: CheckboxGroupContext,
    field_context: Option<FieldContext>,
    inner: AnyElement,
}

impl IntoElement for CheckboxGroupScopeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for CheckboxGroupScopeElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.context.begin_registration_pass(cx);
        let layout_id = with_checkbox_group_context(self.context.clone(), || {
            self.inner.request_layout(window, cx)
        });
        self.context.finish_registration_pass(cx);

        if let Some(field_context) = self.field_context.as_ref() {
            field_context.register_control(
                field_registration(self.group_key.clone(), &self.context, cx),
                cx,
            );
        }

        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        with_checkbox_group_context(self.context.clone(), || {
            self.inner.prepaint(window, cx);
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        with_checkbox_group_context(self.context.clone(), || {
            self.inner.paint(window, cx);
        });
    }
}

fn normalize_values(values: Vec<SharedString>) -> Vec<SharedString> {
    let mut normalized = Vec::new();

    for value in values {
        if !normalized.contains(&value) {
            normalized.push(value);
        }
    }

    normalized
}
