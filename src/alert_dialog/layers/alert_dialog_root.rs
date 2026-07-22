use gpui::{App, Div, ElementId, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::alert_dialog::AlertDialogHandle;
use crate::dialog::{DialogChild, DialogOpenChangeDetails, DialogRoot, DialogRootStyleState};

/// Base UI Alert Dialog root: a Dialog root with the alert invariants forced —
/// always modal and never pointer-dismissable. The `modal`, `modal_mode`,
/// `trap_focus`, and `disable_pointer_dismissal` setters are deliberately not
/// exposed, so callers cannot violate the invariants.
#[derive(IntoElement)]
pub struct AlertDialogRoot<P: Clone + 'static = ()> {
    inner: DialogRoot<P>,
}

impl<P: Clone + 'static> Default for AlertDialogRoot<P> {
    fn default() -> Self {
        Self {
            inner: DialogRoot::new().id("alert-dialog"),
        }
    }
}

impl<P: Clone + 'static> Styled for AlertDialogRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl<P: Clone + 'static> RenderOnce for AlertDialogRoot<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Force the alert invariants regardless of anything else: an Alert
        // Dialog is always modal and never dismissed by pointer interaction.
        self.inner.modal(true).disable_pointer_dismissal(true)
    }
}

impl<P: Clone + 'static> AlertDialogRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<DialogChild<P>>) -> Self {
        self.inner = self.inner.child(child);
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<DialogChild<P>>>,
    ) -> Self {
        self.inner = self.inner.children(children);
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.inner = self.inner.child_any(child);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.inner = self.inner.id(id);
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.inner = self.inner.default_open(default_open);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.inner = self.inner.open(open);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut DialogOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.inner = self.inner.on_open_change(on_open_change);
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &DialogOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.inner = self.inner.on_open_change_complete(on_open_change_complete);
        self
    }

    pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.inner = self.inner.trigger_id(trigger_id);
        self
    }

    pub fn no_trigger_id(mut self) -> Self {
        self.inner = self.inner.no_trigger_id();
        self
    }

    pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.inner = self.inner.default_trigger_id(trigger_id);
        self
    }

    pub fn handle(mut self, handle: AlertDialogHandle<P>) -> Self {
        self.inner = self.inner.handle(handle.dialog_handle());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(DialogRootStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.inner = self.inner.style_with_state(style);
        self
    }
}
