use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, InteractiveElement as _, IntoElement, Orientation,
    ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::tabs::{
    child_wiring::{TabsChildNode, TabsChildWiring},
    Move, TabsActivateHighlighted, TabsContext, TabsListChild, TabsListStyleState, TabsOrientation,
    TabsSelectDown, TabsSelectFirst, TabsSelectLast, TabsSelectLeft, TabsSelectRight, TabsSelectUp,
    TABS_LIST_KEY_CONTEXT,
};

#[derive(IntoElement)]
pub struct TabsList<T: Clone + Eq + 'static> {
    base: Div,
    children: Vec<TabsListChild<T>>,
    context: Option<TabsContext<T>>,
    activate_on_focus: bool,
    loop_focus: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(TabsListStyleState, Div) -> Div + 'static>>,
}

impl<T: Clone + Eq + 'static> Default for TabsList<T> {
    fn default() -> Self {
        Self {
            base: div(),
            children: Vec::from([]),
            context: None,
            activate_on_focus: false,
            loop_focus: true,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl<T: Clone + Eq + 'static> Styled for TabsList<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<T: Clone + Eq + 'static> RenderOnce for TabsList<T> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let context = self.context;
        let (style_state, orientation) = context
            .as_ref()
            .map(|context| {
                context.read(_cx, |runtime, props| {
                    let orientation = props.orientation();
                    (Some(runtime.list_state(orientation)), orientation)
                })
            })
            .unwrap_or((None, TabsOrientation::Horizontal));
        let child_tab_indices = self
            .children
            .iter()
            .map(TabsChildNode::tab_index)
            .collect::<Vec<_>>();
        let bounds_context = context.clone();
        let select_left_context = context.clone();
        let select_right_context = context.clone();
        let select_up_context = context.clone();
        let select_down_context = context.clone();
        let select_first_context = context.clone();
        let select_last_context = context.clone();
        let activate_context = context.clone();
        let activate_on_focus = self.activate_on_focus;
        let loop_focus = self.loop_focus;
        let aria_label = self.aria_label;

        let base = match (self.style_with_state, style_state) {
            (Some(style_with_state), Some(style_state)) => style_with_state(style_state, self.base),
            _ => self.base,
        };

        base.on_children_prepainted(move |bounds, _window, cx| {
            if let Some(context) = bounds_context.as_ref() {
                let tab_bounds = bounds
                    .into_iter()
                    .zip(child_tab_indices.iter().copied())
                    .filter_map(|(bounds, index)| index.map(|index| (index, bounds)))
                    .collect();

                context.update(cx, |runtime| {
                    runtime.set_tab_bounds(tab_bounds);
                });
            }
        })
        .id("tabs-list")
        .key_context(TABS_LIST_KEY_CONTEXT)
        .focusable()
        .role(Role::TabList)
        .when(orientation == TabsOrientation::Vertical, |this| {
            this.aria_orientation(Orientation::Vertical)
        })
        .when_some(aria_label, |this, aria_label| this.aria_label(aria_label))
        .on_action(move |_: &TabsSelectLeft, window, cx| {
            let Some(context) = select_left_context.as_ref() else {
                return;
            };

            if orientation != TabsOrientation::Horizontal {
                return;
            }

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::Previous, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsSelectRight, window, cx| {
            let Some(context) = select_right_context.as_ref() else {
                return;
            };

            if orientation != TabsOrientation::Horizontal {
                return;
            }

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::Next, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsSelectUp, window, cx| {
            let Some(context) = select_up_context.as_ref() else {
                return;
            };

            if orientation != TabsOrientation::Vertical {
                return;
            }

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::Previous, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsSelectDown, window, cx| {
            let Some(context) = select_down_context.as_ref() else {
                return;
            };

            if orientation != TabsOrientation::Vertical {
                return;
            }

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::Next, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsSelectFirst, window, cx| {
            let Some(context) = select_first_context.as_ref() else {
                return;
            };

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::First, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsSelectLast, window, cx| {
            let Some(context) = select_last_context.as_ref() else {
                return;
            };

            context.update(cx, |runtime| {
                runtime.move_highlight(Move::Last, loop_focus);
            });

            if activate_on_focus {
                let value = context.read(cx, |runtime, _| runtime.highlighted_value());
                context.select(value, window, cx);
            }
        })
        .on_action(move |_: &TabsActivateHighlighted, window, cx| {
            let Some(context) = activate_context.as_ref() else {
                return;
            };

            let value = context.read(cx, |runtime, _| runtime.highlighted_value());
            context.select(value, window, cx);
        })
        .children(
            self.children
                .into_iter()
                .map(IntoElement::into_element)
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: Clone + Eq + 'static> TabsChildNode<T> for TabsList<T> {
    fn with_tabs_context(mut self, context: TabsContext<T>) -> Self {
        self.context = Some(context.clone());
        self.children = self
            .children
            .into_iter()
            .map(|child| child.with_tabs_context(context.clone()))
            .collect();
        self
    }

    fn wire_tabs_child(
        mut self,
        wiring: &mut TabsChildWiring<T>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        self.children = wiring.wire_list_children(self.children, window, cx);
        self
    }
}

impl<T: Clone + Eq + 'static> TabsList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<TabsListChild<T>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<TabsListChild<T>>>,
    ) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn activate_on_focus(mut self, activate_on_focus: bool) -> Self {
        self.activate_on_focus = activate_on_focus;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    /// Accessible name for the tab list. This is the only naming path in this
    /// gpui revision (no `aria-labelledby`); pair any visible heading with
    /// `Text::new_inaccessible(...)` so the name is not announced twice.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(TabsListStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
