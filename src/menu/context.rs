use std::rc::Rc;

use gpui::{App, ElementId, Entity, Pixels, Point, Window};

use crate::menu::{
    MenuItemKind, MenuMenubarLink, MenuOpenChangeDetails, MenuOpenChangeReason,
    MenuOpenChangeSource, MenuParentKind, MenuProps, MenuRuntime,
};

/// Thin injection vehicle for one menu of the tree. Submenu contexts carry a
/// link to their parent context (plus the parent item index anchoring the
/// child branch), forming the tree Menubar and Context Menu will host later.
pub struct MenuContext<P: Clone + 'static> {
    id: ElementId,
    runtime: Entity<MenuRuntime<P>>,
    props: Rc<MenuProps<P>>,
    controlled_open: Rc<Option<bool>>,
    parent: Option<Rc<MenuContext<P>>>,
    parent_item_index: Option<usize>,
    menubar: Option<Rc<MenuMenubarLink>>,
}

impl<P: Clone + 'static> Clone for MenuContext<P> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            runtime: self.runtime.clone(),
            props: Rc::clone(&self.props),
            controlled_open: Rc::clone(&self.controlled_open),
            parent: self.parent.clone(),
            parent_item_index: self.parent_item_index,
            menubar: self.menubar.clone(),
        }
    }
}

impl<P: Clone + 'static> MenuContext<P> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<ElementId>,
        cx: &mut App,
        window: &mut Window,
        controlled_open: Option<bool>,
        default_open: bool,
        parent_kind: MenuParentKind,
        parent: Option<(MenuContext<P>, usize)>,
        props: MenuProps<P>,
    ) -> Self {
        let id = id.into();
        let open = controlled_open.unwrap_or(default_open);
        let runtime =
            window.use_keyed_state(id.clone(), cx, |_, _| MenuRuntime::new(open, parent_kind));
        let (parent, parent_item_index) = match parent {
            Some((parent, item_index)) => (Some(Rc::new(parent)), Some(item_index)),
            None => (None, None),
        };

        Self {
            id,
            runtime,
            props: Rc::new(props),
            controlled_open: Rc::new(controlled_open),
            parent,
            parent_item_index,
            menubar: None,
        }
    }

    /// Attaches the menubar link injected by menubar child wiring; only
    /// menus rendered under a `Menubar` carry one.
    pub fn with_menubar_link(mut self, link: MenuMenubarLink) -> Self {
        self.menubar = Some(Rc::new(link));
        self
    }

    pub fn menubar_link(&self) -> Option<&MenuMenubarLink> {
        self.menubar.as_deref()
    }

    /// Walks up the submenu tree to the root menu's menubar link, so nested
    /// submenu popups can relay keys to the menubar.
    pub fn menubar_link_in_tree(&self) -> Option<MenuMenubarLink> {
        match (&self.menubar, self.parent.as_deref()) {
            (Some(link), _) => Some(link.as_ref().clone()),
            (None, Some(parent)) => parent.menubar_link_in_tree(),
            (None, None) => None,
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.id.clone()
    }

    pub fn parent(&self) -> Option<&MenuContext<P>> {
        self.parent.as_deref()
    }

    pub fn parent_item_index(&self) -> Option<usize> {
        self.parent_item_index
    }

    /// Walks up to the root menu context of the tree (the menu whose runtime
    /// carries the tree-level parent kind and Context Menu gesture facts).
    pub fn tree_root(&self) -> &MenuContext<P> {
        match self.parent.as_deref() {
            Some(parent) => parent.tree_root(),
            None => self,
        }
    }

    pub fn read<Output>(
        &self,
        cx: &App,
        read: impl FnOnce(&MenuRuntime<P>, &MenuProps<P>) -> Output,
    ) -> Output {
        read(self.runtime.read(cx), self.props.as_ref())
    }

    pub fn update<Output>(
        &self,
        cx: &mut App,
        update: impl FnOnce(&mut MenuRuntime<P>) -> Output,
    ) -> Output {
        let controlled_open = *self.controlled_open.as_ref();
        self.runtime.update(cx, |runtime, cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            let output = update(runtime);
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            cx.notify();
            output
        })
    }

    /// Resolves controlled vs uncontrolled open, closes descendants on close,
    /// fires callbacks with cancelable details, and coordinates the parent
    /// branch bookkeeping. The controlled/uncontrolled open rule lives here
    /// and nowhere else.
    pub fn set_open(
        &self,
        next_open: bool,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if self.props.disabled() && next_open {
            return false;
        }
        let controlled_open = *self.controlled_open.as_ref();
        let outcome = self.runtime.update(cx, |runtime, _cx| {
            if let Some(open) = controlled_open {
                runtime.sync_open_from_context(open);
            }
            let current = controlled_open.unwrap_or_else(|| runtime.open_value());
            runtime.request_open_change(current, next_open)
        });
        if !outcome.changed() {
            return false;
        }

        // Opening a submenu closes any open sibling branch of the parent.
        if next_open {
            // Opening a menubar-hosted menu closes the open sibling menu
            // (`SiblingOpen`, retaining `has_submenu_open` across the handoff).
            if let Some(link) = self.menubar.as_deref() {
                link.close_sibling(MenuOpenChangeReason::SiblingOpen, source, window, cx);
            }
            if let (Some(parent), Some(item_index)) =
                (self.parent.as_deref(), self.parent_item_index)
            {
                let siblings =
                    parent.read(cx, |runtime, _| runtime.sibling_links_except(item_index));
                for sibling in siblings {
                    if sibling.is_open(cx) {
                        sibling.close(MenuOpenChangeReason::SiblingOpen, window, cx);
                    }
                }
            }
        } else {
            // Closing a parent closes all open descendants first.
            self.close_descendants(reason, window, cx);
        }

        let (open, trigger_id, payload) = outcome.into_parts();
        let mut details = MenuOpenChangeDetails::new(reason, source, trigger_id, payload, true);
        if let Some(on_open_change) = self.props.on_open_change() {
            on_open_change(open, &mut details, window, cx);
        }
        if details.is_canceled() {
            return false;
        }

        let prevent_unmount = !open && details.prevents_unmount_on_close();
        self.runtime.update(cx, |runtime, cx| {
            runtime.commit_open(open, prevent_unmount, controlled_open.is_none());
            runtime.record_open_change(reason, source);
            if open {
                runtime.request_popup_focus_on_open(reason, source);
            }
            cx.notify();
        });

        if let (Some(parent), Some(item_index)) = (self.parent.as_deref(), self.parent_item_index) {
            parent.update(cx, |runtime| match open {
                true => runtime.note_submenu_opened(item_index),
                false => runtime.note_submenu_closed(item_index),
            });
        }

        // Direct menubar children report open changes so the menubar
        // maintains `has_submenu_open` (seam 6); submenus never do.
        if let Some(link) = self.menubar.as_deref() {
            link.note_open_change(open, reason, cx);
        }

        if let Some(on_open_change_complete) = self.props.on_open_change_complete() {
            on_open_change_complete(open, &details, window, cx);
        }

        // Menubar-parent menus skip focus return for outside presses (seam
        // 13) and for sibling handoffs, where focus moves to the new branch.
        let menubar_skips_focus_return = self.menubar.is_some()
            && matches!(
                reason,
                MenuOpenChangeReason::OutsidePress | MenuOpenChangeReason::SiblingOpen
            );
        if !open && reason != MenuOpenChangeReason::FocusOut && !menubar_skips_focus_return {
            let focus_handle = self.read(cx, |runtime, _| match runtime.parent_kind() {
                MenuParentKind::Submenu => self
                    .parent_item_index
                    .and_then(|_| runtime.trigger_focus_handle()),
                _ => runtime.trigger_focus_handle(),
            });
            let focus_handle = focus_handle.or_else(|| {
                // Submenu trigger focus lives in the parent's item metadata.
                match (self.parent.as_deref(), self.parent_item_index) {
                    (Some(parent), Some(item_index)) => {
                        parent.read(cx, |runtime, _| runtime.item_focus_handle(item_index))
                    }
                    _ => None,
                }
            });
            if let Some(focus_handle) = focus_handle {
                focus_handle.focus(window, cx);
            }
        }

        true
    }

    pub fn open(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(true, reason, source, window, cx)
    }

    pub fn close(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        self.set_open(false, reason, source, window, cx)
    }

    pub fn toggle(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        let open = self.read(cx, |runtime, _| runtime.open_value());
        self.set_open(!open, reason, source, window, cx)
    }

    /// Closes every open descendant branch (cascade), innermost included.
    pub fn close_descendants(
        &self,
        reason: MenuOpenChangeReason,
        window: &mut Window,
        cx: &mut App,
    ) {
        let links = self.read(cx, |runtime, _| runtime.all_submenu_links());
        for link in links {
            if link.is_open(cx) {
                link.close(reason, window, cx);
            }
        }
    }

    /// Closes this menu and every ancestor up to the root (item press paths).
    pub fn close_tree(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.close(reason, source, window, cx);
        if let Some(parent) = self.parent.as_deref() {
            parent.close_tree(reason, source, window, cx);
        }
    }

    /// Closes ancestors only (Escape with `close_parent_on_esc`).
    pub fn close_ancestors(
        &self,
        reason: MenuOpenChangeReason,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(parent) = self.parent.as_deref() {
            parent.close(reason, source, window, cx);
            parent.close_ancestors(reason, source, window, cx);
        }
    }

    /// Activates one registered item: runs its activation callback and closes
    /// the whole tree when the item's `close_on_click` says so.
    pub fn activate_item(
        &self,
        index: usize,
        source: MenuOpenChangeSource,
        window: &mut Window,
        cx: &mut App,
    ) -> bool {
        if self.props.disabled() {
            return false;
        }
        let Some((kind, disabled, close_on_click, activation)) =
            self.read(cx, |runtime, _| runtime.item_activation(index))
        else {
            return false;
        };
        if disabled || kind == MenuItemKind::SubmenuTrigger {
            return false;
        }
        if let Some(activation) = activation {
            activation(window, cx);
        }
        if close_on_click {
            self.close_tree(MenuOpenChangeReason::ItemPress, source, window, cx);
        }
        true
    }

    /// Tests a pressed point against the union of this menu's open tree:
    /// its own popup and trigger plus every open descendant branch.
    pub fn press_inside_tree(&self, position: Point<Pixels>, cx: &App) -> bool {
        if self.read(cx, |runtime, _| runtime.own_tree_contains(position)) {
            return true;
        }
        let links = self.read(cx, |runtime, _| runtime.all_submenu_links());
        links
            .into_iter()
            .any(|link| link.is_open(cx) && link.tree_contains(position, cx))
    }
}
