use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::button::{init, ButtonRoot, ButtonRootStyleState};

#[derive(Clone, Copy, Default)]
pub struct ButtonTestConfig {
    pub disabled: bool,
    pub focusable_when_disabled: bool,
}

#[derive(Clone, Default)]
pub struct ButtonObservations {
    pub click_count: usize,
    pub states: Vec<ButtonRootStyleState>,
}

impl ButtonObservations {
    fn begin_render(&mut self) {
        self.states.clear();
    }

    pub fn last_state(&self) -> ButtonRootStyleState {
        self.states
            .last()
            .copied()
            .expect("button style state should be observed")
    }
}

pub struct ButtonTestView {
    pub config: ButtonTestConfig,
    observations: Rc<RefCell<ButtonObservations>>,
}

impl ButtonTestView {
    pub fn new(config: ButtonTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(ButtonObservations::default())),
        }
    }

    pub fn read_observations(&self) -> ButtonObservations {
        self.observations.borrow().clone()
    }
}

impl Render for ButtonTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let click_observations = Rc::clone(&self.observations);
        let state_observations = Rc::clone(&self.observations);

        let button = ButtonRoot::new()
            .id("button-test")
            .disabled(self.config.disabled)
            .focusable_when_disabled(self.config.focusable_when_disabled)
            .w(px(80.0))
            .h(px(24.0))
            .on_click(move |_event, _window, _cx| {
                click_observations.borrow_mut().click_count += 1;
            })
            .style_with_state(move |state, button| {
                state_observations.borrow_mut().states.push(state);
                button.debug_selector(|| "button".into())
            })
            .child("Press");

        div().size_full().p_4().child(button)
    }
}

pub fn open_button(
    cx: &mut TestAppContext,
    config: ButtonTestConfig,
) -> WindowHandle<ButtonTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        ButtonTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<ButtonTestView>,
) -> ButtonObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("button test window should be open")
}

pub fn focus_button(cx: &mut TestAppContext, window: WindowHandle<ButtonTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("button test window should be open");
    cx.run_until_parked();
}

pub fn blur_button(cx: &mut TestAppContext, window: WindowHandle<ButtonTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("button test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<ButtonTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_button(cx: &mut TestAppContext, window: WindowHandle<ButtonTestView>) {
    let bounds = debug_bounds(cx, window, "button").expect("button should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<ButtonTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
