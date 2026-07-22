use gpui::{AnyElement, IntoElement};

use crate::popover::{
    PopoverArrow, PopoverBackdrop, PopoverClose, PopoverDescription, PopoverPopup, PopoverPortal,
    PopoverPositioner, PopoverTitle, PopoverTrigger, PopoverViewport,
};

pub enum PopoverChild<P: Clone + 'static> {
    Trigger(Box<PopoverTrigger<P>>),
    Portal(Box<PopoverPortal<P>>),
    Backdrop(Box<PopoverBackdrop<P>>),
    Positioner(Box<PopoverPositioner<P>>),
    Popup(Box<PopoverPopup<P>>),
    Arrow(Box<PopoverArrow<P>>),
    Title(Box<PopoverTitle<P>>),
    Description(Box<PopoverDescription<P>>),
    Close(Box<PopoverClose<P>>),
    Viewport(Box<PopoverViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PopoverChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

macro_rules! impl_popover_child_from {
    ($variant:ident, $type:ty) => {
        impl<P: Clone + 'static> From<$type> for PopoverChild<P> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

impl_popover_child_from!(Trigger, PopoverTrigger<P>);
impl_popover_child_from!(Portal, PopoverPortal<P>);
impl_popover_child_from!(Backdrop, PopoverBackdrop<P>);
impl_popover_child_from!(Positioner, PopoverPositioner<P>);
impl_popover_child_from!(Popup, PopoverPopup<P>);
impl_popover_child_from!(Arrow, PopoverArrow<P>);
impl_popover_child_from!(Title, PopoverTitle<P>);
impl_popover_child_from!(Description, PopoverDescription<P>);
impl_popover_child_from!(Close, PopoverClose<P>);
impl_popover_child_from!(Viewport, PopoverViewport<P>);

pub enum PopoverPortalChild<P: Clone + 'static> {
    Backdrop(Box<PopoverBackdrop<P>>),
    Positioner(Box<PopoverPositioner<P>>),
    Popup(Box<PopoverPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PopoverPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<PopoverBackdrop<P>> for PopoverPortalChild<P> {
    fn from(value: PopoverBackdrop<P>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverPositioner<P>> for PopoverPortalChild<P> {
    fn from(value: PopoverPositioner<P>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverPopup<P>> for PopoverPortalChild<P> {
    fn from(value: PopoverPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> PopoverPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(backdrop.keep_mounted(true))),
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.keep_mounted_from_portal()))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum PopoverPositionerChild<P: Clone + 'static> {
    Popup(Box<PopoverPopup<P>>),
    Arrow(Box<PopoverArrow<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PopoverPositionerChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<PopoverPopup<P>> for PopoverPositionerChild<P> {
    fn from(value: PopoverPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverArrow<P>> for PopoverPositionerChild<P> {
    fn from(value: PopoverArrow<P>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<P: Clone + 'static> PopoverPositionerChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Arrow(arrow) => Self::Arrow(arrow),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum PopoverPopupChild<P: Clone + 'static> {
    Arrow(Box<PopoverArrow<P>>),
    Title(Box<PopoverTitle<P>>),
    Description(Box<PopoverDescription<P>>),
    Close(Box<PopoverClose<P>>),
    Viewport(Box<PopoverViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PopoverPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Title(title) => (*title).into_any_element(),
            Self::Description(description) => (*description).into_any_element(),
            Self::Close(close) => (*close).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<PopoverArrow<P>> for PopoverPopupChild<P> {
    fn from(value: PopoverArrow<P>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverTitle<P>> for PopoverPopupChild<P> {
    fn from(value: PopoverTitle<P>) -> Self {
        Self::Title(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverDescription<P>> for PopoverPopupChild<P> {
    fn from(value: PopoverDescription<P>) -> Self {
        Self::Description(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverClose<P>> for PopoverPopupChild<P> {
    fn from(value: PopoverClose<P>) -> Self {
        Self::Close(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PopoverViewport<P>> for PopoverPopupChild<P> {
    fn from(value: PopoverViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}
