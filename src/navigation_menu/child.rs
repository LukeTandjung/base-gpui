use gpui::{AnyElement, IntoElement};

use crate::navigation_menu::{
    NavigationMenuArrow, NavigationMenuBackdrop, NavigationMenuContent, NavigationMenuIcon,
    NavigationMenuItem, NavigationMenuLink, NavigationMenuList, NavigationMenuPopup,
    NavigationMenuPortal, NavigationMenuPositioner, NavigationMenuTrigger, NavigationMenuViewport,
};

pub enum NavigationMenuChild<T: Clone + Eq + 'static> {
    List(Box<NavigationMenuList<T>>),
    Portal(Box<NavigationMenuPortal<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::List(list) => (*list).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuList<T>> for NavigationMenuChild<T> {
    fn from(value: NavigationMenuList<T>) -> Self {
        Self::List(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuPortal<T>> for NavigationMenuChild<T> {
    fn from(value: NavigationMenuPortal<T>) -> Self {
        Self::Portal(Box::new(value))
    }
}

pub enum NavigationMenuListChild<T: Clone + Eq + 'static> {
    Item(Box<NavigationMenuItem<T>>),
    Link(Box<NavigationMenuLink<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuListChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => (*item).into_any_element(),
            Self::Link(link) => (*link).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuItem<T>> for NavigationMenuListChild<T> {
    fn from(value: NavigationMenuItem<T>) -> Self {
        Self::Item(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuLink<T>> for NavigationMenuListChild<T> {
    fn from(value: NavigationMenuLink<T>) -> Self {
        Self::Link(Box::new(value))
    }
}

/// Item children: a trigger, plus the item's content — which is *not*
/// rendered in place; child wiring routes it to the popup viewport.
pub enum NavigationMenuItemChild<T: Clone + Eq + 'static> {
    Trigger(Box<NavigationMenuTrigger<T>>),
    Content(Box<NavigationMenuContent<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> From<NavigationMenuTrigger<T>> for NavigationMenuItemChild<T> {
    fn from(value: NavigationMenuTrigger<T>) -> Self {
        Self::Trigger(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuContent<T>> for NavigationMenuItemChild<T> {
    fn from(value: NavigationMenuContent<T>) -> Self {
        Self::Content(Box::new(value))
    }
}

pub enum NavigationMenuPortalChild<T: Clone + Eq + 'static> {
    Backdrop(Box<NavigationMenuBackdrop<T>>),
    Positioner(Box<NavigationMenuPositioner<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuPortalChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuBackdrop<T>> for NavigationMenuPortalChild<T> {
    fn from(value: NavigationMenuBackdrop<T>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuPositioner<T>> for NavigationMenuPortalChild<T> {
    fn from(value: NavigationMenuPositioner<T>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

pub enum NavigationMenuPositionerChild<T: Clone + Eq + 'static> {
    Popup(Box<NavigationMenuPopup<T>>),
    Arrow(Box<NavigationMenuArrow<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuPositionerChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuPopup<T>> for NavigationMenuPositionerChild<T> {
    fn from(value: NavigationMenuPopup<T>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuArrow<T>> for NavigationMenuPositionerChild<T> {
    fn from(value: NavigationMenuArrow<T>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

pub enum NavigationMenuPopupChild<T: Clone + Eq + 'static> {
    Viewport(Box<NavigationMenuViewport<T>>),
    Arrow(Box<NavigationMenuArrow<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuPopupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Viewport(viewport) => (*viewport).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuViewport<T>> for NavigationMenuPopupChild<T> {
    fn from(value: NavigationMenuViewport<T>) -> Self {
        Self::Viewport(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuArrow<T>> for NavigationMenuPopupChild<T> {
    fn from(value: NavigationMenuArrow<T>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

/// Trigger children: an optional icon plus arbitrary visual children.
pub enum NavigationMenuTriggerChild<T: Clone + Eq + 'static> {
    Icon(Box<NavigationMenuIcon<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for NavigationMenuTriggerChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Icon(icon) => (*icon).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<NavigationMenuIcon<T>> for NavigationMenuTriggerChild<T> {
    fn from(value: NavigationMenuIcon<T>) -> Self {
        Self::Icon(Box::new(value))
    }
}
