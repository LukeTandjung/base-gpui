use gpui::{App, ElementId, SharedString, Window};

use crate::navigation_menu::{
    NavigationMenuChild, NavigationMenuContent, NavigationMenuContext, NavigationMenuItemMetadata,
    NavigationMenuListEntry,
};

pub struct WiredNavigationMenuChildren<T: Clone + Eq + 'static> {
    pub items: Vec<NavigationMenuItemMetadata<T>>,
    pub list_entries: Vec<NavigationMenuListEntry<T>>,
    pub children: Vec<NavigationMenuChild<T>>,
}

/// Internal traversal seam: the only code that walks children, assigns
/// roving indices, collects per-item content for the shared viewport, and
/// attaches context before erasure to `AnyElement`.
pub trait NavigationMenuChildNode<T: Clone + Eq + 'static>: Sized {
    fn with_navigation_menu_context(self, context: NavigationMenuContext<T>) -> Self;

    fn wire_navigation_menu_child(
        self,
        _wiring: &mut NavigationMenuChildWiring<T>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

pub struct NavigationMenuChildWiring<T: Clone + Eq + 'static> {
    root_id: ElementId,
    next_item_order: usize,
    next_entry_index: usize,
    items: Vec<NavigationMenuItemMetadata<T>>,
    list_entries: Vec<NavigationMenuListEntry<T>>,
    contents: Vec<NavigationMenuContent<T>>,
}

impl<T: Clone + Eq + 'static> NavigationMenuChildWiring<T> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_item_order: 0,
            next_entry_index: 0,
            items: Vec::new(),
            list_entries: Vec::new(),
            contents: Vec::new(),
        }
    }

    pub fn root_id(&self) -> ElementId {
        self.root_id.clone()
    }

    pub fn next_item_order(&self) -> usize {
        self.next_item_order
    }

    pub fn register_item(&mut self, item: NavigationMenuItemMetadata<T>) -> usize {
        let order = self.next_item_order;
        self.next_item_order += 1;
        self.items.push(item);
        order
    }

    pub fn register_list_entry(&mut self, entry: NavigationMenuListEntry<T>) -> usize {
        let index = self.next_entry_index;
        self.next_entry_index += 1;
        self.list_entries.push(entry);
        index
    }

    pub fn set_entry_focus_handle(&mut self, index: usize, focus_handle: gpui::FocusHandle) {
        if let Some(entry) = self.list_entries.get_mut(index) {
            *entry = NavigationMenuListEntry::new(
                Some(focus_handle),
                entry.trigger_value().cloned(),
                false,
            );
        }
    }

    /// Routes an item's content to the popup side of the tree: the viewport
    /// takes the collected set and renders the active one.
    pub fn collect_content(&mut self, content: NavigationMenuContent<T>) {
        self.contents.push(content);
    }

    pub fn take_contents(&mut self) -> Vec<NavigationMenuContent<T>> {
        std::mem::take(&mut self.contents)
    }
}

/// Wires the root's children: list side first (so item content is collected
/// before the portal chain claims it), then the portal side, then context
/// attachment.
pub fn wire_children<T: Clone + Eq + 'static>(
    children: Vec<NavigationMenuChild<T>>,
    context: NavigationMenuContext<T>,
    window: &mut Window,
    cx: &mut App,
) -> WiredNavigationMenuChildren<T> {
    let mut wiring = NavigationMenuChildWiring::new(context.root_id());

    let children: Vec<NavigationMenuChild<T>> = children
        .into_iter()
        .map(|child| match child {
            NavigationMenuChild::List(list) => NavigationMenuChild::List(Box::new(
                list.wire_navigation_menu_child(&mut wiring, window, cx),
            )),
            other => other,
        })
        .collect();
    let children: Vec<NavigationMenuChild<T>> = children
        .into_iter()
        .map(|child| match child {
            NavigationMenuChild::Portal(portal) => NavigationMenuChild::Portal(Box::new(
                portal.wire_navigation_menu_child(&mut wiring, window, cx),
            )),
            other => other,
        })
        .map(|child| match child {
            NavigationMenuChild::List(list) => NavigationMenuChild::List(Box::new(
                list.with_navigation_menu_context(context.clone()),
            )),
            NavigationMenuChild::Portal(portal) => NavigationMenuChild::Portal(Box::new(
                portal.with_navigation_menu_context(context.clone()),
            )),
            NavigationMenuChild::Any(any) => NavigationMenuChild::Any(any),
        })
        .collect();

    WiredNavigationMenuChildren {
        items: std::mem::take(&mut wiring.items),
        list_entries: std::mem::take(&mut wiring.list_entries),
        children,
    }
}

pub fn scoped_part_id(root_id: &ElementId, part: &str) -> ElementId {
    ElementId::from((root_id.clone(), SharedString::from(part)))
}

pub fn scoped_value_id(root_id: &ElementId, part: &str, order: usize) -> ElementId {
    ElementId::from((
        root_id.clone(),
        SharedString::from(format!("{part}-{order}")),
    ))
}
