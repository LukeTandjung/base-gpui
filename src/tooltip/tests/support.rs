use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Point, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::tooltip::{
    init, TooltipAlign, TooltipHandle, TooltipOpenChangeReason, TooltipPopup,
    TooltipPopupStyleState, TooltipPortal, TooltipPortalStyleState, TooltipPositioner,
    TooltipPositionerStyleState, TooltipProvider, TooltipRoot, TooltipRootStyleState, TooltipSide,
    TooltipTrackCursorAxis, TooltipTrigger, TooltipTriggerStyleState, TooltipViewport,
    TooltipViewportStyleState,
};

#[derive(Clone)]
pub struct TooltipTestConfig {
    pub default_open: bool,
    pub trigger_disabled: bool,
    pub keep_mounted: bool,
    pub delay: Option<Duration>,
    pub close_delay: Option<Duration>,
    pub provider_delay: Duration,
    pub provider_close_delay: Duration,
    pub provider_timeout: Duration,
    pub close_on_click: bool,
    pub disable_hoverable_popup: bool,
    pub positioner_side: TooltipSide,
    pub positioner_align: TooltipAlign,
    pub controlled_open: Option<bool>,
    pub track_cursor_axis: TooltipTrackCursorAxis,
    pub handle: Option<TooltipHandle<()>>,
    pub second_trigger: bool,
    pub cancel_open: bool,
    pub cancel_close: bool,
}

impl Default for TooltipTestConfig {
    fn default() -> Self {
        Self {
            default_open: false,
            trigger_disabled: false,
            keep_mounted: false,
            delay: Some(Duration::ZERO),
            close_delay: Some(Duration::ZERO),
            provider_delay: Duration::from_millis(600),
            provider_close_delay: Duration::ZERO,
            provider_timeout: Duration::from_millis(400),
            close_on_click: true,
            disable_hoverable_popup: false,
            positioner_side: TooltipSide::Top,
            positioner_align: TooltipAlign::Center,
            controlled_open: None,
            track_cursor_axis: TooltipTrackCursorAxis::None,
            handle: None,
            second_trigger: false,
            cancel_open: false,
            cancel_close: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct TooltipObservations {
    pub open_changes: Vec<(bool, TooltipOpenChangeReason)>,
    pub open_change_completes: Vec<(bool, TooltipOpenChangeReason)>,
    pub root_states: Vec<TooltipRootStyleState<()>>,
    pub trigger_states: Vec<TooltipTriggerStyleState<()>>,
    pub second_trigger_states: Vec<TooltipTriggerStyleState<()>>,
    pub portal_states: Vec<TooltipPortalStyleState>,
    pub positioner_states: Vec<TooltipPositionerStyleState>,
    pub popup_states: Vec<TooltipPopupStyleState>,
    pub viewport_states: Vec<TooltipViewportStyleState>,
    pub portal_clicks: usize,
    pub outside_clicks: usize,
}

impl TooltipObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
        self.second_trigger_states.clear();
        self.portal_states.clear();
        self.positioner_states.clear();
        self.popup_states.clear();
        self.viewport_states.clear();
    }

    pub fn root_state(&self) -> Option<TooltipRootStyleState<()>> {
        self.root_states.last().cloned()
    }

    pub fn trigger_state(&self) -> Option<TooltipTriggerStyleState<()>> {
        self.trigger_states.last().cloned()
    }

    pub fn second_trigger_state(&self) -> Option<TooltipTriggerStyleState<()>> {
        self.second_trigger_states.last().cloned()
    }

    pub fn popup_state(&self) -> Option<TooltipPopupStyleState> {
        self.popup_states.last().copied()
    }
}

pub struct TooltipTestView {
    pub config: TooltipTestConfig,
    observations: Rc<RefCell<TooltipObservations>>,
}

impl TooltipTestView {
    pub fn new(config: TooltipTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(TooltipObservations::default())),
        }
    }

    pub fn read_observations(&self) -> TooltipObservations {
        self.observations.borrow().clone()
    }
}

impl Render for TooltipTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let root_observations = Rc::clone(&self.observations);
        let open_observations = Rc::clone(&self.observations);
        let trigger_observations = Rc::clone(&self.observations);
        let second_trigger_observations = Rc::clone(&self.observations);
        let portal_observations = Rc::clone(&self.observations);
        let positioner_observations = Rc::clone(&self.observations);
        let popup_observations = Rc::clone(&self.observations);
        let viewport_observations = Rc::clone(&self.observations);
        let outside_observations = Rc::clone(&self.observations);
        let complete_observations = Rc::clone(&self.observations);
        let cancel_open = self.config.cancel_open;
        let cancel_close = self.config.cancel_close;

        let mut root = TooltipRoot::<()>::new()
            .id("tooltip-test")
            .default_open(self.config.default_open)
            .disable_hoverable_popup(self.config.disable_hoverable_popup)
            .track_cursor_axis(self.config.track_cursor_axis)
            .on_open_change(move |open, details, _window, _cx| {
                open_observations
                    .borrow_mut()
                    .open_changes
                    .push((open, details.reason()));
                if (open && cancel_open) || (!open && cancel_close) {
                    details.cancel();
                }
            })
            .on_open_change_complete(move |open, details, _window, _cx| {
                complete_observations
                    .borrow_mut()
                    .open_change_completes
                    .push((open, details.reason()));
            })
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "tooltip-root".into())
            });
        if let Some(controlled_open) = self.config.controlled_open {
            root = root.open(controlled_open);
        }
        if let Some(handle) = self.config.handle.clone() {
            root = root.handle(handle);
        }

        let mut primary_trigger = TooltipTrigger::<()>::new()
            .id("tooltip-trigger")
            .disabled(self.config.trigger_disabled)
            .close_on_click(self.config.close_on_click)
            .w(px(120.0))
            .h(px(32.0))
            .border_1()
            .border_color(rgb(0xd1d5db))
            .style_with_state(move |state, trigger| {
                trigger_observations.borrow_mut().trigger_states.push(state);
                trigger.debug_selector(|| "tooltip-trigger".into())
            })
            .child("Trigger");
        if let Some(delay) = self.config.delay {
            primary_trigger = primary_trigger.delay(delay);
        }
        if let Some(close_delay) = self.config.close_delay {
            primary_trigger = primary_trigger.close_delay(close_delay);
        }

        div()
            .size_full()
            .p_4()
            .child(
                TooltipProvider::<()>::new()
                    .delay(self.config.provider_delay)
                    .close_delay(self.config.provider_close_delay)
                    .timeout(self.config.provider_timeout)
                    .child(
                        root.child(primary_trigger)
                            .child(
                                TooltipTrigger::<()>::new()
                                    .id("tooltip-trigger-secondary")
                                    .disabled(!self.config.second_trigger)
                                    .w(px(120.0))
                                    .h(px(32.0))
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .payload(())
                                    .style_with_state(move |state, trigger| {
                                        second_trigger_observations
                                            .borrow_mut()
                                            .second_trigger_states
                                            .push(state);
                                        trigger
                                            .debug_selector(|| "tooltip-trigger-secondary".into())
                                    })
                                    .child("Second"),
                            )
                            .child(
                                TooltipPortal::<()>::new()
                                    .keep_mounted(self.config.keep_mounted)
                                    .style_with_state(move |state, portal| {
                                        portal_observations.borrow_mut().portal_states.push(state);
                                        let click_observations = Rc::clone(&portal_observations);
                                        portal
                                            .debug_selector(|| "tooltip-portal".into())
                                            .on_any_mouse_down(move |_event, _window, _cx| {
                                                click_observations.borrow_mut().portal_clicks += 1;
                                            })
                                    })
                                    .child(
                                        TooltipPositioner::<()>::new()
                                            .side(self.config.positioner_side)
                                            .align(self.config.positioner_align)
                                            .style_with_state(move |state, positioner| {
                                                positioner_observations
                                                    .borrow_mut()
                                                    .positioner_states
                                                    .push(state);
                                                positioner
                                            })
                                            .child(
                                                TooltipPopup::<()>::new()
                                                    .w(px(180.0))
                                                    .h(px(72.0))
                                                    .bg(rgb(0xffffff))
                                                    .style_with_state(move |state, popup| {
                                                        popup_observations
                                                            .borrow_mut()
                                                            .popup_states
                                                            .push(state);
                                                        popup.debug_selector(|| {
                                                            "tooltip-popup".into()
                                                        })
                                                    })
                                                    .child(
                                                        TooltipViewport::<()>::new()
                                                            .style_with_state(
                                                                move |state, viewport| {
                                                                    viewport_observations
                                                                        .borrow_mut()
                                                                        .viewport_states
                                                                        .push(state);
                                                                    viewport
                                                                },
                                                            )
                                                            .child("Tooltip content"),
                                                    ),
                                            ),
                                    ),
                            ),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .top(px(260.0))
                    .left(px(16.0))
                    .w(px(100.0))
                    .h(px(32.0))
                    .id("tooltip-outside-target")
                    .focusable()
                    .debug_selector(|| "tooltip-outside-target".into())
                    .on_click(move |_event, _window, _cx| {
                        outside_observations.borrow_mut().outside_clicks += 1;
                    }),
            )
    }
}

pub fn open_tooltip(
    cx: &mut TestAppContext,
    config: TooltipTestConfig,
) -> WindowHandle<TooltipTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        TooltipTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<TooltipTestView>,
) -> TooltipObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("tooltip test window should be open")
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>) {
    click_selector(cx, window, "tooltip-trigger");
}

pub fn click_outside_target(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>) {
    click_selector(cx, window, "tooltip-outside-target");
}

pub fn click_portal(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>) {
    click_selector(cx, window, "tooltip-portal");
}

pub fn move_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<TooltipTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    move_mouse_to(cx, window, bounds.center());
}

pub fn move_mouse_to(
    cx: &mut TestAppContext,
    window: WindowHandle<TooltipTestView>,
    position: Point<Pixels>,
) {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_mouse_move(position, None, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn advance_clock(cx: &mut TestAppContext, duration: Duration) {
    cx.executor().advance_clock(duration);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_next(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>) {
    window
        .update(cx, |_view, window, _cx| {
            window.blur();
        })
        .expect("tooltip test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<TooltipTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<TooltipTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<TooltipTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}
