use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, App, IntoElement, Render, RenderOnce, TestAppContext, Window,
    WindowHandle,
};

use crate::field::{current_field_context, FieldDescription, FieldError, FieldLabel, FieldRoot};

#[derive(Clone, Default)]
struct MetadataConfig {
    label: bool,
    description: bool,
    error: bool,
}

#[derive(Clone, Debug, Default)]
struct MetadataObservations {
    label_registered: Vec<bool>,
    description_counts: Vec<usize>,
    error_counts: Vec<usize>,
}

impl MetadataObservations {
    fn last(&self) -> (bool, usize, usize) {
        (
            *self
                .label_registered
                .last()
                .expect("label metadata should be observed"),
            *self
                .description_counts
                .last()
                .expect("description metadata should be observed"),
            *self
                .error_counts
                .last()
                .expect("error metadata should be observed"),
        )
    }
}

struct MetadataView {
    config: MetadataConfig,
    observations: Rc<RefCell<MetadataObservations>>,
}

impl MetadataView {
    fn new(config: MetadataConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(MetadataObservations::default())),
        }
    }

    fn read_observations(&self) -> MetadataObservations {
        self.observations.borrow().clone()
    }
}

impl Render for MetadataView {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        self.observations.borrow_mut().label_registered.clear();
        self.observations.borrow_mut().description_counts.clear();
        self.observations.borrow_mut().error_counts.clear();

        let mut root = FieldRoot::new()
            .id("metadata-field")
            .invalid(self.config.error);

        if self.config.label {
            root = root.child(FieldLabel::new());
        }
        if self.config.description {
            root = root.child(FieldDescription::new());
        }

        root = root
            .child(FieldError::new())
            .child_any(MetadataProbe::new(&self.observations));

        div().size_full().child(root)
    }
}

#[derive(IntoElement)]
struct MetadataProbe {
    observations: Rc<RefCell<MetadataObservations>>,
}

impl MetadataProbe {
    fn new(observations: &Rc<RefCell<MetadataObservations>>) -> Self {
        Self {
            observations: Rc::clone(observations),
        }
    }
}

impl RenderOnce for MetadataProbe {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        if let Some(context) = current_field_context() {
            context.read(cx, |runtime, _props| {
                let mut observations = self.observations.borrow_mut();
                observations
                    .label_registered
                    .push(runtime.label_registered());
                observations
                    .description_counts
                    .push(runtime.description_count());
                observations.error_counts.push(runtime.error_count());
            });
        }

        div().size(px(0.0))
    }
}

fn open_metadata(cx: &mut TestAppContext, config: MetadataConfig) -> WindowHandle<MetadataView> {
    let window = cx.open_window(size(px(240.0), px(160.0)), move |_, _| {
        MetadataView::new(config)
    });
    cx.run_until_parked();
    cx.run_until_parked();
    window
}

fn update_metadata(
    cx: &mut TestAppContext,
    window: WindowHandle<MetadataView>,
    update: impl FnOnce(&mut MetadataConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("metadata test window should be open");
    cx.run_until_parked();
    cx.run_until_parked();
}

fn read_metadata(
    cx: &mut TestAppContext,
    window: WindowHandle<MetadataView>,
) -> MetadataObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("metadata test window should be open")
}

#[gpui::test]
fn label_description_and_error_metadata_update_when_parts_change(cx: &mut TestAppContext) {
    let window = open_metadata(
        cx,
        MetadataConfig {
            label: true,
            description: true,
            error: true,
        },
    );

    assert_eq!(read_metadata(cx, window).last(), (true, 1, 1));

    update_metadata(cx, window, |config| {
        config.label = false;
        config.description = false;
        config.error = false;
    });

    assert_eq!(read_metadata(cx, window).last(), (false, 0, 0));

    update_metadata(cx, window, |config| {
        config.label = true;
        config.description = true;
        config.error = true;
    });

    assert_eq!(read_metadata(cx, window).last(), (true, 1, 1));
}
