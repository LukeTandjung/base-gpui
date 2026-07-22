use gpui::{App, FocusHandle, Window};

use crate::tabs::{TabsChild, TabsContext, TabsListChild, TabsTabMetadata};

pub struct WiredTabsChildren<T: Clone + Eq + 'static> {
    pub tabs: Vec<TabsTabMetadata<T>>,
    pub tab_focus_handles: Vec<(usize, FocusHandle)>,
    pub children: Vec<TabsChild<T>>,
}

pub trait TabsChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_tabs_context(self, context: TabsContext<T>) -> Self;

    fn wire_tabs_child(
        self,
        _wiring: &mut TabsChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }

    fn tab_index(&self) -> Option<usize> {
        None
    }
}

pub struct TabsChildWiring<T: Clone + Eq + 'static> {
    next_tab_index: usize,
    tabs: Vec<TabsTabMetadata<T>>,
    tab_focus_handles: Vec<(usize, FocusHandle)>,
}

impl<T: Clone + Eq + 'static> TabsChildWiring<T> {
    pub fn new() -> Self {
        Self {
            next_tab_index: 0,
            tabs: Vec::new(),
            tab_focus_handles: Vec::new(),
        }
    }

    pub fn wire_list_children(
        &mut self,
        children: Vec<TabsListChild<T>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<TabsListChild<T>> {
        children
            .into_iter()
            .map(|child| child.wire_tabs_child(self, window, cx))
            .collect()
    }

    pub fn register_tab(
        &mut self,
        value: Option<T>,
        disabled: bool,
        focus_handle: FocusHandle,
    ) -> usize {
        let index = self.next_tab_index;
        self.next_tab_index += 1;

        if let Some(value) = value {
            self.tabs.push(TabsTabMetadata::new(value, disabled, index));
        }

        self.tab_focus_handles.push((index, focus_handle));

        index
    }

    fn finish(self, children: Vec<TabsChild<T>>) -> WiredTabsChildren<T> {
        WiredTabsChildren {
            tabs: self.tabs,
            tab_focus_handles: self.tab_focus_handles,
            children,
        }
    }
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<TabsChild<T>>,
    context: TabsContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredTabsChildren<T> {
    let mut wiring = TabsChildWiring::new();
    let children = children
        .into_iter()
        .map(|child| child.wire_tabs_child(&mut wiring, window, cx))
        .map(|child| child.with_tabs_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<T: Clone + Eq + 'static> TabsChildNode<T> for TabsChild<T> {
    fn with_tabs_context(self, context: TabsContext<T>) -> Self {
        match self {
            Self::List(list) => Self::List(list.with_tabs_context(context)),
            Self::Panel(panel) => Self::Panel(panel.with_tabs_context(context)),
            Self::Indicator(indicator) => Self::Indicator(indicator.with_tabs_context(context)),
        }
    }

    fn wire_tabs_child(
        self,
        wiring: &mut TabsChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::List(list) => Self::List(list.wire_tabs_child(wiring, window, cx)),
            Self::Panel(panel) => Self::Panel(panel),
            Self::Indicator(indicator) => Self::Indicator(indicator),
        }
    }
}

impl<T: Clone + Eq + 'static> TabsChildNode<T> for TabsListChild<T> {
    fn with_tabs_context(self, context: TabsContext<T>) -> Self {
        match self {
            Self::Tab(tab) => Self::Tab(tab.with_tabs_context(context)),
            Self::Indicator(indicator) => Self::Indicator(indicator.with_tabs_context(context)),
        }
    }

    fn wire_tabs_child(
        self,
        wiring: &mut TabsChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Tab(tab) => Self::Tab(tab.wire_tabs_child(wiring, window, cx)),
            Self::Indicator(indicator) => Self::Indicator(indicator),
        }
    }

    fn tab_index(&self) -> Option<usize> {
        match self {
            Self::Tab(tab) => tab.tab_index(),
            Self::Indicator(_) => None,
        }
    }
}
