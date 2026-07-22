use std::{cell::RefCell, rc::Rc};

use gpui::{div, prelude::*, px, size, IntoElement, Render, TestAppContext, WindowHandle};

use crate::separator::{Separator, SeparatorOrientation, SeparatorStyleState};

#[derive(Clone, Copy, Debug, Default)]
struct SeparatorTestConfig {
    orientation: Option<SeparatorOrientation>,
}

#[derive(Clone, Debug, Default)]
struct SeparatorObservations {
    states: Vec<SeparatorStyleState>,
}

impl SeparatorObservations {
    fn begin_render(&mut self) {
        self.states.clear();
    }

    fn last_state(&self) -> SeparatorStyleState {
        self.states
            .last()
            .copied()
            .expect("separator state should be observed")
    }
}

struct SeparatorTestView {
    config: SeparatorTestConfig,
    observations: Rc<RefCell<SeparatorObservations>>,
}

impl SeparatorTestView {
    fn new(config: SeparatorTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(SeparatorObservations::default())),
        }
    }
}

impl Render for SeparatorTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let observations = Rc::clone(&self.observations);
        let mut separator =
            Separator::new()
                .w(px(1.0))
                .h(px(24.0))
                .style_with_state(move |state, separator| {
                    observations.borrow_mut().states.push(state);
                    separator.debug_selector(|| "separator".into())
                });

        if let Some(orientation) = self.config.orientation {
            separator = separator.orientation(orientation);
        }

        div()
            .flex()
            .gap_2()
            .child("before")
            .child(separator)
            .child("after")
    }
}

fn open_separator(
    cx: &mut TestAppContext,
    config: SeparatorTestConfig,
) -> WindowHandle<SeparatorTestView> {
    cx.update(crate::init);
    let window = cx.open_window(size(px(240.0), px(120.0)), move |_, _| {
        SeparatorTestView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn observations(
    cx: &mut TestAppContext,
    window: WindowHandle<SeparatorTestView>,
) -> SeparatorObservations {
    window
        .update(cx, |view, _window, cx| {
            cx.notify();
            view.observations.borrow().clone()
        })
        .expect("separator test window should be open");
    cx.run_until_parked();
    window
        .update(cx, |view, _window, _cx| view.observations.borrow().clone())
        .expect("separator test window should be open")
}

fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<SeparatorTestView>,
    update: impl FnOnce(&mut SeparatorTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("separator test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

#[test]
fn orientation_defaults_to_horizontal() {
    assert_eq!(
        SeparatorOrientation::default(),
        SeparatorOrientation::Horizontal
    );
    assert_eq!(
        SeparatorStyleState::default(),
        SeparatorStyleState::new(SeparatorOrientation::Horizontal)
    );
}

#[gpui::test]
fn default_separator_reports_horizontal_state(cx: &mut TestAppContext) {
    let window = open_separator(cx, SeparatorTestConfig::default());

    assert_eq!(
        observations(cx, window).last_state().orientation,
        SeparatorOrientation::Horizontal
    );
}

#[gpui::test]
fn vertical_separator_reports_vertical_state(cx: &mut TestAppContext) {
    let window = open_separator(
        cx,
        SeparatorTestConfig {
            orientation: Some(SeparatorOrientation::Vertical),
        },
    );

    assert_eq!(
        observations(cx, window).last_state().orientation,
        SeparatorOrientation::Vertical
    );
}

#[gpui::test]
fn orientation_updates_without_stale_state(cx: &mut TestAppContext) {
    let window = open_separator(cx, SeparatorTestConfig::default());

    assert_eq!(
        observations(cx, window).last_state().orientation,
        SeparatorOrientation::Horizontal
    );

    update_config(cx, window, |config| {
        config.orientation = Some(SeparatorOrientation::Vertical);
    });

    assert_eq!(
        observations(cx, window).last_state().orientation,
        SeparatorOrientation::Vertical
    );
}

#[test]
fn convenience_builders_are_available() {
    let _horizontal = Separator::new().horizontal();
    let _vertical = Separator::new().vertical();
}
