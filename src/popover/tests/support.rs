use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::popover::{
    init, PopoverArrow, PopoverArrowStyleState, PopoverBackdropStyleState, PopoverClose,
    PopoverDescription, PopoverDescriptionStyleState, PopoverHandle, PopoverOpenChangeReason,
    PopoverPopup, PopoverPopupStyleState, PopoverPortal, PopoverPositioner,
    PopoverPositionerStyleState, PopoverRoot, PopoverRootStyleState, PopoverSide, PopoverTitle,
    PopoverTitleStyleState, PopoverTrigger, PopoverTriggerStyleState, PopoverViewport,
    PopoverViewportStyleState,
};

#[derive(Clone, Default)]
pub struct PopoverTestConfig {
    pub default_open: bool,
    pub trigger_disabled: bool,
    pub modal: bool,
    pub keep_mounted: bool,
    pub open_on_hover: bool,
    pub delay: Duration,
    pub close_delay: Duration,
    pub positioner_side: PopoverSide,
    pub controlled_open: Option<bool>,
    pub handle: Option<PopoverHandle<()>>,
    pub second_trigger: bool,
}

#[derive(Clone, Default)]
pub struct PopoverObservations {
    pub open_changes: Vec<(bool, PopoverOpenChangeReason)>,
    pub root_states: Vec<PopoverRootStyleState<()>>,
    pub trigger_states: Vec<PopoverTriggerStyleState<()>>,
    pub positioner_states: Vec<PopoverPositionerStyleState>,
    pub popup_states: Vec<PopoverPopupStyleState>,
    pub backdrop_states: Vec<PopoverBackdropStyleState>,
    pub arrow_states: Vec<PopoverArrowStyleState>,
    pub title_states: Vec<PopoverTitleStyleState>,
    pub description_states: Vec<PopoverDescriptionStyleState>,
    pub viewport_states: Vec<PopoverViewportStyleState>,
    pub outside_clicks: usize,
}

impl PopoverObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
        self.positioner_states.clear();
        self.popup_states.clear();
        self.backdrop_states.clear();
        self.arrow_states.clear();
        self.title_states.clear();
        self.description_states.clear();
        self.viewport_states.clear();
    }

    pub fn root_state(&self) -> Option<PopoverRootStyleState<()>> {
        self.root_states.last().cloned()
    }

    pub fn trigger_state(&self) -> Option<PopoverTriggerStyleState<()>> {
        self.trigger_states.last().cloned()
    }

    pub fn popup_state(&self) -> Option<PopoverPopupStyleState> {
        self.popup_states.last().copied()
    }
}

pub struct PopoverTestView {
    pub config: PopoverTestConfig,
    observations: Rc<RefCell<PopoverObservations>>,
}

impl PopoverTestView {
    pub fn new(config: PopoverTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(PopoverObservations::default())),
        }
    }

    pub fn read_observations(&self) -> PopoverObservations {
        self.observations.borrow().clone()
    }
}

impl Render for PopoverTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let root_observations = Rc::clone(&self.observations);
        let open_observations = Rc::clone(&self.observations);
        let trigger_observations = Rc::clone(&self.observations);
        let positioner_observations = Rc::clone(&self.observations);
        let popup_observations = Rc::clone(&self.observations);
        let backdrop_observations = Rc::clone(&self.observations);
        let arrow_observations = Rc::clone(&self.observations);
        let title_observations = Rc::clone(&self.observations);
        let description_observations = Rc::clone(&self.observations);
        let viewport_observations = Rc::clone(&self.observations);
        let outside_observations = Rc::clone(&self.observations);

        let mut root = PopoverRoot::<()>::new()
            .id("popover-test")
            .default_open(self.config.default_open)
            .modal(self.config.modal)
            .on_open_change(move |open, details, _window, _cx| {
                open_observations
                    .borrow_mut()
                    .open_changes
                    .push((open, details.reason()));
            })
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "popover-root".into())
            });
        if let Some(controlled_open) = self.config.controlled_open {
            root = root.open(controlled_open);
        }
        if let Some(handle) = self.config.handle.clone() {
            root = root.handle(handle);
        }

        div()
            .size_full()
            .p_4()
            .child(
                root.child(
                    PopoverTrigger::<()>::new()
                        .id("popover-trigger")
                        .disabled(self.config.trigger_disabled)
                        .open_on_hover(self.config.open_on_hover)
                        .delay(self.config.delay)
                        .close_delay(self.config.close_delay)
                        .w(px(120.0))
                        .h(px(32.0))
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .style_with_state(move |state, trigger| {
                            trigger_observations.borrow_mut().trigger_states.push(state);
                            trigger.debug_selector(|| "popover-trigger".into())
                        })
                        .child("Open"),
                )
                .child(
                    PopoverTrigger::<()>::new()
                        .id("popover-trigger-secondary")
                        .disabled(!self.config.second_trigger)
                        .w(px(120.0))
                        .h(px(32.0))
                        .border_1()
                        .border_color(rgb(0xd1d5db))
                        .style_with_state(|_, trigger| {
                            trigger.debug_selector(|| "popover-trigger-secondary".into())
                        })
                        .child("More"),
                )
                .child(
                    PopoverPortal::<()>::new()
                        .keep_mounted(self.config.keep_mounted)
                        .child(
                            PopoverPositioner::<()>::new()
                                .side(self.config.positioner_side)
                                .side_offset(px(4.0))
                                .style_with_state(move |state, positioner| {
                                    positioner_observations
                                        .borrow_mut()
                                        .positioner_states
                                        .push(state);
                                    positioner
                                })
                                .child(
                                    PopoverPopup::<()>::new()
                                        .w(px(180.0))
                                        .h(px(96.0))
                                        .bg(rgb(0xffffff))
                                        .style_with_state(move |state, popup| {
                                            popup_observations
                                                .borrow_mut()
                                                .popup_states
                                                .push(state);
                                            popup.debug_selector(|| "popover-popup".into())
                                        })
                                        .child(PopoverArrow::<()>::new().style_with_state(
                                            move |state, arrow| {
                                                arrow_observations
                                                    .borrow_mut()
                                                    .arrow_states
                                                    .push(state);
                                                arrow
                                            },
                                        ))
                                        .child(
                                            PopoverTitle::<()>::new()
                                                .style_with_state(move |state, title| {
                                                    title_observations
                                                        .borrow_mut()
                                                        .title_states
                                                        .push(state);
                                                    title
                                                })
                                                .child("Title"),
                                        )
                                        .child(
                                            PopoverDescription::<()>::new()
                                                .style_with_state(move |state, description| {
                                                    description_observations
                                                        .borrow_mut()
                                                        .description_states
                                                        .push(state);
                                                    description
                                                })
                                                .child("Description"),
                                        )
                                        .child(
                                            PopoverViewport::<()>::new()
                                                .style_with_state(move |state, viewport| {
                                                    viewport_observations
                                                        .borrow_mut()
                                                        .viewport_states
                                                        .push(state);
                                                    viewport
                                                })
                                                .child("Viewport"),
                                        )
                                        .child(
                                            PopoverClose::<()>::new()
                                                .style_with_state(|_, close| {
                                                    close.debug_selector(|| "popover-close".into())
                                                })
                                                .child("Close"),
                                        ),
                                ),
                        ),
                )
                .child(
                    crate::popover::PopoverBackdrop::<()>::new().style_with_state(
                        move |state, backdrop| {
                            backdrop_observations
                                .borrow_mut()
                                .backdrop_states
                                .push(state);
                            backdrop
                                .absolute()
                                .top_0()
                                .left_0()
                                .w_full()
                                .h_full()
                                .debug_selector(|| "popover-backdrop".into())
                        },
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
                    .id("popover-outside-target")
                    .focusable()
                    .debug_selector(|| "popover-outside-target".into())
                    .on_click(move |_event, _window, _cx| {
                        outside_observations.borrow_mut().outside_clicks += 1;
                    }),
            )
    }
}

pub fn open_popover(
    cx: &mut TestAppContext,
    config: PopoverTestConfig,
) -> WindowHandle<PopoverTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        PopoverTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<PopoverTestView>,
) -> PopoverObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("popover test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("popover test window should be open")
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>) {
    click_selector(cx, window, "popover-trigger");
}

pub fn click_close(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>) {
    click_selector(cx, window, "popover-close");
}

pub fn click_outside_target(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>) {
    click_selector(cx, window, "popover-outside-target");
}

pub fn move_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<PopoverTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_mouse_move(bounds.center(), None, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn advance_clock(cx: &mut TestAppContext, duration: Duration) {
    cx.executor().advance_clock(duration);
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn focus_next(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("popover test window should be open");
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>) {
    window
        .update(cx, |_view, window, _cx| {
            window.blur();
        })
        .expect("popover test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<PopoverTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<PopoverTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<PopoverTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}
