use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Point, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::preview_card::{
    init, PreviewCardAlign, PreviewCardArrow, PreviewCardArrowStyleState, PreviewCardBackdrop,
    PreviewCardBackdropStyleState, PreviewCardHandle, PreviewCardOpenChangeReason,
    PreviewCardPopup, PreviewCardPopupStyleState, PreviewCardPortal, PreviewCardPortalStyleState,
    PreviewCardPositioner, PreviewCardPositionerStyleState, PreviewCardRoot, PreviewCardSide,
    PreviewCardTrigger, PreviewCardTriggerStyleState, PreviewCardViewport,
    PreviewCardViewportStyleState,
};

#[derive(Clone)]
pub struct PreviewCardTestConfig {
    pub default_open: bool,
    pub keep_mounted: bool,
    pub delay: Option<Duration>,
    pub close_delay: Option<Duration>,
    pub positioner_side: PreviewCardSide,
    pub positioner_align: PreviewCardAlign,
    pub controlled_open: Option<bool>,
    pub handle: Option<PreviewCardHandle<&'static str>>,
    pub second_trigger: bool,
    pub cancel_open: bool,
    pub cancel_close: bool,
    pub prevent_unmount_on_close: bool,
    pub backdrop: bool,
}

impl Default for PreviewCardTestConfig {
    fn default() -> Self {
        Self {
            default_open: false,
            keep_mounted: false,
            delay: Some(Duration::ZERO),
            close_delay: Some(Duration::ZERO),
            positioner_side: PreviewCardSide::Bottom,
            positioner_align: PreviewCardAlign::Center,
            controlled_open: None,
            handle: None,
            second_trigger: false,
            cancel_open: false,
            cancel_close: false,
            prevent_unmount_on_close: false,
            backdrop: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct PreviewCardObservations {
    pub open_changes: Vec<(bool, PreviewCardOpenChangeReason)>,
    pub open_change_completes: Vec<(bool, PreviewCardOpenChangeReason)>,
    pub trigger_states: Vec<PreviewCardTriggerStyleState<&'static str>>,
    pub second_trigger_states: Vec<PreviewCardTriggerStyleState<&'static str>>,
    pub portal_states: Vec<PreviewCardPortalStyleState>,
    pub backdrop_states: Vec<PreviewCardBackdropStyleState>,
    pub positioner_states: Vec<PreviewCardPositionerStyleState>,
    pub popup_states: Vec<PreviewCardPopupStyleState>,
    pub arrow_states: Vec<PreviewCardArrowStyleState>,
    pub viewport_states: Vec<PreviewCardViewportStyleState>,
    pub rendered_payloads: Vec<Option<&'static str>>,
    pub outside_clicks: usize,
}

impl PreviewCardObservations {
    fn begin_render(&mut self) {
        self.trigger_states.clear();
        self.second_trigger_states.clear();
        self.portal_states.clear();
        self.backdrop_states.clear();
        self.positioner_states.clear();
        self.popup_states.clear();
        self.arrow_states.clear();
        self.viewport_states.clear();
        self.rendered_payloads.clear();
    }

    pub fn trigger_state(&self) -> Option<PreviewCardTriggerStyleState<&'static str>> {
        self.trigger_states.last().cloned()
    }

    pub fn popup_state(&self) -> Option<PreviewCardPopupStyleState> {
        self.popup_states.last().copied()
    }

    pub fn rendered_payload(&self) -> Option<Option<&'static str>> {
        self.rendered_payloads.last().copied()
    }
}

pub struct PreviewCardTestView {
    pub config: PreviewCardTestConfig,
    observations: Rc<RefCell<PreviewCardObservations>>,
}

impl PreviewCardTestView {
    pub fn new(config: PreviewCardTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(PreviewCardObservations::default())),
        }
    }

    pub fn read_observations(&self) -> PreviewCardObservations {
        self.observations.borrow().clone()
    }
}

impl Render for PreviewCardTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let open_observations = Rc::clone(&self.observations);
        let complete_observations = Rc::clone(&self.observations);
        let trigger_observations = Rc::clone(&self.observations);
        let second_trigger_observations = Rc::clone(&self.observations);
        let portal_observations = Rc::clone(&self.observations);
        let backdrop_observations = Rc::clone(&self.observations);
        let positioner_observations = Rc::clone(&self.observations);
        let popup_observations = Rc::clone(&self.observations);
        let arrow_observations = Rc::clone(&self.observations);
        let viewport_observations = Rc::clone(&self.observations);
        let payload_observations = Rc::clone(&self.observations);
        let outside_observations = Rc::clone(&self.observations);
        let cancel_open = self.config.cancel_open;
        let cancel_close = self.config.cancel_close;
        let prevent_unmount_on_close = self.config.prevent_unmount_on_close;

        let mut root = PreviewCardRoot::<&'static str>::new()
            .id("preview-card-test")
            .default_open(self.config.default_open)
            .on_open_change(move |open, details, _window, _cx| {
                open_observations
                    .borrow_mut()
                    .open_changes
                    .push((open, details.reason()));
                if (open && cancel_open) || (!open && cancel_close) {
                    details.cancel();
                }
                if !open && prevent_unmount_on_close {
                    details.prevent_unmount_on_close();
                }
            })
            .on_open_change_complete(move |open, details, _window, _cx| {
                complete_observations
                    .borrow_mut()
                    .open_change_completes
                    .push((open, details.reason()));
            });
        if let Some(controlled_open) = self.config.controlled_open {
            root = root.open(controlled_open);
        }
        if let Some(handle) = self.config.handle.clone() {
            root = root.handle(handle);
        }

        let mut primary_trigger = PreviewCardTrigger::<&'static str>::new()
            .id("preview-card-trigger")
            .payload("primary")
            .w(px(120.0))
            .h(px(32.0))
            .border_1()
            .border_color(rgb(0xd1d5db))
            .style_with_state(move |state, trigger| {
                trigger_observations.borrow_mut().trigger_states.push(state);
                trigger.debug_selector(|| "preview-card-trigger".into())
            })
            .child("Trigger");
        if let Some(delay) = self.config.delay {
            primary_trigger = primary_trigger.delay(delay);
        }
        if let Some(close_delay) = self.config.close_delay {
            primary_trigger = primary_trigger.close_delay(close_delay);
        }

        let mut root = root.child(primary_trigger);
        if self.config.second_trigger {
            let mut second_trigger = PreviewCardTrigger::<&'static str>::new()
                .id("preview-card-trigger-secondary")
                .payload("secondary")
                .w(px(120.0))
                .h(px(32.0))
                .border_1()
                .border_color(rgb(0xd1d5db))
                .style_with_state(move |state, trigger| {
                    second_trigger_observations
                        .borrow_mut()
                        .second_trigger_states
                        .push(state);
                    trigger.debug_selector(|| "preview-card-trigger-secondary".into())
                })
                .child("Second");
            if let Some(delay) = self.config.delay {
                second_trigger = second_trigger.delay(delay);
            }
            if let Some(close_delay) = self.config.close_delay {
                second_trigger = second_trigger.close_delay(close_delay);
            }
            root = root.child(second_trigger);
        }

        let mut portal = PreviewCardPortal::<&'static str>::new()
            .keep_mounted(self.config.keep_mounted)
            .style_with_state(move |state, portal| {
                portal_observations.borrow_mut().portal_states.push(state);
                portal.debug_selector(|| "preview-card-portal".into())
            });
        if self.config.backdrop {
            portal = portal.child(PreviewCardBackdrop::<&'static str>::new().style_with_state(
                move |state, backdrop| {
                    backdrop_observations
                        .borrow_mut()
                        .backdrop_states
                        .push(state);
                    backdrop
                        .absolute()
                        .top(px(0.0))
                        .left(px(0.0))
                        .w(px(640.0))
                        .h(px(360.0))
                        .debug_selector(|| "preview-card-backdrop".into())
                },
            ));
        }
        let portal = portal.child(
            PreviewCardPositioner::<&'static str>::new()
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
                    PreviewCardPopup::<&'static str>::new()
                        .w(px(180.0))
                        .h(px(72.0))
                        .bg(rgb(0xffffff))
                        .style_with_state(move |state, popup| {
                            popup_observations.borrow_mut().popup_states.push(state);
                            popup.debug_selector(|| "preview-card-popup".into())
                        })
                        .child(PreviewCardArrow::<&'static str>::new().style_with_state(
                            move |state, arrow| {
                                arrow_observations.borrow_mut().arrow_states.push(state);
                                arrow
                            },
                        ))
                        .child(
                            PreviewCardViewport::<&'static str>::new()
                                .style_with_state(move |state, viewport| {
                                    viewport_observations
                                        .borrow_mut()
                                        .viewport_states
                                        .push(state);
                                    viewport
                                })
                                .payload_content(move |payload, _window, _cx| {
                                    payload_observations
                                        .borrow_mut()
                                        .rendered_payloads
                                        .push(payload.copied());
                                    div()
                                        .child(payload.copied().unwrap_or("no payload"))
                                        .into_any_element()
                                }),
                        ),
                ),
        );

        div()
            .size_full()
            .p_4()
            .child(root.flex().gap_2().child(portal))
            .child(
                div()
                    .absolute()
                    .top(px(280.0))
                    .left(px(16.0))
                    .w(px(100.0))
                    .h(px(32.0))
                    .id("preview-card-outside-target")
                    .focusable()
                    .debug_selector(|| "preview-card-outside-target".into())
                    .on_click(move |_event, _window, _cx| {
                        outside_observations.borrow_mut().outside_clicks += 1;
                    }),
            )
    }
}

pub fn open_preview_card(
    cx: &mut TestAppContext,
    config: PreviewCardTestConfig,
) -> WindowHandle<PreviewCardTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        PreviewCardTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
) -> PreviewCardObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("preview card test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("preview card test window should be open")
}

pub fn hover_trigger(cx: &mut TestAppContext, window: WindowHandle<PreviewCardTestView>) {
    move_over_selector(cx, window, "preview-card-trigger");
}

pub fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn move_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    move_mouse_to(cx, window, bounds.center());
}

pub fn move_mouse_to(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
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

pub fn focus_next(cx: &mut TestAppContext, window: WindowHandle<PreviewCardTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("preview card test window should be open");
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<PreviewCardTestView>) {
    window
        .update(cx, |_view, window, _cx| {
            window.blur();
        })
        .expect("preview card test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
    keys: &str,
) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<PreviewCardTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
