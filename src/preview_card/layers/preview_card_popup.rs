use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::preview_card::{
    child_wiring::{PreviewCardChildNode, PreviewCardChildWiring},
    layers::preview_card_trigger::{evaluate_safe_polygon_move, spawn_delayed_hover},
    PreviewCardAlign, PreviewCardBoundsKind, PreviewCardCloseAction, PreviewCardContext,
    PreviewCardHoverTarget, PreviewCardInstant, PreviewCardOpenChangeReason,
    PreviewCardOpenChangeSource, PreviewCardPayloadContentBuilder, PreviewCardPopupChild,
    PreviewCardPopupStyleState, PreviewCardSide, PREVIEW_CARD_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct PreviewCardPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<PreviewCardPopupChild<P>>,
    context: Option<PreviewCardContext<P>>,
    side: PreviewCardSide,
    align: PreviewCardAlign,
    keep_mounted: bool,
    payload_content: Option<PreviewCardPayloadContentBuilder<P>>,
    style_with_state: Option<Rc<dyn Fn(PreviewCardPopupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for PreviewCardPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("preview-card-popup"),
            base: div().relative(),
            children: Vec::new(),
            context: None,
            side: PreviewCardSide::Bottom,
            align: PreviewCardAlign::Center,
            keep_mounted: false,
            payload_content: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for PreviewCardPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardPopup<P> {
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
                PreviewCardPopupStyleState::new(
                    false,
                    self.keep_mounted,
                    self.side,
                    self.align,
                    PreviewCardInstant::None,
                )
            });
        if !state.mounted {
            return div();
        }

        let context = self.context.clone();
        let close_context = context.clone();
        let hover_context = context.clone();
        let move_context = context.clone();
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

        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(self.id);
        if state.open {
            base = base
                .key_context(PREVIEW_CARD_KEY_CONTEXT)
                .on_action(move |_: &PreviewCardCloseAction, window, cx| {
                    if let Some(context) = close_context.as_ref() {
                        context.close(
                            PreviewCardOpenChangeReason::EscapeKey,
                            PreviewCardOpenChangeSource::Keyboard,
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
                        let close_delay = context.update(cx, |runtime| {
                            runtime.set_popup_hovered(false);
                            runtime.active_close_delay()
                        });
                        if close_delay.is_zero() {
                            context.close(
                                PreviewCardOpenChangeReason::TriggerHover,
                                PreviewCardOpenChangeSource::Pointer,
                                window,
                                cx,
                            );
                        } else {
                            let generation = context.update(cx, |runtime| {
                                runtime.schedule_hover(PreviewCardHoverTarget::Close, None)
                            });
                            spawn_delayed_hover(
                                context.clone(),
                                None,
                                generation,
                                PreviewCardHoverTarget::Close,
                                close_delay,
                                window,
                                cx,
                            );
                        }
                    }
                });
        } else {
            base = base.opacity(0.0).invisible();
        }
        let base = base.children(children);

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime.set_bounds(PreviewCardBoundsKind::Popup, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardPopup<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.id = ElementId::from((context.root_id(), SharedString::from(self.id.to_string())));
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_preview_card_context(context.clone()))
            .collect();
        self
    }

    fn wire_preview_card_child(
        mut self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> PreviewCardPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<PreviewCardPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(PreviewCardPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: PreviewCardSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PreviewCardAlign) -> Self {
        self.align = align;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
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
        style: impl Fn(PreviewCardPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
