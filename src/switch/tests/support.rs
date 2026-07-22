use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::switch::{
    init, SwitchCheckedChangeReason, SwitchCheckedChangeSource, SwitchRoot, SwitchRootStyleState,
    SwitchThumb, SwitchThumbStyleState,
};

#[derive(Clone, Copy, Default)]
pub struct SwitchTestConfig {
    pub default_checked: bool,
    pub controlled_checked: Option<bool>,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub include_thumb: bool,
    pub cancel_changes: bool,
}

#[derive(Clone, Default)]
pub struct SwitchObservations {
    pub value_changes: Vec<bool>,
    pub change_reasons: Vec<SwitchCheckedChangeReason>,
    pub change_sources: Vec<SwitchCheckedChangeSource>,
    pub change_cancelable: Vec<bool>,
    pub change_canceled: Vec<bool>,
    pub root_states: Vec<SwitchRootStyleState>,
    pub thumb_states: Vec<SwitchThumbStyleState>,
}

impl SwitchObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.thumb_states.clear();
    }

    pub fn last_root_state(&self) -> SwitchRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("switch root state should be observed")
    }

    pub fn last_thumb_state(&self) -> Option<SwitchThumbStyleState> {
        self.thumb_states.last().copied()
    }
}

pub struct SwitchTestView {
    pub config: SwitchTestConfig,
    observations: Rc<RefCell<SwitchObservations>>,
}

impl SwitchTestView {
    pub fn new(config: SwitchTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(SwitchObservations::default())),
        }
    }

    pub fn read_observations(&self) -> SwitchObservations {
        self.observations.borrow().clone()
    }
}

impl Render for SwitchTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = SwitchRoot::new()
            .id("switch-test")
            .default_checked(self.config.default_checked)
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .w(px(36.0))
            .h(px(20.0));

        if let Some(checked) = self.config.controlled_checked {
            root = root.checked(Some(checked));
        }

        let cancel_changes = self.config.cancel_changes;
        let change_observations = Rc::clone(&self.observations);
        root = root.on_checked_change(move |next, details, _window, _cx| {
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
        root = root.style_with_state(move |state, root| {
            state_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "switch-root".into())
        });

        if self.config.include_thumb {
            let thumb_observations = Rc::clone(&self.observations);
            root = root.child(SwitchThumb::new().w(px(14.0)).h(px(14.0)).style_with_state(
                move |state, thumb| {
                    thumb_observations.borrow_mut().thumb_states.push(state);
                    thumb.debug_selector(|| "switch-thumb".into())
                },
            ));
        }

        div().size_full().p_4().child(root)
    }
}

pub fn open_switch(
    cx: &mut TestAppContext,
    config: SwitchTestConfig,
) -> WindowHandle<SwitchTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        SwitchTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SwitchTestView>,
) -> SwitchObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("switch test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<SwitchTestView>,
    update: impl FnOnce(&mut SwitchTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("switch test window should be open");
    cx.run_until_parked();
}

pub fn focus_switch(cx: &mut TestAppContext, window: WindowHandle<SwitchTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("switch test window should be open");
    cx.run_until_parked();
}

pub fn blur_switch(cx: &mut TestAppContext, window: WindowHandle<SwitchTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("switch test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<SwitchTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_switch(cx: &mut TestAppContext, window: WindowHandle<SwitchTestView>) {
    let bounds = debug_bounds(cx, window, "switch-root").expect("switch root should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<SwitchTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
