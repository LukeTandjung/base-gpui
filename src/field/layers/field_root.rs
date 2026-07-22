use std::{rc::Rc, time::Duration};

use gpui::{
    div, AnyElement, App, Bounds, Div, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, ParentElement, Pixels, RenderOnce, SharedString, StyleRefinement,
    Styled, Window,
};

use crate::{
    field::{
        child_wiring::wire_children, context::with_field_context, FieldChild, FieldContext,
        FieldProps, FieldRootStyleState, FieldValidationHandler, FieldValidationMode,
        FieldValidationResult, FieldValue,
    },
    fieldset::current_fieldset_disabled,
    form::{current_form_context, FormFieldRegistration, FormFieldSnapshot},
};

#[derive(IntoElement)]
pub struct FieldRoot {
    id: ElementId,
    base: Div,
    children: Vec<FieldChild>,
    name: Option<SharedString>,
    disabled: bool,
    invalid: Option<bool>,
    dirty: Option<bool>,
    touched: Option<bool>,
    validation_mode: Option<FieldValidationMode>,
    validation_debounce: Option<Duration>,
    validate: Option<FieldValidationHandler>,
    style_with_state: Option<Rc<dyn Fn(FieldRootStyleState, Div) -> Div + 'static>>,
}

impl Default for FieldRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("field"),
            base: div(),
            children: Vec::new(),
            name: None,
            disabled: false,
            invalid: None,
            dirty: None,
            touched: None,
            validation_mode: None,
            validation_debounce: None,
            validate: None,
            style_with_state: None,
        }
    }
}

impl Styled for FieldRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for FieldRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let form_context = current_form_context();
        let disabled = self.disabled || current_fieldset_disabled();
        let validation_mode = self.validation_mode.unwrap_or_else(|| {
            form_context
                .as_ref()
                .map(|context| context.validation_mode())
                .unwrap_or(FieldValidationMode::OnSubmit)
        });
        let field_key = self.id.to_string().into();
        let context = FieldContext::new(
            self.id.clone(),
            cx,
            window,
            FieldProps::new(
                self.name,
                disabled,
                self.invalid,
                self.dirty,
                self.touched,
                validation_mode,
                self.validation_debounce,
                self.validate,
            ),
            form_context.clone(),
        );
        if let Some(form_context) = form_context.as_ref() {
            let external_errors = context.read(cx, |runtime, props| {
                let name = runtime.effective_name(props);
                form_context.external_errors_for(name.as_ref(), cx)
            });
            context.set_form_external_errors(external_errors, cx);
        }
        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let children = wire_children(self.children, context.clone());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        FieldScopeElement {
            field_key,
            context,
            form_context,
            inner: base.children(children).into_any_element(),
        }
    }
}

impl FieldRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<FieldChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<FieldChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(FieldChild::Any(child.into_any_element()));
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = Some(invalid);
        self
    }

    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = Some(dirty);
        self
    }

    pub fn touched(mut self, touched: bool) -> Self {
        self.touched = Some(touched);
        self
    }

    pub fn validation_mode(mut self, validation_mode: FieldValidationMode) -> Self {
        self.validation_mode = Some(validation_mode);
        self
    }

    pub fn validation_debounce(mut self, validation_debounce: Duration) -> Self {
        self.validation_debounce = Some(validation_debounce);
        self
    }

    pub fn validate(
        mut self,
        validate: impl Fn(&FieldValue, &mut Window, &mut App) -> FieldValidationResult + 'static,
    ) -> Self {
        self.validate = Some(Rc::new(validate));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FieldRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn form_field_registration(
    field_key: SharedString,
    context: FieldContext,
    cx: &App,
) -> FormFieldRegistration {
    let snapshot = form_field_snapshot(field_key.clone(), &context, cx);
    let validate_context = context.clone();
    let validate_key = field_key;

    FormFieldRegistration::new(snapshot).validate_with(Rc::new(move |window, cx| {
        validate_context.validate(window, cx);
        form_field_snapshot(validate_key.clone(), &validate_context, cx)
    }))
}

fn form_field_snapshot(
    field_key: SharedString,
    context: &FieldContext,
    cx: &App,
) -> FormFieldSnapshot {
    context.read(cx, |runtime, props| {
        FormFieldSnapshot::new(field_key)
            .maybe_name(runtime.effective_name(props))
            .value(runtime.value())
            .disabled(runtime.disabled_for_form(props))
            .valid(runtime.validity_data(props).state.valid)
            .maybe_focus_handle(runtime.focus_handle())
    })
}

struct FieldScopeElement {
    field_key: SharedString,
    context: FieldContext,
    form_context: Option<crate::form::FormContext>,
    inner: AnyElement,
}

impl IntoElement for FieldScopeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for FieldScopeElement {
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
        let layout_id = with_field_context(self.context.clone(), || {
            self.inner.request_layout(window, cx)
        });
        self.context.finish_registration_pass(cx);
        if let Some(form_context) = self.form_context.as_ref() {
            let external_errors = self.context.read(cx, |runtime, props| {
                let name = runtime.effective_name(props);
                form_context.external_errors_for(name.as_ref(), cx)
            });
            self.context.set_form_external_errors(external_errors, cx);
        }
        if self.context.take_validation_request(cx) {
            self.context.validate(window, cx);
        }
        if self.context.take_refresh_request(cx) {
            window.defer(cx, |window, _cx| window.refresh());
        }
        if let Some(form_context) = self.form_context.as_ref() {
            form_context.register_field(
                form_field_registration(self.field_key.clone(), self.context.clone(), cx),
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
        with_field_context(self.context.clone(), || {
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
        with_field_context(self.context.clone(), || {
            self.inner.paint(window, cx);
        });
    }
}
