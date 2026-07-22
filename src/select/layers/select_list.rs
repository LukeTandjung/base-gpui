use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::select::{
    child_wiring::{SelectChildNode, SelectChildWiring},
    key::typeahead_text,
    SelectActivateHighlighted, SelectClose, SelectContext, SelectListChild, SelectListStyleState,
    SelectMove, SelectMoveFirst, SelectMoveLast, SelectMoveNext, SelectMovePrevious,
    SelectOpenChangeReason, SelectOpenChangeSource, SelectValueChangeSource, SELECT_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct SelectList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<SelectListChild<T>>,
    context: Option<SelectContext<T>>,
    loop_focus: bool,
    style_with_state: Option<Rc<dyn Fn(SelectListStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for SelectList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            loop_focus: true,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for SelectList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for SelectList<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.list_state()))
            .unwrap_or_else(|| SelectListStyleState::new(false, 0));
        let scroll_handle = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.list_scroll_handle()));
        let typeahead_context = self.context.clone();
        let next_context = self.context.clone();
        let previous_context = self.context.clone();
        let first_context = self.context.clone();
        let last_context = self.context.clone();
        let activate_context = self.context.clone();
        let close_context = self.context.clone();
        let scroll_context = self.context.clone();
        let loop_focus = self.loop_focus;
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let list = base
            .id("select-list")
            // AccessKit gap in this gpui revision: no `aria-multiselectable`
            // builder, so multiple-selection mode stays style-state only.
            .role(Role::ListBox)
            .key_context(SELECT_KEY_CONTEXT)
            .focusable()
            .on_key_down(move |event, window, cx| {
                let Some(text) = typeahead_text(event) else {
                    return;
                };
                if let Some(context) = typeahead_context.as_ref() {
                    if context.typeahead(&text, window, cx) {
                        cx.stop_propagation();
                    }
                }
            })
            .on_action(move |_: &SelectMoveNext, window, cx| {
                if let Some(context) = next_context.as_ref() {
                    context.move_highlight(SelectMove::Next, loop_focus, window, cx);
                }
            })
            .on_action(move |_: &SelectMovePrevious, window, cx| {
                if let Some(context) = previous_context.as_ref() {
                    context.move_highlight(SelectMove::Previous, loop_focus, window, cx);
                }
            })
            .on_action(move |_: &SelectMoveFirst, window, cx| {
                if let Some(context) = first_context.as_ref() {
                    context.move_highlight(SelectMove::First, loop_focus, window, cx);
                }
            })
            .on_action(move |_: &SelectMoveLast, window, cx| {
                if let Some(context) = last_context.as_ref() {
                    context.move_highlight(SelectMove::Last, loop_focus, window, cx);
                }
            })
            .on_action(move |_: &SelectActivateHighlighted, window, cx| {
                if let Some(context) = activate_context.as_ref() {
                    context.activate_highlighted(SelectValueChangeSource::Keyboard, window, cx);
                }
            })
            .on_action(move |_: &SelectClose, window, cx| {
                if let Some(context) = close_context.as_ref() {
                    context.set_open(
                        false,
                        SelectOpenChangeReason::EscapeKey,
                        SelectOpenChangeSource::Keyboard,
                        window,
                        cx,
                    );
                }
            })
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );
        let list = match scroll_handle.as_ref() {
            Some(scroll_handle) => list.overflow_y_scroll().track_scroll(scroll_handle),
            None => list,
        };

        div()
            .on_children_prepainted(move |_bounds, window, cx| {
                let Some(context) = scroll_context.as_ref() else {
                    return;
                };

                if context.refresh_scroll_arrow_visibility(cx) {
                    window.request_animation_frame();
                }
            })
            .child(list)
    }
}

impl<T: Clone + Eq + 'static> SelectChildNode<T> for SelectList<T> {
    fn with_select_context(mut self, context: SelectContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_select_context(context.clone()))
            .collect();
        self
    }

    fn wire_select_child(
        mut self,
        wiring: &mut SelectChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_list_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> SelectList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<SelectListChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(SelectListChild::Any(child.into_any_element()));
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(SelectListStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
