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

use crate::popover::{
    child_wiring::{PopoverChildNode, PopoverChildWiring},
    PopoverBoundsKind, PopoverCloseAction, PopoverContext, PopoverHandle, PopoverHoverTarget,
    PopoverOpenChangeReason, PopoverOpenChangeSource, PopoverToggleAction, PopoverTriggerMetadata,
    PopoverTriggerStyleState, POPOVER_KEY_CONTEXT,
};
use crate::primitives::safe_polygon::SafePolygonVerdict;

type PopoverTriggerStyle<P> = Rc<dyn Fn(PopoverTriggerStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct PopoverTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<PopoverContext<P>>,
    handle: Option<PopoverHandle<P>>,
    focus_handle: Option<FocusHandle>,
    scoped: bool,
    disabled: bool,
    payload: Option<P>,
    open_on_hover: bool,
    delay: Duration,
    close_delay: Duration,
    order: usize,
    aria_label: Option<SharedString>,
    style_with_state: Option<PopoverTriggerStyle<P>>,
}

impl<P: Clone + 'static> Default for PopoverTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("popover-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            handle: None,
            focus_handle: None,
            scoped: false,
            disabled: false,
            payload: None,
            open_on_hover: false,
            delay: Duration::ZERO,
            close_delay: Duration::ZERO,
            order: 0,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for PopoverTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for PopoverTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for PopoverTrigger<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let context = self
            .context
            .clone()
            .or_else(|| self.handle.as_ref().and_then(PopoverHandle::context));
        let source_id = self.id.clone();
        let scoped_id = context
            .as_ref()
            .filter(|_| !self.scoped)
            .map(|context| context.scope_trigger_id(&self.id))
            .unwrap_or_else(|| self.id.clone());
        let focus_handle = self
            .focus_handle
            .unwrap_or_else(|| trigger_focus_handle(&scoped_id, window, cx));

        if self.context.is_none() {
            if let Some(context) = context.as_ref() {
                let trigger = PopoverTriggerMetadata::new(
                    scoped_id.clone(),
                    source_id.clone(),
                    focus_handle.clone(),
                    self.disabled,
                    self.open_on_hover,
                    self.delay,
                    self.close_delay,
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
                PopoverTriggerStyleState::new(
                    self.disabled,
                    false,
                    false,
                    false,
                    false,
                    self.payload.is_some(),
                    self.payload.clone(),
                )
            });
        state.focused = focus_handle.is_focused(window);
        let disabled = state.disabled;
        let was_open = state.open;
        let was_active = state.active_trigger;
        let click_context = context.clone();
        let toggle_context = context.clone();
        let close_context = context.clone();
        let hover_context = context.clone();
        let move_context = context.clone();
        let measure_context = context.clone();
        let click_id = scoped_id.clone();
        let toggle_id = scoped_id.clone();
        let hover_id = scoped_id.clone();
        let measure_id = scoped_id.clone();
        let open_on_hover = self.open_on_hover;
        let hover_open_delay = self.delay;
        let hover_close_delay = self.close_delay;
        let click_focus_handle = focus_handle.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let expand_id = scoped_id.clone();

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
                        runtime.set_bounds(PopoverBoundsKind::Trigger(measure_id.clone()), bounds)
                    });
                    if changed {
                        window.request_animation_frame();
                    }
                }
            })
            .child(
                base.id(scoped_id)
                    // AccessKit gaps in this gpui revision: no builders for
                    // `aria-haspopup="dialog"`, `aria-controls` (trigger →
                    // popup id reference), or `aria-disabled`, so
                    // Role::Button + aria_expanded is the best available
                    // signal and AT cannot perceive the disabled state.
                    .role(Role::Button)
                    .aria_expanded(was_open && was_active)
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    .on_a11y_action(AccessibleAction::Expand, move |_data, window, cx| {
                        if disabled {
                            return;
                        }
                        if let Some(context) = expand_context.as_ref() {
                            context.open_trigger(
                                expand_id.clone(),
                                PopoverOpenChangeReason::TriggerPress,
                                PopoverOpenChangeSource::Unknown,
                                window,
                                cx,
                            );
                        }
                    })
                    .on_a11y_action(AccessibleAction::Collapse, move |_data, window, cx| {
                        if disabled {
                            return;
                        }
                        if let Some(context) = collapse_context.as_ref() {
                            context.close(
                                PopoverOpenChangeReason::TriggerPress,
                                PopoverOpenChangeSource::Unknown,
                                window,
                                cx,
                            );
                        }
                    })
                    .track_focus(&focus_handle.tab_stop(!disabled).tab_index(if disabled {
                        -1
                    } else {
                        0
                    }))
                    .focusable()
                    .key_context(POPOVER_KEY_CONTEXT)
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        cx.stop_propagation();
                        if disabled {
                            return;
                        }
                        click_focus_handle.focus(window, cx);
                        if let Some(context) = click_context.as_ref() {
                            context.set_open(
                                !(was_open && was_active),
                                Some(click_id.clone()),
                                PopoverOpenChangeReason::TriggerPress,
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
                            context.set_open(
                                !(was_open && was_active),
                                Some(toggle_id.clone()),
                                PopoverOpenChangeReason::TriggerPress,
                                PopoverOpenChangeSource::Keyboard,
                                window,
                                cx,
                            );
                        }
                    })
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
                        if disabled || !open_on_hover {
                            return;
                        }
                        if let Some(context) = hover_context.as_ref() {
                            if *hovered {
                                if hover_open_delay.is_zero() {
                                    context.update(cx, |runtime| {
                                        runtime.cancel_hover();
                                        runtime.disarm_safe_polygon();
                                    });
                                    context.set_open(
                                        true,
                                        Some(hover_id.clone()),
                                        PopoverOpenChangeReason::TriggerHover,
                                        PopoverOpenChangeSource::Pointer,
                                        window,
                                        cx,
                                    );
                                } else {
                                    let generation = context.update(cx, |runtime| {
                                        runtime.disarm_safe_polygon();
                                        runtime.schedule_hover(
                                            PopoverHoverTarget::Open,
                                            Some(hover_id.clone()),
                                        )
                                    });
                                    spawn_delayed_hover(
                                        context.clone(),
                                        Some(hover_id.clone()),
                                        generation,
                                        PopoverHoverTarget::Open,
                                        hover_open_delay,
                                        window,
                                        cx,
                                    );
                                }
                            } else {
                                let (keep_open, open) = context.update(cx, |runtime| {
                                    (
                                        runtime.should_keep_open_for_popup_hover(),
                                        runtime.open_value(),
                                    )
                                });
                                if keep_open {
                                    return;
                                }
                                if !open {
                                    context.update(cx, |runtime| runtime.cancel_hover());
                                    return;
                                }
                                // Arm the safe polygon so the pointer can
                                // travel from the trigger to the popup
                                // without the popover closing underneath it.
                                let pointer = window.mouse_position();
                                let generation = context.update(cx, |runtime| {
                                    runtime.arm_safe_polygon(pointer);
                                    runtime.schedule_hover(
                                        PopoverHoverTarget::Close,
                                        Some(hover_id.clone()),
                                    )
                                });
                                spawn_delayed_hover(
                                    context.clone(),
                                    Some(hover_id.clone()),
                                    generation,
                                    PopoverHoverTarget::Close,
                                    hover_close_delay.max(Duration::from_millis(50)),
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

impl<P: Clone + 'static> PopoverChildNode<P> for PopoverTrigger<P> {
    fn with_popover_context(mut self, context: PopoverContext<P>) -> Self {
        self.context = Some(context);
        self
    }

    fn wire_popover_child(
        mut self,
        wiring: &mut PopoverChildWiring<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let source_id = self.id.clone();
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = trigger_focus_handle(&scoped_id, window, cx);
        let order = wiring.register_trigger(
            scoped_id.clone(),
            source_id,
            focus_handle.clone(),
            self.disabled,
            self.open_on_hover,
            self.delay,
            self.close_delay,
            self.payload.clone(),
        );
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.scoped = true;
        self.order = order;
        self
    }
}

impl<P: Clone + 'static> PopoverTrigger<P> {
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

    pub fn handle(mut self, handle: PopoverHandle<P>) -> Self {
        self.handle = Some(handle);
        self
    }

    pub fn open_on_hover(mut self, open_on_hover: bool) -> Self {
        self.open_on_hover = open_on_hover;
        self
    }

    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn close_delay(mut self, close_delay: Duration) -> Self {
        self.close_delay = close_delay;
        self
    }

    /// Accessible label for the trigger; set this when the visible child is
    /// not plain text. Any visible text that duplicates this label should be
    /// rendered with `Text::new_inaccessible(...)` to avoid double-announcing.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(PopoverTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

pub fn spawn_delayed_hover<P: Clone + 'static>(
    context: PopoverContext<P>,
    trigger_id: Option<ElementId>,
    generation: u64,
    target: PopoverHoverTarget,
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
                    PopoverHoverTarget::Open => {
                        context.set_open(
                            true,
                            trigger_id,
                            PopoverOpenChangeReason::TriggerHover,
                            PopoverOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                    PopoverHoverTarget::Close => {
                        let keep_open = context
                            .read(cx, |runtime, _| runtime.should_keep_open_for_popup_hover());
                        if keep_open {
                            return;
                        }
                        context.update(cx, |runtime| runtime.disarm_safe_polygon());
                        context.close(
                            PopoverOpenChangeReason::TriggerHover,
                            PopoverOpenChangeSource::Pointer,
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
/// root, trigger, and popup mouse-move scopes.
pub fn evaluate_safe_polygon_move<P: Clone + 'static>(
    context: &PopoverContext<P>,
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
                runtime.schedule_hover(PopoverHoverTarget::Close, None)
            });
            spawn_delayed_hover(
                context.clone(),
                None,
                generation,
                PopoverHoverTarget::Close,
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
