use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, MouseButton, Pixels, Render,
    ScrollDelta, ScrollWheelEvent, SharedString, TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    field::{
        FieldError, FieldItem, FieldLabel, FieldRoot, FieldValidationMode, FieldValidity,
        FieldValidityStyleState,
    },
    number_field::{
        NumberFieldChangeReason, NumberFieldCommitReason, NumberFieldDecrement,
        NumberFieldDecrementStyleState, NumberFieldGroup, NumberFieldGroupStyleState,
        NumberFieldIncrement, NumberFieldIncrementStyleState, NumberFieldInput,
        NumberFieldInputStyleState, NumberFieldRoot, NumberFieldRootStyleState,
        NumberFieldScrubArea, NumberFieldScrubAreaCursor, NumberFieldScrubAreaCursorStyleState,
        NumberFieldScrubAreaStyleState, NumberFieldScrubDirection,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct NumberFieldTestConfig {
    pub default_value: Option<f64>,
    pub controlled_value: Option<Option<f64>>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: f64,
    pub small_step: f64,
    pub large_step: f64,
    pub snap_on_step: bool,
    pub allow_out_of_range: bool,
    pub allow_wheel_scrub: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub field: bool,
    pub field_root_disabled: bool,
    pub field_item_disabled: bool,
    pub field_validation_mode: FieldValidationMode,
    pub include_scrub_area: bool,
    pub scrub_direction: NumberFieldScrubDirection,
    pub pixel_sensitivity: f64,
}

impl Default for NumberFieldTestConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            controlled_value: None,
            min: None,
            max: None,
            step: 1.0,
            small_step: 0.1,
            large_step: 10.0,
            snap_on_step: false,
            allow_out_of_range: false,
            allow_wheel_scrub: false,
            disabled: false,
            read_only: false,
            required: false,
            field: false,
            field_root_disabled: false,
            field_item_disabled: false,
            field_validation_mode: FieldValidationMode::OnSubmit,
            include_scrub_area: false,
            scrub_direction: NumberFieldScrubDirection::Horizontal,
            pixel_sensitivity: 2.0,
        }
    }
}

#[derive(Clone, Default)]
pub struct NumberFieldObservations {
    pub value_changes: Vec<Option<f64>>,
    pub change_reasons: Vec<NumberFieldChangeReason>,
    pub committed_values: Vec<Option<f64>>,
    pub commit_reasons: Vec<NumberFieldCommitReason>,
    pub root_states: Vec<NumberFieldRootStyleState>,
    pub input_states: Vec<NumberFieldInputStyleState>,
    pub group_states: Vec<NumberFieldGroupStyleState>,
    pub increment_states: Vec<NumberFieldIncrementStyleState>,
    pub decrement_states: Vec<NumberFieldDecrementStyleState>,
    pub scrub_area_states: Vec<NumberFieldScrubAreaStyleState>,
    pub scrub_cursor_states: Vec<NumberFieldScrubAreaCursorStyleState>,
    pub field_validity_states: Vec<FieldValidityStyleState>,
}

impl NumberFieldObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.input_states.clear();
        self.group_states.clear();
        self.increment_states.clear();
        self.decrement_states.clear();
        self.scrub_area_states.clear();
        self.scrub_cursor_states.clear();
        self.field_validity_states.clear();
    }

    pub fn last_root_state(&self) -> NumberFieldRootStyleState {
        self.root_states
            .last()
            .cloned()
            .expect("number field root state should be observed")
    }

    pub fn last_input_state(&self) -> NumberFieldInputStyleState {
        self.input_states
            .last()
            .cloned()
            .expect("number field input state should be observed")
    }

    pub fn last_increment_state(&self) -> NumberFieldIncrementStyleState {
        self.increment_states
            .last()
            .cloned()
            .expect("number field increment state should be observed")
    }

    pub fn last_decrement_state(&self) -> NumberFieldDecrementStyleState {
        self.decrement_states
            .last()
            .cloned()
            .expect("number field decrement state should be observed")
    }
}

pub struct NumberFieldTestView {
    pub config: NumberFieldTestConfig,
    observations: Rc<RefCell<NumberFieldObservations>>,
}

impl NumberFieldTestView {
    pub fn new(config: NumberFieldTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(NumberFieldObservations::default())),
        }
    }

    pub fn read_observations(&self) -> NumberFieldObservations {
        self.observations.borrow().clone()
    }
}

impl Render for NumberFieldTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let change_observations = Rc::clone(&self.observations);
        let commit_observations = Rc::clone(&self.observations);
        let root_state_observations = Rc::clone(&self.observations);
        let input_state_observations = Rc::clone(&self.observations);
        let group_state_observations = Rc::clone(&self.observations);
        let increment_state_observations = Rc::clone(&self.observations);
        let decrement_state_observations = Rc::clone(&self.observations);
        let scrub_area_state_observations = Rc::clone(&self.observations);
        let scrub_cursor_state_observations = Rc::clone(&self.observations);

        let mut root = NumberFieldRoot::new()
            .id("number-field-test")
            .name("amount")
            .form("test-form")
            .default_value(self.config.default_value)
            .step(self.config.step)
            .small_step(self.config.small_step)
            .large_step(self.config.large_step)
            .snap_on_step(self.config.snap_on_step)
            .allow_out_of_range(self.config.allow_out_of_range)
            .allow_wheel_scrub(self.config.allow_wheel_scrub)
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .on_value_change(move |value, details, _window, _cx| {
                let mut observations = change_observations.borrow_mut();
                observations.value_changes.push(value);
                observations.change_reasons.push(details.reason);
            })
            .on_value_committed(move |value, details, _window, _cx| {
                let mut observations = commit_observations.borrow_mut();
                observations.committed_values.push(value);
                observations.commit_reasons.push(details.reason);
            })
            .style_with_state(move |state, root| {
                root_state_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "number-field-root".into())
            });

        if let Some(value) = self.config.controlled_value {
            root = root.value(value);
        }
        if let Some(min) = self.config.min {
            root = root.min(min);
        }
        if let Some(max) = self.config.max {
            root = root.max(max);
        }

        let input = NumberFieldInput::new()
            .id("number-input")
            .w(px(120.0))
            .h(px(28.0))
            .style_with_state(move |state, input| {
                input_state_observations
                    .borrow_mut()
                    .input_states
                    .push(state);
                input.debug_selector(|| "number-input".into())
            });
        let decrement = NumberFieldDecrement::new()
            .id("number-decrement")
            .w(px(24.0))
            .h(px(24.0))
            .child("-")
            .style_with_state(move |state, decrement| {
                decrement_state_observations
                    .borrow_mut()
                    .decrement_states
                    .push(state);
                decrement.debug_selector(|| "number-decrement".into())
            });
        let increment = NumberFieldIncrement::new()
            .id("number-increment")
            .w(px(24.0))
            .h(px(24.0))
            .child("+")
            .style_with_state(move |state, increment| {
                increment_state_observations
                    .borrow_mut()
                    .increment_states
                    .push(state);
                increment.debug_selector(|| "number-increment".into())
            });
        let group = NumberFieldGroup::new()
            .flex()
            .gap_1()
            .child(decrement)
            .child(input)
            .child(increment)
            .style_with_state(move |state, group| {
                group_state_observations
                    .borrow_mut()
                    .group_states
                    .push(state);
                group.debug_selector(|| "number-group".into())
            });
        root = root.child(group);

        if self.config.include_scrub_area {
            root = root
                .child(
                    NumberFieldScrubArea::new()
                        .id("number-scrub")
                        .direction(self.config.scrub_direction)
                        .pixel_sensitivity(self.config.pixel_sensitivity)
                        .w(px(80.0))
                        .h(px(24.0))
                        .child("scrub")
                        .style_with_state(move |state, scrub_area| {
                            scrub_area_state_observations
                                .borrow_mut()
                                .scrub_area_states
                                .push(state);
                            scrub_area.debug_selector(|| "number-scrub".into())
                        }),
                )
                .child(
                    NumberFieldScrubAreaCursor::new()
                        .id("number-scrub-cursor")
                        .child("cursor")
                        .style_with_state(move |state, cursor| {
                            scrub_cursor_state_observations
                                .borrow_mut()
                                .scrub_cursor_states
                                .push(state);
                            cursor.debug_selector(|| "number-scrub-cursor".into())
                        }),
                );
        }

        let content = if self.config.field {
            let validity_observations = Rc::clone(&self.observations);
            let mut field = FieldRoot::new()
                .id("number-field-field")
                .disabled(self.config.field_root_disabled)
                .validation_mode(self.config.field_validation_mode)
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    FieldLabel::new()
                        .w(px(120.0))
                        .h(px(20.0))
                        .child("Amount")
                        .style_with_state(|_state, label| {
                            label.debug_selector(|| "number-label".into())
                        }),
                );

            if self.config.field_item_disabled {
                field = field.child(FieldItem::new().disabled(true).child(root));
            } else {
                field = field.child(root);
            }

            field
                .child(FieldError::new().style_with_state(|_state, error| {
                    error.debug_selector(|| "number-error".into())
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

        div().size_full().p_4().child(content)
    }
}

pub fn open_number_field(
    cx: &mut TestAppContext,
    config: NumberFieldTestConfig,
) -> WindowHandle<NumberFieldTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(420.0), px(220.0)), move |_, _| {
        NumberFieldTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
) -> NumberFieldObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("number field test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("number field test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    update: impl FnOnce(&mut NumberFieldTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("number field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_number_input(cx: &mut TestAppContext, window: WindowHandle<NumberFieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("number field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur_number_input(cx: &mut TestAppContext, window: WindowHandle<NumberFieldTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("number field test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    keys: &str,
) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_text(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    text: &str,
) {
    cx.simulate_input(window.into(), text);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_wheel(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    selector: &'static str,
    delta_y: gpui::Pixels,
) {
    let bounds = debug_bounds(cx, window, selector).expect("selector should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_event(ScrollWheelEvent {
        position: bounds.center(),
        delta: ScrollDelta::Pixels(gpui::point(px(0.0), delta_y)),
        modifiers: Modifiers::default(),
        touch_phase: Default::default(),
    });
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("selector should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn drag_scrub(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    delta: gpui::Pixels,
) {
    let start = start_scrub(cx, window);
    let end = move_scrub_from(cx, window, start, delta);
    end_scrub_at(cx, window, end);
}

pub fn start_scrub(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
) -> gpui::Point<Pixels> {
    let bounds = debug_bounds(cx, window, "number-scrub").expect("scrub area should render");
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    let start = bounds.center();

    visual.simulate_mouse_down(start, MouseButton::Left, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    start
}

pub fn move_scrub_from(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    start: gpui::Point<Pixels>,
    delta: gpui::Pixels,
) -> gpui::Point<Pixels> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    let end = gpui::point(start.x + delta, start.y);

    visual.simulate_mouse_move(end, Some(MouseButton::Left), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();

    end
}

pub fn end_scrub_at(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    position: gpui::Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_mouse_up(position, MouseButton::Left, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<NumberFieldTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

pub fn assert_float_eq(actual: Option<f64>, expected: Option<f64>) {
    match (actual, expected) {
        (Some(actual), Some(expected)) => {
            assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
        }
        (None, None) => {}
        _ => panic!("{actual:?} != {expected:?}"),
    }
}

pub fn shared(value: &str) -> SharedString {
    SharedString::from(value)
}
