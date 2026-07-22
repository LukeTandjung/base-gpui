use gpui::{App, FocusHandle, Window};

use crate::collapsible::{CollapsibleChild, CollapsibleContext};

pub struct WiredCollapsibleChildren {
    pub trigger_focused: bool,
    pub children: Vec<CollapsibleChild>,
}

pub trait CollapsibleChildNode: Sized {
    fn with_collapsible_context(self, context: CollapsibleContext) -> Self;

    fn wire_collapsible_child(
        self,
        _wiring: &mut CollapsibleChildWiring,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct CollapsibleChildWiring {
    trigger_focused: bool,
}

impl CollapsibleChildWiring {
    fn new() -> Self {
        Self {
            trigger_focused: false,
        }
    }

    pub fn register_trigger(&mut self, focus_handle: &FocusHandle, window: &Window) {
        self.trigger_focused = self.trigger_focused || focus_handle.is_focused(window);
    }

    fn finish(self, children: Vec<CollapsibleChild>) -> WiredCollapsibleChildren {
        WiredCollapsibleChildren {
            trigger_focused: self.trigger_focused,
            children,
        }
    }
}

pub fn wire_children(
    children: Vec<CollapsibleChild>,
    context: CollapsibleContext,
    window: &mut Window,
    cx: &mut App,
) -> WiredCollapsibleChildren {
    let mut wiring = CollapsibleChildWiring::new();
    let children = children
        .into_iter()
        .map(|child| child.wire_collapsible_child(&mut wiring, window, cx))
        .map(|child| child.with_collapsible_context(context.clone()))
        .collect();

    wiring.finish(children)
}

impl CollapsibleChildNode for CollapsibleChild {
    fn with_collapsible_context(self, context: CollapsibleContext) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(trigger.with_collapsible_context(context)),
            Self::Panel(panel) => Self::Panel(panel.with_collapsible_context(context)),
        }
    }

    fn wire_collapsible_child(
        self,
        wiring: &mut CollapsibleChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(trigger.wire_collapsible_child(wiring, window, cx))
            }
            Self::Panel(panel) => Self::Panel(panel),
        }
    }
}
