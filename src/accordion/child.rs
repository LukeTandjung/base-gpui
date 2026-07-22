use gpui::{AnyElement, IntoElement};

use crate::accordion::{AccordionHeader, AccordionItem, AccordionPanel, AccordionTrigger};

pub enum AccordionRootChild<T: Clone + Eq + 'static> {
    Item(AccordionItem<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for AccordionRootChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Item(item) => item.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<AccordionItem<T>> for AccordionRootChild<T> {
    fn from(value: AccordionItem<T>) -> Self {
        Self::Item(value)
    }
}

pub enum AccordionItemChild<T: Clone + Eq + 'static> {
    Header(AccordionHeader<T>),
    Panel(AccordionPanel<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for AccordionItemChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Header(header) => header.into_any_element(),
            Self::Panel(panel) => panel.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<AccordionHeader<T>> for AccordionItemChild<T> {
    fn from(value: AccordionHeader<T>) -> Self {
        Self::Header(value)
    }
}

impl<T: Clone + Eq + 'static> From<AccordionPanel<T>> for AccordionItemChild<T> {
    fn from(value: AccordionPanel<T>) -> Self {
        Self::Panel(value)
    }
}

pub enum AccordionHeaderChild<T: Clone + Eq + 'static> {
    Trigger(AccordionTrigger<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for AccordionHeaderChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => trigger.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<AccordionTrigger<T>> for AccordionHeaderChild<T> {
    fn from(value: AccordionTrigger<T>) -> Self {
        Self::Trigger(value)
    }
}
