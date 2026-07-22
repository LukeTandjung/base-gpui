use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::{
    field::{FieldRoot, FieldRootStyleState},
    fieldset::{FieldsetLegend, FieldsetLegendStyleState, FieldsetRoot, FieldsetRootStyleState},
    input::{Input, InputStyleState},
    switch::{SwitchRoot, SwitchRootStyleState},
};

#[derive(Clone, Debug, Default)]
struct FieldsetTestConfig {
    disabled: bool,
    nested: bool,
}

#[derive(Clone, Default)]
struct FieldsetObservations {
    root_states: Vec<FieldsetRootStyleState>,
    legend_states: Vec<FieldsetLegendStyleState>,
    field_states: Vec<FieldRootStyleState>,
    input_states: Vec<InputStyleState>,
    switch_states: Vec<SwitchRootStyleState>,
}

impl FieldsetObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.legend_states.clear();
        self.field_states.clear();
        self.input_states.clear();
        self.switch_states.clear();
    }
}

struct FieldsetTestView {
    config: FieldsetTestConfig,
    observations: Rc<RefCell<FieldsetObservations>>,
}

impl FieldsetTestView {
    fn new(config: FieldsetTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(FieldsetObservations::default())),
        }
    }
}

impl Render for FieldsetTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let content = fieldset_contents(&self.observations);
        let root_observations = Rc::clone(&self.observations);
        let root = FieldsetRoot::new()
            .id("fieldset-test")
            .disabled(self.config.disabled)
            .flex()
            .flex_col()
            .gap_2()
            .style_with_state(move |state, root| {
                root_observations.borrow_mut().root_states.push(state);
                root
            });

        if self.config.nested {
            root.child_any(
                FieldsetRoot::new()
                    .id("nested-fieldset-test")
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child_any(content),
            )
        } else {
            root.child_any(content)
        }
    }
}

fn fieldset_contents(observations: &Rc<RefCell<FieldsetObservations>>) -> impl IntoElement {
    let legend_observations = Rc::clone(observations);
    let field_observations = Rc::clone(observations);
    let input_observations = Rc::clone(observations);
    let switch_observations = Rc::clone(observations);

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            FieldsetLegend::new()
                .style_with_state(move |state, legend| {
                    legend_observations.borrow_mut().legend_states.push(state);
                    legend
                })
                .child("Billing details"),
        )
        .child(
            FieldRoot::new()
                .id("fieldset-field")
                .style_with_state(move |state, field| {
                    field_observations.borrow_mut().field_states.push(state);
                    field
                })
                .child(
                    Input::new()
                        .id("fieldset-input")
                        .w(px(120.0))
                        .h(px(24.0))
                        .style_with_state(move |state, input| {
                            input_observations.borrow_mut().input_states.push(state);
                            input
                        }),
                ),
        )
        .child(
            SwitchRoot::new()
                .id("fieldset-switch")
                .size(px(24.0))
                .style_with_state(move |state, switch| {
                    switch_observations.borrow_mut().switch_states.push(state);
                    switch
                }),
        )
}

fn open_fieldset(
    cx: &mut TestAppContext,
    config: FieldsetTestConfig,
) -> WindowHandle<FieldsetTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(360.0), px(240.0)), move |_, _| {
        FieldsetTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn observations(
    cx: &mut TestAppContext,
    window: WindowHandle<FieldsetTestView>,
) -> FieldsetObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.observations.borrow().clone()
        })
        .expect("fieldset test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.observations.borrow().clone())
        .expect("fieldset test window should be open")
}

#[gpui::test]
fn fieldset_disabled_state_reaches_legend_fields_and_direct_controls(cx: &mut TestAppContext) {
    let window = open_fieldset(
        cx,
        FieldsetTestConfig {
            disabled: true,
            nested: false,
        },
    );

    let observations = observations(cx, window);

    assert!(observations.root_states.last().unwrap().disabled);
    assert!(observations.legend_states.last().unwrap().disabled);
    assert!(observations.field_states.last().unwrap().disabled);
    assert!(observations.input_states.last().unwrap().disabled);
    assert!(observations.switch_states.last().unwrap().disabled);
}

#[gpui::test]
fn nested_fieldset_inherits_disabled_state_from_ancestor(cx: &mut TestAppContext) {
    let window = open_fieldset(
        cx,
        FieldsetTestConfig {
            disabled: true,
            nested: true,
        },
    );

    let observations = observations(cx, window);

    assert!(observations.legend_states.last().unwrap().disabled);
    assert!(observations.field_states.last().unwrap().disabled);
    assert!(observations.input_states.last().unwrap().disabled);
}

#[gpui::test]
fn enabled_fieldset_leaves_descendants_enabled(cx: &mut TestAppContext) {
    let window = open_fieldset(cx, FieldsetTestConfig::default());

    let observations = observations(cx, window);

    assert!(!observations.root_states.last().unwrap().disabled);
    assert!(!observations.legend_states.last().unwrap().disabled);
    assert!(!observations.field_states.last().unwrap().disabled);
    assert!(!observations.input_states.last().unwrap().disabled);
    assert!(!observations.switch_states.last().unwrap().disabled);
}
