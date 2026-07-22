use std::rc::Rc;

use gpui::{
    div, AnyElement, App, ClickEvent, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::combobox::{
    child_wiring::ComboboxChildNode, ComboboxChangeReason, ComboboxChangeSource, ComboboxContext,
    ComboboxSide, ComboboxTriggerStyleState,
};

type ComboboxTriggerStyle<T> = Rc<dyn Fn(ComboboxTriggerStyleState<T>, Div) -> Div + 'static>;

/// Button that toggles the popup open/closed and focuses the input on press.
#[derive(IntoElement)]
pub struct ComboboxTrigger<T: Clone + Eq + 'static> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<ComboboxContext<T>>,
    disabled: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<ComboboxTriggerStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for ComboboxTrigger<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("combobox-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            disabled: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for ComboboxTrigger<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for ComboboxTrigger<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for ComboboxTrigger<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, props| {
            runtime.trigger_state(props, ComboboxSide::Bottom)
        });
        let disabled = self.disabled || state.root.disabled;
        let read_only = state.root.read_only;
        let open = state.root.open;
        let click_context = context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        // AccessKit: `.on_click` auto-registers `Action::Click`; Base UI's
        // `aria-haspopup`/`aria-controls` have no gpui builders (documented
        // gaps in `combobox/mod.rs`).
        let mut trigger = base.id(self.id).role(Role::Button).aria_expanded(open);
        if let Some(aria_label) = self.aria_label.clone() {
            trigger = trigger.aria_label(aria_label);
        }

        trigger
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) || disabled || read_only {
                    return;
                }
                click_context.toggle_open(
                    ComboboxChangeReason::TriggerPress,
                    ComboboxChangeSource::Pointer,
                    window,
                    cx,
                );
                click_context.focus_input(window, cx);
            })
            .children(self.children)
            .into_any_element()
    }
}

impl<T: Clone + Eq + 'static> ComboboxChildNode<T> for ComboboxTrigger<T> {
    fn with_combobox_context(mut self, context: ComboboxContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> ComboboxTrigger<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible label for the trigger button (e.g. "Open suggestions").
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ComboboxTriggerStyleState<T>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
