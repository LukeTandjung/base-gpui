use gpui::{AnyElement, IntoElement};

use crate::dialog::{DialogClose, DialogDescription, DialogTitle, DialogTrigger};
use crate::drawer::{
    DrawerBackdrop, DrawerContent, DrawerPopup, DrawerPortal, DrawerSwipeArea, DrawerViewport,
};

pub enum DrawerChild<P: Clone + 'static> {
    Trigger(Box<DialogTrigger<P>>),
    SwipeArea(Box<DrawerSwipeArea<P>>),
    Portal(Box<DrawerPortal<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DrawerChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::SwipeArea(swipe_area) => (*swipe_area).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DialogTrigger<P>> for DrawerChild<P> {
    fn from(value: DialogTrigger<P>) -> Self {
        Self::Trigger(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DrawerSwipeArea<P>> for DrawerChild<P> {
    fn from(value: DrawerSwipeArea<P>) -> Self {
        Self::SwipeArea(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DrawerPortal<P>> for DrawerChild<P> {
    fn from(value: DrawerPortal<P>) -> Self {
        Self::Portal(Box::new(value))
    }
}

pub enum DrawerPortalChild<P: Clone + 'static> {
    Backdrop(Box<DrawerBackdrop<P>>),
    Viewport(Box<DrawerViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DrawerPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DrawerBackdrop<P>> for DrawerPortalChild<P> {
    fn from(value: DrawerBackdrop<P>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DrawerViewport<P>> for DrawerPortalChild<P> {
    fn from(value: DrawerViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}

impl<P: Clone + 'static> DrawerPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(backdrop.keep_mounted(true))),
            Self::Viewport(viewport) => Self::Viewport(Box::new(viewport.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum DrawerViewportChild<P: Clone + 'static> {
    Popup(Box<DrawerPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DrawerViewportChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DrawerPopup<P>> for DrawerViewportChild<P> {
    fn from(value: DrawerPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> DrawerViewportChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum DrawerPopupChild<P: Clone + 'static> {
    Content(Box<DrawerContent<P>>),
    Title(Box<DialogTitle<P>>),
    Description(Box<DialogDescription<P>>),
    Close(Box<DialogClose<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DrawerPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Content(content) => (*content).into_any_element(),
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DrawerContent<P>> for DrawerPopupChild<P> {
    fn from(value: DrawerContent<P>) -> Self {
        Self::Content(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogTitle<P>> for DrawerPopupChild<P> {
    fn from(value: DialogTitle<P>) -> Self {
        Self::Title(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogDescription<P>> for DrawerPopupChild<P> {
    fn from(value: DialogDescription<P>) -> Self {
        Self::Description(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogClose<P>> for DrawerPopupChild<P> {
    fn from(value: DialogClose<P>) -> Self {
        Self::Close(Box::new(value))
    }
}
