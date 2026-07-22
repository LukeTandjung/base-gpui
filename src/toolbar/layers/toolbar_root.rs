use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    Orientation, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    toolbar::{
        child_wiring::{move_focus, wire_children},
        ToolbarChild, ToolbarContext, ToolbarFocusDown, ToolbarFocusLeft, ToolbarFocusRight,
        ToolbarFocusUp, ToolbarMove, ToolbarOrientation, ToolbarProps, ToolbarRootStyleState,
        TOOLBAR_KEY_CONTEXT,
    },
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

#[derive(IntoElement)]
pub struct ToolbarRoot {
    id: ElementId,
    base: Div,
    children: Vec<ToolbarChild>,
    orientation: ToolbarOrientation,
    loop_focus: bool,
    disabled: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(ToolbarRootStyleState, Div) -> Div + 'static>>,
}

impl Default for ToolbarRoot {
    fn default() -> Self {
        Self {
            id: ElementId::from("toolbar-root"),
            base: div(),
            children: Vec::new(),
            orientation: ToolbarOrientation::Horizontal,
            loop_focus: true,
            disabled: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl Styled for ToolbarRoot {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for ToolbarRoot {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let orientation = self.orientation;
        let context = ToolbarContext::new(
            self.id.clone(),
            cx,
            window,
            ToolbarProps::new(orientation, self.loop_focus, self.disabled),
        );

        let wired_children =
            wire_children(self.children, context.clone(), self.disabled, window, cx);
        let items = wired_children.items;
        let focus_handles = wired_children.focus_handles;
        let focused_index = wired_children.focused_index;
        let children = wired_children.children;

        context.update(cx, |runtime| {
            runtime.sync_children(items, focus_handles);
            runtime.sync_focused_index(focused_index);
            runtime.reconcile();
        });

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let direction = current_direction();
        let left_context = context.clone();
        let right_context = context.clone();
        let up_context = context.clone();
        let down_context = context;

        base.id(self.id)
            .role(Role::Toolbar)
            .aria_orientation(match orientation {
                ToolbarOrientation::Horizontal => Orientation::Horizontal,
                ToolbarOrientation::Vertical => Orientation::Vertical,
            })
            .when_some(self.aria_label, |this, aria_label| {
                this.aria_label(aria_label)
            })
            .key_context(TOOLBAR_KEY_CONTEXT)
            .on_action(move |_: &ToolbarFocusLeft, window, cx| {
                if orientation != ToolbarOrientation::Horizontal {
                    return;
                }

                move_focus(
                    &left_context,
                    horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Left)),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &ToolbarFocusRight, window, cx| {
                if orientation != ToolbarOrientation::Horizontal {
                    return;
                }

                move_focus(
                    &right_context,
                    horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Right)),
                    window,
                    cx,
                );
            })
            .on_action(move |_: &ToolbarFocusUp, window, cx| {
                if orientation != ToolbarOrientation::Vertical {
                    return;
                }

                move_focus(&up_context, ToolbarMove::Previous, window, cx);
            })
            .on_action(move |_: &ToolbarFocusDown, window, cx| {
                if orientation != ToolbarOrientation::Vertical {
                    return;
                }

                move_focus(&down_context, ToolbarMove::Next, window, cx);
            })
            .children(children)
    }
}

impl ToolbarRoot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<ToolbarChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<ToolbarChild>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible name for the toolbar, announced by screen readers. There is
    /// no `aria-labelledby` id-reference builder in this gpui revision, so
    /// the name is a literal string.
    pub fn aria_label(mut self, aria_label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(ToolbarRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}

fn horizontal_move(direction: HorizontalDirection) -> ToolbarMove {
    match direction {
        HorizontalDirection::Previous => ToolbarMove::Previous,
        HorizontalDirection::Next => ToolbarMove::Next,
    }
}
