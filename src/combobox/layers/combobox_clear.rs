use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxChangeReason, ComboboxChangeSource,
    ComboboxClearStyleState, ComboboxContext,
};

/// Visible only when there is something to clear; pressing it clears input +
/// selection + highlight and refocuses the input without opening the popup.
#[derive(IntoElement)]
pub struct ComboboxClear<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    keep_mounted: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(ComboboxClearStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxClear<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox-clear"),
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxClear<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxClear<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxClear<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, props| runtime.clear_state(props));

        if !state.visible && !self.keep_mounted {
            return div().into_any_element();
        }

        let disabled = state.disabled;
        let click_context = context.clone();
        let has_custom_children = !self.children.is_empty();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // AccessKit: `.on_click` auto-registers `Action::Click`; the "×"
        // glyph is a plain string child with no a11y id, so it is not
        // announced alongside the label.
        let clear = base
            .id(self.id)
            .role(Role::Button)
            .aria_label(
                self.aria_label
                    .unwrap_or_else(|| SharedString::from("Clear")),
            )
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) || disabled {
                    return;
                }
                click_context.clear_all(
                    ComboboxChangeReason::ClearPress,
                    ComboboxChangeSource::Pointer,
                    window,
                    cx,
                );
            });

        if has_custom_children {
            clear.children(self.children).into_any_element()
        } else {
            clear.child("×").into_any_element()
        }
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxClear<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxClear<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    /// Accessible label for the clear button; defaults to "Clear".
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxClearStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
