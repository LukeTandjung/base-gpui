use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, IntoElement, Modifiers, Render, TestAppContext, VisualTestContext,
    WindowHandle,
};

use crate::{
    field::{FieldRoot, FieldRootStyleState},
    form::{current_form_context, Form, FormContext, FormSubmitReason, FormValues},
    switch::{SwitchRoot, SwitchThumb},
};

struct UnnamedSwitchFormView {
    form_context: Rc<RefCell<Option<FormContext>>>,
    field_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
    submissions: Rc<RefCell<Vec<FormValues>>>,
}

impl UnnamedSwitchFormView {
    fn new() -> Self {
        Self {
            form_context: Rc::new(RefCell::new(None)),
            field_states: Rc::new(RefCell::new(Vec::new())),
            submissions: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for UnnamedSwitchFormView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();
        self.field_states.borrow_mut().clear();

        let submissions = Rc::clone(&self.submissions);
        let field_states = Rc::clone(&self.field_states);

        Form::new()
            .id("unnamed-switch-form")
            .on_form_submit(move |values, _details, _window, _cx| {
                submissions.borrow_mut().push(values);
            })
            .child(
                FieldRoot::new()
                    .id("unnamed-switch-field")
                    .style_with_state(move |state, field| {
                        field_states.borrow_mut().push(state);
                        field
                    })
                    .child_any(
                        SwitchRoot::new()
                            .id("required-switch")
                            .required(true)
                            .size(px(24.0))
                            .style_with_state(|_state, switch| {
                                switch.debug_selector(|| "required-switch".into())
                            })
                            .child(SwitchThumb::new().size(px(12.0))),
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

fn open_unnamed_switch_form(cx: &mut TestAppContext) -> WindowHandle<UnnamedSwitchFormView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(240.0), px(160.0)), move |_, _| {
        UnnamedSwitchFormView::new()
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn submit(cx: &mut TestAppContext, window: WindowHandle<UnnamedSwitchFormView>) {
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

fn click_switch(cx: &mut TestAppContext, window: WindowHandle<UnnamedSwitchFormView>) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    let bounds = visual
        .debug_bounds("required-switch")
        .expect("switch should render");
    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

fn last_field_state(
    cx: &mut TestAppContext,
    window: WindowHandle<UnnamedSwitchFormView>,
) -> FieldRootStyleState {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.field_states
                .borrow()
                .last()
                .copied()
                .expect("field state should be captured")
        })
        .expect("form test window should be open")
}

#[gpui::test]
fn unnamed_required_switch_blocks_submit_then_allows_submit_without_value(cx: &mut TestAppContext) {
    let window = open_unnamed_switch_form(cx);

    submit(cx, window);
    assert!(last_field_state(cx, window).invalid);
    let submissions = window
        .update(cx, |view, _window, _cx| view.submissions.borrow().clone())
        .expect("form test window should be open");
    assert!(submissions.is_empty());

    click_switch(cx, window);
    assert_eq!(last_field_state(cx, window).valid, Some(true));

    submit(cx, window);
    let submissions = window
        .update(cx, |view, _window, _cx| view.submissions.borrow().clone())
        .expect("form test window should be open");
    assert_eq!(submissions.len(), 1);
    assert!(submissions[0].is_empty());
}
