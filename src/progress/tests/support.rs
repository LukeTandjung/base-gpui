use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Pixels, Render, SharedString, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::progress::{
    ProgressIndicator, ProgressLabel, ProgressRoot, ProgressStyleState, ProgressTrack,
    ProgressValue,
};

#[derive(Clone)]
pub struct ProgressTestConfig {
    pub value: Option<f64>,
    pub min: f64,
    pub max: f64,
    pub use_format: bool,
    pub use_display: bool,
}

impl Default for ProgressTestConfig {
    fn default() -> Self {
        Self {
            value: None,
            min: 0.0,
            max: 100.0,
            use_format: false,
            use_display: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct ProgressObservations {
    pub root_states: Vec<ProgressStyleState>,
    pub track_states: Vec<ProgressStyleState>,
    pub indicator_states: Vec<ProgressStyleState>,
    pub value_states: Vec<ProgressStyleState>,
    pub label_states: Vec<ProgressStyleState>,
    pub format_calls: Vec<f64>,
    pub display_calls: Vec<(Option<String>, Option<f64>)>,
}

impl ProgressObservations {
    pub fn last_root_state(&self) -> ProgressStyleState {
        self.root_states
            .last()
            .cloned()
            .expect("progress root state should be observed")
    }
}

pub struct ProgressTestView {
    pub config: ProgressTestConfig,
    observations: Rc<RefCell<ProgressObservations>>,
}

impl ProgressTestView {
    pub fn new(config: ProgressTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(ProgressObservations::default())),
        }
    }

    pub fn read_observations(&self) -> ProgressObservations {
        self.observations.borrow().clone()
    }
}

impl Render for ProgressTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        *self.observations.borrow_mut() = ProgressObservations::default();

        let root_observations = Rc::clone(&self.observations);
        let track_observations = Rc::clone(&self.observations);
        let indicator_observations = Rc::clone(&self.observations);
        let value_observations = Rc::clone(&self.observations);
        let label_observations = Rc::clone(&self.observations);

        let mut value_part = ProgressValue::new().style_with_state(move |state, value| {
            value_observations.borrow_mut().value_states.push(state);
            value
        });
        if self.config.use_display {
            let display_observations = Rc::clone(&self.observations);
            value_part = value_part.display(move |formatted, value| {
                display_observations
                    .borrow_mut()
                    .display_calls
                    .push((formatted.map(str::to_string), value));
                SharedString::from("overridden")
            });
        }

        let mut root = ProgressRoot::new()
            .id("progress-test")
            .value(self.config.value)
            .min(self.config.min)
            .max(self.config.max)
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "progress-root".into())
            })
            .child(
                ProgressLabel::new()
                    .style_with_state(move |state, label| {
                        label_observations.borrow_mut().label_states.push(state);
                        label
                    })
                    .child("Export data"),
            )
            .child(value_part)
            .child(
                ProgressTrack::new()
                    .w(px(200.0))
                    .h(px(4.0))
                    .style_with_state(move |state, track| {
                        track_observations.borrow_mut().track_states.push(state);
                        track.debug_selector(|| "progress-track".into())
                    })
                    .child(ProgressIndicator::new().h(px(4.0)).style_with_state(
                        move |state, indicator| {
                            indicator_observations
                                .borrow_mut()
                                .indicator_states
                                .push(state);
                            indicator.debug_selector(|| "progress-indicator".into())
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

pub fn open_progress(
    cx: &mut TestAppContext,
    config: ProgressTestConfig,
) -> WindowHandle<ProgressTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(420.0), px(220.0)), move |_, _| {
        ProgressTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<ProgressTestView>,
) -> ProgressObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("progress test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("progress test window should be open")
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<ProgressTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);
    visual.debug_bounds(selector)
}
