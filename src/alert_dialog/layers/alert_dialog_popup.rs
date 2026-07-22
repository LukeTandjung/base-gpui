use gpui::{
    AnyElement, App, Div, ElementId, IntoElement, RenderOnce, Role, SharedString, StyleRefinement,
    Styled, Window,
};

use crate::dialog::{
    DialogChild, DialogPopup, DialogPopupChild, DialogPopupStyleState, DialogPortalChild,
    DialogViewportChild,
};

/// Base UI Alert Dialog popup: the Dialog popup with the accessibility role
/// pinned to [`Role::AlertDialog`] (Base UI's `role="alertdialog"` fork). The
/// `role` setter is deliberately not exposed, so callers cannot revert the
/// popup to a plain dialog role.
///
/// gpui has no `aria-labelledby` id-reference builder, so pass the title text
/// via [`AlertDialogPopup::aria_label`] to give the alert dialog its
/// accessible name.
#[derive(IntoElement)]
pub struct AlertDialogPopup<P: Clone + 'static = ()> {
    inner: DialogPopup<P>,
}

impl<P: Clone + 'static> Default for AlertDialogPopup<P> {
    fn default() -> Self {
        Self {
            inner: DialogPopup::new().id("alert-dialog-popup"),
        }
    }
}

impl<P: Clone + 'static> Styled for AlertDialogPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl<P: Clone + 'static> RenderOnce for AlertDialogPopup<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.into_dialog_popup()
    }
}

impl<P: Clone + 'static> From<AlertDialogPopup<P>> for DialogChild<P> {
    fn from(value: AlertDialogPopup<P>) -> Self {
        Self::from(value.into_dialog_popup())
    }
}

impl<P: Clone + 'static> From<AlertDialogPopup<P>> for DialogPortalChild<P> {
    fn from(value: AlertDialogPopup<P>) -> Self {
        Self::from(value.into_dialog_popup())
    }
}

impl<P: Clone + 'static> From<AlertDialogPopup<P>> for DialogViewportChild<P> {
    fn from(value: AlertDialogPopup<P>) -> Self {
        Self::from(value.into_dialog_popup())
    }
}

impl<P: Clone + 'static> AlertDialogPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.inner = self.inner.id(id);
        self
    }

    pub fn child(mut self, child: impl Into<DialogPopupChild<P>>) -> Self {
        self.inner = self.inner.child(child);
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.inner = self.inner.child_any(child);
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.inner = self.inner.keep_mounted(keep_mounted);
        self
    }

    /// Accessible name for the alert dialog. gpui has no `aria-labelledby`
    /// id-reference builder, so consumers pass the title string directly.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.inner = self.inner.aria_label(label);
        self
    }

    pub fn payload_content(
        mut self,
        content: impl Fn(Option<&P>, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.inner = self.inner.payload_content(content);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogPopupStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.inner = self.inner.style_with_state(style);
        self
    }

    fn into_dialog_popup(self) -> DialogPopup<P> {
        self.inner.role(Role::AlertDialog)
    }
}
