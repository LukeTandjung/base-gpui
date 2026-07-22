use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, SharedString,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    toggle::{Toggle, ToggleStyleState},
    toggle_group::{
        ToggleGroup, ToggleGroupOrientation, ToggleGroupStyleState, ToggleGroupValueChangeSource,
    },
    utils::direction::{DirectionProvider, TextDirection},
};

pub const LABELS: [&str; 3] = ["bold", "italic", "underline"];

pub fn shared(value: &str) -> SharedString {
    SharedString::from(value.to_string())
}

#[derive(Clone)]
pub struct GroupTestConfig {
    pub multiple: bool,
    pub orientation: ToggleGroupOrientation,
    pub loop_focus: bool,
    pub disabled: bool,
    pub default_value: Vec<SharedString>,
    pub controlled_value: Option<Vec<SharedString>>,
    pub second_disabled: bool,
    pub cancel_toggle: bool,
    pub cancel_group: bool,
    pub omit_first_value: bool,
    pub rtl: bool,
}

impl Default for GroupTestConfig {
    fn default() -> Self {
        Self {
            multiple: false,
            orientation: ToggleGroupOrientation::Horizontal,
            loop_focus: true,
            disabled: false,
            default_value: Vec::new(),
            controlled_value: None,
            second_disabled: false,
            cancel_toggle: false,
            cancel_group: false,
            omit_first_value: false,
            rtl: false,
        }
    }
}

#[derive(Clone, Default)]
pub struct GroupObservations {
    pub pressed_changes: Vec<(usize, bool)>,
    pub value_changes: Vec<Vec<SharedString>>,
    pub value_change_sources: Vec<ToggleGroupValueChangeSource>,
    pub group_states: Vec<ToggleGroupStyleState>,
    pub toggle_states: Vec<(usize, ToggleStyleState)>,
}

impl GroupObservations {
    fn begin_render(&mut self) {
        self.group_states.clear();
        self.toggle_states.clear();
    }

    pub fn last_group_state(&self) -> ToggleGroupStyleState {
        self.group_states
            .last()
            .copied()
            .expect("group style state should be observed")
    }

    pub fn last_toggle_state(&self, index: usize) -> ToggleStyleState {
        self.toggle_states
            .iter()
            .rev()
            .find(|(state_index, _)| *state_index == index)
            .map(|(_, state)| *state)
            .expect("toggle style state should be observed")
    }

    pub fn pressed_flags(&self) -> [bool; 3] {
        [
            self.last_toggle_state(0).pressed,
            self.last_toggle_state(1).pressed,
            self.last_toggle_state(2).pressed,
        ]
    }

    pub fn focused_toggle(&self) -> Option<usize> {
        (0..LABELS.len()).find(|index| self.last_toggle_state(*index).focused)
    }
}

pub struct GroupTestView {
    pub config: GroupTestConfig,
    observations: Rc<RefCell<GroupObservations>>,
}

impl GroupTestView {
    pub fn new(config: GroupTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(GroupObservations::default())),
        }
    }

    pub fn read_observations(&self) -> GroupObservations {
        self.observations.borrow().clone()
    }
}

impl Render for GroupTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let config = self.config.clone();
        let mut group = ToggleGroup::<SharedString>::new()
            .id("toggle-group-test")
            .multiple(config.multiple)
            .orientation(config.orientation)
            .loop_focus(config.loop_focus)
            .disabled(config.disabled)
            .default_value(config.default_value.clone())
            .flex()
            .gap_2();

        if let Some(value) = config.controlled_value.clone() {
            group = group.value(value);
        }

        let cancel_group = config.cancel_group;
        let value_observations = Rc::clone(&self.observations);
        group = group.on_value_change(move |next, details, _window, _cx| {
            let mut observations = value_observations.borrow_mut();
            observations.value_changes.push(next.to_vec());
            observations.value_change_sources.push(details.source());

            if cancel_group {
                details.cancel();
            }
        });

        let group_state_observations = Rc::clone(&self.observations);
        group = group.style_with_state(move |state, base| {
            group_state_observations
                .borrow_mut()
                .group_states
                .push(state);
            base
        });

        for (index, label) in LABELS.iter().enumerate() {
            let mut toggle = Toggle::<SharedString>::new()
                .id(*label)
                .w(px(40.0))
                .h(px(24.0));

            if !(index == 0 && config.omit_first_value) {
                toggle = toggle.value(shared(label));
            }
            if index == 1 && config.second_disabled {
                toggle = toggle.disabled(true);
            }

            let cancel_toggle = config.cancel_toggle;
            let pressed_observations = Rc::clone(&self.observations);
            toggle = toggle.on_pressed_change(move |next, details, _window, _cx| {
                pressed_observations
                    .borrow_mut()
                    .pressed_changes
                    .push((index, next));

                if cancel_toggle {
                    details.cancel();
                }
            });

            let state_observations = Rc::clone(&self.observations);
            toggle = toggle.style_with_state(move |state, base| {
                state_observations
                    .borrow_mut()
                    .toggle_states
                    .push((index, state));
                base.debug_selector(move || format!("toggle-{index}"))
            });

            group = group.child(toggle);
        }

        let content = div().size_full().p_4().child(group);

        if config.rtl {
            div()
                .size_full()
                .child(
                    DirectionProvider::new()
                        .direction(TextDirection::Rtl)
                        .child(content),
                )
                .into_any_element()
        } else {
            content.into_any_element()
        }
    }
}

pub fn open_group(cx: &mut TestAppContext, config: GroupTestConfig) -> WindowHandle<GroupTestView> {
    cx.update(|cx| {
        crate::toggle::init(cx);
        crate::toggle_group::init(cx);
    });

    let window = cx.open_window(size(px(480.0), px(240.0)), move |_, _| {
        GroupTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<GroupTestView>,
) -> GroupObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("toggle group test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<GroupTestView>,
    update: impl FnOnce(&mut GroupTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("toggle group test window should be open");
    cx.run_until_parked();
}

pub fn focus_group(cx: &mut TestAppContext, window: WindowHandle<GroupTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("toggle group test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<GroupTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_item(cx: &mut TestAppContext, window: WindowHandle<GroupTestView>, index: usize) {
    let bounds = item_bounds(cx, window, index).expect("toggle item should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn item_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<GroupTestView>,
    index: usize,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(item_selector(index))
}

fn item_selector(index: usize) -> &'static str {
    match index {
        0 => "toggle-0",
        1 => "toggle-1",
        _ => "toggle-2",
    }
}
