use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::toggle::{
    init, Toggle, TogglePressedChangeReason, TogglePressedChangeSource, ToggleStyleState,
};

#[derive(Clone, Copy, Default)]
pub struct ToggleTestConfig {
    pub default_pressed: bool,
    pub controlled_pressed: Option<bool>,
    pub disabled: bool,
    pub cancel_changes: bool,
}

#[derive(Clone, Default)]
pub struct ToggleObservations {
    pub value_changes: Vec<bool>,
    pub change_reasons: Vec<TogglePressedChangeReason>,
    pub change_sources: Vec<TogglePressedChangeSource>,
    pub change_cancelable: Vec<bool>,
    pub change_canceled: Vec<bool>,
    pub states: Vec<ToggleStyleState>,
}

impl ToggleObservations {
    fn begin_render(&mut self) {
        self.states.clear();
    }

    pub fn last_state(&self) -> ToggleStyleState {
        self.states
            .last()
            .copied()
            .expect("toggle style state should be observed")
    }
}

pub struct ToggleTestView {
    pub config: ToggleTestConfig,
    observations: Rc<RefCell<ToggleObservations>>,
}

impl ToggleTestView {
    pub fn new(config: ToggleTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(ToggleObservations::default())),
        }
    }

    pub fn read_observations(&self) -> ToggleObservations {
        self.observations.borrow().clone()
    }
}

impl Render for ToggleTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut toggle = Toggle::<gpui::SharedString>::new()
            .id("toggle-test")
            .default_pressed(self.config.default_pressed)
            .disabled(self.config.disabled)
            .w(px(36.0))
            .h(px(20.0));

        if let Some(pressed) = self.config.controlled_pressed {
            toggle = toggle.pressed(Some(pressed));
        }

        let cancel_changes = self.config.cancel_changes;
        let change_observations = Rc::clone(&self.observations);
        toggle = toggle.on_pressed_change(move |next, details, _window, _cx| {
            let mut observations = change_observations.borrow_mut();
            observations.value_changes.push(next);
            observations.change_reasons.push(details.reason());
            observations.change_sources.push(details.source());
            observations.change_cancelable.push(details.cancelable());

            if cancel_changes {
                details.cancel();
            }

            observations.change_canceled.push(details.is_canceled());
        });

        let state_observations = Rc::clone(&self.observations);
        toggle = toggle.style_with_state(move |state, toggle| {
            state_observations.borrow_mut().states.push(state);
            toggle.debug_selector(|| "toggle".into())
        });

        div().size_full().p_4().child(toggle)
    }
}

pub fn open_toggle(
    cx: &mut TestAppContext,
    config: ToggleTestConfig,
) -> WindowHandle<ToggleTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        ToggleTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<ToggleTestView>,
) -> ToggleObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("toggle test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<ToggleTestView>,
    update: impl FnOnce(&mut ToggleTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("toggle test window should be open");
    cx.run_until_parked();
}

pub fn focus_toggle(cx: &mut TestAppContext, window: WindowHandle<ToggleTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("toggle test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<ToggleTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_toggle(cx: &mut TestAppContext, window: WindowHandle<ToggleTestView>) {
    let bounds = debug_bounds(cx, window, "toggle").expect("toggle should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<ToggleTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
