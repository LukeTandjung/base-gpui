use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::{
    checkbox::CheckboxRoot,
    field::FieldRoot,
    form::{current_form_context, Form, FormContext, FormSubmitReason, FormValue, FormValues},
    number_field::{NumberFieldInput, NumberFieldRoot},
    radio_group::{RadioGroupRadio, RadioGroupRoot},
    switch::SwitchRoot,
};

struct ControlValuesView {
    form_context: Rc<RefCell<Option<FormContext>>>,
    submissions: Rc<RefCell<Vec<FormValues>>>,
}

impl ControlValuesView {
    fn new() -> Self {
        Self {
            form_context: Rc::new(RefCell::new(None)),
            submissions: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for ControlValuesView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();

        let submissions = Rc::clone(&self.submissions);
        Form::new()
            .id("control-values-form")
            .on_form_submit(move |values, _details, _window, _cx| {
                submissions.borrow_mut().push(values);
            })
            .child(
                FieldRoot::new()
                    .id("checkbox-field")
                    .name("checkbox")
                    .child_any(
                        CheckboxRoot::new()
                            .id("checkbox-control")
                            .default_checked(true),
                    ),
            )
            .child(
                FieldRoot::new()
                    .id("switch-field")
                    .name("switch")
                    .child_any(
                        SwitchRoot::new()
                            .id("switch-control")
                            .default_checked(false),
                    ),
            )
            .child(
                FieldRoot::new()
                    .id("radio-field")
                    .name("delivery")
                    .child_any(
                        RadioGroupRoot::<&'static str>::new()
                            .id("radio-control")
                            .default_value(Some("standard"))
                            .child(
                                RadioGroupRadio::new()
                                    .id("radio-standard")
                                    .value("standard"),
                            )
                            .child(RadioGroupRadio::new().id("radio-express").value("express")),
                    ),
            )
            .child(
                FieldRoot::new().id("number-field").name("quantity").child(
                    NumberFieldRoot::new()
                        .id("number-control")
                        .default_value(Some(2.5))
                        .child(NumberFieldInput::new().w(px(80.0)).h(px(24.0))),
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

fn open_control_values_form(cx: &mut TestAppContext) -> WindowHandle<ControlValuesView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(360.0), px(240.0)), move |_, _| {
        ControlValuesView::new()
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn submit(cx: &mut TestAppContext, window: WindowHandle<ControlValuesView>) {
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

#[gpui::test]
fn checkbox_switch_radio_and_number_field_contribute_form_values(cx: &mut TestAppContext) {
    let window = open_control_values_form(cx);

    submit(cx, window);
    let submissions = window
        .update(cx, |view, _window, _cx| view.submissions.borrow().clone())
        .expect("form test window should be open");

    assert_eq!(submissions.len(), 1);
    let values = &submissions[0];
    assert_eq!(values.get("checkbox"), Some(&FormValue::Bool(true)));
    assert_eq!(values.get("switch"), Some(&FormValue::Bool(false)));
    assert_eq!(values.get("delivery"), Some(&FormValue::Present));
    assert_eq!(values.get("quantity"), Some(&FormValue::Text("2.5".into())));
}
