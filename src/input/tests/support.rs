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
    input::{Input, InputStyleState},
};

#[derive(Clone, Debug)]
pub struct InputTestConfig {
    pub value: Option<SharedString>,
    pub default_value: SharedString,
    pub placeholder: SharedString,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub auto_focus: bool,
    pub field: bool,
    pub field_root_disabled: bool,
    pub field_item_disabled: bool,
    pub field_validation_mode: FieldValidationMode,
}

impl Default for InputTestConfig {
    fn default() -> Self {
        Self {
            value: None,
            default_value: SharedString::default(),
            placeholder: SharedString::default(),
            disabled: false,
            read_only: false,
            required: false,
            auto_focus: false,
            field: false,
            field_root_disabled: false,
            field_item_disabled: false,
            field_validation_mode: FieldValidationMode::OnSubmit,
        }
    }
}

#[derive(Clone, Default)]
pub struct InputObservations {
    pub states: Vec<InputStyleState>,
    pub value_changes: Vec<SharedString>,
    pub enter_values: Vec<SharedString>,
    pub field_validity_states: Vec<FieldValidityStyleState>,
}

impl InputObservations {
    fn begin_render(&mut self) {
        self.states.clear();
        self.field_validity_states.clear();
    }

    pub fn last_state(&self) -> InputStyleState {
        self.states
            .last()
            .cloned()
            .expect("input state should be observed")
    }
}

pub struct InputTestView {
    pub config: InputTestConfig,
    observations: Rc<RefCell<InputObservations>>,
}

impl InputTestView {
    pub fn new(config: InputTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(InputObservations::default())),
        }
    }

    pub fn read_observations(&self) -> InputObservations {
        self.observations.borrow().clone()
    }
}

impl Render for InputTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let state_observations = Rc::clone(&self.observations);
        let change_observations = Rc::clone(&self.observations);
        let enter_observations = Rc::clone(&self.observations);
        let mut input = Input::new()
            .id("input-test")
            .name("input-name")
            .default_value(self.config.default_value.clone())
            .placeholder(self.config.placeholder.clone())
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .auto_focus(self.config.auto_focus)
            .w(px(220.0))
            .h(px(28.0))
            .on_value_change(move |value| {
                change_observations.borrow_mut().value_changes.push(value);
            })
            .on_enter(move |value| {
                enter_observations.borrow_mut().enter_values.push(value);
            })
            .style_with_state(move |state, input| {
                state_observations.borrow_mut().states.push(state);
                input.debug_selector(|| "input-root".into())
            });

        if let Some(value) = self.config.value.clone() {
            input = input.value(value);
        }

        let content =
            if self.config.field {
                let validity_observations = Rc::clone(&self.observations);
                let mut root = FieldRoot::new()
                    .id("input-field")
                    .disabled(self.config.field_root_disabled)
                    .validation_mode(self.config.field_validation_mode)
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        FieldLabel::new()
                            .w(px(80.0))
                            .h(px(20.0))
                            .style_with_state(|_state, label| {
                                label.debug_selector(|| "input-label".into())
                            })
                            .child("Label"),
                    );

                if self.config.field_item_disabled {
                    root = root.child(FieldItem::new().disabled(true).child(input));
                } else {
                    root = root.child(input);
                }

                root.child(FieldError::new().style_with_state(|_state, error| {
                    error.debug_selector(|| "input-error".into())
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
                input.into_any_element()
            };

        div().size_full().p_4().child(content)
    }
}

pub fn open_input(cx: &mut TestAppContext, config: InputTestConfig) -> WindowHandle<InputTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        InputTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<InputTestView>,
) -> InputObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("input test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("input test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<InputTestView>,
    update: impl FnOnce(&mut InputTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("input test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_input(cx: &mut TestAppContext, window: WindowHandle<InputTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("input test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur_input(cx: &mut TestAppContext, window: WindowHandle<InputTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("input test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<InputTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<InputTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

pub fn click_label(cx: &mut TestAppContext, window: WindowHandle<InputTestView>) {
    let bounds = debug_bounds(cx, window, "input-label").expect("label should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}
