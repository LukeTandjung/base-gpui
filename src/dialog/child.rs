use gpui::{AnyElement, IntoElement};

use crate::dialog::{
    DialogBackdrop, DialogClose, DialogDescription, DialogPopup, DialogPortal, DialogTitle,
    DialogTrigger, DialogViewport,
};

pub enum DialogChild<P: Clone + 'static> {
    Trigger(Box<DialogTrigger<P>>),
    Portal(Box<DialogPortal<P>>),
    Backdrop(Box<DialogBackdrop<P>>),
    Viewport(Box<DialogViewport<P>>),
    Popup(Box<DialogPopup<P>>),
    Title(Box<DialogTitle<P>>),
    Description(Box<DialogDescription<P>>),
    Close(Box<DialogClose<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DialogChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

macro_rules! impl_dialog_child_from {
    ($variant:ident, $type:ty) => {
        impl<P: Clone + 'static> From<$type> for DialogChild<P> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

impl_dialog_child_from!(Trigger, DialogTrigger<P>);
impl_dialog_child_from!(Portal, DialogPortal<P>);
impl_dialog_child_from!(Backdrop, DialogBackdrop<P>);
impl_dialog_child_from!(Viewport, DialogViewport<P>);
impl_dialog_child_from!(Popup, DialogPopup<P>);
impl_dialog_child_from!(Title, DialogTitle<P>);
impl_dialog_child_from!(Description, DialogDescription<P>);
impl_dialog_child_from!(Close, DialogClose<P>);

pub enum DialogPortalChild<P: Clone + 'static> {
    Backdrop(Box<DialogBackdrop<P>>),
    Viewport(Box<DialogViewport<P>>),
    Popup(Box<DialogPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DialogPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DialogBackdrop<P>> for DialogPortalChild<P> {
    fn from(value: DialogBackdrop<P>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogViewport<P>> for DialogPortalChild<P> {
    fn from(value: DialogViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogPopup<P>> for DialogPortalChild<P> {
    fn from(value: DialogPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> DialogPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(backdrop.keep_mounted(true))),
            Self::Viewport(viewport) => Self::Viewport(Box::new(viewport.keep_mounted(true))),
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum DialogViewportChild<P: Clone + 'static> {
    Popup(Box<DialogPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DialogViewportChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DialogPopup<P>> for DialogViewportChild<P> {
    fn from(value: DialogPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> DialogViewportChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum DialogPopupChild<P: Clone + 'static> {
    Title(Box<DialogTitle<P>>),
    Description(Box<DialogDescription<P>>),
    Close(Box<DialogClose<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for DialogPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<DialogTitle<P>> for DialogPopupChild<P> {
    fn from(value: DialogTitle<P>) -> Self {
        Self::Title(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogDescription<P>> for DialogPopupChild<P> {
    fn from(value: DialogDescription<P>) -> Self {
        Self::Description(Box::new(value))
    }
}

impl<P: Clone + 'static> From<DialogClose<P>> for DialogPopupChild<P> {
    fn from(value: DialogClose<P>) -> Self {
        Self::Close(Box::new(value))
    }
}
