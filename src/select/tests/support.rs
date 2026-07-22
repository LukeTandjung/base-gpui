use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, point, prelude::*, px, rgb, size, Bounds, Modifiers, Pixels, Render, ScrollDelta,
    ScrollWheelEvent, TestAppContext, TouchPhase, VisualTestContext, WindowHandle,
};

use crate::select::{
    init, SelectGroup, SelectGroupLabel, SelectGroupStyleState, SelectIcon, SelectItem,
    SelectItemIndicator, SelectItemIndicatorStyleState, SelectItemStyleState, SelectItemText,
    SelectLabel, SelectList, SelectPositionerStyleState, SelectRoot, SelectRootStyleState,
    SelectScrollArrowDirection, SelectScrollArrowStyleState, SelectScrollDownArrow,
    SelectScrollUpArrow, SelectSeparator, SelectTrigger, SelectTriggerStyleState, SelectValue,
    SelectValueStyleState,
};

pub type SelectTestValue = &'static str;

pub const APPLE: SelectTestValue = "apple";
pub const BANANA: SelectTestValue = "banana";
pub const ORANGE: SelectTestValue = "orange";

#[derive(Clone)]
pub struct SelectTestConfig {
    pub default_value: Option<SelectTestValue>,
    pub controlled_value: Option<Option<SelectTestValue>>,
    pub default_values: Vec<SelectTestValue>,
    pub controlled_values: Option<Vec<SelectTestValue>>,
    pub default_open: bool,
    pub multiple: bool,
    pub include_group_label_and_separator: bool,
    pub wrap_items_in_group: bool,
    pub include_scroll_arrows: bool,
    pub disabled_values: Vec<SelectTestValue>,
    pub modal: bool,
}

impl Default for SelectTestConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            controlled_value: None,
            default_values: Vec::new(),
            controlled_values: None,
            default_open: false,
            multiple: false,
            include_group_label_and_separator: true,
            wrap_items_in_group: false,
            include_scroll_arrows: false,
            disabled_values: Vec::new(),
            modal: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct SelectObservations {
    pub value_changes: Vec<Option<SelectTestValue>>,
    pub values_changes: Vec<Vec<SelectTestValue>>,
    pub open_changes: Vec<bool>,
    pub root_states: Vec<SelectRootStyleState<SelectTestValue>>,
    pub trigger_states: Vec<SelectTriggerStyleState<SelectTestValue>>,
    pub value_states: Vec<SelectValueStyleState<SelectTestValue>>,
    pub positioner_states: Vec<SelectPositionerStyleState>,
    pub group_states: Vec<SelectGroupStyleState>,
    pub scroll_arrow_states: Vec<SelectScrollArrowStyleState>,
    pub outside_clicks: usize,
    pub outside_scrolls: usize,
    pub item_states: Vec<(SelectTestValue, SelectItemStyleState<SelectTestValue>)>,
    pub indicator_states: Vec<(SelectTestValue, SelectItemIndicatorStyleState)>,
}

impl SelectObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.trigger_states.clear();
        self.value_states.clear();
        self.positioner_states.clear();
        self.group_states.clear();
        self.scroll_arrow_states.clear();
        self.item_states.clear();
        self.indicator_states.clear();
    }

    pub fn root_state(&self) -> Option<SelectRootStyleState<SelectTestValue>> {
        self.root_states.last().cloned()
    }

    pub fn value_state(&self) -> Option<SelectValueStyleState<SelectTestValue>> {
        self.value_states.last().cloned()
    }

    pub fn positioner_state(&self) -> Option<SelectPositionerStyleState> {
        self.positioner_states.last().cloned()
    }

    pub fn group_state(&self) -> Option<SelectGroupStyleState> {
        self.group_states.last().cloned()
    }

    pub fn scroll_arrow_state(
        &self,
        direction: SelectScrollArrowDirection,
    ) -> Option<SelectScrollArrowStyleState> {
        self.scroll_arrow_states
            .iter()
            .rev()
            .find_map(|state| (state.direction == direction).then_some(*state))
    }

    pub fn item_state(
        &self,
        value: SelectTestValue,
    ) -> Option<SelectItemStyleState<SelectTestValue>> {
        self.item_states
            .iter()
            .find_map(|(item_value, state)| (*item_value == value).then(|| state.clone()))
    }

    pub fn latest_indicator(
        &self,
        value: SelectTestValue,
    ) -> Option<SelectItemIndicatorStyleState> {
        self.indicator_states
            .iter()
            .rev()
            .find_map(|(item_value, state)| (*item_value == value).then_some(*state))
    }
}

pub struct SelectTestView {
    pub config: SelectTestConfig,
    observations: Rc<RefCell<SelectObservations>>,
}

impl SelectTestView {
    pub fn new(config: SelectTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(SelectObservations::default())),
        }
    }

    pub fn read_observations(&self) -> SelectObservations {
        self.observations.borrow().clone()
    }

    fn label() -> SelectLabel<SelectTestValue> {
        SelectLabel::new()
            .style_with_state(|_, label| label.debug_selector(|| "select-label".into()))
            .child("Fruit")
    }

    fn trigger(observations: &Rc<RefCell<SelectObservations>>) -> SelectTrigger<SelectTestValue> {
        let trigger_observations = Rc::clone(observations);
        let value_observations = Rc::clone(observations);

        SelectTrigger::new()
            .id("select-trigger")
            .w(px(180.0))
            .h(px(32.0))
            .px_2()
            .border_1()
            .border_color(rgb(0xd1d5db))
            .flex()
            .items_center()
            .justify_between()
            .style_with_state(move |state, trigger| {
                trigger_observations.borrow_mut().trigger_states.push(state);
                trigger.debug_selector(|| "select-trigger".into())
            })
            .child(SelectValue::new().placeholder("Choose").style_with_state(
                move |state, value| {
                    value_observations.borrow_mut().value_states.push(state);
                    value.debug_selector(|| "select-value".into())
                },
            ))
            .child(SelectIcon::new())
    }

    fn item(
        value: SelectTestValue,
        label: &'static str,
        disabled: bool,
        observations: &Rc<RefCell<SelectObservations>>,
    ) -> SelectItem<SelectTestValue> {
        let item_observations = Rc::clone(observations);
        let indicator_observations = Rc::clone(observations);

        SelectItem::new()
            .id(format!("select-item-{value}"))
            .value(value)
            .label(label)
            .disabled(disabled)
            .h(px(28.0))
            .px_2()
            .flex()
            .items_center()
            .gap_1()
            .style_with_state(move |state, item| {
                item_observations
                    .borrow_mut()
                    .item_states
                    .push((value, state));
                item.debug_selector(move || item_selector(value).into())
            })
            .child(
                SelectItemIndicator::new()
                    .keep_mounted(true)
                    .style_with_state(move |state, indicator| {
                        indicator_observations
                            .borrow_mut()
                            .indicator_states
                            .push((value, state));
                        indicator
                    }),
            )
            .child(SelectItemText::new().text(label))
    }
}

impl Render for SelectTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = SelectRoot::<SelectTestValue>::new()
            .id("select-test")
            .default_open(self.config.default_open)
            .multiple(self.config.multiple)
            .item_to_string_value(|value| (*value).into())
            .modal(self.config.modal)
            .style_with_state({
                let observations = Rc::clone(&self.observations);
                move |state, root| {
                    observations.borrow_mut().root_states.push(state);
                    root.debug_selector(|| "select-root".into())
                }
            });

        if self.config.multiple {
            root = root.default_values(self.config.default_values.clone());
            if let Some(values) = self.config.controlled_values.clone() {
                root = root.values(values);
            }
        } else {
            root = root.default_value(self.config.default_value);
            if let Some(value) = self.config.controlled_value {
                root = root.value(value);
            }
        }

        let value_change_observations = Rc::clone(&self.observations);
        root = root.on_value_change(move |value, _details, _window, _cx| {
            value_change_observations
                .borrow_mut()
                .value_changes
                .push(value.copied());
        });

        let values_change_observations = Rc::clone(&self.observations);
        root = root.on_values_change(move |values, _details, _window, _cx| {
            values_change_observations
                .borrow_mut()
                .values_changes
                .push(values.to_vec());
        });

        let open_change_observations = Rc::clone(&self.observations);
        root = root.on_open_change(move |open, _details, _window, _cx| {
            open_change_observations
                .borrow_mut()
                .open_changes
                .push(open);
        });

        let mut list = SelectList::new().flex().flex_col().gap_1();
        let apple_disabled = self.config.disabled_values.contains(&APPLE);
        let banana_disabled = self.config.disabled_values.contains(&BANANA);
        let orange_disabled = self.config.disabled_values.contains(&ORANGE);
        if self.config.wrap_items_in_group {
            let group_observations = Rc::clone(&self.observations);
            let group = SelectGroup::new()
                .style_with_state(move |state, group| {
                    group_observations.borrow_mut().group_states.push(state);
                    group
                })
                .child(SelectGroupLabel::new().text("Fruit"))
                .child(Self::item(
                    APPLE,
                    "Apple",
                    apple_disabled,
                    &self.observations,
                ))
                .child(SelectSeparator::new().h(px(1.0)).bg(rgb(0xe5e7eb)))
                .child(Self::item(
                    BANANA,
                    "Banana",
                    banana_disabled,
                    &self.observations,
                ))
                .child(Self::item(
                    ORANGE,
                    "Orange",
                    orange_disabled,
                    &self.observations,
                ));
            list = list.child(group);
        } else {
            list = list.child(Self::item(
                APPLE,
                "Apple",
                apple_disabled,
                &self.observations,
            ));

            if self.config.include_group_label_and_separator {
                list = list
                    .child(SelectGroupLabel::new().text("Fruit"))
                    .child(SelectSeparator::new().h(px(1.0)).bg(rgb(0xe5e7eb)));
            }

            list = list
                .child(Self::item(
                    BANANA,
                    "Banana",
                    banana_disabled,
                    &self.observations,
                ))
                .child(Self::item(
                    ORANGE,
                    "Orange",
                    orange_disabled,
                    &self.observations,
                ));
        }

        if self.config.include_scroll_arrows {
            list = list.h(px(24.0));
        }

        let mut popup = crate::select::SelectPopup::new()
            .w(px(220.0))
            .p_1()
            .bg(rgb(0xffffff));
        if self.config.include_scroll_arrows {
            let up_observations = Rc::clone(&self.observations);
            popup = popup.child(
                SelectScrollUpArrow::new()
                    .keep_mounted(true)
                    .style_with_state(move |state, arrow| {
                        up_observations.borrow_mut().scroll_arrow_states.push(state);
                        arrow
                            .h(px(12.0))
                            .w_full()
                            .debug_selector(|| "select-scroll-up-arrow".into())
                    }),
            );
        }
        popup = popup.child(list);
        if self.config.include_scroll_arrows {
            let down_observations = Rc::clone(&self.observations);
            popup = popup.child(
                SelectScrollDownArrow::new()
                    .keep_mounted(true)
                    .style_with_state(move |state, arrow| {
                        down_observations
                            .borrow_mut()
                            .scroll_arrow_states
                            .push(state);
                        arrow
                            .h(px(12.0))
                            .w_full()
                            .debug_selector(|| "select-scroll-down-arrow".into())
                    }),
            );
        }

        let outside_observations = Rc::clone(&self.observations);

        div()
            .size_full()
            .p_4()
            .child(
                root.child(Self::label())
                    .child(Self::trigger(&self.observations))
                    .child(
                        crate::select::SelectPortal::<SelectTestValue>::new().child(
                            crate::select::SelectPositioner::new()
                                .style_with_state({
                                    let observations = Rc::clone(&self.observations);
                                    move |state, positioner| {
                                        observations.borrow_mut().positioner_states.push(state);
                                        positioner
                                    }
                                })
                                .child(popup),
                        ),
                    ),
            )
            .child(
                div()
                    .absolute()
                    .top(px(300.0))
                    .left(px(16.0))
                    .w(px(80.0))
                    .h(px(32.0))
                    .id("select-outside-target")
                    .debug_selector(|| "select-outside-target".into())
                    .on_click({
                        let outside_observations = Rc::clone(&outside_observations);
                        move |_event, _window, _cx| {
                            outside_observations.borrow_mut().outside_clicks += 1;
                        }
                    })
                    .on_scroll_wheel(move |_event, _window, _cx| {
                        outside_observations.borrow_mut().outside_scrolls += 1;
                    }),
            )
    }
}

pub fn open_select(
    cx: &mut TestAppContext,
    config: SelectTestConfig,
) -> WindowHandle<SelectTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        SelectTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
) -> SelectObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("select test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("select test window should be open")
}

pub fn click_trigger(cx: &mut TestAppContext, window: WindowHandle<SelectTestView>) {
    click_selector(cx, window, "select-trigger");
}

pub fn click_label(cx: &mut TestAppContext, window: WindowHandle<SelectTestView>) {
    click_selector(cx, window, "select-label");
}

pub fn click_outside_target(cx: &mut TestAppContext, window: WindowHandle<SelectTestView>) {
    click_selector(cx, window, "select-outside-target");
}

pub fn click_item(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
    value: SelectTestValue,
) {
    click_selector(cx, window, item_selector(value));
}

pub fn move_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_mouse_move(bounds.center(), None, Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn scroll_over_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_event(ScrollWheelEvent {
        position: bounds.center(),
        delta: ScrollDelta::Pixels(point(px(0.0), px(-24.0))),
        modifiers: Modifiers::default(),
        touch_phase: TouchPhase::Moved,
    });
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn blur(cx: &mut TestAppContext, window: WindowHandle<SelectTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.blur();
            cx.notify();
        })
        .expect("select test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<SelectTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn click_selector(
    cx: &mut TestAppContext,
    window: WindowHandle<SelectTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
    cx.run_until_parked();
    cx.run_until_parked();
}

fn item_selector(value: SelectTestValue) -> &'static str {
    match value {
        APPLE => "select-item-apple",
        BANANA => "select-item-banana",
        ORANGE => "select-item-orange",
        _ => unreachable!("unknown select item value"),
    }
}
