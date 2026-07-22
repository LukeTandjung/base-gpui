use std::rc::Rc;

use gpui::{AnyElement, App, Window};

use crate::popover::PopoverOpenChangeDetails;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverAlign {
    Start,
    #[default]
    Center,
    End,
}

pub type PopoverOpenChangeHandler<P> =
    Rc<dyn Fn(bool, &mut PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type PopoverOpenChangeCompleteHandler<P> =
    Rc<dyn Fn(bool, &PopoverOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type PopoverPayloadContentBuilder<P> =
    Rc<dyn Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static>;

pub struct PopoverProps<P: Clone + 'static> {
    modal: bool,
    on_open_change: Option<PopoverOpenChangeHandler<P>>,
    on_open_change_complete: Option<PopoverOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Clone for PopoverProps<P> {
    fn clone(&self) -> Self {
        Self {
            modal: self.modal,
            on_open_change: self.on_open_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<P: Clone + 'static> PopoverProps<P> {
    pub fn new(
        modal: bool,
        on_open_change: Option<PopoverOpenChangeHandler<P>>,
        on_open_change_complete: Option<PopoverOpenChangeCompleteHandler<P>>,
    ) -> Self {
        Self {
            modal,
            on_open_change,
            on_open_change_complete,
        }
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn on_open_change(&self) -> Option<&PopoverOpenChangeHandler<P>> {
        self.on_open_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&PopoverOpenChangeCompleteHandler<P>> {
        self.on_open_change_complete.as_ref()
    }
}
