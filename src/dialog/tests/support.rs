use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::dialog::{
    init, DialogBackdrop, DialogBackdropStyleState, DialogClose, DialogCloseStyleState,
    DialogDescription, DialogDescriptionStyleState, DialogHandle, DialogModalMode,
    DialogOpenChangeReason, DialogPopup, DialogPopupStyleState, DialogPortal,
    DialogPortalStyleState, DialogRoot, DialogRootStyleState, DialogTitle, DialogTitleStyleState,
    DialogTrigger, DialogTriggerStyleState, DialogViewport, DialogViewportStyleState,
};

#[derive(Clone)]
pub struct DialogTestConfig {
    pub default_open: bool,
    pub trigger_disabled: bool,
    pub keep_mounted: bool,
    pub controlled_open: Option<bool>,
    pub modal_mode: DialogModalMode,
    pub disable_pointer_dismissal: bool,
    pub handle: Option<DialogHandle<usize>>,
}

impl Default for DialogTestConfig {
    fn default() -> Self {
        Self {
            default_open: false,
            trigger_disabled: false,
            keep_mounted: false,
            controlled_open: None,
            modal_mode: DialogModalMode::Modal,
            disable_pointer_dismissal: false,
            handle: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct DialogObservations {
    pub open_changes: Vec<(bool, DialogOpenChangeReason, Option<usize>)>,
    pub root_states: Vec<DialogRootStyleState<usize>>,
    pub trigger_states: Vec<DialogTriggerStyleState<usize>>,
    pub portal_states: Vec<DialogPortalStyleState>,
    pub backdrop_states: Vec<DialogBackdropStyleState>,
    pub viewport_states: Vec<DialogViewportStyleState<usize>>,
    pub popup_states: Vec<DialogPopupStyleState<usize>>,
    pub title_states: Vec<DialogTitleStyleState<usize>>,
    pub description_states: Vec<DialogDescriptionStyleState<usize>>,
    pub close_states: Vec<DialogCloseStyleState>,
}

impl DialogObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
        self.portal_states.clear();
        self.backdrop_states.clear();
        self.viewport_states.clear();
        self.popup_states.clear();
        self.title_states.clear();
        self.description_states.clear();
        self.close_states.clear();
    }

    pub fn root_state(&self) -> Option<DialogRootStyleState<usize>> {
        self.root_states.last().cloned()
    }

    pub fn trigger_state(&self) -> Option<DialogTriggerStyleState<usize>> {
        self.trigger_states.last().cloned()
    }

    pub fn popup_state(&self) -> Option<DialogPopupStyleState<usize>> {
        self.popup_states.last().cloned()
    }
}

pub struct DialogTestView {
    pub config: DialogTestConfig,
    observations: Rc<RefCell<DialogObservations>>,
}

impl DialogTestView {
    pub fn new(config: DialogTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(DialogObservations::default())),
        }
    }

    pub fn read_observations(&self) -> DialogObservations {
        self.observations.borrow().clone()
    }
}

impl Render for DialogTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let root_observations = Rc::clone(&self.observations);
        let open_observations = Rc::clone(&self.observations);
        let trigger_observations = Rc::clone(&self.observations);
        let portal_observations = Rc::clone(&self.observations);
        let backdrop_observations = Rc::clone(&self.observations);
        let viewport_observations = Rc::clone(&self.observations);
        let popup_observations = Rc::clone(&self.observations);
        let title_observations = Rc::clone(&self.observations);
        let description_observations = Rc::clone(&self.observations);
        let close_observations = Rc::clone(&self.observations);

        let mut root = DialogRoot::<usize>::new()
            .id("dialog-test")
            .default_open(self.config.default_open)
            .modal_mode(self.config.modal_mode)
            .disable_pointer_dismissal(self.config.disable_pointer_dismissal)
            .on_open_change(move |open, details, _window, _cx| {
                open_observations.borrow_mut().open_changes.push((
                    open,
                    details.reason(),
                    details.payload().copied(),
                ));
            })
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root.debug_selector(|| "dialog-root".into())
            });
        if let Some(controlled_open) = self.config.controlled_open {
            root = root.open(controlled_open);
        }
        if let Some(handle) = self.config.handle.clone() {
            root = root.handle(handle);
        }

        div().size_full().p_4().child(
            root.child(
                DialogTrigger::<usize>::new()
                    .id("dialog-trigger")
                    .payload(7)
                    .disabled(self.config.trigger_disabled)
                    .w(px(120.0))
                    .h(px(32.0))
                    .border_1()
                    .border_color(rgb(0xd1d5db))
                    .style_with_state(move |state, trigger| {
                        trigger_observations.borrow_mut().trigger_states.push(state);
                        trigger.debug_selector(|| "dialog-trigger".into())
                    })
                    .child("Open"),
            )
            .child(
                DialogPortal::<usize>::new()
                    .keep_mounted(self.config.keep_mounted)
                    .style_with_state(move |state, portal| {
                        portal_observations.borrow_mut().portal_states.push(state);
                        portal
                    })
                    .child(DialogBackdrop::<usize>::new().style_with_state(
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
                                .debug_selector(|| "dialog-backdrop".into())
                        },
                    ))
                    .child(
                        DialogViewport::<usize>::new()
                            .absolute()
                            .top(px(64.0))
                            .left(px(64.0))
                            .style_with_state(move |state, viewport| {
                                viewport_observations
                                    .borrow_mut()
                                    .viewport_states
                                    .push(state);
                                viewport
                            })
                            .child(
                                DialogPopup::<usize>::new()
                                    .w(px(180.0))
                                    .h(px(112.0))
                                    .bg(rgb(0xffffff))
                                    .style_with_state(move |state, popup| {
                                        popup_observations.borrow_mut().popup_states.push(state);
                                        popup.debug_selector(|| "dialog-popup".into())
                                    })
                                    .child(
                                        DialogTitle::<usize>::new()
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
                                        DialogDescription::<usize>::new()
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
                                        DialogClose::<usize>::new()
                                            .style_with_state(move |state, close| {
                                                close_observations
                                                    .borrow_mut()
                                                    .close_states
                                                    .push(state);
                                                close.debug_selector(|| "dialog-close".into())
                                            })
                                            .child("Close"),
                                    ),
                            ),
                    ),
            ),
        )
    }
}

pub fn open_dialog(
    cx: &mut TestAppContext,
    config: DialogTestConfig,
) -> WindowHandle<DialogTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        DialogTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<DialogTestView>,
) -> DialogObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("dialog test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("dialog test window should be open")
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<DialogTestView>) {
    click_selector(cx, window, "dialog-trigger");
}

pub fn click_close(cx: &mut TestAppContext, window: WindowHandle<DialogTestView>) {
    click_selector(cx, window, "dialog-close");
}

pub fn click_backdrop(cx: &mut TestAppContext, window: WindowHandle<DialogTestView>) {
    click_selector(cx, window, "dialog-backdrop");
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<DialogTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<DialogTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<DialogTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}
