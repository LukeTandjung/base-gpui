use std::rc::Rc;

use gpui::{App, Window};

use crate::dialog::{DialogModalMode, DialogOpenChangeDetails};

pub type DialogOpenChangeHandler<P> =
    Rc<dyn Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type DialogOpenChangeCompleteHandler<P> =
    Rc<dyn Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub struct DialogProps<P: Clone + 'static> {
    modal_mode: DialogModalMode,
    disable_pointer_dismissal: bool,
    on_open_change: Option<DialogOpenChangeHandler<P>>,
    on_open_change_complete: Option<DialogOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Clone for DialogProps<P> {
    fn clone(&self) -> Self {
        Self {
            modal_mode: self.modal_mode,
            disable_pointer_dismissal: self.disable_pointer_dismissal,
            on_open_change: self.on_open_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<P: Clone + 'static> DialogProps<P> {
    pub fn new(
        modal_mode: DialogModalMode,
        disable_pointer_dismissal: bool,
        on_open_change: Option<DialogOpenChangeHandler<P>>,
        on_open_change_complete: Option<DialogOpenChangeCompleteHandler<P>>,
    ) -> Self {
        Self {
            modal_mode,
            disable_pointer_dismissal,
            on_open_change,
            on_open_change_complete,
        }
    }

    pub fn modal_mode(&self) -> DialogModalMode {
        self.modal_mode
    }

    pub fn disable_pointer_dismissal(&self) -> bool {
        self.disable_pointer_dismissal
    }

    pub fn on_open_change(&self) -> Option<&DialogOpenChangeHandler<P>> {
        self.on_open_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&DialogOpenChangeCompleteHandler<P>> {
        self.on_open_change_complete.as_ref()
    }
}
