use std::rc::Rc;

use gpui::{
    div, App, Div, ElementId, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled,
    Window,
};

use crate::menu::{
    child_wiring::wire_children, MenuChild, MenuContext, MenuMenubarLink, MenuMenubarOpenFn,
    MenuMove, MenuOpenChangeCompleteHandler, MenuOpenChangeDetails, MenuOpenChangeHandler,
    MenuOpenChangeReason, MenuOpenChangeSource, MenuOrientation, MenuParentKind, MenuProps,
    MenuRootStyleState,
};

type MenuRootStyle = Rc<dyn Fn(MenuRootStyleState, Div) -> Div + 'static>;

#[derive(IntoElement)]
pub struct MenuRoot<P: Clone + 'static = ()> {
    id: ElementId,
    base: Div,
    children: Vec<MenuChild<P>>,
    default_open: bool,
    open: Option<bool>,
    trigger_id: Option<Option<ElementId>>,
    default_trigger_id: Option<ElementId>,
    disabled: bool,
    modal: bool,
    loop_focus: bool,
    orientation: MenuOrientation,
    close_parent_on_esc: bool,
    highlight_item_on_hover: bool,
    on_open_change: Option<MenuOpenChangeHandler<P>>,
    on_open_change_complete: Option<MenuOpenChangeCompleteHandler<P>>,
    style_with_state: Option<MenuRootStyle>,
    menubar_link: Option<MenuMenubarLink>,
    context_menu_parent: bool,
}

impl<P: Clone + 'static> Default for MenuRoot<P> {
    fn default() -> Self {
        Self {
            id: ElementId::from("menu"),
            base: div(),
            children: Vec::new(),
            default_open: false,
            open: None,
            trigger_id: None,
            default_trigger_id: None,
            disabled: false,
            modal: true,
            loop_focus: true,
            orientation: MenuOrientation::Vertical,
            close_parent_on_esc: false,
            highlight_item_on_hover: true,
            on_open_change: None,
            on_open_change_complete: None,
            style_with_state: None,
            menubar_link: None,
            context_menu_parent: false,
        }
    }
}

impl<P: Clone + 'static> Styled for MenuRoot<P> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<P: Clone + 'static> RenderOnce for MenuRoot<P> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let parent_kind = match (self.menubar_link.is_some(), self.context_menu_parent) {
            (true, _) => MenuParentKind::Menubar,
            (false, true) => MenuParentKind::ContextMenu,
            (false, false) => MenuParentKind::None,
        };
        let mut context = MenuContext::new(
            self.id.clone(),
            cx,
            window,
            self.open,
            self.default_open,
            parent_kind,
            None,
            MenuProps::new(
                self.disabled,
                self.modal,
                self.loop_focus,
                self.orientation,
                self.close_parent_on_esc,
                self.highlight_item_on_hover,
                self.on_open_change,
                self.on_open_change_complete,
            ),
        );
        if let Some(link) = self.menubar_link.clone() {
            context = context.with_menubar_link(link);
        }

        let wired = wire_children(self.children, context.clone(), window, cx);
        let trigger_focus_handle = wired
            .trigger
            .as_ref()
            .and_then(|trigger| trigger.focus_handle());
        let trigger_disabled = wired
            .trigger
            .as_ref()
            .map(|trigger| trigger.disabled())
            .unwrap_or(false);
        let focused = wired
            .focus_handles
            .iter()
            .any(|focus_handle| focus_handle.is_focused(window));
        let close_for_focus_out = context.update(cx, |runtime| {
            runtime.sync_trigger(wired.trigger);
            runtime.sync_items(wired.items);
            runtime.sync_group_labels(wired.group_labels);
            runtime.sync_popup_focus_handle(wired.popup_focus_handle);
            for (index, default_checked) in wired.checkbox_defaults {
                runtime.register_checkbox_default(index, default_checked);
            }
            for (group_index, selected_item) in wired.radio_defaults {
                runtime.register_radio_default(group_index, selected_item);
            }
            runtime.sync_focus(focused);
            runtime.take_focus_out_close_request()
        });
        if close_for_focus_out {
            context.close(
                MenuOpenChangeReason::FocusOut,
                MenuOpenChangeSource::None,
                window,
                cx,
            );
        }
        if let Some(focus_handle) = context.update(cx, |runtime| runtime.take_popup_focus_on_open())
        {
            focus_handle.focus(window, cx);
        }

        if let Some(link) = context.menubar_link().cloned() {
            // Seam 2: the trigger participates in the menubar's roving focus;
            // registration also hands the menubar typed open/close commands
            // for hover-switch and keyboard handoff.
            let open_context = context.clone();
            let open_fn: MenuMenubarOpenFn = Rc::new(move |reason, source, window, cx| {
                if open_context.set_open(true, reason, source, window, cx)
                    && source == MenuOpenChangeSource::Keyboard
                {
                    let loop_focus = open_context.read(cx, |_, props| props.loop_focus());
                    open_context.update(cx, |runtime| {
                        runtime.move_highlight(MenuMove::First, loop_focus)
                    });
                }
            });
            let close_context = context.clone();
            let close_fn: MenuMenubarOpenFn = Rc::new(move |reason, source, window, cx| {
                close_context.set_open(false, reason, source, window, cx);
            });
            link.register_trigger(
                trigger_disabled,
                trigger_focus_handle.clone(),
                open_fn,
                close_fn,
                cx,
            );

            // Seam 4: focusing a menubar trigger opens its menu only while a
            // sibling menu is already open (rising focus edge only).
            let trigger_focused = trigger_focus_handle
                .as_ref()
                .map(|focus_handle| focus_handle.is_focused(window))
                .unwrap_or(false);
            let focus_gained =
                context.update(cx, |runtime| runtime.trigger_focus_edge(trigger_focused));
            let open = context.read(cx, |runtime, _| runtime.open_value());
            if focus_gained && !open && !trigger_disabled && link.has_submenu_open(cx) {
                link.highlight(cx);
                context.open(
                    MenuOpenChangeReason::TriggerFocus,
                    MenuOpenChangeSource::Focus,
                    window,
                    cx,
                );
            }
        }

        let style_state = context.read(cx, |runtime, props| runtime.root_state(props));
        let base = match self.style_with_state {
            Some(style_with_state) => style_with_state(style_state, self.base),
            None => self.base,
        };

        base.children(wired.children)
    }
}

impl<P: Clone + 'static> MenuRoot<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: impl Into<MenuChild<P>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<MenuChild<P>>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    pub fn child_any(mut self, child: impl IntoElement) -> Self {
        self.children.push(MenuChild::Any(child.into_any_element()));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
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

    pub fn trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.trigger_id = Some(Some(trigger_id.into()));
        self
    }

    pub fn default_trigger_id(mut self, trigger_id: impl Into<ElementId>) -> Self {
        self.default_trigger_id = Some(trigger_id.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
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

    /// Wiring seam used by `ContextMenuRoot`: types this menu's parent as a
    /// context menu, activating the cursor-anchor, always-modal, and grace
    /// branches inside the Menu layers.
    pub fn context_menu_parent(mut self) -> Self {
        self.context_menu_parent = true;
        self
    }

    /// Wiring seam used by the Menubar's child wiring: injects the menubar
    /// link that makes this menu a menubar-parented menu.
    pub fn menubar_link(mut self, link: MenuMenubarLink) -> Self {
        self.menubar_link = Some(link);
        self
    }

    /// The menu-level disabled fact, read by menubar child wiring for the
    /// initial trigger slot metadata.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }
}
