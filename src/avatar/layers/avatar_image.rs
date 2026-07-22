use std::rc::Rc;
use std::sync::Arc;

use gpui::{
    div, hash, img, App, Div, Empty, ImageSource, ImgResourceLoader, IntoElement, ParentElement,
    RenderOnce, Resource, StyleRefinement, Styled, Window,
};

use crate::avatar::{
    child_wiring::{AvatarChildNode, AvatarChildWiring},
    AvatarContext, AvatarImageLoadingStatus, AvatarImageStyleState,
};

pub type AvatarLoadingStatusChangeHandler =
    Rc<dyn Fn(AvatarImageLoadingStatus, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct AvatarImage {
    base: Div,
    source: ImageSource,
    context: Option<AvatarContext>,
    on_loading_status_change: Option<AvatarLoadingStatusChangeHandler>,
    style_with_state: Option<Rc<dyn Fn(AvatarImageStyleState, Div) -> Div + 'static>>,
}

impl Styled for AvatarImage {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for AvatarImage {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context else {
            return Empty.into_any_element();
        };

        let status = derive_status(&self.source, window, cx);
        let outcome = context.update(cx, |runtime| runtime.report_image_status(status));

        if outcome.changed() {
            if let Some(on_loading_status_change) = self.on_loading_status_change.as_ref() {
                on_loading_status_change(outcome.status(), window, cx);
            }
        }

        let (visible, state) = context.read(cx, |runtime| {
            (runtime.image_visible(), runtime.image_state())
        });

        if !visible {
            return Empty.into_any_element();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.child(img(self.source).size_full()).into_any_element()
    }
}

impl AvatarChildNode for AvatarImage {
    fn with_avatar_context(mut self, context: AvatarContext) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_avatar_child(self, wiring: &mut AvatarChildWiring) -> Self {
        wiring.register_image(source_key(&self.source));
        self
    }
}

impl AvatarImage {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            base: div(),
            source: source.into(),
            context: None,
            on_loading_status_change: None,
            style_with_state: None,
        }
    }

    pub fn on_loading_status_change(
        mut self,
        on_loading_status_change: impl Fn(AvatarImageLoadingStatus, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_loading_status_change = Some(Rc::new(on_loading_status_change));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AvatarImageStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

/// Derives the loading status GPUI-natively from the image source's asset
/// state, mirroring Base UI's probe result: unresolved asset means `Loading`,
/// a decoded image means `Loaded`, a failed or missing source means `Error`.
fn derive_status(
    source: &ImageSource,
    window: &mut Window,
    cx: &mut App,
) -> AvatarImageLoadingStatus {
    match source {
        ImageSource::Resource(resource) => {
            if resource_is_empty(resource) {
                return AvatarImageLoadingStatus::Error;
            }

            match window.use_asset::<ImgResourceLoader>(resource, cx) {
                None => AvatarImageLoadingStatus::Loading,
                Some(Ok(_)) => AvatarImageLoadingStatus::Loaded,
                Some(Err(_)) => AvatarImageLoadingStatus::Error,
            }
        }
        ImageSource::Custom(load) => match load(window, cx) {
            None => AvatarImageLoadingStatus::Loading,
            Some(Ok(_)) => AvatarImageLoadingStatus::Loaded,
            Some(Err(_)) => AvatarImageLoadingStatus::Error,
        },
        ImageSource::Render(_) => AvatarImageLoadingStatus::Loaded,
        ImageSource::Image(image) => match Arc::clone(image).use_render_image(window, cx) {
            Some(_) => AvatarImageLoadingStatus::Loaded,
            None => AvatarImageLoadingStatus::Loading,
        },
    }
}

fn resource_is_empty(resource: &Resource) -> bool {
    match resource {
        Resource::Uri(uri) => uri.is_empty(),
        Resource::Path(path) => path.as_os_str().is_empty(),
        Resource::Embedded(path) => path.is_empty(),
    }
}

/// Identifies the image source so a source change resets the status machine.
fn source_key(source: &ImageSource) -> u64 {
    match source {
        ImageSource::Resource(resource) => hash(resource),
        ImageSource::Render(render) => Arc::as_ptr(render) as usize as u64,
        ImageSource::Image(image) => image.id(),
        ImageSource::Custom(load) => Arc::as_ptr(load) as *const () as usize as u64,
    }
}
