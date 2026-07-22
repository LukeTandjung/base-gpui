use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Pixels, Render, SharedString, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::meter::{
    MeterIndicator, MeterLabel, MeterRoot, MeterStyleState, MeterTrack, MeterValue,
};

#[derive(Clone)]
pub struct MeterTestConfig {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub use_format: bool,
    pub use_display: bool,
}

impl Default for MeterTestConfig {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            use_format: false,
            use_display: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct MeterObservations {
    pub root_states: Vec<MeterStyleState>,
    pub track_states: Vec<MeterStyleState>,
    pub indicator_states: Vec<MeterStyleState>,
    pub value_states: Vec<MeterStyleState>,
    pub label_states: Vec<MeterStyleState>,
    pub format_calls: Vec<f64>,
    pub display_calls: Vec<(String, f64)>,
}

impl MeterObservations {
    pub fn last_root_state(&self) -> MeterStyleState {
        self.root_states
            .last()
            .cloned()
            .expect("meter root state should be observed")
    }
}

pub struct MeterTestView {
    pub config: MeterTestConfig,
    observations: Rc<RefCell<MeterObservations>>,
}

impl MeterTestView {
    pub fn new(config: MeterTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(MeterObservations::default())),
        }
    }

    pub fn read_observations(&self) -> MeterObservations {
        self.observations.borrow().clone()
    }
}

impl Render for MeterTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        *self.observations.borrow_mut() = MeterObservations::default();

        let root_observations = Rc::clone(&self.observations);
        let track_observations = Rc::clone(&self.observations);
        let indicator_observations = Rc::clone(&self.observations);
        let value_observations = Rc::clone(&self.observations);
        let label_observations = Rc::clone(&self.observations);

        let mut value_part = MeterValue::new().style_with_state(move |state, value| {
            value_observations.borrow_mut().value_states.push(state);
            value
        });
        if self.config.use_display {
            let display_observations = Rc::clone(&self.observations);
            value_part = value_part.display(move |formatted, value| {
                display_observations
                    .borrow_mut()
                    .display_calls
                    .push((formatted.to_string(), value));
                SharedString::from("overridden")
            });
        }

        let mut root = MeterRoot::new()
            .id("meter-test")
            .value(self.config.value)
            .min(self.config.min)
            .max(self.config.max)
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "meter-root".into())
            })
            .child(
                MeterLabel::new()
                    .style_with_state(move |state, label| {
                        label_observations.borrow_mut().label_states.push(state);
                        label
                    })
                    .child("Battery level"),
            )
            .child(value_part)
            .child(
                MeterTrack::new()
                    .w(px(200.0))
                    .h(px(4.0))
                    .style_with_state(move |state, track| {
                        track_observations.borrow_mut().track_states.push(state);
                        track.debug_selector(|| "meter-track".into())
                    })
                    .child(MeterIndicator::new().h(px(4.0)).style_with_state(
                        move |state, indicator| {
                            indicator_observations
                                .borrow_mut()
                                .indicator_states
                                .push(state);
                            indicator.debug_selector(|| "meter-indicator".into())
                        },
                    )),
            );

        if self.config.use_format {
            let format_observations = Rc::clone(&self.observations);
            root = root.format(move |value| {
                format_observations.borrow_mut().format_calls.push(value);
                format!("{value} raw")
            });
        }

        div().size_full().p_4().child(root)
    }
}

pub fn open_meter(cx: &mut TestAppContext, config: MeterTestConfig) -> WindowHandle<MeterTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(420.0), px(220.0)), move |_, _| {
        MeterTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<MeterTestView>,
) -> MeterObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("meter test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("meter test window should be open")
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<MeterTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.debug_bounds(selector)
}
