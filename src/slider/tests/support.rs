use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, MouseButton, Pixels, Render,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    slider::{
        SliderChangeReason, SliderControl, SliderIndicator, SliderIndicatorStyleState, SliderLabel,
        SliderOrientation, SliderRoot, SliderRootStyleState, SliderThumb, SliderThumbAlignment,
        SliderThumbCollisionBehavior, SliderThumbStyleState, SliderTrack, SliderValue,
        SliderValueStyleState, SliderValues,
    },
    utils::{DirectionProvider, TextDirection},
};

#[derive(Clone)]
pub struct SliderTestConfig {
    pub default_value: Option<SliderValues>,
    pub controlled_value: Option<SliderValues>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub large_step: f64,
    pub min_steps_between_values: f64,
    pub orientation: SliderOrientation,
    pub thumb_collision_behavior: SliderThumbCollisionBehavior,
    pub thumb_alignment: SliderThumbAlignment,
    pub disabled: bool,
    pub thumb_count: usize,
    pub disabled_thumbs: Vec<usize>,
    pub direction: TextDirection,
    pub cancel_changes: bool,
}

impl Default for SliderTestConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            controlled_value: None,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            large_step: 10.0,
            min_steps_between_values: 0.0,
            orientation: SliderOrientation::Horizontal,
            thumb_collision_behavior: SliderThumbCollisionBehavior::Push,
            thumb_alignment: SliderThumbAlignment::Center,
            disabled: false,
            thumb_count: 1,
            disabled_thumbs: Vec::from([]),
            direction: TextDirection::Ltr,
            cancel_changes: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct SliderObservations {
    pub value_changes: Vec<SliderValues>,
    pub change_reasons: Vec<SliderChangeReason>,
    pub committed_values: Vec<SliderValues>,
    pub commit_reasons: Vec<SliderChangeReason>,
    pub root_states: Vec<SliderRootStyleState>,
    pub thumb_states: Vec<SliderThumbStyleState>,
    pub indicator_states: Vec<SliderIndicatorStyleState>,
    pub value_states: Vec<SliderValueStyleState>,
}

impl SliderObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.thumb_states.clear();
        self.indicator_states.clear();
        self.value_states.clear();
    }

    pub fn last_root_state(&self) -> SliderRootStyleState {
        self.root_states
            .last()
            .cloned()
            .expect("slider root state should be observed")
    }

    pub fn last_indicator_state(&self) -> SliderIndicatorStyleState {
        self.indicator_states
            .last()
            .cloned()
            .expect("slider indicator state should be observed")
    }

    pub fn last_value_state(&self) -> SliderValueStyleState {
        self.value_states
            .last()
            .cloned()
            .expect("slider value state should be observed")
    }
}

pub struct SliderTestView {
    pub config: SliderTestConfig,
    observations: Rc<RefCell<SliderObservations>>,
}

impl SliderTestView {
    pub fn new(config: SliderTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(SliderObservations::default())),
        }
    }

    pub fn read_observations(&self) -> SliderObservations {
        self.observations.borrow().clone()
    }
}

impl Render for SliderTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let change_observations = Rc::clone(&self.observations);
        let commit_observations = Rc::clone(&self.observations);
        let root_state_observations = Rc::clone(&self.observations);
        let indicator_state_observations = Rc::clone(&self.observations);
        let value_state_observations = Rc::clone(&self.observations);
        let cancel_changes = self.config.cancel_changes;

        let mut control = SliderControl::new()
            .id("slider-control")
            .w(px(200.0))
            .h(px(20.0))
            .style_with_state(|_state, control| control.debug_selector(|| "slider-control".into()))
            .child(
                SliderTrack::new()
                    .id("slider-track")
                    .w_full()
                    .h(px(4.0))
                    .child(
                        SliderIndicator::new()
                            .id("slider-indicator")
                            .h(px(4.0))
                            .style_with_state(move |state, indicator| {
                                indicator_state_observations
                                    .borrow_mut()
                                    .indicator_states
                                    .push(state);
                                indicator.debug_selector(|| "slider-indicator".into())
                            }),
                    ),
            );

        for index in 0..self.config.thumb_count {
            let thumb_observations = Rc::clone(&self.observations);
            control = control.child(
                SliderThumb::new()
                    .id(gpui::ElementId::from(gpui::SharedString::from(format!(
                        "slider-thumb-{index}"
                    ))))
                    .disabled(self.config.disabled_thumbs.contains(&index))
                    .w(px(12.0))
                    .h(px(12.0))
                    .style_with_state(move |state, thumb| {
                        thumb_observations.borrow_mut().thumb_states.push(state);
                        thumb.debug_selector(|| format!("slider-thumb-{index}"))
                    }),
            );
        }

        let mut root = SliderRoot::new()
            .id("slider-test")
            .name("volume")
            .min(self.config.min)
            .max(self.config.max)
            .step(self.config.step)
            .large_step(self.config.large_step)
            .min_steps_between_values(self.config.min_steps_between_values)
            .orientation(self.config.orientation)
            .thumb_collision_behavior(self.config.thumb_collision_behavior)
            .thumb_alignment(self.config.thumb_alignment)
            .disabled(self.config.disabled)
            .on_value_change(move |values, details, _window, _cx| {
                let mut observations = change_observations.borrow_mut();
                observations.value_changes.push(values);
                observations.change_reasons.push(details.reason());
                if cancel_changes {
                    details.cancel();
                }
            })
            .on_value_committed(move |values, details, _window, _cx| {
                let mut observations = commit_observations.borrow_mut();
                observations.committed_values.push(values);
                observations.commit_reasons.push(details.reason());
            })
            .style_with_state(move |state, root| {
                root_state_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "slider-root".into())
            })
            .child(SliderLabel::new().child("Volume"))
            .child(SliderValue::new().style_with_state(move |state, value| {
                value_state_observations
                    .borrow_mut()
                    .value_states
                    .push(state);
                value
            }))
            .child(control);

        if let Some(default_value) = self.config.default_value.clone() {
            root = root.default_value(default_value);
        }
        if let Some(controlled_value) = self.config.controlled_value.clone() {
            root = root.value(controlled_value);
        }

        div().size_full().p_4().child(
            DirectionProvider::new()
                .direction(self.config.direction)
                .child(root),
        )
    }
}

pub fn open_slider(
    cx: &mut TestAppContext,
    config: SliderTestConfig,
) -> WindowHandle<SliderTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(420.0), px(220.0)), move |_, _| {
        SliderTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SliderTestView>,
) -> SliderObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("slider test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("slider test window should be open")
}

pub fn focus_first_thumb(cx: &mut TestAppContext, window: WindowHandle<SliderTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("slider test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<SliderTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<SliderTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.debug_bounds(selector)
}

pub fn mouse_down_at(
    cx: &mut TestAppContext,
    window: WindowHandle<SliderTestView>,
    position: gpui::Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.simulate_mouse_down(position, MouseButton::Left, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn mouse_move_to(
    cx: &mut TestAppContext,
    window: WindowHandle<SliderTestView>,
    position: gpui::Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.simulate_mouse_move(position, Some(MouseButton::Left), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}

pub fn mouse_up_at(
    cx: &mut TestAppContext,
    window: WindowHandle<SliderTestView>,
    position: gpui::Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.simulate_mouse_up(position, MouseButton::Left, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
}
