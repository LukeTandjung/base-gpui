use std::rc::Rc;

use gpui::{
    div, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxAlign, ComboboxContext, ComboboxPopupChild, ComboboxPopupStyleState, ComboboxSide,
};

#[derive(IntoElement)]
pub struct ComboboxPopup<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxPopupChild<T>>,
    context: Option<ComboboxContext<T>>,
    side: ComboboxSide,
    align: ComboboxAlign,
    force_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(ComboboxPopupStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxPopup<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: ComboboxSide::Bottom,
            align: ComboboxAlign::Start,
            force_mounted: false,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxPopup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxPopup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.popup_state(self.side, self.align, self.force_mounted)
                })
            })
            .unwrap_or_else(|| {
                ComboboxPopupStyleState::new(false, self.force_mounted, self.side, self.align, true)
            });

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

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxPopup<T> {
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
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxPopup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxPopupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: ComboboxSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: ComboboxAlign) -> Self {
        self.align = align;
        self
    }

    pub fn force_mounted(mut self, force_mounted: bool) -> Self {
        self.force_mounted = force_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
