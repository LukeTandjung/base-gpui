use gpui::{AnyElement, IntoElement};

use crate::preview_card::{
    PreviewCardArrow, PreviewCardBackdrop, PreviewCardPopup, PreviewCardPortal,
    PreviewCardPositioner, PreviewCardTrigger, PreviewCardViewport,
};

pub enum PreviewCardChild<P: Clone + 'static> {
    Trigger(Box<PreviewCardTrigger<P>>),
    Portal(Box<PreviewCardPortal<P>>),
    Positioner(Box<PreviewCardPositioner<P>>),
    Popup(Box<PreviewCardPopup<P>>),
    Backdrop(Box<PreviewCardBackdrop<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PreviewCardChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

macro_rules! impl_preview_card_child_from {
    ($variant:ident, $type:ty) => {
        impl<P: Clone + 'static> From<$type> for PreviewCardChild<P> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

impl_preview_card_child_from!(Trigger, PreviewCardTrigger<P>);
impl_preview_card_child_from!(Portal, PreviewCardPortal<P>);
impl_preview_card_child_from!(Positioner, PreviewCardPositioner<P>);
impl_preview_card_child_from!(Popup, PreviewCardPopup<P>);
impl_preview_card_child_from!(Backdrop, PreviewCardBackdrop<P>);

pub enum PreviewCardPortalChild<P: Clone + 'static> {
    Backdrop(Box<PreviewCardBackdrop<P>>),
    Positioner(Box<PreviewCardPositioner<P>>),
    Popup(Box<PreviewCardPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PreviewCardPortalChild<P> {
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

impl<P: Clone + 'static> From<PreviewCardBackdrop<P>> for PreviewCardPortalChild<P> {
    fn from(value: PreviewCardBackdrop<P>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PreviewCardPositioner<P>> for PreviewCardPortalChild<P> {
    fn from(value: PreviewCardPositioner<P>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PreviewCardPopup<P>> for PreviewCardPortalChild<P> {
    fn from(value: PreviewCardPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> PreviewCardPortalChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(backdrop.keep_mounted(true))),
            Self::Positioner(positioner) => {
                Self::Positioner(Box::new(positioner.keep_mounted(true)))
            }
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum PreviewCardPositionerChild<P: Clone + 'static> {
    Popup(Box<PreviewCardPopup<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PreviewCardPositionerChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<PreviewCardPopup<P>> for PreviewCardPositionerChild<P> {
    fn from(value: PreviewCardPopup<P>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<P: Clone + 'static> PreviewCardPositionerChild<P> {
    pub fn keep_mounted_from_portal(self) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.keep_mounted(true))),
            Self::Any(any) => Self::Any(any),
        }
    }
}

pub enum PreviewCardPopupChild<P: Clone + 'static> {
    Arrow(Box<PreviewCardArrow<P>>),
    Viewport(Box<PreviewCardViewport<P>>),
    Any(AnyElement),
}

impl<P: Clone + 'static> IntoElement for PreviewCardPopupChild<P> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<P: Clone + 'static> From<PreviewCardArrow<P>> for PreviewCardPopupChild<P> {
    fn from(value: PreviewCardArrow<P>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<P: Clone + 'static> From<PreviewCardViewport<P>> for PreviewCardPopupChild<P> {
    fn from(value: PreviewCardViewport<P>) -> Self {
        Self::Viewport(Box::new(value))
    }
}
