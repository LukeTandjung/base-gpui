use std::{
    rc::Rc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, AccessibleAction, App, Div, FocusHandle, InteractiveElement as _, IntoElement,
    MouseButton, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    navigation_menu::{
        child_wiring::NavigationMenuChildNode, NavigationMenuBoundsKind, NavigationMenuContext,
        NavigationMenuHoverTarget, NavigationMenuTriggerChild, NavigationMenuTriggerStyleState,
        NavigationMenuValueChangeReason, NavigationMenuValueChangeSource,
    },
    primitives::safe_polygon::SafePolygonVerdict,
};

type NavigationMenuTriggerStyle = Rc<dyn Fn(NavigationMenuTriggerStyleState, Div) -> Div + 'static>;

/// Opens its item's content on hover (after the root delay; immediately when
/// the popup is already open — retarget), on click (with patient-click
/// stickiness after a hover open), and via list keyboard navigation. GPUI
/// hover events do not distinguish mouse from touch pointers, so touch/pen
/// hover suppression is documented as deferred (same audit result as
/// Tooltip/Preview Card). A disabled trigger never opens but remains
/// focusable (Base UI `focusableWhenDisabled`).
///
/// Accessibility: `Role::Button` with `aria_expanded` from the active-item
/// fact; pass `.aria_label(...)` for the accessible name (no id-reference
/// labelling exists). AT `Click` routes through the same toggle/retarget
/// transition as the pointer path, bypassing the pointer-only patient-click
/// window; while `disabled` the Click handler is withheld — gpui has no
/// `aria-disabled` builder, so AT cannot perceive the disabled state (blocked
/// pending gpui upstream). `aria-controls`/`aria-owns` relationship props
/// have no gpui builders and are omitted.
#[derive(IntoElement)]
pub struct NavigationMenuTrigger<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuTriggerChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    value: Option<T>,
    disabled: bool,
    focus_handle: Option<FocusHandle>,
    entry_index: usize,
    order: usize,
    aria_label: Option<SharedString>,
    style_with_state: Option<NavigationMenuTriggerStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuTrigger<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            value: None,
            disabled: false,
            focus_handle: None,
            entry_index: 0,
            order: 0,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuTrigger<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuTrigger<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(context), Some(value)) = (self.context.clone(), self.value.clone()) else {
            return div().children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );
        };

        let disabled = self.disabled;
        let state = context.read(cx, |runtime, _| runtime.trigger_state(&value, disabled));
        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuTriggerChild::Icon(icon) => icon
                    .with_value(value.clone())
                    .with_navigation_menu_context(context.clone())
                    .into_any_element(),
                NavigationMenuTriggerChild::Any(any) => any,
            })
            .collect::<Vec<_>>();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let measure_context = context.clone();
        let measure_value = value.clone();
        let press_context = context.clone();
        let press_value = value.clone();
        let a11y_context = context.clone();
        let a11y_value = value.clone();
        let hover_context = context.clone();
        let hover_value = value.clone();
        let move_context = context.clone();
        let focus_handle = self
            .focus_handle
            .clone()
            .unwrap_or_else(|| cx.focus_handle());

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_bounds(
                        NavigationMenuBoundsKind::Trigger(measure_value.clone()),
                        bounds,
                    )
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child({
                let mut trigger = base
                    .id(("navigation-menu-trigger", self.order))
                    .role(Role::Button)
                    .aria_expanded(state.open)
                    .track_focus(&focus_handle.tab_stop(true).tab_index(0));
                if let Some(aria_label) = self.aria_label.clone() {
                    trigger = trigger.aria_label(aria_label);
                }
                if !disabled {
                    // AT-dispatched Click: same toggle/retarget transition as
                    // the pointer path, minus the pointer-only patient-click
                    // stickiness window. Withheld while disabled (no
                    // aria-disabled builder exists to expose the state).
                    trigger = trigger.on_a11y_action(
                        AccessibleAction::Click,
                        move |_data, window, cx| {
                            let active = a11y_context
                                .read(cx, |runtime, _| runtime.is_active_value(&a11y_value));
                            if active {
                                a11y_context.close(
                                    NavigationMenuValueChangeReason::TriggerPress,
                                    NavigationMenuValueChangeSource::Unknown,
                                    window,
                                    cx,
                                );
                                return;
                            }
                            a11y_context.update(cx, |runtime| runtime.cancel_hover());
                            a11y_context.set_value(
                                Some(a11y_value.clone()),
                                NavigationMenuValueChangeReason::TriggerPress,
                                NavigationMenuValueChangeSource::Unknown,
                                window,
                                cx,
                            );
                        },
                    );
                }
                trigger
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        if disabled {
                            return;
                        }
                        let active = press_context
                            .read(cx, |runtime, _| runtime.is_active_value(&press_value));
                        if active {
                            let sticky = press_context
                                .read(cx, |runtime, _| runtime.patient_click_blocks_close(now()));
                            if sticky {
                                return;
                            }
                            press_context.close(
                                NavigationMenuValueChangeReason::TriggerPress,
                                NavigationMenuValueChangeSource::Pointer,
                                window,
                                cx,
                            );
                            return;
                        }
                        press_context.update(cx, |runtime| runtime.cancel_hover());
                        press_context.set_value(
                            Some(press_value.clone()),
                            NavigationMenuValueChangeReason::TriggerPress,
                            NavigationMenuValueChangeSource::Pointer,
                            window,
                            cx,
                        );
                    })
                    .on_mouse_move(move |event, window, cx| {
                        evaluate_safe_polygon_move(&move_context, event.position, window, cx);
                    })
                    .on_hover(move |hovered, window, cx| {
                        if *hovered {
                            hover_context.update(cx, |runtime| {
                                runtime.cancel_hover();
                                runtime.disarm_safe_polygon();
                                runtime.set_trigger_hovered(Some(hover_value.clone()));
                            });
                            if disabled {
                                return;
                            }
                            let (already_open, delay) = hover_context
                                .read(cx, |runtime, props| (runtime.open_value(), props.delay()));
                            if already_open {
                                // Switching triggers while open retargets
                                // immediately (Base UI's restMs collapse).
                                let changed = hover_context.set_value(
                                    Some(hover_value.clone()),
                                    NavigationMenuValueChangeReason::TriggerHover,
                                    NavigationMenuValueChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                                if changed {
                                    hover_context
                                        .update(cx, |runtime| runtime.note_hover_open(now()));
                                }
                                return;
                            }
                            let target = NavigationMenuHoverTarget::Open(hover_value.clone());
                            let generation = hover_context
                                .update(cx, |runtime| runtime.schedule_hover(target.clone()));
                            spawn_delayed_hover(
                                hover_context.clone(),
                                generation,
                                target,
                                delay,
                                window,
                                cx,
                            );
                        } else {
                            let (keep_open, open) = hover_context.update(cx, |runtime| {
                                if !runtime.another_trigger_hovered(&hover_value) {
                                    runtime.set_trigger_hovered(None);
                                }
                                (
                                    runtime.popup_hovered()
                                        || runtime.another_trigger_hovered(&hover_value),
                                    runtime.open_value(),
                                )
                            });
                            if keep_open {
                                return;
                            }
                            if !open {
                                hover_context.update(cx, |runtime| runtime.cancel_hover());
                                return;
                            }
                            let pointer = window.mouse_position();
                            let close_delay =
                                hover_context.read(cx, |_runtime, props| props.close_delay());
                            let generation = hover_context.update(cx, |runtime| {
                                runtime.arm_safe_polygon(pointer);
                                runtime.schedule_hover(NavigationMenuHoverTarget::Close)
                            });
                            spawn_delayed_hover(
                                hover_context.clone(),
                                generation,
                                NavigationMenuHoverTarget::Close,
                                close_delay,
                                window,
                                cx,
                            );
                        }
                    })
                    .children(children)
            })
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuTrigger<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context);
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuTrigger<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Accessible name for the trigger button; pass one when the visible
    /// caption is not plain accessible text (no id-reference labelling
    /// exists). Render the visible caption with `Text::new_inaccessible(...)`
    /// when set, to avoid double announcement.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn child(mut self, child: impl Into<NavigationMenuTriggerChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuTriggerChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuTriggerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }

    /// Internal wiring seam: the enclosing item hands the trigger its value,
    /// focus handle, roving entry index, and order.
    pub fn wired(
        mut self,
        value: T,
        focus_handle: FocusHandle,
        entry_index: usize,
        order: usize,
    ) -> Self {
        self.value = Some(value);
        self.focus_handle = Some(focus_handle);
        self.entry_index = entry_index;
        self.order = order;
        self
    }
}

fn now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
}

pub fn spawn_delayed_hover<T: Clone + Eq + 'static>(
    context: NavigationMenuContext<T>,
    generation: u64,
    target: NavigationMenuHoverTarget<T>,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_hover(generation, &target)
                });
                if !current {
                    return;
                }

                match target {
                    NavigationMenuHoverTarget::Open(value) => {
                        let changed = context.set_value(
                            Some(value),
                            NavigationMenuValueChangeReason::TriggerHover,
                            NavigationMenuValueChangeSource::Pointer,
                            window,
                            cx,
                        );
                        if changed {
                            context.update(cx, |runtime| runtime.note_hover_open(now()));
                        }
                    }
                    NavigationMenuHoverTarget::Close => {
                        let keep_for_popup = context.read(cx, |runtime, _| runtime.popup_hovered());
                        if keep_for_popup {
                            return;
                        }
                        context.update(cx, |runtime| runtime.disarm_safe_polygon());
                        context.close(
                            NavigationMenuValueChangeReason::TriggerHover,
                            NavigationMenuValueChangeSource::Pointer,
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
/// the verdict per the primitive's integration contract.
pub fn evaluate_safe_polygon_move<T: Clone + Eq + 'static>(
    context: &NavigationMenuContext<T>,
    pointer: gpui::Point<gpui::Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let verdict = context.update(cx, |runtime| runtime.evaluate_safe_polygon(pointer, now()));
    let Some(verdict) = verdict else {
        return;
    };
    match verdict {
        SafePolygonVerdict::Inside => {
            let generation = context.update(cx, |runtime| {
                runtime.schedule_hover(NavigationMenuHoverTarget::Close)
            });
            spawn_delayed_hover(
                context.clone(),
                generation,
                NavigationMenuHoverTarget::Close,
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
