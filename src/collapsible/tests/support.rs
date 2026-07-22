use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::collapsible::{
    init, CollapsibleOpenChangeReason, CollapsibleOpenChangeSource, CollapsiblePanel,
    CollapsiblePanelStyleState, CollapsibleRoot, CollapsibleRootStyleState, CollapsibleTrigger,
    CollapsibleTriggerStyleState,
};

#[derive(Clone, Copy)]
pub struct CollapsibleTestConfig {
    pub default_open: bool,
    pub controlled_open: Option<bool>,
    pub disabled: bool,
    pub keep_mounted_panel: bool,
    pub include_panel: bool,
    pub cancel_changes: bool,
}

impl Default for CollapsibleTestConfig {
    fn default() -> Self {
        Self {
            default_open: false,
            controlled_open: None,
            disabled: false,
            keep_mounted_panel: false,
            include_panel: true,
            cancel_changes: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct CollapsibleObservations {
    pub value_changes: Vec<bool>,
    pub change_reasons: Vec<CollapsibleOpenChangeReason>,
    pub change_sources: Vec<CollapsibleOpenChangeSource>,
    pub change_cancelable: Vec<bool>,
    pub change_canceled: Vec<bool>,
    pub root_states: Vec<CollapsibleRootStyleState>,
    pub trigger_states: Vec<CollapsibleTriggerStyleState>,
    pub panel_states: Vec<CollapsiblePanelStyleState>,
}

impl CollapsibleObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
        self.panel_states.clear();
    }

    pub fn last_root_state(&self) -> CollapsibleRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("collapsible root state should be observed")
    }

    pub fn last_trigger_state(&self) -> CollapsibleTriggerStyleState {
        self.trigger_states
            .last()
            .copied()
            .expect("collapsible trigger state should be observed")
    }

    pub fn last_panel_state(&self) -> Option<CollapsiblePanelStyleState> {
        self.panel_states.last().copied()
    }
}

pub struct CollapsibleTestView {
    pub config: CollapsibleTestConfig,
    observations: Rc<RefCell<CollapsibleObservations>>,
}

impl CollapsibleTestView {
    pub fn new(config: CollapsibleTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(CollapsibleObservations::default())),
        }
    }

    pub fn read_observations(&self) -> CollapsibleObservations {
        self.observations.borrow().clone()
    }
}

impl Render for CollapsibleTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = CollapsibleRoot::new()
            .id("collapsible-test")
            .default_open(self.config.default_open)
            .disabled(self.config.disabled)
            .flex()
            .flex_col()
            .gap_2();

        if let Some(open) = self.config.controlled_open {
            root = root.open(Some(open));
        }

        let cancel_changes = self.config.cancel_changes;
        let change_observations = Rc::clone(&self.observations);
        root = root.on_open_change(move |next, details, _window, _cx| {
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

        let root_observations = Rc::clone(&self.observations);
        root = root.style_with_state(move |state, root| {
            root_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "collapsible-root".into())
        });

        let trigger_observations = Rc::clone(&self.observations);
        root = root.child(
            CollapsibleTrigger::new()
                .id("collapsible-trigger")
                .w(px(120.0))
                .h(px(28.0))
                .style_with_state(move |state, trigger| {
                    trigger_observations.borrow_mut().trigger_states.push(state);
                    trigger.debug_selector(|| "collapsible-trigger".into())
                })
                .child("Toggle"),
        );

        if self.config.include_panel {
            let panel_observations = Rc::clone(&self.observations);
            root = root.child(
                CollapsiblePanel::new()
                    .keep_mounted(self.config.keep_mounted_panel)
                    .w(px(140.0))
                    .h(px(32.0))
                    .style_with_state(move |state, panel| {
                        panel_observations.borrow_mut().panel_states.push(state);
                        panel.debug_selector(|| "collapsible-panel".into())
                    })
                    .child("Panel content"),
            );
        }

        div().size_full().p_4().child(root)
    }
}

pub fn open_collapsible(
    cx: &mut TestAppContext,
    config: CollapsibleTestConfig,
) -> WindowHandle<CollapsibleTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        CollapsibleTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<CollapsibleTestView>,
) -> CollapsibleObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("collapsible test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<CollapsibleTestView>,
    update: impl FnOnce(&mut CollapsibleTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("collapsible test window should be open");
    cx.run_until_parked();
}

pub fn focus_trigger(cx: &mut TestAppContext, window: WindowHandle<CollapsibleTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("collapsible test window should be open");
    cx.run_until_parked();
}

pub fn blur_trigger(cx: &mut TestAppContext, window: WindowHandle<CollapsibleTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("collapsible test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(
    cx: &mut TestAppContext,
    window: WindowHandle<CollapsibleTestView>,
    keys: &str,
) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<CollapsibleTestView>) {
    let bounds =
        debug_bounds(cx, window, "collapsible-trigger").expect("collapsible trigger should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<CollapsibleTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
