use gpui::{AnyElement, IntoElement};

use crate::select::{
    SelectArrow, SelectBackdrop, SelectGroup, SelectGroupLabel, SelectIcon, SelectItem,
    SelectItemIndicator, SelectItemText, SelectLabel, SelectList, SelectPopup, SelectPortal,
    SelectPositioner, SelectScrollDownArrow, SelectScrollUpArrow, SelectSeparator, SelectTrigger,
    SelectValue,
};

pub enum SelectChild<T: Clone + Eq + 'static> {
    Label(Box<SelectLabel<T>>),
    Trigger(Box<SelectTrigger<T>>),
    Portal(Box<SelectPortal<T>>),
    Backdrop(Box<SelectBackdrop<T>>),
    Positioner(Box<SelectPositioner<T>>),
    Popup(Box<SelectPopup<T>>),
    List(Box<SelectList<T>>),
    Group(Box<SelectGroup<T>>),
    GroupLabel(Box<SelectGroupLabel<T>>),
    Item(Box<SelectItem<T>>),
    Separator(Box<SelectSeparator>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Label(label) => (*label).into_any_element(),
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::List(list) => (*list).into_any_element(),
            Self::Group(group) => (*group).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Item(item) => (*item).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

macro_rules! impl_select_child_from {
    ($variant:ident, $type:ty) => {
        impl<T: Clone + Eq + 'static> From<$type> for SelectChild<T> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

impl_select_child_from!(Label, SelectLabel<T>);
impl_select_child_from!(Trigger, SelectTrigger<T>);
impl_select_child_from!(Portal, SelectPortal<T>);
impl_select_child_from!(Backdrop, SelectBackdrop<T>);
impl_select_child_from!(Positioner, SelectPositioner<T>);
impl_select_child_from!(Popup, SelectPopup<T>);
impl_select_child_from!(List, SelectList<T>);
impl_select_child_from!(Group, SelectGroup<T>);
impl_select_child_from!(GroupLabel, SelectGroupLabel<T>);
impl_select_child_from!(Item, SelectItem<T>);

impl<T: Clone + Eq + 'static> From<SelectSeparator> for SelectChild<T> {
    fn from(value: SelectSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

pub enum SelectTriggerChild<T: Clone + Eq + 'static> {
    Value(Box<SelectValue<T>>),
    Icon(Box<SelectIcon<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectTriggerChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Value(value) => (*value).into_any_element(),
            Self::Icon(icon) => (*icon).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectValue<T>> for SelectTriggerChild<T> {
    fn from(value: SelectValue<T>) -> Self {
        Self::Value(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectIcon<T>> for SelectTriggerChild<T> {
    fn from(value: SelectIcon<T>) -> Self {
        Self::Icon(Box::new(value))
    }
}

pub enum SelectPortalChild<T: Clone + Eq + 'static> {
    Backdrop(Box<SelectBackdrop<T>>),
    Positioner(Box<SelectPositioner<T>>),
    Popup(Box<SelectPopup<T>>),
    List(Box<SelectList<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectPortalChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Backdrop(backdrop) => (*backdrop).into_any_element(),
            Self::Positioner(positioner) => (*positioner).into_any_element(),
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::List(list) => (*list).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectBackdrop<T>> for SelectPortalChild<T> {
    fn from(value: SelectBackdrop<T>) -> Self {
        Self::Backdrop(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectPositioner<T>> for SelectPortalChild<T> {
    fn from(value: SelectPositioner<T>) -> Self {
        Self::Positioner(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectPopup<T>> for SelectPortalChild<T> {
    fn from(value: SelectPopup<T>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectList<T>> for SelectPortalChild<T> {
    fn from(value: SelectList<T>) -> Self {
        Self::List(Box::new(value))
    }
}

pub enum SelectPositionerChild<T: Clone + Eq + 'static> {
    Popup(Box<SelectPopup<T>>),
    Arrow(Box<SelectArrow<T>>),
    List(Box<SelectList<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectPositionerChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Popup(popup) => (*popup).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::List(list) => (*list).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectPopup<T>> for SelectPositionerChild<T> {
    fn from(value: SelectPopup<T>) -> Self {
        Self::Popup(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectArrow<T>> for SelectPositionerChild<T> {
    fn from(value: SelectArrow<T>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectList<T>> for SelectPositionerChild<T> {
    fn from(value: SelectList<T>) -> Self {
        Self::List(Box::new(value))
    }
}

pub enum SelectPopupChild<T: Clone + Eq + 'static> {
    List(Box<SelectList<T>>),
    Arrow(Box<SelectArrow<T>>),
    ScrollUpArrow(Box<SelectScrollUpArrow<T>>),
    ScrollDownArrow(Box<SelectScrollDownArrow<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectPopupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::List(list) => (*list).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::ScrollUpArrow(arrow) => (*arrow).into_any_element(),
            Self::ScrollDownArrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectList<T>> for SelectPopupChild<T> {
    fn from(value: SelectList<T>) -> Self {
        Self::List(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectArrow<T>> for SelectPopupChild<T> {
    fn from(value: SelectArrow<T>) -> Self {
        Self::Arrow(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectScrollUpArrow<T>> for SelectPopupChild<T> {
    fn from(value: SelectScrollUpArrow<T>) -> Self {
        Self::ScrollUpArrow(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectScrollDownArrow<T>> for SelectPopupChild<T> {
    fn from(value: SelectScrollDownArrow<T>) -> Self {
        Self::ScrollDownArrow(Box::new(value))
    }
}

pub enum SelectListChild<T: Clone + Eq + 'static> {
    Item(Box<SelectItem<T>>),
    Group(Box<SelectGroup<T>>),
    GroupLabel(Box<SelectGroupLabel<T>>),
    Separator(Box<SelectSeparator>),
    ScrollUpArrow(Box<SelectScrollUpArrow<T>>),
    ScrollDownArrow(Box<SelectScrollDownArrow<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectListChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => (*item).into_any_element(),
            Self::Group(group) => (*group).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::ScrollUpArrow(arrow) => (*arrow).into_any_element(),
            Self::ScrollDownArrow(arrow) => (*arrow).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectItem<T>> for SelectListChild<T> {
    fn from(value: SelectItem<T>) -> Self {
        Self::Item(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectGroup<T>> for SelectListChild<T> {
    fn from(value: SelectGroup<T>) -> Self {
        Self::Group(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectGroupLabel<T>> for SelectListChild<T> {
    fn from(value: SelectGroupLabel<T>) -> Self {
        Self::GroupLabel(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectSeparator> for SelectListChild<T> {
    fn from(value: SelectSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectScrollUpArrow<T>> for SelectListChild<T> {
    fn from(value: SelectScrollUpArrow<T>) -> Self {
        Self::ScrollUpArrow(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectScrollDownArrow<T>> for SelectListChild<T> {
    fn from(value: SelectScrollDownArrow<T>) -> Self {
        Self::ScrollDownArrow(Box::new(value))
    }
}

pub enum SelectGroupChild<T: Clone + Eq + 'static> {
    Item(Box<SelectItem<T>>),
    GroupLabel(Box<SelectGroupLabel<T>>),
    Separator(Box<SelectSeparator>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectGroupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => (*item).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectItem<T>> for SelectGroupChild<T> {
    fn from(value: SelectItem<T>) -> Self {
        Self::Item(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectGroupLabel<T>> for SelectGroupChild<T> {
    fn from(value: SelectGroupLabel<T>) -> Self {
        Self::GroupLabel(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectSeparator> for SelectGroupChild<T> {
    fn from(value: SelectSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

pub enum SelectItemChild<T: Clone + Eq + 'static> {
    Text(Box<SelectItemText<T>>),
    Indicator(Box<SelectItemIndicator<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for SelectItemChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Text(text) => (*text).into_any_element(),
            Self::Indicator(indicator) => (*indicator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl<T: Clone + Eq + 'static> From<SelectItemText<T>> for SelectItemChild<T> {
    fn from(value: SelectItemText<T>) -> Self {
        Self::Text(Box::new(value))
    }
}

impl<T: Clone + Eq + 'static> From<SelectItemIndicator<T>> for SelectItemChild<T> {
    fn from(value: SelectItemIndicator<T>) -> Self {
        Self::Indicator(Box::new(value))
    }
}
