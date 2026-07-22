use std::sync::Arc;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

use gpui::{
    div, prelude::*, px, size, Bounds, Image, ImageFormat, ImageSource, Pixels, Render,
    TestAppContext, VisualTestContext, WindowHandle,
};

use crate::avatar::{
    AvatarFallback, AvatarFallbackStyleState, AvatarImage, AvatarImageLoadingStatus,
    AvatarImageStyleState, AvatarRoot, AvatarRootStyleState,
};

/// A minimal valid 1x1 transparent PNG.
const ONE_PIXEL_PNG: [u8; 67] = [
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
    0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
    0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
    0x42, 0x60, 0x82,
];

/// An image source whose asset never resolves, holding status at `Loading`.
pub fn pending_source() -> ImageSource {
    ImageSource::from(
        |_window: &mut gpui::Window,
         _cx: &mut gpui::App|
         -> Option<Result<Arc<gpui::RenderImage>, gpui::ImageCacheError>> { None },
    )
}

/// An image source whose asset resolves to a load error.
pub fn failing_source() -> ImageSource {
    ImageSource::from(
        |_window: &mut gpui::Window,
         _cx: &mut gpui::App|
         -> Option<Result<Arc<gpui::RenderImage>, gpui::ImageCacheError>> {
            Some(Err(std::io::Error::other("load failed").into()))
        },
    )
}

/// An image source that decodes successfully through gpui's image machinery.
pub fn png_source() -> ImageSource {
    ImageSource::from(Arc::new(Image::from_bytes(
        ImageFormat::Png,
        ONE_PIXEL_PNG.to_vec(),
    )))
}

#[derive(Clone)]
pub struct AvatarTestConfig {
    pub root_id: &'static str,
    pub source: Option<ImageSource>,
    pub include_fallback: bool,
    pub fallback_delay: Option<Duration>,
}

impl Default for AvatarTestConfig {
    fn default() -> Self {
        Self {
            root_id: "avatar-test",
            source: None,
            include_fallback: true,
            fallback_delay: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct AvatarObservations {
    pub statuses: Vec<AvatarImageLoadingStatus>,
    pub root_states: Vec<AvatarRootStyleState>,
    pub image_states: Vec<AvatarImageStyleState>,
    pub fallback_states: Vec<AvatarFallbackStyleState>,
}

impl AvatarObservations {
    fn begin_render(&mut self) {
        self.root_states.clear();
        self.image_states.clear();
        self.fallback_states.clear();
    }

    pub fn last_root_state(&self) -> AvatarRootStyleState {
        self.root_states
            .last()
            .copied()
            .expect("avatar root state should be observed")
    }
}

pub struct AvatarTestView {
    pub config: AvatarTestConfig,
    observations: Rc<RefCell<AvatarObservations>>,
}

impl AvatarTestView {
    pub fn new(config: AvatarTestConfig) -> Self {
        Self {
            config,
            observations: Rc::new(RefCell::new(AvatarObservations::default())),
        }
    }

    pub fn read_observations(&self) -> AvatarObservations {
        self.observations.borrow().clone()
    }

    pub fn clear_statuses(&self) {
        self.observations.borrow_mut().statuses.clear();
    }
}

impl Render for AvatarTestView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        self.observations.borrow_mut().begin_render();

        let mut root = AvatarRoot::new()
            .id(self.config.root_id)
            .flex()
            .items_center()
            .justify_center()
            .w(px(48.0))
            .h(px(48.0));

        let root_observations = Rc::clone(&self.observations);
        root = root.style_with_state(move |state, root| {
            root_observations.borrow_mut().root_states.push(state);
            root.debug_selector(|| "avatar-root".into())
        });

        if let Some(source) = self.config.source.clone() {
            let status_observations = Rc::clone(&self.observations);
            let image_observations = Rc::clone(&self.observations);
            root = root.child(
                AvatarImage::new(source)
                    .w(px(40.0))
                    .h(px(40.0))
                    .on_loading_status_change(move |status, _window, _cx| {
                        status_observations.borrow_mut().statuses.push(status);
                    })
                    .style_with_state(move |state, image| {
                        image_observations.borrow_mut().image_states.push(state);
                        image.debug_selector(|| "avatar-image".into())
                    }),
            );
        }

        if self.config.include_fallback {
            let fallback_observations = Rc::clone(&self.observations);
            let mut fallback = AvatarFallback::new()
                .w(px(40.0))
                .h(px(40.0))
                .style_with_state(move |state, fallback| {
                    fallback_observations
                        .borrow_mut()
                        .fallback_states
                        .push(state);
                    fallback.debug_selector(|| "avatar-fallback".into())
                })
                .child("LT");

            if let Some(delay) = self.config.fallback_delay {
                fallback = fallback.delay(delay);
            }

            root = root.child(fallback);
        }

        div().size_full().p_4().flex().gap_2().child(root).child(
            div()
                .w(px(30.0))
                .h(px(30.0))
                .debug_selector(|| "avatar-sibling".into())
                .child("S"),
        )
    }
}

pub fn open_avatar(
    cx: &mut TestAppContext,
    config: AvatarTestConfig,
) -> WindowHandle<AvatarTestView> {
    let window = cx.open_window(size(px(320.0), px(180.0)), move |_, _| {
        AvatarTestView::new(config)
    });
    settle(cx);
    window
}

/// Pumps frames until pending asset loads and render-time notifies settle.
pub fn settle(cx: &mut TestAppContext) {
    for _ in 0..4 {
        cx.run_until_parked();
        cx.refresh().expect("test window should refresh");
        cx.update(|_cx| {});
    }
    cx.run_until_parked();
}

pub fn read_observations(
    cx: &mut TestAppContext,
    window: WindowHandle<AvatarTestView>,
) -> AvatarObservations {
    window
        .update(cx, |view, _window, _cx| view.read_observations())
        .expect("avatar test window should be open")
}

pub fn update_config(
    cx: &mut TestAppContext,
    window: WindowHandle<AvatarTestView>,
    update: impl FnOnce(&mut AvatarTestConfig),
) {
    window
        .update(cx, |view, _window, cx| {
            update(&mut view.config);
            cx.notify();
        })
        .expect("avatar test window should be open");
    settle(cx);
}

pub fn clear_statuses(cx: &mut TestAppContext, window: WindowHandle<AvatarTestView>) {
    window
        .update(cx, |view, _window, _cx| view.clear_statuses())
        .expect("avatar test window should be open");
}

pub fn advance_clock(cx: &mut TestAppContext, duration: Duration) {
    cx.executor().advance_clock(duration);
    settle(cx);
}

pub fn debug_bounds(
    cx: &mut TestAppContext,
    window: WindowHandle<AvatarTestView>,
    selector: &'static str,
) -> Option<Bounds<Pixels>> {
    let mut visual = VisualTestContext::from_window(window.into(), cx);

    visual.debug_bounds(selector)
}
