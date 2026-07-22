use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, Modifiers, Pixels, Render, TestAppContext,
    VisualTestContext, WindowHandle,
};

use crate::tabs::{
    init, TabsIndicator, TabsIndicatorStyleState, TabsList, TabsOrientation, TabsPanel,
    TabsPanelStyleState, TabsRoot, TabsTab, TabsTabStyleState,
};

pub type TabValue = &'static str;

pub const OVERVIEW: TabValue = "overview";
pub const PROJECTS: TabValue = "projects";
pub const ACCOUNT: TabValue = "account";

#[derive(Clone, Copy)]
pub struct TabsTestConfig {
    pub default_value: Option<TabValue>,
    pub controlled_value: Option<Option<TabValue>>,
    pub orientation: TabsOrientation,
    pub activate_on_focus: bool,
    pub loop_focus: bool,
    pub include_overview: bool,
    pub include_projects: bool,
    pub include_account: bool,
    pub overview_disabled: bool,
    pub projects_disabled: bool,
    pub account_disabled: bool,
    pub keep_mounted_projects: bool,
    pub keep_mounted_account: bool,
    pub include_indicator: bool,
}

impl Default for TabsTestConfig {
    fn default() -> Self {
        Self {
            default_value: None,
            controlled_value: None,
            orientation: TabsOrientation::Horizontal,
            activate_on_focus: false,
            loop_focus: true,
            include_overview: true,
            include_projects: true,
            include_account: true,
            overview_disabled: false,
            projects_disabled: false,
            account_disabled: false,
            keep_mounted_projects: false,
            keep_mounted_account: false,
            include_indicator: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct TabsObservations {
    pub value_changes: Vec<Option<TabValue>>,
    pub tab_states: Vec<(TabValue, TabsTabStyleState)>,
    pub panel_states: Vec<(TabValue, TabsPanelStyleState)>,
    pub indicator_states: Vec<TabsIndicatorStyleState>,
}

impl TabsObservations {
    fn begin_render(&mut self) {
        self.tab_states.clear();
        self.panel_states.clear();
        self.indicator_states.clear();
    }

    pub fn active_value(&self) -> Option<TabValue> {
        self.tab_states
            .iter()
            .find_map(|(value, state)| state.active.then_some(*value))
    }

    pub fn highlighted_value(&self) -> Option<TabValue> {
        self.tab_states
            .iter()
            .find_map(|(value, state)| state.highlighted.then_some(*value))
    }

    pub fn tab_state(&self, expected: TabValue) -> Option<TabsTabStyleState> {
        self.tab_states
            .iter()
            .find_map(|(value, state)| (*value == expected).then_some(*state))
    }

    pub fn panel_state(&self, expected: TabValue) -> Option<TabsPanelStyleState> {
        self.panel_states
            .iter()
            .find_map(|(value, state)| (*value == expected).then_some(*state))
    }

    pub fn last_indicator_state(&self) -> Option<TabsIndicatorStyleState> {
        self.indicator_states.last().copied()
    }
}

pub struct TabsTestView {
    pub config: TabsTestConfig,
    observations: Rc<RefCell<TabsObservations>>,
}

impl TabsTestView {
    pub fn new(config: TabsTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(TabsObservations::default())),
        }
    }

    pub fn read_observations(&self) -> TabsObservations {
        self.observations.borrow().clone()
    }

    fn tab(
        value: TabValue,
        label: &'static str,
        disabled: bool,
        observations: &Rc<RefCell<TabsObservations>>,
    ) -> TabsTab<TabValue> {
        let observations = Rc::clone(observations);

        TabsTab::new()
            .id(format!("{value}-tab"))
            .value(value)
            .disabled(disabled)
            .w(px(96.0))
            .h(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .style_with_state(move |state, tab| {
                observations.borrow_mut().tab_states.push((value, state));
                tab.debug_selector(move || format!("tab-{value}"))
            })
            .child(label)
    }

    fn panel(
        value: TabValue,
        label: &'static str,
        keep_mounted: bool,
        observations: &Rc<RefCell<TabsObservations>>,
    ) -> TabsPanel<TabValue> {
        let observations = Rc::clone(observations);

        TabsPanel::new()
            .value(value)
            .keep_mounted(keep_mounted)
            .w(px(320.0))
            .h(px(48.0))
            .style_with_state(move |state, panel| {
                observations.borrow_mut().panel_states.push((value, state));
                panel.debug_selector(move || format!("panel-{value}"))
            })
            .child(label)
    }

    fn indicator(observations: &Rc<RefCell<TabsObservations>>) -> TabsIndicator<TabValue> {
        let observations = Rc::clone(observations);

        TabsIndicator::new()
            .absolute()
            .h(px(2.0))
            .style_with_state(move |state, indicator| {
                observations.borrow_mut().indicator_states.push(state);

                let indicator = match state.active_tab_position {
                    Some(position) => indicator.left(position.left).top(position.bottom),
                    None => indicator,
                };
                let indicator = match state.active_tab_size {
                    Some(size) => indicator.w(size.width),
                    None => indicator,
                };

                indicator.debug_selector(|| "indicator".into())
            })
    }
}

impl Render for TabsTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = TabsRoot::<TabValue>::new()
            .id("tabs-test")
            .default_value(self.config.default_value)
            .orientation(self.config.orientation)
            .flex()
            .flex_col()
            .gap_2();

        if let Some(value) = self.config.controlled_value {
            root = root.value(value);
        }

        let value_change_observations = Rc::clone(&self.observations);
        root = root.on_value_change(move |next, _window, _cx| {
            value_change_observations
                .borrow_mut()
                .value_changes
                .push(next.copied());
        });

        let mut list = TabsList::new()
            .activate_on_focus(self.config.activate_on_focus)
            .loop_focus(self.config.loop_focus)
            .relative()
            .flex()
            .gap_1()
            .w(px(320.0))
            .h(px(40.0));

        if self.config.include_overview {
            list = list.child(Self::tab(
                OVERVIEW,
                "Overview",
                self.config.overview_disabled,
                &self.observations,
            ));
        }

        if self.config.include_projects {
            list = list.child(Self::tab(
                PROJECTS,
                "Projects",
                self.config.projects_disabled,
                &self.observations,
            ));
        }

        if self.config.include_account {
            list = list.child(Self::tab(
                ACCOUNT,
                "Account",
                self.config.account_disabled,
                &self.observations,
            ));
        }

        if self.config.include_indicator {
            list = list.child(Self::indicator(&self.observations));
        }

        let mut root = root.child(list);

        if self.config.include_overview {
            root = root.child(Self::panel(
                OVERVIEW,
                "Overview panel",
                false,
                &self.observations,
            ));
        }

        if self.config.include_projects {
            root = root.child(Self::panel(
                PROJECTS,
                "Projects panel",
                self.config.keep_mounted_projects,
                &self.observations,
            ));
        }

        if self.config.include_account {
            root = root.child(Self::panel(
                ACCOUNT,
                "Account panel",
                self.config.keep_mounted_account,
                &self.observations,
            ));
        }

        div().size_full().p_4().child(root)
    }
}

pub fn open_tabs(cx: &mut TestAppContext, config: TabsTestConfig) -> WindowHandle<TabsTestView> {
    cx.update(init);

    let window = cx.open_window(size(px(640.0), px(360.0)), move |_, _| {
        TabsTestView::new(config)
    });
    cx.run_until_parked();
    window
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<TabsTestView>,
) -> TabsObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("tabs test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<TabsTestView>,
    update: impl FnOnce(&mut TabsTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("tabs test window should be open");
    cx.run_until_parked();
}

pub fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<TabsTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

pub fn click_tab(cx: &mut TestAppContext, window: WindowHandle<TabsTestView>, value: TabValue) {
    let bounds =
        debug_bounds(cx, window, tab_selector(value)).expect("tab debug bounds should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<TabsTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn tab_selector(value: TabValue) -> &'static str {
    match value {
        OVERVIEW => "tab-overview",
        PROJECTS => "tab-projects",
        ACCOUNT => "tab-account",
        _ => unreachable!("unknown test tab value"),
    }
}
