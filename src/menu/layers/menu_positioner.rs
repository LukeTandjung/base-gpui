use std::rc::Rc;

use std::time::Instant;

use gpui::{
    anchored, deferred, div, point, prelude::FluentBuilder as _, px, Anchor, App, Bounds, Div,
    InteractiveElement as _, IntoElement, MouseButton, ParentElement, Pixels, Point, RenderOnce,
    Size, StyleRefinement, Styled, Window,
};

use crate::{
    menu::{
        child_wiring::{MenuChildNode, MenuChildWiring},
        scoped_menu_id, MenuAlign, MenuContext, MenuContextMenuMouseUp, MenuOpenChangeReason,
        MenuOpenChangeSource, MenuParentKind, MenuPositionerChild, MenuPositionerStyleState,
        MenuSide,
    },
    utils::{
        direction::{current_direction, TextDirection},
        modal_backdrop, OverlayDismissHandler,
    },
};

#[derive(IntoElement)]
pub struct MenuPositioner<P: Clone + 'static = ()> {
    base: Div,
    children: Vec<MenuPositionerChild<P>>,
    context: Option<MenuContext<P>>,
    side: Option<MenuSide>,
    align: Option<MenuAlign>,
    side_offset: Option<Pixels>,
    align_offset: Option<Pixels>,
    collision_padding: Pixels,
    keep_mounted: bool,
    style_with_state: Option<Rc<dyn Fn(MenuPositionerStyleState, Div) -> Div + 'static>>,
}

impl<P: Clone + 'static> Default for MenuPositioner<P> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            side: None,
            align: None,
            side_offset: None,
            align_offset: None,
            collision_padding: px(8.0),
            keep_mounted: false,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuPositioner<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuPositioner<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let available_size = window.viewport_size();
        context.update(cx, |runtime| runtime.set_available_size(available_size));
        let (parent_kind, opened_by_hover, modal_props) = context.read(cx, |runtime, props| {
            (
                runtime.parent_kind(),
                runtime.opened_by_hover(),
                props.modal(),
            )
        });
        let menubar = context.menubar_link().cloned();
        // Parent-kind seams: side/align defaults branch here (seam 9 for
        // menubars; Context Menu defaults are implemented by its own issue).
        let (default_side, default_align) = match parent_kind {
            MenuParentKind::Submenu => (MenuSide::InlineEnd, MenuAlign::Start),
            MenuParentKind::Menubar => match menubar
                .as_ref()
                .map(|link| link.horizontal())
                .unwrap_or(true)
            {
                true => (MenuSide::Bottom, MenuAlign::Start),
                false => (MenuSide::InlineEnd, MenuAlign::Start),
            },
            // Context menus anchor their popup corner at/under the cursor:
            // align start with a small negative side inset.
            MenuParentKind::ContextMenu => (MenuSide::Bottom, MenuAlign::Start),
            MenuParentKind::None => (MenuSide::Bottom, MenuAlign::Center),
        };
        let side = self.side.unwrap_or(default_side);
        let align = self.align.unwrap_or(default_align);
        // Base UI Context Menu offset defaults apply only when no explicit
        // side is set and align is not centered; explicit props override.
        let context_menu_offset_defaults = parent_kind == MenuParentKind::ContextMenu
            && self.side.is_none()
            && align != MenuAlign::Center;
        let side_offset = self
            .side_offset
            .unwrap_or(match context_menu_offset_defaults {
                true => px(-5.0),
                false => px(0.0),
            });
        let align_offset = self
            .align_offset
            .unwrap_or(match context_menu_offset_defaults {
                true => px(2.0),
                false => px(0.0),
            });
        let mut state = context.read(cx, |runtime, _| {
            runtime.positioner_state(side, align, self.keep_mounted)
        });
        if !state.mounted {
            return div();
        }

        let resolved_preferred_side = resolve_logical_side(side, current_direction());
        let effective_side = resolve_collision_side(
            resolved_preferred_side,
            state.anchor_bounds,
            state.popup_bounds,
            state.available_size,
            self.collision_padding,
        );
        state.side = effective_side;
        let placement_changed = context.update(cx, |runtime| {
            runtime.set_effective_placement(effective_side, align)
        });
        if placement_changed {
            window.request_animation_frame();
        }
        let position = state
            .anchor_bounds
            .map(|bounds| resolved_position(effective_side, align, bounds));
        let anchor = resolved_anchor(effective_side, align);
        let offset = resolved_offset(effective_side, side_offset, align_offset);
        let outside_context = context.clone();
        let modal_context = context.clone();
        let dismiss_handler: OverlayDismissHandler = Rc::new(move |window, cx| {
            modal_context.close(
                MenuOpenChangeReason::OutsidePress,
                MenuOpenChangeSource::Pointer,
                window,
                cx,
            );
        });
        let measure_context = context.clone();
        // Modal applies only to root menus (never submenus; Context Menu is
        // implemented by its own issue) and is skipped for hover opens.
        // Seam 10: a modal menubar renders the backdrop whenever a child
        // menu is open — hover opens included — with the cutout spanning the
        // whole menubar row so sibling triggers stay interactive.
        let menubar_modal = menubar.as_ref().map(|link| link.modal()).unwrap_or(false);
        let modal = match parent_kind {
            MenuParentKind::Menubar => menubar_modal,
            MenuParentKind::None => modal_props && !opened_by_hover,
            // Context menus are unconditionally modal with no opt-out.
            MenuParentKind::ContextMenu => true,
            _ => false,
        };
        let trigger_bounds = match parent_kind {
            // No button trigger to keep clickable: the modal backdrop has no
            // cutout under a context-menu parent.
            MenuParentKind::ContextMenu => None,
            _ => match &menubar {
                Some(link) => link.menubar_bounds(cx).or(state.anchor_bounds),
                None => state.anchor_bounds,
            },
        };
        let positioner_id = scoped_menu_id(&context.root_id(), "menu-positioner");
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(positioner_id)
        .on_mouse_down_out(move |event, window, cx| {
            // Outside-press tests the union of the whole open tree: a press
            // inside any open descendant popup never dismisses ancestors.
            if outside_context.press_inside_tree(event.position, cx) {
                return;
            }
            outside_context.close(
                MenuOpenChangeReason::OutsidePress,
                MenuOpenChangeSource::Pointer,
                window,
                cx,
            );
        })
        .when(parent_kind == MenuParentKind::ContextMenu, |base| {
            // Context Menu mouseup grace: a window mouse-up before the grace
            // deadline is the tail of the opening right-click and is inert;
            // afterwards a mouse-up outside the open tree cancels the open.
            let left_context = context.clone();
            let right_context = context.clone();
            base.on_mouse_up_out(MouseButton::Left, move |event, window, cx| {
                close_context_menu_on_outside_mouse_up(&left_context, event.position, window, cx);
            })
            .on_mouse_up_out(MouseButton::Right, move |event, window, cx| {
                close_context_menu_on_outside_mouse_up(&right_context, event.position, window, cx);
            })
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        );

        if !context.read(cx, |runtime, _| runtime.open_value()) {
            return div().child(base);
        }

        let mut anchored = anchored()
            .anchor(anchor)
            .offset(offset)
            .snap_to_window_with_margin(self.collision_padding);
        if let Some(position) = position {
            anchored = anchored.position(position);
        }

        let measured = div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_popup_bounds(bounds) | runtime.set_available_size(available_size)
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base);
        let anchored = anchored.child(measured);

        match modal {
            // The modal backdrop cuts a hole for the active trigger so the
            // trigger stays clickable; the popup renders above the backdrop.
            true => div().child(
                deferred(
                    div()
                        .child(modal_backdrop(
                            available_size,
                            trigger_bounds,
                            dismiss_handler,
                        ))
                        .child(anchored),
                )
                .priority(1),
            ),
            false => div().child(deferred(anchored).priority(1)),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuPositioner<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| child.wire_menu_child(wiring, context, window, cx))
            .collect();
        self.context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuPositioner<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<MenuPositionerChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuPositionerChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: MenuSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn align(mut self, align: MenuAlign) -> Self {
        self.align = Some(align);
        self
    }

    pub fn side_offset(mut self, side_offset: Pixels) -> Self {
        self.side_offset = Some(side_offset);
        self
    }

    pub fn align_offset(mut self, align_offset: Pixels) -> Self {
        self.align_offset = Some(align_offset);
        self
    }

    pub fn collision_padding(mut self, collision_padding: Pixels) -> Self {
        self.collision_padding = collision_padding;
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn keep_mounted_from_portal(mut self) -> Self {
        self.keep_mounted = true;
        self.children = self
            .children
            .into_iter()
            .map(MenuPositionerChild::keep_mounted_from_portal)
            .collect();
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuPositionerStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

/// Applies the Context Menu mouseup rules to a mouse-up observed outside the
/// positioner: inert during the grace window or within ±1px of the initial
/// cursor point; otherwise a mouse-up outside the whole open tree closes with
/// `CancelOpen`.
fn close_context_menu_on_outside_mouse_up<P: Clone + 'static>(
    context: &MenuContext<P>,
    position: Point<Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    let inside_tree = context.press_inside_tree(position, cx);
    let outcome = context.update(cx, |runtime| {
        runtime.context_menu_mouse_up(position, inside_tree, Instant::now())
    });
    if outcome == MenuContextMenuMouseUp::CloseCancelOpen {
        context.close(
            MenuOpenChangeReason::CancelOpen,
            MenuOpenChangeSource::Pointer,
            window,
            cx,
        );
    }
}

fn resolve_logical_side(side: MenuSide, direction: TextDirection) -> MenuSide {
    match (side, direction) {
        (MenuSide::InlineStart, TextDirection::Ltr) => MenuSide::Left,
        (MenuSide::InlineStart, TextDirection::Rtl) => MenuSide::Right,
        (MenuSide::InlineEnd, TextDirection::Ltr) => MenuSide::Right,
        (MenuSide::InlineEnd, TextDirection::Rtl) => MenuSide::Left,
        _ => side,
    }
}

fn resolve_collision_side(
    side: MenuSide,
    trigger_bounds: Option<Bounds<Pixels>>,
    popup_bounds: Option<Bounds<Pixels>>,
    available_size: Option<Size<Pixels>>,
    collision_padding: Pixels,
) -> MenuSide {
    let Some(trigger_bounds) = trigger_bounds else {
        return side;
    };
    let Some(popup_bounds) = popup_bounds else {
        return side;
    };
    let Some(available_size) = available_size else {
        return side;
    };

    let top_space = trigger_bounds.top() - collision_padding;
    let bottom_space = available_size.height - collision_padding - trigger_bounds.bottom();
    let left_space = trigger_bounds.left() - collision_padding;
    let right_space = available_size.width - collision_padding - trigger_bounds.right();

    match side {
        MenuSide::Bottom => {
            if popup_bounds.size.height <= bottom_space {
                MenuSide::Bottom
            } else if top_space > bottom_space {
                MenuSide::Top
            } else {
                MenuSide::Bottom
            }
        }
        MenuSide::Top => {
            if popup_bounds.size.height <= top_space {
                MenuSide::Top
            } else if bottom_space > top_space {
                MenuSide::Bottom
            } else {
                MenuSide::Top
            }
        }
        MenuSide::Right => {
            if popup_bounds.size.width <= right_space {
                MenuSide::Right
            } else if left_space > right_space {
                MenuSide::Left
            } else {
                MenuSide::Right
            }
        }
        MenuSide::Left => {
            if popup_bounds.size.width <= left_space {
                MenuSide::Left
            } else if right_space > left_space {
                MenuSide::Right
            } else {
                MenuSide::Left
            }
        }
        MenuSide::InlineStart | MenuSide::InlineEnd => side,
    }
}

fn resolved_anchor(side: MenuSide, align: MenuAlign) -> Anchor {
    match (side, align) {
        (MenuSide::Bottom, MenuAlign::Start) => Anchor::TopLeft,
        (MenuSide::Bottom, MenuAlign::Center) => Anchor::TopCenter,
        (MenuSide::Bottom, MenuAlign::End) => Anchor::TopRight,
        (MenuSide::Top, MenuAlign::Start) => Anchor::BottomLeft,
        (MenuSide::Top, MenuAlign::Center) => Anchor::BottomCenter,
        (MenuSide::Top, MenuAlign::End) => Anchor::BottomRight,
        (MenuSide::Left, MenuAlign::Start) => Anchor::TopRight,
        (MenuSide::Left, _) => Anchor::RightCenter,
        (MenuSide::Right, MenuAlign::Start) => Anchor::TopLeft,
        (MenuSide::Right, _) => Anchor::LeftCenter,
        (MenuSide::InlineStart | MenuSide::InlineEnd, _) => Anchor::TopCenter,
    }
}

fn resolved_position(side: MenuSide, align: MenuAlign, bounds: Bounds<Pixels>) -> Point<Pixels> {
    match (side, align) {
        (MenuSide::Bottom, MenuAlign::Start) => point(bounds.left(), bounds.bottom()),
        (MenuSide::Bottom, MenuAlign::Center) => bounds.bottom_center(),
        (MenuSide::Bottom, MenuAlign::End) => point(bounds.right(), bounds.bottom()),
        (MenuSide::Top, MenuAlign::Start) => bounds.origin,
        (MenuSide::Top, MenuAlign::Center) => bounds.top_center(),
        (MenuSide::Top, MenuAlign::End) => bounds.top_right(),
        (MenuSide::Left, MenuAlign::Start) => point(bounds.left(), bounds.top()),
        (MenuSide::Left, _) => point(bounds.left(), bounds.center().y),
        (MenuSide::Right, MenuAlign::Start) => point(bounds.right(), bounds.top()),
        (MenuSide::Right, _) => point(bounds.right(), bounds.center().y),
        (MenuSide::InlineStart | MenuSide::InlineEnd, _) => bounds.bottom_center(),
    }
}

fn resolved_offset(side: MenuSide, side_offset: Pixels, align_offset: Pixels) -> Point<Pixels> {
    match side {
        MenuSide::Bottom => point(align_offset, side_offset),
        MenuSide::Top => point(align_offset, -side_offset),
        MenuSide::Left => point(-side_offset, align_offset),
        MenuSide::Right => point(side_offset, align_offset),
        MenuSide::InlineStart | MenuSide::InlineEnd => point(align_offset, side_offset),
    }
}
