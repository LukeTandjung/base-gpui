use std::rc::Rc;

use gpui::{
    div, px, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::preview_card::{
    child_wiring::PreviewCardChildNode, PreviewCardAlign, PreviewCardArrowStyleState,
    PreviewCardBoundsKind, PreviewCardContext, PreviewCardSide,
};

/// Base UI marks the arrow `aria-hidden="true"`. This gpui revision has no
/// `.aria_hidden(...)` builder, so the equivalent is achieved by assigning no
/// role: an element without a role never enters the AccessKit tree.
#[derive(IntoElement)]
pub struct PreviewCardArrow<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PreviewCardContext<P>>,
    side: PreviewCardSide,
    align: PreviewCardAlign,
    style_with_state: Option<Rc<dyn Fn(PreviewCardArrowStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PreviewCardArrow<P> {
    fn default() -> Self {
        Self {
            base: div().w(px(8.0)).h(px(8.0)),
            children: Vec::new(),
            context: None,
            side: PreviewCardSide::Bottom,
            align: PreviewCardAlign::Center,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PreviewCardArrow<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PreviewCardArrow<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardArrow<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| runtime.arrow_state(self.side, self.align))
            })
            .unwrap_or_else(|| {
                PreviewCardArrowStyleState::new(
                    false,
                    self.side,
                    self.align,
                    None,
                    None,
                    px(5.0),
                    false,
                )
            });
        let measure_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };
        let base = position_arrow(base, state).children(self.children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime.set_bounds(PreviewCardBoundsKind::Arrow, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

fn position_arrow(base: Div, state: PreviewCardArrowStyleState) -> Div {
    match (state.offset_x, state.offset_y) {
        (Some(offset_x), Some(offset_y)) => base.absolute().left(offset_x).top(offset_y),
        _ => base,
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardArrow<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<P: Clone + 'static> PreviewCardArrow<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: PreviewCardSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PreviewCardAlign) -> Self {
        self.align = align;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PreviewCardArrowStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
