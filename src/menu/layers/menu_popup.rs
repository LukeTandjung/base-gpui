use std::{
    rc::Rc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use gpui::{
    div, App, Div, ElementId, FocusHandle, InteractiveElement as _, IntoElement, KeyDownEvent,
    Orientation, ParentElement, RenderOnce, Role, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window,
};

use crate::{
    menu::{
        child_wiring::{part_focus_handle, MenuChildNode, MenuChildWiring},
        MenuActivateHighlighted, MenuArrowLeft, MenuArrowRight, MenuChildHoverDirective,
        MenuCloseAction, MenuContext, MenuItemKind, MenuMove, MenuMoveFirst, MenuMoveLast,
        MenuMoveNext, MenuMovePrevious, MenuOpenChangeReason, MenuOpenChangeSource, MenuParentKind,
        MenuPopupChild, MenuPopupStyleState, MenuSpaceActivate, MenuSubmenuLink, MENU_KEY_CONTEXT,
    },
    primitives::safe_polygon::SafePolygonVerdict,
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

/// Menu popup. A `final_focus` override is deferred, matching the Popover
/// focus-return audit; closing returns focus to the active trigger.
#[derive(IntoElement)]
pub struct MenuPopup<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<MenuPopupChild<P>>,
    context: Option<MenuContext<P>>,
    keep_mounted: bool,
    focus_handle: Option<FocusHandle>,
    style_with_state: Option<Rc<dyn Fn(MenuPopupStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuPopup<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-popup"),
            base: div(),
            children: Vec::new(),
            context: None,
            keep_mounted: false,
            focus_handle: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuPopup<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuPopup<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().into_any_element();
        };
        let state = context.read(cx, |runtime, _| {
            runtime.popup_state(Default::default(), Default::default(), self.keep_mounted)
        });
        if !state.mounted {
            return div().into_any_element();
        }

        let focus_handle = self
            .focus_handle
            .clone()
            .unwrap_or_else(|| part_focus_handle(&self.id, window, cx));
        let (loop_focus, close_parent_on_esc, parent_kind) = context.read(cx, |runtime, props| {
            (
                props.loop_focus(),
                props.close_parent_on_esc(),
                runtime.parent_kind(),
            )
        });
        let scroll_handle = context.read(cx, |runtime, _| runtime.popup_scroll_handle());

        let move_next = context.clone();
        let move_previous = context.clone();
        let move_first = context.clone();
        let move_last = context.clone();
        let activate = context.clone();
        let space = context.clone();
        let escape = context.clone();
        let arrow_right = context.clone();
        let arrow_left = context.clone();
        let mouse_move = context.clone();
        let typeahead = context.clone();
        let measure_context = context.clone();

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(self.id)
        // Base UI emits `role="menu"` with a vertical orientation; flip the
        // orientation when a horizontal menu variant lands.
        .role(Role::Menu)
        .aria_orientation(Orientation::Vertical)
        .track_focus(&focus_handle)
        .focusable()
        .key_context(MENU_KEY_CONTEXT)
        .track_scroll(&scroll_handle)
        .on_action(move |_: &MenuMoveNext, _window, cx| {
            move_next.update(cx, |runtime| {
                runtime.move_highlight(MenuMove::Next, loop_focus)
            });
        })
        .on_action(move |_: &MenuMovePrevious, _window, cx| {
            move_previous.update(cx, |runtime| {
                runtime.move_highlight(MenuMove::Previous, loop_focus)
            });
        })
        .on_action(move |_: &MenuMoveFirst, _window, cx| {
            move_first.update(cx, |runtime| {
                runtime.move_highlight(MenuMove::First, loop_focus)
            });
        })
        .on_action(move |_: &MenuMoveLast, _window, cx| {
            move_last.update(cx, |runtime| {
                runtime.move_highlight(MenuMove::Last, loop_focus)
            });
        })
        .on_action(move |_: &MenuActivateHighlighted, window, cx| {
            activate_highlighted(&activate, window, cx);
        })
        .on_action(move |_: &MenuSpaceActivate, window, cx| {
            let typing = space.read(cx, |runtime, _| runtime.typeahead_active(Instant::now()));
            if typing {
                space.update(cx, |runtime| {
                    runtime.apply_typeahead(" ", Instant::now());
                });
            } else {
                activate_highlighted(&space, window, cx);
            }
        })
        .on_action(move |_: &MenuCloseAction, window, cx| {
            escape.close(
                MenuOpenChangeReason::EscapeKey,
                MenuOpenChangeSource::Keyboard,
                window,
                cx,
            );
            if close_parent_on_esc {
                escape.close_ancestors(
                    MenuOpenChangeReason::EscapeKey,
                    MenuOpenChangeSource::Keyboard,
                    window,
                    cx,
                );
            }
        })
        .on_action(move |_: &MenuArrowRight, window, cx| {
            let consumed = match current_direction().is_rtl() {
                false => open_highlighted_submenu(&arrow_right, window, cx),
                true => close_own_submenu(&arrow_right, parent_kind, window, cx),
            };
            if !consumed {
                relay_to_menubar(&arrow_right, HorizontalArrowKey::Right, window, cx);
            }
        })
        .on_action(move |_: &MenuArrowLeft, window, cx| {
            let consumed = match current_direction().is_rtl() {
                false => close_own_submenu(&arrow_left, parent_kind, window, cx),
                true => open_highlighted_submenu(&arrow_left, window, cx),
            };
            if !consumed {
                relay_to_menubar(&arrow_left, HorizontalArrowKey::Left, window, cx);
            }
        })
        .on_key_down(move |event: &KeyDownEvent, _window, cx| {
            let Some(text) = typeahead_text(event) else {
                return;
            };
            if text == " " {
                return;
            }
            typeahead.update(cx, |runtime| {
                runtime.apply_typeahead(&text, Instant::now());
            });
        })
        .on_mouse_move(move |event, window, cx| {
            let (verdict, directive) = mouse_move.update(cx, |runtime| {
                runtime.note_pointer_moved();
                let verdict = runtime.evaluate_child_polygon(event.position, polygon_now());
                let directive = runtime.reconcile_child_hover();
                (verdict, directive)
            });
            if let Some((item_index, verdict)) = verdict {
                handle_polygon_verdict(&mouse_move, item_index, verdict, window, cx);
            }
            if let MenuChildHoverDirective::ScheduleClose {
                item_index,
                close_delay,
                generation,
            } = directive
            {
                spawn_delayed_child_close(
                    mouse_move.clone(),
                    generation,
                    item_index,
                    close_delay,
                    window,
                    cx,
                );
            }
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed =
                    measure_context.update(cx, |runtime| runtime.set_popup_bounds(bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base)
            .into_any_element()
    }
}

fn polygon_now() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
}

fn typeahead_text(event: &KeyDownEvent) -> Option<String> {
    if event.keystroke.modifiers.control
        || event.keystroke.modifiers.alt
        || event.keystroke.modifiers.platform
        || event.keystroke.modifiers.function
    {
        return None;
    }

    event
        .keystroke
        .key_char
        .as_ref()
        .or(Some(&event.keystroke.key))
        .filter(|text| text.chars().count() == 1)
        .filter(|text| text.chars().all(|ch| !ch.is_control()))
        .cloned()
}

fn activate_highlighted<P: Clone + 'static>(
    context: &MenuContext<P>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some((index, kind, disabled, _close_on_click, _activation)) =
        context.read(cx, |runtime, _| runtime.highlighted_activation())
    else {
        return;
    };
    if disabled {
        return;
    }
    if kind == MenuItemKind::SubmenuTrigger {
        open_submenu_link(context, index, window, cx);
        return;
    }
    context.activate_item(index, MenuOpenChangeSource::Keyboard, window, cx);
}

fn open_highlighted_submenu<P: Clone + 'static>(
    context: &MenuContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    let Some((index, kind, disabled, _, _)) =
        context.read(cx, |runtime, _| runtime.highlighted_activation())
    else {
        return false;
    };
    if disabled || kind != MenuItemKind::SubmenuTrigger {
        return false;
    }
    open_submenu_link(context, index, window, cx);
    true
}

/// Seam 8: perpendicular arrows the menu did not consume relay to the
/// menubar as a typed runtime command (no highlighted submenu trigger, and
/// no submenu close applied), moving the roving highlight and handing the
/// open menu off to the neighbor.
fn relay_to_menubar<P: Clone + 'static>(
    context: &MenuContext<P>,
    key: HorizontalArrowKey,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(link) = context.menubar_link_in_tree() else {
        return;
    };
    if !link.horizontal() {
        return;
    }
    let direction = match current_direction().horizontal_arrow(key) {
        HorizontalDirection::Next => MenuMove::Next,
        HorizontalDirection::Previous => MenuMove::Previous,
    };
    link.relay(direction, window, cx);
}

fn open_submenu_link<P: Clone + 'static>(
    context: &MenuContext<P>,
    index: usize,
    window: &mut Window,
    cx: &mut App,
) {
    let link: Option<MenuSubmenuLink> = context.read(cx, |runtime, _| runtime.submenu_link(index));
    if let Some(link) = link {
        link.open(window, cx);
    }
}

fn close_own_submenu<P: Clone + 'static>(
    context: &MenuContext<P>,
    parent_kind: MenuParentKind,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    if parent_kind != MenuParentKind::Submenu {
        return false;
    }
    context.close(
        MenuOpenChangeReason::ListNavigation,
        MenuOpenChangeSource::Keyboard,
        window,
        cx,
    );
    true
}

fn handle_polygon_verdict<P: Clone + 'static>(
    context: &MenuContext<P>,
    item_index: usize,
    verdict: SafePolygonVerdict,
    window: &mut Window,
    cx: &mut App,
) {
    match verdict {
        SafePolygonVerdict::Inside => {
            let generation = context.update(cx, |runtime| runtime.schedule_child_close(item_index));
            spawn_delayed_child_close(
                context.clone(),
                generation,
                item_index,
                Duration::from_millis(40),
                window,
                cx,
            );
        }
        SafePolygonVerdict::Outside => {
            context.update(cx, |runtime| {
                runtime.cancel_child_close();
                runtime.disarm_child_polygon();
            });
            let link = context.read(cx, |runtime, _| runtime.submenu_link(item_index));
            if let Some(link) = link {
                if link.is_open(cx) {
                    link.close(MenuOpenChangeReason::TriggerHover, window, cx);
                }
            }
        }
        SafePolygonVerdict::LandedPopup | SafePolygonVerdict::LandedTrigger => {
            context.update(cx, |runtime| runtime.cancel_child_close());
        }
    }
}

pub fn spawn_delayed_child_close<P: Clone + 'static>(
    context: MenuContext<P>,
    generation: u64,
    item_index: usize,
    delay: Duration,
    window: &mut Window,
    cx: &mut App,
) {
    window
        .spawn(cx, async move |cx| {
            cx.background_executor().timer(delay).await;
            cx.update(|window, cx| {
                let current = context.update(cx, |runtime| {
                    runtime.take_scheduled_child_close(generation, item_index)
                });
                if !current {
                    return;
                }
                let link = context.read(cx, |runtime, _| runtime.submenu_link(item_index));
                if let Some(link) = link {
                    if link.is_open(cx) {
                        link.close(MenuOpenChangeReason::TriggerHover, window, cx);
                    }
                }
            })
            .ok();
        })
        .detach();
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuPopup<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let scoped_id = wiring.scope_child_id(&self.id);
        let focus_handle = part_focus_handle(&scoped_id, window, cx);
        wiring.register_popup_focus_handle(focus_handle.clone());
        self.id = scoped_id;
        self.focus_handle = Some(focus_handle);
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_menu_child(wiring, context, window, cx))
            .collect();
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuPopup<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MenuPopupChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
