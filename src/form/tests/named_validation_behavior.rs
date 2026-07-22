use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::{
    field::{FieldRoot, FieldRootStyleState},
    form::{current_form_context, Form, FormContext},
    input::Input,
};

struct NamedValidationView {
    form_context: Rc<RefCell<Option<FormContext>>>,
    email_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
    username_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
}

impl NamedValidationView {
    fn new() -> Self {
        Self {
            form_context: Rc::new(RefCell::new(None)),
            email_states: Rc::new(RefCell::new(Vec::new())),
            username_states: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for NamedValidationView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();
        self.email_states.borrow_mut().clear();
        self.username_states.borrow_mut().clear();

        let email_states = Rc::clone(&self.email_states);
        let username_states = Rc::clone(&self.username_states);

        Form::new()
            .id("named-validation-form")
            .child(
                FieldRoot::new()
                    .id("email-field")
                    .name("email")
                    .style_with_state(move |state, field| {
                        email_states.borrow_mut().push(state);
                        field
                    })
                    .child(
                        Input::new()
                            .id("email-input")
                            .required(true)
                            .w(px(120.0))
                            .h(px(24.0)),
                    ),
            )
            .child(
                FieldRoot::new()
                    .id("username-field")
                    .name("username")
                    .style_with_state(move |state, field| {
                        username_states.borrow_mut().push(state);
                        field
                    })
                    .child(
                        Input::new()
                            .id("username-input")
                            .required(true)
                            .w(px(120.0))
                            .h(px(24.0)),
                    ),
            )
            .child(FormContextProbe::new(&self.form_context))
    }
}

#[derive(IntoElement)]
struct FormContextProbe {
    context: Rc<RefCell<Option<FormContext>>>,
}

impl FormContextProbe {
    fn new(context: &Rc<RefCell<Option<FormContext>>>) -> Self {
        Self {
            context: Rc::clone(context),
        }
    }
}

impl gpui::RenderOnce for FormContextProbe {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        *self.context.borrow_mut() = current_form_context();

        div().size(px(0.0))
    }
}

fn open_named_validation_form(cx: &mut TestAppContext) -> WindowHandle<NamedValidationView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(300.0), px(180.0)), move |_, _| {
        NamedValidationView::new()
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn validate_field(
    cx: &mut TestAppContext,
    window: WindowHandle<NamedValidationView>,
    name: &'static str,
) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.validate_field(name, window, cx);
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn last_states(
    cx: &mut TestAppContext,
    window: WindowHandle<NamedValidationView>,
) -> (FieldRootStyleState, FieldRootStyleState) {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            let email = view
                .email_states
                .borrow()
                .last()
                .copied()
                .expect("email field state should be captured");
            let username = view
                .username_states
                .borrow()
                .last()
                .copied()
                .expect("username field state should be captured");
            (email, username)
        })
        .expect("form test window should be open")
}

#[gpui::test]
fn validate_by_name_only_validates_matching_named_fields(cx: &mut TestAppContext) {
    let window = open_named_validation_form(cx);

    validate_field(cx, window, "email");
    let (email, username) = last_states(cx, window);

    assert!(email.invalid);
    assert_eq!(username.valid, None);
}
