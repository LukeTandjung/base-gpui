use std::{rc::Rc, sync::Arc, time::Duration};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, Div, ElementId, Entity,
    FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::tooltip::{
    child_wiring::{TooltipChildNode, TooltipChildWiring},
    TooltipBoundsKind, TooltipCloseAction, TooltipContext, TooltipHandle, TooltipHoverTarget,
    TooltipOpenChangeReason, TooltipOpenChangeSource, TooltipTriggerMetadata,
    TooltipTriggerStyleState, TOOLTIP_KEY_CONTEXT,
};

type TooltipTriggerStyle<P> = Rc<dyn Fn(TooltipTriggerStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct TooltipTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<TooltipContext<P>>,
    handle: Option<TooltipHandle<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    disabled: bool,
    payload: Option<P>,
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    close_on_click: bool,
    order: usize,
    aria_label: Option<SharedString>,
    style_with_state: Option<TooltipTriggerStyle<P>>,
}

impl<P: Clone + 'static> Default for TooltipTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("tooltip-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            handle: None,
            focus_handle: None,
            scoped: false,
            disabled: false,
            payload: None,
            delay: None,
            close_delay: None,
            close_on_click: true,
            order: 0,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for TooltipTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for TooltipTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for TooltipTrigger<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let detached_trigger = self.context.is_none();
        let context = self
            .context
            .clone()
            .or_else(|| self.handle.as_ref().and_then(TooltipHandle::context));
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

        if self.context.is_none() {
            if let Some(context) = context.as_ref() {
                let trigger = TooltipTriggerMetadata::new(
                    scoped_id.clone(),
                    source_id.clone(),
                    focus_handle.clone(),
                    self.disabled,
                    delay,
                    close_delay,
                    self.close_on_click,
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
                    runtime.trigger_state(&scoped_id, self.disabled, self.payload.is_some())
                })
            })
            .unwrap_or_else(|| {
                TooltipTriggerStyleState::new(
                    self.disabled,
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
                match focus_change {
                    crate::tooltip::TooltipFocusChange::Open(trigger_id) => {
                        context.open_trigger(
                            trigger_id,
                            TooltipOpenChangeReason::TriggerFocus,
                            TooltipOpenChangeSource::Focus,
                            window,
                            cx,
                        );
                    }
                    crate::tooltip::TooltipFocusChange::Close => {
                        context.close(
                            TooltipOpenChangeReason::TriggerFocus,
                            TooltipOpenChangeSource::Focus,
                            window,
                            cx,
                        );
                    }
                    crate::tooltip::TooltipFocusChange::None => {}
                }
            }
        }
        let disabled = state.disabled;
        let close_on_click = self.close_on_click;
        let press_context = context.clone();
        let close_context = context.clone();
        let hover_context = context.clone();
        let move_context = context.clone();
        let measure_context = context.clone();
        let a11y_press_context = context.clone();
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
                        runtime.set_bounds(TooltipBoundsKind::Trigger(measure_id.clone()), bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(
                base.occlude()
                    .id(scoped_id)
                    // Base UI renders the trigger as a native `<button>`;
                    // Role::Button mirrors that. Base UI emits no
                    // aria-expanded on tooltip triggers, so we omit it too.
                    // There is no `.aria_disabled(...)` builder in this gpui
                    // revision, so AT cannot see a disabled state; disabled
                    // triggers are kept out of tab order and skip the Click
                    // a11y handler instead.
                    .role(Role::Button)
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    // Mirrors on_mouse_down: an AT-dispatched Click routes
                    // through the same runtime press transition (with
                    // close_on_click) as the pointer path. `Action::Focus` is
                    // auto-registered by `.track_focus(...)`; `on_click` is
                    // not used here, so Click must be registered manually.
                    .when(!disabled, |this| {
                        this.on_a11y_action(AccessibleAction::Click, move |_data, window, cx| {
                            if let Some(context) = a11y_press_context.as_ref() {
                                let press_change = context.update(cx, |runtime| {
                                    runtime.sync_trigger_press(
                                        a11y_press_id.clone(),
                                        close_on_click,
                                        detached_trigger,
                                    )
                                });
                                if press_change.close_active() {
                                    context.close(
                                        TooltipOpenChangeReason::TriggerPress,
                                        TooltipOpenChangeSource::Unknown,
                                        window,
                                        cx,
                                    );
                                    return;
                                }

                                a11y_press_focus_handle.focus(window, cx);
                                if press_change.open_detached_focus() {
                                    context.open_trigger(
                                        a11y_press_id.clone(),
                                        TooltipOpenChangeReason::TriggerFocus,
                                        TooltipOpenChangeSource::Focus,
                                        window,
                                        cx,
                                    );
                                }
                            } else {
                                a11y_press_focus_handle.focus(window, cx);
                            }
                        })
                    })
                    .track_focus(&focus_handle.tab_stop(!disabled).tab_index(if disabled {
                        -1
                    } else {
                        0
                    }))
                    .focusable()
                    .key_context(TOOLTIP_KEY_CONTEXT)
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        if disabled {
                            return;
                        }
                        if let Some(context) = press_context.as_ref() {
                            let press_change = context.update(cx, |runtime| {
                                runtime.sync_trigger_press(
                                    press_id.clone(),
                                    close_on_click,
                                    detached_trigger,
                                )
                            });
                            if press_change.close_active() {
                                context.close(
                                    TooltipOpenChangeReason::TriggerPress,
                                    TooltipOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                                return;
                            }

                            press_focus_handle.focus(window, cx);
                            if press_change.open_detached_focus() {
                                context.open_trigger(
                                    press_id.clone(),
                                    TooltipOpenChangeReason::TriggerFocus,
                                    TooltipOpenChangeSource::Focus,
                                    window,
                                    cx,
                                );
                            }
                        } else {
                            press_focus_handle.focus(window, cx);
                        }
                    })
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
                    .on_mouse_move(move |event, _window, cx| {
                        if let Some(context) = move_context.as_ref() {
                            context
                                .update(cx, |runtime| runtime.set_cursor_position(event.position));
                        }
                    })
                    .on_hover(move |hovered, window, cx| {
                        if disabled {
                            if *hovered {
                                if let Some(context) = hover_context.as_ref() {
                                    let should_close = context.update(cx, |runtime| {
                                        runtime.sync_disabled_trigger_hover(&hover_id)
                                    });
                                    if should_close {
                                        context.close(
                                            TooltipOpenChangeReason::Disabled,
                                            TooltipOpenChangeSource::Pointer,
                                            window,
                                            cx,
                                        );
                                    }
                                }
                            }
                            return;
                        }
                        if let Some(context) = hover_context.as_ref() {
                            if *hovered {
                                let suppressed = context.update(cx, |runtime| {
                                    runtime.cancel_hover();
                                    runtime.set_trigger_hovered(Some(hover_id.clone()));
                                    runtime.is_trigger_press_suppressed(&hover_id)
                                });
                                if suppressed {
                                    return;
                                }
                                let instant = context.read(cx, |runtime, props| {
                                    runtime.open_value()
                                        || props.delay_group().should_open_instantly()
                                });
                                let open_delay = match instant {
                                    true => Duration::ZERO,
                                    false => delay,
                                };
                                let reason = match instant {
                                    true => TooltipOpenChangeReason::None,
                                    false => TooltipOpenChangeReason::TriggerHover,
                                };
                                if open_delay.is_zero() {
                                    context.set_open(
                                        true,
                                        Some(hover_id.clone()),
                                        reason,
                                        TooltipOpenChangeSource::Pointer,
                                        window,
                                        cx,
                                    );
                                } else {
                                    let generation = context.update(cx, |runtime| {
                                        runtime.schedule_hover(
                                            TooltipHoverTarget::Open,
                                            Some(hover_id.clone()),
                                        )
                                    });
                                    spawn_delayed_hover(
                                        context.clone(),
                                        hover_id.clone(),
                                        generation,
                                        TooltipHoverTarget::Open,
                                        open_delay,
                                        window,
                                        cx,
                                    );
                                }
                            } else {
                                let keep_open = context.update(cx, |runtime| {
                                    runtime.set_trigger_hovered(None);
                                    runtime.should_keep_open_for_popup_hover()
                                        || runtime.should_keep_open_for_trigger_unhover(&hover_id)
                                });
                                if keep_open {
                                    return;
                                }
                                let pointer = window.mouse_position();
                                let close_delay = context.read(cx, |runtime, _| {
                                    runtime.close_delay_for_trigger_unhover(
                                        &hover_id,
                                        pointer,
                                        close_delay,
                                    )
                                });
                                let generation = context.update(cx, |runtime| {
                                    runtime.schedule_hover(
                                        TooltipHoverTarget::Close,
                                        Some(hover_id.clone()),
                                    )
                                });
                                spawn_delayed_hover(
                                    context.clone(),
                                    hover_id.clone(),
                                    generation,
                                    TooltipHoverTarget::Close,
                                    close_delay,
                                    window,
                                    cx,
                                );
                            }
                        }
                    })
                    .children(self.children),
            )
    }
}

impl<P: Clone + 'static> TooltipChildNode<P> for TooltipTrigger<P> {
    fn with_tooltip_context(mut self, context: TooltipContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_tooltip_child(
        mut self,
        wiring: &mut TooltipChildWiring<P>,
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
            self.disabled,
            delay,
            close_delay,
            self.close_on_click,
            self.payload.clone(),
        );
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.order = order;
        self
    }
}

impl<P: Clone + 'static> TooltipTrigger<P> {
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

    pub fn payload(mut self, payload: P) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn handle(mut self, handle: TooltipHandle<P>) -> Self {
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

    pub fn close_on_click(mut self, close_on_click: bool) -> Self {
        self.close_on_click = close_on_click;
        self
    }

    /// Accessible name for the trigger. There is no derived default because
    /// trigger children are arbitrary elements, so icon-only triggers must set
    /// this. When a visible text child duplicates this label, render it with
    /// `Text::new_inaccessible(...)` instead of `text!(...)` to avoid
    /// double-announcing. Tooltip content is *not* an accessible description
    /// of the trigger (no `aria-describedby` relationship exists in this gpui
    /// revision), so do not rely on the popup for the trigger's name.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TooltipTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn effective_delays<P: Clone + 'static>(
    delay: Option<Duration>,
    close_delay: Option<Duration>,
    context: Option<&TooltipContext<P>>,
    cx: &App,
) -> (Duration, Duration) {
    let provider = context
        .map(|context| context.read(cx, |_runtime, props| props.provider()))
        .unwrap_or_default();
    (
        delay.unwrap_or_else(|| provider.delay()),
        close_delay.unwrap_or_else(|| provider.close_delay()),
    )
}

fn spawn_delayed_hover<P: Clone + 'static>(
    context: TooltipContext<P>,
    trigger_id: ElementId,
    generation: u64,
    target: TooltipHoverTarget,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_hover(generation, target, Some(&trigger_id))
                });
                if !current {
                    return;
                }

                match target {
                    TooltipHoverTarget::Open => {
                        context.set_open(
                            true,
                            Some(trigger_id),
                            TooltipOpenChangeReason::TriggerHover,
                            TooltipOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                    TooltipHoverTarget::Close => {
                        let keep_for_popup = context
                            .read(cx, |runtime, _| runtime.should_keep_open_for_popup_hover());
                        if keep_for_popup {
                            return;
                        }
                        context.close(
                            TooltipOpenChangeReason::TriggerHover,
                            TooltipOpenChangeSource::Pointer,
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

fn trigger_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
