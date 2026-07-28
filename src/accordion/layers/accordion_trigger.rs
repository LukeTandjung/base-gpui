use std::{rc::Rc, sync::Arc};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, ClickEvent, Div,
    ElementId, Entity, FocusHandle, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Window,
};

use crate::accordion::{
    child_wiring::{AccordionHeaderChildNode, AccordionItemChildWiring},
    AccordionChangeSource, AccordionItemContext, AccordionItemStyleState, AccordionOrientation,
    AccordionToggle, AccordionTriggerStyleState, ACCORDION_TRIGGER_KEY_CONTEXT,
};

type AccordionTriggerStyle<T> = Rc<dyn Fn(AccordionTriggerStyleState<T>, Div) -> Div + 'static>;

#[derive(derive_setters::Setters, IntoElement)]
pub struct AccordionTrigger<T: Clone + Eq + 'static> {
    #[setters(into)]
    id: ElementId,
    #[setters(skip)]
    base: Div,
    #[setters(skip)]
    children: Vec<AnyElement>,
    #[setters(skip)]
    context: Option<AccordionItemContext<T>>,
    #[setters(skip)]
    focus_handle: Option<FocusHandle>,
    /// Accessible name for triggers whose visible content is iconic/non-textual.
    /// When set, render the visible label with `Text::new_inaccessible(...)` so
    /// screen readers do not announce the name twice.
    #[setters(into, strip_option)]
    aria_label: Option<SharedString>,
    #[setters(skip)]
    style_with_state: Option<AccordionTriggerStyle<T>>,
}

impl<T: Clone + Eq + 'static> Default for AccordionTrigger<T> {
    fn default() -> Self {
        Self {
            id: ElementId::from("accordion-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> ParentElement for AccordionTrigger<T> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<T: Clone + Eq + 'static> Styled for AccordionTrigger<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for AccordionTrigger<T> {
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
        let (state, item_count) = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, props, value, index, disabled| {
                    (
                        runtime.trigger_state(value, index, disabled, props),
                        runtime.item_count(),
                    )
                })
            })
            .unwrap_or_else(|| {
                (
                    AccordionTriggerStyleState::new(
                        AccordionItemStyleState::new(
                            panic_value(),
                            Vec::new(),
                            false,
                            false,
                            0,
                            AccordionOrientation::Vertical,
                        ),
                        false,
                    ),
                    0,
                )
            });
        let disabled = state.item.disabled;
        let expanded = state.panel_open;
        let position_in_set = state.item.index + 1;

        let base = match style_with_state {
            Some(style_with_state) => style_with_state(state, base),
            None => base,
        };

        let keyboard_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let pointer_context = context;

        base.id(id)
            .role(Role::Button)
            .aria_expanded(expanded)
            .aria_position_in_set(position_in_set)
            .when(item_count > 0, |this| this.aria_size_of_set(item_count))
            .when_some(aria_label, |this, label| this.aria_label(label))
            .track_focus(
                &focus_handle
                    .tab_stop(!disabled)
                    .tab_index(if disabled { -1 } else { 0 }),
            )
            .key_context(ACCORDION_TRIGGER_KEY_CONTEXT)
            .focusable()
            .on_action(move |_: &AccordionToggle, window, cx| {
                if let Some(context) = keyboard_context.as_ref() {
                    context.toggle(AccordionChangeSource::Keyboard, window, cx);
                }
            })
            .on_a11y_action(AccessibleAction::Expand, move |_, window, cx| {
                if expanded {
                    return;
                }

                if let Some(context) = expand_context.as_ref() {
                    context.toggle(AccordionChangeSource::Keyboard, window, cx);
                }
            })
            .on_a11y_action(AccessibleAction::Collapse, move |_, window, cx| {
                if !expanded {
                    return;
                }

                if let Some(context) = collapse_context.as_ref() {
                    context.toggle(AccordionChangeSource::Keyboard, window, cx);
                }
            })
            .on_click(move |event, window, cx| {
                if !matches!(event, ClickEvent::Mouse(_)) {
                    return;
                }

                if let Some(context) = pointer_context.as_ref() {
                    context.toggle(AccordionChangeSource::Pointer, window, cx);
                }
            })
            .children(children)
    }
}

impl<T: Clone + Eq + 'static> AccordionHeaderChildNode<T> for AccordionTrigger<T> {
    fn with_accordion_item_context(mut self, context: AccordionItemContext<T>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_accordion_header_child(
        mut self,
        wiring: &mut AccordionItemChildWiring,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let focus_handle = trigger_focus_handle(&self.id, window, cx);
        wiring.register_trigger(focus_handle.clone(), window);
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl<T: Clone + Eq + 'static> AccordionTrigger<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(AccordionTriggerStyleState<T>, Div) -> Div + 'static,
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

fn panic_value<T: Clone + Eq + 'static>() -> T {
    panic!("AccordionTrigger must be rendered inside AccordionItem")
}
