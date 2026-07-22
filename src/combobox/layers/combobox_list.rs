use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::{ComboboxChildNode, ComboboxChildWiring},
    ComboboxContext, ComboboxListChild, ComboboxListStyleState,
};

/// The scrollable option list. Scrolling uses a runtime-owned scroll handle;
/// pointer leaving the list clears the highlight unless `keep_highlight`.
#[derive(IntoElement)]
pub struct ComboboxList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<ComboboxListChild<T>>,
    context: Option<ComboboxContext<T>>,
    style_with_state: Option<Rc<dyn Fn(ComboboxListStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxList<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.list_state()))
            .unwrap_or_else(|| ComboboxListStyleState::new(false, 0, true));
        let scroll_handle = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.list_scroll_handle()));
        let leave_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // AccessKit: `aria-multiselectable` has no gpui builder (documented
        // gap in `combobox/mod.rs`); multiple-mode state is conveyed
        // per-option via `.aria_selected`.
        let list = base
            .id("combobox-list")
            .role(Role::ListBox)
            .on_mouse_move(|_event, _window, _cx| {})
            .on_hover(move |hovered, window, cx| {
                if !hovered {
                    if let Some(context) = leave_context.as_ref() {
                        context.pointer_left_list(window, cx);
                    }
                }
            })
            .children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );

        match scroll_handle.as_ref() {
            Some(scroll_handle) => list.overflow_y_scroll().track_scroll(scroll_handle),
            None => list,
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxList<T> {
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
        self.children = wiring.wire_list_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<ComboboxListChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(ComboboxListChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxListStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
