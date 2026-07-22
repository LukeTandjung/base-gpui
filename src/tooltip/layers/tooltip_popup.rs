use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::tooltip::{
    child_wiring::{TooltipChildNode, TooltipChildWiring},
    TooltipAlign, TooltipBoundsKind, TooltipCloseAction, TooltipContext, TooltipOpenChangeReason,
    TooltipOpenChangeSource, TooltipPayloadContentBuilder, TooltipPopupChild,
    TooltipPopupStyleState, TooltipSide, TOOLTIP_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct TooltipPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<TooltipPopupChild<P>>,
    context: Option<TooltipContext<P>>,
    side: TooltipSide,
    align: TooltipAlign,
    keep_mounted: bool,
    payload_content: Option<TooltipPayloadContentBuilder<P>>,
    style_with_state: Option<Rc<dyn Fn(TooltipPopupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for TooltipPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tooltip-popup"),
            base: div().relative(),
            children: Vec::new(),
            context: None,
            side: TooltipSide::Top,
            align: TooltipAlign::Center,
            keep_mounted: false,
            payload_content: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for TooltipPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipPopup<P> {
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
                TooltipPopupStyleState::new(
                    false,
                    self.keep_mounted,
                    self.side,
                    self.align,
                    crate::tooltip::TooltipInstant::Delay,
                )
            });
        if !state.mounted {
            return div();
        }

        let context = self.context.clone();
        let close_context = context.clone();
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

        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(self.id);
        if state.open {
            base = base
                // Base UI's popup is a plain `div`; Role::Tooltip is the
                // closest AccessKit node for the popup content. The role is
                // only set while open so closed-but-kept-mounted (invisible)
                // subtrees stay out of the accessibility tree. No live-region
                // announcement API exists in this gpui revision, so tooltip
                // content is not announced on open.
                .role(Role::Tooltip)
                .key_context(TOOLTIP_KEY_CONTEXT)
                .on_action(move |_: &TooltipCloseAction, window, cx| {
                    if let Some(context) = close_context.as_ref() {
                        context.close(
                            TooltipOpenChangeReason::EscapeKey,
                            TooltipOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    }
                })
                .on_hover(move |hovered, window, cx| {
                    if let Some(context) = hover_context.as_ref() {
                        if *hovered {
                            let hoverable =
                                context.read(cx, |runtime, _| runtime.hoverable_popup_enabled());
                            if !hoverable {
                                context.close(
                                    TooltipOpenChangeReason::TriggerHover,
                                    TooltipOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                                return;
                            }
                            context.update(cx, |runtime| {
                                runtime.set_popup_hovered(true);
                                runtime.cancel_hover();
                            });
                        } else {
                            let hoverable =
                                context.read(cx, |runtime, _| runtime.hoverable_popup_enabled());
                            if !hoverable {
                                return;
                            }
                            let close_delay = context.update(cx, |runtime| {
                                runtime.set_popup_hovered(false);
                                runtime.active_close_delay()
                            });
                            if close_delay.is_zero() {
                                context.close(
                                    TooltipOpenChangeReason::TriggerHover,
                                    TooltipOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                            } else {
                                let generation = context.update(cx, |runtime| {
                                    runtime.schedule_hover(
                                        crate::tooltip::TooltipHoverTarget::Close,
                                        None,
                                    )
                                });
                                spawn_popup_close(
                                    context.clone(),
                                    generation,
                                    close_delay,
                                    window,
                                    cx,
                                );
                            }
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
                        runtime.set_bounds(TooltipBoundsKind::Popup, bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(base)
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipPopup<P> {
    fn with_tooltip_context(mut self, context: TooltipContext<P>) -> Self {
        self.id = ElementId::from((context.root_id(), SharedString::from(self.id.to_string())));
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_tooltip_context(context.clone()))
            .collect();
        self
    }

    fn wire_tooltip_child(
        mut self,
        wiring: &mut TooltipChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_popup_children(self.children, window, cx);
        self
    }
}

impl<P: Clone + 'static> TooltipPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<TooltipPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(TooltipPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: TooltipAlign) -> Self {
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
        style: impl Fn(TooltipPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn spawn_popup_close<P: Clone + 'static>(
    context: TooltipContext<P>,
    generation: u64,
    delay: std::time::Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_hover(
                        generation,
                        crate::tooltip::TooltipHoverTarget::Close,
                        None,
                    )
                });
                if !current {
                    return;
                }
                let keep_for_popup =
                    context.read(cx, |runtime, _| runtime.should_keep_open_for_popup_hover());
                if keep_for_popup {
                    return;
                }
                context.close(
                    TooltipOpenChangeReason::TriggerHover,
                    TooltipOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .ok();
        })
        .detach();
}
