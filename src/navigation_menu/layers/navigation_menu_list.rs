use std::rc::Rc;

use gpui::{
    div, App, Div, InteractiveElement as _, IntoElement, Orientation, ParentElement, RenderOnce,
    Role, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
};

use crate::{
    navigation_menu::{
        child_wiring::{scoped_part_id, NavigationMenuChildNode, NavigationMenuChildWiring},
        NavigationMenuBoundsKind, NavigationMenuCloseAction, NavigationMenuContext,
        NavigationMenuFocusDown, NavigationMenuFocusFirst, NavigationMenuFocusLast,
        NavigationMenuFocusLeft, NavigationMenuFocusRight, NavigationMenuFocusUp,
        NavigationMenuListChild, NavigationMenuListStyleState, NavigationMenuMove,
        NavigationMenuOrientation, NavigationMenuValueChangeReason,
        NavigationMenuValueChangeSource, NAVIGATION_MENU_KEY_CONTEXT,
    },
    utils::direction::{current_direction, TextDirection},
};

type NavigationMenuListStyle = Rc<dyn Fn(NavigationMenuListStyleState, Div) -> Div + 'static>;

/// Roving arrow-focus composite over triggers and top-level links: arrow keys
/// along the orientation axis move GPUI focus without wrapping; with the menu
/// closed the "open" key (ArrowDown horizontal; ArrowRight vertical,
/// ArrowLeft in RTL) opens the highlighted trigger with `ListNavigation`.
#[derive(IntoElement)]
pub struct NavigationMenuList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<NavigationMenuListChild<T>>,
    context: Option<NavigationMenuContext<T>>,
    style_with_state: Option<NavigationMenuListStyle>,
}

impl<T: Clone + Eq + 'static> Default for NavigationMenuList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::new(),
            context: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for NavigationMenuList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for NavigationMenuList<T> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let Some(context) = self.context.clone() else {
            return div().children(
                self.children
                    .into_iter()
                    .map(IntoElement::into_element)
                    .collect::<Vec<_>>(),
            );
        };

        // Sync the roving highlight with actual GPUI focus.
        let focused_index = context
            .read(cx, |runtime, _| runtime.entry_focus_handles())
            .into_iter()
            .enumerate()
            .find(|(_, focus_handle)| {
                focus_handle
                    .as_ref()
                    .map(|handle| handle.is_focused(window))
                    .unwrap_or(false)
            })
            .map(|(index, _)| index);
        if let Some(index) = focused_index {
            context.update(cx, |runtime| runtime.set_highlight(index));
        }

        let (state, orientation) = context.read(cx, |runtime, props| {
            (runtime.list_state(), props.orientation())
        });

        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(state, self.base),
            None => self.base,
        };

        let measure_context = context.clone();
        let previous_context = context.clone();
        let next_context = context.clone();
        let up_context = context.clone();
        let down_context = context.clone();
        let first_context = context.clone();
        let last_context = context.clone();
        let close_context = context.clone();

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| {
                    runtime.set_bounds(NavigationMenuBoundsKind::List, bounds)
                });
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(
                base.id(scoped_part_id(&context.root_id(), "navigation-menu-list"))
                    .role(Role::List)
                    .aria_orientation(match orientation {
                        NavigationMenuOrientation::Horizontal => Orientation::Horizontal,
                        NavigationMenuOrientation::Vertical => Orientation::Vertical,
                    })
                    .key_context(NAVIGATION_MENU_KEY_CONTEXT)
                    .on_action(
                        move |_: &NavigationMenuFocusLeft, window, cx| match orientation {
                            NavigationMenuOrientation::Horizontal => {
                                move_focus(
                                    &previous_context,
                                    NavigationMenuMove::Previous,
                                    window,
                                    cx,
                                );
                            }
                            NavigationMenuOrientation::Vertical => {
                                if current_direction() == TextDirection::Rtl {
                                    open_highlighted(&previous_context, window, cx);
                                }
                            }
                        },
                    )
                    .on_action(
                        move |_: &NavigationMenuFocusRight, window, cx| match orientation {
                            NavigationMenuOrientation::Horizontal => {
                                move_focus(&next_context, NavigationMenuMove::Next, window, cx);
                            }
                            NavigationMenuOrientation::Vertical => {
                                if current_direction() == TextDirection::Ltr {
                                    open_highlighted(&next_context, window, cx);
                                }
                            }
                        },
                    )
                    .on_action(move |_: &NavigationMenuFocusUp, window, cx| {
                        if orientation == NavigationMenuOrientation::Vertical {
                            move_focus(&up_context, NavigationMenuMove::Previous, window, cx);
                        }
                    })
                    .on_action(
                        move |_: &NavigationMenuFocusDown, window, cx| match orientation {
                            NavigationMenuOrientation::Horizontal => {
                                open_highlighted(&down_context, window, cx);
                            }
                            NavigationMenuOrientation::Vertical => {
                                move_focus(&down_context, NavigationMenuMove::Next, window, cx);
                            }
                        },
                    )
                    .on_action(move |_: &NavigationMenuFocusFirst, window, cx| {
                        move_focus(&first_context, NavigationMenuMove::First, window, cx);
                    })
                    .on_action(move |_: &NavigationMenuFocusLast, window, cx| {
                        move_focus(&last_context, NavigationMenuMove::Last, window, cx);
                    })
                    .on_action(move |_: &NavigationMenuCloseAction, window, cx| {
                        close_context.close(
                            NavigationMenuValueChangeReason::EscapeKey,
                            NavigationMenuValueChangeSource::Keyboard,
                            window,
                            cx,
                        );
                    })
                    .children(
                        self.children
                            .into_iter()
                            .map(IntoElement::into_element)
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}

fn move_focus<T: Clone + Eq + 'static>(
    context: &NavigationMenuContext<T>,
    direction: NavigationMenuMove,
    window: &mut Window,
    cx: &mut App,
) {
    let focus_handle = context.update(cx, |runtime| runtime.move_highlight(direction));
    if let Some(focus_handle) = focus_handle {
        focus_handle.focus(window, cx);
    }
}

/// Keyboard open of the highlighted trigger (`ListNavigation`); the action
/// path never routes through the click path, so no double-fire.
fn open_highlighted<T: Clone + Eq + 'static>(
    context: &NavigationMenuContext<T>,
    window: &mut Window,
    cx: &mut App,
) {
    let Some(value) = context.read(cx, |runtime, _| runtime.highlighted_trigger_value()) else {
        return;
    };
    context.set_value(
        Some(value),
        NavigationMenuValueChangeReason::ListNavigation,
        NavigationMenuValueChangeSource::Keyboard,
        window,
        cx,
    );
}

impl<T: Clone + Eq + 'static> NavigationMenuChildNode<T> for NavigationMenuList<T> {
    fn with_navigation_menu_context(mut self, context: NavigationMenuContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuListChild::Item(item) => NavigationMenuListChild::Item(Box::new(
                    item.with_navigation_menu_context(context.clone()),
                )),
                NavigationMenuListChild::Link(link) => NavigationMenuListChild::Link(Box::new(
                    link.with_navigation_menu_context(context.clone()),
                )),
                NavigationMenuListChild::Any(any) => NavigationMenuListChild::Any(any),
            })
            .collect();
        self
    }

    fn wire_navigation_menu_child(
        mut self,
        wiring: &mut NavigationMenuChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                NavigationMenuListChild::Item(item) => NavigationMenuListChild::Item(Box::new(
                    item.wire_navigation_menu_child(wiring, window, cx),
                )),
                NavigationMenuListChild::Link(link) => NavigationMenuListChild::Link(Box::new(
                    link.wire_navigation_menu_child(wiring, window, cx),
                )),
                other => other,
            })
            .collect();
        self
    }
}

impl<T: Clone + Eq + 'static> NavigationMenuList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<NavigationMenuListChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(NavigationMenuListChild::Any(child.into_any_element()));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(NavigationMenuListStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
