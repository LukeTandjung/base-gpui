use std::rc::Rc;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::slider::{SliderContext, SliderLabelStyleState};

#[derive(IntoElement)]
pub struct SliderLabel {
    id: Option<ElementId>,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<SliderContext>,
    style_with_state: Option<Rc<dyn Fn(SliderLabelStyleState, Div) -> Div + 'static>>,
}

impl Default for SliderLabel {
    fn default() -> Self {
        Self {
            id: None,
            base: div(),
            children: Vec::from([]),
            context: None,
            style_with_state: None,
        }
    }
}

impl Styled for SliderLabel {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for SliderLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .expect("SliderLabel must be rendered inside SliderRoot");
        let id = self.id.unwrap_or_else(|| context.child_id("label"));
        let style_state = context.read(cx, |runtime, props| runtime.label_state(props));
        let click_context = context.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.id(id)
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                // Only a single-thumb slider has an unambiguous focus target.
                let focus_handle =
                    click_context.read(cx, |runtime, _| runtime.single_thumb_focus_handle());
                if let Some(focus_handle) = focus_handle {
                    focus_handle.focus(window, cx);
                }
            })
            .children(self.children)
    }
}

impl SliderLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_slider_context(mut self, context: SliderContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Base UI links this label to the root via `aria-labelledby`; gpui has
    /// no id-reference builder, so the label text is instead supplied to
    /// `SliderRoot::aria_label(...)`. Once that is set, pass the visible
    /// label text here as `Text::new_inaccessible(...)` so it is not
    /// announced twice.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SliderLabelStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
