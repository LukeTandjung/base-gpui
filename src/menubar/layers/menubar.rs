use std::rc::Rc;

use gpui::{
    div, prelude::FluentBuilder as _, App, Div, ElementId, InteractiveElement as _, IntoElement,
    Orientation, ParentElement, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window,
};

use crate::{
    menubar::{
        child_wiring::wire_children, MenubarChild, MenubarContext, MenubarFocusDown,
        MenubarFocusFirst, MenubarFocusLast, MenubarFocusLeft, MenubarFocusRight, MenubarFocusUp,
        MenubarMove, MenubarOrientation, MenubarProps, MenubarStyleState, MENUBAR_KEY_CONTEXT,
    },
    utils::direction::{current_direction, HorizontalArrowKey, HorizontalDirection},
};

/// The single Menubar part: the trigger row container with roving focus
/// across the hosted menus' triggers and cross-menu coordination state.
#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    base: Div,
    children: Vec<MenubarChild>,
    orientation: MenubarOrientation,
    loop_focus: bool,
    modal: bool,
    disabled: bool,
    aria_label: Option<SharedString>,
    style_with_state: Option<Rc<dyn Fn(MenubarStyleState, Div) -> Div + 'static>>,
}

impl Default for Menubar {
    fn default() -> Self {
        Self {
            id: ElementId::from("menubar"),
            base: div(),
            children: Vec::new(),
            orientation: MenubarOrientation::Horizontal,
            loop_focus: true,
            modal: true,
            disabled: false,
            aria_label: None,
            style_with_state: None,
        }
    }
}

impl Styled for Menubar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Menubar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let orientation = self.orientation;
        let context = MenubarContext::new(
            self.id.clone(),
            cx,
            window,
            MenubarProps::new(orientation, self.loop_focus, self.modal, self.disabled),
        );

        let wired = wire_children(self.children, &context);
        context.update(cx, |runtime| {
            runtime.sync_triggers(wired.triggers);
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
        let down_context = context.clone();
        let first_context = context.clone();
        let last_context = context.clone();
        let measure_context = context;

        div()
            .on_children_prepainted(move |bounds, window, cx| {
                let Some(bounds) = bounds.first().copied() else {
                    return;
                };
                let changed = measure_context.update(cx, |runtime| runtime.set_bounds(bounds));
                if changed {
                    window.request_animation_frame();
                }
            })
            .child(
                // Base UI emits exactly `role="menubar"` + `aria-orientation`
                // here. Menubar `disabled` has no `.aria_disabled(...)`
                // builder in this gpui revision (documented gap): disabled
                // state stays unannounced but the trigger gating keeps AT
                // clicks inert. Labeling is consumer-provided via
                // `.aria_label(...)`.
                base.id(self.id)
                    .role(Role::MenuBar)
                    .aria_orientation(match orientation {
                        MenubarOrientation::Horizontal => Orientation::Horizontal,
                        MenubarOrientation::Vertical => Orientation::Vertical,
                    })
                    .when_some(self.aria_label, |this, label| this.aria_label(label))
                    .key_context(MENUBAR_KEY_CONTEXT)
                    .on_action(move |_: &MenubarFocusLeft, window, cx| {
                        if orientation != MenubarOrientation::Horizontal {
                            return;
                        }
                        left_context.move_focus(
                            horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Left)),
                            window,
                            cx,
                        );
                    })
                    .on_action(move |_: &MenubarFocusRight, window, cx| {
                        if orientation != MenubarOrientation::Horizontal {
                            return;
                        }
                        right_context.move_focus(
                            horizontal_move(direction.horizontal_arrow(HorizontalArrowKey::Right)),
                            window,
                            cx,
                        );
                    })
                    .on_action(move |_: &MenubarFocusUp, window, cx| {
                        if orientation != MenubarOrientation::Vertical {
                            return;
                        }
                        up_context.move_focus(MenubarMove::Previous, window, cx);
                    })
                    .on_action(move |_: &MenubarFocusDown, window, cx| {
                        if orientation != MenubarOrientation::Vertical {
                            return;
                        }
                        down_context.move_focus(MenubarMove::Next, window, cx);
                    })
                    .on_action(move |_: &MenubarFocusFirst, window, cx| {
                        first_context.move_focus(MenubarMove::First, window, cx);
                    })
                    .on_action(move |_: &MenubarFocusLast, window, cx| {
                        last_context.move_focus(MenubarMove::Last, window, cx);
                    })
                    .children(
                        wired
                            .children
                            .into_iter()
                            .map(IntoElement::into_element)
                            .collect::<Vec<_>>(),
                    ),
            )
    }
}

fn horizontal_move(direction: HorizontalDirection) -> MenubarMove {
    match direction {
        HorizontalDirection::Next => MenubarMove::Next,
        HorizontalDirection::Previous => MenubarMove::Previous,
    }
}

impl Menubar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MenubarChild>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenubarChild::Any(child.into_any_element()));
        self
    }

    pub fn orientation(mut self, orientation: MenubarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Accessible label for the menubar row; Base UI leaves labeling to the
    /// consumer, so this stays optional.
    pub fn aria_label(mut self, label: impl Into<SharedString>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenubarStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
