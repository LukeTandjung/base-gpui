use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, SharedString,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    checkbox::{CheckboxRoot, CheckboxRootStyleState},
    checkbox_group::CheckboxGroup,
    field::{
        FieldLabel, FieldRoot, FieldRootStyleState, FieldValidationMode, FieldValidationResult,
        FieldValue,
    },
    fieldset::FieldsetRoot,
    form::{
        current_form_context, Form, FormContext, FormErrors, FormSubmitReason, FormValue,
        FormValues,
    },
};

#[derive(Clone, Debug)]
struct CheckboxGroupFormConfig {
    default_value: Vec<SharedString>,
    disabled: bool,
    fieldset_disabled: bool,
    disable_https: bool,
    show_https: bool,
    require_http: bool,
    require_https: bool,
    validation_mode: FieldValidationMode,
    custom_min_count: Option<usize>,
    errors: FormErrors,
    include_label: bool,
    second_group_same_name: bool,
}

impl Default for CheckboxGroupFormConfig {
    fn default() -> Self {
        Self {
            default_value: Vec::new(),
            disabled: false,
            fieldset_disabled: false,
            disable_https: false,
            show_https: true,
            require_http: true,
            require_https: true,
            validation_mode: FieldValidationMode::OnSubmit,
            custom_min_count: None,
            errors: FormErrors::new(),
            include_label: false,
            second_group_same_name: false,
        }
    }
}

struct CheckboxGroupFormView {
    config: CheckboxGroupFormConfig,
    form_context: Rc<RefCell<Option<FormContext>>>,
    field_states: Rc<RefCell<Vec<FieldRootStyleState>>>,
    submissions: Rc<RefCell<Vec<FormValues>>>,
    validate_values: Rc<RefCell<Vec<FieldValue>>>,
    checkbox_states: Rc<RefCell<Vec<(SharedString, CheckboxRootStyleState)>>>,
}

impl CheckboxGroupFormView {
    fn new(config: CheckboxGroupFormConfig) -> Self {
        Self {
            config,
            form_context: Rc::new(RefCell::new(None)),
            field_states: Rc::new(RefCell::new(Vec::new())),
            submissions: Rc::new(RefCell::new(Vec::new())),
            validate_values: Rc::new(RefCell::new(Vec::new())),
            checkbox_states: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl Render for CheckboxGroupFormView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.form_context.borrow_mut().take();
        self.field_states.borrow_mut().clear();
        self.checkbox_states.borrow_mut().clear();

        let field_states = Rc::clone(&self.field_states);
        let submissions = Rc::clone(&self.submissions);
        let validate_values = Rc::clone(&self.validate_values);
        let checkbox_states = Rc::clone(&self.checkbox_states);
        let custom_min_count = self.config.custom_min_count;
        let mut group = CheckboxGroup::new()
            .id("protocols-group")
            .default_value(self.config.default_value.clone())
            .disabled(self.config.disabled)
            .child(checkbox(
                "http",
                self.config.require_http,
                false,
                Rc::clone(&checkbox_states),
            ));
        if self.config.show_https {
            group = group.child(checkbox(
                "https",
                self.config.require_https,
                self.config.disable_https,
                Rc::clone(&checkbox_states),
            ));
        }
        group = group.child(checkbox("ssh", false, false, Rc::clone(&checkbox_states)));
        let mut field = FieldRoot::new()
            .id("protocols-field")
            .name("protocols")
            .validation_mode(self.config.validation_mode)
            .style_with_state(move |state, field| {
                field_states.borrow_mut().push(state);
                field
            });

        if let Some(min_count) = custom_min_count {
            field = field.validate(move |value, _window, _cx| {
                validate_values.borrow_mut().push(value.clone());
                match value {
                    FieldValue::List(values) if values.len() >= min_count => {
                        FieldValidationResult::Valid
                    }
                    _ => FieldValidationResult::Error("pick more".into()),
                }
            });
        }

        if self.config.include_label {
            field = field.child(
                FieldLabel::new()
                    .style_with_state(|_state, label| {
                        label.debug_selector(|| "protocols-label".into())
                    })
                    .child("Protocols"),
            );
        }

        let field = if self.config.fieldset_disabled {
            field.child_any(FieldsetRoot::new().disabled(true).child_any(group))
        } else {
            field.child(group)
        };

        let mut form = Form::new()
            .id("checkbox-group-form")
            .errors(self.config.errors.clone())
            .on_form_submit(move |values, _details, _window, _cx| {
                submissions.borrow_mut().push(values);
            })
            .child(field);

        if self.config.second_group_same_name {
            form = form.child(
                FieldRoot::new()
                    .id("protocols-field-second")
                    .name("protocols")
                    .child(
                        CheckboxGroup::new()
                            .id("protocols-group-second")
                            .default_value(["ssh"])
                            .child(checkbox(
                                "second-ssh",
                                false,
                                false,
                                Rc::clone(&self.checkbox_states),
                            )),
                    ),
            );
        }

        form.child(FormContextProbe::new(&self.form_context))
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

fn checkbox(
    value: &'static str,
    required: bool,
    disabled: bool,
    observations: Rc<RefCell<Vec<(SharedString, CheckboxRootStyleState)>>>,
) -> impl IntoElement {
    CheckboxRoot::new()
        .id(format!("checkbox-{value}"))
        .value(value)
        .required(required)
        .disabled(disabled)
        .w(px(24.0))
        .h(px(24.0))
        .style_with_state({
            let selector = format!("checkbox-{value}");
            let observed_value = SharedString::from(value);
            move |state, checkbox| {
                observations
                    .borrow_mut()
                    .push((observed_value.clone(), state));
                checkbox.debug_selector({
                    let selector = selector.clone();
                    move || selector.clone().into()
                })
            }
        })
}

fn open_form(
    cx: &mut TestAppContext,
    config: CheckboxGroupFormConfig,
) -> WindowHandle<CheckboxGroupFormView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(360.0), px(240.0)), move |_, _| {
        CheckboxGroupFormView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn submit(cx: &mut TestAppContext, window: WindowHandle<CheckboxGroupFormView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.submit(FormSubmitReason::Programmatic, window, cx);
        })
        .expect("checkbox group form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn click(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug selector should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn focus_next(cx: &mut TestAppContext, window: WindowHandle<CheckboxGroupFormView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("checkbox group form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn blur(cx: &mut TestAppContext, window: WindowHandle<CheckboxGroupFormView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("checkbox group form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
    update: impl FnOnce(&mut CheckboxGroupFormConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("checkbox group form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn last_field_state(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
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
        .expect("checkbox group form window should be open")
}

fn submissions(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
) -> Vec<FormValues> {
    window
        .update(cx, |view, _window, _cx| view.submissions.borrow().clone())
        .expect("checkbox group form window should be open")
}

fn validate_values(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
) -> Vec<FieldValue> {
    window
        .update(cx, |view, _window, _cx| {
            view.validate_values.borrow().clone()
        })
        .expect("checkbox group form window should be open")
}

fn checkbox_state(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupFormView>,
    value: &str,
) -> CheckboxRootStyleState {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.checkbox_states
                .borrow()
                .iter()
                .rev()
                .find(|(observed_value, _state)| observed_value.as_ref() == value)
                .map(|(_value, state)| *state)
                .unwrap_or_else(|| panic!("checkbox state for {value} should be captured"))
        })
        .expect("checkbox group form window should be open")
}

fn values(values: &[&str]) -> Vec<SharedString> {
    values
        .iter()
        .map(|value| SharedString::from(*value))
        .collect()
}

#[gpui::test]
fn required_group_validation_requires_every_enabled_required_child(cx: &mut TestAppContext) {
    let window = open_form(cx, CheckboxGroupFormConfig::default());

    submit(cx, window);
    assert!(last_field_state(cx, window).invalid);
    assert!(submissions(cx, window).is_empty());

    click(cx, window, "checkbox-http");
    submit(cx, window);
    assert!(last_field_state(cx, window).invalid);
    assert!(submissions(cx, window).is_empty());

    click(cx, window, "checkbox-https");
    submit(cx, window);
    assert!(!last_field_state(cx, window).invalid);
    assert_eq!(submissions(cx, window).len(), 1);
}

#[gpui::test]
fn submit_focuses_first_enabled_checkbox_when_group_is_invalid(cx: &mut TestAppContext) {
    let window = open_form(cx, CheckboxGroupFormConfig::default());

    submit(cx, window);

    assert!(last_field_state(cx, window).invalid);
    assert!(checkbox_state(cx, window, "http").focused);
}

#[gpui::test]
fn form_submission_collects_checkbox_group_list_value(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            default_value: values(&["http", "https"]),
            ..Default::default()
        },
    );

    submit(cx, window);

    let submissions = submissions(cx, window);
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions[0].get(&SharedString::from("protocols")),
        Some(&FormValue::List(values(&["http", "https"])))
    );
}

#[gpui::test]
fn disabled_checkbox_group_is_skipped_by_form(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            disabled: true,
            ..Default::default()
        },
    );

    submit(cx, window);

    let submissions = submissions(cx, window);
    assert_eq!(submissions.len(), 1);
    assert!(submissions[0].is_empty());
    assert!(!last_field_state(cx, window).invalid);
}

#[gpui::test]
fn same_name_checkbox_group_values_are_deterministic(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            default_value: values(&["http", "https"]),
            second_group_same_name: true,
            ..Default::default()
        },
    );

    submit(cx, window);

    let submissions = submissions(cx, window);
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions[0].get(&SharedString::from("protocols")),
        Some(&FormValue::List(values(&["ssh"])))
    );
}

#[gpui::test]
fn fieldset_disabled_checkbox_group_is_skipped_by_form(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            fieldset_disabled: true,
            ..Default::default()
        },
    );

    submit(cx, window);

    let submissions = submissions(cx, window);
    assert_eq!(submissions.len(), 1);
    assert!(submissions[0].is_empty());
    assert!(!last_field_state(cx, window).invalid);
}

#[gpui::test]
fn disabled_required_child_is_ignored_by_required_validation(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            disable_https: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-http");
    submit(cx, window);

    assert!(!last_field_state(cx, window).invalid);
    assert_eq!(submissions(cx, window).len(), 1);
}

#[gpui::test]
fn on_change_custom_validation_receives_full_group_value(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            validation_mode: FieldValidationMode::OnChange,
            custom_min_count: Some(2),
            require_http: false,
            require_https: false,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-http");
    assert!(last_field_state(cx, window).invalid);
    assert_eq!(
        validate_values(cx, window).last(),
        Some(&FieldValue::List(values(&["http"])))
    );

    click(cx, window, "checkbox-https");
    assert!(!last_field_state(cx, window).invalid);
    assert_eq!(
        validate_values(cx, window).last(),
        Some(&FieldValue::List(values(&["http", "https"])))
    );
}

#[gpui::test]
fn external_errors_mark_group_invalid_and_clear_on_group_change(cx: &mut TestAppContext) {
    let mut errors = FormErrors::new();
    errors.insert("protocols".into(), vec!["server error".into()]);
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            default_value: values(&["http", "https"]),
            require_http: false,
            require_https: false,
            errors,
            ..Default::default()
        },
    );

    assert!(last_field_state(cx, window).invalid);

    click(cx, window, "checkbox-ssh");

    assert!(!last_field_state(cx, window).invalid);
}

#[gpui::test]
fn field_label_click_focuses_first_group_checkbox(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            include_label: true,
            require_http: false,
            require_https: false,
            ..Default::default()
        },
    );

    click(cx, window, "protocols-label");

    assert!(checkbox_state(cx, window, "http").focused);
}

#[gpui::test]
fn empty_group_value_is_collected_deterministically_when_not_required(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            require_http: false,
            require_https: false,
            ..Default::default()
        },
    );

    submit(cx, window);

    let submissions = submissions(cx, window);
    assert_eq!(submissions.len(), 1);
    assert_eq!(
        submissions[0].get(&SharedString::from("protocols")),
        Some(&FormValue::List(Vec::new()))
    );
}

#[gpui::test]
fn on_blur_custom_validation_receives_full_group_value(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            validation_mode: FieldValidationMode::OnBlur,
            custom_min_count: Some(1),
            require_http: false,
            require_https: false,
            ..Default::default()
        },
    );

    focus_next(cx, window);
    blur(cx, window);

    assert!(last_field_state(cx, window).invalid);
    assert_eq!(
        validate_values(cx, window).last(),
        Some(&FieldValue::List(Vec::new()))
    );
}

#[gpui::test]
fn required_validation_prunes_unmounted_group_children(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            default_value: values(&["https"]),
            ..Default::default()
        },
    );

    update_config(cx, window, |config| {
        config.show_https = false;
    });
    submit(cx, window);

    assert!(last_field_state(cx, window).invalid);

    click(cx, window, "checkbox-http");
    submit(cx, window);

    assert!(!last_field_state(cx, window).invalid);
}

#[gpui::test]
fn group_field_state_tracks_filled_dirty_focused_and_touched(cx: &mut TestAppContext) {
    let window = open_form(
        cx,
        CheckboxGroupFormConfig {
            require_http: false,
            require_https: false,
            ..Default::default()
        },
    );

    assert!(!last_field_state(cx, window).filled);
    focus_next(cx, window);
    assert!(last_field_state(cx, window).focused);

    click(cx, window, "checkbox-http");
    let state = last_field_state(cx, window);
    assert!(state.filled);
    assert!(state.dirty);

    blur(cx, window);
    assert!(last_field_state(cx, window).touched);
}
