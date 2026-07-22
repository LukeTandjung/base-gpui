use gpui::{App, FocusHandle, Window};

use crate::accordion::{
    AccordionContext, AccordionHeaderChild, AccordionItemChild, AccordionItemContext,
    AccordionItemMetadata, AccordionRootChild,
};

pub struct WiredAccordionChildren<T: Clone + Eq + 'static> {
    pub items: Vec<AccordionItemMetadata<T>>,
    pub children: Vec<AccordionRootChild<T>>,
}

pub trait AccordionRootChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_accordion_context(self, context: AccordionContext<T>) -> Self;

    fn wire_accordion_child(
        self,
        _wiring: &mut AccordionChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub trait AccordionItemChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_accordion_item_context(self, context: AccordionItemContext<T>) -> Self;

    fn wire_accordion_item_child(
        self,
        _wiring: &mut AccordionItemChildWiring,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub trait AccordionHeaderChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_accordion_item_context(self, context: AccordionItemContext<T>) -> Self;

    fn wire_accordion_header_child(
        self,
        _wiring: &mut AccordionItemChildWiring,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct AccordionChildWiring<T: Clone + Eq + 'static> {
    next_item_index: usize,
    items: Vec<AccordionItemMetadata<T>>,
}

impl<T: Clone + Eq + 'static> AccordionChildWiring<T> {
    fn new() -> Self {
        Self {
            next_item_index: 0,
            items: Vec::new(),
        }
    }

    pub fn next_item_index(&mut self) -> usize {
        let index = self.next_item_index;
        self.next_item_index += 1;
        index
    }

    pub fn register_item(
        &mut self,
        value: T,
        disabled: bool,
        index: usize,
        trigger_focus_handle: Option<FocusHandle>,
        trigger_focused: bool,
    ) {
        self.items.push(AccordionItemMetadata::new(
            value,
            disabled,
            index,
            trigger_focus_handle,
            trigger_focused,
        ));
    }

    fn finish(self, children: Vec<AccordionRootChild<T>>) -> WiredAccordionChildren<T> {
        WiredAccordionChildren {
            items: self.items,
            children,
        }
    }
}

pub struct AccordionItemChildWiring {
    trigger_focus_handle: Option<FocusHandle>,
    trigger_focused: bool,
}

impl AccordionItemChildWiring {
    pub fn new() -> Self {
        Self {
            trigger_focus_handle: None,
            trigger_focused: false,
        }
    }

    pub fn register_trigger(&mut self, focus_handle: FocusHandle, window: &Window) {
        self.trigger_focused = self.trigger_focused || focus_handle.is_focused(window);
        self.trigger_focus_handle = Some(focus_handle);
    }

    pub fn trigger_focus_handle(&self) -> Option<FocusHandle> {
        self.trigger_focus_handle.clone()
    }

    pub fn trigger_focused(&self) -> bool {
        self.trigger_focused
    }
}

pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<AccordionRootChild<T>>,
    context: AccordionContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredAccordionChildren<T> {
    let mut wiring = AccordionChildWiring::new();
    let children = children
        .into_iter()
        .map(|child| child.wire_accordion_child(&mut wiring, window, cx))
        .map(|child| child.with_accordion_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl<T: Clone + Eq + 'static> AccordionRootChildNode<T> for AccordionRootChild<T> {
    fn with_accordion_context(self, context: AccordionContext<T>) -> Self {
        match self {
            Self::Item(item) => Self::Item(item.with_accordion_context(context)),
        }
    }

    fn wire_accordion_child(
        self,
        wiring: &mut AccordionChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => Self::Item(item.wire_accordion_child(wiring, window, cx)),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionItemChildNode<T> for AccordionItemChild<T> {
    fn with_accordion_item_context(self, context: AccordionItemContext<T>) -> Self {
        match self {
            Self::Header(header) => Self::Header(header.with_accordion_item_context(context)),
            Self::Panel(panel) => Self::Panel(panel.with_accordion_item_context(context)),
        }
    }

    fn wire_accordion_item_child(
        self,
        wiring: &mut AccordionItemChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Header(header) => {
                Self::Header(header.wire_accordion_item_child(wiring, window, cx))
            }
            Self::Panel(panel) => Self::Panel(panel),
        }
    }
}

impl<T: Clone + Eq + 'static> AccordionHeaderChildNode<T> for AccordionHeaderChild<T> {
    fn with_accordion_item_context(self, context: AccordionItemContext<T>) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(trigger.with_accordion_item_context(context)),
        }
    }

    fn wire_accordion_header_child(
        self,
        wiring: &mut AccordionItemChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(trigger.wire_accordion_header_child(wiring, window, cx))
            }
        }
    }
}
