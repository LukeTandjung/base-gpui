use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::accordion::{
    init, AccordionChangeReason, AccordionChangeSource, AccordionHeader, AccordionHeaderStyleState,
    AccordionItem, AccordionItemStyleState, AccordionPanel, AccordionPanelStyleState,
    AccordionRoot, AccordionRootStyleState, AccordionTrigger, AccordionTriggerStyleState,
};

pub const FIRST: &str = "first";
pub const SECOND: &str = "second";

#[derive(Clone)]
pub struct AccordionTestConfig {
    pub root_id: &'static str,
    pub default_values: Vec<&'static str>,
    pub controlled_values: Option<Vec<&'static str>>,
    pub multiple: bool,
    pub disabled_root: bool,
    pub disabled_first: bool,
    pub disabled_second: bool,
    pub keep_mounted_root: bool,
    pub first_panel_keep_mounted: Option<bool>,
    pub second_panel_keep_mounted: Option<bool>,
    pub include_first_panel: bool,
    pub include_second_item: bool,
    pub second_value: Option<&'static str>,
    pub cancel_item_changes: bool,
    pub cancel_root_changes: bool,
}

impl Default for AccordionTestConfig {
    fn default() -> Self {
        Self {
            root_id: "accordion-test",
            default_values: Vec::new(),
            controlled_values: None,
            multiple: false,
            disabled_root: false,
            disabled_first: false,
            disabled_second: false,
            keep_mounted_root: false,
            first_panel_keep_mounted: None,
            second_panel_keep_mounted: None,
            include_first_panel: true,
            include_second_item: true,
            second_value: None,
            cancel_item_changes: false,
            cancel_root_changes: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct AccordionObservations {
    pub root_value_changes: Vec<Vec<&'static str>>,
    pub item_open_changes: Vec<(&'static str, bool)>,
    pub change_reasons: Vec<AccordionChangeReason>,
    pub change_sources: Vec<AccordionChangeSource>,
    pub change_cancelable: Vec<bool>,
    pub change_canceled: Vec<bool>,
    pub root_states: Vec<AccordionRootStyleState<&'static str>>,
    pub item_states: Vec<AccordionItemStyleState<&'static str>>,
    pub header_states: Vec<AccordionHeaderStyleState<&'static str>>,
    pub trigger_states: Vec<AccordionTriggerStyleState<&'static str>>,
    pub panel_states: Vec<AccordionPanelStyleState<&'static str>>,
}

impl AccordionObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.item_states.clear();
        self.header_states.clear();
        self.trigger_states.clear();
        self.panel_states.clear();
    }

    pub fn last_root_state(&self) -> AccordionRootStyleState<&'static str> {
        self.root_states
            .last()
            .cloned()
            .expect("accordion root state should be observed")
    }

    pub fn item_state_at(&self, index: usize) -> AccordionItemStyleState<&'static str> {
        self.item_states
            .iter()
            .find(|state| state.index == index)
            .cloned()
            .expect("accordion item state should be observed")
    }

    pub fn trigger_state_at(&self, index: usize) -> AccordionTriggerStyleState<&'static str> {
        self.trigger_states
            .iter()
            .find(|state| state.item.index == index)
            .cloned()
            .expect("accordion trigger state should be observed")
    }

    pub fn panel_state_at(&self, index: usize) -> Option<AccordionPanelStyleState<&'static str>> {
        self.panel_states
            .iter()
            .find(|state| state.item.index == index)
            .cloned()
    }

    pub fn header_state_at(&self, index: usize) -> AccordionHeaderStyleState<&'static str> {
        self.header_states
            .iter()
            .find(|state| state.item.index == index)
            .cloned()
            .expect("accordion header state should be observed")
    }
}

pub struct AccordionTestView {
    pub config: AccordionTestConfig,
    observations: Rc<RefCell<AccordionObservations>>,
}

impl AccordionTestView {
    pub fn new(config: AccordionTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(AccordionObservations::default())),
        }
    }

    pub fn read_observations(&self) -> AccordionObservations {
        self.observations.borrow().clone()
    }
}

impl Render for AccordionTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = AccordionRoot::new()
            .id(self.config.root_id)
            .default_value(self.config.default_values.clone())
            .multiple(self.config.multiple)
            .disabled(self.config.disabled_root)
            .keep_mounted(self.config.keep_mounted_root)
            .flex()
            .flex_col()
            .gap_2();

        if let Some(values) = self.config.controlled_values.clone() {
            root = root.value(values);
        }

        let cancel_root_changes = self.config.cancel_root_changes;
        let root_change_observations = Rc::clone(&self.observations);
        root = root.on_value_change(move |next, details, _window, _cx| {
            let mut observations = root_change_observations.borrow_mut();
            observations.root_value_changes.push(next.to_vec());
            observations.change_reasons.push(details.reason());
            observations.change_sources.push(details.source());
            observations.change_cancelable.push(details.cancelable());

            if cancel_root_changes {
                details.cancel();
            }

            observations.change_canceled.push(details.is_canceled());
        });

        let root_state_observations = Rc::clone(&self.observations);
        root = root.style_with_state(move |state, root| {
            root_state_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "accordion-root".into())
        });

        root = root.child(self.render_item(
            FIRST,
            0,
            self.config.disabled_first,
            self.config.first_panel_keep_mounted,
            self.config.include_first_panel,
        ));

        if self.config.include_second_item {
            root = root.child(self.render_item(
                self.config.second_value.unwrap_or(SECOND),
                1,
                self.config.disabled_second,
                self.config.second_panel_keep_mounted,
                true,
            ));
        }

        div().size_full().p_4().child(root)
    }
}

impl AccordionTestView {
    fn render_item(
        &self,
        value: &'static str,
        index: usize,
        disabled: bool,
        panel_keep_mounted: Option<bool>,
        include_panel: bool,
    ) -> AccordionItem<&'static str> {
        let selector_suffix = if index == 0 { "first" } else { "second" };
        let cancel_item_changes = self.config.cancel_item_changes;
        let item_change_observations = Rc::clone(&self.observations);
        let item_state_observations = Rc::clone(&self.observations);
        let header_state_observations = Rc::clone(&self.observations);
        let trigger_state_observations = Rc::clone(&self.observations);

        let mut item = AccordionItem::new(value)
            .disabled(disabled)
            .on_open_change(move |next_open, details, _window, _cx| {
                let mut observations = item_change_observations.borrow_mut();
                observations.item_open_changes.push((value, next_open));

                if cancel_item_changes {
                    details.cancel();
                }
            })
            .style_with_state(move |state, item| {
                item_state_observations.borrow_mut().item_states.push(state);
                item.debug_selector(move || format!("accordion-item-{selector_suffix}").into())
            });

        let header = AccordionHeader::new()
            .style_with_state(move |state, header| {
                header_state_observations
                    .borrow_mut()
                    .header_states
                    .push(state);
                header.debug_selector(move || format!("accordion-header-{selector_suffix}").into())
            })
            .child(
                AccordionTrigger::new()
                    .id(format!("accordion-trigger-{selector_suffix}"))
                    .w(px(140.0))
                    .h(px(28.0))
                    .style_with_state(move |state, trigger| {
                        trigger_state_observations
                            .borrow_mut()
                            .trigger_states
                            .push(state);
                        trigger.debug_selector(move || {
                            format!("accordion-trigger-{selector_suffix}").into()
                        })
                    })
                    .child(if index == 0 { "Trigger 1" } else { "Trigger 2" }),
            );
        item = item.child(header);

        if include_panel {
            let panel_state_observations = Rc::clone(&self.observations);
            let mut panel = AccordionPanel::new()
                .w(px(160.0))
                .h(px(32.0))
                .style_with_state(move |state, panel| {
                    panel_state_observations
                        .borrow_mut()
                        .panel_states
                        .push(state);
                    panel
                        .debug_selector(move || format!("accordion-panel-{selector_suffix}").into())
                })
                .child(if index == 0 { "Panel 1" } else { "Panel 2" });

            if let Some(keep_mounted) = panel_keep_mounted {
                panel = panel.keep_mounted(keep_mounted);
            }

            item = item.child(panel);
        }

        item
    }
}

pub fn open_accordion(
    cx: &mut TestAppContext,
    config: AccordionTestConfig,
) -> WindowHandle<AccordionTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(360.0), px(220.0)), move |_, _| {
        AccordionTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<AccordionTestView>,
) -> AccordionObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("accordion test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<AccordionTestView>,
    update: impl FnOnce(&mut AccordionTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("accordion test window should be open");
    cx.run_until_parked();
}

pub fn click_trigger(
    cx: &mut TestAppContext,
    window: WindowHandle<AccordionTestView>,
    selector_suffix: &'static str,
) {
    let selector = match selector_suffix {
        "first" => "accordion-trigger-first",
        "second" => "accordion-trigger-second",
        _ => panic!("unknown accordion trigger selector suffix"),
    };
    let bounds = debug_bounds(cx, window, selector).expect("accordion trigger should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn focus_trigger(
    cx: &mut TestAppContext,
    window: WindowHandle<AccordionTestView>,
    index: usize,
) {
    window
        .update(cx, |_view, window, cx| {
            for _ in 0..=index {
                window.focus_next(cx);
            }
        })
        .expect("accordion test window should be open");
    cx.run_until_parked();
}

pub fn blur_trigger(cx: &mut TestAppContext, window: WindowHandle<AccordionTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("accordion test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<AccordionTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<AccordionTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
