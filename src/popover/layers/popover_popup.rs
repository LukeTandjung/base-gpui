use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    layers::popover_trigger::{evaluate_safe_polygon_move, spawn_delayed_hover},
    PopoverAlign, PopoverBoundsKind, PopoverCloseAction, PopoverContext, PopoverHoverTarget,
    PopoverOpenChangeReason, PopoverOpenChangeSource, PopoverPayloadContentBuilder,
    PopoverPopupChild, PopoverPopupStyleState, PopoverSide, POPOVER_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct PopoverPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<PopoverPopupChild<P>>,
    context: Option<PopoverContext<P>>,
    side: PopoverSide,
    align: PopoverAlign,
    keep_mounted: bool,
    payload_content: Option<PopoverPayloadContentBuilder<P>>,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(PopoverPopupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PopoverPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover-popup"),
            base: div().relative(),
            children: Vec::new(),
            context: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Center,
            keep_mounted: false,
            payload_content: None,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PopoverPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverPopup<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.popup_state(self.side, self.align, self.keep_mounted)
                })
            })
            .unwrap_or_else(|| {
                PopoverPopupStyleState::new(false, self.keep_mounted, self.side, self.align, false)
            });
        if !state.mounted {
            return div();
        }

        let context = self.context.clone();
        let close_context = context.clone();
        let move_context = context.clone();
        let hover_context = context.clone();
        let measure_context = context.clone();
        let mut children = Vec::new();
        if let Some(payload_content) = self.payload_content {
            let payload = context
                .as_ref()
                .and_then(|context| context.read(cx, |runtime, _| runtime.active_payload()));
            children.push(payload_content(payload.as_ref(), window, cx));
        }
        children.extend(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(self.id)
        // AccessKit gaps in this gpui revision: no `aria-labelledby` /
        // `aria-describedby` relationship builders (title/description ids are
        // kept in PopoverRuntime for future wiring) and no `aria-modal`
        // builder for modal roots, so a literal aria_label is the fallback.
        .role(Role::Dialog)
        .when_some(self.aria_label, |this, label| this.aria_label(label))
        .key_context(POPOVER_KEY_CONTEXT)
        .on_action(move |_: &PopoverCloseAction, window, cx| {
            if let Some(context) = close_context.as_ref() {
                context.close(
                    PopoverOpenChangeReason::EscapeKey,
                    PopoverOpenChangeSource::Keyboard,
                    window,
                    cx,
                );
            }
        })
        .on_mouse_move(move |event, window, cx| {
            if let Some(context) = move_context.as_ref() {
                evaluate_safe_polygon_move(context, event.position, window, cx);
            }
        })
        .on_hover(move |hovered, window, cx| {
            let Some(context) = hover_context.as_ref() else {
                return;
            };
            if *hovered {
                context.update(cx, |runtime| {
                    runtime.set_popup_hovered(true);
                    runtime.cancel_hover();
                    runtime.disarm_safe_polygon();
                });
            } else {
                // Leaving the popup only closes a hover-opened popover;
                // click-opened popovers stay until dismissed.
                let (opened_by_hover, close_delay) = context.update(cx, |runtime| {
                    runtime.set_popup_hovered(false);
                    (runtime.opened_by_hover(), runtime.active_close_delay())
                });
                if !opened_by_hover {
                    return;
                }
                let generation = context.update(cx, |runtime| {
                    runtime.schedule_hover(PopoverHoverTarget::Close, None)
                });
                spawn_delayed_hover(
                    context.clone(),
                    None,
                    generation,
                    PopoverHoverTarget::Close,
                    close_delay.max(std::time::Duration::from_millis(50)),
                    window,
                    cx,
                );
            }
        })
        .children(children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime.set_bounds(PopoverBoundsKind::Popup, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverPopup<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.id = ElementId::from((context.root_id(), SharedString::from(self.id.to_string())));
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_popover_context(context.clone()))
            .collect();
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PopoverPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<PopoverPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PopoverPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    /// Accessible label for the dialog popup. Until gpui supports
    /// `aria-labelledby` relationships, pass the same string rendered inside
    /// `PopoverTitle`.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn payload_content(
        mut self,
        content: impl Fn(Option<&P>, &mut Window, &mut App) -> gpui::AnyElement + 'static,
    ) -> Self {
        self.payload_content = Some(Rc::new(content));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
