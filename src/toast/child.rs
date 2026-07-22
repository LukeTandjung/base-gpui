use gpui::{AnyElement, IntoElement};

use crate::toast::child_wiring::{ToastContextNode, ToastPartNode};
use crate::toast::{
    ToastAction, ToastClose, ToastContent, ToastContext, ToastDescription, ToastId, ToastPortal,
    ToastTitle, ToastViewport,
};

/// Children of `ToastProvider`.
pub enum ToastProviderChild<P: Clone + 'static> {
    Viewport(Box<ToastViewport<P>>),
    Portal(Box<ToastPortal<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> ToastProviderChild<P> {
    pub fn with_toast_context(self, context: ToastContext<P>) -> Self {
        match self {
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_toast_context(context)))
            }
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_toast_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> IntoElement for ToastProviderChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<ToastViewport<P>> for ToastProviderChild<P> {
    fn from(value: ToastViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}

impl<P: Clone + 'static> From<ToastPortal<P>> for ToastProviderChild<P> {
    fn from(value: ToastPortal<P>) -> Self {
        Self::Portal(Box::new(value))
    }
}

/// Children of `ToastPortal`.
pub enum ToastPortalChild<P: Clone + 'static> {
    Viewport(Box<ToastViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> ToastPortalChild<P> {
    pub fn with_toast_context(self, context: ToastContext<P>) -> Self {
        match self {
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_toast_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> IntoElement for ToastPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<ToastViewport<P>> for ToastPortalChild<P> {
    fn from(value: ToastViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}

/// Children of `ToastRoot` (Base UI examples allow arbitrary children, hence
/// the `Any` escape hatch).
pub enum ToastRootChild<P: Clone + 'static> {
    Content(Box<ToastContent<P>>),
    Title(Box<ToastTitle<P>>),
    Description(Box<ToastDescription<P>>),
    Close(Box<ToastClose<P>>),
    Action(Box<ToastAction<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> ToastRootChild<P> {
    pub fn with_toast(self, context: ToastContext<P>, id: ToastId) -> Self {
        match self {
            Self::Content(content) => Self::Content(Box::new(content.with_toast(context, id))),
            Self::Title(title) => Self::Title(Box::new(title.with_toast(context, id))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_toast(context, id)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_toast(context, id))),
            Self::Action(action) => Self::Action(Box::new(action.with_toast(context, id))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> IntoElement for ToastRootChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Content(content) => (*content).into_any_element(),
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Action(action) => (*action).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<ToastContent<P>> for ToastRootChild<P> {
    fn from(value: ToastContent<P>) -> Self {
        Self::Content(Box::new(value))
    }
}

impl<P: Clone + 'static> From<ToastTitle<P>> for ToastRootChild<P> {
    fn from(value: ToastTitle<P>) -> Self {
        Self::Title(Box::new(value))
    }
}

impl<P: Clone + 'static> From<ToastDescription<P>> for ToastRootChild<P> {
    fn from(value: ToastDescription<P>) -> Self {
        Self::Description(Box::new(value))
    }
}

impl<P: Clone + 'static> From<ToastClose<P>> for ToastRootChild<P> {
    fn from(value: ToastClose<P>) -> Self {
        Self::Close(Box::new(value))
    }
}

impl<P: Clone + 'static> From<ToastAction<P>> for ToastRootChild<P> {
    fn from(value: ToastAction<P>) -> Self {
        Self::Action(Box::new(value))
    }
}
