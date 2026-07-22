use std::rc::Rc;

use gpui::{App, Window};

use crate::collapsible::CollapsibleOpenChangeDetails;

pub type CollapsibleOpenChangeHandler =
    Rc<dyn Fn(bool, &mut CollapsibleOpenChangeDetails, &mut Window, &mut App) + 'static>;

#[derive(Clone, Default)]
pub struct CollapsibleProps {
    disabled: bool,
    on_open_change: Option<CollapsibleOpenChangeHandler>,
}

impl CollapsibleProps {
    pub fn new(disabled: bool, on_open_change: Option<CollapsibleOpenChangeHandler>) -> Self {
        Self {
            disabled,
            on_open_change,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn on_open_change(&self) -> Option<&CollapsibleOpenChangeHandler> {
        self.on_open_change.as_ref()
    }
}
