use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::{
    separator::{SeparatorOrientation, SeparatorStyleState},
    toolbar::{
        ToolbarButton, ToolbarButtonStyleState, ToolbarGroup, ToolbarGroupStyleState, ToolbarInput,
        ToolbarInputStyleState, ToolbarLink, ToolbarLinkStyleState, ToolbarOrientation,
        ToolbarRoot, ToolbarRootStyleState, ToolbarSeparator,
    },
    utils::direction::{DirectionProvider, TextDirection},
};

/// Roving item order in the harness toolbar:
/// 0 = button "b0", 1 = button "b1", 2 = grouped button "b2",
/// 3 = grouped link, 4 = input. The separator occupies no slot.
pub const INPUT_INDEX: usize = 4;
pub const LINK_INDEX: usize = 3;

#[derive(Clone)]
pub struct ToolbarTestConfig {
    pub orientation: ToolbarOrientation,
    pub loop_focus: bool,
    pub toolbar_disabled: bool,
    pub rtl: bool,
    pub first_disabled: bool,
    pub first_focusable: bool,
    pub second_disabled: bool,
    pub second_focusable: bool,
    pub group_disabled: bool,
    pub input_disabled: bool,
    pub input_focusable: bool,
    pub separator_override: Option<SeparatorOrientation>,
}

impl Default for ToolbarTestConfig {
    fn default() -> Self {
        Self {
            orientation: ToolbarOrientation::Horizontal,
            loop_focus: true,
            toolbar_disabled: false,
            rtl: false,
            first_disabled: false,
            first_focusable: true,
            second_disabled: false,
            second_focusable: true,
            group_disabled: false,
            input_disabled: false,
            input_focusable: true,
            separator_override: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct ToolbarObservations {
    pub clicks: Vec<usize>,
    pub root_states: Vec<ToolbarRootStyleState>,
    pub button_states: Vec<(usize, ToolbarButtonStyleState)>,
    pub link_states: Vec<ToolbarLinkStyleState>,
    pub input_states: Vec<ToolbarInputStyleState>,
    pub group_states: Vec<ToolbarGroupStyleState>,
    pub separator_states: Vec<SeparatorStyleState>,
}

impl ToolbarObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.button_states.clear();
        self.link_states.clear();
        self.input_states.clear();
        self.group_states.clear();
        self.separator_states.clear();
    }

    pub fn last_root_state(&self) -> ToolbarRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("root style state should be observed")
    }

    pub fn last_button_state(&self, index: usize) -> ToolbarButtonStyleState {
        self.button_states
            .iter()
            .rev()
            .find(|(state_index, _)| *state_index == index)
            .map(|(_, state)| *state)
            .expect("button style state should be observed")
    }

    pub fn last_link_state(&self) -> ToolbarLinkStyleState {
        self.link_states
            .last()
            .copied()
            .expect("link style state should be observed")
    }

    pub fn last_input_state(&self) -> ToolbarInputStyleState {
        self.input_states
            .last()
            .cloned()
            .expect("input style state should be observed")
    }

    pub fn last_group_state(&self) -> ToolbarGroupStyleState {
        self.group_states
            .last()
            .copied()
            .expect("group style state should be observed")
    }

    pub fn last_separator_state(&self) -> SeparatorStyleState {
        self.separator_states
            .last()
            .copied()
            .expect("separator style state should be observed")
    }

    pub fn focused_item(&self) -> Option<usize> {
        for index in [0usize, 1, 2] {
            if self.last_button_state(index).focused {
                return Some(index);
            }
        }
        if self.last_link_state().focused {
            return Some(LINK_INDEX);
        }
        if self.last_input_state().input.focused {
            return Some(INPUT_INDEX);
        }

        None
    }
}

pub struct ToolbarTestView {
    pub config: ToolbarTestConfig,
    observations: Rc<RefCell<ToolbarObservations>>,
}

impl ToolbarTestView {
    pub fn new(config: ToolbarTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(ToolbarObservations::default())),
        }
    }

    pub fn read_observations(&self) -> ToolbarObservations {
        self.observations.borrow().clone()
    }

    fn button(&self, index: usize, id: &'static str) -> ToolbarButton {
        let config = &self.config;
        let mut button = ToolbarButton::new().id(id).w(px(40.0)).h(px(24.0));

        if index == 0 {
            button = button
                .disabled(config.first_disabled)
                .focusable_when_disabled(config.first_focusable);
        }
        if index == 1 {
            button = button
                .disabled(config.second_disabled)
                .focusable_when_disabled(config.second_focusable);
        }

        let click_observations = Rc::clone(&self.observations);
        button = button.on_click(move |_event, _window, _cx| {
            click_observations.borrow_mut().clicks.push(index);
        });

        let state_observations = Rc::clone(&self.observations);
        button.style_with_state(move |state, base| {
            state_observations
                .borrow_mut()
                .button_states
                .push((index, state));
            base.debug_selector(move || format!("toolbar-item-{index}"))
        })
    }
}

impl Render for ToolbarTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let config = self.config.clone();

        let click_observations = Rc::clone(&self.observations);
        let link_state_observations = Rc::clone(&self.observations);
        let link = ToolbarLink::new()
            .id("link")
            .w(px(40.0))
            .h(px(24.0))
            .on_click(move |_event, _window, _cx| {
                click_observations.borrow_mut().clicks.push(LINK_INDEX);
            })
            .style_with_state(move |state, base| {
                link_state_observations.borrow_mut().link_states.push(state);
                base.debug_selector(|| format!("toolbar-item-{LINK_INDEX}"))
            });

        let group_state_observations = Rc::clone(&self.observations);
        let group = ToolbarGroup::new()
            .disabled(config.group_disabled)
            .child(self.button(2, "b2"))
            .child(link)
            .style_with_state(move |state, base| {
                group_state_observations
                    .borrow_mut()
                    .group_states
                    .push(state);
                base.flex().gap_2()
            });

        let separator_state_observations = Rc::clone(&self.observations);
        let mut separator = ToolbarSeparator::new().style_with_state(move |state, base| {
            separator_state_observations
                .borrow_mut()
                .separator_states
                .push(state);
            base.w(px(2.0)).h(px(24.0))
        });
        if let Some(orientation) = config.separator_override {
            separator = separator.orientation(orientation);
        }

        let input_state_observations = Rc::clone(&self.observations);
        let input = ToolbarInput::new()
            .id("toolbar-input-test")
            .default_value("abc")
            .disabled(config.input_disabled)
            .focusable_when_disabled(config.input_focusable)
            .w(px(120.0))
            .h(px(24.0))
            .style_with_state(move |state, base| {
                input_state_observations
                    .borrow_mut()
                    .input_states
                    .push(state);
                base.debug_selector(|| format!("toolbar-item-{INPUT_INDEX}"))
            });

        let root_state_observations = Rc::clone(&self.observations);
        let toolbar = ToolbarRoot::new()
            .id("toolbar-test")
            .orientation(config.orientation)
            .loop_focus(config.loop_focus)
            .disabled(config.toolbar_disabled)
            .child(self.button(0, "b0"))
            .child(self.button(1, "b1"))
            .child(group)
            .child(separator)
            .child(input)
            .flex()
            .gap_2()
            .style_with_state(move |state, base| {
                root_state_observations.borrow_mut().root_states.push(state);
                base
            });

        let content = div().size_full().p_4().child(toolbar);

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

pub fn open_toolbar(
    cx: &mut TestAppContext,
    config: ToolbarTestConfig,
) -> WindowHandle<ToolbarTestView> {
    cx.update(|cx| {
        crate::primitives::input::init(cx);
        crate::toolbar::init(cx);
    });

    let window = cx.open_window(size(px(640.0), px(240.0)), move |_, _| {
        ToolbarTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<ToolbarTestView>,
) -> ToolbarObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("toolbar test window should be open")
}

pub fn focus_toolbar(cx: &mut TestAppContext, window: WindowHandle<ToolbarTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("toolbar test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<ToolbarTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_item(cx: &mut TestAppContext, window: WindowHandle<ToolbarTestView>, index: usize) {
    let bounds = item_bounds(cx, window, index).expect("toolbar item should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn item_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<ToolbarTestView>,
    index: usize,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(item_selector(index))
}

fn item_selector(index: usize) -> &'static str {
    match index {
        0 => "toolbar-item-0",
        1 => "toolbar-item-1",
        2 => "toolbar-item-2",
        3 => "toolbar-item-3",
        _ => "toolbar-item-4",
    }
}
