use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::{
    radio_group::{
        init, RadioGroupIndicator, RadioGroupIndicatorStyleState, RadioGroupRadio,
        RadioGroupRadioStyleState, RadioGroupRoot, RadioGroupRootStyleState,
        RadioGroupValueChangeReason, RadioGroupValueChangeSource,
    },
    utils::direction::{DirectionProvider, TextDirection},
};

pub type RadioValue = &'static str;

pub const STANDARD: RadioValue = "standard";
pub const EXPRESS: RadioValue = "express";
pub const OVERNIGHT: RadioValue = "overnight";

#[derive(Clone, Copy)]
pub struct RadioGroupTestConfig {
    pub default_value: Option<RadioValue>,
    pub controlled_value: Option<Option<RadioValue>>,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub include_standard: bool,
    pub include_express: bool,
    pub include_overnight: bool,
    pub standard_disabled: bool,
    pub express_disabled: bool,
    pub overnight_disabled: bool,
    pub standard_read_only: bool,
    pub express_read_only: bool,
    pub overnight_read_only: bool,
    pub standard_required: bool,
    pub express_required: bool,
    pub overnight_required: bool,
    pub include_indicators: bool,
    pub keep_mounted_indicators: bool,
    pub cancel_changes: bool,
    pub direction: Option<TextDirection>,
    pub include_trailing_focusable: bool,
}

impl Default for RadioGroupTestConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            controlled_value: None,
            disabled: false,
            read_only: false,
            required: false,
            include_standard: true,
            include_express: true,
            include_overnight: true,
            standard_disabled: false,
            express_disabled: false,
            overnight_disabled: false,
            standard_read_only: false,
            express_read_only: false,
            overnight_read_only: false,
            standard_required: false,
            express_required: false,
            overnight_required: false,
            include_indicators: true,
            keep_mounted_indicators: false,
            cancel_changes: false,
            direction: None,
            include_trailing_focusable: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct RadioGroupObservations {
    pub value_changes: Vec<Option<RadioValue>>,
    pub change_reasons: Vec<RadioGroupValueChangeReason>,
    pub change_sources: Vec<RadioGroupValueChangeSource>,
    pub change_cancelable: Vec<bool>,
    pub change_canceled: Vec<bool>,
    pub root_states: Vec<RadioGroupRootStyleState>,
    pub radio_states: Vec<(RadioValue, RadioGroupRadioStyleState)>,
    pub indicator_states: Vec<(RadioValue, RadioGroupIndicatorStyleState)>,
}

impl RadioGroupObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.radio_states.clear();
        self.indicator_states.clear();
    }

    pub fn selected_value(&self) -> Option<RadioValue> {
        self.radio_states
            .iter()
            .find_map(|(value, state)| state.checked.then_some(*value))
    }

    pub fn highlighted_value(&self) -> Option<RadioValue> {
        self.radio_states
            .iter()
            .find_map(|(value, state)| state.highlighted.then_some(*value))
    }

    pub fn tab_stop_value(&self) -> Option<RadioValue> {
        self.radio_states
            .iter()
            .find_map(|(value, state)| state.tab_stop.then_some(*value))
    }

    pub fn focused_value(&self) -> Option<RadioValue> {
        self.radio_states
            .iter()
            .find_map(|(value, state)| state.focused.then_some(*value))
    }

    pub fn radio_state(&self, expected: RadioValue) -> Option<RadioGroupRadioStyleState> {
        self.radio_states
            .iter()
            .find_map(|(value, state)| (*value == expected).then_some(*state))
    }

    pub fn indicator_state(&self, expected: RadioValue) -> Option<RadioGroupIndicatorStyleState> {
        self.indicator_states
            .iter()
            .find_map(|(value, state)| (*value == expected).then_some(*state))
    }

    pub fn last_root_state(&self) -> RadioGroupRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("radio group root state should be observed")
    }
}

pub struct RadioGroupTestView {
    pub config: RadioGroupTestConfig,
    observations: Rc<RefCell<RadioGroupObservations>>,
}

impl RadioGroupTestView {
    pub fn new(config: RadioGroupTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(RadioGroupObservations::default())),
        }
    }

    pub fn read_observations(&self) -> RadioGroupObservations {
        self.observations.borrow().clone()
    }

    fn radio(
        value: RadioValue,
        disabled: bool,
        read_only: bool,
        required: bool,
        include_indicator: bool,
        keep_mounted_indicator: bool,
        observations: &Rc<RefCell<RadioGroupObservations>>,
    ) -> RadioGroupRadio<RadioValue> {
        let radio_observations = Rc::clone(observations);
        let mut radio = RadioGroupRadio::new()
            .id(format!("{value}-radio"))
            .value(value)
            .disabled(disabled)
            .read_only(read_only)
            .required(required)
            .size(px(24.0))
            .rounded_full()
            .border_1()
            .style_with_state(move |state, radio| {
                radio_observations
                    .borrow_mut()
                    .radio_states
                    .push((value, state));
                radio.debug_selector(move || radio_selector(value).into())
            });

        if include_indicator {
            let indicator_observations = Rc::clone(observations);
            radio = radio.child(
                RadioGroupIndicator::new()
                    .keep_mounted(keep_mounted_indicator)
                    .size(px(10.0))
                    .rounded_full()
                    .style_with_state(move |state, indicator| {
                        indicator_observations
                            .borrow_mut()
                            .indicator_states
                            .push((value, state));
                        indicator.debug_selector(move || indicator_selector(value).into())
                    }),
            );
        }

        radio
    }
}

impl Render for RadioGroupTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = RadioGroupRoot::<RadioValue>::new()
            .id("radio-group-test")
            .default_value(self.config.default_value)
            .disabled(self.config.disabled)
            .read_only(self.config.read_only)
            .required(self.config.required)
            .flex()
            .gap_2();

        if let Some(value) = self.config.controlled_value {
            root = root.value(value);
        }

        let cancel_changes = self.config.cancel_changes;
        let value_change_observations = Rc::clone(&self.observations);
        root = root.on_value_change(move |next, details, _window, _cx| {
            let mut observations = value_change_observations.borrow_mut();
            observations.value_changes.push(next.copied());
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
            root.debug_selector(|| "radio-group-root".into())
        });

        if self.config.include_standard {
            root = root.child(Self::radio(
                STANDARD,
                self.config.standard_disabled,
                self.config.standard_read_only,
                self.config.standard_required,
                self.config.include_indicators,
                self.config.keep_mounted_indicators,
                &self.observations,
            ));
        }

        if self.config.include_express {
            root = root.child(Self::radio(
                EXPRESS,
                self.config.express_disabled,
                self.config.express_read_only,
                self.config.express_required,
                self.config.include_indicators,
                self.config.keep_mounted_indicators,
                &self.observations,
            ));
        }

        if self.config.include_overnight {
            root = root.child(Self::radio(
                OVERNIGHT,
                self.config.overnight_disabled,
                self.config.overnight_read_only,
                self.config.overnight_required,
                self.config.include_indicators,
                self.config.keep_mounted_indicators,
                &self.observations,
            ));
        }

        let content = match self.config.direction {
            Some(direction) => DirectionProvider::new()
                .direction(direction)
                .child(root)
                .into_any_element(),
            None => root.into_any_element(),
        };

        div().size_full().p_4().child(content).children(
            self.config.include_trailing_focusable.then(|| {
                div()
                    .id("after-radio-group")
                    .focusable()
                    .tab_index(0)
                    .size(px(24.0))
                    .debug_selector(|| "after-radio-group".into())
            }),
        )
    }
}

pub fn open_radio_group(
    cx: &mut TestAppContext,
    config: RadioGroupTestConfig,
) -> WindowHandle<RadioGroupTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(240.0)), move |_, _| {
        RadioGroupTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<RadioGroupTestView>,
) -> RadioGroupObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("radio group test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<RadioGroupTestView>,
    update: impl FnOnce(&mut RadioGroupTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("radio group test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(
    cx: &mut TestAppContext,
    window: WindowHandle<RadioGroupTestView>,
    keys: &str,
) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn focus_next(cx: &mut TestAppContext, window: WindowHandle<RadioGroupTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("radio group test window should be open");
    cx.run_until_parked();
}

pub fn click_radio(
    cx: &mut TestAppContext,
    window: WindowHandle<RadioGroupTestView>,
    value: RadioValue,
) {
    let bounds =
        debug_bounds(cx, window, radio_selector(value)).expect("radio debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<RadioGroupTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn radio_selector(value: RadioValue) -> &'static str {
    match value {
        STANDARD => "radio-standard",
        EXPRESS => "radio-express",
        OVERNIGHT => "radio-overnight",
        _ => unreachable!("unknown test radio value"),
    }
}

fn indicator_selector(value: RadioValue) -> &'static str {
    match value {
        STANDARD => "indicator-standard",
        EXPRESS => "indicator-express",
        OVERNIGHT => "indicator-overnight",
        _ => unreachable!("unknown test radio value"),
    }
}
