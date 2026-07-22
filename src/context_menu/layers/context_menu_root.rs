use gpui::{App, Div, ElementId, IntoElement, RenderOnce, StyleRefinement, Styled, Window};

use crate::menu::{
    MenuChild, MenuOpenChangeDetails, MenuOrientation, MenuRoot, MenuRootStyleState,
};

/// Base UI Context Menu root: a Menu root typed with
/// `MenuParentKind::ContextMenu`. It renders no element of its own; children
/// pass through to the wrapped menu tree. Context menus are unconditionally
/// modal and never hover-opened, so the `modal`, `open_on_hover`, and delay
/// setters are deliberately not exposed; nor are detached-trigger handles
/// (`handle`, `trigger_id`, `default_trigger_id`), matching Base UI.
#[derive(IntoElement)]
pub struct ContextMenuRoot<P: Clone + 'static = ()> {
    inner: MenuRoot<P>,
}

impl<P: Clone + 'static> Default for ContextMenuRoot<P> {
    fn default() -> Self {
        Self {
            inner: MenuRoot::new().id("context-menu").context_menu_parent(),
        }
    }
}

impl<P: Clone + 'static> Styled for ContextMenuRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.inner.style()
    }
}

impl<P: Clone + 'static> RenderOnce for ContextMenuRoot<P> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.inner
    }
}

impl<P: Clone + 'static> ContextMenuRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<MenuChild<P>>) -> Self {
        self.inner = self.inner.child(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<MenuChild<P>>>) -> Self {
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner = self.inner.disabled(disabled);
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.inner = self.inner.loop_focus(loop_focus);
        self
    }

    pub fn orientation(mut self, orientation: MenuOrientation) -> Self {
        self.inner = self.inner.orientation(orientation);
        self
    }

    pub fn close_parent_on_esc(mut self, close_parent_on_esc: bool) -> Self {
        self.inner = self.inner.close_parent_on_esc(close_parent_on_esc);
        self
    }

    pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self {
        self.inner = self.inner.highlight_item_on_hover(highlight_item_on_hover);
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.inner = self.inner.on_open_change(on_open_change);
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.inner = self.inner.on_open_change_complete(on_open_change_complete);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.inner = self.inner.style_with_state(style);
        self
    }
}
