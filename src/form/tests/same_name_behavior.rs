use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::{
    field::{FieldRoot, FieldRootStyleState},
    form::{current_form_context, Form, FormContext, FormSubmitReason},
    input::Input,
};

struct SameNameView {
    form_context: Rc<RefCell<Option<FormContext>>>,
    empty_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
    filled_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
}

impl SameNameView {
    fn new() -> Self {
        Self {
            form_context: Rc::new(RefCell::new(None)),
            empty_states: Rc::new(RefCell::new(Vec::new())),
            filled_states: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for SameNameView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();
        self.empty_states.borrow_mut().clear();
        self.filled_states.borrow_mut().clear();

        let empty_states = Rc::clone(&self.empty_states);
        let filled_states = Rc::clone(&self.filled_states);

        Form::new()
            .id("same-name-form")
            .child(
                FieldRoot::new()
                    .id("empty-email-field")
                    .name("email")
                    .style_with_state(move |state, field| {
                        empty_states.borrow_mut().push(state);
                        field
                    })
                    .child(
                        Input::new()
                            .id("empty-email-input")
                            .required(true)
                            .w(px(120.0))
                            .h(px(24.0)),
                    ),
            )
            .child(
                FieldRoot::new()
                    .id("filled-email-field")
                    .name("email")
                    .style_with_state(move |state, field| {
                        filled_states.borrow_mut().push(state);
                        field
                    })
                    .child(
                        Input::new()
                            .id("filled-email-input")
                            .default_value("filled@example.com")
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

fn open_same_name_form(cx: &mut TestAppContext) -> WindowHandle<SameNameView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(300.0), px(180.0)), move |_, _| SameNameView::new());
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn submit(cx: &mut TestAppContext, window: WindowHandle<SameNameView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.submit(FormSubmitReason::Programmatic, window, cx);
        })
        .expect("form test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn states(
    cx: &mut TestAppContext,
    window: WindowHandle<SameNameView>,
) -> (FieldRootStyleState, FieldRootStyleState) {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            let empty = view
                .empty_states
                .borrow()
                .last()
                .copied()
                .expect("empty field state should be captured");
            let filled = view
                .filled_states
                .borrow()
                .last()
                .copied()
                .expect("filled field state should be captured");
            (empty, filled)
        })
        .expect("form test window should be open")
}

#[gpui::test]
fn same_name_fields_keep_independent_validity_on_submit(cx: &mut TestAppContext) {
    let window = open_same_name_form(cx);

    submit(cx, window);
    let (empty, filled) = states(cx, window);

    assert!(empty.invalid);
    assert_eq!(filled.valid, Some(true));
}
