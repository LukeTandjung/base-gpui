use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Bounds, Div, Element, ElementId,
    GlobalElementId, InspectorElementId, InteractiveElement, IntoElement, LayoutId, ParentElement,
    Pixels, RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window,
};

use crate::{
    field::FieldValidationMode,
    form::{
        actions::{FormSubmitAction, FormValidateAction, FORM_KEY_CONTEXT},
        context::with_form_context,
        FormContext, FormErrors, FormProps, FormStyleState, FormSubmitDetails, FormSubmitHandler,
        FormSubmitReason, FormValues,
    },
};

#[derive(IntoElement)]
pub struct Form {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    validation_mode: FieldValidationMode,
    errors: FormErrors,
    aria_label: Option<SharedString>,
    on_form_submit: Option<FormSubmitHandler>,
    style_with_state: Option<Rc<dyn Fn(FormStyleState, Div) -> Div + 'static>>,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            id: ElementId::from("form"),
            base: div(),
            children: Vec::new(),
            validation_mode: FieldValidationMode::OnSubmit,
            errors: FormErrors::new(),
            aria_label: None,
            on_form_submit: None,
            style_with_state: None,
        }
    }
}

impl Styled for Form {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Form {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = FormContext::new(
            self.id.clone(),
            cx,
            window,
            FormProps::new(self.validation_mode, self.errors, self.on_form_submit),
        );
        let style_state = context.read(cx, |runtime, _props| runtime.root_state());
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };
        let submit_context = context.clone();
        let validate_context = context.clone();

        FormScopeElement {
            context,
            inner: base
                .id(self.id.clone())
                .role(Role::Form)
                .when_some(self.aria_label, |this, aria_label| {
                    this.aria_label(aria_label)
                })
                .key_context(FORM_KEY_CONTEXT)
                .on_action(move |_: &FormSubmitAction, window, cx| {
                    submit_context.submit(FormSubmitReason::Action, window, cx);
                })
                .on_action(move |_: &FormValidateAction, window, cx| {
                    validate_context.validate(window, cx);
                })
                .children(self.children)
                .into_any_element(),
        }
    }
}

impl Form {
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

    pub fn validation_mode(mut self, validation_mode: FieldValidationMode) -> Self {
        self.validation_mode = validation_mode;
        self
    }

    pub fn errors(mut self, errors: FormErrors) -> Self {
        self.errors = errors;
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn on_form_submit(
        mut self,
        on_form_submit: impl Fn(FormValues, FormSubmitDetails, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_form_submit = Some(Rc::new(on_form_submit));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(FormStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

struct FormScopeElement {
    context: FormContext,
    inner: AnyElement,
}

impl IntoElement for FormScopeElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for FormScopeElement {
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
        self.context.sync_external_errors(cx);
        self.context.begin_registration_pass(cx);
        let layout_id = with_form_context(self.context.clone(), || {
            self.inner.request_layout(window, cx)
        });
        self.context.finish_registration_pass(cx);

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
        with_form_context(self.context.clone(), || {
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
        with_form_context(self.context.clone(), || {
            self.inner.paint(window, cx);
        });
    }
}
