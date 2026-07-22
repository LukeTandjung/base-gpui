use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::{
    field::{FieldRoot, FieldRootStyleState},
    fieldset::FieldsetRoot,
    form::{current_form_context, Form, FormContext, FormSubmitReason, FormValues},
    input::Input,
};

#[derive(Clone, Debug)]
struct FieldsetFormConfig {
    fieldset_disabled: bool,
}

impl Default for FieldsetFormConfig {
    fn default() -> Self {
        Self {
            fieldset_disabled: false,
        }
    }
}

struct FieldsetFormView {
    config: FieldsetFormConfig,
    form_context: Rc<RefCell<Option<FormContext>>>,
    field_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
    submissions: Rc<RefCell<Vec<FormValues>>>,
}

impl FieldsetFormView {
    fn new(config: FieldsetFormConfig) -> Self {
        Self {
            config,
            form_context: Rc::new(RefCell::new(None)),
            field_states: Rc::new(RefCell::new(Vec::new())),
            submissions: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for FieldsetFormView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();
        self.field_states.borrow_mut().clear();

        let field_states = Rc::clone(&self.field_states);
        let submissions = Rc::clone(&self.submissions);

        Form::new()
            .id("fieldset-form")
            .on_form_submit(move |values, _details, _window, _cx| {
                submissions.borrow_mut().push(values);
            })
            .child(
                FieldsetRoot::new()
                    .id("fieldset-form-fieldset")
                    .disabled(self.config.fieldset_disabled)
                    .child_any(
                        FieldRoot::new()
                            .id("fieldset-form-field")
                            .name("email")
                            .style_with_state(move |state, field| {
                                field_states.borrow_mut().push(state);
                                field
                            })
                            .child(
                                Input::new()
                                    .id("fieldset-form-input")
                                    .required(true)
                                    .w(px(120.0))
                                    .h(px(24.0)),
                            ),
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

fn open_fieldset_form(
    cx: &mut TestAppContext,
    config: FieldsetFormConfig,
) -> WindowHandle<FieldsetFormView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(320.0), px(200.0)), move |_, _| {
        FieldsetFormView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn submit(cx: &mut TestAppContext, window: WindowHandle<FieldsetFormView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.submit(FormSubmitReason::Programmatic, window, cx);
        })
        .expect("fieldset form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldsetFormView>,
    update: impl FnOnce(&mut FieldsetFormConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("fieldset form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn last_field_state(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldsetFormView>,
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
        .expect("fieldset form window should be open")
}

fn submission_count(cx: &mut TestAppContext, window: WindowHandle<FieldsetFormView>) -> usize {
    window
        .update(cx, |view, _window, _cx| view.submissions.borrow().len())
        .expect("fieldset form window should be open")
}

fn last_submission_empty(cx: &mut TestAppContext, window: WindowHandle<FieldsetFormView>) -> bool {
    window
        .update(cx, |view, _window, _cx| {
            view.submissions
                .borrow()
                .last()
                .map(FormValues::is_empty)
                .unwrap_or(false)
        })
        .expect("fieldset form window should be open")
}

#[gpui::test]
fn form_skips_validation_and_values_for_fieldset_disabled_fields(cx: &mut TestAppContext) {
    let window = open_fieldset_form(
        cx,
        FieldsetFormConfig {
            fieldset_disabled: true,
        },
    );

    submit(cx, window);

    assert_eq!(submission_count(cx, window), 1);
    assert!(last_submission_empty(cx, window));
    assert!(!last_field_state(cx, window).invalid);
}

#[gpui::test]
fn reenabled_fieldset_allows_required_field_to_block_submit(cx: &mut TestAppContext) {
    let window = open_fieldset_form(
        cx,
        FieldsetFormConfig {
            fieldset_disabled: true,
        },
    );

    submit(cx, window);
    assert_eq!(submission_count(cx, window), 1);

    update_config(cx, window, |config| {
        config.fieldset_disabled = false;
    });
    submit(cx, window);

    assert_eq!(submission_count(cx, window), 1);
    assert!(last_field_state(cx, window).invalid);
}

#[gpui::test]
fn disabling_fieldset_clears_invalid_ui_for_descendant_field(cx: &mut TestAppContext) {
    let window = open_fieldset_form(cx, FieldsetFormConfig::default());

    submit(cx, window);
    assert!(last_field_state(cx, window).invalid);

    update_config(cx, window, |config| {
        config.fieldset_disabled = true;
    });

    let state = last_field_state(cx, window);
    assert!(state.disabled);
    assert!(!state.invalid);
}
