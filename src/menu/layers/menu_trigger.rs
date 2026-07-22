use std::{rc::Rc, time::Duration};

use gpui::{
    div, prelude::FluentBuilder as _, AccessibleAction, AnyElement, App, Div, ElementId,
    FocusHandle, InteractiveElement as _, IntoElement, MouseButton, ParentElement, RenderOnce,
    Role, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::{
    menu::{
        child_wiring::{part_focus_handle, MenuChildNode, MenuChildWiring},
        MenuActivateHighlighted, MenuArrowLeft, MenuArrowRight, MenuContext, MenuHoverTarget,
        MenuMove, MenuMoveFirst, MenuMoveLast, MenuMoveNext, MenuMovePrevious,
        MenuOpenChangeReason, MenuOpenChangeSource, MenuSpaceActivate, MenuTriggerMetadata,
        MenuTriggerStyleState, MENU_KEY_CONTEXT,
    },
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

type MenuTriggerStyle<P> = Rc<dyn Fn(MenuTriggerStyleState<P>, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct MenuTrigger<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<AnyElement>,
    context: Option<MenuContext<P>>,
    focus_handle: Option<FocusHandle>,
    disabled: bool,
    payload: Option<P>,
    open_on_hover: bool,
    delay: Duration,
    close_delay: Duration,
    aria_label: Option<SharedString>,
    style_with_state: Option<MenuTriggerStyle<P>>,
}

impl<P: Clone + 'static> Default for MenuTrigger<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-trigger"),
            base: div(),
            children: Vec::new(),
            context: None,
            focus_handle: None,
            disabled: false,
            payload: None,
            open_on_hover: false,
            delay: Duration::from_millis(100),
            close_delay: Duration::ZERO,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> ParentElement for MenuTrigger<P> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl<P: Clone + 'static> Styled for MenuTrigger<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuTrigger<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().children(self.children);
        };
        let focus_handle = self
            .focus_handle
            .clone()
            .unwrap_or_else(|| part_focus_handle(&self.id, window, cx));

        let mut state = context.read(cx, |runtime, props| {
            let mut state = runtime.trigger_state(self.disabled, self.payload.is_some());
            state.disabled = state.disabled || props.disabled();
            state
        });
        state.focused = focus_handle.is_focused(window);
        let disabled = state.disabled;
        let was_open = state.open;

        let menubar = context.menubar_link().cloned();
        let click_context = context.clone();
        let toggle_context = context.clone();
        let open_down_context = context.clone();
        let open_up_context = context.clone();
        let hover_context = context.clone();
        let measure_context = context.clone();
        let expand_context = context.clone();
        let collapse_context = context.clone();
        let open_on_hover = self.open_on_hover;
        let hover_open_delay = self.delay;
        let hover_close_delay = self.close_delay;
        let click_focus_handle = focus_handle.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed =
                    measure_context.update(cx, |runtime| runtime.set_trigger_bounds(bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(
                base.id(self.id)
                    // Base UI renders a button trigger; under a menubar the
                    // trigger is itself an item of the bar. `aria-haspopup`
                    // and `aria-controls` have no gpui builders (documented
                    // gap); `aria_expanded` + the popup's `Role::Menu` carry
                    // the relationship.
                    .role(match &menubar {
                        Some(_) => Role::MenuItem,
                        None => Role::Button,
                    })
                    .aria_expanded(was_open)
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    .on_a11y_action(AccessibleAction::Expand, move |_data, window, cx| {
                        if disabled || was_open {
                            return;
                        }
                        expand_context.set_open(
                            true,
                            MenuOpenChangeReason::ImperativeAction,
                            MenuOpenChangeSource::Imperative,
                            window,
                            cx,
                        );
                    })
                    .on_a11y_action(AccessibleAction::Collapse, move |_data, window, cx| {
                        if disabled || !was_open {
                            return;
                        }
                        collapse_context.set_open(
                            false,
                            MenuOpenChangeReason::ImperativeAction,
                            MenuOpenChangeSource::Imperative,
                            window,
                            cx,
                        );
                    })
                    .track_focus(&{
                        // Menubar triggers rove: exactly one is the tab stop
                        // and disabled triggers stay keyboard-reachable
                        // (seam 2).
                        let (tab_stop, tab_index) = match &menubar {
                            Some(link) => match link.is_tab_stop(link.index(), cx) {
                                true => (true, 0),
                                false => (false, -1),
                            },
                            None => (!disabled, if disabled { -1 } else { 0 }),
                        };
                        focus_handle.tab_stop(tab_stop).tab_index(tab_index)
                    })
                    .focusable()
                    .key_context(MENU_KEY_CONTEXT)
                    .on_mouse_down(MouseButton::Left, {
                        let menubar = menubar.clone();
                        move |_event, window, cx| {
                            cx.stop_propagation();
                            if disabled {
                                return;
                            }
                            click_focus_handle.focus(window, cx);
                            if let Some(link) = &menubar {
                                // Seam 5: press-down opens; the paired click
                                // handler closes the already-open menu. The
                                // mouse-up click of the opening press itself
                                // is marked so it does not immediately close
                                // the menu it just opened.
                                click_context.update(cx, |runtime| {
                                    runtime.set_opened_by_current_press(!was_open)
                                });
                                if !was_open {
                                    link.highlight(cx);
                                    click_context.set_open(
                                        true,
                                        MenuOpenChangeReason::TriggerPress,
                                        MenuOpenChangeSource::Pointer,
                                        window,
                                        cx,
                                    );
                                }
                                return;
                            }
                            click_context.set_open(
                                !was_open,
                                MenuOpenChangeReason::TriggerPress,
                                MenuOpenChangeSource::Pointer,
                                window,
                                cx,
                            );
                        }
                    })
                    .on_click({
                        let menubar = menubar.clone();
                        let context = context.clone();
                        move |_event, window, cx| {
                            if disabled || menubar.is_none() || !was_open {
                                return;
                            }
                            let opened_by_this_press = context
                                .update(cx, |runtime| runtime.take_opened_by_current_press());
                            if opened_by_this_press {
                                return;
                            }
                            context.set_open(
                                false,
                                MenuOpenChangeReason::TriggerPress,
                                MenuOpenChangeSource::Pointer,
                                window,
                                cx,
                            );
                        }
                    })
                    .on_action(move |_: &MenuActivateHighlighted, window, cx| {
                        if disabled {
                            return;
                        }
                        toggle_context.set_open(
                            !was_open,
                            MenuOpenChangeReason::TriggerPress,
                            MenuOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    })
                    .on_action(move |_: &MenuSpaceActivate, window, cx| {
                        if disabled {
                            return;
                        }
                        open_down_context.set_open(
                            !was_open,
                            MenuOpenChangeReason::TriggerPress,
                            MenuOpenChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    })
                    .on_action({
                        let context = context.clone();
                        let menubar = menubar.clone();
                        move |_: &MenuMoveNext, window, cx| {
                            if let Some(link) = vertical_menubar(&menubar) {
                                link.relay(MenuMove::Next, window, cx);
                                return;
                            }
                            if disabled {
                                return;
                            }
                            open_from_trigger(&context, MenuMove::First, window, cx);
                        }
                    })
                    .on_action({
                        let menubar = menubar.clone();
                        move |_: &MenuMovePrevious, window, cx| {
                            if let Some(link) = vertical_menubar(&menubar) {
                                link.relay(MenuMove::Previous, window, cx);
                                return;
                            }
                            if disabled {
                                return;
                            }
                            open_from_trigger(&open_up_context, MenuMove::Last, window, cx);
                        }
                    })
                    .on_action({
                        let menubar = menubar.clone();
                        let context = context.clone();
                        move |_: &MenuArrowRight, window, cx| {
                            let Some(link) = &menubar else {
                                return;
                            };
                            match link.horizontal() {
                                true => link.relay(
                                    horizontal_menubar_move(HorizontalArrowKey::Right),
                                    window,
                                    cx,
                                ),
                                false => {
                                    if !disabled {
                                        open_from_trigger(&context, MenuMove::First, window, cx);
                                    }
                                }
                            }
                        }
                    })
                    .on_action({
                        let menubar = menubar.clone();
                        move |_: &MenuArrowLeft, window, cx| {
                            let Some(link) = &menubar else {
                                return;
                            };
                            if link.horizontal() {
                                link.relay(
                                    horizontal_menubar_move(HorizontalArrowKey::Left),
                                    window,
                                    cx,
                                );
                            }
                        }
                    })
                    .on_action({
                        let menubar = menubar.clone();
                        move |_: &MenuMoveFirst, window, cx| {
                            if let Some(link) = &menubar {
                                link.relay(MenuMove::First, window, cx);
                            }
                        }
                    })
                    .on_action({
                        let menubar = menubar.clone();
                        move |_: &MenuMoveLast, window, cx| {
                            if let Some(link) = &menubar {
                                link.relay(MenuMove::Last, window, cx);
                            }
                        }
                    })
                    .on_hover(move |hovered, window, cx| {
                        if let Some(link) = &menubar {
                            // Seam 3: hover opens only while a sibling menu
                            // is already open; hovering the open trigger is
                            // a no-op; the roving highlight follows hover.
                            if *hovered && link.has_submenu_open(cx) {
                                link.highlight(cx);
                                if !was_open && !disabled {
                                    hover_context.set_open(
                                        true,
                                        MenuOpenChangeReason::TriggerHover,
                                        MenuOpenChangeSource::Pointer,
                                        window,
                                        cx,
                                    );
                                }
                            }
                            return;
                        }
                        if disabled || !open_on_hover {
                            return;
                        }
                        if *hovered {
                            if hover_open_delay.is_zero() {
                                hover_context.update(cx, |runtime| runtime.cancel_hover());
                                hover_context.set_open(
                                    true,
                                    MenuOpenChangeReason::TriggerHover,
                                    MenuOpenChangeSource::Pointer,
                                    window,
                                    cx,
                                );
                            } else {
                                let generation = hover_context.update(cx, |runtime| {
                                    runtime.schedule_hover(MenuHoverTarget::Open)
                                });
                                spawn_delayed_hover(
                                    hover_context.clone(),
                                    generation,
                                    MenuHoverTarget::Open,
                                    hover_open_delay,
                                    window,
                                    cx,
                                );
                            }
                        } else if hover_close_delay.is_zero() {
                            hover_context.update(cx, |runtime| runtime.cancel_hover());
                        } else {
                            let generation = hover_context.update(cx, |runtime| {
                                runtime.schedule_hover(MenuHoverTarget::Close)
                            });
                            spawn_delayed_hover(
                                hover_context.clone(),
                                generation,
                                MenuHoverTarget::Close,
                                hover_close_delay,
                                window,
                                cx,
                            );
                        }
                    })
                    .children(self.children),
            )
    }
}

fn vertical_menubar(
    menubar: &Option<crate::menu::MenuMenubarLink>,
) -> Option<&crate::menu::MenuMenubarLink> {
    menubar.as_ref().filter(|link| !link.horizontal())
}

/// Maps a physical horizontal arrow to the menubar roving direction,
/// flipping in RTL.
fn horizontal_menubar_move(key: HorizontalArrowKey) -> MenuMove {
    match current_direction().horizontal_arrow(key) {
        HorizontalDirection::Next => MenuMove::Next,
        HorizontalDirection::Previous => MenuMove::Previous,
    }
}

fn open_from_trigger<P: Clone + 'static>(
    context: &MenuContext<P>,
    first_move: MenuMove,
    window: &mut Window,
    cx: &mut App,
) {
    let open = context.read(cx, |runtime, _| runtime.open_value());
    if open {
        return;
    }
    // Reserved: suppressed for the future Context Menu parent kind.
    if context.set_open(
        true,
        MenuOpenChangeReason::ListNavigation,
        MenuOpenChangeSource::Keyboard,
        window,
        cx,
    ) {
        let loop_focus = context.read(cx, |_, props| props.loop_focus());
        context.update(cx, |runtime| runtime.move_highlight(first_move, loop_focus));
    }
}

fn spawn_delayed_hover<P: Clone + 'static>(
    context: MenuContext<P>,
    generation: u64,
    target: MenuHoverTarget,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_hover(generation, target)
                });
                if !current {
                    return;
                }
                match target {
                    MenuHoverTarget::Open => {
                        context.set_open(
                            true,
                            MenuOpenChangeReason::TriggerHover,
                            MenuOpenChangeSource::Pointer,
                            window,
                            cx,
                        );
                    }
                    MenuHoverTarget::Close => {
                        context.close(
                            MenuOpenChangeReason::TriggerHover,
                            MenuOpenChangeSource::Pointer,
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

impl<P: Clone + 'static> MenuChildNode<P> for MenuTrigger<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = part_focus_handle(&scoped_id, window, cx);
        // Seam 12: menubar-wide disabled combines into every child trigger.
        if let Some(link) = context.menubar_link() {
            self.disabled = self.disabled || link.disabled();
        }
        wiring.register_trigger(MenuTriggerMetadata::new(
            scoped_id.clone(),
            self.disabled,
            self.open_on_hover,
            self.delay,
            self.close_delay,
            self.payload.clone(),
            Some(focus_handle.clone()),
        ));
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuTrigger<P> {
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

    /// Accessible label announced for the trigger. When set, render the
    /// visible trigger text as `Text::new_inaccessible(...)` so the label is
    /// not announced twice.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuTriggerStyleState<P>, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
