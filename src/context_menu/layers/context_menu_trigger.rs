use std::rc::Rc;
use std::time::Instant;

use gpui::{
    div, AnyElement, App, Div, ElementId, InteractiveElement as _, IntoElement, MouseButton,
    ParentElement, StyleRefinement, Styled,
};

use crate::context_menu::ContextMenuTriggerStyleState;
use crate::menu::{MenuChild, MenuContext, MenuOpenChangeReason, MenuOpenChangeSource};

type ContextMenuTriggerStyle = Rc<dyn Fn(ContextMenuTriggerStyleState, Div) -> Div + 'static>;

/// Base UI Context Menu trigger: a plain, non-focusable area `div` that opens
/// the menu at the cursor on right mouse-down. It is not a button trigger —
/// left-click and hover do nothing, and disabling is the root's
/// `.disabled(bool)`. Touch long-press open is deferred until GPUI exposes
/// touch pointer metadata; the open command is gesture-source-agnostic so a
/// long-press source can be added later.
///
/// # Accessibility
///
/// The trigger area is intentionally **inaccessible**, mirroring Base UI's
/// role-less `<div>`: it carries no `Role`, no `aria_*` props, no focus
/// tracking, and no `on_a11y_action` handlers, so despite having an element
/// id it never appears in the AccessKit tree or the tab order. The menu
/// popup itself carries the tree semantics (`Role::Menu` etc.) once open,
/// via the re-exported Menu parts. Relationship props (`aria-haspopup`,
/// `aria-controls`) are both un-emitted by Base UI for this part and
/// unavailable as builders in the pinned gpui revision, so nothing is lost.
/// The `ContextMenuTriggerStyleState` `open`/`pressed` facts are visual only.
pub struct ContextMenuTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    style_with_state: Option<ContextMenuTriggerStyle>,
    marker: std::marker::PhantomData<P>,
}

impl<P: Clone + 'static> Default for ContextMenuTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("context-menu-trigger"),
            base: div(),
            children: Vec::new(),
            style_with_state: None,
            marker: std::marker::PhantomData,
        }
    }
}

impl<P: Clone + 'static> ParentElement for ContextMenuTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for ContextMenuTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> From<ContextMenuTrigger<P>> for MenuChild<P> {
    fn from(trigger: ContextMenuTrigger<P>) -> Self {
        MenuChild::ContextArea(Box::new(move |context, _window, cx| {
            trigger.build(context, cx)
        }))
    }
}

impl<P: Clone + 'static> ContextMenuTrigger<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ContextMenuTriggerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Builds the wired trigger area. The `pressed`/`open` style facts come
    /// from a runtime query, not local trigger state.
    fn build(self, context: &MenuContext<P>, cx: &mut App) -> AnyElement {
        let state = context.read(cx, |runtime, _| {
            let open = runtime.open_value();
            ContextMenuTriggerStyleState::new(
                open,
                open && runtime.last_open_reason() == MenuOpenChangeReason::TriggerPress,
            )
        });
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let open_context = context.clone();
        base.id(self.id)
            .on_mouse_down(MouseButton::Right, move |event, window, cx| {
                if open_context.read(cx, |_, props| props.disabled()) {
                    return;
                }
                // Record the cursor point on every open gesture: re-anchors
                // while open, arms the mouseup grace window, and stores the
                // initial cursor point for item-activation suppression.
                open_context.update(cx, |runtime| {
                    runtime.open_context_menu_at(event.position, Instant::now());
                });
                open_context.open(
                    MenuOpenChangeReason::TriggerPress,
                    MenuOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .children(self.children)
            .into_any_element()
    }
}
