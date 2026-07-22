use std::rc::Rc;

use gpui::{App, Window};

use crate::menu::{MenuItemChangeDetails, MenuOpenChangeDetails};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
    InlineStart,
    InlineEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuAlign {
    Start,
    #[default]
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MenuOrientation {
    #[default]
    Vertical,
    Horizontal,
}

pub type MenuOpenChangeHandler<P> =
    Rc<dyn Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type MenuOpenChangeCompleteHandler<P> =
    Rc<dyn Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static>;

pub type MenuCheckedChangeHandler =
    Rc<dyn Fn(bool, &mut MenuItemChangeDetails, &mut Window, &mut App) + 'static>;

pub type MenuValueChangeHandler<V> =
    Rc<dyn Fn(V, &mut MenuItemChangeDetails, &mut Window, &mut App) + 'static>;

pub type MenuActivationHandler = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

pub struct MenuProps<P: Clone + 'static> {
    disabled: bool,
    modal: bool,
    loop_focus: bool,
    orientation: MenuOrientation,
    close_parent_on_esc: bool,
    highlight_item_on_hover: bool,
    on_open_change: Option<MenuOpenChangeHandler<P>>,
    on_open_change_complete: Option<MenuOpenChangeCompleteHandler<P>>,
}

impl<P: Clone + 'static> Clone for MenuProps<P> {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            modal: self.modal,
            loop_focus: self.loop_focus,
            orientation: self.orientation,
            close_parent_on_esc: self.close_parent_on_esc,
            highlight_item_on_hover: self.highlight_item_on_hover,
            on_open_change: self.on_open_change.clone(),
            on_open_change_complete: self.on_open_change_complete.clone(),
        }
    }
}

impl<P: Clone + 'static> MenuProps<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disabled: bool,
        modal: bool,
        loop_focus: bool,
        orientation: MenuOrientation,
        close_parent_on_esc: bool,
        highlight_item_on_hover: bool,
        on_open_change: Option<MenuOpenChangeHandler<P>>,
        on_open_change_complete: Option<MenuOpenChangeCompleteHandler<P>>,
    ) -> Self {
        Self {
            disabled,
            modal,
            loop_focus,
            orientation,
            close_parent_on_esc,
            highlight_item_on_hover,
            on_open_change,
            on_open_change_complete,
        }
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn modal(&self) -> bool {
        self.modal
    }

    pub fn loop_focus(&self) -> bool {
        self.loop_focus
    }

    pub fn orientation(&self) -> MenuOrientation {
        self.orientation
    }

    pub fn close_parent_on_esc(&self) -> bool {
        self.close_parent_on_esc
    }

    pub fn highlight_item_on_hover(&self) -> bool {
        self.highlight_item_on_hover
    }

    pub fn on_open_change(&self) -> Option<&MenuOpenChangeHandler<P>> {
        self.on_open_change.as_ref()
    }

    pub fn on_open_change_complete(&self) -> Option<&MenuOpenChangeCompleteHandler<P>> {
        self.on_open_change_complete.as_ref()
    }
}
