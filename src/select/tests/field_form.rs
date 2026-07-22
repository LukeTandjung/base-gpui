use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Render, SharedString,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    field::{FieldError, FieldLabel, FieldRoot, FieldRootStyleState, FieldValidationMode},
    form::{
        current_form_context, Form, FormContext, FormSubmitDetails, FormSubmitReason, FormValue,
        FormValues,
    },
    select::{
        SelectIcon, SelectItem, SelectItemIndicator, SelectItemText, SelectList, SelectPopup,
        SelectPortal, SelectPositioner, SelectRoot, SelectRootStyleState, SelectTrigger,
        SelectValue,
    },
};

const APPLE: &str = "apple";
const BANANA: &str = "banana";

#[derive(Clone)]
struct SelectFieldFormConfig {
    default_value: Option<&'static str>,
    required: bool,
    disabled: bool,
    validation_mode: FieldValidationMode,
    external_errors: BTreeMap<SharedString, Vec<SharedString>>,
}

impl Default for SelectFieldFormConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            required: false,
            disabled: false,
            validation_mode: FieldValidationMode::OnSubmit,
            external_errors: BTreeMap::new(),
        }
    }
}

#[derive(Clone)]
struct SubmitObservation {
    values: FormValues,
    details: FormSubmitDetails,
}

#[derive(Clone, Default)]
struct SelectFieldFormObservations {
    field_states: Vec<FieldRootStyleState>,
    select_states: Vec<SelectRootStyleState<&'static str>>,
    submissions: Vec<SubmitObservation>,
}

impl SelectFieldFormObservations {
    fn begin_render(&mut self) {
        self.field_states.clear();
        self.select_states.clear();
    }

    fn field_state(&self) -> FieldRootStyleState {
        *self.field_states.last().expect("field state should render")
    }

    fn select_state(&self) -> SelectRootStyleState<&'static str> {
        self.select_states
            .last()
            .cloned()
            .expect("select state should render")
    }
}

struct SelectFieldFormView {
    config: SelectFieldFormConfig,
    observations: Rc<RefCell<SelectFieldFormObservations>>,
    form_context: Rc<RefCell<Option<FormContext>>>,
}

impl SelectFieldFormView {
    fn new(config: SelectFieldFormConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(SelectFieldFormObservations::default())),
            form_context: Rc::new(RefCell::new(None)),
        }
    }

    fn read_observations(&self) -> SelectFieldFormObservations {
        self.observations.borrow().clone()
    }
}

impl Render for SelectFieldFormView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();
        self.form_context.borrow_mut().take();

        let submit_observations = Rc::clone(&self.observations);
        let mut field = FieldRoot::new()
            .id("select-field")
            .name("fruit")
            .validation_mode(self.config.validation_mode)
            .style_with_state({
                let observations = Rc::clone(&self.observations);
                move |state, field| {
                    observations.borrow_mut().field_states.push(state);
                    field.debug_selector(|| "select-field".into())
                }
            })
            .child(FieldLabel::new().child("Fruit"))
            .child_any(select_control(
                self.config.default_value,
                self.config.required,
                self.config.disabled,
                Rc::clone(&self.observations),
            ))
            .child(FieldError::new());

        if self.config.disabled {
            field = field.disabled(true);
        }

        div().size_full().p_4().child(
            Form::new()
                .id("select-form")
                .validation_mode(self.config.validation_mode)
                .errors(self.config.external_errors.clone())
                .on_form_submit(move |values, details, _window, _cx| {
                    submit_observations
                        .borrow_mut()
                        .submissions
                        .push(SubmitObservation { values, details });
                })
                .child(field)
                .child(FormContextProbe::new(&self.form_context)),
        )
    }
}

fn select_control(
    default_value: Option<&'static str>,
    required: bool,
    disabled: bool,
    observations: Rc<RefCell<SelectFieldFormObservations>>,
) -> impl IntoElement {
    SelectRoot::<&'static str>::new()
        .id("fruit-select")
        .name("fruit")
        .default_value(default_value)
        .required(required)
        .disabled(disabled)
        .item_to_string_value(|value| (*value).into())
        .style_with_state(move |state, root| {
            observations.borrow_mut().select_states.push(state);
            root.debug_selector(|| "fruit-select".into())
        })
        .child(
            SelectTrigger::new()
                .id("fruit-trigger")
                .style_with_state(|_state, trigger| {
                    trigger.debug_selector(|| "fruit-trigger".into())
                })
                .w(px(180.0))
                .h(px(32.0))
                .border_1()
                .border_color(rgb(0xd1d5db))
                .child(SelectValue::new().placeholder("Choose"))
                .child(SelectIcon::new()),
        )
        .child(
            SelectPortal::<&'static str>::new().child(
                SelectPositioner::new().child(
                    SelectPopup::new().child(
                        SelectList::new()
                            .child(select_item(APPLE, "Apple"))
                            .child(select_item(BANANA, "Banana")),
                    ),
                ),
            ),
        )
}

fn select_item(value: &'static str, label: &'static str) -> SelectItem<&'static str> {
    SelectItem::new()
        .id(format!("field-form-select-item-{value}"))
        .value(value)
        .label(label)
        .h(px(28.0))
        .style_with_state(move |_state, item| {
            item.debug_selector(move || item_selector(value).into())
        })
        .child(SelectItemIndicator::new().keep_mounted(true))
        .child(SelectItemText::new().text(label))
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

fn open_select_field_form(
    cx: &mut TestAppContext,
    config: SelectFieldFormConfig,
) -> WindowHandle<SelectFieldFormView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        SelectFieldFormView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectFieldFormView>,
) -> SelectFieldFormObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("select field form window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("select field form window should be open")
}

fn submit_form(cx: &mut TestAppContext, window: WindowHandle<SelectFieldFormView>) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .form_context
                .borrow()
                .clone()
                .expect("form context should be captured");
            context.submit(FormSubmitReason::Programmatic, window, cx);
        })
        .expect("select field form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<SelectFieldFormView>) {
    click_selector(cx, window, "fruit-trigger");
}

fn blur(cx: &mut TestAppContext, window: WindowHandle<SelectFieldFormView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("select field form window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn click_item(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectFieldFormView>,
    value: &'static str,
) {
    click_selector(cx, window, item_selector(value));
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectFieldFormView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectFieldFormView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn item_selector(value: &'static str) -> &'static str {
    match value {
        APPLE => "field-form-select-item-apple",
        BANANA => "field-form-select-item-banana",
        _ => unreachable!("unknown select field form item value"),
    }
}

fn external_error() -> BTreeMap<SharedString, Vec<SharedString>> {
    BTreeMap::from([(
        SharedString::from("fruit"),
        vec![SharedString::from("Pick fruit")],
    )])
}

#[gpui::test]
fn field_state_tracks_filled_dirty_required_and_focused(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            required: true,
            ..SelectFieldFormConfig::default()
        },
    );

    let observations = read_observations(cx, window);
    assert!(observations.select_state().required);
    assert!(!observations.field_state().filled);

    click_trigger(cx, window);
    assert!(read_observations(cx, window).field_state().focused);

    click_item(cx, window, BANANA);
    let observations = read_observations(cx, window);
    assert!(observations.field_state().filled);
    assert!(observations.field_state().dirty);
    assert_eq!(observations.select_state().selected_value, Some(BANANA));
}

#[gpui::test]
fn on_blur_validation_marks_required_select_touched_and_invalid(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            required: true,
            validation_mode: FieldValidationMode::OnBlur,
            ..SelectFieldFormConfig::default()
        },
    );

    click_trigger(cx, window);
    blur(cx, window);

    let observations = read_observations(cx, window);
    assert!(observations.field_state().touched);
    assert!(!observations.field_state().focused);
    assert!(observations.field_state().invalid);
}

#[gpui::test]
fn required_select_blocks_submit_until_value_is_selected(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            required: true,
            ..SelectFieldFormConfig::default()
        },
    );

    submit_form(cx, window);
    let observations = read_observations(cx, window);
    assert!(observations.submissions.is_empty());
    assert!(observations.field_state().invalid);
    assert!(observations.select_state().focused);

    click_trigger(cx, window);
    click_item(cx, window, BANANA);
    submit_form(cx, window);

    assert_eq!(read_observations(cx, window).submissions.len(), 1);
}

#[gpui::test]
fn named_single_select_submits_serialized_value(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            default_value: Some(BANANA),
            ..SelectFieldFormConfig::default()
        },
    );

    submit_form(cx, window);

    let observations = read_observations(cx, window);
    let submission = observations.submissions.last().expect("form should submit");
    assert_eq!(submission.details.reason, FormSubmitReason::Programmatic);
    assert_eq!(
        submission.values.get(&SharedString::from("fruit")),
        Some(&FormValue::Text(SharedString::from(BANANA)))
    );
}

#[gpui::test]
fn disabled_select_is_skipped_by_form(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            default_value: Some(BANANA),
            disabled: true,
            ..SelectFieldFormConfig::default()
        },
    );

    submit_form(cx, window);

    let observations = read_observations(cx, window);
    let submission = observations.submissions.last().expect("form should submit");
    assert!(!submission.values.contains_key(&SharedString::from("fruit")));
}

#[gpui::test]
fn select_value_change_clears_matching_external_form_error(cx: &mut TestAppContext) {
    let window = open_select_field_form(
        cx,
        SelectFieldFormConfig {
            external_errors: external_error(),
            ..SelectFieldFormConfig::default()
        },
    );

    assert!(read_observations(cx, window).field_state().invalid);

    click_trigger(cx, window);
    click_item(cx, window, BANANA);

    assert!(!read_observations(cx, window).field_state().invalid);
}
