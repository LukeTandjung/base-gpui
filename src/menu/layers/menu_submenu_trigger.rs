use std::{rc::Rc, time::Duration};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, Div, ElementId,
    FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::{
    menu::{
        layers::menu_popup::spawn_delayed_child_close, MenuContext, MenuHoverTarget,
        MenuOpenChangeReason, MenuOpenChangeSource, MenuSide, MenuSubmenuTriggerStyleState,
    },
    primitives::safe_polygon::SafePolygonSide,
};

type MenuSubmenuTriggerStyle = Rc<dyn Fn(MenuSubmenuTriggerStyleState, Div) -> Div + 'static>;

/// Item of the parent menu and trigger of the child menu at once. Hover
/// intent consumes the safe-polygon primitive
/// (`base_gpui::primitives::safe_polygon`) armed on unhover, alongside
/// delayed open/close timers.
#[derive(IntoElement)]
pub struct MenuSubmenuTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    parent_context: Option<MenuContext<P>>,
    child_context: Option<MenuContext<P>>,
    label: Option<SharedString>,
    disabled: bool,
    open_on_hover: bool,
    delay: Duration,
    close_delay: Duration,
    index: Option<usize>,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<MenuSubmenuTriggerStyle>,
}

impl<P: Clone + 'static> Default for MenuSubmenuTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-submenu-trigger"),
            base: div(),
            children: Vec::new(),
            parent_context: None,
            child_context: None,
            label: None,
            disabled: false,
            open_on_hover: true,
            delay: Duration::from_millis(100),
            close_delay: Duration::ZERO,
            index: None,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for MenuSubmenuTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for MenuSubmenuTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuSubmenuTrigger<P> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(parent), Some(child), Some(index)) = (
            self.parent_context.clone(),
            self.child_context.clone(),
            self.index,
        ) else {
            return div().children(self.children).into_any_element();
        };
        let (state, tab_stop) = parent.read(cx, |runtime, props| {
            let mut state = runtime.submenu_trigger_state(self.index, self.disabled);
            state.disabled = state.disabled || props.disabled();
            (state, runtime.item_is_tab_stop(self.index))
        });
        let disabled = state.disabled;
        let open_on_hover = self.open_on_hover;
        let open_delay = self.delay;
        let close_delay = self.close_delay;

        let click_child = child.clone();
        let hover_parent = parent.clone();
        let hover_child = child.clone();
        let move_parent = parent.clone();
        let measure_parent = parent.clone();
        let measure_child = child.clone();
        let expand_child = child.clone();
        let collapse_child = child.clone();
        let focus_handle = self.focus_handle.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let mut item = base
            .id(self.id)
            // Item of the parent menu and trigger of the child menu at once.
            // `aria-haspopup`/`aria-controls` have no gpui builders
            // (documented gap); `aria_expanded` + the child popup's
            // `Role::Menu` carry the relationship.
            .role(Role::MenuItem)
            .aria_expanded(state.open)
            .when_some(self.label.clone(), |this, label| this.aria_label(label))
            .on_a11y_action(AccessibleAction::Expand, move |_data, window, cx| {
                if disabled {
                    return;
                }
                let open = expand_child.read(cx, |runtime, _| runtime.open_value());
                if open {
                    return;
                }
                expand_child.set_open(
                    true,
                    MenuOpenChangeReason::ImperativeAction,
                    MenuOpenChangeSource::Imperative,
                    window,
                    cx,
                );
            })
            .on_a11y_action(AccessibleAction::Collapse, move |_data, window, cx| {
                if disabled {
                    return;
                }
                let open = collapse_child.read(cx, |runtime, _| runtime.open_value());
                if !open {
                    return;
                }
                collapse_child.set_open(
                    false,
                    MenuOpenChangeReason::ImperativeAction,
                    MenuOpenChangeSource::Imperative,
                    window,
                    cx,
                );
            })
            .on_mouse_move(move |_event, _window, cx| {
                let should_highlight =
                    move_parent.read(cx, |_runtime, props| props.highlight_item_on_hover());
                if !should_highlight || disabled {
                    return;
                }
                move_parent.update(cx, |runtime| {
                    runtime.highlight_item_from_pointer(index);
                });
            })
            .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                cx.stop_propagation();
                if disabled {
                    return;
                }
                let open = click_child.read(cx, |runtime, _| runtime.open_value());
                // With `open_on_hover` mouse clicks never toggle-close.
                if open && open_on_hover {
                    return;
                }
                click_child.set_open(
                    !open,
                    MenuOpenChangeReason::TriggerPress,
                    MenuOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .on_hover(move |hovered, window, cx| {
                if disabled || !open_on_hover {
                    return;
                }
                if *hovered {
                    // Re-hovering the trigger cancels a pending child close.
                    hover_parent.update(cx, |runtime| {
                        runtime.cancel_child_close();
                        runtime.disarm_child_polygon();
                    });
                    let child_open = hover_child.read(cx, |runtime, _| runtime.open_value());
                    let allow = hover_parent.read(cx, |runtime, _| runtime.allow_mouse_enter());
                    if child_open || !allow {
                        return;
                    }
                    if open_delay.is_zero() {
                        hover_child.set_open(
                            true,
                            MenuOpenChangeReason::TriggerHover,
                            MenuOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    } else {
                        let generation = hover_child
                            .update(cx, |runtime| runtime.schedule_hover(MenuHoverTarget::Open));
                        spawn_delayed_hover_open(
                            hover_child.clone(),
                            generation,
                            open_delay,
                            window,
                            cx,
                        );
                    }
                } else {
                    let child_open = hover_child.read(cx, |runtime, _| runtime.open_value());
                    if !child_open {
                        hover_child.update(cx, |runtime| runtime.cancel_hover());
                        return;
                    }
                    // Arm the safe polygon toward the child popup and schedule
                    // a generation-counted delayed close.
                    let exit_point = window.mouse_position();
                    let trigger_bounds =
                        hover_parent.read(cx, |runtime, _| runtime.item_bounds(index));
                    let (popup_bounds, side) = hover_child.read(cx, |runtime, _| {
                        (runtime.popup_bounds(), runtime.effective_side())
                    });
                    if let (Some(trigger_bounds), Some(popup_bounds)) =
                        (trigger_bounds, popup_bounds)
                    {
                        let side = match side {
                            Some(MenuSide::Left | MenuSide::InlineStart) => SafePolygonSide::Left,
                            Some(MenuSide::Top) => SafePolygonSide::Top,
                            Some(MenuSide::Bottom) => SafePolygonSide::Bottom,
                            _ => SafePolygonSide::Right,
                        };
                        hover_parent.update(cx, |runtime| {
                            runtime.arm_child_polygon(
                                index,
                                exit_point,
                                trigger_bounds,
                                popup_bounds,
                                side,
                            );
                        });
                    }
                    let generation =
                        hover_parent.update(cx, |runtime| runtime.schedule_child_close(index));
                    spawn_delayed_child_close(
                        hover_parent.clone(),
                        generation,
                        index,
                        close_delay.max(Duration::from_millis(40)),
                        window,
                        cx,
                    );
                }
            })
            .children(self.children);

        if let Some(focus_handle) = focus_handle {
            item = item
                .track_focus(
                    &focus_handle
                        .tab_stop(tab_stop && !disabled)
                        .tab_index(if tab_stop && !disabled { 0 } else { -1 }),
                )
                .focusable();
        }

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_parent
                    .update(cx, |runtime| runtime.set_item_bounds(index, bounds))
                    | measure_child.update(cx, |runtime| runtime.set_trigger_bounds(bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(item)
            .into_any_element()
    }
}

fn spawn_delayed_hover_open<P: Clone + 'static>(
    child: MenuContext<P>,
    generation: u64,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = child.update(cx, |runtime| {
                    runtime.take_scheduled_hover(generation, MenuHoverTarget::Open)
                });
                if !current {
                    return;
                }
                child.set_open(
                    true,
                    MenuOpenChangeReason::TriggerHover,
                    MenuOpenChangeSource::Pointer,
                    window,
                    cx,
                );
            })
            .ok();
        })
        .detach();
}

impl<P: Clone + 'static> MenuSubmenuTrigger<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn label_value(&self) -> Option<SharedString> {
        self.label.clone()
    }

    pub fn disabled_value(&self) -> bool {
        self.disabled
    }

    pub fn open_on_hover_value(&self) -> bool {
        self.open_on_hover
    }

    pub fn delay_value(&self) -> Duration {
        self.delay
    }

    pub fn close_delay_value(&self) -> Duration {
        self.close_delay
    }

    pub fn wired(mut self, index: usize, focus_handle: FocusHandle) -> Self {
        self.index = Some(index);
        self.focus_handle = Some(focus_handle);
        self
    }

    pub fn with_contexts(
        mut self,
        parent: MenuContext<P>,
        child: MenuContext<P>,
        item_index: usize,
    ) -> Self {
        self.parent_context = Some(parent);
        self.child_context = Some(child);
        self.index = Some(item_index);
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuSubmenuTriggerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
