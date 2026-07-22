use std::{cell::RefCell, rc::Rc};

use gpui::{
    prelude::*, px, size, Bounds, IntoElement, Modifiers, Pixels, Render, SharedString,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::{
    checkbox::{CheckboxRoot, CheckboxRootStyleState},
    checkbox_group::{CheckboxGroup, CheckboxGroupStyleState},
};

#[derive(Clone, Debug)]
struct CheckboxGroupTestConfig {
    default_value: Vec<SharedString>,
    controlled_value: Option<Vec<SharedString>>,
    all_values: Vec<SharedString>,
    disabled: bool,
    include_parent: bool,
    disable_a: bool,
    read_only_a: bool,
    cancel_group: bool,
    cancel_child_a: bool,
    cancel_parent: bool,
    read_only_parent: bool,
    include_nested: bool,
    nested_default_value: Vec<SharedString>,
    nested_controlled_value: Option<Vec<SharedString>>,
    include_nested_parent: bool,
}

impl Default for CheckboxGroupTestConfig {
    fn default() -> Self {
        Self {
            default_value: Vec::new(),
            controlled_value: None,
            all_values: values(&["a", "b", "c"]),
            disabled: false,
            include_parent: false,
            disable_a: false,
            read_only_a: false,
            cancel_group: false,
            cancel_child_a: false,
            cancel_parent: false,
            read_only_parent: false,
            include_nested: false,
            nested_default_value: Vec::new(),
            nested_controlled_value: None,
            include_nested_parent: false,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct CheckboxGroupObservations {
    group_states: Vec<CheckboxGroupStyleState>,
    checkbox_states: Vec<(SharedString, CheckboxRootStyleState)>,
    group_changes: Vec<Vec<SharedString>>,
    nested_group_changes: Vec<Vec<SharedString>>,
    child_changes: Vec<(SharedString, bool)>,
}

impl CheckboxGroupObservations {
    fn begin_render(&mut self) {
        self.group_states.clear();
        self.checkbox_states.clear();
    }

    fn checkbox_state(&self, value: &str) -> CheckboxRootStyleState {
        self.checkbox_states
            .iter()
            .rev()
            .find(|(observed_value, _state)| observed_value.as_ref() == value)
            .map(|(_value, state)| *state)
            .unwrap_or_else(|| panic!("checkbox state for {value} should be observed"))
    }

    fn group_state(&self) -> CheckboxGroupStyleState {
        self.group_states
            .last()
            .copied()
            .expect("group state should be observed")
    }
}

struct CheckboxGroupTestView {
    config: CheckboxGroupTestConfig,
    observations: Rc<RefCell<CheckboxGroupObservations>>,
}

impl CheckboxGroupTestView {
    fn new(config: CheckboxGroupTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(CheckboxGroupObservations::default())),
        }
    }

    fn read_observations(&self) -> CheckboxGroupObservations {
        self.observations.borrow().clone()
    }
}

impl Render for CheckboxGroupTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let observations = Rc::clone(&self.observations);
        let cancel_group = self.config.cancel_group;
        let mut group = CheckboxGroup::new()
            .id("checkbox-group-test")
            .default_value(self.config.default_value.clone())
            .all_values(self.config.all_values.clone())
            .disabled(self.config.disabled)
            .flex()
            .flex_col()
            .gap_2()
            .on_value_change(move |next_value, details, _window, _cx| {
                observations.borrow_mut().group_changes.push(next_value);
                if cancel_group {
                    details.cancel();
                }
            });

        if let Some(value) = self.config.controlled_value.clone() {
            group = group.value(value);
        }

        let group_observations = Rc::clone(&self.observations);
        group = group.style_with_state(move |state, group| {
            group_observations.borrow_mut().group_states.push(state);
            group.debug_selector(|| "checkbox-group".into())
        });

        if self.config.include_parent {
            group = group.child(checkbox(
                "parent",
                None,
                true,
                false,
                self.config.read_only_parent,
                self.config.cancel_parent,
                Rc::clone(&self.observations),
            ));
        }

        group = group
            .child(checkbox(
                "a",
                Some("a"),
                false,
                self.config.disable_a,
                self.config.read_only_a,
                self.config.cancel_child_a,
                Rc::clone(&self.observations),
            ))
            .child(checkbox(
                "b",
                Some("b"),
                false,
                false,
                false,
                false,
                Rc::clone(&self.observations),
            ))
            .child(checkbox(
                "c",
                Some("c"),
                false,
                false,
                false,
                false,
                Rc::clone(&self.observations),
            ));

        if self.config.include_nested {
            let nested_observations = Rc::clone(&self.observations);
            let mut nested = CheckboxGroup::new()
                .id("nested-checkbox-group")
                .default_value(self.config.nested_default_value.clone())
                .all_values(["inner", "inner-2"])
                .on_value_change(move |next_value, _details, _window, _cx| {
                    nested_observations
                        .borrow_mut()
                        .nested_group_changes
                        .push(next_value);
                });
            if let Some(value) = self.config.nested_controlled_value.clone() {
                nested = nested.value(value);
            }
            if self.config.include_nested_parent {
                nested = nested.child(checkbox(
                    "inner-parent",
                    None,
                    true,
                    false,
                    false,
                    false,
                    Rc::clone(&self.observations),
                ));
            }
            nested = nested
                .child(checkbox(
                    "inner",
                    Some("inner"),
                    false,
                    false,
                    false,
                    false,
                    Rc::clone(&self.observations),
                ))
                .child(checkbox(
                    "inner-2",
                    Some("inner-2"),
                    false,
                    false,
                    false,
                    false,
                    Rc::clone(&self.observations),
                ));
            group = group.child(nested);
        }

        group
    }
}

fn checkbox(
    id_suffix: &'static str,
    value: Option<&'static str>,
    parent: bool,
    disabled: bool,
    read_only: bool,
    cancel_change: bool,
    observations: Rc<RefCell<CheckboxGroupObservations>>,
) -> impl IntoElement {
    let observed_value = SharedString::from(id_suffix);
    let change_value = observed_value.clone();
    let state_value = observed_value.clone();
    let selector = format!("checkbox-{id_suffix}");
    let change_observations = Rc::clone(&observations);
    let state_observations = Rc::clone(&observations);
    let mut checkbox = CheckboxRoot::new()
        .id(selector.clone())
        .parent(parent)
        .disabled(disabled)
        .read_only(read_only)
        .w(px(24.0))
        .h(px(24.0))
        .on_checked_change(move |next_checked, details, _window, _cx| {
            change_observations
                .borrow_mut()
                .child_changes
                .push((change_value.clone(), next_checked));
            if cancel_change {
                details.cancel();
            }
        });

    if let Some(value) = value {
        checkbox = checkbox.value(value);
    }

    checkbox.style_with_state(move |state, checkbox| {
        state_observations
            .borrow_mut()
            .checkbox_states
            .push((state_value.clone(), state));
        checkbox.debug_selector({
            let selector = selector.clone();
            move || selector.clone().into()
        })
    })
}

fn open_group(
    cx: &mut TestAppContext,
    config: CheckboxGroupTestConfig,
) -> WindowHandle<CheckboxGroupTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(360.0), px(240.0)), move |_, _| {
        CheckboxGroupTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn observations(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupTestView>,
) -> CheckboxGroupObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.read_observations()
        })
        .expect("checkbox group test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("checkbox group test window should be open")
}

fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupTestView>,
    update: impl FnOnce(&mut CheckboxGroupTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("checkbox group test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn click(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupTestView>,
    selector: &'static str,
) {
    let bounds = debug_bounds(cx, window, selector).expect("debug selector should exist");
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.simulate_click(bounds.center(), Modifiers::default());
    visual.run_until_parked();
}

fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<CheckboxGroupTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}

fn focus_next(cx: &mut TestAppContext, window: WindowHandle<CheckboxGroupTestView>) {
    window
        .update(cx, |_view, window, cx| {
            window.focus_next(cx);
        })
        .expect("checkbox group test window should be open");
    cx.run_until_parked();
}

fn simulate_keys(cx: &mut TestAppContext, window: WindowHandle<CheckboxGroupTestView>, keys: &str) {
    cx.simulate_keystrokes(window.into(), keys);
    cx.run_until_parked();
}

fn values(values: &[&str]) -> Vec<SharedString> {
    values
        .iter()
        .map(|value| SharedString::from(*value))
        .collect()
}

#[gpui::test]
fn default_value_initializes_checked_children(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            default_value: values(&["a"]),
            ..Default::default()
        },
    );

    let observations = observations(cx, window);

    assert!(observations.checkbox_state("a").checked);
    assert!(!observations.checkbox_state("b").checked);
}

#[gpui::test]
fn child_click_appends_and_removes_group_values(cx: &mut TestAppContext) {
    let window = open_group(cx, CheckboxGroupTestConfig::default());

    click(cx, window, "checkbox-a");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.last().unwrap(), &values(&["a"]));
    assert!(obs.checkbox_state("a").checked);

    click(cx, window, "checkbox-a");
    let obs = observations(cx, window);
    assert_eq!(
        obs.group_changes.last().unwrap(),
        &Vec::<SharedString>::new()
    );
    assert!(!obs.checkbox_state("a").checked);
}

#[gpui::test]
fn controlled_value_reflects_external_state(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            controlled_value: Some(values(&["a"])),
            ..Default::default()
        },
    );

    assert!(observations(cx, window).checkbox_state("a").checked);

    update_config(cx, window, |config| {
        config.controlled_value = Some(values(&["b"]));
    });

    let observations = observations(cx, window);
    assert!(!observations.checkbox_state("a").checked);
    assert!(observations.checkbox_state("b").checked);
}

#[gpui::test]
fn group_disabled_state_disables_children_and_blocks_changes(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    assert!(observations(cx, window).checkbox_state("a").disabled);

    click(cx, window, "checkbox-a");
    let observations = observations(cx, window);
    assert!(observations.group_changes.is_empty());
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn child_cancellation_prevents_group_change(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            cancel_child_a: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-a");
    let observations = observations(cx, window);

    assert_eq!(observations.child_changes.len(), 1);
    assert!(observations.group_changes.is_empty());
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn group_cancellation_prevents_uncontrolled_mutation(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            cancel_group: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-a");
    let observations = observations(cx, window);

    assert_eq!(observations.group_changes.last().unwrap(), &values(&["a"]));
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn parent_checkbox_selects_and_clears_enabled_children(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-parent");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.last().unwrap(), &values(&["a", "b", "c"]));
    assert!(obs.checkbox_state("parent").checked);

    click(cx, window, "checkbox-parent");
    let obs = observations(cx, window);
    assert_eq!(
        obs.group_changes.last().unwrap(),
        &Vec::<SharedString>::new()
    );
    assert!(!obs.checkbox_state("parent").checked);
}

#[gpui::test]
fn parent_checkbox_preserves_checked_disabled_children(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            default_value: values(&["a"]),
            disable_a: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-parent");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.last().unwrap(), &values(&["a", "b", "c"]));
    assert!(obs.checkbox_state("a").checked);

    click(cx, window, "checkbox-parent");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.last().unwrap(), &values(&["a"]));
    assert!(obs.checkbox_state("a").checked);
    assert!(!obs.checkbox_state("b").checked);
}

#[gpui::test]
fn parent_checkbox_is_indeterminate_when_some_children_are_checked(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            default_value: values(&["a"]),
            ..Default::default()
        },
    );

    let observations = observations(cx, window);
    assert!(observations.checkbox_state("parent").indeterminate);
    assert!(!observations.checkbox_state("parent").checked);
}

#[gpui::test]
fn parent_checkbox_does_not_call_child_change_handlers(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-parent");
    let observations = observations(cx, window);

    assert_eq!(observations.child_changes, vec![("parent".into(), true)]);
    assert_eq!(
        observations.group_changes.last().unwrap(),
        &values(&["a", "b", "c"])
    );
}

#[gpui::test]
fn parent_cancellation_prevents_group_change(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            cancel_parent: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-parent");
    let observations = observations(cx, window);

    assert_eq!(observations.child_changes, vec![("parent".into(), true)]);
    assert!(observations.group_changes.is_empty());
    assert!(!observations.checkbox_state("parent").checked);
}

#[gpui::test]
fn parent_checkbox_does_not_select_unchecked_disabled_children(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            disable_a: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-parent");
    let observations = observations(cx, window);

    assert_eq!(
        observations.group_changes.last().unwrap(),
        &values(&["b", "c"])
    );
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn read_only_child_and_parent_do_not_change_group_value(cx: &mut TestAppContext) {
    let child_window = open_group(
        cx,
        CheckboxGroupTestConfig {
            read_only_a: true,
            ..Default::default()
        },
    );
    click(cx, child_window, "checkbox-a");
    let child_observations = observations(cx, child_window);
    assert!(child_observations.group_changes.is_empty());
    assert!(!child_observations.checkbox_state("a").checked);

    let parent_window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            read_only_parent: true,
            ..Default::default()
        },
    );
    click(cx, parent_window, "checkbox-parent");
    let parent_observations = observations(cx, parent_window);
    assert!(parent_observations.group_changes.is_empty());
    assert!(!parent_observations.checkbox_state("parent").checked);
}

#[gpui::test]
fn keyboard_activation_uses_checkbox_semantics_inside_group(cx: &mut TestAppContext) {
    let window = open_group(cx, CheckboxGroupTestConfig::default());

    focus_next(cx, window);
    simulate_keys(cx, window, "space");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.last().unwrap(), &values(&["a"]));
    assert!(obs.checkbox_state("a").checked);

    simulate_keys(cx, window, "enter");
    let obs = observations(cx, window);
    assert_eq!(obs.group_changes.len(), 1);
    assert!(obs.checkbox_state("a").checked);
}

#[gpui::test]
fn parent_keyboard_activation_uses_checkbox_semantics(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_parent: true,
            ..Default::default()
        },
    );

    focus_next(cx, window);
    simulate_keys(cx, window, "space");
    let observations = observations(cx, window);

    assert_eq!(
        observations.group_changes.last().unwrap(),
        &values(&["a", "b", "c"])
    );
    assert!(observations.checkbox_state("parent").checked);
}

#[gpui::test]
fn disabled_group_ignores_keyboard_activation(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            disabled: true,
            ..Default::default()
        },
    );

    focus_next(cx, window);
    simulate_keys(cx, window, "space");
    let observations = observations(cx, window);

    assert!(observations.group_changes.is_empty());
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn nested_groups_isolate_state(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_nested: true,
            ..Default::default()
        },
    );

    click(cx, window, "checkbox-inner");
    let observations = observations(cx, window);

    assert!(observations.group_changes.is_empty());
    assert_eq!(
        observations.nested_group_changes.last().unwrap(),
        &values(&["inner"])
    );
    assert!(observations.checkbox_state("inner").checked);
    assert!(!observations.checkbox_state("a").checked);
}

#[gpui::test]
fn nested_groups_can_be_controlled_independently(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            controlled_value: Some(values(&["a"])),
            include_nested: true,
            nested_controlled_value: Some(values(&["inner"])),
            ..Default::default()
        },
    );

    let obs = observations(cx, window);
    assert!(obs.checkbox_state("a").checked);
    assert!(obs.checkbox_state("inner").checked);

    update_config(cx, window, |config| {
        config.nested_controlled_value = Some(values(&["inner-2"]));
    });

    let obs = observations(cx, window);
    assert!(obs.checkbox_state("a").checked);
    assert!(!obs.checkbox_state("inner").checked);
    assert!(obs.checkbox_state("inner-2").checked);
}

#[gpui::test]
fn nested_parent_checkbox_uses_nested_all_values(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            include_nested: true,
            include_nested_parent: true,
            nested_default_value: values(&["inner"]),
            ..Default::default()
        },
    );

    let observations = observations(cx, window);
    assert!(observations.checkbox_state("inner-parent").indeterminate);
    assert!(!observations.checkbox_state("inner-parent").checked);
}

#[gpui::test]
fn style_with_state_receives_group_state(cx: &mut TestAppContext) {
    let window = open_group(
        cx,
        CheckboxGroupTestConfig {
            default_value: values(&["a"]),
            disabled: true,
            ..Default::default()
        },
    );

    let state = observations(cx, window).group_state();
    assert!(state.disabled);
    assert!(state.filled);
}
