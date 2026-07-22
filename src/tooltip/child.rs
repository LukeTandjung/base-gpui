use gpui::{AnyElement, IntoElement};

use crate::tooltip::{
    TooltipPopup, TooltipPortal, TooltipPositioner, TooltipTrigger, TooltipViewport,
};

pub enum TooltipChild<P: Clone + 'static> {
    Trigger(Box<TooltipTrigger<P>>),
    Portal(Box<TooltipPortal<P>>),
    Positioner(Box<TooltipPositioner<P>>),
    Popup(Box<TooltipPopup<P>>),
    Viewport(Box<TooltipViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for TooltipChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

macro_rules! impl_tooltip_child_from {
    ($variant:ident, $type:ty) => {
        impl<P: Clone + 'static> From<$type> for TooltipChild<P> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

impl_tooltip_child_from!(Trigger, TooltipTrigger<P>);
impl_tooltip_child_from!(Portal, TooltipPortal<P>);
impl_tooltip_child_from!(Positioner, TooltipPositioner<P>);
impl_tooltip_child_from!(Popup, TooltipPopup<P>);
impl_tooltip_child_from!(Viewport, TooltipViewport<P>);

pub enum TooltipPortalChild<P: Clone + 'static> {
    Positioner(Box<TooltipPositioner<P>>),
    Popup(Box<TooltipPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for TooltipPortalChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<TooltipPositioner<P>> for TooltipPortalChild<P> {
    fn from(value: TooltipPositioner<P>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

impl<P: Clone + 'static> From<TooltipPopup<P>> for TooltipPortalChild<P> {
    fn from(value: TooltipPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> TooltipPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.keep_mounted(true)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum TooltipPositionerChild<P: Clone + 'static> {
    Popup(Box<TooltipPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for TooltipPositionerChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<TooltipPopup<P>> for TooltipPositionerChild<P> {
    fn from(value: TooltipPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> TooltipPositionerChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum TooltipPopupChild<P: Clone + 'static> {
    Viewport(Box<TooltipViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for TooltipPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<TooltipViewport<P>> for TooltipPopupChild<P> {
    fn from(value: TooltipViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}
