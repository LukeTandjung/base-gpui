use std::{rc::Rc, sync::Arc};

use gpui::{
    div, AnyElement, App, ClickEvent, Div, ElementId, Entity, FocusHandle, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    PopoverCloseAction, PopoverCloseStyleState, PopoverContext, PopoverOpenChangeReason,
    PopoverOpenChangeSource, PopoverToggleAction, POPOVER_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct PopoverClose<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PopoverContext<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    disabled: bool,
    aria_label: SharedString,
    style_with_state: Option<Rc<dyn Fn(PopoverCloseStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverClose<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover-close"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            scoped: false,
            disabled: false,
            aria_label: SharedString::from("Close"),
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PopoverClose<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PopoverClose<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverClose<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| close_focus_handle(&self.id, window, cx));
        let state = self
            .context
            .as_ref()
            .map(|context| context.read(cx, |runtime, _| runtime.close_state(self.disabled)))
            .unwrap_or_else(|| PopoverCloseStyleState::new(self.disabled, false));
        let disabled = state.disabled;
        let click_context = self.context.clone();
        let toggle_context = self.context.clone();
        let escape_context = self.context.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        base.id(self.id)
            // AccessKit gap in this gpui revision: no `aria-disabled`
            // builder, so AT cannot perceive the disabled state; handlers
            // early-return and tab_index(-1) applies while disabled.
            .role(Role::Button)
            .aria_label(self.aria_label)
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .focusable()
            .key_context(POPOVER_KEY_CONTEXT)
            .on_click(move |event, window, cx| {
                if disabled || !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }
                if let Some(context) = click_context.as_ref() {
                    context.close(
                        PopoverOpenChangeReason::ClosePress,
                        PopoverOpenChangeSource::Pointer,
                        window,
                        cx,
                    );
                }
            })
            .on_action(move |_: &PopoverToggleAction, window, cx| {
                if disabled {
                    return;
                }
                if let Some(context) = toggle_context.as_ref() {
                    context.close(
                        PopoverOpenChangeReason::ClosePress,
                        PopoverOpenChangeSource::Keyboard,
                        window,
                        cx,
                    );
                }
            })
            .on_action(move |_: &PopoverCloseAction, window, cx| {
                if let Some(context) = escape_context.as_ref() {
                    context.close(
                        PopoverOpenChangeReason::EscapeKey,
                        PopoverOpenChangeSource::Keyboard,
                        window,
                        cx,
                    );
                }
            })
            .children(self.children)
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverClose<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        if !self.scoped {
            self.id = ElementId::from((context.root_id(), SharedString::from(self.id.to_string())));
        }
        self.context = Some(context);
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.id = wiring.scope_child_id(&self.id);
        let focus_handle = close_focus_handle(&self.id, window, cx);
        wiring.register_popup_focus_handle(focus_handle.clone());
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self
    }
}

impl<P: Clone + 'static> PopoverClose<P> {
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

    /// Accessible label for the close button; defaults to "Close". Any
    /// visible text that duplicates this label should be rendered with
    /// `Text::new_inaccessible(...)` to avoid double-announcing.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = label.into();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverCloseStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn close_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
