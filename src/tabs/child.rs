use gpui::{AnyElement, IntoElement};

use crate::tabs::{TabsIndicator, TabsList, TabsPanel, TabsTab};

pub enum TabsChild<T: Clone + Eq + 'static> {
    List(TabsList<T>),
    Panel(TabsPanel<T>),
    Indicator(TabsIndicator<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for TabsChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::List(list) => list.into_any_element(),
            Self::Panel(panel) => panel.into_any_element(),
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<TabsList<T>> for TabsChild<T> {
    fn from(value: TabsList<T>) -> Self {
        Self::List(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsPanel<T>> for TabsChild<T> {
    fn from(value: TabsPanel<T>) -> Self {
        Self::Panel(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsIndicator<T>> for TabsChild<T> {
    fn from(value: TabsIndicator<T>) -> Self {
        Self::Indicator(value)
    }
}

pub enum TabsListChild<T: Clone + Eq + 'static> {
    Tab(TabsTab<T>),
    Indicator(TabsIndicator<T>),
}

impl<T: Clone + Eq + 'static> IntoElement for TabsListChild<T> {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Tab(tab) => tab.into_any_element(),
            Self::Indicator(indicator) => indicator.into_any_element(),
        }
    }
}

impl<T: Clone + Eq + 'static> From<TabsTab<T>> for TabsListChild<T> {
    fn from(value: TabsTab<T>) -> Self {
        Self::Tab(value)
    }
}

impl<T: Clone + Eq + 'static> From<TabsIndicator<T>> for TabsListChild<T> {
    fn from(value: TabsIndicator<T>) -> Self {
        Self::Indicator(value)
    }
}
