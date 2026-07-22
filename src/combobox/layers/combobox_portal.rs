use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxContext, ComboboxPortalChild, ComboboxPortalStyleState,
};

/// Renders overlay content only while open (or force-mounted for
/// measurement). Combobox-local equivalent of `select_portal.rs`.
#[derive(IntoElement)]
pub struct ComboboxPortal<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxPortalChild<T>>,
    context: Option<ComboboxContext<T>>,
    force_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(ComboboxPortalStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxPortal<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            force_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxPortal<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxPortal<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.portal_state(self.force_mounted)))
            .unwrap_or_else(|| ComboboxPortalStyleState::new(false, self.force_mounted));

        if !state.mounted {
            return div();
        }

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPortal<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_combobox_context(context.clone()))
            .collect();
        self
    }

    fn wire_combobox_child(
        mut self,
        wiring: &mut ComboboxChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_portal_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxPortal<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxPortalChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxPortalChild::Any(child.into_any_element()));
        self
    }

    pub fn force_mounted(mut self, force_mounted: bool) -> Self {
        self.force_mounted = force_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxPortalStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
