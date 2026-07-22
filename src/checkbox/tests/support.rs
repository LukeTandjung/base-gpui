use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::checkbox::{
    init, CheckboxIndicator, CheckboxIndicatorStyleState, CheckboxRoot, CheckboxRootStyleState,
};

#[derive(Clone, Copy, Default)]
pub struct CheckboxTestConfig {
    pub default_checked: bool,
    pub controlled_checked: Option<bool>,
    pub indeterminate: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub include_indicator: bool,
    pub keep_mounted_indicator: bool,
}

#[derive(Clone, Default)]
pub struct CheckboxObservations {
    pub value_changes: Vec<bool>,
    pub root_states: Vec<CheckboxRootStyleState>,
    pub indicator_states: Vec<CheckboxIndicatorStyleState>,
}

impl CheckboxObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.indicator_states.clear();
    }

    pub fn last_root_state(&self) -> CheckboxRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("checkbox root state should be observed")
    }

    pub fn last_indicator_state(&self) -> Option<CheckboxIndicatorStyleState> {
        self.indicator_states.last().copied()
    }
}

pub struct CheckboxTestView {
    pub config: CheckboxTestConfig,
    observations: Rc<RefCell<CheckboxObservations>>,
}

impl CheckboxTestView {
    pub fn new(config: CheckboxTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(CheckboxObservations::default())),
        }
    }

    pub fn read_observations(&self) -> CheckboxObservations {
        self.observations.borrow().clone()
    }
}

impl Render for CheckboxTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = CheckboxRoot::new()
            .id("checkbox-test")
            .default_checked(self.config.default_checked)
            .indeterminate(self.config.indeterminate)
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .w(px(24.0))
            .h(px(24.0));

        if let Some(checked) = self.config.controlled_checked {
            root = root.checked(Some(checked));
        }

        let change_observations = Rc::clone(&self.observations);
        root = root.on_checked_change(move |next, _details, _window, _cx| {
            change_observations.borrow_mut().value_changes.push(next);
        });

        let state_observations = Rc::clone(&self.observations);
        root = root.style_with_state(move |state, root| {
            state_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "checkbox-root".into())
        });

        if self.config.include_indicator {
            let indicator_observations = Rc::clone(&self.observations);
            root = root.child(
                CheckboxIndicator::new()
                    .keep_mounted(self.config.keep_mounted_indicator)
                    .w(px(12.0))
                    .h(px(12.0))
                    .style_with_state(move |state, indicator| {
                        indicator_observations
                            .borrow_mut()
                            .indicator_states
                            .push(state);
                        indicator.debug_selector(|| "checkbox-indicator".into())
                    }),
            );
        }

        div().size_full().p_4().child(root)
    }
}

pub fn open_checkbox(
    cx: &mut TestAppContext,
    config: CheckboxTestConfig,
) -> WindowHandle<CheckboxTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        CheckboxTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxTestView>,
) -> CheckboxObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("checkbox test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxTestView>,
    update: impl FnOnce(&mut CheckboxTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("checkbox test window should be open");
    cx.run_until_parked();
}

pub fn focus_checkbox(cx: &mut TestAppContext, window: WindowHandle<CheckboxTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("checkbox test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<CheckboxTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_checkbox(cx: &mut TestAppContext, window: WindowHandle<CheckboxTestView>) {
    let bounds = debug_bounds(cx, window, "checkbox-root").expect("checkbox root should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
