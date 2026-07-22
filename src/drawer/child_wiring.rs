use gpui::{App, Window};

use crate::dialog::child_wiring::{DialogChildNode, DialogChildWiring, WiredDialogChildren};
use crate::drawer::{
    DrawerChild, DrawerContext, DrawerPopupChild, DrawerPortalChild, DrawerViewportChild,
};

/// Drawer-local child wiring. The dialog wiring collects the trigger/title/
/// description/popup-focus registration exactly as a plain dialog would, so the
/// reused dialog machinery keeps working; drawer parts additionally receive the
/// `DrawerContext`.
pub struct WiredDrawerChildren<P: Clone + 'static> {
    pub dialog: WiredDialogChildren<P>,
    pub children: Vec<DrawerChild<P>>,
}

pub trait DrawerChildNode<P: Clone + 'static>: Sized {
    fn with_drawer_context(self, context: DrawerContext<P>) -> Self;

    fn wire_drawer_child(
        self,
        _wiring: &mut DialogChildWiring<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<DrawerChild<P>>,
    context: DrawerContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredDrawerChildren<P> {
    let mut wiring = DialogChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_drawer_child(&mut wiring, window, cx))
        .map(|child| child.with_drawer_context(context.clone()))
        .collect();
    let dialog = wiring.finish(Vec::new());

    WiredDrawerChildren { dialog, children }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerChild<P> {
    fn with_drawer_context(self, context: DrawerContext<P>) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(Box::new(
                trigger.with_dialog_context(context.dialog().clone()),
            )),
            Self::SwipeArea(swipe_area) => {
                Self::SwipeArea(Box::new(swipe_area.with_drawer_context(context)))
            }
            Self::Portal(portal) => Self::Portal(Box::new(portal.with_drawer_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_drawer_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => {
                Self::Trigger(Box::new(trigger.wire_dialog_child(wiring, window, cx)))
            }
            Self::SwipeArea(swipe_area) => {
                Self::SwipeArea(Box::new(swipe_area.wire_drawer_child(wiring, window, cx)))
            }
            Self::Portal(portal) => {
                Self::Portal(Box::new(portal.wire_drawer_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerPortalChild<P> {
    fn with_drawer_context(self, context: DrawerContext<P>) -> Self {
        match self {
            Self::Backdrop(backdrop) => {
                Self::Backdrop(Box::new(backdrop.with_drawer_context(context)))
            }
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.with_drawer_context(context)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_drawer_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Viewport(viewport) => {
                Self::Viewport(Box::new(viewport.wire_drawer_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerViewportChild<P> {
    fn with_drawer_context(self, context: DrawerContext<P>) -> Self {
        match self {
            Self::Popup(popup) => Self::Popup(Box::new(popup.with_drawer_context(context))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_drawer_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_drawer_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}

impl<P: Clone + 'static> DrawerChildNode<P> for DrawerPopupChild<P> {
    fn with_drawer_context(self, context: DrawerContext<P>) -> Self {
        let dialog = context.dialog().clone();
        match self {
            Self::Content(content) => Self::Content(Box::new(content.with_drawer_context(context))),
            Self::Title(title) => Self::Title(Box::new(title.with_dialog_context(dialog))),
            Self::Description(description) => {
                Self::Description(Box::new(description.with_dialog_context(dialog)))
            }
            Self::Close(close) => Self::Close(Box::new(close.with_dialog_context(dialog))),
            Self::Any(any) => Self::Any(any),
        }
    }

    fn wire_drawer_child(
        self,
        wiring: &mut DialogChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Title(title) => {
                Self::Title(Box::new(title.wire_dialog_child(wiring, window, cx)))
            }
            Self::Description(description) => {
                Self::Description(Box::new(description.wire_dialog_child(wiring, window, cx)))
            }
            Self::Close(close) => {
                Self::Close(Box::new(close.wire_dialog_child(wiring, window, cx)))
            }
            other => other,
        }
    }
}
