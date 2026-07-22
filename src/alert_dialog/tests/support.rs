use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::alert_dialog::{
    AlertDialogBackdrop, AlertDialogClose, AlertDialogDescription, AlertDialogHandle,
    AlertDialogPopup, AlertDialogPortal, AlertDialogRoot, AlertDialogTitle, AlertDialogTrigger,
    AlertDialogViewport,
};
use crate::dialog::{DialogOpenChangeReason, DialogRootStyleState, DialogTriggerStyleState};

#[derive(Clone, Default)]
pub struct AlertDialogTestConfig {
    pub default_open: bool,
    pub trigger_disabled: bool,
    pub controlled_open: Option<bool>,
    pub handle: Option<AlertDialogHandle<usize>>,
    pub detached_trigger: bool,
}

#[derive(Clone, Default)]
pub struct AlertDialogObservations {
    pub open_changes: Vec<(bool, DialogOpenChangeReason, Option<usize>)>,
    pub root_states: Vec<DialogRootStyleState<usize>>,
    pub trigger_states: Vec<DialogTriggerStyleState<usize>>,
}

impl AlertDialogObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
    }

    pub fn root_state(&self) -> Option<DialogRootStyleState<usize>> {
        self.root_states.last().cloned()
    }

    pub fn trigger_state(&self) -> Option<DialogTriggerStyleState<usize>> {
        self.trigger_states.last().cloned()
    }
}

pub struct AlertDialogTestView {
    pub config: AlertDialogTestConfig,
    observations: Rc<RefCell<AlertDialogObservations>>,
}

impl AlertDialogTestView {
    pub fn new(config: AlertDialogTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(AlertDialogObservations::default())),
        }
    }

    pub fn read_observations(&self) -> AlertDialogObservations {
        self.observations.borrow().clone()
    }
}

impl Render for AlertDialogTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let root_observations = Rc::clone(&self.observations);
        let open_observations = Rc::clone(&self.observations);
        let trigger_observations = Rc::clone(&self.observations);

        let mut root = AlertDialogRoot::<usize>::new()
            .id("alert-dialog-test")
            .default_open(self.config.default_open)
            .on_open_change(move |open, details, _window, _cx| {
                open_observations.borrow_mut().open_changes.push((
                    open,
                    details.reason(),
                    details.payload().copied(),
                ));
            })
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "alert-dialog-root".into())
            });
        if let Some(controlled_open) = self.config.controlled_open {
            root = root.open(controlled_open);
        }
        if let Some(handle) = self.config.handle.clone() {
            root = root.handle(handle);
        }

        let detached_trigger = self.config.detached_trigger.then(|| {
            AlertDialogTrigger::<usize>::new()
                .id("detached-trigger")
                .payload(9)
                .handle(
                    self.config
                        .handle
                        .clone()
                        .expect("detached trigger requires a handle"),
                )
                .w(px(120.0))
                .h(px(32.0))
                .style_with_state(|_, trigger| {
                    trigger.debug_selector(|| "alert-dialog-detached-trigger".into())
                })
                .child("Detached")
        });

        div().size_full().p_4().children(detached_trigger).child(
            root.child(
                AlertDialogTrigger::<usize>::new()
                    .id("alert-dialog-trigger")
                    .payload(7)
                    .disabled(self.config.trigger_disabled)
                    .w(px(120.0))
                    .h(px(32.0))
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .style_with_state(move |state, trigger| {
                        trigger_observations.borrow_mut().trigger_states.push(state);
                        trigger.debug_selector(|| "alert-dialog-trigger".into())
                    })
                    .child("Open"),
            )
            .child(
                AlertDialogPortal::<usize>::new()
                    .child(
                        AlertDialogBackdrop::<usize>::new().style_with_state(|_, backdrop| {
                            backdrop
                                .absolute()
                                .top_0()
                                .left_0()
                                .w_full()
                                .h_full()
                                .debug_selector(|| "alert-dialog-backdrop".into())
                        }),
                    )
                    .child(
                        AlertDialogViewport::<usize>::new()
                            .absolute()
                            .top(px(64.0))
                            .left(px(64.0))
                            .child(
                                AlertDialogPopup::<usize>::new()
                                    .w(px(180.0))
                                    .h(px(112.0))
                                    .bg(rgb(0xffffff))
                                    .style_with_state(|_, popup| {
                                        popup.debug_selector(|| "alert-dialog-popup".into())
                                    })
                                    .child(AlertDialogTitle::<usize>::new().child("Discard?"))
                                    .child(
                                        AlertDialogDescription::<usize>::new()
                                            .child("This cannot be undone."),
                                    )
                                    .child(
                                        AlertDialogClose::<usize>::new()
                                            .style_with_state(|_, close| {
                                                close.debug_selector(|| "alert-dialog-close".into())
                                            })
                                            .child("Cancel"),
                                    ),
                            ),
                    ),
            ),
        )
    }
}

pub fn open_alert_dialog(
    cx: &mut TestAppContext,
    config: AlertDialogTestConfig,
) -> WindowHandle<AlertDialogTestView> {
    cx.update(crate::dialog::init);
    cx.update(crate::alert_dialog::init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        AlertDialogTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<AlertDialogTestView>,
) -> AlertDialogObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("alert dialog test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("alert dialog test window should be open")
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<AlertDialogTestView>) {
    click_selector(cx, window, "alert-dialog-trigger");
}

pub fn click_detached_trigger(cx: &mut TestAppContext, window: WindowHandle<AlertDialogTestView>) {
    click_selector(cx, window, "alert-dialog-detached-trigger");
}

pub fn click_close(cx: &mut TestAppContext, window: WindowHandle<AlertDialogTestView>) {
    click_selector(cx, window, "alert-dialog-close");
}

pub fn click_backdrop(cx: &mut TestAppContext, window: WindowHandle<AlertDialogTestView>) {
    click_selector(cx, window, "alert-dialog-backdrop");
}

pub fn simulate_keys(
    cx: &mut TestAppContext,
    window: WindowHandle<AlertDialogTestView>,
    keys: &str,
) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<AlertDialogTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<AlertDialogTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}
