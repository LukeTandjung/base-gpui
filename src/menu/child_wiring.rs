use std::sync::Arc;

use gpui::{AnyElement, App, ElementId, Entity, FocusHandle, SharedString, Window};

use crate::menu::{
    MenuChild, MenuContext, MenuGroupChild, MenuItemMetadata, MenuPopupChild, MenuPortalChild,
    MenuPositionerChild, MenuRadioGroupChild, MenuSubmenuRootChild, MenuTriggerMetadata,
};

/// Wiring output collected while walking one menu's child tree.
pub struct WiredMenuChildren<P: Clone + 'static> {
    pub trigger: Option<MenuTriggerMetadata<P>>,
    pub items: Vec<MenuItemMetadata>,
    pub group_labels: Vec<SharedString>,
    pub focus_handles: Vec<FocusHandle>,
    pub popup_focus_handle: Option<FocusHandle>,
    pub checkbox_defaults: Vec<(usize, bool)>,
    pub radio_defaults: Vec<(usize, Option<usize>)>,
    pub children: Vec<MenuChild<P>>,
}

/// The only module that walks menu children, assigns item indices (groups,
/// labels, and separators never consume indices), collects typeahead labels,
/// and attaches contexts.
pub struct MenuChildWiring<P: Clone + 'static> {
    root_id: ElementId,
    next_item_index: usize,
    next_radio_group_index: usize,
    trigger: Option<MenuTriggerMetadata<P>>,
    items: Vec<MenuItemMetadata>,
    group_labels: Vec<SharedString>,
    focus_handles: Vec<FocusHandle>,
    popup_focus_handle: Option<FocusHandle>,
    checkbox_defaults: Vec<(usize, bool)>,
    radio_defaults: Vec<(usize, Option<usize>)>,
}

impl<P: Clone + 'static> MenuChildWiring<P> {
    pub fn new(root_id: ElementId) -> Self {
        Self {
            root_id,
            next_item_index: 0,
            next_radio_group_index: 0,
            trigger: None,
            items: Vec::new(),
            group_labels: Vec::new(),
            focus_handles: Vec::new(),
            popup_focus_handle: None,
            checkbox_defaults: Vec::new(),
            radio_defaults: Vec::new(),
        }
    }

    pub fn scope_child_id(&self, id: &ElementId) -> ElementId {
        ElementId::from((self.root_id.clone(), SharedString::from(id.to_string())))
    }

    pub fn register_trigger(&mut self, trigger: MenuTriggerMetadata<P>) {
        if let Some(focus_handle) = trigger.focus_handle() {
            self.focus_handles.push(focus_handle);
        }
        self.trigger = Some(trigger);
    }

    pub fn register_item(&mut self, item: MenuItemMetadata) -> usize {
        let index = item.index();
        self.next_item_index = index + 1;
        if let Some(focus_handle) = item.focus_handle() {
            self.focus_handles.push(focus_handle);
        }
        self.items.push(item);
        index
    }

    pub fn next_item_index(&self) -> usize {
        self.next_item_index
    }

    pub fn next_radio_group_index(&mut self) -> usize {
        let index = self.next_radio_group_index;
        self.next_radio_group_index += 1;
        index
    }

    pub fn register_group_label(&mut self, label: SharedString) {
        self.group_labels.push(label);
    }

    pub fn register_popup_focus_handle(&mut self, focus_handle: FocusHandle) {
        self.focus_handles.push(focus_handle.clone());
        self.popup_focus_handle = Some(focus_handle);
    }

    pub fn register_checkbox_default(&mut self, index: usize, default_checked: bool) {
        self.checkbox_defaults.push((index, default_checked));
    }

    pub fn register_radio_default(&mut self, group_index: usize, selected_item: Option<usize>) {
        self.radio_defaults.push((group_index, selected_item));
    }

    fn finish(self, children: Vec<MenuChild<P>>) -> WiredMenuChildren<P> {
        WiredMenuChildren {
            trigger: self.trigger,
            items: self.items,
            group_labels: self.group_labels,
            focus_handles: self.focus_handles,
            popup_focus_handle: self.popup_focus_handle,
            checkbox_defaults: self.checkbox_defaults,
            radio_defaults: self.radio_defaults,
            children,
        }
    }
}

/// Single-pass wiring: attaches the context and registers metadata.
pub trait MenuChildNode<P: Clone + 'static>: Sized {
    fn wire_menu_child(
        self,
        _wiring: &mut MenuChildWiring<P>,
        _context: &MenuContext<P>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        self
    }
}

/// Type-erased radio group node so `MenuRadioGroup<P, V>` can live inside
/// `MenuPopupChild<P>` without leaking `V`.
pub trait MenuRadioGroupErased<P: Clone + 'static> {
    fn wire_erased(
        self: Box<Self>,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Box<dyn MenuRadioGroupErased<P>>;

    fn into_any(self: Box<Self>) -> AnyElement;
}

pub fn wire_children<P: Clone + 'static>(
    children: Vec<MenuChild<P>>,
    context: MenuContext<P>,
    window: &mut Window,
    cx: &mut App,
) -> WiredMenuChildren<P> {
    let mut wiring = MenuChildWiring::new(context.root_id());
    let children = children
        .into_iter()
        .map(|child| child.wire_menu_child(&mut wiring, &context, window, cx))
        .collect();

    wiring.finish(children)
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuChild<P> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Trigger(trigger) => Self::Trigger(Box::new(
                trigger.wire_menu_child(wiring, context, window, cx),
            )),
            Self::Portal(portal) => Self::Portal(Box::new(
                portal.wire_menu_child(wiring, context, window, cx),
            )),
            Self::ContextArea(build) => Self::Any(build(context, window, cx)),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuPortalChild<P> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Backdrop(backdrop) => Self::Backdrop(Box::new(
                backdrop.wire_menu_child(wiring, context, window, cx),
            )),
            Self::Positioner(positioner) => Self::Positioner(Box::new(
                positioner.wire_menu_child(wiring, context, window, cx),
            )),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuPositionerChild<P> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Popup(popup) => {
                Self::Popup(Box::new(popup.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Arrow(arrow) => {
                Self::Arrow(Box::new(arrow.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuPopupChild<P> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::Item(item) => {
                Self::Item(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::LinkItem(item) => {
                Self::LinkItem(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::CheckboxItem(item) => {
                Self::CheckboxItem(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::RadioGroup(group) => {
                Self::RadioGroup(group.wire_erased(wiring, context, window, cx))
            }
            Self::Group(group) => {
                Self::Group(Box::new(group.wire_menu_child(wiring, context, window, cx)))
            }
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::SubmenuRoot(submenu) => Self::SubmenuRoot(Box::new(
                submenu.wire_menu_child(wiring, context, window, cx),
            )),
            Self::Arrow(arrow) => {
                Self::Arrow(Box::new(arrow.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuGroupChild<P> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Item(item) => {
                Self::Item(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::LinkItem(item) => {
                Self::LinkItem(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::CheckboxItem(item) => {
                Self::CheckboxItem(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static, V: Clone + Eq + 'static> MenuChildNode<P> for MenuRadioGroupChild<P, V> {
    fn wire_menu_child(
        self,
        wiring: &mut MenuChildWiring<P>,
        context: &MenuContext<P>,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        match self {
            Self::RadioItem(item) => {
                Self::RadioItem(Box::new(item.wire_menu_child(wiring, context, window, cx)))
            }
            Self::GroupLabel(label) => {
                Self::GroupLabel(Box::new(label.wire_menu_child(wiring, context, window, cx)))
            }
            Self::Separator(separator) => Self::Separator(separator),
            Self::Any(any) => Self::Any(any),
        }
    }
}

impl<P: Clone + 'static> MenuChildNode<P> for MenuSubmenuRootChild<P> {}

pub fn part_focus_handle(id: &ElementId, window: &mut Window, cx: &mut App) -> FocusHandle {
    let focus_handle_entity: Entity<FocusHandle> = window.use_keyed_state(
        ElementId::NamedChild(Arc::new(id.clone()), SharedString::from("focus")),
        cx,
        |_, cx| cx.focus_handle(),
    );

    focus_handle_entity.read(cx).clone()
}
