use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::switch::{child_wiring::SwitchChildNode, SwitchContext, SwitchThumbStyleState};

#[derive(IntoElement)]
pub struct SwitchThumb {
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SwitchContext>,
    style_with_state: Option<Rc<dyn Fn(SwitchThumbStyleState, Div) -> Div + 'static>>,
}

impl Default for SwitchThumb {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for SwitchThumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SwitchThumb {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SwitchThumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.thumb_state(props)))
            .unwrap_or_default();

        let base = match self.style_with_state {
            Some(style) => style(state, self.base),
            None => self.base,
        };

        base.children(self.children)
    }
}

impl SwitchChildNode for SwitchThumb {
    fn with_switch_context(mut self, context: SwitchContext) -> Self {
        self.context = Some(context);
        self
    }
}

impl SwitchThumb {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SwitchThumbStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
