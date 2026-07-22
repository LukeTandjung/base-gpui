use std::rc::Rc;

use gpui::AnyElement;

use crate::menu::{MenuMenubarLink, MenuMove, MenuRoot};
use crate::menubar::{MenubarChild, MenubarContext, MenubarMove, MenubarTriggerSlot};

/// Type-erased hosted menu so `MenuRoot<P>` menus with different payload
/// types can share one menubar row without leaking `P`.
pub trait MenubarMenuErased {
    fn attach_menubar(self: Box<Self>, link: MenuMenubarLink) -> Box<dyn MenubarMenuErased>;

    fn menu_disabled(&self) -> bool;

    fn into_any(self: Box<Self>) -> AnyElement;
}

impl<P: Clone + 'static> MenubarMenuErased for MenuRoot<P> {
    fn attach_menubar(self: Box<Self>, link: MenuMenubarLink) -> Box<dyn MenubarMenuErased> {
        Box::new((*self).menubar_link(link))
    }

    fn menu_disabled(&self) -> bool {
        self.is_disabled()
    }

    fn into_any(self: Box<Self>) -> AnyElement {
        use gpui::IntoElement as _;
        (*self).into_any_element()
    }
}

pub struct WiredMenubarChildren {
    pub triggers: Vec<MenubarTriggerSlot>,
    pub children: Vec<MenubarChild>,
}

/// The only module that walks menubar children: assigns trigger indices to
/// hosted menus in source order and attaches the typed menubar link that
/// makes each one a menubar-parented menu.
pub fn wire_children(
    children: Vec<MenubarChild>,
    context: &MenubarContext,
) -> WiredMenubarChildren {
    let mut triggers = Vec::new();
    let mut wired = Vec::new();

    for child in children {
        wired.push(match child {
            MenubarChild::Menu(menu) => {
                let index = triggers.len();
                let disabled = context.props_disabled() || menu.menu_disabled();
                triggers.push(MenubarTriggerSlot::new(disabled));
                MenubarChild::Menu(menu.attach_menubar(menubar_link(context, index)))
            }
            MenubarChild::Any(any) => MenubarChild::Any(any),
        });
    }

    WiredMenubarChildren {
        triggers,
        children: wired,
    }
}

fn menubar_move(direction: MenuMove) -> MenubarMove {
    match direction {
        MenuMove::Next => MenubarMove::Next,
        MenuMove::Previous => MenubarMove::Previous,
        MenuMove::First => MenubarMove::First,
        MenuMove::Last => MenubarMove::Last,
    }
}

/// Builds the typed link one hosted menu uses to talk to the menubar: the
/// exact Base UI `MenubarContext` facts plus the keyboard-relay command.
fn menubar_link(context: &MenubarContext, index: usize) -> MenuMenubarLink {
    let (disabled, modal, horizontal) = (
        context.props_disabled(),
        context.props_modal(),
        context.props_horizontal(),
    );

    let has_submenu_open = {
        let context = context.clone();
        Rc::new(move |cx: &gpui::App| context.read(cx, |runtime, _| runtime.has_submenu_open()))
    };
    let is_tab_stop = {
        let context = context.clone();
        Rc::new(move |trigger_index: usize, cx: &gpui::App| {
            context.read(cx, |runtime, _| runtime.is_tab_stop(trigger_index))
        })
    };
    let register_trigger = {
        let context = context.clone();
        Rc::new(
            move |trigger_index,
                  trigger_disabled,
                  focus_handle,
                  open,
                  close,
                  cx: &mut gpui::App| {
                context.update(cx, |runtime| {
                    runtime.register_trigger(
                        trigger_index,
                        trigger_disabled,
                        focus_handle,
                        open,
                        close,
                    );
                });
            },
        )
    };
    let note_open_change = {
        let context = context.clone();
        Rc::new(move |trigger_index, open, reason, cx: &mut gpui::App| {
            context.update(cx, |runtime| {
                runtime.note_child_open_change(trigger_index, open, reason);
            });
        })
    };
    let close_sibling = {
        let context = context.clone();
        Rc::new(
            move |except, reason, source, window: &mut gpui::Window, cx: &mut gpui::App| {
                context.close_open_sibling(except, reason, source, window, cx);
            },
        )
    };
    let relay = {
        let context = context.clone();
        Rc::new(
            move |from, direction, window: &mut gpui::Window, cx: &mut gpui::App| {
                context.relay(from, menubar_move(direction), window, cx);
            },
        )
    };
    let highlight = {
        let context = context.clone();
        Rc::new(move |trigger_index, cx: &mut gpui::App| {
            context.update(cx, |runtime| runtime.set_highlight(trigger_index));
        })
    };
    let menubar_bounds = {
        let context = context.clone();
        Rc::new(move |cx: &gpui::App| context.read(cx, |runtime, _| runtime.bounds()))
    };

    MenuMenubarLink::new(
        index,
        disabled,
        modal,
        horizontal,
        has_submenu_open,
        is_tab_stop,
        register_trigger,
        note_open_change,
        close_sibling,
        relay,
        highlight,
        menubar_bounds,
    )
}
