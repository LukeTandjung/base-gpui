use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, App, IntoElement, Render, RenderOnce, TestAppContext, Window,
    WindowHandle,
};

use crate::{
    checkbox::CheckboxRoot,
    field::{
        current_field_context, FieldContext, FieldItem, FieldRoot, FieldValidity,
        FieldValidityStyleState,
    },
    radio_group::{RadioGroupRadio, RadioGroupRoot},
    switch::SwitchRoot,
};

#[derive(Clone, Copy)]
enum IntegrationCase {
    CheckboxRootDisabled,
    CheckboxItemDisabled,
    SwitchRootDisabled,
    RadioGroupRootDisabled,
    RadioGroupItemRadioDisabled,
}

#[derive(Clone, Default)]
struct IntegrationObservations {
    disabled: Vec<bool>,
}

struct IntegrationView {
    case: IntegrationCase,
    observations: Rc<RefCell<IntegrationObservations>>,
}

impl IntegrationView {
    fn new(case: IntegrationCase) -> Self {
        Self {
            case,
            observations: Rc::new(RefCell::new(IntegrationObservations::default())),
        }
    }

    fn read_observations(&self) -> IntegrationObservations {
        self.observations.borrow().clone()
    }
}

impl Render for IntegrationView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().disabled.clear();
        let observations = Rc::clone(&self.observations);

        let content = match self.case {
            IntegrationCase::CheckboxRootDisabled => FieldRoot::new()
                .id("checkbox-field")
                .disabled(true)
                .child_any(
                    CheckboxRoot::new()
                        .id("field-checkbox")
                        .size(px(20.0))
                        .style_with_state(move |state, checkbox| {
                            observations.borrow_mut().disabled.push(state.disabled);
                            checkbox
                        }),
                )
                .into_any_element(),
            IntegrationCase::CheckboxItemDisabled => FieldRoot::new()
                .id("checkbox-item-field")
                .child(
                    FieldItem::new().disabled(true).child_any(
                        CheckboxRoot::new()
                            .id("field-item-checkbox")
                            .size(px(20.0))
                            .style_with_state(move |state, checkbox| {
                                observations.borrow_mut().disabled.push(state.disabled);
                                checkbox
                            }),
                    ),
                )
                .into_any_element(),
            IntegrationCase::SwitchRootDisabled => FieldRoot::new()
                .id("switch-field")
                .disabled(true)
                .child_any(
                    SwitchRoot::new()
                        .id("field-switch")
                        .size(px(20.0))
                        .style_with_state(move |state, switch| {
                            observations.borrow_mut().disabled.push(state.disabled);
                            switch
                        }),
                )
                .into_any_element(),
            IntegrationCase::RadioGroupRootDisabled => FieldRoot::new()
                .id("radio-field")
                .disabled(true)
                .child_any(
                    RadioGroupRoot::<&'static str>::new()
                        .id("field-radio-group")
                        .child(RadioGroupRadio::new().id("one-radio").value("one"))
                        .style_with_state(move |state, root| {
                            observations.borrow_mut().disabled.push(state.disabled);
                            root
                        }),
                )
                .into_any_element(),
            IntegrationCase::RadioGroupItemRadioDisabled => FieldRoot::new()
                .id("radio-item-field")
                .child(
                    FieldItem::new().disabled(true).child_any(
                        RadioGroupRoot::<&'static str>::new()
                            .id("field-item-radio-group")
                            .child(
                                RadioGroupRadio::new()
                                    .id("field-item-radio")
                                    .value("one")
                                    .style_with_state(move |state, radio| {
                                        observations.borrow_mut().disabled.push(state.disabled);
                                        radio
                                    }),
                            ),
                    ),
                )
                .into_any_element(),
        };

        div().size_full().child(content)
    }
}

fn open_integration(
    cx: &mut TestAppContext,
    case: IntegrationCase,
) -> WindowHandle<IntegrationView> {
    let window = cx.open_window(size(px(240.0), px(160.0)), move |_, _| {
        IntegrationView::new(case)
    });
    cx.run_until_parked();
    window
}

fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<IntegrationView>,
) -> IntegrationObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("integration test window should be open")
}

#[gpui::test]
fn checkbox_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
    let window = open_integration(cx, IntegrationCase::CheckboxRootDisabled);

    assert_eq!(read_observations(cx, window).disabled, vec![true]);
}

#[gpui::test]
fn checkbox_inside_disabled_field_item_is_disabled(cx: &mut TestAppContext) {
    let window = open_integration(cx, IntegrationCase::CheckboxItemDisabled);

    assert_eq!(read_observations(cx, window).disabled, vec![true]);
}

#[gpui::test]
fn switch_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
    let window = open_integration(cx, IntegrationCase::SwitchRootDisabled);

    assert_eq!(read_observations(cx, window).disabled, vec![true]);
}

#[gpui::test]
fn radio_group_consumes_field_root_disabled_state(cx: &mut TestAppContext) {
    let window = open_integration(cx, IntegrationCase::RadioGroupRootDisabled);

    assert_eq!(read_observations(cx, window).disabled, vec![true]);
}

#[gpui::test]
fn radio_group_radio_inside_disabled_field_item_is_disabled(cx: &mut TestAppContext) {
    let window = open_integration(cx, IntegrationCase::RadioGroupItemRadioDisabled);

    assert_eq!(read_observations(cx, window).disabled, vec![true]);
}

#[derive(Clone, Copy)]
enum RequiredIntegrationCase {
    Checkbox,
    Switch,
    RadioGroup,
}

#[derive(Clone, Default)]
struct RequiredIntegrationObservations {
    validity: Vec<FieldValidityStyleState>,
}

struct RequiredIntegrationView {
    case: RequiredIntegrationCase,
    observations: Rc<RefCell<RequiredIntegrationObservations>>,
    context: Rc<RefCell<Option<FieldContext>>>,
}

impl RequiredIntegrationView {
    fn new(case: RequiredIntegrationCase) -> Self {
        Self {
            case,
            observations: Rc::new(RefCell::new(RequiredIntegrationObservations::default())),
            context: Rc::new(RefCell::new(None)),
        }
    }

    fn read_observations(&self) -> RequiredIntegrationObservations {
        self.observations.borrow().clone()
    }
}

impl Render for RequiredIntegrationView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        self.observations.borrow_mut().validity.clear();
        self.context.borrow_mut().take();
        let observations = Rc::clone(&self.observations);
        let validity = FieldValidity::new().style_with_state(move |state, validity| {
            observations.borrow_mut().validity.push(state);
            validity
        });
        let capture_context = CaptureFieldContext::new(&self.context);

        let root = match self.case {
            RequiredIntegrationCase::Checkbox => FieldRoot::new()
                .id("required-checkbox-field")
                .child_any(CheckboxRoot::new().id("required-checkbox").required(true))
                .child(validity)
                .child_any(capture_context)
                .into_any_element(),
            RequiredIntegrationCase::Switch => FieldRoot::new()
                .id("required-switch-field")
                .child_any(SwitchRoot::new().id("required-switch").required(true))
                .child(validity)
                .child_any(capture_context)
                .into_any_element(),
            RequiredIntegrationCase::RadioGroup => FieldRoot::new()
                .id("required-radio-field")
                .child_any(
                    RadioGroupRoot::<&'static str>::new()
                        .id("required-radio-group")
                        .required(true)
                        .child(RadioGroupRadio::new().id("required-radio").value("one")),
                )
                .child(validity)
                .child_any(capture_context)
                .into_any_element(),
        };

        div().size_full().child(root)
    }
}

#[derive(IntoElement)]
struct CaptureFieldContext {
    context: Rc<RefCell<Option<FieldContext>>>,
}

impl CaptureFieldContext {
    fn new(context: &Rc<RefCell<Option<FieldContext>>>) -> Self {
        Self {
            context: Rc::clone(context),
        }
    }
}

impl RenderOnce for CaptureFieldContext {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        *self.context.borrow_mut() = current_field_context();

        div().size(px(0.0))
    }
}

fn open_required_integration(
    cx: &mut TestAppContext,
    case: RequiredIntegrationCase,
) -> WindowHandle<RequiredIntegrationView> {
    let window = cx.open_window(size(px(240.0), px(160.0)), move |_, _| {
        RequiredIntegrationView::new(case)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn validate_required_integration(
    cx: &mut TestAppContext,
    window: WindowHandle<RequiredIntegrationView>,
) {
    window
        .update(cx, |view, window, cx| {
            let context = view
                .context
                .borrow()
                .clone()
                .expect("required integration context should be captured");
            context.validate(window, cx);
        })
        .expect("required integration test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn read_required_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<RequiredIntegrationView>,
) -> RequiredIntegrationObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("required integration test window should be open")
}

#[gpui::test]
fn required_checkbox_validation_uses_field_registration(cx: &mut TestAppContext) {
    let window = open_required_integration(cx, RequiredIntegrationCase::Checkbox);

    validate_required_integration(cx, window);

    let validity = read_required_observations(cx, window)
        .validity
        .pop()
        .expect("validity should be observed")
        .validity;
    assert!(validity.state.value_missing);
    assert_eq!(validity.state.valid, Some(false));
}

#[gpui::test]
fn required_switch_validation_uses_field_registration(cx: &mut TestAppContext) {
    let window = open_required_integration(cx, RequiredIntegrationCase::Switch);

    validate_required_integration(cx, window);

    let validity = read_required_observations(cx, window)
        .validity
        .pop()
        .expect("validity should be observed")
        .validity;
    assert!(validity.state.value_missing);
    assert_eq!(validity.state.valid, Some(false));
}

#[gpui::test]
fn required_radio_group_validation_uses_field_registration(cx: &mut TestAppContext) {
    let window = open_required_integration(cx, RequiredIntegrationCase::RadioGroup);

    validate_required_integration(cx, window);

    let validity = read_required_observations(cx, window)
        .validity
        .pop()
        .expect("validity should be observed")
        .validity;
    assert!(validity.state.value_missing);
    assert_eq!(validity.state.valid, Some(false));
}
