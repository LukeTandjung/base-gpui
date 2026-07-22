use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, IntoElement, Render, RenderOnce, TestAppContext, WindowHandle,
};

use super::{current_direction, DirectionProvider, TextDirection};

#[derive(Clone, Default)]
struct DirectionObservations {
    directions: Vec<(&'static str, TextDirection)>,
}

impl DirectionObservations {
    fn begin_render(&mut self) {
        self.directions.clear();
    }

    fn direction_for(&self, label: &'static str) -> TextDirection {
        self.directions
            .iter()
            .find_map(|(observed_label, direction)| {
                (*observed_label == label).then_some(*direction)
            })
            .expect("direction should be observed")
    }
}

#[derive(IntoElement)]
struct DirectionProbe {
    label: &'static str,
    observations: Rc<RefCell<DirectionObservations>>,
}

impl DirectionProbe {
    fn new(label: &'static str, observations: &Rc<RefCell<DirectionObservations>>) -> Self {
        Self {
            label,
            observations: Rc::clone(observations),
        }
    }
}

impl RenderOnce for DirectionProbe {
    fn render(self, _window: &mut gpui::Window, _cx: &mut gpui::App) -> impl IntoElement {
        self.observations
            .borrow_mut()
            .directions
            .push((self.label, current_direction()));

        div()
            .size(px(8.0))
            .debug_selector(move || self.label.into())
    }
}

#[derive(Clone, Copy)]
enum DirectionScenario {
    OutsideProvider,
    Provider(TextDirection),
    Nested {
        outer: TextDirection,
        inner: TextDirection,
    },
    SiblingLeak,
    MultipleChildren,
}

struct DirectionTestView {
    scenario: DirectionScenario,
    observations: Rc<RefCell<DirectionObservations>>,
}

impl DirectionTestView {
    fn new(scenario: DirectionScenario) -> Self {
        Self {
            scenario,
            observations: Rc::new(RefCell::new(DirectionObservations::default())),
        }
    }

    fn read_observations(&self) -> DirectionObservations {
        self.observations.borrow().clone()
    }
}

impl Render for DirectionTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let observations = Rc::clone(&self.observations);
        let content = match self.scenario {
            DirectionScenario::OutsideProvider => div()
                .child(DirectionProbe::new("outside", &observations))
                .into_any_element(),
            DirectionScenario::Provider(direction) => DirectionProvider::new()
                .direction(direction)
                .child(DirectionProbe::new("inside", &observations))
                .into_any_element(),
            DirectionScenario::Nested { outer, inner } => DirectionProvider::new()
                .direction(outer)
                .child(DirectionProbe::new("outer", &observations))
                .child(
                    DirectionProvider::new()
                        .direction(inner)
                        .child(DirectionProbe::new("inner", &observations)),
                )
                .child(DirectionProbe::new("outer-after", &observations))
                .into_any_element(),
            DirectionScenario::SiblingLeak => div()
                .child(
                    DirectionProvider::new()
                        .direction(TextDirection::Rtl)
                        .child(DirectionProbe::new("inside", &observations)),
                )
                .child(DirectionProbe::new("sibling", &observations))
                .into_any_element(),
            DirectionScenario::MultipleChildren => DirectionProvider::new()
                .direction(TextDirection::Rtl)
                .child(DirectionProbe::new("first", &observations))
                .child(DirectionProbe::new("second", &observations))
                .into_any_element(),
        };

        div().size_full().child(content)
    }
}

fn open_direction(
    cx: &mut TestAppContext,
    scenario: DirectionScenario,
) -> WindowHandle<DirectionTestView> {
    let window = cx.open_window(size(px(240.0), px(120.0)), move |_, _| {
        DirectionTestView::new(scenario)
    });
    cx.run_until_parked();
    window
}

fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<DirectionTestView>,
) -> DirectionObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("direction test window should be open")
}

#[gpui::test]
fn default_direction_outside_provider_is_ltr(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::OutsideProvider);

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("outside"), TextDirection::Ltr);
}

#[gpui::test]
fn provider_supplies_rtl_to_descendants(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::Provider(TextDirection::Rtl));

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("inside"), TextDirection::Rtl);
}

#[gpui::test]
fn provider_supplies_ltr_to_descendants(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::Provider(TextDirection::Ltr));

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("inside"), TextDirection::Ltr);
}

#[gpui::test]
fn nested_rtl_inside_ltr_overrides_to_rtl(cx: &mut TestAppContext) {
    let window = open_direction(
        cx,
        DirectionScenario::Nested {
            outer: TextDirection::Ltr,
            inner: TextDirection::Rtl,
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("outer"), TextDirection::Ltr);
    assert_eq!(observations.direction_for("inner"), TextDirection::Rtl);
    assert_eq!(
        observations.direction_for("outer-after"),
        TextDirection::Ltr
    );
}

#[gpui::test]
fn nested_ltr_inside_rtl_overrides_to_ltr(cx: &mut TestAppContext) {
    let window = open_direction(
        cx,
        DirectionScenario::Nested {
            outer: TextDirection::Rtl,
            inner: TextDirection::Ltr,
        },
    );

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("outer"), TextDirection::Rtl);
    assert_eq!(observations.direction_for("inner"), TextDirection::Ltr);
    assert_eq!(
        observations.direction_for("outer-after"),
        TextDirection::Rtl
    );
}

#[gpui::test]
fn direction_does_not_leak_to_siblings(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::SiblingLeak);

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("inside"), TextDirection::Rtl);
    assert_eq!(observations.direction_for("sibling"), TextDirection::Ltr);
}

#[gpui::test]
fn updating_provider_direction_updates_descendant_observations(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::Provider(TextDirection::Rtl));
    assert_eq!(
        read_observations(cx, window).direction_for("inside"),
        TextDirection::Rtl
    );

    window
        .update(cx, |view, _window, cx| {
            view.scenario = DirectionScenario::Provider(TextDirection::Ltr);
            cx.notify();
        })
        .expect("direction test window should be open");
    cx.run_until_parked();

    assert_eq!(
        read_observations(cx, window).direction_for("inside"),
        TextDirection::Ltr
    );
}

#[gpui::test]
fn direction_can_be_read_during_descendant_render(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::Provider(TextDirection::Rtl));

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("inside"), TextDirection::Rtl);
}

#[gpui::test]
fn direction_provider_wraps_multiple_children(cx: &mut TestAppContext) {
    let window = open_direction(cx, DirectionScenario::MultipleChildren);

    let observations = read_observations(cx, window);
    assert_eq!(observations.direction_for("first"), TextDirection::Rtl);
    assert_eq!(observations.direction_for("second"), TextDirection::Rtl);
}
