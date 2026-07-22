use gpui::{AnyElement, IntoElement};

use crate::combobox::{
    ComboboxArrow, ComboboxBackdrop, ComboboxChipRemove, ComboboxChips, ComboboxClear,
    ComboboxCollection, ComboboxEmpty, ComboboxGroup, ComboboxGroupLabel, ComboboxIcon,
    ComboboxInput, ComboboxInputGroup, ComboboxItem, ComboboxItemIndicator, ComboboxLabel,
    ComboboxList, ComboboxPopup, ComboboxPortal, ComboboxPositioner, ComboboxSeparator,
    ComboboxStatus, ComboboxTrigger, ComboboxValue,
};

macro_rules! impl_combobox_child_from {
    ($child:ident, $variant:ident, $type:ty) => {
        impl<T: Clone + Eq + 'static> From<$type> for $child<T> {
            fn from(value: $type) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

pub enum ComboboxChild<T: Clone + Eq + 'static> {
    Label(Box<ComboboxLabel<T>>),
    Value(Box<ComboboxValue<T>>),
    Input(Box<ComboboxInput<T>>),
    InputGroup(Box<ComboboxInputGroup<T>>),
    Trigger(Box<ComboboxTrigger<T>>),
    Chips(Box<ComboboxChips<T>>),
    Clear(Box<ComboboxClear<T>>),
    Portal(Box<ComboboxPortal<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Label(label) => (*label).into_any_element(),
            Self::Value(value) => (*value).into_any_element(),
            Self::Input(input) => (*input).into_any_element(),
            Self::InputGroup(group) => (*group).into_any_element(),
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Chips(chips) => (*chips).into_any_element(),
            Self::Clear(clear) => (*clear).into_any_element(),
            Self::Portal(portal) => (*portal).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxChild, Label, ComboboxLabel<T>);
impl_combobox_child_from!(ComboboxChild, Value, ComboboxValue<T>);
impl_combobox_child_from!(ComboboxChild, Input, ComboboxInput<T>);
impl_combobox_child_from!(ComboboxChild, InputGroup, ComboboxInputGroup<T>);
impl_combobox_child_from!(ComboboxChild, Trigger, ComboboxTrigger<T>);
impl_combobox_child_from!(ComboboxChild, Chips, ComboboxChips<T>);
impl_combobox_child_from!(ComboboxChild, Clear, ComboboxClear<T>);
impl_combobox_child_from!(ComboboxChild, Portal, ComboboxPortal<T>);

pub enum ComboboxInputGroupChild<T: Clone + Eq + 'static> {
    Input(Box<ComboboxInput<T>>),
    Trigger(Box<ComboboxTrigger<T>>),
    Clear(Box<ComboboxClear<T>>),
    Chips(Box<ComboboxChips<T>>),
    Icon(Box<ComboboxIcon<T>>),
    Value(Box<ComboboxValue<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxInputGroupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Input(input) => (*input).into_any_element(),
            Self::Trigger(trigger) => (*trigger).into_any_element(),
            Self::Clear(clear) => (*clear).into_any_element(),
            Self::Chips(chips) => (*chips).into_any_element(),
            Self::Icon(icon) => (*icon).into_any_element(),
            Self::Value(value) => (*value).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxInputGroupChild, Input, ComboboxInput<T>);
impl_combobox_child_from!(ComboboxInputGroupChild, Trigger, ComboboxTrigger<T>);
impl_combobox_child_from!(ComboboxInputGroupChild, Clear, ComboboxClear<T>);
impl_combobox_child_from!(ComboboxInputGroupChild, Chips, ComboboxChips<T>);
impl_combobox_child_from!(ComboboxInputGroupChild, Icon, ComboboxIcon<T>);
impl_combobox_child_from!(ComboboxInputGroupChild, Value, ComboboxValue<T>);

pub enum ComboboxPortalChild<T: Clone + Eq + 'static> {
    Backdrop(Box<ComboboxBackdrop<T>>),
    Positioner(Box<ComboboxPositioner<T>>),
    Popup(Box<ComboboxPopup<T>>),
    List(Box<ComboboxList<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxPortalChild<T> {
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

impl_combobox_child_from!(ComboboxPortalChild, Backdrop, ComboboxBackdrop<T>);
impl_combobox_child_from!(ComboboxPortalChild, Positioner, ComboboxPositioner<T>);
impl_combobox_child_from!(ComboboxPortalChild, Popup, ComboboxPopup<T>);
impl_combobox_child_from!(ComboboxPortalChild, List, ComboboxList<T>);

pub enum ComboboxPositionerChild<T: Clone + Eq + 'static> {
    Popup(Box<ComboboxPopup<T>>),
    Arrow(Box<ComboboxArrow<T>>),
    List(Box<ComboboxList<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxPositionerChild<T> {
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

impl_combobox_child_from!(ComboboxPositionerChild, Popup, ComboboxPopup<T>);
impl_combobox_child_from!(ComboboxPositionerChild, Arrow, ComboboxArrow<T>);
impl_combobox_child_from!(ComboboxPositionerChild, List, ComboboxList<T>);

pub enum ComboboxPopupChild<T: Clone + Eq + 'static> {
    List(Box<ComboboxList<T>>),
    Arrow(Box<ComboboxArrow<T>>),
    Status(Box<ComboboxStatus<T>>),
    Empty(Box<ComboboxEmpty<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxPopupChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::List(list) => (*list).into_any_element(),
            Self::Arrow(arrow) => (*arrow).into_any_element(),
            Self::Status(status) => (*status).into_any_element(),
            Self::Empty(empty) => (*empty).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxPopupChild, List, ComboboxList<T>);
impl_combobox_child_from!(ComboboxPopupChild, Arrow, ComboboxArrow<T>);
impl_combobox_child_from!(ComboboxPopupChild, Status, ComboboxStatus<T>);
impl_combobox_child_from!(ComboboxPopupChild, Empty, ComboboxEmpty<T>);

pub enum ComboboxListChild<T: Clone + Eq + 'static> {
    Item(Box<ComboboxItem<T>>),
    Group(Box<ComboboxGroup<T>>),
    GroupLabel(Box<ComboboxGroupLabel<T>>),
    Collection(Box<ComboboxCollection<T>>),
    Separator(Box<ComboboxSeparator>),
    Empty(Box<ComboboxEmpty<T>>),
    Status(Box<ComboboxStatus<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxListChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => (*item).into_any_element(),
            Self::Group(group) => (*group).into_any_element(),
            Self::GroupLabel(label) => (*label).into_any_element(),
            Self::Collection(collection) => (*collection).into_any_element(),
            Self::Separator(separator) => (*separator).into_any_element(),
            Self::Empty(empty) => (*empty).into_any_element(),
            Self::Status(status) => (*status).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxListChild, Item, ComboboxItem<T>);
impl_combobox_child_from!(ComboboxListChild, Group, ComboboxGroup<T>);
impl_combobox_child_from!(ComboboxListChild, GroupLabel, ComboboxGroupLabel<T>);
impl_combobox_child_from!(ComboboxListChild, Collection, ComboboxCollection<T>);
impl_combobox_child_from!(ComboboxListChild, Empty, ComboboxEmpty<T>);
impl_combobox_child_from!(ComboboxListChild, Status, ComboboxStatus<T>);

impl<T: Clone + Eq + 'static> From<ComboboxSeparator> for ComboboxListChild<T> {
    fn from(value: ComboboxSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

pub enum ComboboxGroupChild<T: Clone + Eq + 'static> {
    Item(Box<ComboboxItem<T>>),
    GroupLabel(Box<ComboboxGroupLabel<T>>),
    Separator(Box<ComboboxSeparator>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxGroupChild<T> {
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

impl_combobox_child_from!(ComboboxGroupChild, Item, ComboboxItem<T>);
impl_combobox_child_from!(ComboboxGroupChild, GroupLabel, ComboboxGroupLabel<T>);

impl<T: Clone + Eq + 'static> From<ComboboxSeparator> for ComboboxGroupChild<T> {
    fn from(value: ComboboxSeparator) -> Self {
        Self::Separator(Box::new(value))
    }
}

pub enum ComboboxItemChild<T: Clone + Eq + 'static> {
    Indicator(Box<ComboboxItemIndicator<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxItemChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Indicator(indicator) => (*indicator).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxItemChild, Indicator, ComboboxItemIndicator<T>);

pub enum ComboboxChipChild<T: Clone + Eq + 'static> {
    Remove(Box<ComboboxChipRemove<T>>),
    Any(AnyElement),
}

impl<T: Clone + Eq + 'static> IntoElement for ComboboxChipChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Remove(remove) => (*remove).into_any_element(),
            Self::Any(any) => any,
        }
    }
}

impl_combobox_child_from!(ComboboxChipChild, Remove, ComboboxChipRemove<T>);
