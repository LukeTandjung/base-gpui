use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window,
};

use crate::select::{
    child_wiring::SelectChildNode, SelectContext, SelectScrollArrowDirection,
    SelectScrollArrowStyleState, SelectSide,
};

#[derive(IntoElement)]
pub struct SelectScrollDownArrow<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SelectContext<T>>,
    keep_mounted: bool,
    side: SelectSide,
    style_with_state: Option<Rc<dyn Fn(SelectScrollArrowStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectScrollDownArrow<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            side: SelectSide::Bottom,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for SelectScrollDownArrow<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectScrollDownArrow<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectScrollDownArrow<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.scroll_arrow_state(
                        SelectScrollArrowDirection::Down,
                        self.side,
                        self.keep_mounted,
                    )
                })
            })
            .unwrap_or_else(|| {
                SelectScrollArrowStyleState::new(
                    SelectScrollArrowDirection::Down,
                    false,
                    self.side,
                    self.keep_mounted,
                )
            });
        if !state.present {
            return div();
        }
        let scroll_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.on_mouse_move(move |_event, window, cx| {
            if let Some(context) = scroll_context.as_ref() {
                context.scroll_toward(SelectScrollArrowDirection::Down, window, cx);
            }
        })
        .children(self.children)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectScrollDownArrow<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectScrollDownArrow<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn side(mut self, side: SelectSide) -> Self {
        self.side = side;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectScrollArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
