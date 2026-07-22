use std::{
    rc::Rc,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::{
    preview_card::{
        child_wiring::{PreviewCardChildNode, PreviewCardChildWiring},
        PreviewCardBoundsKind, PreviewCardCloseAction, PreviewCardContext, PreviewCardFocusChange,
        PreviewCardHandle, PreviewCardHoverTarget, PreviewCardOpenChangeReason,
        PreviewCardOpenChangeSource, PreviewCardTriggerMetadata, PreviewCardTriggerStyleState,
        PREVIEW_CARD_KEY_CONTEXT,
    },
    primitives::safe_polygon::SafePolygonVerdict,
};

type PreviewCardTriggerStyle<P> = Rc<dyn Fn(PreviewCardTriggerStyleState<P>, Div) -> Div + 'static>;

/// Link-like trigger: opens on mouse hover (after the open delay) and on
/// focus, never on press; pressing an open trigger dismisses. GPUI has no
/// anchor element, so this is an interactive `div` — link navigation is the
/// consumer's concern. GPUI hover events do not distinguish mouse from touch
/// pointers, so strict mouse-only hover parity is documented as a gap
/// (same audit result as the Tooltip port). Focus opens immediately rather
/// than after the open delay — a deliberate deviation from Base UI's delayed
/// focus-open, matching the Tooltip port.
#[derive(IntoElement)]
pub struct PreviewCardTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PreviewCardContext<P>>,
    handle: Option<PreviewCardHandle<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    payload: Option<P>,
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    order: usize,
    aria_label: Option<SharedString>,
    style_with_state: Option<PreviewCardTriggerStyle<P>>,
}

impl<P: Clone + 'static> Default for PreviewCardTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("preview-card-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            handle: None,
            focus_handle: None,
            scoped: false,
            payload: None,
            delay: None,
            close_delay: None,
            order: 0,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PreviewCardTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PreviewCardTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PreviewCardTrigger<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let detached_trigger = self.context.is_none();
        let context = self
            .context
            .clone()
            .or_else(|| self.handle.as_ref().and_then(PreviewCardHandle::context));
        let source_id = self.id.clone();
        let scoped_id = context
            .as_ref()
            .filter(|_| !self.scoped)
            .map(|context| context.scope_trigger_id(&self.id))
            .unwrap_or_else(|| self.id.clone());
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| trigger_focus_handle(&scoped_id, window, cx));
        let (delay, close_delay) =
            effective_delays(self.delay, self.close_delay, context.as_ref(), cx);

        if detached_trigger {
            if let Some(context) = context.as_ref() {
                let trigger = PreviewCardTriggerMetadata::new(
                    scoped_id.clone(),
                    source_id.clone(),
                    focus_handle.clone(),
                    delay,
                    close_delay,
                    self.payload.clone(),
                    self.order,
                    true,
                );
                context.update(cx, |runtime| runtime.register_detached_trigger(trigger));
            }
        }

        let mut state = context
            .as_ref()
            .map(|context| {
                context.read(cx, |runtime, _| {
                    runtime.trigger_state(&scoped_id, self.payload.is_some())
                })
            })
            .unwrap_or_else(|| {
                PreviewCardTriggerStyleState::new(
                    false,
                    false,
                    false,
                    false,
                    scoped_id.clone(),
                    self.payload.is_some(),
                    self.payload.clone(),
                )
            });
        state.focused = focus_handle.is_focused(window);
        if detached_trigger {
            if let Some(context) = context.as_ref() {
                let focus_change = context.update(cx, |runtime| {
                    runtime.sync_detached_trigger_focus(scoped_id.clone(), state.focused)
                });
                apply_focus_change(context, focus_change, window, cx);
            }
        }

        let press_context = context.clone();
        let a11y_press_context = context.clone();
        let close_context = context.clone();
        let hover_context = context.clone();
        let move_context = context.clone();
        let measure_context = context.clone();
        let press_id = scoped_id.clone();
        let a11y_press_id = scoped_id.clone();
        let hover_id = scoped_id.clone();
        let press_focus_handle = focus_handle.clone();
        let a11y_press_focus_handle = focus_handle.clone();
        let measure_id = scoped_id.clone();
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                if let Some(context) = measure_context.as_ref() {
                    let changed = context.update(cx, |runtime| {
                        runtime
                            .set_bounds(PreviewCardBoundsKind::Trigger(measure_id.clone()), bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(
                base.occlude()
                    .id(scoped_id)
                    // Base UI's trigger renders an `<a>`; Role::Link mirrors
                    // that. gpui surfaces no href/URL property, so link
                    // navigation remains the consumer's press concern. Base UI
                    // emits no aria-expanded on this trigger, so we omit it
                    // too (parity, not an omission).
                    .role(Role::Link)
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    // Mirrors on_mouse_down: pressing an open trigger
                    // dismisses (TriggerPress), otherwise the press focuses
                    // the trigger and focus-open takes over. `Action::Focus`
                    // is auto-registered by `.track_focus(...)`; `on_click`
                    // is not used here, so Click must be registered manually.
                    .on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
                        if let Some(context) = a11y_press_context.as_ref() {
                            let close_active = context.update(cx, |runtime| {
                                runtime.sync_trigger_press(a11y_press_id.clone())
                            });
                            if close_active {
                                context.close(
                                    PreviewCardOpenChangeReason::TriggerPress,
                                    PreviewCardOpenChangeSource::Unknown,
                                    window,
                                    cx,
                                );
                                return;
                            }
                        }
                        a11y_press_focus_handle.focus(window, cx);
                    })
                    .track_focus(&focus_handle.tab_stop(true).tab_index(0))
                    .focusable()
                    .key_context(PREVIEW_CARD_KEY_CONTEXT)
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        if let Some(context) = press_context.as_ref() {
                            let close_active = context
                                .update(cx, |runtime| runtime.sync_trigger_press(press_id.clone()));
                            if close_active {
                                context.close(
                                    PreviewCardOpenChangeReason::TriggerPress,
                                    PreviewCardOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                                return;
                            }
                        }
                        press_focus_handle.focus(window, cx);
                    })
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
                            let suppressed = context.update(cx, |runtime| {
                                runtime.cancel_hover();
                                runtime.disarm_safe_polygon();
                                runtime.set_trigger_hovered(Some(hover_id.clone()));
                                runtime.is_trigger_press_suppressed(&hover_id)
                            });
                            if suppressed {
                                return;
                            }
                            let already_open = context.read(cx, |runtime, _| runtime.open_value());
                            let open_delay = match already_open {
                                true => Duration::ZERO,
                                false => delay,
                            };
                            let reason = match already_open {
                                true => PreviewCardOpenChangeReason::None,
                                false => PreviewCardOpenChangeReason::TriggerHover,
                            };
                            if open_delay.is_zero() {
                                context.set_open(
                                    true,
                                    Some(hover_id.clone()),
                                    reason,
                                    PreviewCardOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                            } else {
                                let generation = context.update(cx, |runtime| {
                                    runtime.schedule_hover(
                                        PreviewCardHoverTarget::Open,
                                        Some(hover_id.clone()),
                                    )
                                });
                                spawn_delayed_hover(
                                    context.clone(),
                                    Some(hover_id.clone()),
                                    generation,
                                    PreviewCardHoverTarget::Open,
                                    open_delay,
                                    window,
                                    cx,
                                );
                            }
                        } else {
                            let keep_open = context.update(cx, |runtime| {
                                runtime.clear_trigger_hovered(&hover_id);
                                runtime.should_keep_open_for_popup_hover()
                                    || runtime.should_keep_open_for_trigger_unhover(&hover_id)
                                    || runtime.another_trigger_hovered(&hover_id)
                            });
                            if keep_open {
                                return;
                            }
                            let open = context.read(cx, |runtime, _| runtime.open_value());
                            if !open {
                                context.update(cx, |runtime| runtime.cancel_hover());
                                return;
                            }
                            let pointer = window.mouse_position();
                            let generation = context.update(cx, |runtime| {
                                runtime.arm_safe_polygon(pointer);
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
                    })
                    .children(self.children),
            )
    }
}

impl<P: Clone + 'static> PreviewCardChildNode<P> for PreviewCardTrigger<P> {
    fn with_preview_card_context(mut self, context: PreviewCardContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_preview_card_child(
        mut self,
        wiring: &mut PreviewCardChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let source_id = self.id.clone();
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = trigger_focus_handle(&scoped_id, window, cx);
        let delay = wiring.effective_delay(self.delay);
        let close_delay = wiring.effective_close_delay(self.close_delay);
        let order = wiring.register_trigger(
            scoped_id.clone(),
            source_id,
            focus_handle.clone(),
            delay,
            close_delay,
            self.payload.clone(),
        );
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.order = order;
        self
    }
}

impl<P: Clone + 'static> PreviewCardTrigger<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn payload(mut self, payload: P) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn handle(mut self, handle: PreviewCardHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn close_delay(mut self, close_delay: Duration) -> Self {
        self.close_delay = Some(close_delay);
        self
    }

    /// Accessible name for the link trigger. When set, construct the visible
    /// label children with `Text::new_inaccessible(...)` instead of `text!`
    /// so the label is not announced twice.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PreviewCardTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

pub fn apply_focus_change<P: Clone + 'static>(
    context: &PreviewCardContext<P>,
    focus_change: PreviewCardFocusChange,
    window: &mut Window,
    cx: &mut App,
) {
    match focus_change {
        PreviewCardFocusChange::Open(trigger_id) => {
            context.open_trigger(
                trigger_id,
                PreviewCardOpenChangeReason::TriggerFocus,
                PreviewCardOpenChangeSource::Focus,
                window,
                cx,
            );
        }
        PreviewCardFocusChange::Close => {
            context.close(
                PreviewCardOpenChangeReason::TriggerFocus,
                PreviewCardOpenChangeSource::Focus,
                window,
                cx,
            );
        }
        PreviewCardFocusChange::None => {}
    }
}

fn effective_delays<P: Clone + 'static>(
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    context: Option<&PreviewCardContext<P>>,
    cx: &App,
) -> (Duration, Duration) {
    let (default_delay, default_close_delay) = context
        .map(|context| context.read(cx, |_runtime, props| (props.delay(), props.close_delay())))
        .unwrap_or((
            crate::preview_card::DEFAULT_PREVIEW_CARD_DELAY,
            crate::preview_card::DEFAULT_PREVIEW_CARD_CLOSE_DELAY,
        ));
    (
        delay.unwrap_or(default_delay),
        close_delay.unwrap_or(default_close_delay),
    )
}

pub fn spawn_delayed_hover<P: Clone + 'static>(
    context: PreviewCardContext<P>,
    trigger_id: Option<ElementId>,
    generation: u64,
    target: PreviewCardHoverTarget,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_hover(generation, target, trigger_id.as_ref())
                });
                if !current {
                    return;
                }

                match target {
                    PreviewCardHoverTarget::Open => {
                        context.set_open(
                            true,
                            trigger_id,
                            PreviewCardOpenChangeReason::TriggerHover,
                            PreviewCardOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                    PreviewCardHoverTarget::Close => {
                        let keep_for_popup = context
                            .read(cx, |runtime, _| runtime.should_keep_open_for_popup_hover());
                        if keep_for_popup {
                            return;
                        }
                        context.update(cx, |runtime| runtime.disarm_safe_polygon());
                        context.close(
                            PreviewCardOpenChangeReason::TriggerHover,
                            PreviewCardOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                }
            })
            .ok();
        })
        .detach();
}

/// Feeds a pointer position to the runtime's armed safe polygon and acts on
/// the verdict per the primitive's integration contract. Observed from the
/// root, trigger, and popup mouse-move scopes (interim: window-level
/// observation is deferred; the close delay covers the uncovered trough).
pub fn evaluate_safe_polygon_move<P: Clone + 'static>(
    context: &PreviewCardContext<P>,
    pointer: gpui::Point<gpui::Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let verdict = context.update(cx, |runtime| {
        runtime.evaluate_safe_polygon(pointer, polygon_now())
    });
    let Some(verdict) = verdict else {
        return;
    };
    match verdict {
        SafePolygonVerdict::Inside => {
            let generation = context.update(cx, |runtime| {
                runtime.schedule_hover(PreviewCardHoverTarget::Close, None)
            });
            spawn_delayed_hover(
                context.clone(),
                None,
                generation,
                PreviewCardHoverTarget::Close,
                Duration::from_millis(40),
                window,
                cx,
            );
        }
        SafePolygonVerdict::Outside => {
            context.update(cx, |runtime| runtime.disarm_safe_polygon());
        }
        SafePolygonVerdict::LandedPopup => {
            context.update(cx, |runtime| {
                runtime.set_popup_hovered(true);
                runtime.cancel_hover();
            });
        }
        SafePolygonVerdict::LandedTrigger => {
            context.update(cx, |runtime| runtime.cancel_hover());
        }
    }
}

fn polygon_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
}

fn trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
