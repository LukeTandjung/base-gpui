use std::rc::Rc;

use gpui::{
    div, App, Bounds, Div, ElementId, IntoElement, ParentElement, Pixels, Point, RenderOnce,
    SharedString, StyleRefinement, Styled, Window,
};

use crate::menu::{
    child_wiring::{part_focus_handle, wire_children, MenuChildNode, MenuChildWiring},
    MenuChild, MenuContext, MenuItemKind, MenuItemMetadata, MenuMove,
    MenuOpenChangeCompleteHandler, MenuOpenChangeDetails, MenuOpenChangeHandler,
    MenuOpenChangeReason, MenuOpenChangeSource, MenuOrientation, MenuParentKind, MenuProps,
    MenuRootStyleState, MenuSubmenuLink, MenuSubmenuRootChild, MenuTriggerMetadata,
};

type MenuSubmenuRootStyle = Rc<dyn Fn(MenuRootStyleState, Div) -> Div + 'static>;

/// Submenu root: simultaneously anchors an item of the parent menu (its
/// trigger) and owns a child menu runtime linked to the parent, matching
/// Base UI's `MenuSubmenuRoot`. Submenus never apply modal behavior and omit
/// `modal`, `trigger_id`, and `default_trigger_id` from the root surface.
#[derive(IntoElement)]
pub struct MenuSubmenuRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<MenuSubmenuRootChild<P>>,
    parent_context: Option<MenuContext<P>>,
    item_index: Option<usize>,
    default_open: bool,
    open: Option<bool>,
    disabled: bool,
    loop_focus: bool,
    orientation: MenuOrientation,
    close_parent_on_esc: bool,
    highlight_item_on_hover: bool,
    on_open_change: Option<MenuOpenChangeHandler<P>>,
    on_open_change_complete: Option<MenuOpenChangeCompleteHandler<P>>,
    style_with_state: Option<MenuSubmenuRootStyle>,
}

impl<P: Clone + 'static> Default for MenuSubmenuRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu-submenu"),
            base: div(),
            children: Vec::new(),
            parent_context: None,
            item_index: None,
            default_open: false,
            open: None,
            disabled: false,
            loop_focus: true,
            orientation: MenuOrientation::Vertical,
            close_parent_on_esc: false,
            highlight_item_on_hover: true,
            on_open_change: None,
            on_open_change_complete: None,
            style_with_state: None,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuSubmenuRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuSubmenuRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (Some(parent), Some(item_index)) = (self.parent_context.clone(), self.item_index)
        else {
            return div();
        };

        let child_context = MenuContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            MenuParentKind::Submenu,
            Some((parent.clone(), item_index)),
            MenuProps::new(
                self.disabled,
                // Submenus never apply modal behavior regardless of config.
                false,
                self.loop_focus,
                self.orientation,
                self.close_parent_on_esc,
                self.highlight_item_on_hover,
                self.on_open_change,
                self.on_open_change_complete,
            ),
        );

        // Split children: the submenu trigger renders in the parent popup
        // flow; portal children are wired against the child context.
        let mut trigger_element = None;
        let mut portal_children: Vec<MenuChild<P>> = Vec::new();
        let mut any_children = Vec::new();
        let mut trigger_facts = (false, std::time::Duration::ZERO, std::time::Duration::ZERO);
        for child in self.children {
            match child {
                MenuSubmenuRootChild::SubmenuTrigger(trigger) => {
                    trigger_facts = (
                        trigger.open_on_hover_value(),
                        trigger.delay_value(),
                        trigger.close_delay_value(),
                    );
                    trigger_element = Some(
                        trigger
                            .with_contexts(parent.clone(), child_context.clone(), item_index)
                            .into_any_element(),
                    );
                }
                MenuSubmenuRootChild::Portal(portal) => {
                    portal_children.push(MenuChild::Portal(portal));
                }
                MenuSubmenuRootChild::Any(any) => any_children.push(any),
            }
        }
        let (open_on_hover, _delay, close_delay) = trigger_facts;
        let _ = open_on_hover;

        let wired = wire_children(portal_children, child_context.clone(), window, cx);

        // The child's trigger is the parent item: sync trigger metadata and
        // mirror the measured parent item bounds as the child anchor.
        let parent_item_bounds = parent.read(cx, |runtime, _| runtime.item_bounds(item_index));
        let parent_item_focus = parent.read(cx, |runtime, _| runtime.item_focus_handle(item_index));
        let trigger_id = self.id.clone();
        child_context.update(cx, |runtime| {
            runtime.sync_trigger(Some(MenuTriggerMetadata::new(
                trigger_id,
                false,
                trigger_facts.0,
                trigger_facts.1,
                trigger_facts.2,
                None,
                parent_item_focus,
            )));
            if let Some(bounds) = parent_item_bounds {
                runtime.set_trigger_bounds(bounds);
            }
            runtime.sync_items(wired.items);
            runtime.sync_group_labels(wired.group_labels);
            runtime.sync_popup_focus_handle(wired.popup_focus_handle);
            for (index, default_checked) in wired.checkbox_defaults {
                runtime.register_checkbox_default(index, default_checked);
            }
            for (group_index, selected_item) in wired.radio_defaults {
                runtime.register_radio_default(group_index, selected_item);
            }
        });

        // Register the tree link in the parent runtime.
        let close_context = child_context.clone();
        let open_context = child_context.clone();
        let is_open_context = child_context.clone();
        let contains_context = child_context.clone();
        let bounds_context = child_context.clone();
        parent.update(cx, |runtime| {
            runtime.register_submenu_link(MenuSubmenuLink::new(
                item_index,
                close_delay,
                Rc::new(move |reason, window, cx| {
                    close_context.close(reason, MenuOpenChangeSource::None, window, cx);
                }),
                Rc::new(move |window, cx| {
                    if open_context.set_open(
                        true,
                        MenuOpenChangeReason::ListNavigation,
                        MenuOpenChangeSource::Keyboard,
                        window,
                        cx,
                    ) {
                        let loop_focus = open_context.read(cx, |_, props| props.loop_focus());
                        open_context.update(cx, |runtime| {
                            runtime.move_highlight(MenuMove::First, loop_focus)
                        });
                    }
                }),
                Rc::new(move |cx| is_open_context.read(cx, |runtime, _| runtime.open_value())),
                Rc::new(move |point: Point<Pixels>, cx: &App| {
                    contains_context.press_inside_tree(point, cx)
                }),
                Rc::new(move |cx: &App| -> Option<Bounds<Pixels>> {
                    bounds_context.read(cx, |runtime, _| runtime.popup_bounds())
                }),
            ));
        });

        if let Some(focus_handle) =
            child_context.update(cx, |runtime| runtime.take_popup_focus_on_open())
        {
            focus_handle.focus(window, cx);
        }

        let style_state = child_context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        let mut base = base;
        if let Some(trigger) = trigger_element {
            base = base.child(trigger);
        }
        base.children(any_children).children(wired.children)
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuSubmenuRoot<P> {
    fn wire_menu_child(
        mut self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        // The submenu trigger participates in the parent's item list.
        let index = wiring.next_item_index();
        let scoped_id = wiring.scope_child_id(&ElementId::from(SharedString::from(format!(
            "{}-{index}",
            self.id
        ))));
        let focus_handle = part_focus_handle(&scoped_id, window, cx);
        let (label, disabled) = self
            .children
            .iter()
            .find_map(|child| match child {
                MenuSubmenuRootChild::SubmenuTrigger(trigger) => {
                    Some((trigger.label_value(), trigger.disabled_value()))
                }
                _ => None,
            })
            .unwrap_or((None, false));
        wiring.register_item(
            MenuItemMetadata::new(index, MenuItemKind::SubmenuTrigger, label, disabled, false)
                .with_focus_handle(focus_handle.clone()),
        );
        self.children = self
            .children
            .into_iter()
            .map(|child| match child {
                MenuSubmenuRootChild::SubmenuTrigger(trigger) => {
                    MenuSubmenuRootChild::SubmenuTrigger(Box::new(
                        trigger.wired(index, focus_handle.clone()),
                    ))
                }
                other => other,
            })
            .collect();
        self.id = scoped_id;
        self.item_index = Some(index);
        self.parent_context = Some(context.clone());
        self
    }
}

impl<P: Clone + 'static> MenuSubmenuRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn child(mut self, child: impl Into<MenuSubmenuRootChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children
            .push(MenuSubmenuRootChild::Any(child.into_any_element()));
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = default_open;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn orientation(mut self, orientation: MenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn close_parent_on_esc(mut self, close_parent_on_esc: bool) -> Self {
        self.close_parent_on_esc = close_parent_on_esc;
        self
    }

    pub fn highlight_item_on_hover(mut self, highlight_item_on_hover: bool) -> Self {
        self.highlight_item_on_hover = highlight_item_on_hover;
        self
    }

    pub fn on_open_change(
        mut self,
        on_open_change: impl Fn(bool, &mut MenuOpenChangeDetails<P>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    pub fn on_open_change_complete(
        mut self,
        on_open_change_complete: impl Fn(bool, &MenuOpenChangeDetails<P>, &mut Window, &mut App)
            + 'static,
    ) -> Self {
        self.on_open_change_complete = Some(Rc::new(on_open_change_complete));
        self
    }

    pub fn style_with_state(
        mut self,
        style: impl Fn(MenuRootStyleState, Div) -> Div + 'static,
    ) -> Self {
        self.style_with_state = Some(Rc::new(style));
        self
    }
}
