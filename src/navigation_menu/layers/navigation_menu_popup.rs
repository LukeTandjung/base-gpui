use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, ParentElement, RenderOnce, Role,
    SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::navigation_menu::{
    child_wiring::{scoped_part_id, NavigationMenuChildNode, NavigationMenuChildWiring},
    layers::navigation_menu_trigger::{evaluate_safe_polygon_move, spawn_delayed_hover},
    NavigationMenuAlign, NavigationMenuBoundsKind, NavigationMenuCloseAction,
    NavigationMenuContext, NavigationMenuHoverTarget, NavigationMenuPopupChild,
    NavigationMenuPopupStyleState, NavigationMenuSide, NavigationMenuValueChangeReason,
    NavigationMenuValueChangeSource, NAVIGATION_MENU_KEY_CONTEXT,
};

type NavigationMenuPopupStyle = Rc<dyn Fn(NavigationMenuPopupStyleState, Div) -> Div + 'static>;

/// The single shared popup surface: one element serving every trigger,
/// retargeted (never re-created) when the active value changes. Hovering it
/// keeps the menu open; unhovering schedules the close delay.
///
/// Accessibility: `Role::Group` with the same `.aria_label(...)` text as the
/// root. This diverges from Base UI's `<nav>` deliberately — a second
/// `Navigation` landmark for one menu is noise in AccessKit. Retargeting the
/// popup produces no live announcement (no gpui live-region API; omitted,
/// blocked pending gpui upstream).
#[derive(IntoElement)]
pub struct NavigationMenuPopup<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuPopupChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    side: NavigationMenuSide,
    align: NavigationMenuAlign,
    keep_mounted: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<NavigationMenuPopupStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuPopup<T> {
    fn default() -> Self {
        Self {
            base: div().relative(),
            children: Vec::new(),
            context: None,
            side: NavigationMenuSide::Bottom,
            align: NavigationMenuAlign::Center,
            keep_mounted: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuPopup<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuPopup<T> {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div();
        };
        let state = context.read(cx, |runtime, _| {
            runtime.popup_state(self.side, self.align, self.keep_mounted)
        });
        if !state.mounted {
            return div();
        }

        let close_context = context.clone();
        let hover_context = context.clone();
        let move_context = context.clone();
        let measure_context = context.clone();

        let mut base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        }
        .id(scoped_part_id(&context.root_id(), "navigation-menu-popup"))
        .role(Role::Group);
        if let Some(aria_label) = self.aria_label.clone() {
            base = base.aria_label(aria_label);
        }

        if state.open {
            base = base
                .key_context(NAVIGATION_MENU_KEY_CONTEXT)
                .on_action(move |_: &NavigationMenuCloseAction, window, cx| {
                    close_context.close(
                        NavigationMenuValueChangeReason::EscapeKey,
                        NavigationMenuValueChangeSource::Keyboard,
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
                            runtime.set_popup_hovered(true);
                            runtime.cancel_hover();
                            runtime.disarm_safe_polygon();
                        });
                    } else {
                        hover_context.update(cx, |runtime| {
                            runtime.set_popup_hovered(false);
                        });
                        let close_delay =
                            hover_context.read(cx, |_runtime, props| props.close_delay());
                        let generation = hover_context.update(cx, |runtime| {
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
                });
        } else {
            base = base.opacity(0.0).invisible();
        }

        let base = base.children(
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
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_bounds(NavigationMenuBoundsKind::Popup, bounds)
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(base)
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuPopup<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPopupChild::Viewport(viewport) => NavigationMenuPopupChild::Viewport(
                    Box::new(viewport.with_navigation_menu_context(context.clone())),
                ),
                NavigationMenuPopupChild::Arrow(arrow) => NavigationMenuPopupChild::Arrow(
                    Box::new(arrow.with_navigation_menu_context(context.clone())),
                ),
                NavigationMenuPopupChild::Any(any) => NavigationMenuPopupChild::Any(any),
            })
            .collect();
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuPopupChild::Viewport(viewport) => NavigationMenuPopupChild::Viewport(
                    Box::new(viewport.with_contents(wiring.take_contents())),
                ),
                other => other,
            })
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuPopup<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<NavigationMenuPopupChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuPopupChild::Any(child.into_any_element()));
        self
    }

    pub fn side(mut self, side: NavigationMenuSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: NavigationMenuAlign) -> Self {
        self.align = align;
        self
    }

    /// Accessible label for the popup group; use the same text as the root's
    /// `.aria_label(...)`.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn keep_mounted(mut self, keep_mounted: bool) -> Self {
        self.keep_mounted = keep_mounted;
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuPopupStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
