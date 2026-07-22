use gpui::{AnyElement, IntoElement};

use crate::collapsible::{CollapsiblePanel, CollapsibleTrigger};

pub enum CollapsibleChild {
    Trigger(CollapsibleTrigger),
    Panel(CollapsiblePanel),
}

impl IntoElement for CollapsibleChild {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        match self {
            Self::Trigger(trigger) => trigger.into_any_element(),
            Self::Panel(panel) => panel.into_any_element(),
        }
    }
}

impl From<CollapsibleTrigger> for CollapsibleChild {
    fn from(value: CollapsibleTrigger) -> Self {
        Self::Trigger(value)
    }
}

impl From<CollapsiblePanel> for CollapsibleChild {
    fn from(value: CollapsiblePanel) -> Self {
        Self::Panel(value)
    }
}
