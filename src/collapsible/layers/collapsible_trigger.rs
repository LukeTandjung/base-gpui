use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, ClickEvent, Div,
    ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

use crate::collapsible::{
    child_wiring::{CollapsibleChildNode, CollapsibleChildWiring},
    CollapsibleContext, CollapsibleOpenChangeSource, CollapsibleToggle,
    CollapsibleTriggerStyleState, COLLAPSIBLE_TRIGGER_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct CollapsibleTrigger {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<CollapsibleContext>,
    focus_handle: Option<FocusHandle>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(CollapsibleTriggerStyleState, Div) -> Div + 'static>>,
}

impl Default for CollapsibleTrigger {
    fn default() -> Self {
        Self {
            id: ElementId::from("collapsible-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl ParentElement for CollapsibleTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for CollapsibleTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for CollapsibleTrigger {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            base,
            children,
            context,
            focus_handle,
            aria_label,
            style_with_state,
        } = self;

        let focus_handle = focus_handle.unwrap_or_else(|| trigger_focus_handle(&id, window, cx));
        let state = context
            .as_ref()
            .map(|context| context.read(cx, |runtime, props| runtime.trigger_state(props)))
            .unwrap_or_default();
        let disabled = state.disabled;

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        let keyboard_context = context.clone();
        let pointer_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context;
        let open = state.open;

        base.id(id)
            .role(Role::Button)
            .aria_expanded(open)
            .when_some(aria_label, |this, aria_label| this.aria_label(aria_label))
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .key_context(COLLAPSIBLE_TRIGGER_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &CollapsibleToggle, window, cx| {
                if let Some(context) = keyboard_context.as_ref() {
                    context.toggle(CollapsibleOpenChangeSource::Keyboard, window, cx);
                }
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                if let Some(context) = pointer_context.as_ref() {
                    context.toggle(CollapsibleOpenChangeSource::Pointer, window, cx);
                }
            })
            .on_a11y_action(AccessibleAction::Expand, move |_, window, cx| {
                if let Some(context) = expand_context.as_ref() {
                    let open = context.read(cx, |runtime, _| runtime.open());
                    if !open {
                        context.toggle(CollapsibleOpenChangeSource::Keyboard, window, cx);
                    }
                }
            })
            .on_a11y_action(AccessibleAction::Collapse, move |_, window, cx| {
                if let Some(context) = collapse_context.as_ref() {
                    let open = context.read(cx, |runtime, _| runtime.open());
                    if open {
                        context.toggle(CollapsibleOpenChangeSource::Keyboard, window, cx);
                    }
                }
            })
            .children(children)
    }
}

impl CollapsibleChildNode for CollapsibleTrigger {
    fn with_collapsible_context(mut self, context: CollapsibleContext) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_collapsible_child(
        mut self,
        wiring: &mut CollapsibleChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let focus_handle = trigger_focus_handle(&self.id, window, cx);
        wiring.register_trigger(&focus_handle, window);
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl CollapsibleTrigger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the accessible label announced for this trigger.
    ///
    /// Required for icon-only triggers. When the trigger also has a visible text
    /// label child, pass that child as `Text::new_inaccessible(...)` so screen
    /// readers do not announce the label twice; without an `aria_label`, leave
    /// child text accessible (`text!(...)`) so it names the button.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(CollapsibleTriggerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
