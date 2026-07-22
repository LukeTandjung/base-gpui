use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, SharedString,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    field::{
        FieldError, FieldItem, FieldLabel, FieldRoot, FieldValidationMode, FieldValidity,
        FieldValidityStyleState,
    },
    form::{Form, FormValues},
    otp_field::{
        OTPFieldChangeReason, OTPFieldInput, OTPFieldInputStyleState, OTPFieldRoot,
        OTPFieldRootStyleState, OTPFieldValidationType, Separator,
    },
};

#[derive(Clone)]
pub struct OTPFieldTestConfig {
    pub length: usize,
    pub default_value: &'static str,
    pub controlled_value: Option<&'static str>,
    pub validation_type: OTPFieldValidationType,
    pub mask: bool,
    pub auto_submit: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub field: bool,
    pub field_root_disabled: bool,
    pub field_item_disabled: bool,
    pub field_validation_mode: FieldValidationMode,
    pub form: bool,
}

impl Default for OTPFieldTestConfig {
    fn default() -> Self {
        Self {
            length: 4,
            default_value: "",
            controlled_value: None,
            validation_type: OTPFieldValidationType::Numeric,
            mask: false,
            auto_submit: false,
            disabled: false,
            read_only: false,
            required: false,
            field: false,
            field_root_disabled: false,
            field_item_disabled: false,
            field_validation_mode: FieldValidationMode::OnSubmit,
            form: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct OTPFieldObservations {
    pub value_changes: Vec<SharedString>,
    pub change_reasons: Vec<OTPFieldChangeReason>,
    pub completed_values: Vec<SharedString>,
    pub complete_reasons: Vec<OTPFieldChangeReason>,
    pub invalid_values: Vec<SharedString>,
    pub invalid_reasons: Vec<OTPFieldChangeReason>,
    pub submitted: Vec<FormValues>,
    pub root_states: Vec<OTPFieldRootStyleState>,
    pub input_states: Vec<OTPFieldInputStyleState>,
    pub field_validity_states: Vec<FieldValidityStyleState>,
}

impl OTPFieldObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.input_states.clear();
        self.field_validity_states.clear();
    }

    pub fn last_root_state(&self) -> OTPFieldRootStyleState {
        self.root_states
            .last()
            .cloned()
            .expect("otp field root state should be observed")
    }

    pub fn slot_state(&self, index: usize) -> OTPFieldInputStyleState {
        self.input_states
            .iter()
            .rev()
            .find(|state| state.index == index)
            .cloned()
            .expect("otp slot state should be observed")
    }

    pub fn active_index(&self) -> Option<usize> {
        let mut seen: Vec<&OTPFieldInputStyleState> = Vec::new();
        for state in self.input_states.iter().rev() {
            if seen.iter().any(|s| s.index == state.index) {
                continue;
            }
            seen.push(state);
        }
        seen.iter().find(|state| state.active).map(|s| s.index)
    }
}

pub struct OTPFieldTestView {
    pub config: OTPFieldTestConfig,
    observations: Rc<RefCell<OTPFieldObservations>>,
}

impl OTPFieldTestView {
    pub fn new(config: OTPFieldTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(OTPFieldObservations::default())),
        }
    }

    pub fn read_observations(&self) -> OTPFieldObservations {
        self.observations.borrow().clone()
    }
}

impl Render for OTPFieldTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let change_observations = Rc::clone(&self.observations);
        let complete_observations = Rc::clone(&self.observations);
        let invalid_observations = Rc::clone(&self.observations);
        let root_state_observations = Rc::clone(&self.observations);

        let mut root = OTPFieldRoot::new()
            .id("otp-field-test")
            .name("code")
            .length(self.config.length)
            .default_value(self.config.default_value)
            .validation_type(self.config.validation_type)
            .mask(self.config.mask)
            .auto_submit(self.config.auto_submit)
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .flex()
            .gap_1()
            .on_value_change(move |value, details, _window, _cx| {
                let mut observations = change_observations.borrow_mut();
                observations.value_changes.push(value);
                observations.change_reasons.push(details.reason);
            })
            .on_value_complete(move |value, details, _window, _cx| {
                let mut observations = complete_observations.borrow_mut();
                observations.completed_values.push(value);
                observations.complete_reasons.push(details.reason);
            })
            .on_value_invalid(move |value, details, _window, _cx| {
                let mut observations = invalid_observations.borrow_mut();
                observations.invalid_values.push(value);
                observations.invalid_reasons.push(details.reason);
            })
            .style_with_state(move |state, root| {
                root_state_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "otp-root".into())
            });

        if let Some(value) = self.config.controlled_value {
            root = root.value(value);
        }

        for index in 0..self.config.length {
            if index == self.config.length / 2 && self.config.length > 2 {
                root = root.child(Separator::new().w(px(8.0)).h(px(2.0)));
            }
            let slot_observations = Rc::clone(&self.observations);
            root = root.child(
                OTPFieldInput::new()
                    .w(px(28.0))
                    .h(px(32.0))
                    .style_with_state(move |state, slot| {
                        let index = state.index;
                        slot_observations.borrow_mut().input_states.push(state);
                        slot.debug_selector(move || format!("otp-slot-{index}").into())
                    }),
            );
        }

        let content =
            if self.config.field {
                let mut field = FieldRoot::new()
                    .id("otp-field-field")
                    .disabled(self.config.field_root_disabled)
                    .validation_mode(self.config.field_validation_mode)
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        FieldLabel::new()
                            .w(px(120.0))
                            .h(px(20.0))
                            .child("Code")
                            .style_with_state(|_state, label| {
                                label.debug_selector(|| "otp-label".into())
                            }),
                    );
                if self.config.field_item_disabled {
                    field = field.child(FieldItem::new().disabled(true).child(root));
                } else {
                    field = field.child(root);
                }
                let validity_observations = Rc::clone(&self.observations);
                field
                    .child(FieldError::new().style_with_state(|_state, error| {
                        error.debug_selector(|| "otp-error".into())
                    }))
                    .child(
                        FieldValidity::new().style_with_state(move |state, validity| {
                            validity_observations
                                .borrow_mut()
                                .field_validity_states
                                .push(state);
                            validity
                        }),
                    )
                    .into_any_element()
            } else {
                root.into_any_element()
            };

        let content = if self.config.form {
            let submit_observations = Rc::clone(&self.observations);
            Form::new()
                .id("otp-form")
                .on_form_submit(move |values, _details, _window, _cx| {
                    submit_observations.borrow_mut().submitted.push(values);
                })
                .child_any(content)
                .into_any_element()
        } else {
            content
        };

        div().size_full().p_4().child(content)
    }
}

pub fn open_otp_field(
    cx: &mut TestAppContext,
    config: OTPFieldTestConfig,
) -> WindowHandle<OTPFieldTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(420.0), px(220.0)), move |_, _| {
        OTPFieldTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<OTPFieldTestView>,
) -> OTPFieldObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("otp field test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("otp field test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<OTPFieldTestView>,
    update: impl FnOnce(&mut OTPFieldTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("otp field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_otp_field(cx: &mut TestAppContext, window: WindowHandle<OTPFieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("otp field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur_otp_field(cx: &mut TestAppContext, window: WindowHandle<OTPFieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("otp field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<OTPFieldTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_text(cx: &mut TestAppContext, window: WindowHandle<OTPFieldTestView>, text: &str) {
    cx.simulate_input(window.into(), text);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<OTPFieldTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("selector should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<OTPFieldTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

pub fn write_clipboard(cx: &mut TestAppContext, text: &str) {
    cx.update(|cx| {
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.to_string()));
    });
}

pub fn shared(value: &str) -> SharedString {
    SharedString::from(value)
}
